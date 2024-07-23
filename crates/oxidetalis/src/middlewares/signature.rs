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

//! Request signature middleware.

use salvo::{
    handler,
    http::{Body, StatusCode},
    Depot,
    FlowCtrl,
    Request,
    Response,
};

use crate::{extensions::DepotExt, utils};

/// Middleware to check the signature of the request.
///
/// If the signature is valid, the request will be passed to the next handler.
/// Otherwise, a 401 Unauthorized response will be returned.
#[handler]
pub async fn signature_check(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
    ctrl: &mut FlowCtrl,
) {
    const UNAUTHORIZED: StatusCode = StatusCode::UNAUTHORIZED;
    let mut write_err =
        |message: &str, status_code| super::write_error(res, ctrl, message.to_owned(), status_code);

    let data = if req.body().is_end_stream() {
        format!("{}{}", req.method(), req.uri().path())
    } else {
        match req.parse_json::<serde_json::Value>().await {
            Ok(j) => j.to_string(),
            Err(err) => {
                write_err(&err.to_string(), UNAUTHORIZED);
                return;
            }
        }
    };

    let signature = match utils::extract_signature(req) {
        Ok(s) => s,
        Err(err) => {
            write_err(&err.to_string(), UNAUTHORIZED);
            return;
        }
    };

    let sender_public_key = match utils::extract_public_key(req) {
        Ok(k) => k,
        Err(err) => {
            write_err(&err.to_string(), UNAUTHORIZED);
            return;
        }
    };

    if !utils::is_valid_nonce(&signature, &depot.nonce_cache()).await
        || !depot.config().server.private_key.verify(
            data.as_bytes(),
            &signature,
            &sender_public_key,
        )
    {
        write_err("Invalid signature", UNAUTHORIZED);
        return;
    }
}
