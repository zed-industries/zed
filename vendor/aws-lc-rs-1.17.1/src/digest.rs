// Copyright 2015-2019 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! SHA-2 and the legacy SHA-1 digest algorithm.
//!
//! If all the data is available in a single contiguous slice then the `digest`
//! function should be used. Otherwise, the digest can be calculated in
//! multiple steps using `Context`.

//! # Example
//!
//! ```
//! use aws_lc_rs::digest;
//!
//! // Using `digest::digest`
//! let one_shot = digest::digest(&digest::SHA384, b"hello, world");
//!
//! // Using `digest::Context`
//! let mut ctx = digest::Context::new(&digest::SHA384);
//! ctx.update(b"hello");
//! ctx.update(b", ");
//! ctx.update(b"world");
//! let multi_part = ctx.finish();
//!
//! assert_eq!(&one_shot.as_ref(), &multi_part.as_ref());
//! ```

#![allow(non_snake_case)]
use crate::fips::indicator_check;
use crate::{debug, derive_debug_via_id};

pub(crate) mod digest_ctx;
mod sha;
use crate::aws_lc::{
    EVP_DigestFinal, EVP_DigestUpdate, EVP_sha1, EVP_sha224, EVP_sha256, EVP_sha384, EVP_sha3_256,
    EVP_sha3_384, EVP_sha3_512, EVP_sha512, EVP_sha512_256, EVP_MD,
};
use crate::error::Unspecified;
use crate::ptr::ConstPointer;
use core::ffi::c_uint;
use core::mem::MaybeUninit;
use digest_ctx::DigestContext;
pub use sha::{
    SHA1_FOR_LEGACY_USE_ONLY, SHA1_OUTPUT_LEN, SHA224, SHA224_OUTPUT_LEN, SHA256,
    SHA256_OUTPUT_LEN, SHA384, SHA384_OUTPUT_LEN, SHA3_256, SHA3_384, SHA3_512, SHA512, SHA512_256,
    SHA512_256_OUTPUT_LEN, SHA512_OUTPUT_LEN,
};

/// A context for multi-step (Init-Update-Finish) digest calculations.
//
// # FIPS
// Context must be used with one of the following algorithms:
// * `SHA1_FOR_LEGACY_USE_ONLY`
// * `SHA224`
// * `SHA256`
// * `SHA384`
// * `SHA512`
// * `SHA512_256`
#[derive(Clone)]
pub struct Context {
    /// The context's algorithm.
    pub(crate) algorithm: &'static Algorithm,
    digest_ctx: DigestContext,
    // The spec specifies that SHA-1 and SHA-256 support up to
    // 2^64-1 bits of input. SHA-384 and SHA-512 support up to
    // 2^128-1 bits.
    // Implementations of `digest` only support up
    // to 2^64-1 bits of input, which should be sufficient enough for
    // practical use cases.
    msg_len: u64,
    max_input_reached: bool,
}

impl Context {
    /// Constructs a new context.
    ///
    /// # Panics
    ///
    /// `new` panics if it fails to initialize an aws-lc digest context for the given
    /// algorithm.
    #[must_use]
    pub fn new(algorithm: &'static Algorithm) -> Self {
        Self {
            algorithm,
            digest_ctx: DigestContext::new(algorithm).unwrap(),
            msg_len: 0u64,
            max_input_reached: false,
        }
    }

    /// Updates the message to digest with all the data in `data`.
    ///
    /// # Panics
    /// Panics if update causes total input length to exceed maximum allowed (`u64::MAX`).
    #[inline]
    pub fn update(&mut self, data: &[u8]) {
        Self::try_update(self, data).expect("digest update failed");
    }

