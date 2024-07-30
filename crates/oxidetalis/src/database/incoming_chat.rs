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

//! Database extension for the `incoming_chat` table.

use chrono::Utc;
use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use sea_orm::{sea_query::OnConflict, DatabaseConnection};

use crate::errors::ServerResult;

/// Extension trait for the `incoming_chat` table.
pub trait IncomingChatExt {
    /// Save the incoming chat request
    async fn save_in_chat_request(
        &self,
        chat_request_recipient: &UserModel,
        chat_request_sender: &PublicKey,
    ) -> ServerResult<()>;

    /// Returns all incoming chat requests for the given recipient
    async fn get_all_chat_requests(
        &self,
        chat_request_recipient: &UserModel,
    ) -> ServerResult<Vec<IncomingChatModel>>;

    /// Save the incoming chat response
    async fn save_in_chat_response(
        &self,
        chat_response_recipient: &UserModel,
        chat_response_sender: &PublicKey,
        accepted_response: bool,
    ) -> ServerResult<()>;

    /// Returns all incoming chat responses for the given recipient
    async fn get_all_chat_responses(
        &self,
        chat_response_recipient: &UserModel,
    ) -> ServerResult<Vec<IncomingChatModel>>;
}

impl IncomingChatExt for DatabaseConnection {
    #[logcall::logcall]
    async fn save_in_chat_request(
        &self,
        chat_request_recipient: &UserModel,
        chat_request_sender: &PublicKey,
    ) -> ServerResult<()> {
        save(self, chat_request_recipient, chat_request_sender, None).await
    }

    async fn get_all_chat_requests(
        &self,
        chat_request_recipient: &UserModel,
    ) -> ServerResult<Vec<IncomingChatModel>> {
        get_all::<true>(self, chat_request_recipient).await
    }

    #[logcall::logcall]
    async fn save_in_chat_response(
        &self,
        chat_response_recipient: &UserModel,
        chat_response_sender: &PublicKey,
        accepted_response: bool,
    ) -> ServerResult<()> {
        save(
            self,
            chat_response_recipient,
            chat_response_sender,
            Some(accepted_response),
        )
        .await
    }

    async fn get_all_chat_responses(
        &self,
        chat_response_recipient: &UserModel,
    ) -> ServerResult<Vec<IncomingChatModel>> {
        get_all::<false>(self, chat_response_recipient).await
    }
}

/// Utility function to save incoming chat request or response
async fn save(
    db: &DatabaseConnection,
    recipient: &UserModel,
    sender: &PublicKey,
    accepted_response: Option<bool>,
) -> ServerResult<()> {
    IncomingChatEntity::insert(IncomingChatActiveModel {
        recipient_id: Set(recipient.id),
        sender: Set(*sender),
        received_timestamp: Set(Utc::now()),
        accepted_response: Set(accepted_response),
        ..Default::default()
    })
    .on_conflict(
        OnConflict::columns([
            IncomingChatColumn::RecipientId,
            IncomingChatColumn::Sender,
            IncomingChatColumn::AcceptedResponse,
        ])
        .do_nothing()
        .to_owned(),
    )
    .exec(db)
    .await?;
    Ok(())
}

/// Utility function to get all incoming chat requests or responses
async fn get_all<const IS_REQUEST: bool>(
    db: &DatabaseConnection,
    recipient: &UserModel,
) -> ServerResult<Vec<IncomingChatModel>> {
    recipient
        .find_related(IncomingChatEntity)
        .filter(
            if IS_REQUEST {
                IncomingChatColumn::AcceptedResponse.is_null()
            } else {
                IncomingChatColumn::AcceptedResponse.is_not_null()
            },
        )
        .all(db)
        .await
        .map_err(Into::into)
}
