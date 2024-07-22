// OxideTalis homeserver configurations
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

//! Oxidetalis config types

use std::{fmt, str::FromStr};

use salvo_oapi::{rapidoc::RapiDoc, redoc::ReDoc, scalar::Scalar, swagger_ui::SwaggerUi};
use serde::{Deserialize, Serialize};

/// OpenApi viewers, the viewers that can be used to view the OpenApi
/// documentation
#[derive(Debug, Clone, Deserialize, Serialize, clap::ValueEnum)]
#[serde(rename_all = "PascalCase")]
pub enum OpenApiViewer {
    /// Redoc viewer <https://github.com/rapi-doc/RapiDoc>
    RapiDoc,
    /// Redoc viewer <https://github.com/Redocly/redoc>
    ReDoc,
    /// Scalar viewer <https://github.com/ScalaR/ScalaR>
    Scalar,
    /// Swagger-UI viewer <https://github.com/swagger-api/swagger-ui>
    SwaggerUi,
}

/// Host type, a wrapper around `url::Host`
///
/// Because `url::Host` does not implement `FromStr`, we need to wrap it
/// in a newtype to implement `FromStr` for it.
#[derive(Debug, Clone)]
pub struct Host(pub url::Host);

impl FromStr for Host {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It appears that @SimonSapin prefers not to use the `FromStr` trait.
        // Instead, he is implementing `parse` without utilizing `FromStr`.
        //
        // - <https://github.com/servo/rust-url/pull/18#issuecomment-53467026>
        // - <https://github.com/servo/rust-url/pull/107#issuecomment-100611345>
        // - <https://github.com/servo/rust-url/issues/286#issuecomment-284193315>
        Ok(Self(url::Host::parse(s)?))
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl OpenApiViewer {
    /// Create a router for the viewer
    pub fn into_router(&self, config: &crate::Config) -> salvo_core::Router {
        let spec_url = config.openapi.path.clone();
        let title = config.openapi.title.clone();
        let description = config.openapi.description.clone();

        match self {
            OpenApiViewer::RapiDoc => {
                RapiDoc::new(spec_url)
                    .title(title)
                    .description(description)
                    .into_router(&config.openapi.viewer_path)
            }
            OpenApiViewer::ReDoc => {
                ReDoc::new(spec_url)
                    .title(title)
                    .description(description)
                    .into_router(&config.openapi.viewer_path)
            }
            OpenApiViewer::Scalar => {
                Scalar::new(spec_url)
                    .title(title)
                    .description(description)
                    .into_router(&config.openapi.viewer_path)
            }
            OpenApiViewer::SwaggerUi => {
                SwaggerUi::new(spec_url)
                    .title(title)
                    .description(description)
                    .into_router(&config.openapi.viewer_path)
            }
        }
    }
}
