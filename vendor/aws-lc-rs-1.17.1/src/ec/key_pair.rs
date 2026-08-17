// Copyright 2015-2016 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{EVP_PKEY, EVP_PKEY_EC};
use crate::digest::Digest;
use crate::ec::evp_key_generate;
use crate::ec::signature::{EcdsaSignatureFormat, EcdsaSigningAlgorithm, PublicKey};
#[cfg(feature = "fips")]
use crate::ec::validate_ec_evp_key;
#[cfg(not(feature = "fips"))]
use crate::ec::verify_evp_key_nid;
use core::fmt;
use core::fmt::{Debug, Formatter};

use crate::ec;
use crate::ec::encoding::rfc5915::{marshal_rfc5915_private_key, parse_rfc5915_private_key};
use crate::ec::encoding::sec1::{
    marshal_sec1_private_key, parse_sec1_private_bn, parse_sec1_public_point,
};
use crate::encoding::{AsBigEndian, AsDer, EcPrivateKeyBin, EcPrivateKeyRfc5915Der};
use crate::error::{KeyRejected, Unspecified};
use crate::evp_pkey::No_EVP_PKEY_CTX_consumer;
use crate::pkcs8::{Document, Version};
use crate::ptr::LcPtr;
use crate::rand::SecureRandom;
use crate::signature::{KeyPair, Signature};

/// An ECDSA key pair, used for signing.
#[allow(clippy::module_name_repetitions)]
pub struct EcdsaKeyPair {
    algorithm: &'static EcdsaSigningAlgorithm,
    evp_pkey: LcPtr<EVP_PKEY>,
    pubkey: PublicKey,
}

impl Debug for EcdsaKeyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&format!("EcdsaKeyPair {{ public_key: {:?} }}", self.pubkey))
    }
}

unsafe impl Send for EcdsaKeyPair {}

unsafe impl Sync for EcdsaKeyPair {}

impl KeyPair for EcdsaKeyPair {
    type PublicKey = PublicKey;

    #[inline]
    /// Provides the public key.
    fn public_key(&self) -> &Self::PublicKey {
        &self.pubkey
    }
}

impl EcdsaKeyPair {
    #[allow(clippy::needless_pass_by_value)]
    fn new(
        algorithm: &'static EcdsaSigningAlgorithm,
        evp_pkey: LcPtr<EVP_PKEY>,
    ) -> Result<Self, ()> {
        let pubkey = ec::signature::public_key_from_evp_pkey(&evp_pkey, algorithm)?;

        Ok(Self {
            algorithm,
            evp_pkey,
            pubkey,
        })
    }

    /// Generates a new key pair.
    ///
    /// # Errors
    /// `error::Unspecified` on internal error.
    ///
    pub fn generate(alg: &'static EcdsaSigningAlgorithm) -> Result<Self, Unspecified> {
        let evp_pkey = evp_key_generate(alg.0.id.nid())?;

        Ok(Self::new(alg, evp_pkey)?)
    }

    /// Constructs an ECDSA key pair by parsing an unencrypted PKCS#8 v1
    /// id-ecPublicKey `ECPrivateKey` key.
    ///
    /// # Errors
    /// `error::KeyRejected` if bytes do not encode an ECDSA key pair or if the key is otherwise not
    /// acceptable.
    pub fn from_pkcs8(
        alg: &'static EcdsaSigningAlgorithm,
        pkcs8: &[u8],
    ) -> Result<Self, KeyRejected> {
        // Includes a call to `EC_KEY_check_key`
        let evp_pkey = LcPtr::<EVP_PKEY>::parse_rfc5208_private_key(pkcs8, EVP_PKEY_EC)?;

        #[cfg(not(feature = "fips"))]
        verify_evp_key_nid(&evp_pkey.as_const(), alg.id.nid())?;
        #[cfg(feature = "fips")]
        validate_ec_evp_key(&evp_pkey.as_const(), alg.id.nid())?;

        let key_pair = Self::new(alg, evp_pkey)?;

        Ok(key_pair)
    }

    /// Generates a new key pair and returns the key pair serialized as a
    /// PKCS#8 v1 document.
    ///
    /// # *ring* Compatibility
    /// Our implementation ignores the `SecureRandom` parameter.
    ///
    /// # Errors
    /// `error::Unspecified` on internal error.
    pub fn generate_pkcs8(
        alg: &'static EcdsaSigningAlgorithm,
        _rng: &dyn SecureRandom,
    ) -> Result<Document, Unspecified> {
        let key_pair = Self::generate(alg)?;

        key_pair.to_pkcs8v1()
    }

