// Copyright 2015-2022 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! PBKDF2 derivation and verification.
//!
//! Use `derive` to derive PBKDF2 outputs. Use `verify` to verify secret
//! against previously-derived outputs.
//!
//! PBKDF2 is specified in [RFC 2898 Section 5.2] with test vectors given in
//! [RFC 6070]. See also [NIST Special Publication 800-132].
//!
//! [RFC 2898 Section 5.2]: https://tools.ietf.org/html/rfc2898#section-5.2
//! [RFC 6070]: https://tools.ietf.org/html/rfc6070
//! [NIST Special Publication 800-132]:
//!    http://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-132.pdf
//!
//! # Examples
//!
//! ## Password Database Example
//!
//! ```
//! use aws_lc_rs::{digest, pbkdf2};
//! use std::{collections::HashMap, num::NonZeroU32};
//!
//! static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
//! const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
//! pub type Credential = [u8; CREDENTIAL_LEN];
//!
//! enum Error {
//!     WrongUsernameOrPassword
//! }
//!
//! struct PasswordDatabase {
//!     pbkdf2_iterations: NonZeroU32,
//!     db_salt_component: [u8; 16],
//!
//!     // Normally this would be a persistent database.
//!     storage: HashMap<String, Credential>,
//! }
//!
//! impl PasswordDatabase {
//!     pub fn store_password(&mut self, username: &str, password: &str) {
//!         let salt = self.salt(username);
//!         let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
//!         pbkdf2::derive(PBKDF2_ALG, self.pbkdf2_iterations, &salt,
//!                        password.as_bytes(), &mut to_store);
//!         self.storage.insert(String::from(username), to_store);
//!     }
//!
//!     pub fn verify_password(&self, username: &str, attempted_password: &str)
//!                            -> Result<(), Error> {
//!         match self.storage.get(username) {
//!            Some(actual_password) => {
//!                let salt = self.salt(username);
//!                pbkdf2::verify(PBKDF2_ALG, self.pbkdf2_iterations, &salt,
//!                               attempted_password.as_bytes(),
//!                               actual_password)
//!                     .map_err(|_| Error::WrongUsernameOrPassword)
//!            },
//!
//!            None => Err(Error::WrongUsernameOrPassword)
//!         }
//!     }
//!
//!     // The salt should have a user-specific component so that an attacker
//!     // cannot crack one password for multiple users in the database. It
//!     // should have a database-unique component so that an attacker cannot
//!     // crack the same user's password across databases in the unfortunate
//!     // but common case that the user has used the same password for
//!     // multiple systems.
//!     fn salt(&self, username: &str) -> Vec<u8> {
//!         let mut salt = Vec::with_capacity(self.db_salt_component.len() +
//!                                           username.as_bytes().len());
//!         salt.extend(self.db_salt_component.as_ref());
//!         salt.extend(username.as_bytes());
//!         salt
//!     }
//! }
//!
//! fn main() {
//!     // Normally these parameters would be loaded from a configuration file.
//!     let mut db = PasswordDatabase {
//!         pbkdf2_iterations: NonZeroU32::new(100_000).unwrap(),
//!         db_salt_component: [
//!             // This value was generated from a secure PRNG.
//!             0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52,
//!             0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a
//!         ],
//!         storage: HashMap::new(),
//!     };
//!
//!     db.store_password("alice", "@74d7]404j|W}6u");
//!
//!     // An attempt to log in with the wrong password fails.
//!     assert!(db.verify_password("alice", "wrong password").is_err());
//!
//!     // Normally there should be an expoentially-increasing delay between
//!     // attempts to further protect against online attacks.
//!
//!     // An attempt to log in with the right password succeeds.
//!     assert!(db.verify_password("alice", "@74d7]404j|W}6u").is_ok());
//! }

use crate::aws_lc::PKCS5_PBKDF2_HMAC;
use crate::error::Unspecified;
use crate::fips::indicator_check;
use crate::{constant_time, digest, hmac};
use core::num::NonZeroU32;
use zeroize::Zeroize;

/// A PBKDF2 algorithm.
///
/// `max_output_len` is computed as u64 instead of usize to prevent overflowing on 32-bit machines.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Algorithm {
    algorithm: hmac::Algorithm,
    max_output_len: u64,
}

/// PBKDF2 using HMAC-SHA1.
pub const PBKDF2_HMAC_SHA1: Algorithm = Algorithm {
    algorithm: hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
    max_output_len: MAX_USIZE32 * digest::SHA1_OUTPUT_LEN as u64,
};

/// PBKDF2 using HMAC-SHA256.
pub const PBKDF2_HMAC_SHA256: Algorithm = Algorithm {
    algorithm: hmac::HMAC_SHA256,
    max_output_len: MAX_USIZE32 * digest::SHA256_OUTPUT_LEN as u64,
};

/// PBKDF2 using HMAC-SHA384.
pub const PBKDF2_HMAC_SHA384: Algorithm = Algorithm {
    algorithm: hmac::HMAC_SHA384,
    max_output_len: MAX_USIZE32 * digest::SHA384_OUTPUT_LEN as u64,
};

/// PBKDF2 using HMAC-SHA512.
pub const PBKDF2_HMAC_SHA512: Algorithm = Algorithm {
    algorithm: hmac::HMAC_SHA512,
    max_output_len: MAX_USIZE32 * digest::SHA512_OUTPUT_LEN as u64,
};

const MAX_USIZE32: u64 = u32::MAX as u64;

