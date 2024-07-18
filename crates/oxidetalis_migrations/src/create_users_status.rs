// OxideTalis Messaging Protocol homeserver core implementation
// Copyright (c) 2024 OxideTalis Developers <otmp@4rs.nl>
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

use std::fmt;

use sea_orm::sea_query::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use super::create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(AccessStatus::Name)
                    .values(vec![AccessStatus::Whitelisted, AccessStatus::Blacklisted])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UsersStatus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UsersStatus::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UsersStatus::UserId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users_status-users")
                            .from(UsersStatus::Table, UsersStatus::UserId)
                            .to(Users::Table, Users::Id)
                            .on_update(ForeignKeyAction::NoAction)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(UsersStatus::Target).string().not_null())
                    .col(
                        ColumnDef::new(UsersStatus::Status)
                            .enumeration(
                                AccessStatus::Name,
                                [AccessStatus::Whitelisted, AccessStatus::Blacklisted],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UsersStatus::UpdatedAt)
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
                    .name("sep_status")
                    .table(UsersStatus::Table)
                    .col(UsersStatus::UserId)
                    .col(UsersStatus::Target)
                    .unique()
                    .to_owned(),
            )
            .await
    }
}

enum AccessStatus {
    Name,
    Whitelisted,
    Blacklisted,
}

#[derive(DeriveIden)]
enum UsersStatus {
    Table,
    Id,
    UserId,
    /// Public key of the target
    Target,
    Status,
    UpdatedAt,
}

impl Iden for AccessStatus {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Name => "access_status",
                Self::Whitelisted => "whitelisted",
                Self::Blacklisted => "blacklisted",
            }
        )
        .expect("is a string")
    }
}
