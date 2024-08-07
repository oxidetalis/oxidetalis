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

//! Oxidetalis server routes, all the routes of the server.

use std::env;
use std::sync::Arc;

use oxidetalis_config::Config;
use salvo::http::ResBody;
use salvo::oapi::{Info, License};
use salvo::rate_limiter::{BasicQuota, FixedGuard, MokaStore, RateLimiter, RemoteIpIssuer};
use salvo::{catcher::Catcher, logging::Logger, prelude::*};

use crate::nonce::NonceCache;
use crate::schemas::MessageSchema;
use crate::{middlewares, websocket};

mod errors;
mod user;

pub use errors::*;

pub fn write_json_body(res: &mut Response, json_body: impl serde::Serialize) {
    res.write_body(serde_json::to_string(&json_body).expect("Json serialization can't be fail"))
        .ok();
}

#[handler]
async fn handle404(res: &mut Response, ctrl: &mut FlowCtrl) {
    if res.status_code == Some(StatusCode::NOT_FOUND) {
        write_json_body(res, MessageSchema::new("Not Found".to_owned()));
        ctrl.skip_rest();
    }
}

#[handler]
async fn handle_server_errors(res: &mut Response, ctrl: &mut FlowCtrl) {
    log::info!("New response catched: {res:#?}");
    if matches!(res.status_code, Some(status) if !status.is_success()) {
        if res.status_code == Some(StatusCode::TOO_MANY_REQUESTS) {
            write_json_body(
                res,
                MessageSchema::new("Too many requests, please try again later".to_owned()),
            );
            ctrl.skip_rest();
        } else if let ResBody::Error(err) = &res.body {
            log::error!("Error: {err}");
            write_json_body(
                res,
                MessageSchema::new(format!(
                    "{}, {}: {}",
                    err.name,
                    err.brief.trim_end_matches('.'),
                    err.cause
                        .as_deref()
                        .map_or_else(String::new, ToString::to_string)
                        .trim_end_matches('.')
                        .split(':')
                        .last()
                        .unwrap_or_default()
                        .trim()
                )),
            );
            ctrl.skip_rest();
        } else {
            log::warn!("Unknown error uncatched: {res:#?}");
        }
    } else {
        log::warn!("Unknown response uncatched: {res:#?}");
    }
}

/// Hoop a middleware if the condation is true
fn hoop_if(router: Router, middleware: impl Handler, condation: bool) -> Router {
    if condation {
        router.hoop(middleware)
    } else {
        router
    }
}

/// Create the ratelimit middleware
fn ratelimiter(
    config: &Config,
) -> RateLimiter<FixedGuard, MokaStore<String, FixedGuard>, RemoteIpIssuer, BasicQuota> {
    RateLimiter::new(
        FixedGuard::new(),
        MokaStore::<String, FixedGuard>::new(),
        RemoteIpIssuer,
        BasicQuota::set_seconds(config.ratelimit.limit, config.ratelimit.period_secs as i64),
    )
    .add_headers(true)
}

/// Create openapi and its viewer, and unshift them
fn route_openapi(config: &Config, router: Router) -> Router {
    if config.openapi.enable {
        let openapi = OpenApi::new(&config.openapi.title, env!("CARGO_PKG_VERSION"))
            .info(
                Info::new(&config.openapi.title, env!("CARGO_PKG_VERSION"))
                    .license(
                        License::new("AGPL-3.0-or-later").url("https://gnu.org/licenses/agpl-3.0"),
                    )
                    .description(&config.openapi.description),
            )
            .merge_router(&router);
        let router = router
            .unshift(openapi.into_router(&config.openapi.path))
            .unshift(config.openapi.viewer.into_router(config));
        return router;
    }
    router
}

pub fn service(conn: sea_orm::DatabaseConnection, config: &Config) -> Service {
    let nonce_cache: NonceCache = NonceCache::new(&config.server.nonce_cache_size);
    log::info!(
        "Nonce cache created with a capacity of {}",
        config.server.nonce_cache_size
    );

    let router = Router::new()
        .push(Router::with_path("user").push(user::route()))
        .push(Router::with_path("ws").push(websocket::route()))
        .hoop(middlewares::add_server_headers)
        .hoop(Logger::new())
        .hoop(
            affix::inject(Arc::new(conn))
                .inject(Arc::new(config.clone()))
                .inject(Arc::new(nonce_cache)),
        );

    let router = hoop_if(router, ratelimiter(config), config.ratelimit.enable);
    let router = route_openapi(config, router);

    Service::new(router).catcher(
        Catcher::default()
            .hoop(middlewares::add_server_headers)
            .hoop(handle404)
            .hoop(handle_server_errors),
    )
}
