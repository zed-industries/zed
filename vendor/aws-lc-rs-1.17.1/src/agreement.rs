// Copyright 2015-2017 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Key Agreement: ECDH, including X25519.
//!
//! # Example
//!
//! Note that this example uses X25519, but ECDH using NIST P-256/P-384 is done
//! exactly the same way, just substituting
//! `agreement::ECDH_P256`/`agreement::ECDH_P384` for `agreement::X25519`.
//!
//! ```
//! use aws_lc_rs::{agreement, rand};
//!
//! let rng = rand::SystemRandom::new();
//!
//! let my_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
//!
//! // Make `my_public_key` a byte slice containing my public key. In a real
//! // application, this would be sent to the peer in an encoded protocol
//! // message.
//! let my_public_key = my_private_key.compute_public_key()?;
//!
//! let peer_public_key = {
//!     // In a real application, the peer public key would be parsed out of a
//!     // protocol message. Here we just generate one.
//!     let peer_public_key = {
//!         let peer_private_key =
//!             agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
//!         peer_private_key.compute_public_key()?
//!     };
//!
//!     agreement::UnparsedPublicKey::new(&agreement::X25519, peer_public_key)
//! };
//!
//! agreement::agree_ephemeral(
//!     my_private_key,
//!     &peer_public_key,
//!     aws_lc_rs::error::Unspecified,
//!     |_key_material| {
//!         // In a real application, we'd apply a KDF to the key material and the
//!         // public keys (as recommended in RFC 7748) and then derive session
//!         // keys from the result. We omit all that here.
//!         Ok(())
//!     },
//! )?;
//!
//! # Ok::<(), aws_lc_rs::error::Unspecified>(())
//! ```
mod ephemeral;

use crate::ec::encoding::sec1::{
    marshal_sec1_private_key, marshal_sec1_public_point, marshal_sec1_public_point_into_buffer,
    parse_sec1_private_bn, parse_sec1_public_point,
};
#[cfg(not(feature = "fips"))]
use crate::ec::verify_evp_key_nid;
use crate::ec::{evp_key_generate, validate_ec_evp_key};
use crate::error::{KeyRejected, Unspecified};
use crate::hex;
pub use ephemeral::{agree_ephemeral, EphemeralPrivateKey};

use crate::aws_lc::{
    i2d_ECPrivateKey, EVP_PKEY_get0_EC_KEY, NID_X9_62_prime256v1, NID_secp384r1, NID_secp521r1,
    EVP_PKEY, EVP_PKEY_EC, EVP_PKEY_X25519, NID_X25519,
};

use crate::buffer::Buffer;
use crate::ec;
use crate::ec::encoding::rfc5915::parse_rfc5915_private_key;
use crate::encoding::{
    AsBigEndian, AsDer, Curve25519SeedBin, EcPrivateKeyBin, EcPrivateKeyRfc5915Der,
    EcPublicKeyCompressedBin, EcPublicKeyUncompressedBin, Pkcs8V1Der, PublicKeyX509Der,
};
use crate::evp_pkey::No_EVP_PKEY_CTX_consumer;
use crate::pkcs8::Version;
use crate::ptr::LcPtr;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::ptr::null_mut;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq)]
enum AlgorithmID {
    ECDH_P256,
    ECDH_P384,
    ECDH_P521,
    X25519,
}

impl AlgorithmID {
    #[inline]
    const fn nid(&self) -> i32 {
        match self {
            AlgorithmID::ECDH_P256 => NID_X9_62_prime256v1,
            AlgorithmID::ECDH_P384 => NID_secp384r1,
            AlgorithmID::ECDH_P521 => NID_secp521r1,
            AlgorithmID::X25519 => NID_X25519,
        }
    }

    // Uncompressed public key length in bytes
    #[inline]
    const fn pub_key_len(&self) -> usize {
        match self {
            AlgorithmID::ECDH_P256 => ec::uncompressed_public_key_size_bytes(256),
            AlgorithmID::ECDH_P384 => ec::uncompressed_public_key_size_bytes(384),
            AlgorithmID::ECDH_P521 => ec::uncompressed_public_key_size_bytes(521),
            AlgorithmID::X25519 => 32,
        }
    }

