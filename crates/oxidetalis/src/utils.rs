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

use std::str::FromStr;

use chrono::Utc;
use logcall::logcall;
use oxidetalis_config::Postgres;
use oxidetalis_core::{
    cipher::K256Secret,
    types::{PrivateKey, PublicKey, Signature},
    PUBLIC_KEY_HEADER,
    SIGNATURE_HEADER,
};
use salvo::Request;

use crate::{extensions::NonceCacheExt, NonceCache};

/// Returns the postgres database url
#[logcall]
pub(crate) fn postgres_url(db_config: &Postgres) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user,
        db_config.password,
        db_config.host.as_str(),
        db_config.port,
        db_config.name
    )
}

/// Returns true if the given nonce a valid nonce.
pub(crate) fn is_valid_nonce(
    signature: &Signature,
    nonce_cache: &NonceCache,
    nonce_cache_limit: &usize,
) -> bool {
    let new_timestamp = Utc::now()
        .timestamp()
        .checked_sub(u64::from_be_bytes(*signature.timestamp()) as i64)
        .is_some_and(|n| n <= 20);
    let unused_nonce = new_timestamp && nonce_cache.add_nonce(signature.nonce(), nonce_cache_limit);
    new_timestamp && unused_nonce
}

/// Returns true if the given signature is valid.
pub(crate) fn is_valid_signature(
    signer: &PublicKey,
    private_key: &PrivateKey,
    signature: &Signature,
    data: &[u8],
) -> bool {
    K256Secret::from_privkey(private_key).verify(data, signature, signer)
}

/// Extract the sender public key from the request
///
/// Returns the public key of the sender extracted from the request, or the
/// reason why it failed.
pub(crate) fn extract_public_key(req: &Request) -> Result<PublicKey, String> {
    req.headers()
        .get(PUBLIC_KEY_HEADER)
        .map(|v| {
            PublicKey::from_str(v.to_str().map_err(|err| err.to_string())?)
                .map_err(|err| err.to_string())
        })
        .ok_or_else(|| "The public key is missing".to_owned())?
}

/// Extract the signature from the request
///
/// Returns the signature extracted from the request, or the reason why it
/// failed.
pub(crate) fn extract_signature(req: &Request) -> Result<Signature, String> {
    req.headers()
        .get(SIGNATURE_HEADER)
        .map(|v| {
            Signature::from_str(v.to_str().map_err(|err| err.to_string())?)
                .map_err(|err| err.to_string())
        })
        .ok_or_else(|| "The signature is missing".to_owned())?
}
