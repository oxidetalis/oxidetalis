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

use salvo::{
    http::StatusCode,
    oapi::{Components as OapiComponents, EndpointOutRegister, Operation as OapiOperation},
    Response,
    Scribe,
};

use crate::{routes::write_json_body, schemas::MessageSchema};

/// Result type of the homeserver
#[allow(clippy::absolute_paths)]
pub(crate) type Result<T> = std::result::Result<T, Error>;
#[allow(clippy::absolute_paths)]
pub type ApiResult<T> = std::result::Result<T, ApiError>;

/// The homeserver errors
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Database Error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("{0}")]
    Configuration(#[from] oxidetalis_config::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Error from the database (500 Internal Server Error)
    #[error("Internal server error")]
    SeaOrm(#[from] sea_orm::DbErr),
    /// The server registration is closed (403 Forbidden)
    #[error("Server registration is closed")]
    RegistrationClosed,
    /// The entered public key is already registered (400 Bad Request)
    #[error("The entered public key is already registered")]
    DuplicatedUser,
    /// The user enterd tow different public keys
    /// one in the header and other in the request body
    /// (400 Bad Request)
    #[error("TODO")]
    TwoDifferentKeys,
}

impl ApiError {
    /// Status code of the error
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::SeaOrm(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::RegistrationClosed => StatusCode::FORBIDDEN,
            Self::DuplicatedUser | Self::TwoDifferentKeys => StatusCode::BAD_REQUEST,
        }
    }
}

impl EndpointOutRegister for ApiError {
    fn register(_: &mut OapiComponents, _: &mut OapiOperation) {}
}

impl Scribe for ApiError {
    fn render(self, res: &mut Response) {
        log::error!("Error: {self}");

        res.status_code(self.status_code());
        write_json_body(res, MessageSchema::new(self.to_string()));
    }
}
