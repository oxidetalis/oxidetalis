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

//! Handler for incoming and outgoing chat requests.

use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use sea_orm::DatabaseConnection;

use crate::database::IncomingChatExt;
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
    chat_request_sender: Option<&UserModel>,
    chat_request_recipient: &PublicKey,
) -> Option<ServerEvent<Unsigned>> {
    let Some(chat_request_sender) = chat_request_sender else {
        return Some(WsError::RegistredUserEvent.into());
    };
    let Some(chat_request_recipient) =
        try_ws!(Some db.get_user_by_pubk(chat_request_recipient).await)
    else {
        return Some(WsError::UserNotFound.into());
    };
    if chat_request_sender.id == chat_request_recipient.id {
        return Some(WsError::CannotSendChatRequestToSelf.into());
    }

    if try_ws!(Some db.get_chat_request_to(chat_request_sender, &chat_request_recipient.public_key).await).is_some() {
        return Some(WsError::AlreadySendChatRequest.into());
    }

    if try_ws!(Some db.is_blacklisted(&chat_request_recipient, &chat_request_sender.public_key).await)
    {
        return Some(WsError::RecipientBlacklist.into());
    }

    // To ignore the error if the requester added the recipient to the whitelist
    // table before send a request to them
    if let Err(ServerError::Internal(_)) = db
        .add_to_whitelist(chat_request_sender, &chat_request_recipient.public_key)
        .await
    {
        return Some(WsError::InternalServerError.into());
    }

    if try_ws!(Some db.is_whitelisted(&chat_request_recipient, &chat_request_sender.public_key).await)
    {
        return Some(WsError::AlreadyInRecipientWhitelist.into());
    }

    try_ws!(Some db.save_out_chat_request(chat_request_sender, &chat_request_recipient.public_key).await);

    if let Some(conn_id) = ONLINE_USERS
        .is_online(&chat_request_recipient.public_key)
        .await
    {
        ONLINE_USERS
            .send(
                &conn_id,
                ServerEvent::chat_request(&chat_request_sender.public_key),
            )
            .await;
    } else {
        try_ws!(Some db.save_in_chat_request(&chat_request_recipient, &chat_request_sender.public_key).await);
    }
    None
}

#[logcall::logcall]
pub async fn handle_chat_response(
    db: &DatabaseConnection,
    response_sender: Option<&UserModel>,
    response_recipient: &PublicKey,
    accepted: bool,
) -> Option<ServerEvent<Unsigned>> {
    let Some(response_sender) = response_sender else {
        return Some(WsError::RegistredUserEvent.into());
    };

    let Some(response_recipient) = try_ws!(Some db.get_user_by_pubk(response_recipient).await)
    else {
        return Some(WsError::UserNotFound.into());
    };

    if response_sender.id == response_recipient.id {
        return Some(WsError::CannotRespondToOwnChatRequest.into());
    }

    if try_ws!(Some
        db.get_chat_request_to(&response_recipient, &response_sender.public_key)
            .await
    )
    .is_none()
    {
        return Some(WsError::NoChatRequestFromRecipient.into());
    }

    // We don't need to handle the case where the sender is blacklisted or
    // whitelisted already, just add it if it is not already there
    let _ = if accepted {
        db.add_to_whitelist(response_sender, &response_recipient.public_key)
            .await
    } else {
        db.add_to_blacklist(response_sender, &response_recipient.public_key)
            .await
    };

    try_ws!(Some
        db.remove_out_chat_request(&response_recipient, &response_sender.public_key)
            .await
    );

    if let Some(conn_id) = ONLINE_USERS.is_online(&response_recipient.public_key).await {
        ONLINE_USERS
            .send(
                &conn_id,
                ServerEvent::chat_request_response(response_sender.public_key, accepted),
            )
            .await;
    } else {
        try_ws!(Some
            db.save_in_chat_response(&response_recipient, &response_sender.public_key, accepted).await
        );
    }

    None
}
