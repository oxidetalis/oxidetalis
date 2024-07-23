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

//! The config defaults value

/// Server default configs
pub(crate) mod server {
    use std::net::{IpAddr, Ipv4Addr};

    use oxidetalis_core::{cipher::K256Secret, types::Size};

    pub fn name() -> String {
        "example.com".to_owned()
    }
    pub const fn host() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    }
    pub const fn port() -> u16 {
        7294
    }
    pub fn private_key() -> K256Secret {
        K256Secret::new()
    }
    pub const fn nonce_cache_size() -> Size {
        Size::MB(1)
    }
}

/// Ratelimit default configs
pub(crate) mod ratelimit {

    pub const fn limit() -> usize {
        1500
    }
    pub const fn period_secs() -> usize {
        60
    }
}

/// OpenApi default configs
pub(crate) mod openapi {
    use crate::types;

    pub fn title() -> String {
        "Oxidetalis homeserver".to_owned()
    }
    pub fn description() -> String {
        "OxideTalis Messaging Protocol homeserver".to_owned()
    }
    pub fn path() -> String {
        "/openapi.json".to_owned()
    }
    pub const fn viewer() -> types::OpenApiViewer {
        types::OpenApiViewer::Scalar
    }
    pub fn viewer_path() -> String {
        "/scalar-ui".to_owned()
    }
}

/// Postgres default configs
pub(crate) mod postgres {

    pub fn user() -> String {
        "oxidetalis".to_owned()
    }
    pub fn password() -> String {
        "oxidetalis".to_owned()
    }
    pub const fn host() -> crate::Host {
        #[allow(clippy::absolute_paths)]
        crate::Host(url::Host::Ipv4(std::net::Ipv4Addr::new(127, 0, 0, 1)))
    }
    pub fn name() -> String {
        "oxidetalis_db".to_owned()
    }
    pub const fn port() -> u16 {
        5432
    }
}

pub(crate) const fn bool_true() -> bool {
    true
}

pub(crate) const fn bool_false() -> bool {
    false
}
