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

//! Oxidetalis server utilities, utilities shared across the crate.

use chrono::Utc;
use logcall::logcall;
use oxidetalis_config::Postgres;
use oxidetalis_core::types::Signature;

use crate::nonce::NonceCache;

/// Returns the postgres database url
#[logcall]
pub(crate) fn postgres_url(db_config: &Postgres) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.name
    )
}

/// Returns true if the given nonce a valid nonce.
pub(crate) async fn is_valid_nonce(signature: &Signature, nonce_cache: &NonceCache) -> bool {
    let new_timestamp = Utc::now()
        .timestamp()
        .checked_sub(u64::from_be_bytes(*signature.timestamp()) as i64)
        .is_some_and(|n| n <= 20);
    let unused_nonce = new_timestamp && nonce_cache.add_nonce(signature.nonce()).await;
    new_timestamp && unused_nonce
}
