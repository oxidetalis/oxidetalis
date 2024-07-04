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

//! Events that the server send it

use std::marker::PhantomData;

use chrono::Utc;
use oxidetalis_core::{cipher::K256Secret, types::Signature};
use salvo::websocket::Message;
use serde::Serialize;

use crate::websocket::errors::WsError;

/// Signed marker, used to indicate that the event is signed
pub struct Signed;
/// Unsigned marker, used to indicate that the event is unsigned
pub struct Unsigned;

/// Server websocket event
#[derive(Serialize, Clone, Debug)]
pub struct ServerEvent<T> {
    #[serde(flatten)]
    event:     ServerEventType,
    signature: Signature,
    #[serde(skip)]
    phantom:   PhantomData<T>,
}

/// server websocket event type
#[derive(Serialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ServerEventType {
    /// Ping event
    Ping { timestamp: u64 },
    /// Pong event
    Pong { timestamp: u64 },
    /// Error event
    Error {
        name:   &'static str,
        reason: &'static str,
    },
}

impl ServerEventType {
    /// Returns event data as json bytes
    pub fn data(&self) -> Vec<u8> {
        serde_json::to_value(self).expect("can't fail")["data"]
            .to_string()
            .into_bytes()
    }
}

impl ServerEvent<Unsigned> {
    /// Creates new [`ServerEvent`]
    pub fn new(event: ServerEventType) -> Self {
        Self {
            event,
            signature: Signature::from([0u8; 56]),
            phantom: PhantomData,
        }
    }

    /// Creates ping event
    pub fn ping() -> Self {
        Self::new(ServerEventType::Ping {
            timestamp: Utc::now().timestamp() as u64,
        })
    }

    /// Creates pong event
    pub fn pong() -> Self {
        Self::new(ServerEventType::Pong {
            timestamp: Utc::now().timestamp() as u64,
        })
    }

    /// Sign the event
    pub fn sign(self, shared_secret: &[u8; 32]) -> ServerEvent<Signed> {
        ServerEvent::<Signed> {
            signature: K256Secret::sign_with_shared_secret(
                &serde_json::to_vec(&self.event.data()).expect("Can't fail"),
                shared_secret,
            ),
            event:     self.event,
            phantom:   PhantomData,
        }
    }
}

impl From<&ServerEvent<Signed>> for Message {
    fn from(value: &ServerEvent<Signed>) -> Self {
        Message::text(serde_json::to_string(value).expect("This can't fail"))
    }
}

impl From<WsError> for ServerEvent<Unsigned> {
    fn from(err: WsError) -> Self {
        ServerEvent::new(ServerEventType::Error {
            name:   err.name(),
            reason: err.reason(),
        })
    }
}
