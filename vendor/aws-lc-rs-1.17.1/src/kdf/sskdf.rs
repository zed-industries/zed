// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![allow(clippy::module_name_repetitions)]

use crate::aws_lc::{SSKDF_digest, SSKDF_hmac, EVP_MD};

use crate::digest::{match_digest_type, AlgorithmID};
use crate::error::Unspecified;
use crate::ptr::ConstPointer;

/// SSKDF with HMAC-SHA224
#[allow(dead_code)]
const SSKDF_HMAC_SHA224: SskdfHmacAlgorithm = SskdfHmacAlgorithm {
    id: SskdfHmacAlgorithmId::Sha224,
};

/// SSKDF with HMAC-SHA256
#[allow(dead_code)]
const SSKDF_HMAC_SHA256: SskdfHmacAlgorithm = SskdfHmacAlgorithm {
    id: SskdfHmacAlgorithmId::Sha256,
};

/// SSKDF with HMAC-SHA384
#[allow(dead_code)]
const SSKDF_HMAC_SHA384: SskdfHmacAlgorithm = SskdfHmacAlgorithm {
    id: SskdfHmacAlgorithmId::Sha384,
};

/// SSKDF with HMAC-SHA512
#[allow(dead_code)]
const SSKDF_HMAC_SHA512: SskdfHmacAlgorithm = SskdfHmacAlgorithm {
    id: SskdfHmacAlgorithmId::Sha512,
};

/// SSKDF with SHA224
#[allow(dead_code)]
const SSKDF_DIGEST_SHA224: SskdfDigestAlgorithm = SskdfDigestAlgorithm {
    id: SskdfDigestAlgorithmId::Sha224,
};

/// SSKDF with SHA256
#[allow(dead_code)]
const SSKDF_DIGEST_SHA256: SskdfDigestAlgorithm = SskdfDigestAlgorithm {
    id: SskdfDigestAlgorithmId::Sha256,
};

/// SSKDF with SHA384
#[allow(dead_code)]
const SSKDF_DIGEST_SHA384: SskdfDigestAlgorithm = SskdfDigestAlgorithm {
    id: SskdfDigestAlgorithmId::Sha384,
};

/// SSKDF with SHA512
#[allow(dead_code)]
const SSKDF_DIGEST_SHA512: SskdfDigestAlgorithm = SskdfDigestAlgorithm {
    id: SskdfDigestAlgorithmId::Sha512,
};

/// Retrieve [`SskdfHmacAlgorithm`] using the [`SskdfHmacAlgorithmId`] specified by `id`.
#[must_use]
pub const fn get_sskdf_hmac_algorithm(
    id: SskdfHmacAlgorithmId,
) -> Option<&'static SskdfHmacAlgorithm> {
    {
        match id {
            SskdfHmacAlgorithmId::Sha224 => Some(&SSKDF_HMAC_SHA224),
            SskdfHmacAlgorithmId::Sha256 => Some(&SSKDF_HMAC_SHA256),
            SskdfHmacAlgorithmId::Sha384 => Some(&SSKDF_HMAC_SHA384),
            SskdfHmacAlgorithmId::Sha512 => Some(&SSKDF_HMAC_SHA512),
        }
    }
}

/// Retrieve [`SskdfDigestAlgorithm`] using the [`SskdfDigestAlgorithmId`] specified by `id`.
#[must_use]
pub const fn get_sskdf_digest_algorithm(
    id: SskdfDigestAlgorithmId,
) -> Option<&'static SskdfDigestAlgorithm> {
    {
        match id {
            SskdfDigestAlgorithmId::Sha224 => Some(&SSKDF_DIGEST_SHA224),
            SskdfDigestAlgorithmId::Sha256 => Some(&SSKDF_DIGEST_SHA256),
            SskdfDigestAlgorithmId::Sha384 => Some(&SSKDF_DIGEST_SHA384),
            SskdfDigestAlgorithmId::Sha512 => Some(&SSKDF_DIGEST_SHA512),
        }
    }
}

/// SSKDF algorithm using HMAC
pub struct SskdfHmacAlgorithm {
    id: SskdfHmacAlgorithmId,
}

impl SskdfHmacAlgorithm {
    /// Returns the SSKDF HMAC Algorithm Identifier
    #[must_use]
    pub fn id(&self) -> SskdfHmacAlgorithmId {
        self.id
    }

    #[must_use]
    fn get_evp_md(&self) -> ConstPointer<'_, EVP_MD> {
        match_digest_type(match self.id {
            SskdfHmacAlgorithmId::Sha224 => &AlgorithmID::SHA224,
            SskdfHmacAlgorithmId::Sha256 => &AlgorithmID::SHA256,
            SskdfHmacAlgorithmId::Sha384 => &AlgorithmID::SHA384,
            SskdfHmacAlgorithmId::Sha512 => &AlgorithmID::SHA512,
        })
    }
}

