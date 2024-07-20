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

//! Database extension for the `out_chat_requests` table.

use chrono::Utc;
use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use sea_orm::DatabaseConnection;

use crate::{errors::ServerResult, websocket::errors::WsError};

/// Extension trait for the `out_chat_requests` table.
pub trait OutChatRequestsExt {
    /// Returns the outgoing chat request if the `user` have a sent chat request
    /// to the `recipient`
    async fn get_chat_request_to(
        &self,
        requester: &UserModel,
        recipient: &PublicKey,
    ) -> ServerResult<Option<OutChatRequestsModel>>;

    /// Save the chat request in the requester table
    async fn save_out_chat_request(
        &self,
        requester: &UserModel,
        recipient: &PublicKey,
    ) -> ServerResult<()>;

    /// Remove the chat request from requester table
    async fn remove_out_chat_request(
        &self,
        requester: &UserModel,
        recipient: &PublicKey,
    ) -> ServerResult<()>;
}

impl OutChatRequestsExt for DatabaseConnection {
    #[logcall::logcall]
    async fn get_chat_request_to(
        &self,
        requester: &UserModel,
        recipient: &PublicKey,
    ) -> ServerResult<Option<OutChatRequestsModel>> {
        requester
            .find_related(OutChatRequestsEntity)
            .filter(OutChatRequestsColumn::Recipient.eq(recipient.to_string()))
            .one(self)
            .await
            .map_err(Into::into)
    }

    #[logcall::logcall]
    async fn save_out_chat_request(
        &self,
        requester: &UserModel,
        recipient: &PublicKey,
    ) -> ServerResult<()> {
        if let Err(err) = (OutChatRequestsActiveModel {
            sender_id: Set(requester.id),
            recipient: Set(recipient.to_string()),
            out_on: Set(Utc::now()),
            ..Default::default()
        }
        .save(self)
        .await)
        {
            match err.sql_err() {
                Some(SqlErr::UniqueConstraintViolation(_)) => {
                    return Err(WsError::AlreadySendChatRequest.into());
                }
                _ => return Err(err.into()),
            }
        }

        Ok(())
    }

    async fn remove_out_chat_request(
        &self,
        requester: &UserModel,
        recipient: &PublicKey,
    ) -> ServerResult<()> {
        if let Some(out_model) = self.get_chat_request_to(requester, recipient).await? {
            out_model.delete(self).await?;
        }
        Ok(())
    }
}
