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

//! REST API endpoints for user management

use oxidetalis_core::types::{PublicKey, Signature};
use salvo::{http::StatusCode, oapi::endpoint, writing::Json, Depot, Request, Router, Writer};

use super::{ApiError, ApiResult};
use crate::{
    database::{UserTableExt, UsersStatusExt},
    extensions::DepotExt,
    middlewares,
    parameters::Pagination,
    schemas::{BlackListedUser, EmptySchema, MessageSchema, WhiteListedUser},
    utils,
};

/// (🔓) Register a user
///
/// Register the request sender as a user in the server, the server registration
/// must be open to register a user.
#[endpoint(
    operation_id = "register",
    tags("User"),
    responses(
        (status_code = 201, description = "User registered"),
        (status_code = 400, description = "The entered public key is already registered", content_type = "application/json", body = MessageSchema),
        (status_code = 401, description = "The entered signature or public key is invalid", content_type = "application/json", body = MessageSchema),
        (status_code = 403, description = "Server registration is closed", content_type = "application/json", body = MessageSchema),
        (status_code = 429, description = "Too many requests", content_type = "application/json", body = MessageSchema),
        (status_code = 500, description = "Internal server error", content_type = "application/json", body = MessageSchema),
    ),
    parameters(
        ("X-OTMP-SIGNATURE" = Signature, Header, description = "Signature of the request"),
        ("X-OTMP-PUBLIC"    = PublicKey, Header, description = "Public key of the sender"),
    ),
)]
pub async fn register(req: &Request, depot: &mut Depot) -> ApiResult<EmptySchema> {
    let db = depot.db_conn();
    let config = depot.config();
    let public_key =
        utils::extract_public_key(req).expect("Public key should be checked in the middleware");

    if !db.users_exists_in_database().await? {
        db.register_user(&public_key, true).await?;
    } else if config.register.enable {
        db.register_user(&public_key, false).await?;
    } else {
        return Err(ApiError::RegistrationClosed);
    }

    Ok(EmptySchema::new(StatusCode::CREATED))
}

/// (🔐) Get whitelisted users
#[endpoint(
    operation_id = "whitelist",
    tags("User"),
    responses(
        (status_code = 200, description = "Returns whitelisted users", content_type = "application/json", body = Vec<WhiteListedUser>),
        (status_code = 400, description = "Wrong query parameter", content_type = "application/json", body = MessageSchema),
        (status_code = 401, description = "The entered signature or public key is invalid", content_type = "application/json", body = MessageSchema),
        (status_code = 403, description = "Not registered user, must register first", content_type = "application/json", body = MessageSchema),
        (status_code = 429, description = "Too many requests", content_type = "application/json", body = MessageSchema),
        (status_code = 500, description = "Internal server error", content_type = "application/json", body = MessageSchema),
    ),
    parameters(
        ("X-OTMP-PUBLIC"    = PublicKey, Header, description = "Public key of the sender"),
        ("X-OTMP-SIGNATURE" = Signature, Header, description = "Signature of the request"),
    ),
)]
async fn user_whitelist(
    req: &mut Request,
    depot: &mut Depot,
    pagination: Pagination,
) -> ApiResult<Json<Vec<WhiteListedUser>>> {
    let conn = depot.db_conn();
    let user = conn
        .get_user_by_pubk(
            &utils::extract_public_key(req)
                .expect("Public key should be checked in the middleware"),
        )
        .await?
        .ok_or(ApiError::NotRegisteredUser)?;
    Ok(Json(
        conn.user_whitelist(&user, pagination.page, pagination.page_size)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    ))
}

/// (🔐) Get blacklisted users
#[endpoint(
    operation_id = "blacklist",
    tags("User"),
    responses(
        (status_code = 200, description = "Returns blacklisted users", content_type = "application/json", body = Vec<BlackListedUser>),
        (status_code = 400, description = "Wrong query parameter", content_type = "application/json", body = MessageSchema),
        (status_code = 401, description = "The entered signature or public key is invalid", content_type = "application/json", body = MessageSchema),
        (status_code = 403, description = "Not registered user, must register first", content_type = "application/json", body = MessageSchema),
        (status_code = 429, description = "Too many requests", content_type = "application/json", body = MessageSchema),
        (status_code = 500, description = "Internal server error", content_type = "application/json", body = MessageSchema),
    ),
    parameters(
        ("X-OTMP-PUBLIC"    = PublicKey, Header, description = "Public key of the sender"),
        ("X-OTMP-SIGNATURE" = Signature, Header, description = "Signature of the request"),
    ),
)]
async fn user_blacklist(
    req: &mut Request,
    depot: &mut Depot,
    pagination: Pagination,
) -> ApiResult<Json<Vec<BlackListedUser>>> {
    let conn = depot.db_conn();
    let user = conn
        .get_user_by_pubk(
            &utils::extract_public_key(req)
                .expect("Public key should be checked in the middleware"),
        )
        .await?
        .ok_or(ApiError::NotRegisteredUser)?;
    Ok(Json(
        conn.user_blacklist(&user, pagination.page, pagination.page_size)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    ))
}

/// The route of the endpoints of this module
pub fn route() -> Router {
    Router::new()
        .push(Router::with_path("register").post(register))
        .push(Router::with_path("whitelist").get(user_whitelist))
        .push(Router::with_path("blacklist").get(user_blacklist))
        .hoop(middlewares::public_key_check)
        .hoop(middlewares::signature_check)
}
