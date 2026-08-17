// Copyright 2015-2022 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! HMAC is specified in [RFC 2104].
//!
//! After a `Key` is constructed, it can be used for multiple signing or
//! verification operations. Separating the construction of the key from the
//! rest of the HMAC operation allows the per-key precomputation to be done
//! only once, instead of it being done in every HMAC operation.
//!
//! Frequently all the data to be signed in a message is available in a single
//! contiguous piece. In that case, the module-level `sign` function can be
//! used. Otherwise, if the input is in multiple parts, `Context` should be
//! used.
//!
//! # Examples:
//!
//! ## Signing a value and verifying it wasn't tampered with
//!
//! ```
//! use aws_lc_rs::{hmac, rand};
//!
//! let rng = rand::SystemRandom::new();
//! let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng)?;
//!
//! let msg = "hello, world";
//!
//! let tag = hmac::sign(&key, msg.as_bytes());
//!
//! // [We give access to the message to an untrusted party, and they give it
//! // back to us. We need to verify they didn't tamper with it.]
//!
//! hmac::verify(&key, msg.as_bytes(), tag.as_ref())?;
//!
//! # Ok::<(), aws_lc_rs::error::Unspecified>(())
//! ```
//!
//! ## Using the one-shot API:
//!
//! ```
//! use aws_lc_rs::rand::SecureRandom;
//! use aws_lc_rs::{digest, hmac, rand};
//!
//! let msg = "hello, world";
//!
//! // The sender generates a secure key value and signs the message with it.
//! // Note that in a real protocol, a key agreement protocol would be used to
//! // derive `key_value`.
//! let rng = rand::SystemRandom::new();
//! let key_value: [u8; digest::SHA256_OUTPUT_LEN] = rand::generate(&rng)?.expose();
//!
//! let s_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
//! let tag = hmac::sign(&s_key, msg.as_bytes());
//!
//! // The receiver (somehow!) knows the key value, and uses it to verify the
//! // integrity of the message.
//! let v_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
//! hmac::verify(&v_key, msg.as_bytes(), tag.as_ref())?;
//!
//! # Ok::<(), aws_lc_rs::error::Unspecified>(())
//! ```
//!
//! ## Using the multi-part API:
//! ```
//! use aws_lc_rs::rand::SecureRandom;
//! use aws_lc_rs::{digest, hmac, rand};
//!
//! let parts = ["hello", ", ", "world"];
//!
//! // The sender generates a secure key value and signs the message with it.
//! // Note that in a real protocol, a key agreement protocol would be used to
//! // derive `key_value`.
//! let rng = rand::SystemRandom::new();
//! let mut key_value: [u8; digest::SHA384_OUTPUT_LEN] = rand::generate(&rng)?.expose();
//!
//! let s_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
//! let mut s_ctx = hmac::Context::with_key(&s_key);
//! for part in &parts {
//!     s_ctx.update(part.as_bytes());
//! }
//! let tag = s_ctx.sign();
//!
//! // The receiver (somehow!) knows the key value, and uses it to verify the
//! // integrity of the message.
//! let v_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
//! let mut msg = Vec::<u8>::new();
//! for part in &parts {
//!     msg.extend(part.as_bytes());
//! }
//! hmac::verify(&v_key, &msg.as_ref(), tag.as_ref())?;
//!
//! # Ok::<(), aws_lc_rs::error::Unspecified>(())
//! ```
//! [RFC 2104]: https://tools.ietf.org/html/rfc2104

use crate::aws_lc::{
    HMAC_CTX_cleanup, HMAC_CTX_copy_ex, HMAC_CTX_init, HMAC_Final, HMAC_Init_ex, HMAC_Update,
    HMAC_CTX,
};
use crate::error::Unspecified;
use crate::fips::indicator_check;
use crate::{constant_time, digest, hkdf};
use core::ffi::c_uint;
use core::mem::MaybeUninit;
use core::ptr::null_mut;

