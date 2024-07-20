// OxideTalis Messaging Protocol homeserver core implementation
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

//! Size type. Used to represent sizes in bytes, kilobytes, megabytes, and
//! gigabytes.

use std::{fmt, str::FromStr};

use logcall::logcall;
use serde::{de::Error as DeError, Deserialize, Serialize};

/// Size type. Used to represent sizes in bytes, kilobytes, megabytes, and
/// gigabytes.
#[derive(Copy, Clone, Debug)]
pub enum Size {
    /// Byte
    B(usize),
    /// Kilobyte
    KB(usize),
    /// Megabyte
    MB(usize),
    /// Gigabyte
    GB(usize),
}

impl Size {
    /// Returns the size in bytes, regardless of the unit
    pub const fn as_bytes(&self) -> usize {
        match self {
            Size::B(n) => *n,
            Size::KB(n) => *n * 1e+3 as usize,
            Size::MB(n) => *n * 1e+6 as usize,
            Size::GB(n) => *n * 1e+9 as usize,
        }
    }

    /// Returns the unit name of the size (e.g. `B`, `KB`, `MB`, `GB`)
    pub const fn unit_name(&self) -> &'static str {
        match self {
            Size::B(_) => "B",
            Size::KB(_) => "KB",
            Size::MB(_) => "MB",
            Size::GB(_) => "GB",
        }
    }

    /// Returns the size in the unit (e.g. `2MB` -> `2`, `2GB` -> `2`)
    pub const fn size(&self) -> usize {
        match self {
            Size::B(n) | Size::KB(n) | Size::MB(n) | Size::GB(n) => *n,
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.size(), self.unit_name())
    }
}

impl FromStr for Size {
    type Err = String;

    #[logcall]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(first_alpha) = s.find(|c: char| c.is_alphabetic()) else {
            return Err("Missing unit, e.g. `2MB`".to_owned());
        };

        let (size, unit) = s.split_at(first_alpha);
        let Ok(size) = size.parse() else {
            return Err(format!("Invalid size `{size}`"));
        };
        Ok(match unit {
            "B" => Self::B(size),
            "KB" => Self::KB(size),
            "MB" => Self::MB(size),
            "GB" => Self::GB(size),
            unknown_unit => {
                return Err(format!(
                    "Unsupported unit `{unknown_unit}`, supported units are `B`, `KB`, `MB`, `GB`"
                ));
            }
        })
    }
}

impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .as_str()
            .parse()
            .map_err(DeError::custom)
    }
}

impl Serialize for Size {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
