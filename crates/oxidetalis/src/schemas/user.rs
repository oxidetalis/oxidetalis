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

use oxidetalis_core::{cipher::K256Secret, types::PublicKey};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

/// The schema for the user registration request
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema, derive_new::new)]
#[salvo(schema(name = RegisterUserBody, example = json!(RegisterUserBody::new(K256Secret::new().pubkey()))))]
pub struct RegisterUserBody {
    /// The public key of the user
    pub public_key: PublicKey,
}
