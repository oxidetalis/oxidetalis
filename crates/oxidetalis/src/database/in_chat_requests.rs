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

//! Database extension for the `in_chat_requests` table.

use chrono::Utc;
use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use sea_orm::{sea_query::OnConflict, DatabaseConnection};

use crate::errors::ServerResult;

/// Extension trait for the `in_chat_requests` table.
pub trait InChatRequestsExt {
    /// Save the chat request in the recipient table
    async fn save_in_chat_request(
        &self,
        requester: &PublicKey,
        recipient: &UserModel,
    ) -> ServerResult<()>;
}

impl InChatRequestsExt for DatabaseConnection {
    #[logcall::logcall]
    async fn save_in_chat_request(
        &self,
        sender: &PublicKey,
        recipient: &UserModel,
    ) -> ServerResult<()> {
        InChatRequestsEntity::insert(InChatRequestsActiveModel {
            recipient_id: Set(recipient.id),
            sender: Set(sender.to_string()),
            in_on: Set(Utc::now()),
            ..Default::default()
        })
        .on_conflict(
            OnConflict::columns([
                InChatRequestsColumn::RecipientId,
                InChatRequestsColumn::Sender,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec(self)
        .await?;
        Ok(())
    }
}
