// OxideTalis Messaging Protocol homeserver core implementation
// Copyright (C) 2024 Awiteb <a@4rs.nl>, OxideTalis Contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! OpenAPI schema for some core types.

use std::str::FromStr;

use salvo_core::{extract::Metadata as ExtractMetadata, http::StatusError, Extractible, Request};
use salvo_oapi::{
    schema::{
        Schema as OapiSchema,
        SchemaFormat as OapiSchemaFormat,
        SchemaType as OapiSchemaType,
    },
    Components as OapiComponents,
    EndpointArgRegister,
    Parameter,
    ParameterIn,
    Parameters,
    ToParameters,
    ToSchema,
};

use super::{PublicKey as CorePublicKey, Signature};

impl ToSchema for CorePublicKey {
    fn to_schema(_components: &mut salvo_oapi::Components) -> salvo_oapi::RefOr<OapiSchema> {
        salvo_oapi::Object::new()
            .name(crate::PUBLIC_KEY_HEADER)
            .description("User's public key")
            .schema_type(OapiSchemaType::String)
            .format(OapiSchemaFormat::Custom("base58".to_owned()))
            .required(crate::PUBLIC_KEY_HEADER)
            // A 33-byte base58 string can be either 44 or 45 characters long
            .example("rW8FMG5D75NVNJV3Wd498dEh65BgUuhwY1Yk5zYJPpRe".into())
            .max_length(45)
            .min_length(44)
            .into()
    }
}

impl ToSchema for Signature {
    fn to_schema(_components: &mut salvo_oapi::Components) -> salvo_oapi::RefOr<OapiSchema> {
        salvo_oapi::Object::new()
            .name(crate::SIGNATURE_HEADER)
            .description("Signature of the request")
            .schema_type(OapiSchemaType::String)
            .format(OapiSchemaFormat::Custom("hex".to_owned()))
            .required(crate::SIGNATURE_HEADER)
            // 56 bytes in hex (valid signature)
            .example("0".repeat(112).into())
            .max_length(112)
            .min_length(112)
            .into()
    }
}

impl<'ex> Extractible<'ex> for CorePublicKey {
    fn metadata() -> &'ex ExtractMetadata {
        static METADATA: ExtractMetadata = ExtractMetadata::new("");
        &METADATA
    }

    #[allow(refining_impl_trait)]
    async fn extract(req: &'ex mut Request) -> Result<Self, StatusError> {
        extract_header(req, crate::PUBLIC_KEY_HEADER).and_then(|public_key| {
            CorePublicKey::from_str(public_key).map_err(|err| {
                StatusError::bad_request()
                    .brief("Invalid public key")
                    .cause(err.to_string())
            })
        })
    }

    #[allow(refining_impl_trait)]
    async fn extract_with_arg(req: &'ex mut Request, _: &str) -> Result<Self, StatusError> {
        Self::extract(req).await
    }
}

impl EndpointArgRegister for CorePublicKey {
    fn register(components: &mut OapiComponents, operation: &mut salvo_oapi::Operation, _: &str) {
        operation.parameters.insert(
            Parameter::new(crate::PUBLIC_KEY_HEADER)
                .parameter_in(ParameterIn::Header)
                .required(true)
                .description("User's public key")
                .example("2BiUSWkJUy5bcdJB8qszq9K6a5EXVHvK41vQWZVkUBUM8".into())
                .schema(CorePublicKey::to_schema(components)),
        )
    }
}

impl<'ex> Extractible<'ex> for Signature {
    fn metadata() -> &'ex ExtractMetadata {
        static METADATA: ExtractMetadata = ExtractMetadata::new("");
        &METADATA
    }

    #[allow(refining_impl_trait)]
    async fn extract(req: &'ex mut Request) -> Result<Self, StatusError> {
        extract_header(req, crate::SIGNATURE_HEADER)
            .and_then(|sig| {
                Signature::from_str(sig).map_err(|err| {
                    StatusError::unauthorized()
                        .brief("Invalid signature")
                        .cause(err.to_string())
                })
            })
            .map_err(|err| {
                StatusError::unauthorized().brief(err.brief).cause(
                    err.cause
                        .expect("The cause was set when we extract the header"),
                )
            })
    }
}

impl ToParameters<'_> for Signature {
    fn to_parameters(components: &mut OapiComponents) -> Parameters {
        Parameters::new().parameter(
            Parameter::new(crate::SIGNATURE_HEADER)
                .parameter_in(ParameterIn::Header)
                .required(true)
                .description("Signature of the request")
                .example("0".repeat(112).into())
                .schema(Self::to_schema(components)),
        )
    }
}

fn extract_header<'req>(req: &'req Request, name: &str) -> Result<&'req str, StatusError> {
    req.headers()
        .get(name)
        .map(|v| {
            v.to_str().map_err(|_| {
                StatusError::bad_request()
                    .brief("Invalid header value")
                    .cause("Header value must be a valid ascii string")
            })
        })
        .transpose()?
        .ok_or_else(|| {
            StatusError::bad_request()
                .brief(format!("Could not find {name} in headers"))
                .cause(format!(
                    "{name} is required to authenication and authorization"
                ))
        })
}