    #[inline]
    const fn private_key_len(&self) -> usize {
        match self {
            AlgorithmID::ECDH_P256 | AlgorithmID::X25519 => 32,
            AlgorithmID::ECDH_P384 => 48,
            AlgorithmID::ECDH_P521 => 66,
        }
    }
}

impl Debug for AlgorithmID {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let output = match self {
            AlgorithmID::ECDH_P256 => "curve: P256",
            AlgorithmID::ECDH_P384 => "curve: P384",
            AlgorithmID::ECDH_P521 => "curve: P521",
            AlgorithmID::X25519 => "curve: Curve25519",
        };
        f.write_str(output)
    }
}

/// A key agreement algorithm.
#[derive(PartialEq, Eq)]
pub struct Algorithm {
    id: AlgorithmID,
}

impl Debug for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&format!("Algorithm {{ {:?} }}", self.id))
    }
}

/// ECDH using the NSA Suite B P-256 (secp256r1) curve.
pub const ECDH_P256: Algorithm = Algorithm {
    id: AlgorithmID::ECDH_P256,
};

/// ECDH using the NSA Suite B P-384 (secp384r1) curve.
pub const ECDH_P384: Algorithm = Algorithm {
    id: AlgorithmID::ECDH_P384,
};

/// ECDH using the NSA Suite B P-521 (secp521r1) curve.
pub const ECDH_P521: Algorithm = Algorithm {
    id: AlgorithmID::ECDH_P521,
};

/// X25519 (ECDH using Curve25519) as described in [RFC 7748].
///
/// Everything is as described in RFC 7748. Key agreement will fail if the
/// result of the X25519 operation is zero; see the notes on the
/// "all-zero value" in [RFC 7748 section 6.1].
///
/// [RFC 7748]: https://tools.ietf.org/html/rfc7748
/// [RFC 7748 section 6.1]: https://tools.ietf.org/html/rfc7748#section-6.1
pub const X25519: Algorithm = Algorithm {
    id: AlgorithmID::X25519,
};

#[allow(non_camel_case_types)]
enum KeyInner {
    ECDH_P256(LcPtr<EVP_PKEY>),
    ECDH_P384(LcPtr<EVP_PKEY>),
    ECDH_P521(LcPtr<EVP_PKEY>),
    X25519(LcPtr<EVP_PKEY>),
}

impl Clone for KeyInner {
    fn clone(&self) -> KeyInner {
        match self {
            KeyInner::ECDH_P256(evp_pkey) => KeyInner::ECDH_P256(evp_pkey.clone()),
            KeyInner::ECDH_P384(evp_pkey) => KeyInner::ECDH_P384(evp_pkey.clone()),
            KeyInner::ECDH_P521(evp_pkey) => KeyInner::ECDH_P521(evp_pkey.clone()),
            KeyInner::X25519(evp_pkey) => KeyInner::X25519(evp_pkey.clone()),
        }
    }
}

/// A private key for use (only) with `agree`. The
/// signature of `agree` allows `PrivateKey` to be
/// used for more than one key agreement.
pub struct PrivateKey {
    inner_key: KeyInner,
}

impl KeyInner {
    #[inline]
    fn algorithm(&self) -> &'static Algorithm {
        match self {
            KeyInner::ECDH_P256(..) => &ECDH_P256,
            KeyInner::ECDH_P384(..) => &ECDH_P384,
            KeyInner::ECDH_P521(..) => &ECDH_P521,
            KeyInner::X25519(..) => &X25519,
        }
    }

    fn get_evp_pkey(&self) -> &LcPtr<EVP_PKEY> {
        match self {
            KeyInner::ECDH_P256(evp_pkey)
            | KeyInner::ECDH_P384(evp_pkey)
            | KeyInner::ECDH_P521(evp_pkey)
            | KeyInner::X25519(evp_pkey) => evp_pkey,
        }
    }
}

// See EVP_PKEY documentation here:
// https://github.com/aws/aws-lc/blob/125af14c57451565b875fbf1282a38a6ecf83782/include/openssl/evp.h#L83-L89
// An |EVP_PKEY| object represents a public or private key. A given object may
// be used concurrently on multiple threads by non-mutating functions, provided
// no other thread is concurrently calling a mutating function. Unless otherwise
// documented, functions which take a |const| pointer are non-mutating and
// functions which take a non-|const| pointer are mutating.
unsafe impl Send for PrivateKey {}
unsafe impl Sync for PrivateKey {}

