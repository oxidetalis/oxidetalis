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

//! Serialize and deserialize some oxidetalis configurations

use serde::{de::Error as DeError, Deserialize, Deserializer};

/// Serialize and deserialze the string of IpOrUrl struct
pub(crate) mod ip_or_url {
    use std::str::FromStr;

    use serde::{de::Error as DeError, Deserialize, Deserializer, Serializer};

    use crate::IpOrUrl;

    pub fn serialize<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(value)
    }

    pub fn deserialize<'de, D>(de: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(IpOrUrl::from_str(&String::deserialize(de)?)
            .map_err(DeError::custom)?
            .as_str()
            .to_owned())
    }
}

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
