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

use std::{net::IpAddr, str::FromStr};

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

/// Type hold url or ip (used for database host)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpOrUrl(#[serde(with = "crate::serde_with::ip_or_url")] String);

impl Default for IpOrUrl {
    fn default() -> Self {
        IpOrUrl("localhost".to_owned())
    }
}

impl IpOrUrl {
    /// Returns &str ip or url
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for IpOrUrl {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IpOrUrl(
            if let Ok(res) = IpAddr::from_str(s).map(|i| i.to_string()) {
                res
            } else {
                validate_domain(s)?
            },
        ))
    }
}

fn validate_domain(domain: &str) -> Result<String, String> {
    if domain != "localhost" {
        let subs = domain.split('.');
        for sub in subs {
            let length = sub.chars().count();
            if !sub.chars().all(|c| c.is_alphanumeric() || c == '-')
                || sub.starts_with('-')
                || sub.ends_with('-')
                || (length > 0 && length <= 64)
            {
                return Err("Invalid domain name".to_owned());
            }
        }
    }
    Ok(domain.to_owned())
}
