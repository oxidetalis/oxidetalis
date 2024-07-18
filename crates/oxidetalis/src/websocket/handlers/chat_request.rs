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

//! Handler for incoming and outgoing chat requests.

use std::str::FromStr;

use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use sea_orm::DatabaseConnection;

use crate::database::InChatRequestsExt;
use crate::errors::ServerError;
use crate::extensions::OnlineUsersExt;
use crate::{
    database::{OutChatRequestsExt, UserTableExt, UsersStatusExt},
    try_ws,
    websocket::{errors::WsError, ServerEvent, Unsigned, ONLINE_USERS},
};

/// Handle a chat request from a user.
#[logcall::logcall]
pub async fn handle_chat_request(
    db: &DatabaseConnection,
    from: Option<&UserModel>,
    to_public_key: &PublicKey,
) -> Option<ServerEvent<Unsigned>> {
    let Some(from_user) = from else {
        return Some(WsError::RegistredUserEvent.into());
    };
    let Some(to_user) = try_ws!(Some db.get_user_by_pubk(to_public_key).await) else {
        return Some(WsError::UserNotFound.into());
    };
    if from_user.id == to_user.id {
        return Some(WsError::CannotSendChatRequestToSelf.into());
    }
    // FIXME: When change the entity public key to a PublicKey type, change this
    let from_public_key = PublicKey::from_str(&from_user.public_key).expect("Is valid public key");

    if try_ws!(Some db.get_chat_request_to(from_user, to_public_key).await).is_some() {
        return Some(WsError::AlreadySendChatRequest.into());
    }

    if try_ws!(Some db.is_blacklisted(&to_user, &from_public_key).await) {
        return Some(WsError::RecipientBlacklist.into());
    }

    // To ignore the error if the requester added the recipient to the whitelist
    // table before send a request to them
    if let Err(ServerError::Internal(_)) = db.add_to_whitelist(from_user, to_public_key).await {
        return Some(WsError::InternalServerError.into());
    }

    if try_ws!(Some db.is_whitelisted(&to_user, &from_public_key).await) {
        return Some(WsError::AlreadyInRecipientWhitelist.into());
    }

    try_ws!(Some db.save_out_chat_request(from_user, to_public_key).await);
    if let Some(conn_id) = ONLINE_USERS.is_online(to_public_key).await {
        ONLINE_USERS
            .send(&conn_id, ServerEvent::chat_request(&from_public_key))
            .await;
    } else {
        try_ws!(Some db.save_in_chat_request(&from_public_key, &to_user).await);
    }
    None
}

#[logcall::logcall]
pub async fn handle_chat_response(
    db: &DatabaseConnection,
    recipient: Option<&UserModel>,
    sender_public_key: &PublicKey,
    accepted: bool,
) -> Option<ServerEvent<Unsigned>> {
    let Some(recipient_user) = recipient else {
        return Some(WsError::RegistredUserEvent.into());
    };
    let Some(sender_user) = try_ws!(Some db.get_user_by_pubk(sender_public_key).await) else {
        return Some(WsError::UserNotFound.into());
    };
    if recipient_user.id == sender_user.id {
        return Some(WsError::CannotRespondToOwnChatRequest.into());
    }

    // FIXME: When change the entity public key to a PublicKey type, change this
    let recipient_public_key =
        PublicKey::from_str(&recipient_user.public_key).expect("Is valid public key");

    if try_ws!(Some
        db.get_chat_request_to(&sender_user, &recipient_public_key)
            .await
    )
    .is_none()
    {
        return Some(WsError::NoChatRequestFromRecipient.into());
    }

    // We don't need to handle the case where the sender is blacklisted or
    // whitelisted already, just add it if it is not already there
    let _ = if accepted {
        db.add_to_whitelist(recipient_user, sender_public_key).await
    } else {
        db.add_to_blacklist(recipient_user, sender_public_key).await
    };

    try_ws!(Some
        db.remove_out_chat_request(&sender_user, &recipient_public_key)
            .await
    );

    if let Some(conn_id) = ONLINE_USERS.is_online(sender_public_key).await {
        ONLINE_USERS
            .send(
                &conn_id,
                ServerEvent::chat_request_response(recipient_public_key, accepted),
            )
            .await;
    } else {
        // TODO: Create a table for chat request responses, and send them when
        // the user logs in
    }

    None
}
