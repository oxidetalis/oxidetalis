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

//! Database extension to work with the whitelist table

use std::num::{NonZeroU32, NonZeroU8};

use chrono::Utc;
use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use sea_orm::DatabaseConnection;

use crate::{errors::ServerResult, websocket::errors::WsError};

/// Extension trait for the `DatabaseConnection` to work with the whitelist
/// table
pub trait UsersStatusExt {
    /// Returns true if the `whitelister` has whitelisted the
    /// `target_public_key`
    async fn is_whitelisted(
        &self,
        whitelister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<bool>;

    /// Returns true if the `blacklister` has blacklisted the
    /// `target_public_key`
    async fn is_blacklisted(
        &self,
        blacklister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<bool>;

    /// Add the `target_public_key` to the whitelist of the `whitelister` and
    /// remove it from the blacklist table (if it's there)
    async fn add_to_whitelist(
        &self,
        whitelister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<()>;

    /// Add the `target_public_key` to the blacklist of the `blacklister` and
    /// remove it from the whitelist table (if it's there)
    async fn add_to_blacklist(
        &self,
        blacklister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<()>;

    /// Remove the target from whitelist table
    // FIXME(awiteb): This method will be used when I work on decentralization, So, I'm keeping it
    // for now
    #[allow(dead_code)]
    async fn remove_from_whitelist(
        &self,
        whitelister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<()>;

    /// Remove the target from blacklist table
    // FIXME(awiteb): This method will be used when I work on decentralization, So, I'm keeping it
    // for now
    #[allow(dead_code)]
    async fn remove_from_blacklist(
        &self,
        blacklister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<()>;

    /// Returns the whitelist of the user
    async fn user_whitelist(
        &self,
        whitelister: &UserModel,
        page: NonZeroU32,
        page_size: NonZeroU8,
    ) -> ServerResult<Vec<UsersStatusModel>>;

    /// Returns the blacklist of the user
    async fn user_blacklist(
        &self,
        blacklister: &UserModel,
        page: NonZeroU32,
        page_size: NonZeroU8,
    ) -> ServerResult<Vec<UsersStatusModel>>;
}

impl UsersStatusExt for DatabaseConnection {
    #[logcall::logcall]
    async fn is_whitelisted(
        &self,
        whitelister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<bool> {
        get_user_status(
            self,
            whitelister,
            target_public_key,
            AccessStatus::Whitelisted,
        )
        .await
        .map(|u| u.is_some())
        .map_err(Into::into)
    }

    #[logcall::logcall]
    async fn is_blacklisted(
        &self,
        blacklister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<bool> {
        get_user_status(
            self,
            blacklister,
            target_public_key,
            AccessStatus::Blacklisted,
        )
        .await
        .map(|u| u.is_some())
        .map_err(Into::into)
    }

    #[logcall::logcall]
    async fn add_to_whitelist(
        &self,
        whitelister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<()> {
        if whitelister.public_key == target_public_key.to_string() {
            return Err(WsError::CannotAddSelfToWhitelist.into());
        }

        if let Some(mut user) = get_user_status(
            self,
            whitelister,
            target_public_key,
            AccessStatus::Blacklisted,
        )
        .await?
        .map(IntoActiveModel::into_active_model)
        {
            user.status = Set(AccessStatus::Whitelisted);
            user.updated_at = Set(Utc::now());
            user.update(self).await?;
        } else if let Err(err) = (UsersStatusActiveModel {
            user_id: Set(whitelister.id),
            target: Set(target_public_key.to_string()),
            status: Set(AccessStatus::Whitelisted),
            updated_at: Set(Utc::now()),
            ..Default::default()
        }
        .save(self)
        .await)
        {
            match err.sql_err() {
                Some(SqlErr::UniqueConstraintViolation(_)) => {
                    return Err(WsError::AlreadyOnTheWhitelist.into());
                }
                _ => return Err(err.into()),
            }
        }

        Ok(())
    }

    #[logcall::logcall]
    async fn add_to_blacklist(
        &self,
        blacklister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<()> {
        if blacklister.public_key == target_public_key.to_string() {
            return Err(WsError::CannotAddSelfToBlacklist.into());
        }

        if let Some(mut user) = get_user_status(
            self,
            blacklister,
            target_public_key,
            AccessStatus::Whitelisted,
        )
        .await?
        .map(IntoActiveModel::into_active_model)
        {
            user.status = Set(AccessStatus::Blacklisted);
            user.updated_at = Set(Utc::now());
            user.update(self).await?;
        } else if let Err(err) = (UsersStatusActiveModel {
            user_id: Set(blacklister.id),
            target: Set(target_public_key.to_string()),
            status: Set(AccessStatus::Blacklisted),
            updated_at: Set(Utc::now()),
            ..Default::default()
        }
        .save(self)
        .await)
        {
            match err.sql_err() {
                Some(SqlErr::UniqueConstraintViolation(_)) => {
                    return Err(WsError::AlreadyOnTheBlacklist.into());
                }
                _ => return Err(err.into()),
            }
        }

        Ok(())
    }

    #[logcall::logcall]
    async fn remove_from_whitelist(
        &self,
        whitelister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<()> {
        if let Some(target_user) = get_user_status(
            self,
            whitelister,
            target_public_key,
            AccessStatus::Whitelisted,
        )
        .await?
        {
            target_user.delete(self).await?;
        }
        Ok(())
    }

    #[logcall::logcall]
    async fn remove_from_blacklist(
        &self,
        blacklister: &UserModel,
        target_public_key: &PublicKey,
    ) -> ServerResult<()> {
        if let Some(target_user) = get_user_status(
            self,
            blacklister,
            target_public_key,
            AccessStatus::Blacklisted,
        )
        .await?
        {
            target_user.delete(self).await?;
        }
        Ok(())
    }

    async fn user_whitelist(
        &self,
        whitelister: &UserModel,
        page: NonZeroU32,
        page_size: NonZeroU8,
    ) -> ServerResult<Vec<UsersStatusModel>> {
        whitelister
            .find_related(UsersStatusEntity)
            .filter(UsersStatusColumn::Status.eq(AccessStatus::Whitelisted))
            .paginate(self, u64::from(page_size.get()))
            .fetch_page(u64::from(page.get() - 1))
            .await
            .map_err(Into::into)
    }

    async fn user_blacklist(
        &self,
        blacklister: &UserModel,
        page: NonZeroU32,
        page_size: NonZeroU8,
    ) -> ServerResult<Vec<UsersStatusModel>> {
        blacklister
            .find_related(UsersStatusEntity)
            .filter(UsersStatusColumn::Status.eq(AccessStatus::Blacklisted))
            .paginate(self, u64::from(page_size.get()))
            .fetch_page(u64::from(page.get() - 1))
            .await
            .map_err(Into::into)
    }
}

/// Returns user from user_status table by the entered and target public key
async fn get_user_status(
    conn: &DatabaseConnection,
    user: &UserModel,
    target_public_key: &PublicKey,
    status: AccessStatus,
) -> ServerResult<Option<UsersStatusModel>> {
    user.find_related(UsersStatusEntity)
        .filter(
            UsersStatusColumn::Target
                .eq(target_public_key.to_string())
                .and(UsersStatusColumn::Status.eq(status)),
        )
        .one(conn)
        .await
        .map_err(Into::into)
}
