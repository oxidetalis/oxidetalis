// OxideTalis Messaging Protocol homeserver implementation
// Copyright (C) 2024 Awiteb <a@4rs.nl>, OxideTalis Contributors
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

//! Oxidetalis WebSocket server implementation, handling the WebSocket
//! connections.

use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::Utc;
use errors::{WsError, WsResult};
use futures::{channel::mpsc, FutureExt, StreamExt, TryStreamExt};
use once_cell::sync::Lazy;
use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use salvo::{
    handler,
    http::StatusError,
    websocket::{Message, WebSocket, WebSocketUpgrade},
    Depot,
    Request,
    Response,
    Router,
};
use sea_orm::DatabaseConnection;
use tokio::{sync::RwLock, task::spawn as tokio_spawn, time::sleep as tokio_sleep};

pub mod errors;
mod events;
mod handlers;

pub use events::*;
use uuid::Uuid;

use crate::{
    database::UserTableExt,
    extensions::{DepotExt, OnlineUsersExt},
    middlewares,
    nonce::NonceCache,
    utils,
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
    let db_conn = depot.db_conn();
    let public_key =
        utils::extract_public_key(req).expect("The public key was checked in the middleware");
    let shared_secret = depot.config().server.private_key.shared_secret(&public_key);

    WebSocketUpgrade::new()
        .upgrade(req, res, move |ws| {
            handle_socket(ws, db_conn, nonce_cache, public_key, shared_secret)
        })
        .await
}

/// Handle the websocket connection
async fn handle_socket(
    ws: WebSocket,
    db_conn: Arc<DatabaseConnection>,
    nonce_cache: Arc<NonceCache>,
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
    let Ok(user) = db_conn.get_user_by_pubk(&user_public_key).await else {
        let _ = sender.unbounded_send(Ok(ServerEvent::from(WsError::InternalServerError)
            .sign(&user_shared_secret)
            .as_ref()
            .into()));
        return;
    };
    ONLINE_USERS
        .add_user(
            &conn_id,
            SocketUserData::new(user_public_key, user_shared_secret, sender.clone()),
        )
        .await;
    log::info!("New user connected: ConnId(={conn_id}) PublicKey(={user_public_key})");

    // TODO: Send the incoming chat request to the user, while they are offline.
    // This after adding last_login col to the user table

    while let Some(Ok(msg)) = user_ws_receiver.next().await {
        match handle_ws_msg(msg, &nonce_cache, &user_shared_secret).await {
            Ok(event) => {
                if let Some(server_event) =
                    handle_events(event, &db_conn, &conn_id, user.as_ref()).await
                {
                    if let Err(err) = sender
                        .unbounded_send(Ok(server_event.sign(&user_shared_secret).as_ref().into()))
                    {
                        log::error!("Websocket Error: {err}");
                        break;
                    }
                };
            }
            Err(err) => {
                if let Err(err) = sender.unbounded_send(Ok(ServerEvent::from(err)
                    .sign(&user_shared_secret)
                    .as_ref()
                    .into()))
                {
                    log::error!("Websocket Error: {err}");
                    break;
                };
            }
        };
    }
    user_disconnected(&db_conn, &conn_id, &user_public_key, user).await;
}

/// Handle websocket msg
async fn handle_ws_msg(
    msg: Message,
    nonce_cache: &NonceCache,
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
    if !event.verify_signature(shared_secret, nonce_cache).await {
        return Err(WsError::InvalidSignature);
    }
    Ok(event)
}

/// Handle user events, and return the server event if needed
async fn handle_events(
    event: ClientEvent,
    db: &DatabaseConnection,
    conn_id: &Uuid,
    user: Option<&UserModel>,
) -> Option<ServerEvent<Unsigned>> {
    match &event.event {
        ClientEventType::Ping { .. } => Some(ServerEvent::pong()),
        ClientEventType::Pong { .. } => {
            ONLINE_USERS.update_pong(conn_id).await;
            None
        }
        ClientEventType::ChatRequest { to } => handlers::handle_chat_request(db, user, to).await,
        ClientEventType::ChatRequestResponse { to, accepted } => {
            handlers::handle_chat_response(db, user, to, *accepted).await
        }
    }
}

/// Handle user disconnected
async fn user_disconnected(
    db_conn: &DatabaseConnection,
    conn_id: &Uuid,
    public_key: &PublicKey,
    user: Option<UserModel>,
) {
    ONLINE_USERS.remove_user(conn_id).await;
    if ONLINE_USERS.is_online(public_key).await.is_none() {
        if let Some(mut user) = user.map(IntoActiveModel::into_active_model) {
            user.last_logout = Set(Utc::now());
            if let Err(err) = user.update(db_conn).await {
                log::error!("{err}");
            }
        }
    }
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