/// A deprecated alias for `Tag`.
#[deprecated]
pub type Signature = Tag;
/// Renamed to `Context`.
#[deprecated]
pub type SigningContext = Context;
/// Renamed to `Key`.
#[deprecated]
pub type SigningKey = Key;
/// Merged into `Key`.
#[deprecated]
pub type VerificationKey = Key;

/// An HMAC algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Algorithm(&'static digest::Algorithm);

impl Algorithm {
    /// The digest algorithm this HMAC algorithm is based on.
    #[inline]
    #[must_use]
    pub fn digest_algorithm(&self) -> &'static digest::Algorithm {
        self.0
    }

    /// The tag length for this HMAC algorithm.
    #[inline]
    #[must_use]
    pub fn tag_len(&self) -> usize {
        self.digest_algorithm().output_len
    }
}

/// HMAC using SHA-1. Obsolete.
pub const HMAC_SHA1_FOR_LEGACY_USE_ONLY: Algorithm = Algorithm(&digest::SHA1_FOR_LEGACY_USE_ONLY);

/// HMAC using SHA-224.
pub const HMAC_SHA224: Algorithm = Algorithm(&digest::SHA224);

/// HMAC using SHA-256.
pub const HMAC_SHA256: Algorithm = Algorithm(&digest::SHA256);

/// HMAC using SHA-384.
pub const HMAC_SHA384: Algorithm = Algorithm(&digest::SHA384);

/// HMAC using SHA-512.
pub const HMAC_SHA512: Algorithm = Algorithm(&digest::SHA512);

/// An HMAC tag.
///
/// For a given tag `t`, use `t.as_ref()` to get the tag value as a byte slice.
#[derive(Clone, Copy, Debug)]
pub struct Tag {
    msg: [u8; digest::MAX_OUTPUT_LEN],
    msg_len: usize,
}

impl AsRef<[u8]> for Tag {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.msg[..self.msg_len]
    }
}

struct LcHmacCtx(HMAC_CTX);

impl LcHmacCtx {
    fn as_mut_ptr(&mut self) -> *mut HMAC_CTX {
        &mut self.0
    }
    fn as_ptr(&self) -> *const HMAC_CTX {
        &self.0
    }

    fn try_clone(&self) -> Result<Self, Unspecified> {
        unsafe {
            let mut hmac_ctx = MaybeUninit::<HMAC_CTX>::uninit();
            HMAC_CTX_init(hmac_ctx.as_mut_ptr());
            let mut hmac_ctx = hmac_ctx.assume_init();
            if 1 != HMAC_CTX_copy_ex(&mut hmac_ctx, self.as_ptr()) {
                return Err(Unspecified);
            }
            Ok(LcHmacCtx(hmac_ctx))
        }
    }
}
unsafe impl Send for LcHmacCtx {}

impl Drop for LcHmacCtx {
    fn drop(&mut self) {
        unsafe { HMAC_CTX_cleanup(self.as_mut_ptr()) }
    }
}

impl Clone for LcHmacCtx {
    fn clone(&self) -> Self {
        self.try_clone().expect("Unable to clone LcHmacCtx")
    }
}

/// A key to use for HMAC signing.
//
// # FIPS
// Use this type with one of the following algorithms:
// * `HMAC_SHA1_FOR_LEGACY_USE_ONLY`
// * `HMAC_SHA224`
// * `HMAC_SHA256`
// * `HMAC_SHA384`
// * `HMAC_SHA512`
#[derive(Clone)]
pub struct Key {
    pub(crate) algorithm: Algorithm,
    ctx: LcHmacCtx,
}

unsafe impl Send for Key {}
// All uses of *mut HMAC_CTX require the creation of a Context, which will clone the Key.
unsafe impl Sync for Key {}

#[allow(clippy::missing_fields_in_debug)]
impl core::fmt::Debug for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        f.debug_struct("Key")
            .field("algorithm", &self.algorithm.digest_algorithm())
            .finish()
    }
}