    #[inline]
    fn try_update(&mut self, data: &[u8]) -> Result<(), Unspecified> {
        unsafe {
            // Check if the message has reached the algorithm's maximum allowed input, or overflowed
            // the msg_len counter.
            let (msg_len, overflowed) = self.msg_len.overflowing_add(data.len() as u64);
            if overflowed || msg_len > self.algorithm.max_input_len {
                return Err(Unspecified);
            }

            self.msg_len = msg_len;
            self.max_input_reached = self.msg_len == self.algorithm.max_input_len;

            // Doesn't require boundary_check! guard
            if 1 != EVP_DigestUpdate(
                self.digest_ctx.as_mut_ptr(),
                data.as_ptr().cast(),
                data.len(),
            ) {
                return Err(Unspecified);
            }
            Ok(())
        }
    }

    /// Finalizes the digest calculation and returns the digest value.
    ///
    /// `finish` consumes the context so it cannot be (mis-)used after `finish`
    /// has been called.
    ///
    /// # Panics
    /// Panics if the digest is unable to be finalized
    #[inline]
    #[must_use]
    pub fn finish(self) -> Digest {
        Self::try_finish(self).expect("EVP_DigestFinal failed")
    }

    #[inline]
    fn try_finish(mut self) -> Result<Digest, Unspecified> {
        let mut output = [0u8; MAX_OUTPUT_LEN];
        let mut out_len = MaybeUninit::<c_uint>::uninit();
        if 1 != indicator_check!(unsafe {
            EVP_DigestFinal(
                self.digest_ctx.as_mut_ptr(),
                output.as_mut_ptr(),
                out_len.as_mut_ptr(),
            )
        }) {
            return Err(Unspecified);
        }

        Ok(Digest {
            algorithm: self.algorithm,
            message: output,
            len: self.algorithm.output_len,
        })
    }

    /// The algorithm that this context is using.
    #[inline]
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.algorithm
    }
}

/// Returns the digest of `data` using the given digest algorithm.
///
// # FIPS
// This function must only be used with one of the following algorithms:
// * `SHA1_FOR_LEGACY_USE_ONLY`
// * `SHA224`
// * `SHA256`
// * `SHA384`
// * `SHA512`
// * `SHA512_256`
//
/// # Examples:
///
/// ```
/// # {
/// use aws_lc_rs::{digest, test};
/// let expected_hex = "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
/// let expected: Vec<u8> = test::from_hex(expected_hex).unwrap();
/// let actual = digest::digest(&digest::SHA256, b"hello, world");
///
/// assert_eq!(&expected, &actual.as_ref());
/// # }
/// ```
#[inline]
#[must_use]
pub fn digest(algorithm: &'static Algorithm, data: &[u8]) -> Digest {
    let mut output = [0u8; MAX_OUTPUT_LEN];
    (algorithm.one_shot_hash)(data, &mut output);

    Digest {
        algorithm,
        message: output,
        len: algorithm.output_len,
    }
}

/// A calculated digest value.
///
/// Use [`Self::as_ref`] to get the value as a `&[u8]`.
#[derive(Clone, Copy)]
pub struct Digest {
    /// The trait `Copy` can't be implemented for dynamic arrays, so we set a
    /// fixed array and the appropriate length.
    message: [u8; MAX_OUTPUT_LEN],
    len: usize,

    algorithm: &'static Algorithm,
}

impl Digest {
    /// Imports a digest value provide by an external source. This allows for the signing of
    /// content that might not be directly accessible.
    ///
    /// WARNING: Ensure that the digest is provided by a trusted source.
    /// When possible, prefer to directly compute the digest of content.
    ///
    /// # Errors
    /// Returns `Unspecified` if the imported value is the wrong length for the specified algorithm.
    pub fn import_less_safe(
        digest: &[u8],
        algorithm: &'static Algorithm,
    ) -> Result<Self, Unspecified> {
        if digest.len() != algorithm.output_len {
            return Err(Unspecified);
        }
        let mut my_digest = [0u8; MAX_OUTPUT_LEN];
        my_digest[0..digest.len()].copy_from_slice(&digest[0..digest.len()]);
        Ok(Digest {
            message: my_digest,
            len: digest.len(),
            algorithm,
        })
    }

    /// The algorithm that was used to calculate the digest value.
    #[inline]
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.algorithm
    }
}

