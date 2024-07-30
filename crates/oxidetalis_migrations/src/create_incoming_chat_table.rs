// OxideTalis Messaging Protocol homeserver database migrations
// Copyright (C) 2024 Awiteb <a@4rs.nl>, OxideTalis Contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Migration to create the `incoming_chat` table, a table for incoming chat
//! requests and responses from other users

use sea_orm_migration::prelude::*;

use crate::create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IncomingChat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IncomingChat::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(IncomingChat::RecipientId)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-incoming_chat-users")
                            .from(IncomingChat::Table, IncomingChat::RecipientId)
                            .to(Users::Table, Users::Id)
                            .on_update(ForeignKeyAction::NoAction)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(IncomingChat::Sender).binary().not_null())
                    .col(
                        ColumnDef::new(IncomingChat::AcceptedResponse)
                            .boolean()
                            .null()
                            .default(Option::<bool>::None),
                    )
                    .col(
                        ColumnDef::new(IncomingChat::ReceivedTimestamp)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("sep_request")
                    .table(IncomingChat::Table)
                    .col(IncomingChat::RecipientId)
                    .col(IncomingChat::Sender)
                    .col(IncomingChat::AcceptedResponse)
                    .unique()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum IncomingChat {
    Table,
    Id,
    RecipientId,
    /// Public key of the sender
    Sender,
    /// Whether the chat response accepted or not
    AcceptedResponse,
    ReceivedTimestamp,
}