impl Debug for PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&format!(
            "PrivateKey {{ algorithm: {:?} }}",
            self.inner_key.algorithm()
        ))
    }
}

impl PrivateKey {
    fn new(alg: &'static Algorithm, evp_pkey: LcPtr<EVP_PKEY>) -> Self {
        match alg.id {
            AlgorithmID::X25519 => Self {
                inner_key: KeyInner::X25519(evp_pkey),
            },
            AlgorithmID::ECDH_P256 => Self {
                inner_key: KeyInner::ECDH_P256(evp_pkey),
            },
            AlgorithmID::ECDH_P384 => Self {
                inner_key: KeyInner::ECDH_P384(evp_pkey),
            },
            AlgorithmID::ECDH_P521 => Self {
                inner_key: KeyInner::ECDH_P521(evp_pkey),
            },
        }
    }

    #[inline]
    /// Generate a new private key for the given algorithm.
    // # FIPS
    // Use this function with one of the following algorithms:
    // * `ECDH_P256`
    // * `ECDH_P384`
    // * `ECDH_P521`
    //
    /// # Errors
    /// `error::Unspecified` when operation fails due to internal error.
    pub fn generate(alg: &'static Algorithm) -> Result<Self, Unspecified> {
        let evp_pkey = match alg.id {
            AlgorithmID::X25519 => generate_x25519()?,
            _ => evp_key_generate(alg.id.nid())?,
        };
        Ok(Self::new(alg, evp_pkey))
    }

    /// Deserializes a DER-encoded private key structure to produce a `agreement::PrivateKey`.
    ///
    /// This function is typically used to deserialize RFC 5915 encoded private keys, but it will
    /// attempt to automatically detect other key formats. This function supports unencrypted
    /// PKCS#8 `PrivateKeyInfo` structures as well as key type specific formats.
    ///
    /// X25519 keys are not supported. See `PrivateKey::as_der`.
    ///
    /// # Errors
    /// `error::KeyRejected` if parsing failed or key otherwise unacceptable.
    ///
    /// # Panics
    pub fn from_private_key_der(
        alg: &'static Algorithm,
        key_bytes: &[u8],
    ) -> Result<Self, KeyRejected> {
        if AlgorithmID::X25519 == alg.id {
            return Err(KeyRejected::invalid_encoding());
        }
        let evp_pkey = LcPtr::<EVP_PKEY>::parse_rfc5208_private_key(key_bytes, EVP_PKEY_EC)
            .or(parse_rfc5915_private_key(key_bytes, alg.id.nid()))?;
        #[cfg(not(feature = "fips"))]
        verify_evp_key_nid(&evp_pkey.as_const(), alg.id.nid())?;
        #[cfg(feature = "fips")]
        validate_ec_evp_key(&evp_pkey.as_const(), alg.id.nid())?;

        Ok(Self::new(alg, evp_pkey))
    }

    /// Constructs an ECDH key from private key bytes
    ///
    /// The private key must encoded as a big-endian fixed-length integer. For
    /// example, a P-256 private key must be 32 bytes prefixed with leading
    /// zeros as needed.
    ///
    /// # Errors
    /// `error::KeyRejected` if parsing failed or key otherwise unacceptable.
    pub fn from_private_key(
        alg: &'static Algorithm,
        key_bytes: &[u8],
    ) -> Result<Self, KeyRejected> {
        if key_bytes.len() != alg.id.private_key_len() {
            return Err(KeyRejected::wrong_algorithm());
        }
        let evp_pkey = if AlgorithmID::X25519 == alg.id {
            LcPtr::<EVP_PKEY>::parse_raw_private_key(key_bytes, EVP_PKEY_X25519)?
        } else {
            parse_sec1_private_bn(key_bytes, alg.id.nid())?
        };
        Ok(Self::new(alg, evp_pkey))
    }