impl AsRef<[u8]> for Digest {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.message[..self.len]
    }
}

impl core::fmt::Debug for Digest {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "{:?}:", self.algorithm)?;
        debug::write_hex_bytes(fmt, self.as_ref())
    }
}

/// A digest algorithm.
pub struct Algorithm {
    /// The length of a finalized digest.
    pub output_len: usize,

    /// The size of the chaining value of the digest function, in bytes. For
    /// non-truncated algorithms (SHA-1, SHA-256, SHA-512), this is equal to
    /// `output_len`. For truncated algorithms (e.g. SHA-224, SHA-384, SHA-512/256),
    /// this is equal to the length before truncation. This is mostly helpful
    /// for determining the size of an HMAC key that is appropriate for the
    /// digest algorithm.
    ///
    /// This function isn't actually used in *aws-lc-rs*, and is only
    /// kept for compatibility with the original *ring* implementation.
    #[deprecated]
    pub chaining_len: usize,

    /// The internal block length.
    pub block_len: usize,

    // max_input_len is computed as u64 instead of usize to prevent overflowing on 32-bit machines.
    max_input_len: u64,

    one_shot_hash: fn(msg: &[u8], output: &mut [u8]),

    pub(crate) id: AlgorithmID,
}

unsafe impl Send for Algorithm {}

impl Algorithm {
    /// The length of a finalized digest.
    #[inline]
    #[must_use]
    pub fn output_len(&self) -> usize {
        self.output_len
    }

    /// The size of the chaining value of the digest function, in bytes. For
    /// non-truncated algorithms (SHA-1, SHA-256, SHA-512), this is equal to
    /// `output_len`. For truncated algorithms (e.g. SHA-224, SHA-384, SHA-512/256),
    /// this is equal to the length before truncation. This is mostly helpful
    /// for determining the size of an HMAC key that is appropriate for the
    /// digest algorithm.
    ///
    /// This function isn't actually used in *aws-lc-rs*, and is only
    /// kept for compatibility with the original *ring* implementation.
    #[deprecated]
    #[inline]
    #[must_use]
    pub fn chaining_len(&self) -> usize {
        // clippy warns on deprecated functions accessing deprecated fields
        #![allow(deprecated)]
        self.chaining_len
    }