impl Key {
    /// Generate an HMAC signing key using the given digest algorithm with a
    /// random value generated from `rng`.
    ///
    /// The key will be `digest_alg.output_len` bytes long, based on the
    /// recommendation in [RFC 2104 Section 3].
    ///
    /// [RFC 2104 Section 3]: https://tools.ietf.org/html/rfc2104#section-3
    ///
    //
    // # FIPS
    // Use this function with one of the following algorithms:
    // * `HMAC_SHA1_FOR_LEGACY_USE_ONLY`
    // * `HMAC_SHA224`
    // * `HMAC_SHA256`
    // * `HMAC_SHA384`
    // * `HMAC_SHA512`
    //
    /// # Errors
    /// `error::Unspecified` is the `rng` fails.
    pub fn generate(
        algorithm: Algorithm,
        rng: &dyn crate::rand::SecureRandom,
    ) -> Result<Self, Unspecified> {
        Self::construct(algorithm, |buf| rng.fill(buf))
    }

    fn construct<F>(algorithm: Algorithm, fill: F) -> Result<Self, Unspecified>
    where
        F: FnOnce(&mut [u8]) -> Result<(), Unspecified>,
    {
        let mut key_bytes = [0; digest::MAX_OUTPUT_LEN];
        let key_bytes = &mut key_bytes[..algorithm.tag_len()];
        fill(key_bytes)?;
        Ok(Self::new(algorithm, key_bytes))
    }

    /// Construct an HMAC signing key using the given digest algorithm and key
    /// value.
    ///
    /// `key_value` should be a value generated using a secure random number
    /// generator (e.g. the `key_value` output by
    /// `SealingKey::generate_serializable()`) or derived from a random key by
    /// a key derivation function (e.g. `aws_lc_rs::hkdf`). In particular,
    /// `key_value` shouldn't be a password.
    ///
    /// As specified in RFC 2104, if `key_value` is shorter than the digest
    /// algorithm's block length (as returned by `digest::Algorithm::block_len`,
    /// not the digest length returned by `digest::Algorithm::output_len`) then
    /// it will be padded with zeros. Similarly, if it is longer than the block
    /// length then it will be compressed using the digest algorithm.
    ///
    /// You should not use keys larger than the `digest_alg.block_len` because
    /// the truncation described above reduces their strength to only
    /// `digest_alg.output_len * 8` bits.
    ///
    /// # Panics
    /// Panics if the HMAC context cannot be constructed
    #[inline]
    #[must_use]
    pub fn new(algorithm: Algorithm, key_value: &[u8]) -> Self {
        Key::try_new(algorithm, key_value).expect("Unable to create HmacContext")
    }

    fn try_new(algorithm: Algorithm, key_value: &[u8]) -> Result<Self, Unspecified> {
        unsafe {
            let mut ctx = MaybeUninit::<HMAC_CTX>::uninit();
            HMAC_CTX_init(ctx.as_mut_ptr());
            let evp_md_type = digest::match_digest_type(&algorithm.digest_algorithm().id);
            if 1 != HMAC_Init_ex(
                ctx.as_mut_ptr(),
                key_value.as_ptr().cast(),
                key_value.len(),
                evp_md_type.as_const_ptr(),
                null_mut(),
            ) {
                return Err(Unspecified);
            }
            let result = Self {
                algorithm,
                ctx: LcHmacCtx(ctx.assume_init()),
            };
            Ok(result)
        }
    }

    unsafe fn get_hmac_ctx_ptr(&mut self) -> *mut HMAC_CTX {
        self.ctx.as_mut_ptr()
    }

    /// The digest algorithm for the key.
    #[inline]
    #[must_use]
    pub fn algorithm(&self) -> Algorithm {
        Algorithm(self.algorithm.digest_algorithm())
    }
}

impl hkdf::KeyType for Algorithm {
    #[inline]
    fn len(&self) -> usize {
        self.tag_len()
    }
}

impl From<hkdf::Okm<'_, Algorithm>> for Key {
    fn from(okm: hkdf::Okm<Algorithm>) -> Self {
        Self::construct(*okm.len(), |buf| okm.fill(buf)).unwrap()
    }
}

