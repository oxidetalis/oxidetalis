// OxideTalis Messaging Protocol homeserver database entities
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

//! Entity for `users_status` table

use chrono::Utc;
use oxidetalis_core::types::PublicKey;
use sea_orm::entity::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "access_status")]
pub enum AccessStatus {
    #[sea_orm(string_value = "whitelisted")]
    Whitelisted,
    #[sea_orm(string_value = "blacklisted")]
    Blacklisted,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users_status")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id:         IdCol,
    pub user_id:    IdCol,
    pub target:     PublicKey,
    pub status:     AccessStatus,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UserEntity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    UserId,
}

impl Related<UserEntity> for Entity {
    fn to() -> RelationDef {
        Relation::UserId.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