    #[cfg(any(test, dev_tests_only))]
    #[allow(missing_docs, clippy::missing_errors_doc)]
    pub fn generate_for_test(
        alg: &'static Algorithm,
        rng: &mut dyn crate::rand::SecureRandom,
    ) -> Result<Self, Unspecified> {
        match alg.id {
            AlgorithmID::X25519 => {
                let mut priv_key = [0u8; AlgorithmID::X25519.private_key_len()];
                rng.mut_fill(&mut priv_key)?;
                Self::from_x25519_private_key(&priv_key)
            }
            AlgorithmID::ECDH_P256 => {
                let mut priv_key = [0u8; AlgorithmID::ECDH_P256.private_key_len()];
                rng.mut_fill(&mut priv_key)?;
                Self::from_p256_private_key(&priv_key)
            }
            AlgorithmID::ECDH_P384 => {
                let mut priv_key = [0u8; AlgorithmID::ECDH_P384.private_key_len()];
                rng.mut_fill(&mut priv_key)?;
                Self::from_p384_private_key(&priv_key)
            }
            AlgorithmID::ECDH_P521 => {
                let mut priv_key = [0u8; AlgorithmID::ECDH_P521.private_key_len()];
                rng.mut_fill(&mut priv_key)?;
                Self::from_p521_private_key(&priv_key)
            }
        }
    }

    #[cfg(any(test, dev_tests_only))]
    fn from_x25519_private_key(
        priv_key: &[u8; AlgorithmID::X25519.private_key_len()],
    ) -> Result<Self, Unspecified> {
        let pkey = LcPtr::<EVP_PKEY>::parse_raw_private_key(priv_key, EVP_PKEY_X25519)?;

        Ok(PrivateKey {
            inner_key: KeyInner::X25519(pkey),
        })
    }

    #[cfg(any(test, dev_tests_only))]
    fn from_p256_private_key(priv_key: &[u8]) -> Result<Self, Unspecified> {
        let pkey = parse_sec1_private_bn(priv_key, ECDH_P256.id.nid())?;
        Ok(PrivateKey {
            inner_key: KeyInner::ECDH_P256(pkey),
        })
    }

    #[cfg(any(test, dev_tests_only))]
    fn from_p384_private_key(priv_key: &[u8]) -> Result<Self, Unspecified> {
        let pkey = parse_sec1_private_bn(priv_key, ECDH_P384.id.nid())?;
        Ok(PrivateKey {
            inner_key: KeyInner::ECDH_P384(pkey),
        })
    }

    #[cfg(any(test, dev_tests_only))]
    fn from_p521_private_key(priv_key: &[u8]) -> Result<Self, Unspecified> {
        let pkey = parse_sec1_private_bn(priv_key, ECDH_P521.id.nid())?;
        Ok(PrivateKey {
            inner_key: KeyInner::ECDH_P521(pkey),
        })
    }

    /// Computes the public key from the private key.
    ///
    /// # Errors
    /// `error::Unspecified` when operation fails due to internal error.
    pub fn compute_public_key(&self) -> Result<PublicKey, Unspecified> {
        match &self.inner_key {
            KeyInner::ECDH_P256(evp_pkey)
            | KeyInner::ECDH_P384(evp_pkey)
            | KeyInner::ECDH_P521(evp_pkey) => {
                let mut public_key = [0u8; MAX_PUBLIC_KEY_LEN];
                let len = marshal_sec1_public_point_into_buffer(&mut public_key, evp_pkey, false)?;
                Ok(PublicKey {
                    inner_key: self.inner_key.clone(),
                    key_bytes: public_key,
                    len,
                })
            }
            KeyInner::X25519(priv_key) => {
                let mut buffer = [0u8; MAX_PUBLIC_KEY_LEN];
                let out_len = priv_key
                    .as_const()
                    .marshal_raw_public_to_buffer(&mut buffer)?;
                Ok(PublicKey {
                    inner_key: self.inner_key.clone(),
                    key_bytes: buffer,
                    len: out_len,
                })
            }
        }
    }

    /// The algorithm for the private key.
    #[inline]
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.inner_key.algorithm()
    }
}

