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

//! Oxidetalis api schemas.

use salvo::{
    http::{header, StatusCode},
    oapi::{
        Components as OapiComponents,
        EndpointOutRegister,
        Operation as OapiOperation,
        ToSchema,
    },
    Response,
    Scribe,
};
use serde::{Deserialize, Serialize};

mod user;

pub use user::*;

/// Json message schema, used for returning messages to the client, the message
/// must be human readable.
///
/// # Example
/// ```json
/// {
///    "message": "Message"
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema, derive_new::new)]
#[salvo(schema(name = MessageSchema, example = json!(MessageSchema::new("Message".to_owned()))))]
pub struct MessageSchema {
    #[salvo(schema(example = "Message"))]
    message: String,
}

/// Empty schema, used for returning empty responses.
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[salvo(schema(name = EmptySchema))]
pub struct EmptySchema(u16);

impl EmptySchema {
    /// Returns empty schema with the given status code
    pub fn new(code: StatusCode) -> Self {
        Self(code.as_u16())
    }
}

impl EndpointOutRegister for EmptySchema {
    fn register(_components: &mut OapiComponents, _operation: &mut OapiOperation) {}
}

impl Scribe for EmptySchema {
    fn render(self, res: &mut Response) {
        res.status_code(StatusCode::from_u16(self.0).expect("Is correct, from new function"));
        res.headers_mut().remove(header::CONTENT_TYPE);
    }
}