/// Fills `out` with the key derived using PBKDF2 with the given inputs.
///
/// Do not use `derive` as part of verifying a secret; use `verify` instead, to
/// minimize the effectiveness of timing attacks.
///
/// `out.len()` must be no larger than the digest length * (2**32 - 1), per the
/// PBKDF2 specification.
///
/// | Parameter   | RFC 2898 Section 5.2 Term
/// |-------------|-------------------------------------------
/// | `digest_alg`  | PRF (HMAC with the given digest algorithm)
/// | `iterations`  | c (iteration count)
/// | `salt`        | S (salt)
/// | `secret`      | P (password)
/// | `out`         | dk (derived key)
/// | `out.len()`   | dkLen (derived key length)
///
/// # Panics
///
/// `derive` panics if `out.len()` is larger than (2**32 - 1) * the digest
/// algorithm's output length, per the PBKDF2 specification.
//
// # FIPS
// The following conditions must be met:
// * Algorithm is one of the following:
//   * `PBKDF2_HMAC_SHA1`
//   * `PBKDF2_HMAC_SHA256`
//   * `PBKDF2_HMAC_SHA384`
//   * `PBKDF2_HMAC_SHA512`
// * `salt.len()` >= 16
// * `sercet.len()` >= 14
// * `iterations` >= 1000
#[inline]
pub fn derive(
    algorithm: Algorithm,
    iterations: NonZeroU32,
    salt: &[u8],
    secret: &[u8],
    out: &mut [u8],
) {
    try_derive(algorithm, iterations, salt, secret, out).expect("pbkdf2 derive failed");
}

#[inline]
fn try_derive(
    algorithm: Algorithm,
    iterations: NonZeroU32,
    salt: &[u8],
    secret: &[u8],
    out: &mut [u8],
) -> Result<(), Unspecified> {
    assert!(
        out.len() as u64 <= algorithm.max_output_len,
        "derived key too long"
    );

    if 1 != indicator_check!(unsafe {
        PKCS5_PBKDF2_HMAC(
            secret.as_ptr().cast(),
            secret.len(),
            salt.as_ptr(),
            salt.len(),
            iterations.get(),
            digest::match_digest_type(&algorithm.algorithm.digest_algorithm().id).as_const_ptr(),
            out.len(),
            out.as_mut_ptr(),
        )
    }) {
        return Err(Unspecified);
    }
    Ok(())
}

/// Verifies that a previously-derived (e.g., using `derive`) PBKDF2 value
/// matches the PBKDF2 value derived from the other inputs.
///
/// The comparison is done in constant time to prevent timing attacks. The
/// comparison will fail if `previously_derived` is empty (has a length of
/// zero).
///
/// | Parameter                  | RFC 2898 Section 5.2 Term
/// |----------------------------|--------------------------------------------
/// | `digest_alg`                 | PRF (HMAC with the given digest algorithm).
/// | `iterations`               | c (iteration count)
/// | `salt`                     | S (salt)
/// | `secret`                   | P (password)
/// | `previously_derived`       | dk (derived key)
/// | `previously_derived.len()` | dkLen (derived key length)
///
/// # Errors
/// `error::Unspecified` is the inputs were not verified.
///
/// # Panics
///
/// `verify` panics if `previously_derived.len()` is larger than (2**32 - 1) * the digest
/// algorithm's output length, per the PBKDF2 specification.
//
// # FIPS
// The following conditions must be met:
// * Algorithm is one of the following:
//   * `PBKDF2_HMAC_SHA1`
//   * `PBKDF2_HMAC_SHA256`
//   * `PBKDF2_HMAC_SHA384`
//   * `PBKDF2_HMAC_SHA512`
// * `salt.len()` >= 16
// * `secret.len()` >= 14
// * `iterations` >= 1000
#[inline]
pub fn verify(
    algorithm: Algorithm,
    iterations: NonZeroU32,
    salt: &[u8],
    secret: &[u8],
    previously_derived: &[u8],
) -> Result<(), Unspecified> {
    if previously_derived.is_empty() {
        return Err(Unspecified);
    }
    assert!(
        previously_derived.len() as u64 <= algorithm.max_output_len,
        "derived key too long"
    );

    // Create a vector with the expected output length.
    let mut derived_buf = vec![0u8; previously_derived.len()];

    try_derive(algorithm, iterations, salt, secret, &mut derived_buf)?;

    let result = constant_time::verify_slices_are_equal(&derived_buf, previously_derived);
    derived_buf.zeroize();
    result
}

#[cfg(test)]
mod tests {
    use crate::pbkdf2;
    use core::num::NonZeroU32;

    #[cfg(feature = "fips")]
    mod fips;

    #[test]
    fn pbkdf2_coverage() {
        // Coverage sanity check.
        assert!(pbkdf2::PBKDF2_HMAC_SHA256 == pbkdf2::PBKDF2_HMAC_SHA256);
        assert!(pbkdf2::PBKDF2_HMAC_SHA256 != pbkdf2::PBKDF2_HMAC_SHA384);

        let iterations = NonZeroU32::new(100_u32).unwrap();
        for &alg in &[
            pbkdf2::PBKDF2_HMAC_SHA1,
            pbkdf2::PBKDF2_HMAC_SHA256,
            pbkdf2::PBKDF2_HMAC_SHA384,
            pbkdf2::PBKDF2_HMAC_SHA512,
        ] {
            let mut out = vec![0u8; 64];
            pbkdf2::derive(alg, iterations, b"salt", b"password", &mut out);

            let alg_clone = alg;
            let mut out2 = vec![0u8; 64];
            pbkdf2::derive(alg_clone, iterations, b"salt", b"password", &mut out2);
            assert_eq!(out, out2);
        }
    }
}
