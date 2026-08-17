// Copyright 2018 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! QUIC Header Protection.
//!
//! See draft-ietf-quic-tls.

use crate::cipher::aes::encrypt_block;
use crate::cipher::block;
use crate::cipher::chacha::encrypt_block_chacha20;
use crate::cipher::key::SymmetricCipherKey;
use crate::hkdf::KeyType;
use crate::{derive_debug_via_id, error, hkdf};

/// A key for generating QUIC Header Protection masks.
pub struct HeaderProtectionKey {
    inner: SymmetricCipherKey,
    algorithm: &'static Algorithm,
}

impl From<hkdf::Okm<'_, &'static Algorithm>> for HeaderProtectionKey {
    fn from(okm: hkdf::Okm<&'static Algorithm>) -> Self {
        let mut key_bytes = [0; super::MAX_KEY_LEN];
        let algorithm = *okm.len();
        let key_bytes = &mut key_bytes[..algorithm.key_len()];
        okm.fill(key_bytes).unwrap();
        Self::new(algorithm, key_bytes).unwrap()
    }
}

impl HeaderProtectionKey {
    /// Create a new header protection key.
    ///
    /// # Errors
    /// `error::Unspecified` when `key_bytes` length is not `algorithm.key_len`
    pub fn new(
        algorithm: &'static Algorithm,
        key_bytes: &[u8],
    ) -> Result<Self, error::Unspecified> {
        Ok(Self {
            inner: (algorithm.init)(key_bytes)?,
            algorithm,
        })
    }

    /// Generate a new QUIC Header Protection mask.
    ///
    /// # Errors
    /// `error::Unspecified` when `sample` length is not `self.algorithm().sample_len()`.
    #[inline]
    pub fn new_mask(&self, sample: &[u8]) -> Result<[u8; 5], error::Unspecified> {
        let sample = <&[u8; SAMPLE_LEN]>::try_from(sample)?;

        cipher_new_mask(&self.inner, *sample)
    }

    /// The key's algorithm.
    #[inline]
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.algorithm
    }
}

const SAMPLE_LEN: usize = super::TAG_LEN;

/// QUIC sample for new key masks
pub type Sample = [u8; SAMPLE_LEN];

/// A QUIC Header Protection Algorithm.
pub struct Algorithm {
    init: fn(key: &[u8]) -> Result<SymmetricCipherKey, error::KeyRejected>,

    key_len: usize,
    id: AlgorithmID,
}

impl KeyType for &'static Algorithm {
    #[inline]
    fn len(&self) -> usize {
        self.key_len()
    }
}

impl Algorithm {
    /// The length of the key.
    #[inline]
    #[must_use]
    pub fn key_len(&self) -> usize {
        self.key_len
    }

    /// The required sample length.
    #[inline]
    #[must_use]
    pub fn sample_len(&self) -> usize {
        SAMPLE_LEN
    }
}

derive_debug_via_id!(Algorithm);

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
enum AlgorithmID {
    AES_128,
    AES_256,
    CHACHA20,
}

impl PartialEq for Algorithm {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Algorithm {}

/// AES-128.
pub const AES_128: Algorithm = Algorithm {
    key_len: 16,
    init: SymmetricCipherKey::aes128,
    id: AlgorithmID::AES_128,
};

/// AES-256.
pub const AES_256: Algorithm = Algorithm {
    key_len: 32,
    init: SymmetricCipherKey::aes256,
    id: AlgorithmID::AES_256,
};

/// `ChaCha20`.
pub const CHACHA20: Algorithm = Algorithm {
    key_len: 32,
    init: SymmetricCipherKey::chacha20,
    id: AlgorithmID::CHACHA20,
};

#[inline]
fn cipher_new_mask(
    cipher_key: &SymmetricCipherKey,
    sample: Sample,
) -> Result<[u8; 5], error::Unspecified> {
    let block = block::Block::from(sample);

    let encrypted_block = match cipher_key {
        SymmetricCipherKey::Aes128 { enc_key, .. }
        | SymmetricCipherKey::Aes192 { enc_key, .. }
        | SymmetricCipherKey::Aes256 { enc_key, .. } => encrypt_block(enc_key, block),
        SymmetricCipherKey::ChaCha20 { raw_key } => {
            let plaintext = block.as_ref();
            let counter_bytes: &[u8; 4] = plaintext[0..=3]
                .try_into()
                .map_err(|_| error::Unspecified)?;
            let nonce: &[u8; 12] = plaintext[4..=15]
                .try_into()
                .map_err(|_| error::Unspecified)?;
            let input = block::Block::zero();
            let counter = u32::from_ne_bytes(*counter_bytes).to_le();
            encrypt_block_chacha20(raw_key, input, nonce, counter)?
        }
        // DES variants cannot reach this branch: `Algorithm::init` is private
        // and only the AES_128/AES_256/CHACHA20 consts populate it, so a
        // `HeaderProtectionKey` can never be constructed with a DES
        // `SymmetricCipherKey`. The arm exists only to satisfy match
        // exhaustiveness when the `legacy-des` feature is enabled.
        #[cfg(feature = "legacy-des")]
        SymmetricCipherKey::Des { .. }
        | SymmetricCipherKey::DesEde { .. }
        | SymmetricCipherKey::DesEde3 { .. } => {
            unreachable!("DES cipher keys cannot be used with QUIC header protection")
        }
    };

    let mut out: [u8; 5] = [0; 5];
    out.copy_from_slice(&encrypted_block.as_ref()[..5]);
    Ok(out)
}

#[cfg(test)]
mod test {
    use crate::aead::quic::{Algorithm, HeaderProtectionKey};
    use crate::test;

    #[test]
    fn test_types() {
        test::compile_time_assert_send::<Algorithm>();
        test::compile_time_assert_sync::<Algorithm>();

        test::compile_time_assert_send::<HeaderProtectionKey>();
        test::compile_time_assert_sync::<HeaderProtectionKey>();
    }
}
