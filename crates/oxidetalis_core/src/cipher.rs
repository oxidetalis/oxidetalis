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

//! The `cipher` module contains the encryption and decryption functions for the
//! OxideTalis protocol.

use std::time::{SystemTime, UNIX_EPOCH};

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hmac::Mac;
use k256::{
    ecdh::diffie_hellman,
    elliptic_curve::sec1::ToEncodedPoint,
    FieldBytes,
    NonZeroScalar,
    PublicKey,
};
use logcall::logcall;
use rand::{thread_rng, RngCore};

use crate::types::{
    PrivateKey as CorePrivateKey,
    PublicKey as CorePublicKey,
    Signature as CoreSignature,
};

/// The errors that can occur during in the cipher module.
#[derive(Debug, thiserror::Error)]
pub enum CipherError {
    /// The public key is invalid.
    #[error("Invalid Public Key")]
    InvalidPublicKey,
    /// The private key is invalid.
    #[error("Invalid Private Key")]
    InvalidPrivateKey,
    /// The signature is invalid
    #[error("Invalid signature")]
    InvalidSignature,

    /// A decryption error
    #[error("Decryption Error")]
    Decryption,
    /// Invalid base58 string
    #[error("Invalid base58 string `{0}`")]
    InvalidBase58(String),
    /// Invalid hex string
    #[error("Invalid hex string `{0}`")]
    InvalidHex(String),
}
#[allow(clippy::absolute_paths)]
type Result<T> = std::result::Result<T, CipherError>;
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
type HmacSha256 = hmac::Hmac<sha2::Sha256>;

/// An wrapper around the k256 crate to provide a simple API for ecdh key
/// exchange and keypair generation.
pub struct K256Secret {
    /// The private key scalar
    scalar:     NonZeroScalar,
    /// The public key
    public_key: PublicKey,
}

impl From<NonZeroScalar> for K256Secret {
    fn from(scalar: NonZeroScalar) -> Self {
        Self {
            public_key: PublicKey::from_secret_scalar(&scalar),
            scalar,
        }
    }
}

impl K256Secret {
    /// Generate a new random keypair, using the system random number generator.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::from(NonZeroScalar::random(&mut rand::thread_rng()))
    }

    /// Restore a keypair from a private key.
    pub fn from_privkey(private_key: &CorePrivateKey) -> Self {
        Self::from(
            Option::<NonZeroScalar>::from(NonZeroScalar::from_repr(*FieldBytes::from_slice(
                private_key.as_bytes(),
            )))
            .expect("The private key is correct"),
        )
    }

    /// Returns the public key.
    pub fn pubkey(&self) -> CorePublicKey {
        CorePublicKey::try_from(
            <[u8; 33]>::try_from(self.public_key.to_encoded_point(true).as_bytes())
                .expect("The length is correct"),
        )
        .expect("Is correct public key")
    }

    /// Returns the private key.
    pub fn privkey(&self) -> CorePrivateKey {
        CorePrivateKey::try_from(<[u8; 32]>::from(FieldBytes::from(self.scalar)))
            .expect("Correct private key")
    }

    /// Compute the shared secret with the given public key.
    pub fn shared_secret(&self, with: &CorePublicKey) -> [u8; 32] {
        let mut secret_buf = [0u8; 32];
        diffie_hellman(
            self.scalar,
            PublicKey::from_sec1_bytes(with.as_bytes())
                .expect("Correct public key")
                .as_affine(),
        )
        .extract::<sha2::Sha256>(None)
        .expand(&[], &mut secret_buf)
        .expect("The buffer size is correct");

        secret_buf
    }

    /// Encrypt a data with the shared secret.
    ///
    /// The data is encrypted using AES-256-CBC with a random IV (last 16 bytes
    /// of the ciphertext).
    pub fn encrypt_data(&self, encrypt_to: &CorePublicKey, data: &[u8]) -> Vec<u8> {
        let mut iv = [0u8; 16];
        thread_rng().fill_bytes(&mut iv);

        let mut ciphertext =
            Aes256CbcEnc::new(self.shared_secret(encrypt_to).as_slice().into(), &iv.into())
                .encrypt_padded_vec_mut::<Pkcs7>(data);
        ciphertext.extend(&iv);
        ciphertext
    }

    /// Decrypt a data with the shared secret.
    ///
    /// The data is decrypted using AES-256-CBC with the IV being the last 16
    /// bytes of the ciphertext.
    ///
    /// ## Errors
    /// - If the data less then 16 bytes.
    /// - If the iv less then 16 bytes.
    /// - Falid to decrypt the data (invalid encrypted data)
    pub fn decrypt_data(&self, decrypt_from: &CorePublicKey, data: &[u8]) -> Result<Vec<u8>> {
        let (ciphertext, iv) =
            data.split_at(data.len().checked_sub(16).ok_or(CipherError::Decryption)?);

        if iv.len() != 16 {
            return Err(CipherError::Decryption);
        }

        Aes256CbcDec::new(
            self.shared_secret(decrypt_from).as_slice().into(),
            iv.into(),
        )
        .decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
        .map_err(|_| CipherError::Decryption)
    }

    /// Sign a data with the shared secret.
    ///
    /// The signature is exiplained in the OTMP specification.
    #[logcall]
    pub fn sign(&self, data: &[u8], sign_to: &CorePublicKey) -> CoreSignature {
        let mut time_and_nonce = [0u8; 24];
        time_and_nonce[0..=7].copy_from_slice(
            &SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime before UNIX EPOCH!")
                .as_secs()
                .to_be_bytes(),
        );
        thread_rng().fill_bytes(&mut time_and_nonce[8..=23]);

        let mut hmac_secret = [0u8; 56];
        hmac_secret[0..=31].copy_from_slice(&self.shared_secret(sign_to));
        hmac_secret[32..=55].copy_from_slice(&time_and_nonce);
        let mut signature = [0u8; 56];
        signature[0..=31].copy_from_slice(&hmac_sha256(data, &hmac_secret));
        signature[32..=55].copy_from_slice(&time_and_nonce);

        CoreSignature::from(signature)
    }

    /// Verify a signature with the shared secret.
    ///
    /// Note:
    /// The time and the nonce will not be checked here
    #[logcall]
    pub fn verify(&self, data: &[u8], signature: &CoreSignature, signer: &CorePublicKey) -> bool {
        let mut hmac_secret = [0u8; 56];
        hmac_secret[0..=31].copy_from_slice(&self.shared_secret(signer));
        hmac_secret[32..=39].copy_from_slice(signature.timestamp());
        hmac_secret[40..=55].copy_from_slice(signature.nonce());

        &hmac_sha256(data, &hmac_secret) == signature.hmac_output()
    }
}

fn hmac_sha256(data: &[u8], secret: &[u8]) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().into()
}
