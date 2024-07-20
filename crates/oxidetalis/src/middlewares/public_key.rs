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

//! Request sender public key middleware.

use salvo::{handler, http::StatusCode, FlowCtrl, Request, Response};

use crate::utils;

/// Middleware to check the public key of the request sender.
///
/// If the public key is valid, the request will be passed to the next handler.
/// Otherwise, a 401 Unauthorized response will be returned.
#[handler]
pub async fn public_key_check(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Err(err) = utils::extract_public_key(req) {
        super::write_error(res, ctrl, err.to_string(), StatusCode::UNAUTHORIZED)
    }
}
