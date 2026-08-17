// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! TLS 1.2 PRF API's for usage in [RFC 5246](https://www.rfc-editor.org/rfc/rfc5246) and [RFC 7627](https://www.rfc-editor.org/rfc/rfc7627).
//!
//! # Example
//!
//! ```rust
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use aws_lc_rs::tls_prf::{Secret, P_SHA256};
//!
//! let pre_master_secret = &[42; 32]; // Value is established during key exchange
//! let session_hash = &[7; 32]; // Session hash of handshake log
//!
//! let secret = Secret::new(&P_SHA256, pre_master_secret)?;
//!
//! let derived_secret = secret.derive(b"extended master secret", session_hash, 48)?;
//!
//! let derived_secret_bytes = derived_secret.as_ref();
//!
//! assert_eq!(derived_secret_bytes.len(), 48);
//! # Ok(())
//! # }
//! ```

use core::fmt::Debug;

use crate::digest::{match_digest_type, AlgorithmID};
use crate::error::Unspecified;
use crate::fips::indicator_check;
use core::ptr::null;

use crate::aws_lc::CRYPTO_tls1_prf;

/// The TLS PRF `P_hash` Algorithm
pub struct Algorithm(AlgorithmID);

impl Debug for Algorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

/// SHA-256 `P_hash` algorithm
pub const P_SHA256: Algorithm = Algorithm(AlgorithmID::SHA256);

/// SHA-384 `P_hash` algorithm
pub const P_SHA384: Algorithm = Algorithm(AlgorithmID::SHA384);

/// SHA-512 `P_hash` algorithm
pub const P_SHA512: Algorithm = Algorithm(AlgorithmID::SHA512);

/// Encapsulates a PRF algorithm and secret bytes to be used to derive output.
pub struct Secret {
    algorithm: &'static Algorithm,
    secret: Box<[u8]>,
}

impl Secret {
    /// Constructs a new `Secret` for use with the associated `P_hash` `Algorithm`.
    ///
    /// # Errors
    /// * `Unspecified`: If `secret.is_empty() == true`.
    pub fn new(algorithm: &'static Algorithm, secret: &[u8]) -> Result<Self, Unspecified> {
        if secret.is_empty() {
            return Err(Unspecified);
        }

        let secret = Vec::from(secret).into_boxed_slice();

        Ok(Self { algorithm, secret })
    }

    /// Calculates `len` bytes of TLS PRF using the configured [`Algorithm`], and returns [`Secret`] of length `len`.
    /// See [RFC5246](https://datatracker.ietf.org/doc/html/rfc5246#section-5)
    ///
    /// # Errors
    /// * `Unspecified`: Returned if the PRF derivation fails.
    pub fn derive(self, label: &[u8], seed: &[u8], output: usize) -> Result<Secret, Unspecified> {
        prf(self.algorithm, &self.secret, label, seed, None, output)
    }

    /// Calculates `len` bytes of TLS PRF using the configured [`Algorithm`], and returns [`Secret`] of length `len`.
    ///
    /// In this method, `seed1` and `seed2` will be concatenated together: `seed1 || seed2`.
    ///
    /// See [RFC5246](https://datatracker.ietf.org/doc/html/rfc5246#section-5)
    ///
    /// # Errors
    /// * `Unspecified`: Returned if the PRF derivation fails.
    pub fn derive_with_seed_concatination(
        self,
        label: &[u8],
        seed1: &[u8],
        seed2: &[u8],
        len: usize,
    ) -> Result<Secret, Unspecified> {
        prf(self.algorithm, &self.secret, label, seed1, Some(seed2), len)
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.secret.zeroize();
    }
}

impl AsRef<[u8]> for Secret {
    fn as_ref(&self) -> &[u8] {
        &self.secret
    }
}

impl<const L: usize> TryFrom<Secret> for [u8; L] {
    type Error = Unspecified;

