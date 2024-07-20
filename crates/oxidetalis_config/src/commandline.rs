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

//! Command-line arguments parser

use std::{net::IpAddr, path::PathBuf};

use clap::Parser;
use oxidetalis_core::types::Size;

use crate::{types::OpenApiViewer, IpOrUrl};

#[derive(Parser)]
#[clap(version)]
/// Command-line arguments for the Oxidetalis server.
pub struct CliArgs {
    /// Path to the configuration file, toml format.
    #[clap(long, env = "OXIDETALIS_CONFIG")]
    pub config:                  PathBuf,
    /// Server name, for example, `example.com`.
    #[clap(long, env = "OXIDETALIS_SERVER_NAME")]
    pub server_name:             Option<String>,
    /// Local IP address to bind the server to.
    #[clap(long, env = "OXIDETALIS_SERVER_HOST")]
    pub server_host:             Option<IpAddr>,
    /// Port to bind the server to.
    #[clap(long, env = "OXIDETALIS_SERVER_PORT")]
    pub server_port:             Option<u16>,
    /// Nonce cache size
    ///
    /// e.g. "50B", "300KB", "1MB", "1GB"
    #[clap(long, env = "OXIDETALIS_SERVER_NONCE_CACHE_SIZE")]
    pub server_nonce_cache_size: Option<Size>,
    /// Enable or disable user registration.
    #[clap(long, env = "OXIDETALIS_REGISTER_ENABLE")]
    pub register_enable:         Option<bool>,
    /// Hostname or IP address of the PostgreSQL database.
    #[clap(long, env = "OXIDETALIS_DB_HOST")]
    pub postgres_host:           Option<IpOrUrl>,
    /// Port number of the PostgreSQL database.
    #[clap(long, env = "OXIDETALIS_DB_PORT")]
    pub postgres_port:           Option<u16>,
    /// Username for the PostgreSQL database.
    #[clap(long, env = "OXIDETALIS_DB_USER")]
    pub postgres_user:           Option<String>,
    /// Password for the PostgreSQL database.
    #[clap(long, env = "OXIDETALIS_DB_PASSWORD")]
    pub postgres_password:       Option<String>,
    /// Name of the PostgreSQL database.
    #[clap(long, env = "OXIDETALIS_DB_NAME")]
    pub postgres_name:           Option<String>,
    /// Enable or disable rate limiting.
    #[clap(long, env = "OXIDETALIS_RATELIMIT_ENABLE")]
    pub ratelimit_enable:        Option<bool>,
    /// Maximum number of requests allowed within a given time period for rate
    /// limiting.
    #[clap(long, env = "OXIDETALIS_RATELIMIT_LIMIT")]
    pub ratelimit_limit:         Option<usize>,
    /// Time period in seconds for rate limiting.
    #[clap(long, env = "OXIDETALIS_RATELIMIT_PREIOD")]
    pub ratelimit_preiod:        Option<usize>,
    /// Enable or disable OpenAPI documentation generation.
    #[clap(long, env = "OXIDETALIS_OPENAPI_ENABLE")]
    pub openapi_enable:          Option<bool>,
    /// Title for the OpenAPI documentation.
    #[clap(long, env = "OXIDETALIS_OPENAPI_TITLE")]
    pub openapi_title:           Option<String>,
    /// Description for the OpenAPI documentation.
    #[clap(long, env = "OXIDETALIS_OPENAPI_DESCRIPTION")]
    pub openapi_description:     Option<String>,
    /// Path to serve the OpenAPI documentation.
    #[clap(long, env = "OXIDETALIS_OPENAPI_PATH")]
    pub openapi_path:            Option<String>,
    /// OpenAPI viewer to use for rendering the documentation.
    #[clap(long, env = "OXIDETALIS_OPENAPI_VIEWER")]
    pub openapi_viewer:          Option<OpenApiViewer>,
    /// Path to the OpenAPI viewer HTML file.
    #[clap(long, env = "OXIDETALIS_OPENAPI_VIEWER_PATH")]
    pub openapi_viewer_path:     Option<String>,
}
