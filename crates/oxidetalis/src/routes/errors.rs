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

/// Result type of the API
pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Internal server error")]
    Internal,
    /// The server registration is closed (403 Forbidden)
    #[error("Server registration is closed")]
    RegistrationClosed,
    /// The entered public key is already registered (400 Bad Request)
    #[error("The entered public key is already registered")]
    AlreadyRegistered,
    /// The user entered two different public keys
    /// one in the header and other in the request body
    /// (400 Bad Request)
    #[error("You entered two different public keys")]
    TwoDifferentKeys,
    /// Error in the query parameters (400 Bad Request)
    #[error("{0}")]
    Querys(String),
    /// Non registered user tried to access to registered user only endpoint
    /// (403 Forbidden)
    #[error("You are not a registered user, please register first")]
    NotRegisteredUser,
}

impl ApiError {
    /// Status code of the error
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::RegistrationClosed | Self::NotRegisteredUser => StatusCode::FORBIDDEN,
            Self::AlreadyRegistered | Self::TwoDifferentKeys | Self::Querys(_) => {
                StatusCode::BAD_REQUEST
            }
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