impl AsDer<EcPrivateKeyRfc5915Der<'static>> for PrivateKey {
    /// Serializes the key as a DER-encoded `ECPrivateKey` (RFC 5915) structure.
    ///
    /// X25519 is not supported.
    ///
    /// # Errors
    /// `error::Unspecified`  if serialization failed.
    fn as_der(&self) -> Result<EcPrivateKeyRfc5915Der<'static>, Unspecified> {
        if AlgorithmID::X25519 == self.inner_key.algorithm().id {
            return Err(Unspecified);
        }

        let mut outp = null_mut::<u8>();
        let ec_key = {
            self.inner_key
                .get_evp_pkey()
                .project_const_lifetime(unsafe {
                    |evp_pkey| EVP_PKEY_get0_EC_KEY(evp_pkey.as_const_ptr())
                })?
        };
        let length = usize::try_from(unsafe { i2d_ECPrivateKey(ec_key.as_const_ptr(), &mut outp) })
            .map_err(|_| Unspecified)?;
        let mut outp = LcPtr::new(outp)?;
        Ok(EcPrivateKeyRfc5915Der::take_from_slice(unsafe {
            core::slice::from_raw_parts_mut(outp.as_mut_ptr(), length)
        }))
    }
}

impl AsDer<Pkcs8V1Der<'static>> for PrivateKey {
    /// Serializes the key as a PKCS #8 private key structure.
    ///
    /// X25519 is not supported.
    ///
    /// # Errors
    /// `error::Unspecified`  if serialization failed.
    fn as_der(&self) -> Result<Pkcs8V1Der<'static>, Unspecified> {
        if AlgorithmID::X25519 == self.inner_key.algorithm().id {
            return Err(Unspecified);
        }

        Ok(Pkcs8V1Der::new(
            self.inner_key
                .get_evp_pkey()
                .as_const()
                .marshal_rfc5208_private_key(Version::V1)?,
        ))
    }
}

impl AsBigEndian<EcPrivateKeyBin<'static>> for PrivateKey {
    /// Exposes the private key encoded as a big-endian fixed-length integer.
    ///
    /// X25519 is not supported.
    ///
    /// # Errors
    /// `error::Unspecified` if serialization failed.
    fn as_be_bytes(&self) -> Result<EcPrivateKeyBin<'static>, Unspecified> {
        if AlgorithmID::X25519 == self.inner_key.algorithm().id {
            return Err(Unspecified);
        }
        let buffer = marshal_sec1_private_key(self.inner_key.get_evp_pkey())?;
        Ok(EcPrivateKeyBin::new(buffer))
    }
}

impl AsBigEndian<Curve25519SeedBin<'static>> for PrivateKey {
    /// Exposes the seed encoded as a big-endian fixed-length integer.
    ///
    /// Only X25519 is supported.
    ///
    /// # Errors
    /// `error::Unspecified` if serialization failed.
    fn as_be_bytes(&self) -> Result<Curve25519SeedBin<'static>, Unspecified> {
        if AlgorithmID::X25519 != self.inner_key.algorithm().id {
            return Err(Unspecified);
        }
        let evp_pkey = self.inner_key.get_evp_pkey();
        Ok(Curve25519SeedBin::new(
            evp_pkey.as_const().marshal_raw_private_key()?,
        ))
    }
}

pub(crate) fn generate_x25519() -> Result<LcPtr<EVP_PKEY>, Unspecified> {
    LcPtr::<EVP_PKEY>::generate(EVP_PKEY_X25519, No_EVP_PKEY_CTX_consumer)
}

const MAX_PUBLIC_KEY_LEN: usize = ec::PUBLIC_KEY_MAX_LEN;

/// A public key for key agreement.
pub struct PublicKey {
    inner_key: KeyInner,
    key_bytes: [u8; MAX_PUBLIC_KEY_LEN],
    len: usize,
}

impl PublicKey {
    /// The algorithm for the public key.
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.inner_key.algorithm()
    }
}

// See EVP_PKEY documentation here:
// https://github.com/aws/aws-lc/blob/125af14c57451565b875fbf1282a38a6ecf83782/include/openssl/evp.h#L83-L89
// An |EVP_PKEY| object represents a public or private key. A given object may
// be used concurrently on multiple threads by non-mutating functions, provided
// no other thread is concurrently calling a mutating function. Unless otherwise
// documented, functions which take a |const| pointer are non-mutating and
// functions which take a non-|const| pointer are mutating.
unsafe impl Send for PublicKey {}
unsafe impl Sync for PublicKey {}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "PublicKey {{ algorithm: {:?}, bytes: \"{}\" }}",
            self.inner_key.algorithm(),
            hex::encode(&self.key_bytes[0..self.len])
        ))
    }
}

