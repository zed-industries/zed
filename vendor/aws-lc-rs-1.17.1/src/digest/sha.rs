// Copyright 2015-2022 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::digest::{Algorithm, AlgorithmID, Context};

/// The length of a block for SHA-1, in bytes.
const SHA1_BLOCK_LEN: usize = 512 / 8;

/// The length of the output of SHA-1, in bytes.
pub const SHA1_OUTPUT_LEN: usize = 160 / 8;

/// The length of the output of SHA-224, in bytes.
pub const SHA224_OUTPUT_LEN: usize = 224 / 8;

/// The length of the output of SHA-256, in bytes.
pub const SHA256_OUTPUT_LEN: usize = 256 / 8;

/// The length of a block for SHA-256-based algorithms, in bytes.
const SHA256_BLOCK_LEN: usize = 512 / 8;

/// The length of the output of SHA-384, in bytes.
pub const SHA384_OUTPUT_LEN: usize = 384 / 8;

/// The length of the output of SHA-512, in bytes.
pub const SHA512_OUTPUT_LEN: usize = 512 / 8;

/// The length of the output of SHA-512/256, in bytes.
pub const SHA512_256_OUTPUT_LEN: usize = 256 / 8;

/// The length of a block for SHA-512-based algorithms, in bytes.
const SHA512_BLOCK_LEN: usize = 1024 / 8;

/// The length of a block for SHA3-256-based algorithms, in bytes.
const SHA3_256_BLOCK_LEN: usize = 136;

/// The length of a block for SHA3-384-based algorithms, in bytes.
const SHA3_384_BLOCK_LEN: usize = 104;

/// The length of a block for SHA3-512-based algorithms, in bytes.
const SHA3_512_BLOCK_LEN: usize = 72;

/// The length of the output of SHA3-256 in bytes.
pub const SHA3_256_OUTPUT_LEN: usize = 256 / 8;

/// The length of the output of SHA3-384, in bytes.
pub const SHA3_384_OUTPUT_LEN: usize = 384 / 8;

/// The length of the output of SHA3-512, in bytes.
pub const SHA3_512_OUTPUT_LEN: usize = 512 / 8;

/// SHA-1, SHA-224, and SHA-256 are limited to an input size of 2^64-1 bits.
/// SHA-384, SHA-512, and SHA-512/256 are limited to an input size of 2^128-1 bits according to the spec.
/// u64 is more than sufficient enough for practical usecases, so we limit the input length to 2^64-1 bits.
#[allow(clippy::cast_possible_truncation)]
const DIGEST_MAX_INPUT_LEN: u64 = u64::MAX;

/// SHA-1 as specified in [FIPS 180-4]. Deprecated.
///
/// [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
#[allow(deprecated)]
pub const SHA1_FOR_LEGACY_USE_ONLY: Algorithm = Algorithm {
    output_len: SHA1_OUTPUT_LEN,
    chaining_len: SHA1_OUTPUT_LEN,
    block_len: SHA1_BLOCK_LEN,
    max_input_len: DIGEST_MAX_INPUT_LEN,

    one_shot_hash: sha1_digest,

    id: AlgorithmID::SHA1,
};

/// SHA-224 as specified in [FIPS 180-4].
///
/// [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
#[allow(deprecated)]
pub const SHA224: Algorithm = Algorithm {
    output_len: SHA224_OUTPUT_LEN,

    // The chaining length is equivalent to the length before truncation.
    // SHA-224 is truncated from 256 bits so the chaining length is 256 bits, or 32 bytes.
    chaining_len: SHA256_OUTPUT_LEN,
    block_len: SHA256_BLOCK_LEN,
    max_input_len: DIGEST_MAX_INPUT_LEN,

    one_shot_hash: sha224_digest,

    id: AlgorithmID::SHA224,
};

/// SHA-256 as specified in [FIPS 180-4].
///
/// [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
#[allow(deprecated)]
pub const SHA256: Algorithm = Algorithm {
    output_len: SHA256_OUTPUT_LEN,
    chaining_len: SHA256_OUTPUT_LEN,
    block_len: SHA256_BLOCK_LEN,
    max_input_len: DIGEST_MAX_INPUT_LEN,

    one_shot_hash: sha256_digest,

    id: AlgorithmID::SHA256,
};

/// SHA-384 as specified in [FIPS 180-4].
///
/// [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
#[allow(deprecated)]
pub const SHA384: Algorithm = Algorithm {
    output_len: SHA384_OUTPUT_LEN,

    // The chaining length is equivalent to the length before truncation.
    // SHA-384 is truncated from 512 bits so the chaining length is 512 bits, or 64 bytes.
    chaining_len: SHA512_OUTPUT_LEN,
    block_len: SHA512_BLOCK_LEN,
    max_input_len: DIGEST_MAX_INPUT_LEN,

    one_shot_hash: sha384_digest,

    id: AlgorithmID::SHA384,
};

