// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{
    ECDSA_SIG_new, ECDSA_SIG_set0, ECDSA_SIG_to_bytes, NID_X9_62_prime256v1, NID_secp256k1,
    NID_secp384r1, NID_secp521r1, BIGNUM, ECDSA_SIG, EVP_PKEY,
};

use crate::digest::Digest;
use crate::ec::compressed_public_key_size_bytes;
use crate::ec::encoding::parse_ec_public_key;
use crate::ec::encoding::sec1::marshal_sec1_public_point;
use crate::encoding::{
    AsBigEndian, AsDer, EcPublicKeyCompressedBin, EcPublicKeyUncompressedBin, PublicKeyX509Der,
};
use crate::error::Unspecified;
use crate::evp_pkey::No_EVP_PKEY_CTX_consumer;
use crate::ptr::{DetachableLcPtr, LcPtr};
use crate::signature::{ParsedPublicKey, ParsedVerificationAlgorithm, VerificationAlgorithm};
use crate::{digest, sealed};
use core::fmt;
use core::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ptr::null_mut;
#[cfg(feature = "ring-sig-verify")]
use untrusted::Input;

/// An ECDSA verification algorithm.
#[derive(Debug, Eq, PartialEq)]
pub struct EcdsaVerificationAlgorithm {
    pub(crate) id: &'static AlgorithmID,
    pub(crate) digest: &'static digest::Algorithm,
    pub(crate) sig_format: EcdsaSignatureFormat,
}

/// An ECDSA signing algorithm.
#[derive(Debug, Eq, PartialEq)]
pub struct EcdsaSigningAlgorithm(pub(crate) &'static EcdsaVerificationAlgorithm);

impl Deref for EcdsaSigningAlgorithm {
    type Target = EcdsaVerificationAlgorithm;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl sealed::Sealed for EcdsaVerificationAlgorithm {}
impl sealed::Sealed for EcdsaSigningAlgorithm {}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum EcdsaSignatureFormat {
    ASN1,
    Fixed,
}

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum AlgorithmID {
    ECDSA_P256,
    ECDSA_P384,
    ECDSA_P521,
    ECDSA_P256K1,
}

impl AlgorithmID {
    #[inline]
    pub(crate) fn nid(&'static self) -> i32 {
        match self {
            AlgorithmID::ECDSA_P256 => NID_X9_62_prime256v1,
            AlgorithmID::ECDSA_P384 => NID_secp384r1,
            AlgorithmID::ECDSA_P521 => NID_secp521r1,
            AlgorithmID::ECDSA_P256K1 => NID_secp256k1,
        }
    }
    pub(crate) fn private_key_size(&self) -> usize {
        match self {
            AlgorithmID::ECDSA_P256 | AlgorithmID::ECDSA_P256K1 => 32,
            AlgorithmID::ECDSA_P384 => 48,
            AlgorithmID::ECDSA_P521 => 66,
        }
    }
    // Compressed public key length in bytes
    #[inline]
    #[allow(dead_code)]
    const fn compressed_pub_key_len(&self) -> usize {
        match self {
            AlgorithmID::ECDSA_P256 | AlgorithmID::ECDSA_P256K1 => {
                compressed_public_key_size_bytes(256)
            }
            AlgorithmID::ECDSA_P384 => compressed_public_key_size_bytes(384),
            AlgorithmID::ECDSA_P521 => compressed_public_key_size_bytes(521),
        }
    }
}

/// Elliptic curve public key.
#[derive(Clone)]
pub struct PublicKey {
    #[allow(dead_code)]
    algorithm: &'static EcdsaSigningAlgorithm,
    evp_pkey: LcPtr<EVP_PKEY>,
    octets: Box<[u8]>,
}

pub(crate) fn public_key_from_evp_pkey(
    evp_pkey: &LcPtr<EVP_PKEY>,
    algorithm: &'static EcdsaSigningAlgorithm,
) -> Result<PublicKey, Unspecified> {
    let pub_key_bytes = marshal_sec1_public_point(evp_pkey, false)?;

    Ok(PublicKey {
        evp_pkey: evp_pkey.clone(),
        algorithm,
        octets: pub_key_bytes.into_boxed_slice(),
    })
}

impl AsDer<PublicKeyX509Der<'static>> for PublicKey {
    /// Provides the public key as a DER-encoded (X.509) `SubjectPublicKeyInfo` structure.
    /// # Errors
    /// Returns an error if the public key fails to marshal to X.509.
    fn as_der(&self) -> Result<PublicKeyX509Der<'static>, Unspecified> {
        let der = self.evp_pkey.as_const().marshal_rfc5280_public_key()?;
        Ok(PublicKeyX509Der::new(der))
    }
}

impl AsBigEndian<EcPublicKeyCompressedBin<'static>> for PublicKey {
    /// Provides the public key elliptic curve point to a compressed point bytes format.
    /// # Errors
    /// Returns an error if the public key fails to marshal.
    fn as_be_bytes(&self) -> Result<EcPublicKeyCompressedBin<'static>, crate::error::Unspecified> {
        let pub_point = marshal_sec1_public_point(&self.evp_pkey, true)?;
        Ok(EcPublicKeyCompressedBin::new(pub_point))
    }
}

impl AsBigEndian<EcPublicKeyUncompressedBin<'static>> for PublicKey {
    /// Provides the public key elliptic curve point to an uncompressed point bytes format.
    /// # Errors
    /// Returns an error if the public key fails to marshal.
    fn as_be_bytes(
        &self,
    ) -> Result<EcPublicKeyUncompressedBin<'static>, crate::error::Unspecified> {
        let mut uncompressed_bytes = vec![0u8; self.octets.len()];
        uncompressed_bytes.copy_from_slice(&self.octets);
        Ok(EcPublicKeyUncompressedBin::new(uncompressed_bytes))
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&format!(
            "EcdsaPublicKey(\"{}\")",
            crate::hex::encode(self.octets.as_ref())
        ))
    }
}