impl AsRef<[u8]> for PublicKey {
    /// Serializes the public key in an uncompressed form (X9.62) using the
    /// Octet-String-to-Elliptic-Curve-Point algorithm in
    /// [SEC 1: Elliptic Curve Cryptography, Version 2.0].
    fn as_ref(&self) -> &[u8] {
        &self.key_bytes[0..self.len]
    }
}

impl Clone for PublicKey {
    fn clone(&self) -> Self {
        PublicKey {
            inner_key: self.inner_key.clone(),
            key_bytes: self.key_bytes,
            len: self.len,
        }
    }
}

impl AsDer<PublicKeyX509Der<'static>> for PublicKey {
    /// Provides the public key as a DER-encoded (X.509) `SubjectPublicKeyInfo` structure.
    /// # Errors
    /// Returns an error if the public key fails to marshal to X.509.
    fn as_der(&self) -> Result<PublicKeyX509Der<'static>, crate::error::Unspecified> {
        match &self.inner_key {
            KeyInner::ECDH_P256(evp_pkey)
            | KeyInner::ECDH_P384(evp_pkey)
            | KeyInner::ECDH_P521(evp_pkey)
            | KeyInner::X25519(evp_pkey) => {
                let der = evp_pkey.as_const().marshal_rfc5280_public_key()?;
                Ok(PublicKeyX509Der::from(Buffer::new(der)))
            }
        }
    }
}

impl AsBigEndian<EcPublicKeyCompressedBin<'static>> for PublicKey {
    /// Provides the public key elliptic curve point to a compressed point format.
    /// # Errors
    /// Returns an error if the underlying implementation is unable to marshal the public key to this format.
    fn as_be_bytes(&self) -> Result<EcPublicKeyCompressedBin<'static>, crate::error::Unspecified> {
        let evp_pkey = match &self.inner_key {
            KeyInner::ECDH_P256(evp_pkey)
            | KeyInner::ECDH_P384(evp_pkey)
            | KeyInner::ECDH_P521(evp_pkey) => evp_pkey,
            KeyInner::X25519(_) => return Err(Unspecified),
        };
        let pub_point = marshal_sec1_public_point(evp_pkey, true)?;
        Ok(EcPublicKeyCompressedBin::new(pub_point))
    }
}

impl AsBigEndian<EcPublicKeyUncompressedBin<'static>> for PublicKey {
    /// Provides the public key elliptic curve point to a compressed point format.
    ///
    /// Equivalent to [`PublicKey::as_ref`] for ECDH key types, except that it provides you a copy instead of a reference.
    ///
    /// # Errors
    /// Returns an error if the underlying implementation is unable to marshal the public key to this format.
    fn as_be_bytes(
        &self,
    ) -> Result<EcPublicKeyUncompressedBin<'static>, crate::error::Unspecified> {
        if self.algorithm().id == AlgorithmID::X25519 {
            return Err(Unspecified);
        }

        let mut buffer = vec![0u8; self.len];
        buffer.copy_from_slice(&self.key_bytes[0..self.len]);

        Ok(EcPublicKeyUncompressedBin::new(buffer))
    }
}

/// An unparsed, possibly malformed, public key for key agreement.
#[derive(Clone)]
pub struct UnparsedPublicKey<B: AsRef<[u8]>> {
    alg: &'static Algorithm,
    bytes: B,
}

impl<B: Copy + AsRef<[u8]>> Copy for UnparsedPublicKey<B> {}

impl<B: Debug + AsRef<[u8]>> Debug for UnparsedPublicKey<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&format!(
            "UnparsedPublicKey {{ algorithm: {:?}, bytes: {:?} }}",
            self.alg,
            hex::encode(self.bytes.as_ref())
        ))
    }
}

impl<B: AsRef<[u8]>> UnparsedPublicKey<B> {
    /// Constructs a new `UnparsedPublicKey`.
    pub fn new(algorithm: &'static Algorithm, bytes: B) -> Self {
        UnparsedPublicKey {
            alg: algorithm,
            bytes,
        }
    }

