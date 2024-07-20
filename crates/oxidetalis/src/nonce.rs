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

//! Nonce cache implementation

use std::{collections::HashMap, mem};

use chrono::Utc;
use oxidetalis_core::types::Size;
use tokio::sync::Mutex as TokioMutex;

/// Size of each entry in the nonce cache
pub(crate) const NONCE_ENTRY_SIZE: usize = mem::size_of::<[u8; 16]>() + mem::size_of::<i16>();
/// Size of the hashmap itself without the entrys (48 bytes)
pub(crate) const HASH_MAP_SIZE: usize = mem::size_of::<HashMap<u8, u8>>();

/// Nonce cache struct, used to store nonces for a short period of time
/// to prevent replay attacks, each nonce has a 30 seconds lifetime.
///
/// The cache will remove first 10% nonces if the cache limit is reached.
pub struct NonceCache {
    /// The nonce cache hashmap, the key is the nonce and the value is the time
    cache: TokioMutex<HashMap<[u8; 16], i64>>,
}

impl NonceCache {
    /// Creates new [`NonceCache`] instance, with the given cache limit
    pub fn new(cache_limit: &Size) -> Self {
        Self {
            cache: TokioMutex::new(HashMap::with_capacity(
                (cache_limit.as_bytes() - HASH_MAP_SIZE) / NONCE_ENTRY_SIZE,
            )),
        }
    }

    /// Add a nonce to the cache, returns `true` if the nonce is added, `false`
    /// if the nonce is already exist in the cache.
    pub async fn add_nonce(&self, nonce: &[u8; 16]) -> bool {
        let mut cache = self.cache.lock().await;
        let now = Utc::now().timestamp();
        cache.retain(|_, time| (now - *time) < 30);

        if cache.len() == cache.capacity() {
            log::warn!("Nonce cache limit reached, clearing 10% of the cache");
            let num_to_remove = cache.capacity() / 10;
            let keys: Vec<[u8; 16]> = cache.keys().copied().collect();
            for key in keys.iter().take(num_to_remove) {
                cache.remove(key);
            }
        }

        // We can use insert directly, but it's will update the value if the key is
        // already exist so we need to check if the key is already exist or not
        if cache.contains_key(nonce) {
            return false;
        }
        cache.insert(*nonce, now);
        true
    }
}