    /// Serializes this `EcdsaKeyPair` into a PKCS#8 v1 document.
    ///
    /// # Errors
    /// `error::Unspecified` on internal error.
    ///
    pub fn to_pkcs8v1(&self) -> Result<Document, Unspecified> {
        Ok(Document::new(
            self.evp_pkey
                .as_const()
                .marshal_rfc5208_private_key(Version::V1)?,
        ))
    }

    /// Constructs an ECDSA key pair from the private key and public key bytes
    ///
    /// The private key must encoded as a big-endian fixed-length integer. For
    /// example, a P-256 private key must be 32 bytes prefixed with leading
    /// zeros as needed.
    ///
    /// The public key is encoding in uncompressed form using the
    /// Octet-String-to-Elliptic-Curve-Point algorithm in
    /// [SEC 1: Elliptic Curve Cryptography, Version 2.0].
    ///
    /// This is intended for use by code that deserializes key pairs. It is
    /// recommended to use `EcdsaKeyPair::from_pkcs8()` (with a PKCS#8-encoded
    /// key) instead.
    ///
    /// [SEC 1: Elliptic Curve Cryptography, Version 2.0]:
    ///     http://www.secg.org/sec1-v2.pdf
    ///
    /// # Errors
    /// `error::KeyRejected` if parsing failed or key otherwise unacceptable.
    pub fn from_private_key_and_public_key(
        alg: &'static EcdsaSigningAlgorithm,
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<Self, KeyRejected> {
        let priv_evp_pkey = parse_sec1_private_bn(private_key, alg.id.nid())?;
        let pub_evp_pkey = parse_sec1_public_point(public_key, alg.id.nid())?;
        // EVP_PKEY_cmp only compares params and public key
        if !priv_evp_pkey.eq(&pub_evp_pkey) {
            return Err(KeyRejected::inconsistent_components());
        }

        let key_pair = Self::new(alg, priv_evp_pkey)?;
        Ok(key_pair)
    }

    /// Deserializes a DER-encoded private key structure to produce a `EcdsaKeyPair`.
    ///
    /// This function is typically used to deserialize RFC 5915 encoded private keys, but it will
    /// attempt to automatically detect other key formats. This function supports unencrypted
    /// PKCS#8 `PrivateKeyInfo` structures as well as key type specific formats.
    ///
    /// See `EcdsaPrivateKey::as_der`.
    ///
    /// # Errors
    /// `error::KeyRejected` if parsing failed or key otherwise unacceptable.
    ///
    /// # Panics
    pub fn from_private_key_der(
        alg: &'static EcdsaSigningAlgorithm,
        private_key: &[u8],
    ) -> Result<Self, KeyRejected> {
        let evp_pkey = LcPtr::<EVP_PKEY>::parse_rfc5208_private_key(private_key, EVP_PKEY_EC)
            .or(parse_rfc5915_private_key(private_key, alg.id.nid()))?;
        #[cfg(not(feature = "fips"))]
        verify_evp_key_nid(&evp_pkey.as_const(), alg.id.nid())?;
        #[cfg(feature = "fips")]
        validate_ec_evp_key(&evp_pkey.as_const(), alg.id.nid())?;

        Ok(Self::new(alg, evp_pkey)?)
    }

    /// Access functions related to the private key.
    #[must_use]
    pub fn private_key(&self) -> PrivateKey<'_> {
        PrivateKey(self)
    }

    /// [`EcdsaSigningAlgorithm`] which was used to create this [`EcdsaKeyPair`]
    #[must_use]
    pub fn algorithm(&self) -> &'static EcdsaSigningAlgorithm {
        self.algorithm
    }

    /// Returns a signature for the message.
    ///
    /// # *ring* Compatibility
    /// Our implementation ignores the `SecureRandom` parameter.
    ///
    /// # Errors
    /// `error::Unspecified` on internal error.
    //
    // # FIPS
    // The following conditions must be met:
    // * NIST Elliptic Curves: P256, P384, P521
    // * Digest Algorithms: SHA256, SHA384, SHA512
    #[inline]
    pub fn sign(&self, _rng: &dyn SecureRandom, message: &[u8]) -> Result<Signature, Unspecified> {
        let out_sig = self.evp_pkey.sign(
            message,
            Some(self.algorithm.digest),
            No_EVP_PKEY_CTX_consumer,
        )?;

        Ok(match self.algorithm.sig_format {
            EcdsaSignatureFormat::ASN1 => Signature::new(|slice| {
                slice[..out_sig.len()].copy_from_slice(&out_sig);
                out_sig.len()
            }),
            EcdsaSignatureFormat::Fixed => ec::ecdsa_asn1_to_fixed(self.algorithm.id, &out_sig)?,
        })
    }

    /// Returns a signature for the message corresponding to the provided digest.
    ///
    /// # Errors
    /// `error::Unspecified` on internal error.
    //
    // # FIPS
    // Not allowed.
    #[inline]
    pub fn sign_digest(&self, digest: &Digest) -> Result<Signature, Unspecified> {
        let out_sig = self
            .evp_pkey
            .sign_digest(digest, No_EVP_PKEY_CTX_consumer)?;
        if self.algorithm.digest != digest.algorithm() {
            return Err(Unspecified);
        }

        Ok(match self.algorithm.sig_format {
            EcdsaSignatureFormat::ASN1 => Signature::new(|slice| {
                slice[..out_sig.len()].copy_from_slice(&out_sig);
                out_sig.len()
            }),
            EcdsaSignatureFormat::Fixed => ec::ecdsa_asn1_to_fixed(self.algorithm.id, &out_sig)?,
        })
    }
}

