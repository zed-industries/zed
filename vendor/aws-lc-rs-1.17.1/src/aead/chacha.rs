// Copyright 2016 Brian Smith.
// Portions Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aead::aead_ctx::AeadCtx;
use crate::aead::{Algorithm, AlgorithmID};
use crate::cipher::chacha::KEY_LEN;
use crate::error;

/// ChaCha20-Poly1305 as described in [RFC 7539].
///
/// The keys are 256 bits long and the nonces are 96 bits long.
///
/// [RFC 7539]: https://tools.ietf.org/html/rfc7539
pub const CHACHA20_POLY1305: Algorithm = Algorithm {
    init: init_chacha_aead,
    key_len: KEY_LEN,
    id: AlgorithmID::CHACHA20_POLY1305,
    max_input_len: u64::MAX,
};

#[inline]
fn init_chacha_aead(key: &[u8], tag_len: usize) -> Result<AeadCtx, error::Unspecified> {
    AeadCtx::chacha20(key, tag_len)
}