    fn try_from(value: Secret) -> Result<Self, Self::Error> {
        if value.secret.len() != L {
            return Err(Unspecified);
        }

        let mut ret = [0u8; L];
        ret.copy_from_slice(&value.secret);

        Ok(ret)
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for Secret {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Secret")
            .field("algorithm", &self.algorithm)
            .finish()
    }
}

fn prf(
    algorithm: &'static Algorithm,
    secret: &[u8],
    label: &[u8],
    seed1: &[u8],
    seed2: Option<&[u8]>,
    output: usize,
) -> Result<Secret, Unspecified> {
    if output == 0 {
        return Err(Unspecified);
    }

    let mut output = vec![0u8; output];

    let digest = match_digest_type(&algorithm.0);

    let (seed2, seed2_len) = if let Some(seed2) = seed2 {
        (seed2.as_ptr(), seed2.len())
    } else {
        (null(), 0usize)
    };

    if 1 != indicator_check!(unsafe {
        CRYPTO_tls1_prf(
            digest.as_const_ptr(),
            output.as_mut_ptr(),
            output.len(),
            secret.as_ptr(),
            secret.len(),
            label.as_ptr().cast(),
            label.len(),
            seed1.as_ptr(),
            seed1.len(),
            seed2,
            seed2_len,
        )
    }) {
        return Err(Unspecified);
    }

    Ok(Secret {
        algorithm,
        secret: output.into_boxed_slice(),
    })
}

#[cfg(test)]
mod tests {
    use alloc::ffi::CString;

    use super::{Secret, P_SHA256, P_SHA384, P_SHA512};

    #[cfg(feature = "fips")]
    mod fips;

    #[test]
    fn aws_lc_kat() {
        // Testings constants from https://github.com/aws/aws-lc/blob/c5ef1079b605acac1068ac362a7008f30fd8c100/crypto/fipsmodule/self_check/self_check.c#L1130
        const TLS_SECRET: &[u8; 32] = &[
            0xab, 0xc3, 0x65, 0x7b, 0x09, 0x4c, 0x76, 0x28, 0xa0, 0xb2, 0x82, 0x99, 0x6f, 0xe7,
            0x5a, 0x75, 0xf4, 0x98, 0x4f, 0xd9, 0x4d, 0x4e, 0xcc, 0x2f, 0xcf, 0x53, 0xa2, 0xc4,
            0x69, 0xa3, 0xf7, 0x31,
        ];

        const TLS_LABEL: &str = "FIPS self test";

        const TLS_SEED1: &[u8; 16] = &[
            0x8f, 0x0d, 0xe8, 0xb6, 0x90, 0x8f, 0xb1, 0xd2, 0x6d, 0x51, 0xf4, 0x79, 0x18, 0x63,
            0x51, 0x65,
        ];

        const TLS_SEED2: &[u8; 16] = &[
            0x7d, 0x24, 0x1a, 0x9d, 0x3c, 0x59, 0xbf, 0x3c, 0x31, 0x1e, 0x2b, 0x21, 0x41, 0x8d,
            0x32, 0x81,
        ];

        const TLS_OUTPUT_P_SHA256: &[u8; 32] = &[
            0xe2, 0x1d, 0xd6, 0xc2, 0x68, 0xc7, 0x57, 0x03, 0x2c, 0x2c, 0xeb, 0xbb, 0xb8, 0xa9,
            0x7d, 0xe9, 0xee, 0xe6, 0xc9, 0x47, 0x83, 0x0a, 0xbd, 0x11, 0x60, 0x5d, 0xd5, 0x2c,
            0x47, 0xb6, 0x05, 0x88,
        ];

        let secret = Secret::new(&P_SHA256, TLS_SECRET).expect("secret created");

        // The AWS-LC KAT annoyingly includes the null terminator in its KAT, so using CString here to handle that for us.
        let label = CString::new(TLS_LABEL).expect("failed to create CString");

        let output = secret
            .derive_with_seed_concatination(
                label.as_bytes_with_nul(),
                TLS_SEED1,
                TLS_SEED2,
                TLS_OUTPUT_P_SHA256.len(),
            )
            .unwrap();

        assert_eq!(TLS_OUTPUT_P_SHA256, output.as_ref());

        let mut seed = Vec::<u8>::with_capacity(TLS_SEED1.len() + TLS_SEED2.len());
        seed.extend(TLS_SEED1);
        seed.extend(TLS_SEED2);

        let secret = Secret::new(&P_SHA256, TLS_SECRET).expect("secret created");
        let output = secret
            .derive(label.as_bytes_with_nul(), &seed, TLS_OUTPUT_P_SHA256.len())
            .unwrap();

        assert_eq!(TLS_OUTPUT_P_SHA256, output.as_ref());
    }

