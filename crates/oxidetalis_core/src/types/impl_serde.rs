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

use base58::FromBase58;
use serde::{de::Error as DeError, Deserialize, Serialize};

use super::{PrivateKey, PublicKey, Signature};

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let private_key = String::deserialize(deserializer)?
            .from_base58()
            .map_err(|_| DeError::custom("Invalid base58"))?;

        Self::try_from(
            <[u8; 32]>::try_from(private_key)
                .map_err(|_| DeError::custom("Invalid private key length, must be 32 bytes"))?,
        )
        .map_err(|_| DeError::custom("Invalid private key"))
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let public_key = String::deserialize(deserializer)?
            .from_base58()
            .map_err(|_| DeError::custom("Invalid base58"))?;

        Self::try_from(
            <[u8; 33]>::try_from(public_key)
                .map_err(|_| DeError::custom("Invalid public key length, must be 33 bytes"))?,
        )
        .map_err(|_| DeError::custom("Invalid public key"))
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let signature = hex::decode(String::deserialize(deserializer)?)
            .map_err(|_| DeError::custom("Invalid hex string"))?;
        Ok(Self::from(<[u8; 56]>::try_from(signature).map_err(
            |_| DeError::custom("Invalid signature length, must be 56 bytes"),
        )?))
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
