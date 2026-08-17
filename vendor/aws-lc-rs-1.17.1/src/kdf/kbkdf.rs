// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![allow(clippy::module_name_repetitions)]

use crate::aws_lc::{KBKDF_ctr_hmac, EVP_MD};

use crate::digest::{match_digest_type, AlgorithmID};
use crate::error::Unspecified;
use crate::ptr::ConstPointer;

/// KBKDF in Counter Mode with HMAC-SHA224
#[allow(dead_code)]
const KBKDF_CTR_HMAC_SHA224: KbkdfCtrHmacAlgorithm = KbkdfCtrHmacAlgorithm {
    id: KbkdfCtrHmacAlgorithmId::Sha224,
};

/// KBKDF in Counter Mode with HMAC-SHA256
#[allow(dead_code)]
const KBKDF_CTR_HMAC_SHA256: KbkdfCtrHmacAlgorithm = KbkdfCtrHmacAlgorithm {
    id: KbkdfCtrHmacAlgorithmId::Sha256,
};

/// KBKDF in Counter Mode with HMAC-SHA384
#[allow(dead_code)]
const KBKDF_CTR_HMAC_SHA384: KbkdfCtrHmacAlgorithm = KbkdfCtrHmacAlgorithm {
    id: KbkdfCtrHmacAlgorithmId::Sha384,
};

/// KBKDF in Counter Mode with HMAC-SHA512
#[allow(dead_code)]
const KBKDF_CTR_HMAC_SHA512: KbkdfCtrHmacAlgorithm = KbkdfCtrHmacAlgorithm {
    id: KbkdfCtrHmacAlgorithmId::Sha512,
};

/// Retrieve [`KbkdfCtrHmacAlgorithm`] using the [`KbkdfCtrHmacAlgorithmId`] specified by `id`.
#[must_use]
pub const fn get_kbkdf_ctr_hmac_algorithm(
    id: KbkdfCtrHmacAlgorithmId,
) -> Option<&'static KbkdfCtrHmacAlgorithm> {
    {
        Some(match id {
            KbkdfCtrHmacAlgorithmId::Sha224 => &KBKDF_CTR_HMAC_SHA224,
            KbkdfCtrHmacAlgorithmId::Sha256 => &KBKDF_CTR_HMAC_SHA256,
            KbkdfCtrHmacAlgorithmId::Sha384 => &KBKDF_CTR_HMAC_SHA384,
            KbkdfCtrHmacAlgorithmId::Sha512 => &KBKDF_CTR_HMAC_SHA512,
        })
    }
}

/// KBKDF in Counter Mode with HMAC Algorithm
pub struct KbkdfCtrHmacAlgorithm {
    id: KbkdfCtrHmacAlgorithmId,
}

impl KbkdfCtrHmacAlgorithm {
    /// Returns the KBKDF Counter HMAC Algorithm Identifier
    #[must_use]
    pub fn id(&self) -> KbkdfCtrHmacAlgorithmId {
        self.id
    }

    #[must_use]
    fn get_evp_md(&self) -> ConstPointer<'_, EVP_MD> {
        match_digest_type(match self.id {
            KbkdfCtrHmacAlgorithmId::Sha224 => &AlgorithmID::SHA224,
            KbkdfCtrHmacAlgorithmId::Sha256 => &AlgorithmID::SHA256,
            KbkdfCtrHmacAlgorithmId::Sha384 => &AlgorithmID::SHA384,
            KbkdfCtrHmacAlgorithmId::Sha512 => &AlgorithmID::SHA512,
        })
    }
}

impl PartialEq for KbkdfCtrHmacAlgorithm {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for KbkdfCtrHmacAlgorithm {}

impl core::fmt::Debug for KbkdfCtrHmacAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.id, f)
    }
}

/// Key-based Derivation Function Algorithm Identifier
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KbkdfCtrHmacAlgorithmId {
    /// KBKDF in Counter Mode with HMAC-SHA224
    Sha224,

    /// KBKDF in Counter Mode with HMAC-SHA256
    Sha256,

    /// KBKDF in Counter Mode with HMAC-SHA384
    Sha384,

    /// KBKDF in Counter Mode with HMAC-SHA512
    Sha512,
}

/// # Key-based Key Derivation Function (KBKDF) in Counter Mode with HMAC PRF
///
/// ## Input Validation and Defaults
/// * `output.len() > 0 and `secret.len() > 0`
/// * `output.len() <= usize::MAX - DIGEST_LENGTH`
/// * The requested `output.len()` would result in overflowing the counter.
///
/// ## Implementation Notes
///
/// This implementation adheres to the algorithm specified in Section 4.1 of the
/// NIST Special Publication 800-108 Revision 1 Update 1 published on August
/// 2022. Using HMAC as the PRF function. In this implementation:
/// * The counter is 32-bits and is represented in big-endian format
/// * The counter is placed before the fixed info string
///
/// Specification available at <https://doi.org/10.6028/NIST.SP.800-108r1-upd1>
///
/// # Errors
/// `Unspecified` is returned if input validation fails or an unexpected error occurs.
pub fn kbkdf_ctr_hmac(
    algorithm: &'static KbkdfCtrHmacAlgorithm,
    secret: &[u8],
    info: &[u8],
    output: &mut [u8],
) -> Result<(), Unspecified> {
    let evp_md = algorithm.get_evp_md();
    let out_len = output.len();
    if 1 != unsafe {
        KBKDF_ctr_hmac(
            output.as_mut_ptr(),
            out_len,
            evp_md.as_const_ptr(),
            secret.as_ptr(),
            secret.len(),
            info.as_ptr(),
            info.len(),
        )
    } {
        return Err(Unspecified);
    }
    Ok(())
}
