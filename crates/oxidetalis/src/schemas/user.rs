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

use std::str::FromStr;

use chrono::{DateTime, Utc};
use oxidetalis_core::types::PublicKey;
use oxidetalis_entities::prelude::*;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema, derive_new::new)]
#[salvo(schema(name = WhiteListedUser, example = json!(WhiteListedUser::default())))]
pub struct WhiteListedUser {
    /// User's public key
    pub public_key:     PublicKey,
    /// When the user was whitelisted
    pub whitelisted_at: DateTime<Utc>,
}

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
            Utc::now(),
        )
    }
}

impl From<UsersStatusModel> for WhiteListedUser {
    fn from(user: UsersStatusModel) -> Self {
        Self {
            public_key:     PublicKey::from_str(&user.target).expect("Is valid public key"),
            whitelisted_at: user.updated_at,
        }
    }
}

impl Default for BlackListedUser {
    fn default() -> Self {
        BlackListedUser::new(
            PublicKey::from_str("bYhbrm61ov8GLZfskUYbsCLJTfaacMsuTBYgBABEH9dy").expect("is valid"),
            Utc::now(),
        )
    }
}

impl From<UsersStatusModel> for BlackListedUser {
    fn from(user: UsersStatusModel) -> Self {
        Self {
            public_key:     PublicKey::from_str(&user.target).expect("Is valid public key"),
            blacklisted_at: user.updated_at,
        }
    }
}