/// A context for multi-step (Init-Update-Finish) HMAC signing.
///
/// Use `sign` for single-step HMAC signing.
pub struct Context {
    key: Key,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
        }
    }
}

unsafe impl Send for Context {}

impl core::fmt::Debug for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        f.debug_struct("Context")
            .field("algorithm", &self.key.algorithm.digest_algorithm())
            .finish()
    }
}

impl Context {
    /// Constructs a new HMAC signing context using the given digest algorithm
    /// and key.
    #[inline]
    #[must_use]
    pub fn with_key(signing_key: &Key) -> Self {
        Self {
            key: signing_key.clone(),
        }
    }

    /// Updates the HMAC with all the data in `data`. `update` may be called
    /// zero or more times until `finish` is called.
    ///
    /// # Panics
    /// Panics if the HMAC cannot be updated
    #[inline]
    pub fn update(&mut self, data: &[u8]) {
        Self::try_update(self, data).expect("HMAC_Update failed");
    }

    #[inline]
    fn try_update(&mut self, data: &[u8]) -> Result<(), Unspecified> {
        unsafe {
            if 1 != HMAC_Update(self.key.get_hmac_ctx_ptr(), data.as_ptr(), data.len()) {
                return Err(Unspecified);
            }
        }
        Ok(())
    }

    /// Finalizes the HMAC calculation and returns the HMAC value. `sign`
    /// consumes the context so it cannot be (mis-)used after `sign` has been
    /// called.
    ///
    /// It is generally not safe to implement HMAC verification by comparing
    /// the return value of `sign` to a tag. Use `verify` for verification
    /// instead.
    ///
    // # FIPS
    // Use this method with one of the following algorithms:
    // * `HMAC_SHA1_FOR_LEGACY_USE_ONLY`
    // * `HMAC_SHA224`
    // * `HMAC_SHA256`
    // * `HMAC_SHA384`
    // * `HMAC_SHA512`
    //
    /// # Panics
    /// Panics if the HMAC calculation cannot be finalized
    #[inline]
    #[must_use]
    pub fn sign(self) -> Tag {
        Self::try_sign(self).expect("HMAC_Final failed")
    }
    #[inline]
    fn try_sign(mut self) -> Result<Tag, Unspecified> {
        let mut output = [0u8; digest::MAX_OUTPUT_LEN];
        let msg_len = {
            let result = internal_sign(&mut self, &mut output)?;
            result.len()
        };
        Ok(Tag {
            msg: output,
            msg_len,
        })
    }
}

#[inline]
pub(crate) fn internal_sign<'in_out>(
    ctx: &mut Context,
    output: &'in_out mut [u8],
) -> Result<&'in_out mut [u8], Unspecified> {
    let tag_len = ctx.key.algorithm().tag_len();
    if output.len() < tag_len {
        return Err(Unspecified);
    }

    let mut out_len = MaybeUninit::<c_uint>::uninit();

    if 1 != indicator_check!(unsafe {
        HMAC_Final(
            ctx.key.get_hmac_ctx_ptr(),
            output.as_mut_ptr(),
            out_len.as_mut_ptr(),
        )
    }) {
        return Err(Unspecified);
    }
    let actual_len = unsafe { out_len.assume_init() } as usize;

    debug_assert!(
        actual_len == tag_len,
        "HMAC tag length {actual_len} does not match expected length {tag_len}"
    );

    Ok(&mut output[0..tag_len])
}

/// Calculates the HMAC of `data` using the key `key` in one step.
///
/// Use `Context` to calculate HMACs where the input is in multiple parts.
///
/// It is generally not safe to implement HMAC verification by comparing the
/// return value of `sign` to a tag. Use `verify` for verification instead.
//
// # FIPS
// Use this function with one of the following algorithms:
// * `HMAC_SHA1_FOR_LEGACY_USE_ONLY`
// * `HMAC_SHA224`
// * `HMAC_SHA256`
// * `HMAC_SHA384`
// * `HMAC_SHA512`
#[inline]
#[must_use]
pub fn sign(key: &Key, data: &[u8]) -> Tag {
    let mut ctx = Context::with_key(key);
    ctx.update(data);
    ctx.sign()
}