/// Elliptic curve private key.
pub struct PrivateKey<'a>(&'a EcdsaKeyPair);

impl Debug for PrivateKey<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("EcdsaPrivateKey({:?})", self.0.algorithm.id))
    }
}

impl AsBigEndian<EcPrivateKeyBin<'static>> for PrivateKey<'_> {
    /// Exposes the private key encoded as a big-endian fixed-length integer.
    ///
    /// For most use-cases, `EcdsaKeyPair::to_pkcs8()` should be preferred.
    ///
    /// # Errors
    /// `error::Unspecified` if serialization failed.
    fn as_be_bytes(&self) -> Result<EcPrivateKeyBin<'static>, Unspecified> {
        let buffer = marshal_sec1_private_key(&self.0.evp_pkey)?;
        Ok(EcPrivateKeyBin::new(buffer))
    }
}

impl AsDer<EcPrivateKeyRfc5915Der<'static>> for PrivateKey<'_> {
    /// Serializes the key as a DER-encoded `ECPrivateKey` (RFC 5915) structure.
    ///
    /// # Errors
    /// `error::Unspecified`  if serialization failed.
    fn as_der(&self) -> Result<EcPrivateKeyRfc5915Der<'static>, Unspecified> {
        let bytes = marshal_rfc5915_private_key(&self.0.evp_pkey)?;
        Ok(EcPrivateKeyRfc5915Der::new(bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::encoding::AsDer;
    use crate::signature::{
        EcdsaKeyPair, ECDSA_P256K1_SHA256_ASN1_SIGNING, ECDSA_P256_SHA256_FIXED_SIGNING,
        ECDSA_P384_SHA3_384_FIXED_SIGNING, ECDSA_P521_SHA512_FIXED_SIGNING,
    };

    #[test]
    fn test_reject_wrong_curve() {
        let supported_algs = [
            &ECDSA_P256_SHA256_FIXED_SIGNING,
            &ECDSA_P384_SHA3_384_FIXED_SIGNING,
            &ECDSA_P521_SHA512_FIXED_SIGNING,
            &ECDSA_P256K1_SHA256_ASN1_SIGNING,
        ];

        for marshal_alg in supported_algs {
            let key_pair = EcdsaKeyPair::generate(marshal_alg).unwrap();
            let key_pair_doc = key_pair.to_pkcs8v1().unwrap();
            let key_pair_bytes = key_pair_doc.as_ref();

            for parse_alg in supported_algs {
                if parse_alg == marshal_alg {
                    continue;
                }

                let result = EcdsaKeyPair::from_private_key_der(parse_alg, key_pair_bytes);
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn test_from_private_key_der() {
        let key_pair = EcdsaKeyPair::generate(&ECDSA_P256_SHA256_FIXED_SIGNING).unwrap();

        let bytes_5208 = key_pair.to_pkcs8v1().unwrap();
        let bytes_5915 = key_pair.private_key().as_der().unwrap();

        let key_pair_5208 = EcdsaKeyPair::from_private_key_der(
            &ECDSA_P256_SHA256_FIXED_SIGNING,
            bytes_5208.as_ref(),
        )
        .unwrap();
        let key_pair_5915 = EcdsaKeyPair::from_private_key_der(
            &ECDSA_P256_SHA256_FIXED_SIGNING,
            bytes_5915.as_ref(),
        )
        .unwrap();

        assert_eq!(key_pair.evp_pkey, key_pair_5208.evp_pkey);
        assert_eq!(key_pair.evp_pkey, key_pair_5915.evp_pkey);
        assert_eq!(key_pair_5208.evp_pkey, key_pair_5915.evp_pkey);
        assert_eq!(key_pair_5915.algorithm, &ECDSA_P256_SHA256_FIXED_SIGNING);
    }
}
