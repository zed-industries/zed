// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aead::{Algorithm, AlgorithmID};

use crate::aead::aead_ctx::AeadCtx;
use crate::cipher::aes::{AES_128_KEY_LEN, AES_192_KEY_LEN, AES_256_KEY_LEN};
use crate::error::Unspecified;

/// AES-128 in GCM mode with 128-bit tags and 96 bit nonces.
pub const AES_128_GCM: Algorithm = Algorithm {
    init: init_128_aead,
    key_len: AES_128_KEY_LEN,
    id: AlgorithmID::AES_128_GCM,
    max_input_len: u64::MAX,
};

/// AES-192 in GCM mode with 128-bit tags and 96 bit nonces.
pub const AES_192_GCM: Algorithm = Algorithm {
    init: init_192_aead,
    key_len: AES_192_KEY_LEN,
    id: AlgorithmID::AES_192_GCM,
    max_input_len: u64::MAX,
};

/// AES-256 in GCM mode with 128-bit tags and 96 bit nonces.
pub const AES_256_GCM: Algorithm = Algorithm {
    init: init_256_aead,
    key_len: AES_256_KEY_LEN,
    id: AlgorithmID::AES_256_GCM,
    max_input_len: u64::MAX,
};

/// AES-256 in GCM mode with nonce reuse resistance, 128-bit tags and 96 bit nonces.
pub const AES_256_GCM_SIV: Algorithm = Algorithm {
    init: init_256_aead_siv,
    key_len: AES_256_KEY_LEN,
    id: AlgorithmID::AES_256_GCM_SIV,
    max_input_len: u64::MAX,
};

/// AES-128 in GCM mode with nonce reuse resistance, 128-bit tags and 96 bit nonces.
pub const AES_128_GCM_SIV: Algorithm = Algorithm {
    init: init_128_aead_siv,
    key_len: AES_128_KEY_LEN,
    id: AlgorithmID::AES_128_GCM_SIV,
    max_input_len: u64::MAX,
};

#[inline]
fn init_128_aead(key: &[u8], tag_len: usize) -> Result<AeadCtx, Unspecified> {
    AeadCtx::aes_128_gcm(key, tag_len)
}

#[inline]
fn init_192_aead(key: &[u8], tag_len: usize) -> Result<AeadCtx, Unspecified> {
    AeadCtx::aes_192_gcm(key, tag_len)
}

#[inline]
fn init_256_aead(key: &[u8], tag_len: usize) -> Result<AeadCtx, Unspecified> {
    AeadCtx::aes_256_gcm(key, tag_len)
}

#[inline]
fn init_256_aead_siv(key: &[u8], tag_len: usize) -> Result<AeadCtx, Unspecified> {
    AeadCtx::aes_256_gcm_siv(key, tag_len)
}

#[inline]
fn init_128_aead_siv(key: &[u8], tag_len: usize) -> Result<AeadCtx, Unspecified> {
    AeadCtx::aes_128_gcm_siv(key, tag_len)
}
