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

//! Middlewares for the OxideTalis homeserver.

use salvo::{
    handler,
    http::{header, HeaderValue, StatusCode},
    FlowCtrl,
    Request,
    Response,
};

mod public_key;
mod signature;

pub use public_key::*;
pub use signature::*;

use crate::{routes::write_json_body, schemas::MessageSchema};

/// Add server headers to the response and request.
#[handler]
pub async fn add_server_headers(req: &mut Request, res: &mut Response) {
    let res_headers = res.headers_mut();
    let req_headers = req.headers_mut();
    res_headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    // Insert the accept header for salvo, so it returns JSON if there is error
    req_headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
}

/// Write an errror message in the response
pub fn write_error(
    res: &mut Response,
    ctrl: &mut FlowCtrl,
    message: String,
    status_code: StatusCode,
) {
    res.status_code(status_code);
    write_json_body(res, MessageSchema::new(message));
    ctrl.skip_rest();
}