/// Calculates the HMAC of `data` using the key `key` in one step,
/// writing the result into the provided `output` buffer.
///
/// The `output` buffer must be at least as large as the algorithm's
/// tag length (i.e., `key.algorithm().tag_len()`). The returned slice will be a
/// sub-slice of `output` containing exactly the tag bytes.
///
/// It is generally not safe to implement HMAC verification by comparing the
/// return value of `sign_to_buffer` to a tag. Use `verify` for verification instead.
//
// # FIPS
// Use this function with one of the following algorithms:
// * `HMAC_SHA1_FOR_LEGACY_USE_ONLY`
// * `HMAC_SHA224`
// * `HMAC_SHA256`
// * `HMAC_SHA384`
// * `HMAC_SHA512`
//
/// # Errors
/// `error::Unspecified` if `output` is too small or if the HMAC operation fails.
#[inline]
pub fn sign_to_buffer<'out>(
    key: &Key,
    data: &[u8],
    output: &'out mut [u8],
) -> Result<&'out mut [u8], Unspecified> {
    let mut ctx = Context::with_key(key);
    ctx.update(data);

    internal_sign(&mut ctx, output)
}

/// Calculates the HMAC of `data` using the signing key `key`, and verifies
/// whether the resultant value equals `tag`, in one step.
///
/// This is logically equivalent to, but more efficient than, constructing a
/// `Key` with the same value as `key` and then using `verify`.
///
/// The verification will be done in constant time to prevent timing attacks.
///
/// # Errors
/// `error::Unspecified` if the inputs are not verified.
//
// # FIPS
// Use this function with one of the following algorithms:
// * `HMAC_SHA1_FOR_LEGACY_USE_ONLY`
// * `HMAC_SHA224`
// * `HMAC_SHA256`
// * `HMAC_SHA384`
// * `HMAC_SHA512`
#[inline]
pub fn verify(key: &Key, data: &[u8], tag: &[u8]) -> Result<(), Unspecified> {
    constant_time::verify_slices_are_equal(sign(key, data).as_ref(), tag)
}

#[cfg(test)]
mod tests {
    use crate::{hmac, rand};

    #[cfg(feature = "fips")]
    mod fips;

    #[test]
    fn hmac_algorithm_properties() {
        assert_eq!(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY.tag_len(), 20);
        assert_eq!(hmac::HMAC_SHA224.tag_len(), 28);
        assert_eq!(hmac::HMAC_SHA256.tag_len(), 32);
        assert_eq!(hmac::HMAC_SHA384.tag_len(), 48);
        assert_eq!(hmac::HMAC_SHA512.tag_len(), 64);
    }

    // Make sure that internal_sign properly rejects too small buffers
    // (and does not corrupt memory by buffer overflow)
    #[test]
    fn hmac_internal_sign_too_small_buffer() {
        let rng = rand::SystemRandom::new();

        for algorithm in &[
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            hmac::HMAC_SHA224,
            hmac::HMAC_SHA256,
            hmac::HMAC_SHA384,
            hmac::HMAC_SHA512,
        ] {
            let key = hmac::Key::generate(*algorithm, &rng).unwrap();
            let data = b"hello, world";

            // Buffer one byte too small should fail
            let mut small_buf = vec![0u8; algorithm.tag_len() - 1];
            let mut ctx = hmac::Context::with_key(&key);
            ctx.update(data);
            assert!(super::internal_sign(&mut ctx, &mut small_buf).is_err());

            // Empty buffer should fail
            let mut empty_buf = vec![];
            let mut ctx = hmac::Context::with_key(&key);
            ctx.update(data);
            assert!(super::internal_sign(&mut ctx, &mut empty_buf).is_err());
        }
    }

