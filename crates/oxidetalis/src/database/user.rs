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

//! Functions for interacting with the user table in the database.

use logcall::logcall;
use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use sea_orm::DatabaseConnection;

use crate::{errors::ServerResult, routes::ApiError};

pub trait UserTableExt {
    /// Returns true if there is users in the database
    async fn users_exists_in_database(&self) -> ServerResult<bool>;
    /// Register new user
    async fn register_user(&self, public_key: &PublicKey, is_admin: bool) -> ServerResult<()>;
    /// Returns user by its public key
    async fn get_user_by_pubk(&self, public_key: &PublicKey) -> ServerResult<Option<UserModel>>;
}

impl UserTableExt for DatabaseConnection {
    #[logcall]
    async fn users_exists_in_database(&self) -> ServerResult<bool> {
        UserEntity::find()
            .one(self)
            .await
            .map(|u| u.is_some())
            .map_err(Into::into)
    }

    #[logcall]
    async fn register_user(&self, public_key: &PublicKey, is_admin: bool) -> ServerResult<()> {
        if let Err(err) = (UserActiveModel {
            public_key: Set(*public_key),
            is_admin: Set(is_admin),
            ..Default::default()
        })
        .save(self)
        .await
        {
            if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
                return Err(ApiError::AlreadyRegistered.into());
            }
        }

        Ok(())
    }

    #[logcall]
    async fn get_user_by_pubk(&self, public_key: &PublicKey) -> ServerResult<Option<UserModel>> {
        UserEntity::find()
            .filter(UserColumn::PublicKey.eq(public_key))
            .one(self)
            .await
            .map_err(Into::into)
    }
}
