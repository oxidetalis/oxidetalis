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

//! Entity for `out_chat_requests` table

use chrono::Utc;
use oxidetalis_core::types::PublicKey;
use sea_orm::entity::prelude::*;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "out_chat_requests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id:        IdCol,
    pub sender_id: IdCol,
    /// Public key of the recipient
    pub recipient: PublicKey,
    /// The timestamp of the request, when it was sent
    pub out_on:    chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UserEntity",
        from = "Column::SenderId",
        to = "super::users::Column::Id"
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    SenderId,
}

impl Related<UserEntity> for Entity {
    fn to() -> RelationDef {
        Relation::SenderId.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