    /// The agreement algorithm associated with this public key
    pub fn algorithm(&self) -> &'static Algorithm {
        self.alg
    }

    /// The bytes provided for this public key
    pub fn bytes(&self) -> &B {
        &self.bytes
    }
}

/// A parsed public key for key agreement.
///
/// This represents a public key that has been successfully parsed and validated
/// from its encoded form. The key can be used with the `agree` function to
/// perform key agreement operations.
#[derive(Debug, Clone)]
pub struct ParsedPublicKey {
    format: ParsedPublicKeyFormat,
    nid: i32,
    key: LcPtr<EVP_PKEY>,
    bytes: Box<[u8]>,
}

// See EVP_PKEY documentation here:
// https://github.com/aws/aws-lc/blob/125af14c57451565b875fbf1282a38a6ecf83782/include/openssl/evp.h#L83-L89
// An |EVP_PKEY| object represents a public or private key. A given object may
// be used concurrently on multiple threads by non-mutating functions, provided
// no other thread is concurrently calling a mutating function. Unless otherwise
// documented, functions which take a |const| pointer are non-mutating and
// functions which take a non-|const| pointer are mutating.
unsafe impl Send for ParsedPublicKey {}
unsafe impl Sync for ParsedPublicKey {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// The format of a parsed public key.
///
/// This is used to distinguish between different types of public key formats
/// supported by *aws-lc-rs*.
#[non_exhaustive]
pub enum ParsedPublicKeyFormat {
    /// The key is in an X.509 SubjectPublicKeyInfo format.
    X509,
    /// The key is in an uncompressed form (X9.62).
    Uncompressed,
    /// The key is in a compressed form (SEC 1: Elliptic Curve Cryptography, Version 2.0).
    Compressed,
    /// The key is in a hybrid form (SEC 1: Elliptic Curve Cryptography, Version 2.0).
    Hybrid,
    /// The key is in a raw form. (X25519 only)
    Raw,
    /// The key is in an unknown format.
    Unknown,
}

/// A parsed public key for key agreement.
impl ParsedPublicKey {
    fn nid(&self) -> i32 {
        self.nid
    }

    /// The format of the data the public key was parsed from.
    #[must_use]
    pub fn format(&self) -> ParsedPublicKeyFormat {
        self.format
    }

    pub(crate) fn mut_key(&mut self) -> &mut LcPtr<EVP_PKEY> {
        &mut self.key
    }

    /// The algorithm of the public key.
    #[must_use]
    #[allow(non_upper_case_globals)]
    pub fn alg(&self) -> &'static Algorithm {
        match self.nid() {
            NID_X25519 => &X25519,
            NID_X9_62_prime256v1 => &ECDH_P256,
            NID_secp384r1 => &ECDH_P384,
            NID_secp521r1 => &ECDH_P521,
            _ => unreachable!("Unreachable agreement algorithm nid: {}", self.nid()),
        }
    }
}

impl ParsedPublicKey {
    #[allow(non_upper_case_globals)]
    pub(crate) fn new(bytes: impl AsRef<[u8]>, nid: i32) -> Result<Self, KeyRejected> {
        let bytes = bytes.as_ref().to_vec().into_boxed_slice();
        if bytes.is_empty() {
            return Err(KeyRejected::unspecified());
        }
        match nid {
            NID_X25519 => {
                let format: ParsedPublicKeyFormat;
                let key = if let Ok(evp_pkey) =
                    LcPtr::<EVP_PKEY>::parse_rfc5280_public_key(&bytes, EVP_PKEY_X25519)
                {
                    format = ParsedPublicKeyFormat::X509;
                    evp_pkey
                } else {
                    format = ParsedPublicKeyFormat::Raw;
                    try_parse_x25519_public_key_raw_bytes(&bytes)?
                };

                Ok(ParsedPublicKey {
                    format,
                    nid,
                    key,
                    bytes,
                })
            }
            NID_X9_62_prime256v1 | NID_secp384r1 | NID_secp521r1 => {
                let format: ParsedPublicKeyFormat;
                let key = if let Ok(evp_pkey) =
                    LcPtr::<EVP_PKEY>::parse_rfc5280_public_key(&bytes, EVP_PKEY_EC)
                {
                    validate_ec_evp_key(&evp_pkey.as_const(), nid)?;
                    format = ParsedPublicKeyFormat::X509;
                    evp_pkey
                } else if let Ok(evp_pkey) = parse_sec1_public_point(&bytes, nid) {
                    format = match bytes[0] {
                        0x02 | 0x03 => ParsedPublicKeyFormat::Compressed,
                        0x04 => ParsedPublicKeyFormat::Uncompressed,
                        0x06 | 0x07 => ParsedPublicKeyFormat::Hybrid,
                        _ => ParsedPublicKeyFormat::Unknown,
                    };
                    evp_pkey
                } else {
                    return Err(KeyRejected::invalid_encoding());
                };

                Ok(ParsedPublicKey {
                    format,
                    nid,
                    key,
                    bytes,
                })
            }
            _ => Err(KeyRejected::unspecified()),
        }
    }
}

