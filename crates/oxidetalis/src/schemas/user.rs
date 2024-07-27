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

//! User API schemas

use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

/// WhiteListed user schema, represents a whitelisted user.
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema, derive_new::new)]
#[salvo(schema(name = WhiteListedUser, example = json!(WhiteListedUser::default())))]
pub struct WhiteListedUser {
    /// User's public key
    pub public_key:     PublicKey,
    /// When the user was whitelisted
    pub whitelisted_at: DateTime<Utc>,
}

/// Blacklisted user schema, represents a blacklisted user.
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema, derive_new::new)]
#[salvo(schema(name = BlackListedUser, example = json!(BlackListedUser::default())))]
pub struct BlackListedUser {
    /// User's public key
    pub public_key:     PublicKey,
    /// When the user was blacklisted
    pub blacklisted_at: DateTime<Utc>,
}

impl Default for WhiteListedUser {
    fn default() -> Self {
        WhiteListedUser::new(
            PublicKey::from_str("bYhbrm61ov8GLZfskUYbsCLJTfaacMsuTBYgBABEH9dy").expect("is valid"),
            chrono::NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2015, 5, 16).expect("Is valid date"),
                NaiveTime::from_hms_opt(12, 17, 20).expect("Is valid time"),
            )
            .and_utc(),
        )
    }
}

impl From<UsersStatusModel> for WhiteListedUser {
    fn from(user: UsersStatusModel) -> Self {
        Self {
            public_key:     user.target,
            whitelisted_at: user.updated_at,
        }
    }
}

impl Default for BlackListedUser {
    fn default() -> Self {
        BlackListedUser::new(
            PublicKey::from_str("bYhbrm61ov8GLZfskUYbsCLJTfaacMsuTBYgBABEH9dy").expect("is valid"),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2015, 5, 16).expect("Is valid date"),
                NaiveTime::from_hms_opt(12, 17, 20).expect("Is valid time"),
            )
            .and_utc(),
        )
    }
}

impl From<UsersStatusModel> for BlackListedUser {
    fn from(user: UsersStatusModel) -> Self {
        Self {
            public_key:     user.target,
            blacklisted_at: user.updated_at,
        }
    }
}
