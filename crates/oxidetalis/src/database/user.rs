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

//! Functions for interacting with the user table in the database.

use logcall::logcall;
use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use sea_orm::DatabaseConnection;

use crate::errors::{ApiError, ApiResult};

pub trait UserTableExt {
    /// Returns true if there is users in the database
    async fn users_exists_in_database(&self) -> ApiResult<bool>;
    /// Register new user
    async fn register_user(&self, public_key: &PublicKey, is_admin: bool) -> ApiResult<()>;
}

impl UserTableExt for DatabaseConnection {
    #[logcall]
    async fn users_exists_in_database(&self) -> ApiResult<bool> {
        UserEntity::find()
            .one(self)
            .await
            .map_err(Into::into)
            .map(|u| u.is_some())
    }

    #[logcall]
    async fn register_user(&self, public_key: &PublicKey, is_admin: bool) -> ApiResult<()> {
        if let Err(err) = (UserActiveModel {
            public_key: Set(public_key.to_string()),
            is_admin: Set(is_admin),
            ..Default::default()
        })
        .save(self)
        .await
        {
            if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
                return Err(ApiError::DuplicatedUser);
            }
        }

        Ok(())
    }
}