    /// The internal block length.
    #[inline]
    #[must_use]
    pub fn block_len(&self) -> usize {
        self.block_len
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AlgorithmID {
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    SHA512_256,
    SHA3_256,
    SHA3_384,
    SHA3_512,
}

impl PartialEq for Algorithm {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Algorithm {}

derive_debug_via_id!(Algorithm);

/// The maximum block length ([`Algorithm::block_len`]) of all the algorithms
/// in this module.
pub const MAX_BLOCK_LEN: usize = 1024 / 8;

/// The maximum output length ([`Algorithm::output_len`]) of all the
/// algorithms in this module.
pub const MAX_OUTPUT_LEN: usize = 512 / 8;

/// The maximum chaining length ([`Algorithm::chaining_len`]) of all the
/// algorithms in this module.
pub const MAX_CHAINING_LEN: usize = MAX_OUTPUT_LEN;

/// Match digest types for `EVP_MD` functions.
pub(crate) fn match_digest_type(algorithm_id: &AlgorithmID) -> ConstPointer<'_, EVP_MD> {
    unsafe {
        ConstPointer::new_static(match algorithm_id {
            AlgorithmID::SHA1 => EVP_sha1(),
            AlgorithmID::SHA224 => EVP_sha224(),
            AlgorithmID::SHA256 => EVP_sha256(),
            AlgorithmID::SHA384 => EVP_sha384(),
            AlgorithmID::SHA512 => EVP_sha512(),
            AlgorithmID::SHA512_256 => EVP_sha512_256(),
            AlgorithmID::SHA3_256 => EVP_sha3_256(),
            AlgorithmID::SHA3_384 => EVP_sha3_384(),
            AlgorithmID::SHA3_512 => EVP_sha3_512(),
        })
        .unwrap_or_else(|()| panic!("Digest algorithm not found: {algorithm_id:?}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::digest;
    #[cfg(feature = "fips")]
    mod fips;

    mod max_input {
        extern crate alloc;

        use super::super::super::digest;
        use crate::digest::digest_ctx::DigestContext;
        use crate::digest::Digest;
        use alloc::vec;

        macro_rules! max_input_tests {
            ( $algorithm_name:ident ) => {
                mod $algorithm_name {
                    use super::super::super::super::digest;

                    #[test]
                    fn max_input_test() {
                        super::max_input_test(&digest::$algorithm_name);
                    }
                    #[test]
                    #[should_panic(expected = "digest update failed")]
                    fn too_long_input_test_block() {
                        super::too_long_input_test_block(&digest::$algorithm_name);
                    }

                    #[test]
                    #[should_panic(expected = "digest update failed")]
                    fn too_long_input_test_byte() {
                        super::too_long_input_test_byte(&digest::$algorithm_name);
                    }
                }
            };
        }

        fn max_input_test(alg: &'static digest::Algorithm) {
            let mut context = nearly_full_context(alg);
            let next_input = vec![0u8; alg.block_len - 1];
            context.update(&next_input);
            let _: Digest = context.finish(); // no panic
        }

        fn too_long_input_test_block(alg: &'static digest::Algorithm) {
            let mut context = nearly_full_context(alg);
            let next_input = vec![0u8; alg.block_len];
            context.update(&next_input);
            let _: Digest = context.finish(); // should panic
        }

        fn too_long_input_test_byte(alg: &'static digest::Algorithm) {
            let mut context = nearly_full_context(alg);
            let next_input = vec![0u8; alg.block_len - 1];
            context.update(&next_input); // no panic
            context.update(&[0]);
            let _: Digest = context.finish(); // should panic
        }

        fn nearly_full_context(alg: &'static digest::Algorithm) -> digest::Context {
            // Implementations of `digest` only support up
            // to 2^64-1 bits of input.
            let block_len = alg.block_len as u64;
            digest::Context {
                algorithm: alg,
                digest_ctx: DigestContext::new(alg).unwrap(),
                msg_len: alg.max_input_len - block_len + 1,
                max_input_reached: false,
            }
        }

        max_input_tests!(SHA1_FOR_LEGACY_USE_ONLY);
        max_input_tests!(SHA224);
        max_input_tests!(SHA256);
        max_input_tests!(SHA384);
        max_input_tests!(SHA512);
        max_input_tests!(SHA3_384);
        max_input_tests!(SHA3_512);
    }

    #[test]
    fn digest_coverage() {
        for alg in [
            &digest::SHA1_FOR_LEGACY_USE_ONLY,
            &digest::SHA224,
            &digest::SHA256,
            &digest::SHA384,
            &digest::SHA512,
            &digest::SHA3_384,
            &digest::SHA3_512,
        ] {
            // Clone after updating context with message, then check if the final Digest is the same.
            let mut ctx = digest::Context::new(alg);
            ctx.update(b"hello, world");
            let ctx_clone = ctx.clone();
            assert_eq!(ctx_clone.algorithm(), ctx.algorithm());

            let orig_digest = ctx.finish();
            let clone_digest = ctx_clone.finish();
            assert_eq!(orig_digest.algorithm(), clone_digest.algorithm());
            assert_eq!(orig_digest.as_ref(), clone_digest.as_ref());
            assert_eq!(orig_digest.clone().as_ref(), clone_digest.as_ref());
        }
    }

    #[test]
    fn test_import_less_safe() {
        let digest = digest::digest(&digest::SHA256, b"hello, world");
        let digest_copy =
            digest::Digest::import_less_safe(digest.as_ref(), &digest::SHA256).unwrap();

        assert_eq!(digest.as_ref(), digest_copy.as_ref());
        assert_eq!(digest.algorithm, digest_copy.algorithm);
    }
}
