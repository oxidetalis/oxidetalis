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

//! Serialize and deserialize some oxidetalis configurations

use std::str::FromStr;

use serde::{de::Error as DeError, Deserialize, Deserializer, Serializer};

pub fn deserialize_url_path<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let url_path = String::deserialize(de)?;
    if !url_path.starts_with('/') || url_path.ends_with('/') {
        return Err(DeError::custom(
            "Invalid url path, must start with `/` and not ends with `/`",
        ));
    }
    Ok(url_path)
}

pub mod host {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use crate::Host;

    pub fn deserialize<'de, D>(de: D) -> Result<Host, D::Error>
    where
        D: Deserializer<'de>,
    {
        Host::from_str(&String::deserialize(de)?)
            .map_err(|_| DeError::custom("Invalid host name, invalid IPv4, IPv6 or domain name"))
    }

    pub fn serialize<S>(host: &Host, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&host.to_string())
    }
}
