// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

pub(super) mod oaep;
pub(super) mod pkcs1;

use super::key::{generate_rsa_key, validate_rsa_key};
use super::{encoding, KeySize};
use crate::aws_lc::{EVP_PKEY, EVP_PKEY_RSA};
use crate::encoding::{AsDer, Pkcs8V1Der, PublicKeyX509Der};
use crate::error::{KeyRejected, Unspecified};
use crate::pkcs8::Version;
use crate::ptr::LcPtr;
use core::fmt::Debug;

/// RSA Encryption Algorithm Identifier
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum EncryptionAlgorithmId {
    /// RSA-OAEP with SHA1 Hash and SHA1 MGF1
    OaepSha1Mgf1sha1,

    /// RSA-OAEP with SHA256 Hash and SHA256 MGF1
    OaepSha256Mgf1sha256,

    /// RSA-OAEP with SHA384 Hash and SHA384 MGF1
    OaepSha384Mgf1sha384,

    /// RSA-OAEP with SHA512 Hash and SHA512 MGF1
    OaepSha512Mgf1sha512,
}

/// An RSA private key used for decrypting ciphertext encrypted by a [`PublicEncryptingKey`].
pub struct PrivateDecryptingKey(LcPtr<EVP_PKEY>);

// https://github.com/aws/aws-lc/blob/ebaa07a207fee02bd68fe8d65f6b624afbf29394/include/openssl/evp.h#L295
// An |EVP_PKEY| object represents a public or private RSA key. A given object may be
// used concurrently on multiple threads by non-mutating functions, provided no
// other thread is concurrently calling a mutating function. Unless otherwise
// documented, functions which take a |const| pointer are non-mutating and
// functions which take a non-|const| pointer are mutating.
unsafe impl Send for PrivateDecryptingKey {}
unsafe impl Sync for PrivateDecryptingKey {}

impl PrivateDecryptingKey {
    fn new(evp_pkey: LcPtr<EVP_PKEY>) -> Result<Self, KeyRejected> {
        Self::validate_key(&evp_pkey)?;
        Ok(Self(evp_pkey))
    }

    fn validate_key(key: &LcPtr<EVP_PKEY>) -> Result<(), KeyRejected> {
        validate_rsa_key(key)
    }

    /// Generate a new RSA private key pair for use with asymmetrical encryption.
    ///
    /// Supports the following key sizes:
    /// * `KeySize::Rsa2048`
    /// * `KeySize::Rsa3072`
    /// * `KeySize::Rsa4096`
    /// * `KeySize::Rsa8192`
    ///
    /// # Errors
    /// * `Unspecified` for any error that occurs during the generation of the RSA keypair.
    pub fn generate(size: KeySize) -> Result<Self, Unspecified> {
        let key = generate_rsa_key(size.bits())?;
        Ok(Self::new(key)?)
    }

    /// Generate a new RSA private key pair for use with asymmetrical encryption.
    ///
    /// Supports the following key sizes:
    /// * `KeySize::Rsa2048`
    /// * `KeySize::Rsa3072`
    /// * `KeySize::Rsa4096`
    /// * `KeySize::Rsa8192`
    ///
    /// ## Deprecated
    /// This is equivalent to `KeyPair::generate`.
    ///
    /// # Errors
    /// * `Unspecified` for any error that occurs during the generation of the RSA keypair.
    #[cfg(feature = "fips")]
    #[deprecated]
    pub fn generate_fips(size: KeySize) -> Result<Self, Unspecified> {
        Self::generate(size)
    }

    /// Construct a `PrivateDecryptingKey` from the provided PKCS#8 (v1) document.
    ///
    /// Supports RSA key sizes between 2048 and 8192 (inclusive).
    ///
    /// # Errors
    /// * `KeyRejected` if bytes do not encode an RSA private key or if the key is otherwise not
    ///   acceptable.
    pub fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, KeyRejected> {
        let key = LcPtr::<EVP_PKEY>::parse_rfc5208_private_key(pkcs8, EVP_PKEY_RSA)?;
        Self::new(key)
    }

    /// Returns a boolean indicator if this RSA key is an approved FIPS 140-3 key.
    #[cfg(feature = "fips")]
    #[must_use]
    pub fn is_valid_fips_key(&self) -> bool {
        super::key::is_valid_fips_key(&self.0)
    }

    /// Returns the RSA signature size in bytes.
    #[must_use]
    pub fn key_size_bytes(&self) -> usize {
        self.0.as_const().signature_size_bytes()
    }

    /// Returns the RSA key size in bits.
    #[must_use]
    pub fn key_size_bits(&self) -> usize {
        self.0.as_const().key_size_bits()
    }

    /// Retrieves the `PublicEncryptingKey` corresponding with this `PrivateDecryptingKey`.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn public_key(&self) -> PublicEncryptingKey {
        PublicEncryptingKey::new(self.0.clone()).expect(
            "PublicEncryptingKey key size to be supported by PrivateDecryptingKey key sizes",
        )
    }
}

impl Debug for PrivateDecryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrivateDecryptingKey").finish()
    }
}

impl AsDer<Pkcs8V1Der<'static>> for PrivateDecryptingKey {
    fn as_der(&self) -> Result<Pkcs8V1Der<'static>, Unspecified> {
        Ok(Pkcs8V1Der::new(
            self.0.as_const().marshal_rfc5208_private_key(Version::V1)?,
        ))
    }
}

impl Clone for PrivateDecryptingKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// An RSA public key used for encrypting plaintext that is decrypted by a [`PrivateDecryptingKey`].
pub struct PublicEncryptingKey(LcPtr<EVP_PKEY>);

// See thread-safety note on `PrivateDecryptingKey`.
unsafe impl Send for PublicEncryptingKey {}
unsafe impl Sync for PublicEncryptingKey {}

impl PublicEncryptingKey {
    pub(crate) fn new(evp_pkey: LcPtr<EVP_PKEY>) -> Result<Self, KeyRejected> {
        Self::validate_key(&evp_pkey)?;
        Ok(Self(evp_pkey))
    }

    fn validate_key(key: &LcPtr<EVP_PKEY>) -> Result<(), KeyRejected> {
        validate_rsa_key(key)
    }

    /// Construct a `PublicEncryptingKey` from X.509 `SubjectPublicKeyInfo` DER encoded bytes.
    ///
    /// # Errors
    /// * `KeyRejected` if bytes do not encode a supported RSA public key or if the key is
    ///   otherwise not acceptable.
    pub fn from_der(value: &[u8]) -> Result<Self, KeyRejected> {
        let key = encoding::rfc5280::decode_public_key_der(value)?;
        Self::new(key)
    }

    /// Returns the RSA signature size in bytes.
    #[must_use]
    pub fn key_size_bytes(&self) -> usize {
        self.0.as_const().signature_size_bytes()
    }

    /// Returns the RSA key size in bits.
    #[must_use]
    pub fn key_size_bits(&self) -> usize {
        self.0.as_const().key_size_bits()
    }
}

impl Debug for PublicEncryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PublicEncryptingKey").finish()
    }
}

impl Clone for PublicEncryptingKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl AsDer<PublicKeyX509Der<'static>> for PublicEncryptingKey {
    /// Serialize this `PublicEncryptingKey` to a X.509 `SubjectPublicKeyInfo` structure as DER encoded bytes.
    ///
    /// # Errors
    /// * `Unspecified` for any error that occurs serializing to bytes.
    fn as_der(&self) -> Result<PublicKeyX509Der<'static>, Unspecified> {
        encoding::rfc5280::encode_public_key_der(&self.0)
    }
}
