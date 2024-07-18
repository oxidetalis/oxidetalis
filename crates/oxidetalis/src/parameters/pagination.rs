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

//! Pagination parameters for the API

use std::{
    fmt,
    num::{NonZeroU32, NonZeroU8},
    str::FromStr,
};

use salvo::{
    extract::Metadata as ExtractMetadata,
    oapi::{
        Components as OapiComponents,
        EndpointArgRegister,
        Object,
        Operation as OapiOperation,
        Parameter,
        ParameterIn,
        Parameters,
        SchemaType,
        ToParameters,
    },
    Extractible,
    Request,
};
use serde_json::json;

use crate::routes::{ApiError, ApiResult};

#[derive(Debug)]
pub struct Pagination {
    /// The page number of the result
    pub page:      NonZeroU32,
    /// The page size
    pub page_size: NonZeroU8,
}

impl<'ex> Extractible<'ex> for Pagination {
    fn metadata() -> &'ex ExtractMetadata {
        static METADATA: ExtractMetadata = ExtractMetadata::new("");
        &METADATA
    }

    #[allow(refining_impl_trait)]
    async fn extract(req: &'ex mut Request) -> ApiResult<Self> {
        let page = extract_query(req, "page", NonZeroU32::new(1).expect("is non-zero"))?;
        let page_size = extract_query(req, "page_size", NonZeroU8::new(10).expect("is non-zero"))?;

        Ok(Self { page, page_size })
    }

    #[allow(refining_impl_trait)]
    async fn extract_with_arg(req: &'ex mut Request, _arg: &str) -> ApiResult<Self> {
        Self::extract(req).await
    }
}

impl ToParameters<'_> for Pagination {
    fn to_parameters(_components: &mut OapiComponents) -> Parameters {
        Parameters::new()
            .parameter(create_parameter(
                "page",
                "Page number, starting from 1",
                1,
                f64::from(u32::MAX),
            ))
            .parameter(create_parameter(
                "page_size",
                "How many items per page, starting from 1",
                10,
                f64::from(u8::MAX),
            ))
    }
}

impl EndpointArgRegister for Pagination {
    fn register(components: &mut OapiComponents, operation: &mut OapiOperation, _arg: &str) {
        for parameter in Self::to_parameters(components) {
            operation.parameters.insert(parameter);
        }
    }
}

/// Extract a query parameter from the request
fn extract_query<T: FromStr>(req: &Request, name: &str, default_value: T) -> ApiResult<T>
where
    <T as FromStr>::Err: fmt::Display,
{
    Ok(req
        .queries()
        .get(name)
        .map(|p| p.parse::<T>())
        .transpose()
        .map_err(|err| ApiError::Querys(format!("Invalid value for `{name}` query ({err})")))?
        .unwrap_or(default_value))
}

/// Create a parameter for the pagination
fn create_parameter(name: &str, description: &str, default: usize, max: f64) -> Parameter {
    Parameter::new(name)
        .parameter_in(ParameterIn::Query)
        .required(false)
        .description(description)
        .example(json!(default))
        .schema(
            Object::new()
                .name(name)
                .description(description)
                .schema_type(SchemaType::Integer)
                .default_value(json!(default))
                .example(json!(default))
                .maximum(max)
                .minimum(1.0)
                .read_only(true),
        )
}
