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

//! REST API endpoints for user management

use oxidetalis_core::types::{PublicKey, Signature};
use salvo::{
    http::StatusCode,
    oapi::{endpoint, extract::JsonBody},
    Depot,
    Request,
    Router,
    Writer,
};

use crate::{
    database::UserTableExt,
    errors::{ApiError, ApiResult},
    extensions::DepotExt,
    middlewares,
    schemas::{EmptySchema, MessageSchema, RegisterUserBody},
    utils,
};

#[endpoint(
    operation_id = "register",
    tags("User"),
    responses(
        (status_code = 201, description = "User registered"),
        (status_code = 403, description = "Server registration is closed", content_type = "application/json", body = MessageSchema),
        (status_code = 400, description = "The public key in the header is not the same as the key in the body", content_type = "application/json", body = MessageSchema),
        (status_code = 400, description = "The entered public key is already registered", content_type = "application/json", body = MessageSchema),
        (status_code = 401, description = "The entered signature is invalid", content_type = "application/json", body = MessageSchema),
        (status_code = 401, description = "The entered public key is invalid", content_type = "application/json", body = MessageSchema),
        (status_code = 429, description = "Too many requests", content_type = "application/json", body = MessageSchema),
        (status_code = 500, description = "Internal server error", content_type = "application/json", body = MessageSchema),
    ),
    parameters(
        ("X-OTMP-SIGNATURE" = Signature, Header, description = "Signature of the request"),
        ("X-OTMP-PUBLIC"    = PublicKey, Header, description = "Public key of the sender"),
    ),
)]
pub async fn register(
    body: JsonBody<RegisterUserBody>,
    req: &Request,
    depot: &mut Depot,
) -> ApiResult<EmptySchema> {
    let body = body.into_inner();
    let db = depot.db_conn();
    let config = depot.config();

    if utils::extract_public_key(req).expect("Public key should be checked in the middleware")
        != body.public_key
    {
        return Err(ApiError::TwoDifferentKeys);
    }

    if !db.users_exists_in_database().await? {
        db.register_user(&body.public_key, true).await?;
    } else if config.register.enable {
        db.register_user(&body.public_key, false).await?;
    } else {
        return Err(ApiError::RegistrationClosed);
    }

    Ok(EmptySchema::new(StatusCode::CREATED))
}

/// The route of the endpoints of this module
pub fn route() -> Router {
    Router::new()
        .push(Router::with_path("register").post(register))
        .hoop(middlewares::public_key_check)
        .hoop(middlewares::signature_check)
}
