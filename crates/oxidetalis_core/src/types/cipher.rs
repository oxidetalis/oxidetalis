// OxideTalis Messaging Protocol homeserver core implementation
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

use std::{fmt, str::FromStr};

use base58::{FromBase58, ToBase58};
use salvo_oapi::{
    schema::{
        Schema as OapiSchema,
        SchemaFormat as OapiSchemaFormat,
        SchemaType as OapiSchemaType,
    },
    ToSchema,
};

use crate::cipher::CipherError;

/// Correct length except message
const CORRECT_LENGTH: &str = "The length is correct";

/// K256 public key
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PublicKey([u8; 33]);

/// K256 private key
#[derive(Clone, Copy)]
pub struct PrivateKey([u8; 32]);

/// OTMP signature
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Signature {
    hmac_output: [u8; 32],
    timestamp:   [u8; 8],
    nonce:       [u8; 16],
}

impl PublicKey {
    /// Returns the public key as bytes
    pub const fn as_bytes(&self) -> &[u8; 33] {
        &self.0
    }
}

impl PrivateKey {
    /// Returns the private key as bytes
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Signature {
    /// Returns the hmac output from the signature
    pub const fn hmac_output(&self) -> &[u8; 32] {
        &self.hmac_output
    }

    /// Returns the timestamp from the signature
    pub const fn timestamp(&self) -> &[u8; 8] {
        &self.timestamp
    }

    /// Returns the nonce from the signature
    pub const fn nonce(&self) -> &[u8; 16] {
        &self.nonce
    }

    /// Returns the signature as bytes
    pub fn as_bytes(&self) -> [u8; 56] {
        let mut sig = [0u8; 56];
        sig[0..=31].copy_from_slice(&self.hmac_output);
        sig[32..=39].copy_from_slice(&self.timestamp);
        sig[40..=55].copy_from_slice(&self.nonce);
        sig
    }
}

/// Public key to base58 string
impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_base58())
    }
}

/// Public key to base58 string
impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_base58())
    }
}

/// Signature to hex string
impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_bytes()))
    }
}

/// Public key from base58 string
impl FromStr for PublicKey {
    type Err = CipherError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let public_key = s
            .from_base58()
            .map_err(|_| CipherError::InvalidBase58(s.to_owned()))?;
        if public_key.len() != 33 {
            return Err(CipherError::InvalidPublicKey);
        }
        Self::try_from(<[u8; 33]>::try_from(public_key).expect(CORRECT_LENGTH))
    }
}

/// Private key from base58 string
impl FromStr for PrivateKey {
    type Err = CipherError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let private_key = s
            .from_base58()
            .map_err(|_| CipherError::InvalidBase58(s.to_owned()))?;
        if private_key.len() != 32 {
            return Err(CipherError::InvalidPrivateKey);
        }

        Self::try_from(<[u8; 32]>::try_from(private_key).expect(CORRECT_LENGTH))
    }
}

/// Signature from hex string
impl FromStr for Signature {
    type Err = CipherError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let signature = hex::decode(s).map_err(|_| CipherError::InvalidHex(s.to_owned()))?;
        if signature.len() != 56 {
            return Err(CipherError::InvalidSignature);
        }
        Ok(Signature::from(
            <[u8; 56]>::try_from(signature).expect(CORRECT_LENGTH),
        ))
    }
}

impl TryFrom<[u8; 33]> for PublicKey {
    type Error = CipherError;

    fn try_from(public_key: [u8; 33]) -> Result<Self, Self::Error> {
        if k256::PublicKey::from_sec1_bytes(&public_key).is_err() {
            return Err(CipherError::InvalidPublicKey);
        }
        Ok(Self(public_key))
    }
}

impl TryFrom<[u8; 32]> for PrivateKey {
    type Error = CipherError;

    fn try_from(private_key: [u8; 32]) -> Result<Self, Self::Error> {
        if k256::NonZeroScalar::from_repr(*k256::FieldBytes::from_slice(&private_key))
            .is_none()
            .into()
        {
            return Err(CipherError::InvalidPrivateKey);
        }
        Ok(Self(private_key))
    }
}

impl From<[u8; 56]> for Signature {
    fn from(signature: [u8; 56]) -> Self {
        Self {
            hmac_output: signature[0..=31].try_into().expect(CORRECT_LENGTH),
            timestamp:   signature[32..=39].try_into().expect(CORRECT_LENGTH),
            nonce:       signature[40..=55].try_into().expect(CORRECT_LENGTH),
        }
    }
}

impl ToSchema for PublicKey {
    fn to_schema(_components: &mut salvo_oapi::Components) -> salvo_oapi::RefOr<OapiSchema> {
        salvo_oapi::Object::new()
            .schema_type(OapiSchemaType::String)
            .format(OapiSchemaFormat::Custom("base58".to_owned()))
            .into()
    }
}

impl ToSchema for Signature {
    fn to_schema(_components: &mut salvo_oapi::Components) -> salvo_oapi::RefOr<OapiSchema> {
        salvo_oapi::Object::new()
            .schema_type(OapiSchemaType::String)
            .format(OapiSchemaFormat::Custom("hex".to_owned()))
            .into()
    }
}