    #[test]
    fn sha256() {
        // KAT Sourced from https://csrc.nist.gov/Projects/cryptographic-algorithm-validation-program/Component-Testing

        const SECRET: &[u8] = &[
            0xf8, 0x93, 0x8e, 0xcc, 0x9e, 0xde, 0xbc, 0x50, 0x30, 0xc0, 0xc6, 0xa4, 0x41, 0xe2,
            0x13, 0xcd, 0x24, 0xe6, 0xf7, 0x70, 0xa5, 0x0d, 0xda, 0x07, 0x87, 0x6f, 0x8d, 0x55,
            0xda, 0x06, 0x2b, 0xca, 0xdb, 0x38, 0x6b, 0x41, 0x1f, 0xd4, 0xfe, 0x43, 0x13, 0xa6,
            0x04, 0xfc, 0xe6, 0xc1, 0x7f, 0xbc,
        ];
        const LABEL: &[u8] = b"master secret";
        const SEED1: &[u8] = &[
            0x36, 0xc1, 0x29, 0xd0, 0x1a, 0x32, 0x00, 0x89, 0x4b, 0x91, 0x79, 0xfa, 0xac, 0x58,
            0x9d, 0x98, 0x35, 0xd5, 0x87, 0x75, 0xf9, 0xb5, 0xea, 0x35, 0x87, 0xcb, 0x8f, 0xd0,
            0x36, 0x4c, 0xae, 0x8c,
        ];
        const SEED2: &[u8] = &[
            0xf6, 0xc9, 0x57, 0x5e, 0xd7, 0xdd, 0xd7, 0x3e, 0x1f, 0x7d, 0x16, 0xec, 0xa1, 0x15,
            0x41, 0x58, 0x12, 0xa4, 0x3c, 0x2b, 0x74, 0x7d, 0xaa, 0xaa, 0xe0, 0x43, 0xab, 0xfb,
            0x50, 0x05, 0x3f, 0xce,
        ];
        const EXPECT: &[u8] = &[
            0x20, 0x2c, 0x88, 0xc0, 0x0f, 0x84, 0xa1, 0x7a, 0x20, 0x02, 0x70, 0x79, 0x60, 0x47,
            0x87, 0x46, 0x11, 0x76, 0x45, 0x55, 0x39, 0xe7, 0x05, 0xbe, 0x73, 0x08, 0x90, 0x60,
            0x2c, 0x28, 0x9a, 0x50, 0x01, 0xe3, 0x4e, 0xeb, 0x3a, 0x04, 0x3e, 0x5d, 0x52, 0xa6,
            0x5e, 0x66, 0x12, 0x51, 0x88, 0xbf,
        ];

        let secret = Secret::new(&P_SHA256, SECRET).expect("secret created");

        let output = secret
            .derive_with_seed_concatination(LABEL, SEED1, SEED2, EXPECT.len())
            .expect("derive successful");

        assert_eq!(EXPECT, output.as_ref());
    }

    #[test]
    fn sha384() {
        // KAT Sourced from https://csrc.nist.gov/Projects/cryptographic-algorithm-validation-program/Component-Testing

        const SECRET: &[u8] = &[
            0xa5, 0xe2, 0x64, 0x26, 0x33, 0xf5, 0xb8, 0xc8, 0x1a, 0xd3, 0xfe, 0x0c, 0x2f, 0xe3,
            0xa8, 0xe5, 0xef, 0x80, 0x6b, 0x06, 0x12, 0x1d, 0xd1, 0x0d, 0xf4, 0xbb, 0x0f, 0xe8,
            0x57, 0xbf, 0xdc, 0xf5, 0x22, 0x55, 0x8e, 0x05, 0xd2, 0x68, 0x2c, 0x9a, 0x80, 0xc7,
            0x41, 0xa3, 0xaa, 0xb1, 0x71, 0x6f,
        ];
        const LABEL: &[u8] = b"master secret";
        const SEED1: &[u8] = &[
            0xab, 0xe4, 0xbf, 0x55, 0x27, 0x42, 0x9a, 0xc8, 0xeb, 0x13, 0x57, 0x4d, 0x27, 0x09,
            0xe8, 0x01, 0x2b, 0xd1, 0xa1, 0x13, 0xc6, 0xd3, 0xb1, 0xd3, 0xaa, 0x2c, 0x38, 0x40,
            0x51, 0x87, 0x78, 0xac,
        ];
        const SEED2: &[u8] = &[
            0xcb, 0x6e, 0x0b, 0x3e, 0xb0, 0x29, 0x76, 0xb6, 0x46, 0x6d, 0xfa, 0x96, 0x51, 0xc2,
            0x91, 0x94, 0x14, 0xf1, 0x64, 0x8f, 0xd3, 0xa7, 0x83, 0x8d, 0x02, 0x15, 0x3e, 0x5b,
            0xd3, 0x95, 0x35, 0xb6,
        ];
        const EXPECT: &[u8] = &[
            0xb4, 0xd4, 0x9b, 0xfa, 0x87, 0x74, 0x7f, 0xe8, 0x15, 0x45, 0x7b, 0xc3, 0xda, 0x15,
            0x07, 0x3d, 0x6a, 0xc7, 0x33, 0x89, 0xe7, 0x03, 0x07, 0x9a, 0x35, 0x03, 0xc0, 0x9e,
            0x14, 0xbd, 0x55, 0x9a, 0x5b, 0x3c, 0x7c, 0x60, 0x1c, 0x73, 0x65, 0xf6, 0xea, 0x8c,
            0x68, 0xd3, 0xd9, 0x59, 0x68, 0x27,
        ];

        let secret = Secret::new(&P_SHA384, SECRET).expect("secret created");

        let output = secret
            .derive_with_seed_concatination(LABEL, SEED1, SEED2, EXPECT.len())
            .expect("derive successful");

        assert_eq!(EXPECT, output.as_ref());
    }

