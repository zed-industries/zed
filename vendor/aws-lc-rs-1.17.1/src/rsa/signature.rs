// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use std::fmt::{self, Debug, Formatter};
use std::ops::RangeInclusive;

use crate::aws_lc::{
    EVP_PKEY_CTX_set_rsa_padding, EVP_PKEY_CTX_set_rsa_pss_saltlen, EVP_PKEY_CTX_set_signature_md,
    RSA_bits, EVP_PKEY, EVP_PKEY_CTX, RSA_PKCS1_PSS_PADDING, RSA_PSS_SALTLEN_DIGEST,
};

use crate::digest::{self, match_digest_type, Digest};
use crate::error::Unspecified;
use crate::ptr::LcPtr;
use crate::rsa::key::parse_rsa_public_key;
use crate::sealed::Sealed;
use crate::signature::{ParsedPublicKey, ParsedVerificationAlgorithm, VerificationAlgorithm};

use super::encoding;
#[cfg(feature = "ring-sig-verify")]
use untrusted::Input;

#[allow(non_camel_case_types)]
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum RsaPadding {
    RSA_PKCS1_PADDING,
    RSA_PKCS1_PSS_PADDING,
}

/// Parameters for RSA verification.
pub struct RsaParameters(
    &'static digest::Algorithm,
    &'static RsaPadding,
    RangeInclusive<u32>,
    &'static RsaVerificationAlgorithmId,
);

impl RsaParameters {
    #[inline]
    pub(crate) fn digest_algorithm(&self) -> &'static digest::Algorithm {
        self.0
    }

    #[inline]
    pub(crate) fn padding(&self) -> &'static RsaPadding {
        self.1
    }

    #[inline]
    pub(crate) fn bit_size_range(&self) -> &RangeInclusive<u32> {
        &self.2
    }
}

impl ParsedVerificationAlgorithm for RsaParameters {
    fn parsed_verify_sig(
        &self,
        public_key: &ParsedPublicKey,
        msg: &[u8],
        signature: &[u8],
    ) -> Result<(), Unspecified> {
        let evp_pkey = public_key.key();
        verify_rsa_signature(
            self.digest_algorithm(),
            self.padding(),
            evp_pkey,
            msg,
            signature,
            self.bit_size_range(),
        )
    }

    fn parsed_verify_digest_sig(
        &self,
        public_key: &ParsedPublicKey,
        digest: &Digest,
        signature: &[u8],
    ) -> Result<(), Unspecified> {
        let evp_pkey = public_key.key();
        verify_rsa_digest_signature(
            self.padding(),
            evp_pkey,
            digest,
            signature,
            self.bit_size_range(),
        )
    }
}

impl VerificationAlgorithm for RsaParameters {
    #[cfg(feature = "ring-sig-verify")]
    fn verify(
        &self,
        public_key: Input<'_>,
        msg: Input<'_>,
        signature: Input<'_>,
    ) -> Result<(), Unspecified> {
        self.verify_sig(
            public_key.as_slice_less_safe(),
            msg.as_slice_less_safe(),
            signature.as_slice_less_safe(),
        )
    }

    fn verify_sig(
        &self,
        public_key: &[u8],
        msg: &[u8],
        signature: &[u8],
    ) -> Result<(), Unspecified> {
        let evp_pkey = parse_rsa_public_key(public_key)?;
        verify_rsa_signature(
            self.digest_algorithm(),
            self.padding(),
            &evp_pkey,
            msg,
            signature,
            self.bit_size_range(),
        )
    }

    fn verify_digest_sig(
        &self,
        public_key: &[u8],
        digest: &Digest,
        signature: &[u8],
    ) -> Result<(), Unspecified> {
        if self.digest_algorithm() != digest.algorithm() {
            return Err(Unspecified);
        }
        let evp_pkey = parse_rsa_public_key(public_key)?;
        verify_rsa_digest_signature(
            self.padding(),
            &evp_pkey,
            digest,
            signature,
            self.bit_size_range(),
        )
    }
}

impl Sealed for RsaParameters {}

impl Debug for RsaParameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{{ {:?} }}", self.3))
    }
}

impl RsaParameters {
    pub(crate) const fn new(
        digest_alg: &'static digest::Algorithm,
        padding: &'static RsaPadding,
        range: RangeInclusive<u32>,
        verification_alg: &'static RsaVerificationAlgorithmId,
    ) -> Self {
        Self(digest_alg, padding, range, verification_alg)
    }

    /// Parses a DER-encoded `RSAPublicKey` structure (RFC 8017) to determine its size in bits.
    ///
    /// # Errors
    /// `error::Unspecified` on parse error.
    pub fn public_modulus_len(public_key: &[u8]) -> Result<u32, Unspecified> {
        let rsa = encoding::rfc8017::decode_public_key_der(public_key)?;
        Ok(unsafe { RSA_bits(rsa.as_const().get_rsa()?.as_const_ptr()) })
    }