/// SHA-512 as specified in [FIPS 180-4].
///
/// [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
#[allow(deprecated)]
pub const SHA512: Algorithm = Algorithm {
    output_len: SHA512_OUTPUT_LEN,
    chaining_len: SHA512_OUTPUT_LEN,
    block_len: SHA512_BLOCK_LEN,
    max_input_len: DIGEST_MAX_INPUT_LEN,

    one_shot_hash: sha512_digest,

    id: AlgorithmID::SHA512,
};

/// SHA-512/256 as specified in [FIPS 180-4].
///
/// [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
#[allow(deprecated)]
pub const SHA512_256: Algorithm = Algorithm {
    output_len: SHA512_256_OUTPUT_LEN,
    chaining_len: SHA512_OUTPUT_LEN,
    block_len: SHA512_BLOCK_LEN,
    max_input_len: DIGEST_MAX_INPUT_LEN,

    one_shot_hash: sha512_256_digest,

    id: AlgorithmID::SHA512_256,
};

/// SHA3-256 as specified in [FIPS 202].
///
/// [FIPS 202]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
#[allow(deprecated)]
pub const SHA3_256: Algorithm = Algorithm {
    output_len: SHA3_256_OUTPUT_LEN,
    chaining_len: SHA3_256_OUTPUT_LEN,
    block_len: SHA3_256_BLOCK_LEN,
    max_input_len: DIGEST_MAX_INPUT_LEN,

    one_shot_hash: sha3_256_digest,

    id: AlgorithmID::SHA3_256,
};

/// SHA3-384 as specified in [FIPS 202].
///
/// [FIPS 202]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
#[allow(deprecated)]
pub const SHA3_384: Algorithm = Algorithm {
    output_len: SHA3_384_OUTPUT_LEN,
    chaining_len: SHA3_384_OUTPUT_LEN,
    block_len: SHA3_384_BLOCK_LEN,
    max_input_len: DIGEST_MAX_INPUT_LEN,

    one_shot_hash: sha3_384_digest,

    id: AlgorithmID::SHA3_384,
};

/// SHA3-512 as specified in [FIPS 202].
///
/// [FIPS 202]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
#[allow(deprecated)]
pub const SHA3_512: Algorithm = Algorithm {
    output_len: SHA3_512_OUTPUT_LEN,
    chaining_len: SHA3_512_OUTPUT_LEN,
    block_len: SHA3_512_BLOCK_LEN,
    max_input_len: DIGEST_MAX_INPUT_LEN,

    one_shot_hash: sha3_512_digest,

    id: AlgorithmID::SHA3_512,
};

fn sha1_digest(msg: &[u8], output: &mut [u8]) {
    unsafe {
        aws_lc::SHA1(msg.as_ptr(), msg.len(), output.as_mut_ptr());
    }
}

fn sha224_digest(msg: &[u8], output: &mut [u8]) {
    unsafe {
        aws_lc::SHA224(msg.as_ptr(), msg.len(), output.as_mut_ptr());
    }
}

fn sha256_digest(msg: &[u8], output: &mut [u8]) {
    unsafe {
        aws_lc::SHA256(msg.as_ptr(), msg.len(), output.as_mut_ptr());
    }
}

fn sha384_digest(msg: &[u8], output: &mut [u8]) {
    unsafe {
        aws_lc::SHA384(msg.as_ptr(), msg.len(), output.as_mut_ptr());
    }
}

fn sha512_digest(msg: &[u8], output: &mut [u8]) {
    unsafe {
        aws_lc::SHA512(msg.as_ptr(), msg.len(), output.as_mut_ptr());
    }
}

fn sha512_256_digest(msg: &[u8], output: &mut [u8]) {
    unsafe {
        aws_lc::SHA512_256(msg.as_ptr(), msg.len(), output.as_mut_ptr());
    }
}

fn sha3_256_digest(msg: &[u8], output: &mut [u8]) {
    let mut ctx = Context::new(&SHA3_256);
    ctx.update(msg);
    let digest = ctx.finish();
    output[0..SHA3_256_OUTPUT_LEN].copy_from_slice(digest.as_ref());
}

fn sha3_384_digest(msg: &[u8], output: &mut [u8]) {
    let mut ctx = Context::new(&SHA3_384);
    ctx.update(msg);
    let digest = ctx.finish();
    output[0..SHA3_384_OUTPUT_LEN].copy_from_slice(digest.as_ref());
}

fn sha3_512_digest(msg: &[u8], output: &mut [u8]) {
    let mut ctx = Context::new(&SHA3_512);
    ctx.update(msg);
    let digest = ctx.finish();
    output[0..SHA3_512_OUTPUT_LEN].copy_from_slice(digest.as_ref());
}
