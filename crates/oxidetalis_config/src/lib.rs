// OxideTalis homeserver configurations
// Copyright (c) 2024 OxideTalis Developers <otmp@4rs.nl>
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
#![doc = include_str!("../README.md")]

use std::{fs, io::Error as IoError, net::IpAddr, path::Path};

use derivative::Derivative;
use oxidetalis_core::types::{PrivateKey, Size};
use serde::{Deserialize, Serialize};
use toml::{de::Error as TomlDeError, ser::Error as TomlSerError};

mod commandline;
mod defaults;
mod serde_with;
mod types;

pub use clap::Parser;
pub use commandline::CliArgs;
pub use types::*;

/// Configuration errors
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("IO: {0}")]
    IO(#[from] IoError),
    #[error("Toml error: {0}")]
    DeToml(#[from] TomlDeError),
    #[error("Toml error: {0}")]
    SeToml(#[from] TomlSerError),
    #[error("Missing required option `--{0}`")]
    RequiredConfiguration(String),
}

/// Server startup configuration
#[derive(Deserialize, Serialize, Derivative, Clone)]
#[derivative(Default)]
#[serde(default)]
pub struct Server {
    /// Name of the server, for example, `example.com`
    #[derivative(Default(value = "defaults::server::name()"))]
    pub server_name:      String,
    /// Host that the server will listen in
    #[derivative(Default(value = "defaults::server::host()"))]
    pub host:             IpAddr,
    /// Port that the server will listen in
    #[derivative(Default(value = "defaults::server::port()"))]
    pub port:             u16,
    /// Server keypair
    #[derivative(Default(value = "defaults::server::private_key()"))]
    pub private_key:      PrivateKey,
    /// Nonce cache limit
    #[derivative(Default(value = "defaults::server::nonce_cache_size()"))]
    pub nonce_cache_size: Size,
}

/// Registration config
#[derive(Debug, Deserialize, Serialize, Derivative, Clone)]
#[derivative(Default)]
#[serde(default)]
pub struct Register {
    /// Whether to enable the registration or not
    #[derivative(Default(value = "defaults::bool_false()"))]
    pub enable: bool,
}

/// Database configuration
#[derive(Debug, Deserialize, Serialize, Derivative, Clone)]
#[derivative(Default)]
#[serde(default)]
pub struct Postgres {
    /// Username
    #[derivative(Default(value = "defaults::postgres::user()"))]
    pub user:     String,
    /// User password
    #[derivative(Default(value = "defaults::postgres::password()"))]
    pub password: String,
    /// Database host
    #[derivative(Default(value = "defaults::postgres::host()"))]
    pub host:     IpOrUrl,
    /// Database port
    #[derivative(Default(value = "defaults::postgres::port()"))]
    pub port:     u16,
    /// Database name
    #[derivative(Default(value = "defaults::postgres::name()"))]
    pub name:     String,
}

/// Ratelimit configuration
#[derive(Debug, Deserialize, Serialize, Derivative, Clone)]
#[derivative(Default)]
#[serde(default)]
pub struct Ratelimit {
    /// Whether to enable the ratelimit or not
    #[derivative(Default(value = "defaults::bool_true()"))]
    pub enable:      bool,
    /// The limit of requests.
    #[derivative(Default(value = "defaults::ratelimit::limit()"))]
    pub limit:       usize,
    /// The period of requests.
    #[derivative(Default(value = "defaults::ratelimit::period_secs()"))]
    pub period_secs: usize,
}

/// OpenApi configuration
#[derive(Debug, Deserialize, Serialize, Derivative, Clone)]
#[derivative(Default)]
#[serde(default)]
pub struct OpenApi {
    /// Whether to enable the openapi or not
    #[derivative(Default(value = "defaults::bool_false()"))]
    pub enable:      bool,
    /// Title of the openapi
    #[derivative(Default(value = "defaults::openapi::title()"))]
    pub title:       String,
    /// Description of the openapi
    #[derivative(Default(value = "defaults::openapi::description()"))]
    pub description: String,
    /// Location to serve openapi json in
    #[derivative(Default(value = "defaults::openapi::path()"))]
    #[serde(deserialize_with = "serde_with::deserialize_url_path")]
    pub path:        String,
    /// The openapi viewer
    #[derivative(Default(value = "defaults::openapi::viewer()"))]
    pub viewer:      types::OpenApiViewer,
    /// Location to server the viewer in
    #[derivative(Default(value = "defaults::openapi::viewer_path()"))]
    #[serde(deserialize_with = "serde_with::deserialize_url_path")]
    pub viewer_path: String,
}

#[derive(Deserialize, Serialize, Default, Clone)]
/// Oxidetalis homeserver configurations
pub struct Config {
    /// Server configuration (server startup configuration)
    #[serde(default)]
    pub server:     Server,
    /// Server registration configuration
    #[serde(default)]
    pub register:   Register,
    /// Database configuration
    pub postgresdb: Postgres,
    /// Ratelimit configuration
    #[serde(default)]
    pub ratelimit:  Ratelimit,
    /// OpenApi configuration
    #[serde(default)]
    pub openapi:    OpenApi,
}

/// Check if required new configuration options are provided
fn check_required_new_config(args: &CliArgs) -> Result<(), Error> {
    log::info!("Checking the required options for the new configuration");
    if args.server_name.is_none() {
        return Err(Error::RequiredConfiguration("server-name".to_owned()));
    }
    Ok(())
}

impl Config {
    /// Load the config from toml file and command-line options
    ///
    /// The priority is:
    /// 1. Command-line options
    /// 2. Environment variables
    /// 3. Configuration file
    /// 4. Default values (or ask you to provide the value)
    ///
    /// ## Errors
    /// - Failed to read the config file
    /// - Invalid toml file
    pub fn load(args: CliArgs) -> Result<Self, Error> {
        let mut config = if args.config.exists() {
            log::info!("Loading configuration from {}", args.config.display());
            toml::from_str(&fs::read_to_string(&args.config)?)?
        } else {
            log::info!("Configuration file not found, creating a new one");
            check_required_new_config(&args)?;
            if let Some(parent) = args.config.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            Config::default()
        };

        assign_option(&mut config.server.server_name, args.server_name);
        assign_option(&mut config.server.host, args.server_host);
        assign_option(&mut config.server.port, args.server_port);
        assign_option(
            &mut config.server.nonce_cache_size,
            args.server_nonce_cache_size,
        );
        assign_option(&mut config.register.enable, args.register_enable);
        assign_option(&mut config.postgresdb.host, args.postgres_host);
        assign_option(&mut config.postgresdb.port, args.postgres_port);
        assign_option(&mut config.postgresdb.user, args.postgres_user);
        assign_option(&mut config.postgresdb.password, args.postgres_password);
        assign_option(&mut config.postgresdb.name, args.postgres_name);
        assign_option(&mut config.ratelimit.enable, args.ratelimit_enable);
        assign_option(&mut config.ratelimit.limit, args.ratelimit_limit);
        assign_option(&mut config.ratelimit.period_secs, args.ratelimit_preiod);
        assign_option(&mut config.openapi.enable, args.openapi_enable);
        assign_option(&mut config.openapi.title, args.openapi_title);
        assign_option(&mut config.openapi.description, args.openapi_description);
        assign_option(&mut config.openapi.path, args.openapi_path);
        assign_option(&mut config.openapi.viewer, args.openapi_viewer);
        assign_option(&mut config.openapi.viewer_path, args.openapi_viewer_path);

        config.write(&args.config)?;
        Ok(config)
    }

    /// Write the configs to the config file
    ///
    /// ## Errors
    /// - Failed to write to the config file
    pub fn write(&self, config_file: impl AsRef<Path>) -> Result<(), Error> {
        fs::write(config_file, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}

/// Assign the command-line option to the config if it is not None
fn assign_option<T>(config: &mut T, arg: Option<T>) {
    if let Some(value) = arg {
        *config = value
    }
}