    #[must_use]
    /// Minimum modulus length in bits.
    pub fn min_modulus_len(&self) -> u32 {
        *self.2.start()
    }

    #[must_use]
    /// Maximum modulus length in bits.
    pub fn max_modulus_len(&self) -> u32 {
        *self.2.end()
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum RsaVerificationAlgorithmId {
    RSA_PKCS1_1024_8192_SHA1_FOR_LEGACY_USE_ONLY,
    RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY,
    RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY,
    RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY,
    RSA_PKCS1_2048_8192_SHA256,
    RSA_PKCS1_2048_8192_SHA384,
    RSA_PKCS1_2048_8192_SHA512,
    RSA_PKCS1_3072_8192_SHA384,
    RSA_PSS_2048_8192_SHA256,
    RSA_PSS_2048_8192_SHA384,
    RSA_PSS_2048_8192_SHA512,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum RsaSigningAlgorithmId {
    RSA_PSS_SHA256,
    RSA_PSS_SHA384,
    RSA_PSS_SHA512,
    RSA_PKCS1_SHA256,
    RSA_PKCS1_SHA384,
    RSA_PKCS1_SHA512,
}

#[allow(clippy::module_name_repetitions)]
/// Encoding type for an RSA signature
pub struct RsaSignatureEncoding(
    &'static digest::Algorithm,
    &'static RsaPadding,
    &'static RsaSigningAlgorithmId,
);

impl RsaSignatureEncoding {
    pub(crate) const fn new(
        digest_alg: &'static digest::Algorithm,
        padding: &'static RsaPadding,
        sig_alg: &'static RsaSigningAlgorithmId,
    ) -> Self {
        Self(digest_alg, padding, sig_alg)
    }

    #[inline]
    pub(super) fn digest_algorithm(&self) -> &'static digest::Algorithm {
        self.0
    }

    #[inline]
    pub(super) fn padding(&self) -> &'static RsaPadding {
        self.1
    }
}

impl Sealed for RsaSignatureEncoding {}

/// An RSA signature encoding as described in [RFC 3447 Section 8].
///
/// [RFC 3447 Section 8]: https://tools.ietf.org/html/rfc3447#section-8
#[allow(clippy::module_name_repetitions)]
pub trait RsaEncoding: 'static + Sync + Sealed + Debug {
    /// The signature encoding.
    fn encoding(&'static self) -> &'static RsaSignatureEncoding;
}

impl RsaEncoding for RsaSignatureEncoding {
    fn encoding(&'static self) -> &'static RsaSignatureEncoding {
        self
    }
}

impl Debug for RsaSignatureEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&format!("{{ {:?} }}", self.2))
    }
}

#[inline]
pub(crate) fn configure_rsa_pkcs1_pss_padding(pctx: *mut EVP_PKEY_CTX) -> Result<(), ()> {
    if 1 != unsafe { EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING) } {
        return Err(());
    }
    if 1 != unsafe { EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, RSA_PSS_SALTLEN_DIGEST) } {
        return Err(());
    }
    Ok(())
}

#[inline]
pub(crate) fn verify_rsa_signature(
    algorithm: &'static digest::Algorithm,
    padding: &'static RsaPadding,
    public_key: &LcPtr<EVP_PKEY>,
    msg: &[u8],
    signature: &[u8],
    allowed_bit_size: &RangeInclusive<u32>,
) -> Result<(), Unspecified> {
    if !allowed_bit_size.contains(&public_key.as_const().key_size_bits().try_into()?) {
        return Err(Unspecified);
    }

    let padding_fn = if let RsaPadding::RSA_PKCS1_PSS_PADDING = padding {
        Some(configure_rsa_pkcs1_pss_padding)
    } else {
        None
    };

    public_key.verify(msg, Some(algorithm), padding_fn, signature)
}

#[inline]
pub(crate) fn verify_rsa_digest_signature(
    padding: &'static RsaPadding,
    public_key: &LcPtr<EVP_PKEY>,
    digest: &Digest,
    signature: &[u8],
    allowed_bit_size: &RangeInclusive<u32>,
) -> Result<(), Unspecified> {
    if !allowed_bit_size.contains(&public_key.as_const().key_size_bits().try_into()?) {
        return Err(Unspecified);
    }

    let padding_fn = Some({
        |pctx: *mut EVP_PKEY_CTX| {
            let evp_md = match_digest_type(&digest.algorithm().id);
            if 1 != unsafe { EVP_PKEY_CTX_set_signature_md(pctx, evp_md.as_const_ptr()) } {
                return Err(());
            }
            if let RsaPadding::RSA_PKCS1_PSS_PADDING = padding {
                configure_rsa_pkcs1_pss_padding(pctx)
            } else {
                Ok(())
            }
        }
    });

    public_key.verify_digest_sig(digest, padding_fn, signature)
}
