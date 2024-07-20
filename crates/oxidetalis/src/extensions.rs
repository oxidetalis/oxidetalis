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

use std::sync::Arc;

use chrono::Utc;
use oxidetalis_config::Config;
use oxidetalis_core::types::PublicKey;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use salvo::Depot;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    nonce::NonceCache,
    websocket::{OnlineUsers, ServerEvent, SocketUserData, Unsigned},
};

/// Extension trait for the Depot.
pub trait DepotExt {
    /// Returns the database connection
    fn db_conn(&self) -> Arc<DatabaseConnection>;
    /// Returns the server configuration
    fn config(&self) -> &Config;
    /// Retutns the nonce cache
    fn nonce_cache(&self) -> Arc<NonceCache>;
}

/// Extension trait for online websocket users
pub trait OnlineUsersExt {
    /// Add new user to the online users
    async fn add_user(&self, conn_id: &Uuid, data: SocketUserData);

    /// Remove user from online users
    async fn remove_user(&self, conn_id: &Uuid);

    /// Ping all online users
    async fn ping_all(&self);

    /// Update user pong at time
    async fn update_pong(&self, conn_id: &Uuid);

    /// Disconnect inactive users (who not respond for the ping event)
    async fn disconnect_inactive_users(&self);

    /// Returns the connection id of the user, if it is online
    async fn is_online(&self, public_key: &PublicKey) -> Option<Uuid>;

    /// Send an event to user by connection id
    async fn send(&self, conn_id: &Uuid, event: ServerEvent<Unsigned>);
}

impl DepotExt for Depot {
    fn db_conn(&self) -> Arc<DatabaseConnection> {
        Arc::clone(
            self.obtain::<Arc<DatabaseConnection>>()
                .expect("Database connection not found"),
        )
    }

    fn config(&self) -> &Config {
        self.obtain::<Arc<Config>>().expect("Config not found")
    }

    fn nonce_cache(&self) -> Arc<NonceCache> {
        Arc::clone(
            self.obtain::<Arc<NonceCache>>()
                .expect("Nonce cache not found"),
        )
    }
}

impl OnlineUsersExt for OnlineUsers {
    async fn add_user(&self, conn_id: &Uuid, data: SocketUserData) {
        self.write().await.insert(*conn_id, data);
    }

    async fn remove_user(&self, conn_id: &Uuid) {
        self.write().await.remove(conn_id);
    }

    async fn ping_all(&self) {
        let now = Utc::now();
        self.write().await.par_iter_mut().for_each(|(_, u)| {
            u.pinged_at = now;
            let _ = u.sender.unbounded_send(Ok(ServerEvent::ping()
                .sign(&u.shared_secret)
                .as_ref()
                .into()));
        });
    }

    async fn update_pong(&self, conn_id: &Uuid) {
        if let Some(user) = self.write().await.get_mut(conn_id) {
            user.ponged_at = Utc::now()
        }
    }

    async fn disconnect_inactive_users(&self) {
        self.write().await.retain(|_, u| {
            // if we send ping and the client doesn't send pong
            if u.pinged_at > u.ponged_at {
                log::info!("Disconnected from {}, inactive", u.public_key);
                u.sender.close_channel();
                return false;
            }
            true
        });
    }

    async fn is_online(&self, public_key: &PublicKey) -> Option<Uuid> {
        self.read()
            .await
            .iter()
            .find(|(_, u)| &u.public_key == public_key)
            .map(|(c, _)| *c)
    }

    async fn send(&self, conn_id: &Uuid, event: ServerEvent<Unsigned>) {
        if let Some(user) = self.read().await.get(conn_id) {
            let _ = user
                .sender
                .unbounded_send(Ok(event.sign(&user.shared_secret).as_ref().into()));
        }
    }
}
