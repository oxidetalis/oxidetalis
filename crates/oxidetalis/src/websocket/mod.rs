// OxideTalis Messaging Protocol homeserver implementation
// Copyright (C) 2024 OxideTalis Developers <otmp@4rs.nl>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://gnu.org/licenses/agpl-3.0>.

use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::Utc;
use errors::{WsError, WsResult};
use futures::{channel::mpsc, FutureExt, StreamExt, TryStreamExt};
use once_cell::sync::Lazy;
use oxidetalis_core::{cipher::K256Secret, types::PublicKey};
use salvo::{
    handler,
    http::StatusError,
    websocket::{Message, WebSocket, WebSocketUpgrade},
    Depot,
    Request,
    Response,
    Router,
};
use tokio::{sync::RwLock, task::spawn as tokio_spawn, time::sleep as tokio_sleep};

mod errors;
mod events;

pub use events::*;
use uuid::Uuid;

use crate::{
    extensions::{DepotExt, OnlineUsersExt},
    middlewares,
    utils,
    NonceCache,
};

/// Online users type
pub type OnlineUsers = RwLock<HashMap<Uuid, SocketUserData>>;

/// List of online users, users that are connected to the server
// FIXME: Use `std::sync::LazyLock` after it becomes stable in `1.80.0`
static ONLINE_USERS: Lazy<OnlineUsers> = Lazy::new(OnlineUsers::default);

/// A user connected to the server
pub struct SocketUserData {
    /// Sender to send messages to the user
    pub sender:        mpsc::UnboundedSender<salvo::Result<Message>>,
    /// User public key
    pub public_key:    PublicKey,
    /// Time that the user pinged at
    pub pinged_at:     chrono::DateTime<Utc>,
    /// Time that the user ponged at
    pub ponged_at:     chrono::DateTime<Utc>,
    /// User shared secret
    pub shared_secret: [u8; 32],
}

impl SocketUserData {
    /// Creates new [`SocketUserData`]
    pub fn new(
        public_key: PublicKey,
        shared_secret: [u8; 32],
        sender: mpsc::UnboundedSender<salvo::Result<Message>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            sender,
            public_key,
            shared_secret,
            pinged_at: now,
            ponged_at: now,
        }
    }
}

/// WebSocket handler, that handles the user connection
#[handler]
pub async fn user_connected(
    req: &mut Request,
    res: &mut Response,
    depot: &Depot,
) -> Result<(), StatusError> {
    let nonce_cache = depot.nonce_cache();
    let nonce_limit = *depot.nonce_cache_size();
    let public_key =
        utils::extract_public_key(req).expect("The public key was checked in the middleware");
    // FIXME: The config should hold `K256Secret` not `PrivateKey`
    let shared_secret =
        K256Secret::from_privkey(&depot.config().server.private_key).shared_secret(&public_key);

    WebSocketUpgrade::new()
        .upgrade(req, res, move |ws| {
            handle_socket(ws, nonce_cache, nonce_limit, public_key, shared_secret)
        })
        .await
}

/// Handle the websocket connection
async fn handle_socket(
    ws: WebSocket,
    nonce_cache: Arc<NonceCache>,
    nonce_limit: usize,
    user_public_key: PublicKey,
    user_shared_secret: [u8; 32],
) {
    let (user_ws_sender, mut user_ws_receiver) = ws.split();

    let (sender, receiver) = mpsc::unbounded();
    let receiver = receiver.into_stream();
    let fut = receiver.forward(user_ws_sender).map(|result| {
        if let Err(err) = result {
            log::error!("websocket send error: {err}");
        }
    });
    tokio_spawn(fut);
    let conn_id = Uuid::new_v4();
    let user = SocketUserData::new(user_public_key, user_shared_secret, sender.clone());
    ONLINE_USERS.add_user(&conn_id, user).await;
    log::info!("New user connected: ConnId(={conn_id}) PublicKey(={user_public_key})");

    let fut = async move {
        while let Some(Ok(msg)) = user_ws_receiver.next().await {
            match handle_ws_msg(msg, &nonce_cache, &nonce_limit, &user_shared_secret) {
                Ok(event) => {
                    if let Some(server_event) = handle_events(event, &conn_id).await {
                        if let Err(err) = sender.unbounded_send(Ok(Message::from(
                            &server_event.sign(&user_shared_secret),
                        ))) {
                            log::error!("Websocket Error: {err}");
                            break;
                        }
                    };
                }
                Err(err) => {
                    if let Err(err) = sender.unbounded_send(Ok(Message::from(
                        &ServerEvent::from(err).sign(&user_shared_secret),
                    ))) {
                        log::error!("Websocket Error: {err}");
                        break;
                    };
                }
            };
        }
        user_disconnected(&conn_id, &user_public_key).await;
    };
    tokio_spawn(fut);
}

/// Handle websocket msg
fn handle_ws_msg(
    msg: Message,
    nonce_cache: &NonceCache,
    nonce_limit: &usize,
    shared_secret: &[u8; 32],
) -> WsResult<ClientEvent> {
    let Ok(text) = msg.to_str() else {
        return Err(WsError::NotTextMessage);
    };
    let event = serde_json::from_str::<ClientEvent>(text).map_err(|err| {
        if err.is_data() {
            WsError::UnknownClientEvent
        } else {
            WsError::InvalidJsonData
        }
    })?;
    if !event.verify_signature(shared_secret, nonce_cache, nonce_limit) {
        return Err(WsError::InvalidSignature);
    }
    Ok(event)
}

/// Handle user events, and return the server event if needed
async fn handle_events(event: ClientEvent, conn_id: &Uuid) -> Option<ServerEvent<Unsigned>> {
    match &event.event {
        ClientEventType::Ping { .. } => Some(ServerEvent::pong()),
        ClientEventType::Pong { .. } => {
            ONLINE_USERS.update_pong(conn_id).await;
            None
        }
    }
}

/// Handle user disconnected
async fn user_disconnected(conn_id: &Uuid, public_key: &PublicKey) {
    ONLINE_USERS.remove_user(conn_id).await;
    log::debug!("User disconnect: ConnId(={conn_id}) PublicKey(={public_key})");
}

pub fn route() -> Router {
    let users_pinger = async {
        /// Seconds to wait for pongs, before disconnecting the user
        const WAIT_FOR_PONGS_SECS: u32 = 10;
        /// Seconds to sleep between pings (10 minutes)
        const SLEEP_SECS: u32 = 60 * 10;
        loop {
            log::debug!("Start pinging online users");
            ONLINE_USERS.ping_all().await;
            tokio_sleep(Duration::from_secs(u64::from(WAIT_FOR_PONGS_SECS))).await;
            ONLINE_USERS.disconnect_inactive_users().await;
            log::debug!("Done pinging online users and disconnected inactive ones");
            tokio_sleep(Duration::from_secs(u64::from(SLEEP_SECS))).await;
        }
    };

    tokio_spawn(users_pinger);

    Router::new()
        .push(Router::with_path("chat").get(user_connected))
        .hoop(middlewares::signature_check)
        .hoop(middlewares::public_key_check)
}
