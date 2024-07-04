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

#![doc = include_str!("../../../README.md")]
#![warn(missing_docs, unsafe_code)]

use std::{collections::HashMap, process::ExitCode, sync::Mutex};

use oxidetalis_config::{CliArgs, Parser};
use oxidetalis_migrations::MigratorTrait;
use salvo::{conn::TcpListener, Listener, Server};

mod database;
mod errors;
mod extensions;
mod middlewares;
mod routes;
mod schemas;
mod utils;
mod websocket;

/// Nonce cache type, used to store nonces for a certain amount of time
pub type NonceCache = Mutex<HashMap<[u8; 16], i64>>;

async fn try_main() -> errors::Result<()> {
    pretty_env_logger::init_timed();

    log::info!("Parsing configuration");
    let config = oxidetalis_config::Config::load(CliArgs::parse())?;
    log::info!("Configuration parsed successfully");
    log::info!("Connecting to the database");
    let connection = sea_orm::Database::connect(utils::postgres_url(&config.postgresdb)).await?;
    log::info!("Connected to the database successfully");
    oxidetalis_migrations::Migrator::up(&connection, None).await?;
    log::info!("Migrations applied successfully");

    let local_addr = format!("{}:{}", config.server.host, config.server.port);
    let acceptor = TcpListener::new(&local_addr).bind().await;
    log::info!("Server listening on http://{local_addr}");
    if config.openapi.enable {
        log::info!(
            "The openapi schema is available at http://{local_addr}{}",
            config.openapi.path
        );
        log::info!(
            "The openapi viewer is available at http://{local_addr}{}",
            config.openapi.viewer_path
        );
    }
    log::info!("Server version: {}", env!("CARGO_PKG_VERSION"));
    Server::new(acceptor)
        .serve(routes::service(connection, &config))
        .await;
    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(err) = try_main().await {
        eprintln!("{err}");
        log::error!("{err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