impl PartialEq for SskdfHmacAlgorithm {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SskdfHmacAlgorithm {}

impl core::fmt::Debug for SskdfHmacAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.id, f)
    }
}

/// SSKDF algorithm using digest
pub struct SskdfDigestAlgorithm {
    id: SskdfDigestAlgorithmId,
}

impl SskdfDigestAlgorithm {
    /// Returns the SSKDF Algorithm Identifier
    #[must_use]
    pub fn id(&self) -> SskdfDigestAlgorithmId {
        self.id
    }

    #[must_use]
    fn get_evp_md(&self) -> ConstPointer<'_, EVP_MD> {
        match_digest_type(match self.id {
            SskdfDigestAlgorithmId::Sha224 => &AlgorithmID::SHA224,
            SskdfDigestAlgorithmId::Sha256 => &AlgorithmID::SHA256,
            SskdfDigestAlgorithmId::Sha384 => &AlgorithmID::SHA384,
            SskdfDigestAlgorithmId::Sha512 => &AlgorithmID::SHA512,
        })
    }
}

impl PartialEq for SskdfDigestAlgorithm {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SskdfDigestAlgorithm {}

impl core::fmt::Debug for SskdfDigestAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.id, f)
    }
}

/// Single-step (One-step) Key Derivation Function Digest Algorithm Identifier
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SskdfDigestAlgorithmId {
    /// SSKDF with SHA224
    Sha224,

    /// SSKDF with SHA256
    Sha256,

    /// SSKDF with SHA384
    Sha384,

    /// SSKDF with SHA512
    Sha512,
}

/// Single-step (One-step) Key Derivation Function HMAC Algorithm Identifier
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SskdfHmacAlgorithmId {
    /// SSKDF with HMAC-SHA224
    Sha224,

    /// SSKDF with HMAC-SHA256
    Sha256,

    /// SSKDF with HMAC-SHA384
    Sha384,

    /// SSKDF with HMAC-SHA512
    Sha512,
}

/// # Single-step Key Derivation Function using HMAC
///
/// This algorithm may be referred to as "Single-Step KDF" or "NIST Concatenation KDF" by other
/// implementors.
///
/// ## Input Validation and Defaults
/// * `output.len()`, `secret.len()`, `info.len()` each must be <= 2^30.
/// * The default salt, an all zero byte string with length equal to the digest block length, is used
///   if `salt.len() == 0`.
/// * `output.len() > 0 and `secret.len() > 0`
///
/// ## Implementation Notes
///
/// This implementation adheres to the algorithm specified in Section 4 of the
/// NIST Special Publication 800-56C Revision 2 published on August 2020.
/// Using Option 2 for the auxiliary function H.
///
/// Specification is available at <https://doi.org/10.6028/NIST.SP.800-56Cr2>
///
/// # Errors
/// `Unspecified` is returned if input validation fails or an unexpected error occurs.
pub fn sskdf_hmac(
    algorithm: &'static SskdfHmacAlgorithm,
    secret: &[u8],
    info: &[u8],
    salt: &[u8],
    output: &mut [u8],
) -> Result<(), Unspecified> {
    let evp_md = algorithm.get_evp_md();
    let out_len = output.len();
    if 1 != unsafe {
        SSKDF_hmac(
            output.as_mut_ptr(),
            out_len,
            evp_md.as_const_ptr(),
            secret.as_ptr(),
            secret.len(),
            info.as_ptr(),
            info.len(),
            salt.as_ptr(),
            salt.len(),
        )
    } {
        return Err(Unspecified);
    }
    Ok(())
}

/// # Single-step Key Derivation Function using digest
///
/// This algorithm may be referred to as "Single-Step KDF" or "NIST Concatenation KDF" by other
/// implementors.
///
/// ## Input Validation and Defaults
/// * `output.len()`, `secret.len()`, `info.len()` each must be <= 2^30.
/// * `output.len() > 0 and `secret.len() > 0`
///
/// ## Implementation Notes
///
/// This implementation adheres to the algorithm specified in Section 4 of the
/// NIST Special Publication 800-56C Revision 2 published on August 2020.
/// Using Option 1 for the auxiliary function H.
///
/// Specification is available at <https://doi.org/10.6028/NIST.SP.800-56Cr2>
///
/// # Errors
/// `Unspecified` is returned if input validation fails or an unexpected error occurs.
pub fn sskdf_digest(
    algorithm: &'static SskdfDigestAlgorithm,
    secret: &[u8],
    info: &[u8],
    output: &mut [u8],
) -> Result<(), Unspecified> {
    let evp_md = algorithm.get_evp_md();
    let out_len = output.len();
    if 1 != unsafe {
        SSKDF_digest(
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
