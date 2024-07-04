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

//! Websocket errors

/// Result type of websocket
pub type WsResult<T> = Result<T, WsError>;

/// Websocket errors, returned in the websocket communication
#[derive(Debug)]
pub enum WsError {
    /// The signature is invalid
    InvalidSignature,
    /// Message type must be text
    NotTextMessage,
    /// Invalid json data
    InvalidJsonData,
    /// Unknown client event
    UnknownClientEvent,
}

impl WsError {
    /// Returns error name
    pub const fn name(&self) -> &'static str {
        match self {
            WsError::InvalidSignature => "InvalidSignature",
            WsError::NotTextMessage => "NotTextMessage",
            WsError::InvalidJsonData => "InvalidJsonData",
            WsError::UnknownClientEvent => "UnknownClientEvent",
        }
    }

    /// Returns the error reason
    pub const fn reason(&self) -> &'static str {
        match self {
            WsError::InvalidSignature => "Invalid event signature",
            WsError::NotTextMessage => "The websocket message must be text message",
            WsError::InvalidJsonData => "Received invalid json data, the text must be valid json",
            WsError::UnknownClientEvent => {
                "Unknown client event, the event is not recognized by the server"
            }
        }
    }
}
