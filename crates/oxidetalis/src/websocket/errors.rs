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

use crate::ws_errors;

/// Result type of websocket
pub type WsResult<T> = Result<T, WsError>;

ws_errors! {
    InternalServerError = "Internal server error",
    InvalidSignature = "Invalid event signature",
    NotTextMessage = "The websocket message must be text message",
    InvalidJsonData = "Received invalid json data, the text must be valid json",
    UnknownClientEvent = "Unknown client event, the event is not recognized by the server",
    RegistredUserEvent = "The event is only for registred users",
    UserNotFound = "The user is not registered in the server",
    AlreadyOnTheWhitelist = "The user is already on your whitelist",
    CannotAddSelfToWhitelist = "You cannot add yourself to the whitelist",
    AlreadyOnTheBlacklist = "The user is already on your blacklist",
    CannotAddSelfToBlacklist = "You cannot add yourself to the blacklist",
    AlreadySendChatRequest = "You have already sent a chat request to this user",
    CannotSendChatRequestToSelf = "You cannot send a chat request to yourself",
    CannotRespondToOwnChatRequest = "You cannot respond to your own chat request",
    NoChatRequestFromRecipient = "You do not have a chat request from the recipient",
    RecipientBlacklist = "You cannot send a chat request because you are on the recipient's blacklist.",
    AlreadyInRecipientWhitelist = "You are already on the recipient's whitelist and can chat with them."
}