    #[test]
    fn sha512() {
        // KAT Sourced from https://csrc.nist.gov/Projects/cryptographic-algorithm-validation-program/Component-Testing

        const SECRET: &[u8] = &[
            0xdf, 0xef, 0x39, 0xaf, 0x25, 0xc1, 0x26, 0x63, 0xa9, 0x1e, 0xe5, 0xd2, 0x70, 0x42,
            0xb9, 0x64, 0x4a, 0x16, 0xef, 0x55, 0xb8, 0x10, 0x55, 0xd1, 0xbd, 0x7d, 0xcb, 0x0b,
            0x8f, 0x06, 0xeb, 0x00, 0x17, 0x08, 0xcd, 0xef, 0xcf, 0x82, 0x59, 0x1d, 0xef, 0xca,
            0x1a, 0x6f, 0x1a, 0xc6, 0x93, 0xab,
        ];
        const LABEL: &[u8] = b"master secret";
        const SEED1: &[u8] = &[
            0x78, 0xbc, 0x52, 0x98, 0xdf, 0xe9, 0xcf, 0x8e, 0xd3, 0x36, 0xc2, 0xe2, 0xf0, 0xf6,
            0xb4, 0x6e, 0x24, 0x56, 0xf3, 0x9f, 0x35, 0xf1, 0x14, 0x3c, 0xd2, 0x1e, 0xaa, 0x16,
            0x27, 0x70, 0x25, 0xb2,
        ];
        const SEED2: &[u8] = &[
            0xe2, 0x33, 0x9a, 0x6c, 0x68, 0x1e, 0xb3, 0x08, 0x08, 0x88, 0x39, 0x71, 0xb1, 0xce,
            0x5b, 0x9b, 0x1e, 0xce, 0x0f, 0x3d, 0x01, 0x1a, 0x96, 0xa7, 0xff, 0xf1, 0xf5, 0xf9,
            0xd8, 0x0f, 0xfd, 0x4b,
        ];
        const EXPECT: &[u8] = &[
            0xa7, 0x0c, 0x5f, 0xe8, 0xd3, 0x4b, 0x64, 0x5a, 0x20, 0xce, 0x98, 0x96, 0x9b, 0xd3,
            0x08, 0x58, 0xe7, 0x29, 0xc7, 0x7c, 0x8a, 0x5f, 0x05, 0xd3, 0xe2, 0x89, 0x21, 0x9d,
            0x6b, 0x57, 0x52, 0xb7, 0x5b, 0x75, 0xe1, 0xca, 0x00, 0xd3, 0x32, 0x96, 0x58, 0xd7,
            0xf1, 0x88, 0xed, 0x1a, 0xb7, 0xe0,
        ];

        let secret = Secret::new(&P_SHA512, SECRET).expect("secret created");

        let output = secret
            .derive_with_seed_concatination(LABEL, SEED1, SEED2, EXPECT.len())
            .expect("derive successful");

        assert_eq!(EXPECT, output.as_ref());
    }

    #[test]
    fn try_into_array() {
        let secret = Secret::new(&P_SHA256, &[42u8; 32]).expect("secret creation to succeed");

        let secret = secret
            .derive("master secret".as_bytes(), &[7u8; 3], 7)
            .expect("derive to succeed");

        let _secret: [u8; 7] = secret.try_into().expect("try_into to succeed");
    }
}
