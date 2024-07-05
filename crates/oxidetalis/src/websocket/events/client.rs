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

//! Events that the client send it

use oxidetalis_core::types::Signature;
use serde::{Deserialize, Serialize};

use crate::{nonce::NonceCache, utils};

/// Client websocket event
#[derive(Deserialize, Clone, Debug)]
pub struct ClientEvent {
    pub event: ClientEventType,
    signature: Signature,
}

/// Client websocket event type
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase", tag = "event", content = "data")]
pub enum ClientEventType {
    /// Ping event
    Ping { timestamp: u64 },
    /// Pong event
    Pong { timestamp: u64 },
}

impl ClientEventType {
    /// Returns event data as json bytes
    pub fn data(&self) -> Vec<u8> {
        serde_json::to_value(self).expect("can't fail")["data"]
            .to_string()
            .into_bytes()
    }
}

impl ClientEvent {
    /// Verify the signature of the event
    pub async fn verify_signature(
        &self,
        shared_secret: &[u8; 32],
        nonce_cache: &NonceCache,
    ) -> bool {
        utils::is_valid_nonce(&self.signature, nonce_cache).await
            && self.signature.verify(&self.event.data(), shared_secret)
    }
}