impl AsRef<[u8]> for ParsedPublicKey {
    /// Returns the original bytes from which this key was parsed.
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<B: AsRef<[u8]>> UnparsedPublicKey<B> {
    #[allow(dead_code)]
    fn parse(&self) -> Result<ParsedPublicKey, KeyRejected> {
        ParsedPublicKey::new(&self.bytes, self.alg.id.nid())
    }
}

impl<B: AsRef<[u8]>> TryFrom<&UnparsedPublicKey<B>> for ParsedPublicKey {
    type Error = KeyRejected;
    fn try_from(upk: &UnparsedPublicKey<B>) -> Result<Self, Self::Error> {
        upk.parse()
    }
}

impl<B: AsRef<[u8]>> TryFrom<UnparsedPublicKey<B>> for ParsedPublicKey {
    type Error = KeyRejected;
    fn try_from(upk: UnparsedPublicKey<B>) -> Result<Self, Self::Error> {
        upk.parse()
    }
}

/// Performs a key agreement with a private key and the given public key.
///
/// `my_private_key` is the private key to use. Only a reference to the key
/// is required, allowing the key to continue to be used.
///
/// `peer_public_key` is the peer's public key. `agree` will return
/// `Err(error_value)` if it does not match `my_private_key's` algorithm/curve.
/// `agree` verifies that it is encoded in the standard form for the
/// algorithm and that the key is *valid*; see the algorithm's documentation for
/// details on how keys are to be encoded and what constitutes a valid key for
/// that algorithm.
///
/// `error_value` is the value to return if an error occurs before `kdf` is
/// called, e.g. when decoding of the peer's public key fails or when the public
/// key is otherwise invalid.
///
/// After the key agreement is done, `agree` calls `kdf` with the raw
/// key material from the key agreement operation and then returns what `kdf`
/// returns.
// # FIPS
// Use this function with one of the following key algorithms:
// * `ECDH_P256`
// * `ECDH_P384`
// * `ECDH_P521`
//
/// # Errors
/// `error_value` on internal failure.
#[inline]
#[allow(clippy::missing_panics_doc)]
pub fn agree<B: TryInto<ParsedPublicKey>, F, R, E>(
    my_private_key: &PrivateKey,
    peer_public_key: B,
    error_value: E,
    kdf: F,
) -> Result<R, E>
where
    F: FnOnce(&[u8]) -> Result<R, E>,
{
    let expected_alg = my_private_key.algorithm();

    let parse_result = peer_public_key.try_into();

    if let Ok(mut peer_pub_key) = parse_result {
        if peer_pub_key.alg() != expected_alg {
            return Err(error_value);
        }
        let secret = my_private_key
            .inner_key
            .get_evp_pkey()
            .agree(peer_pub_key.mut_key())
            .or(Err(error_value))?;

        kdf(secret.as_ref())
    } else {
        Err(error_value)
    }
}

fn try_parse_x25519_public_key_raw_bytes(key_bytes: &[u8]) -> Result<LcPtr<EVP_PKEY>, KeyRejected> {
    let expected_pub_key_len = X25519.id.pub_key_len();
    if key_bytes.len() != expected_pub_key_len {
        return Err(KeyRejected::invalid_encoding());
    }

    LcPtr::<EVP_PKEY>::parse_raw_public_key(key_bytes, EVP_PKEY_X25519)
}

#[cfg(test)]
mod agreement_tests;
#[cfg(test)]
mod parsed_public_key_tests;
