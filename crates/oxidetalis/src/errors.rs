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

//! Oxidetalis errors types, internal, api and websocket errors.

use sea_orm::DbErr;

use crate::{routes::ApiError, websocket::errors::WsError};

/// Result type of the homeserver
pub(crate) type ServerResult<T> = Result<T, ServerError>;

/// The homeserver errors
#[derive(Debug, thiserror::Error)]
pub enum InternalError {
    #[error("Database Error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("{0}")]
    Configuration(#[from] oxidetalis_config::Error),
}

#[derive(Debug, thiserror::Error)]
/// The homeserver errors
pub enum ServerError {
    /// Internal server error, should not be exposed to the user
    #[error("{0}")]
    Internal(#[from] InternalError),
    /// API error, errors happening in the API
    #[error("{0}")]
    Api(#[from] ApiError),
    /// WebSocket error, errors happening in the WebSocket
    #[error("{0}")]
    Ws(#[from] WsError),
}

impl From<DbErr> for ServerError {
    fn from(err: DbErr) -> Self {
        Self::Internal(err.into())
    }
}

impl From<ServerError> for WsError {
    fn from(err: ServerError) -> Self {
        match err {
            ServerError::Api(ApiError::NotRegisteredUser) => WsError::RegistredUserEvent,
            ServerError::Internal(_) | ServerError::Api(_) => WsError::InternalServerError,
            ServerError::Ws(err) => err,
        }
    }
}

impl From<ServerError> for ApiError {
    fn from(err: ServerError) -> Self {
        match err {
            ServerError::Ws(WsError::RegistredUserEvent) => ApiError::NotRegisteredUser,
            ServerError::Internal(_) | ServerError::Ws(_) => ApiError::Internal,
            ServerError::Api(err) => err,
        }
    }
}