impl AsRef<[u8]> for PublicKey {
    #[inline]
    /// Serializes the public key in an uncompressed form (X9.62) using the
    /// Octet-String-to-Elliptic-Curve-Point algorithm in
    /// [SEC 1: Elliptic Curve Cryptography, Version 2.0].
    fn as_ref(&self) -> &[u8] {
        self.octets.as_ref()
    }
}

unsafe impl Send for PublicKey {}
unsafe impl Sync for PublicKey {}

impl VerificationAlgorithm for EcdsaVerificationAlgorithm {
    #[inline]
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
        let public_key = parse_ec_public_key(public_key, self.id.nid())?;
        self.verify_ecdsa(msg, signature, &public_key)
    }

    fn verify_digest_sig(
        &self,
        public_key: &[u8],
        digest: &digest::Digest,
        signature: &[u8],
    ) -> Result<(), Unspecified> {
        let public_key = parse_ec_public_key(public_key, self.id.nid())?;

        self.verify_digest_ecdsa(digest, signature, &public_key)
    }
}

impl EcdsaVerificationAlgorithm {
    fn verify_ecdsa(
        &self,
        msg: &[u8],
        signature: &[u8],
        public_key: &LcPtr<EVP_PKEY>,
    ) -> Result<(), Unspecified> {
        match self.sig_format {
            EcdsaSignatureFormat::ASN1 => {
                verify_asn1_signature(self.digest, public_key, msg, signature)
            }
            EcdsaSignatureFormat::Fixed => {
                let (out_bytes, out_bytes_len) = convert_fixed_signature(self.id, signature)?;
                verify_asn1_signature(self.digest, public_key, msg, unsafe {
                    out_bytes.as_slice(out_bytes_len)
                })
            }
        }
    }

    fn verify_digest_ecdsa(
        &self,
        digest: &Digest,
        signature: &[u8],
        public_key: &LcPtr<EVP_PKEY>,
    ) -> Result<(), Unspecified> {
        if self.digest != digest.algorithm() {
            return Err(Unspecified);
        }
        match self.sig_format {
            EcdsaSignatureFormat::ASN1 => {
                verify_asn1_digest_signature(digest, public_key, signature)
            }
            EcdsaSignatureFormat::Fixed => {
                let (out_bytes, out_bytes_len) = convert_fixed_signature(self.id, signature)?;
                verify_asn1_digest_signature(digest, public_key, unsafe {
                    out_bytes.as_slice(out_bytes_len)
                })
            }
        }
    }
}

impl ParsedVerificationAlgorithm for EcdsaVerificationAlgorithm {
    fn parsed_verify_sig(
        &self,
        public_key: &ParsedPublicKey,
        msg: &[u8],
        signature: &[u8],
    ) -> Result<(), Unspecified> {
        self.verify_ecdsa(msg, signature, public_key.key())
    }

    fn parsed_verify_digest_sig(
        &self,
        public_key: &ParsedPublicKey,
        digest: &Digest,
        signature: &[u8],
    ) -> Result<(), Unspecified> {
        self.verify_digest_ecdsa(digest, signature, public_key.key())
    }
}

fn convert_fixed_signature(
    alg: &'static AlgorithmID,
    signature: &[u8],
) -> Result<(LcPtr<u8>, usize), Unspecified> {
    let mut out_bytes = null_mut::<u8>();
    let mut out_bytes_len = MaybeUninit::<usize>::uninit();
    let sig = unsafe { ecdsa_sig_from_fixed(alg, signature)? };
    if 1 != unsafe {
        ECDSA_SIG_to_bytes(
            &mut out_bytes,
            out_bytes_len.as_mut_ptr(),
            sig.as_const_ptr(),
        )
    } {
        return Err(Unspecified);
    }
    Ok((LcPtr::new(out_bytes)?, unsafe {
        out_bytes_len.assume_init()
    }))
}

fn verify_asn1_signature(
    digest_alg: &'static digest::Algorithm,
    public_key: &LcPtr<EVP_PKEY>,
    msg: &[u8],
    signature: &[u8],
) -> Result<(), Unspecified> {
    public_key.verify(msg, Some(digest_alg), No_EVP_PKEY_CTX_consumer, signature)
}

fn verify_asn1_digest_signature(
    digest: &Digest,
    public_key: &LcPtr<EVP_PKEY>,
    signature: &[u8],
) -> Result<(), Unspecified> {
    public_key.verify_digest_sig(digest, No_EVP_PKEY_CTX_consumer, signature)
}

#[inline]
unsafe fn ecdsa_sig_from_fixed(
    alg_id: &'static AlgorithmID,
    signature: &[u8],
) -> Result<LcPtr<ECDSA_SIG>, ()> {
    let num_size_bytes = alg_id.private_key_size();
    if signature.len() != 2 * num_size_bytes {
        return Err(());
    }
    let mut r_bn = DetachableLcPtr::<BIGNUM>::try_from(&signature[..num_size_bytes])?;
    let mut s_bn = DetachableLcPtr::<BIGNUM>::try_from(&signature[num_size_bytes..])?;

    let mut ecdsa_sig = LcPtr::new(ECDSA_SIG_new())?;

    if 1 != ECDSA_SIG_set0(ecdsa_sig.as_mut_ptr(), r_bn.as_mut_ptr(), s_bn.as_mut_ptr()) {
        return Err(());
    }
    r_bn.detach();
    s_bn.detach();

    Ok(ecdsa_sig)
}
