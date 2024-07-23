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

//! Entity for `users` table

use oxidetalis_core::types::PublicKey;
use sea_orm::entity::prelude::*;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id:         UserId,
    pub public_key: PublicKey,
    pub is_admin:   bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "InChatRequestsEntity")]
    InChatRequests,
    #[sea_orm(has_many = "OutChatRequestsEntity")]
    OutChatRequests,
    #[sea_orm(has_many = "UsersStatusEntity")]
    UsersStatus,
}

impl Related<InChatRequestsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::InChatRequests.def()
    }
}

impl Related<OutChatRequestsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::OutChatRequests.def()
    }
}

impl Related<UsersStatusEntity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersStatus.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