    // Make sure that `Key::generate` and `verify_with_own_key` aren't
    // completely wacky.
    #[test]
    pub fn hmac_signing_key_coverage() {
        const HELLO_WORLD_GOOD: &[u8] = b"hello, world";
        const HELLO_WORLD_BAD: &[u8] = b"hello, worle";

        let rng = rand::SystemRandom::new();

        for algorithm in &[
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            hmac::HMAC_SHA224,
            hmac::HMAC_SHA256,
            hmac::HMAC_SHA384,
            hmac::HMAC_SHA512,
        ] {
            let key = hmac::Key::generate(*algorithm, &rng).unwrap();
            let tag = hmac::sign(&key, HELLO_WORLD_GOOD);
            println!("{key:?}");
            assert!(hmac::verify(&key, HELLO_WORLD_GOOD, tag.as_ref()).is_ok());
            assert!(hmac::verify(&key, HELLO_WORLD_BAD, tag.as_ref()).is_err());
        }
    }

    #[test]
    fn hmac_coverage() {
        // Something would have gone horribly wrong for this to not pass, but we test this so our
        // coverage reports will look better.
        assert_ne!(hmac::HMAC_SHA256, hmac::HMAC_SHA384);

        for &alg in &[
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            hmac::HMAC_SHA224,
            hmac::HMAC_SHA256,
            hmac::HMAC_SHA384,
            hmac::HMAC_SHA512,
        ] {
            // Clone after updating context with message, then check if the final Tag is the same.
            let key = hmac::Key::new(alg, &[0; 32]);
            let mut ctx = hmac::Context::with_key(&key);
            ctx.update(b"hello, world");
            let ctx_clone = ctx.clone();

            let orig_tag = ctx.sign();
            let clone_tag = ctx_clone.sign();
            assert_eq!(orig_tag.as_ref(), clone_tag.as_ref());
            assert_eq!(orig_tag.clone().as_ref(), clone_tag.as_ref());
        }
    }

    #[test]
    fn hmac_sign_to_buffer_test() {
        let rng = rand::SystemRandom::new();

        for &algorithm in &[
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            hmac::HMAC_SHA224,
            hmac::HMAC_SHA256,
            hmac::HMAC_SHA384,
            hmac::HMAC_SHA512,
        ] {
            let key = hmac::Key::generate(algorithm, &rng).unwrap();
            let data = b"hello, world";
            let tag_len = algorithm.tag_len();

            // Test with exact size buffer
            let mut output = vec![0u8; tag_len];
            let result = hmac::sign_to_buffer(&key, data, &mut output).unwrap();
            assert_eq!(result.len(), tag_len);

            // Verify the returned tag matches sign() and passes verify()
            let tag = hmac::sign(&key, data);
            assert_eq!(result, tag.as_ref());
            assert!(hmac::verify(&key, data, result).is_ok());

            // Verify the output buffer also matches sign() and passes verify()
            assert_eq!(output.as_slice(), tag.as_ref());
            assert!(hmac::verify(&key, data, output.as_slice()).is_ok());

            // Test with larger buffer
            let mut large_output = vec![0u8; tag_len + 10];
            let result2 = hmac::sign_to_buffer(&key, data, &mut large_output).unwrap();
            assert_eq!(result2.len(), tag_len);
            assert_eq!(result2, tag.as_ref());
            assert!(hmac::verify(&key, data, result2).is_ok());
            assert_eq!(&large_output[0..tag_len], tag.as_ref());
        }
    }

    #[test]
    fn hmac_sign_to_buffer_too_small_test() {
        let key = hmac::Key::new(hmac::HMAC_SHA256, &[0; 32]);
        let data = b"hello";

        // Buffer too small should fail
        let mut small_buffer = vec![0u8; hmac::HMAC_SHA256.tag_len() - 1];
        assert!(hmac::sign_to_buffer(&key, data, &mut small_buffer).is_err());

        // Empty buffer should fail
        let mut empty_buffer = vec![];
        assert!(hmac::sign_to_buffer(&key, data, &mut empty_buffer).is_err());
    }
}
