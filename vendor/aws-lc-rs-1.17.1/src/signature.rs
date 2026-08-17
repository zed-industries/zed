// Copyright 2015-2017 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Public key signatures: signing and verification.
//!
//! Use the `verify` function to verify signatures, passing a reference to the
//! algorithm that identifies the algorithm. See the documentation for `verify`
//! for examples.
//!
//! For signature verification, this API treats each combination of parameters
//! as a separate algorithm. For example, instead of having a single "RSA"
//! algorithm with a verification function that takes a bunch of parameters,
//! there are `RSA_PKCS1_2048_8192_SHA256`, `RSA_PKCS1_2048_8192_SHA384`, etc.,
//! which encode sets of parameter choices into objects. This is designed to
//! reduce the risks of algorithm agility and to provide consistency with ECDSA
//! and `EdDSA`.
//!
//! Currently this module does not support digesting the message to be signed
//! separately from the public key operation, as it is currently being
//! optimized for Ed25519 and for the implementation of protocols that do not
//! requiring signing large messages. An interface for efficiently supporting
//! larger messages may be added later.
//!
//!
//! # Algorithm Details
//!
//! ## `ECDSA_*_ASN1` Details: ASN.1-encoded ECDSA Signatures
//!
//! The signature is a ASN.1 DER-encoded `Ecdsa-Sig-Value` as described in
//! [RFC 3279 Section 2.2.3]. This is the form of ECDSA signature used in
//! X.509-related structures and in TLS's `ServerKeyExchange` messages.
//!
//! The public key is encoding in uncompressed form using the
//! Octet-String-to-Elliptic-Curve-Point algorithm in
//! [SEC 1: Elliptic Curve Cryptography, Version 2.0].
//!
//! During verification, the public key is validated using the ECC Partial
//! Public-Key Validation Routine from Section 5.6.2.3.3 of
//! [NIST Special Publication 800-56A, revision 2] and Appendix A.3 of the
//! NSA's [Suite B implementer's guide to FIPS 186-3]. Note that, as explained
//! in the NSA guide, ECC Partial Public-Key Validation is equivalent to ECC
//! Full Public-Key Validation for prime-order curves like this one.
//!
//! ## `ECDSA_*_FIXED` Details: Fixed-length (PKCS#11-style) ECDSA Signatures
//!
//! The signature is *r*||*s*, where || denotes concatenation, and where both
//! *r* and *s* are both big-endian-encoded values that are left-padded to the
//! maximum length. A P-256 signature will be 64 bytes long (two 32-byte
//! components) and a P-384 signature will be 96 bytes long (two 48-byte
//! components). This is the form of ECDSA signature used PKCS#11 and DNSSEC.
//!
//! The public key is encoding in uncompressed form using the
//! Octet-String-to-Elliptic-Curve-Point algorithm in
//! [SEC 1: Elliptic Curve Cryptography, Version 2.0].
//!
//! During verification, the public key is validated using the ECC Partial
//! Public-Key Validation Routine from Section 5.6.2.3.3 of
//! [NIST Special Publication 800-56A, revision 2] and Appendix A.3 of the
//! NSA's [Suite B implementer's guide to FIPS 186-3]. Note that, as explained
//! in the NSA guide, ECC Partial Public-Key Validation is equivalent to ECC
//! Full Public-Key Validation for prime-order curves like this one.
//!
//! ## `RSA_PKCS1_*` Details: RSA PKCS#1 1.5 Signatures
//!
//! The signature is an RSASSA-PKCS1-v1_5 signature as described in
//! [RFC 3447 Section 8.2].
//!
//! The public key is encoded as an ASN.1 `RSAPublicKey` as described in
//! [RFC 3447 Appendix-A.1.1]. The public key modulus length, rounded *up* to
//! the nearest (larger) multiple of 8 bits, must be in the range given in the
//! name of the algorithm. The public exponent must be an odd integer of 2-33
//! bits, inclusive.
//!
//!
//! ## `RSA_PSS_*` Details: RSA PSS Signatures
//!
//! The signature is an RSASSA-PSS signature as described in
//! [RFC 3447 Section 8.1].
//!
//! The public key is encoded as an ASN.1 `RSAPublicKey` as described in
//! [RFC 3447 Appendix-A.1.1]. The public key modulus length, rounded *up* to
//! the nearest (larger) multiple of 8 bits, must be in the range given in the
//! name of the algorithm. The public exponent must be an odd integer of 2-33
//! bits, inclusive.
//!
//! During verification, signatures will only be accepted if the MGF1 digest
//! algorithm is the same as the message digest algorithm and if the salt
//! length is the same length as the message digest. This matches the
//! requirements in TLS 1.3 and other recent specifications.
//!
//! During signing, the message digest algorithm will be used as the MGF1
//! digest algorithm. The salt will be the same length as the message digest.
//! This matches the requirements in TLS 1.3 and other recent specifications.
//! Additionally, the entire salt is randomly generated separately for each
//! signature using the secure random number generator passed to `sign()`.
//!
//!
//! [SEC 1: Elliptic Curve Cryptography, Version 2.0]:
//!     http://www.secg.org/sec1-v2.pdf
//! [NIST Special Publication 800-56A, revision 2]:
//!     http://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Ar2.pdf
//! [Suite B implementer's guide to FIPS 186-3]:
//!     https://github.com/briansmith/ring/blob/main/doc/ecdsa.pdf
//! [RFC 3279 Section 2.2.3]:
//!     https://tools.ietf.org/html/rfc3279#section-2.2.3
//! [RFC 3447 Section 8.2]:
//!     https://tools.ietf.org/html/rfc3447#section-7.2
//! [RFC 3447 Section 8.1]:
//!     https://tools.ietf.org/html/rfc3447#section-8.1
//! [RFC 3447 Appendix-A.1.1]:
//!     https://tools.ietf.org/html/rfc3447#appendix-A.1.1
//!
//!
//! # Examples
//!
//! ## Signing and verifying with Ed25519
//!
//! ```
//! use aws_lc_rs::{
//!     rand,
//!     signature::{self, KeyPair},
//! };
//!
//! fn main() -> Result<(), aws_lc_rs::error::Unspecified> {
//!     // Generate a new key pair for Ed25519.
//!     let key_pair = signature::Ed25519KeyPair::generate()?;
//!
//!     // Sign the message "hello, world".
//!     const MESSAGE: &[u8] = b"hello, world";
//!     let sig = key_pair.sign(MESSAGE);
//!
//!     // Normally an application would extract the bytes of the signature and
//!     // send them in a protocol message to the peer(s). Here we just get the
//!     // public key key directly from the key pair.
//!     let peer_public_key_bytes = key_pair.public_key().as_ref();
//!
//!     // Verify the signature of the message using the public key. Normally the
//!     // verifier of the message would parse the inputs to this code out of the
//!     // protocol message(s) sent by the signer.
//!     let peer_public_key =
//!         signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
//!     peer_public_key.verify(MESSAGE, sig.as_ref())?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Signing and verifying with RSA (PKCS#1 1.5 padding)
//!
//! By default OpenSSL writes RSA public keys in `SubjectPublicKeyInfo` format,
//! not `RSAPublicKey` format, and Base64-encodes them (“PEM” format).
//!
//! To convert the PEM `SubjectPublicKeyInfo` format (“BEGIN PUBLIC KEY”) to the
//! binary `RSAPublicKey` format needed by `verify()`, use:
//!
//! ```sh
//! openssl rsa -pubin \
//!             -in public_key.pem \
//!             -inform PEM \
//!             -RSAPublicKey_out \
//!             -outform DER \
//!             -out public_key.der
//! ```
//!
//! To extract the RSAPublicKey-formatted public key from an ASN.1 (binary)
//! DER-encoded `RSAPrivateKey` format private key file, use:
//!
//! ```sh
//! openssl rsa -in private_key.der \
//!             -inform DER \
//!             -RSAPublicKey_out \
//!             -outform DER \
//!             -out public_key.der
//! ```
//!
//! ```
//! use aws_lc_rs::{rand, signature};
//!
//! fn sign_and_verify_rsa(
//!     private_key_path: &std::path::Path,
//!     public_key_path: &std::path::Path,
//! ) -> Result<(), MyError> {
//!     // Create an `RsaKeyPair` from the DER-encoded bytes. This example uses
//!     // a 2048-bit key, but larger keys are also supported.
//!     let private_key_der = read_file(private_key_path)?;
//!     let key_pair = signature::RsaKeyPair::from_der(&private_key_der)
//!         .map_err(|_| MyError::BadPrivateKey)?;
//!
//!     // Sign the message "hello, world", using PKCS#1 v1.5 padding and the
//!     // SHA256 digest algorithm.
//!     const MESSAGE: &'static [u8] = b"hello, world";
//!     let rng = rand::SystemRandom::new();
//!     let mut signature = vec![0; key_pair.public_modulus_len()];
//!     key_pair
//!         .sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature)
//!         .map_err(|_| MyError::OOM)?;
//!
//!     // Verify the signature.
//!     let public_key = signature::UnparsedPublicKey::new(
//!         &signature::RSA_PKCS1_2048_8192_SHA256,
//!         read_file(public_key_path)?,
//!     );
//!     public_key
//!         .verify(MESSAGE, &signature)
//!         .map_err(|_| MyError::BadSignature)
//! }
//!
//! #[derive(Debug)]
//! enum MyError {
//!     IO(std::io::Error),
//!     BadPrivateKey,
//!     OOM,
//!     BadSignature,
//! }
//!
//! fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
//!     use std::io::Read;
//!
//!     let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
//!     let mut contents: Vec<u8> = Vec::new();
//!     file.read_to_end(&mut contents)
//!         .map_err(|e| MyError::IO(e))?;
//!     Ok(contents)
//! }
//!
//! fn main() {
//! #   if !cfg!(target_arch = "wasm32") {
//!     let private_key_path =
//!         std::path::Path::new("tests/data/signature_rsa_example_private_key.der");
//!     let public_key_path =
//!         std::path::Path::new("tests/data/signature_rsa_example_public_key.der");
//!     sign_and_verify_rsa(&private_key_path, &public_key_path).unwrap()
//! #   }
//! }
//! ```
use crate::aws_lc::EVP_PKEY;
pub use crate::rsa::signature::{RsaEncoding, RsaSignatureEncoding};
pub use crate::rsa::{
    KeyPair as RsaKeyPair, PublicKey as RsaSubjectPublicKey,
    PublicKeyComponents as RsaPublicKeyComponents, RsaParameters,
};
use core::fmt::{Debug, Formatter};
use std::any::{Any, TypeId};
#[cfg(feature = "ring-sig-verify")]
use untrusted::Input;

use crate::rsa::signature::RsaSigningAlgorithmId;
use crate::rsa::RsaVerificationAlgorithmId;

pub use crate::ec::key_pair::{EcdsaKeyPair, PrivateKey as EcdsaPrivateKey};
use crate::ec::signature::EcdsaSignatureFormat;
pub use crate::ec::signature::{
    EcdsaSigningAlgorithm, EcdsaVerificationAlgorithm, PublicKey as EcdsaPublicKey,
};
pub use crate::ed25519::{
    Ed25519KeyPair, EdDSAParameters, PublicKey as Ed25519PublicKey, Seed as Ed25519Seed,
    ED25519_PUBLIC_KEY_LEN,
};

use crate::digest::Digest;
use crate::ec::encoding::parse_ec_public_key;
use crate::ed25519::parse_ed25519_public_key;
use crate::encoding::{AsDer, PublicKeyX509Der};
use crate::error::{KeyRejected, Unspecified};
#[cfg(all(feature = "unstable", not(feature = "fips")))]
use crate::pqdsa::{parse_pqdsa_public_key, signature::PqdsaVerificationAlgorithm};
use crate::ptr::LcPtr;
use crate::rsa::key::parse_rsa_public_key;
use crate::{digest, ec, error, hex, rsa, sealed};

/// The longest signature is for ML-DSA-87
pub(crate) const MAX_LEN: usize = 4627;

/// A public key signature returned from a signing operation.
#[derive(Clone, Copy)]
pub struct Signature {
    value: [u8; MAX_LEN],
    len: usize,
}

impl Signature {
    // Panics if `value` is too long.
    pub(crate) fn new<F>(fill: F) -> Self
    where
        F: FnOnce(&mut [u8; MAX_LEN]) -> usize,
    {
        let mut r = Self {
            value: [0; MAX_LEN],
            len: 0,
        };
        r.len = fill(&mut r.value);
        r
    }
}

impl AsRef<[u8]> for Signature {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.value[..self.len]
    }
}

/// Key pairs for signing messages (private key and public key).
pub trait KeyPair: Debug + Send + Sized + Sync {
    /// The type of the public key.
    type PublicKey: AsRef<[u8]> + Debug + Clone + Send + Sized + Sync;

    /// The public key for the key pair.
    fn public_key(&self) -> &Self::PublicKey;
}

// Private trait
pub(crate) trait ParsedVerificationAlgorithm: Debug + Sync {
    fn parsed_verify_sig(
        &self,
        public_key: &ParsedPublicKey,
        msg: &[u8],
        signature: &[u8],
    ) -> Result<(), error::Unspecified>;

    fn parsed_verify_digest_sig(
        &self,
        public_key: &ParsedPublicKey,
        digest: &Digest,
        signature: &[u8],
    ) -> Result<(), error::Unspecified>;
}

/// A signature verification algorithm.
pub trait VerificationAlgorithm: Debug + Sync + Any + sealed::Sealed {
    /// Verify the signature `signature` of message `msg` with the public key
    /// `public_key`.
    ///
    // # FIPS
    // The following conditions must be met:
    // * RSA Key Sizes: 1024, 2048, 3072, 4096
    // * NIST Elliptic Curves: P256, P384, P521
    // * Digest Algorithms: SHA1, SHA256, SHA384, SHA512
    //
    /// # Errors
    /// `error::Unspecified` if inputs not verified.
    #[cfg(feature = "ring-sig-verify")]
    #[deprecated(note = "please use `VerificationAlgorithm::verify_sig` instead")]
    fn verify(
        &self,
        public_key: Input<'_>,
        msg: Input<'_>,
        signature: Input<'_>,
    ) -> Result<(), error::Unspecified>;

    /// Verify the signature `signature` of message `msg` with the public key
    /// `public_key`.
    ///
    // # FIPS
    // The following conditions must be met:
    // * RSA Key Sizes: 1024, 2048, 3072, 4096
    // * NIST Elliptic Curves: P256, P384, P521
    // * Digest Algorithms: SHA1, SHA256, SHA384, SHA512
    //
    /// # Errors
    /// `error::Unspecified` if inputs not verified.
    fn verify_sig(
        &self,
        public_key: &[u8],
        msg: &[u8],
        signature: &[u8],
    ) -> Result<(), error::Unspecified>;

    /// Verify the signature `signature` of `digest` with the `public_key`.
    ///
    // # FIPS
    // Not approved.
    //
    /// # Errors
    /// `error::Unspecified` if inputs not verified.
    fn verify_digest_sig(
        &self,
        public_key: &[u8],
        digest: &Digest,
        signature: &[u8],
    ) -> Result<(), error::Unspecified>;
}

/// An unparsed, possibly malformed, public key for signature verification.
#[derive(Clone)]
pub struct UnparsedPublicKey<B: AsRef<[u8]>> {
    algorithm: &'static dyn VerificationAlgorithm,
    bytes: B,
}
/// A parsed public key for signature verification.
///
/// A `ParsedPublicKey` can be created in two ways:
/// - Directly from public key bytes using [`ParsedPublicKey::new`]
/// - By parsing an `UnparsedPublicKey` using [`UnparsedPublicKey::parse`]
///
/// This pre-validates the public key format and stores the parsed key material,
/// allowing for more efficient signature verification operations compared to
/// parsing the key on each verification.
///
/// See the [`crate::signature`] module-level documentation for examples.
#[derive(Clone)]
pub struct ParsedPublicKey {
    algorithm: &'static dyn VerificationAlgorithm,
    parsed_algorithm: &'static dyn ParsedVerificationAlgorithm,
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

impl ParsedPublicKey {
    /// Creates a new `ParsedPublicKey` directly from public key bytes.
    ///
    /// This method validates the public key format and creates a `ParsedPublicKey`
    /// that can be used for efficient signature verification operations.
    ///
    /// # Errors
    /// `KeyRejected` if the public key bytes are malformed or incompatible
    /// with the specified algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use aws_lc_rs::signature::{self, ParsedPublicKey};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let parsed_key = ParsedPublicKey::new(&signature::ED25519, include_bytes!("../tests/data/ed25519_test_public_key.bin"))?;
    ///     let signature = [
    ///         0xED, 0xDB, 0x67, 0xE9, 0xF7, 0x8C, 0x9A, 0x0, 0xFD, 0xEE, 0x2D, 0x22, 0x21, 0xA3, 0x9A,
    ///         0x8A, 0x79, 0xF2, 0x53, 0x88, 0x78, 0xF0, 0xA0, 0x1, 0x80, 0xA, 0x49, 0xA4, 0x17, 0x88,
    ///         0xAB, 0x44, 0x4B, 0xD2, 0x58, 0xB0, 0x3B, 0x51, 0x8A, 0x1B, 0x61, 0x24, 0x52, 0x78, 0x48,
    ///         0x58, 0x40, 0x5, 0xB5, 0x45, 0x22, 0xB6, 0x40, 0xBD, 0x14, 0x47, 0xB1, 0xF0, 0xDC, 0x13,
    ///         0xB3, 0xE9, 0xD0, 0x6,
    ///     ];
    ///     assert!(parsed_key.verify_sig(b"hello world!", &signature).is_ok());
    ///     assert!(parsed_key.verify_sig(b"hello world.", &signature).is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        algorithm: &'static dyn VerificationAlgorithm,
        bytes: B,
    ) -> Result<Self, KeyRejected> {
        parse_public_key(bytes.as_ref(), algorithm)
    }

    /// Returns the algorithm used by this public key.
    #[must_use]
    pub fn algorithm(&self) -> &'static dyn VerificationAlgorithm {
        self.algorithm
    }

    pub(crate) fn key(&self) -> &LcPtr<EVP_PKEY> {
        &self.key
    }

    /// Constructs a `ParsedPublicKey` directly from an already-built RSA
    /// `EVP_PKEY`, skipping the DER-encode / DER-decode round-trip that
    /// [`parse_public_key`] would otherwise perform. The SubjectPublicKeyInfo
    /// DER is still marshalled once so that [`AsRef<[u8]>`] on the resulting
    /// `ParsedPublicKey` continues to return a canonical encoding.
    ///
    /// `params` is used as both the public [`VerificationAlgorithm`] and the
    /// internal `ParsedVerificationAlgorithm`, matching what
    /// [`parse_public_key`] would assign for RSA inputs.
    pub(crate) fn from_rsa_evp_pkey(
        params: &'static RsaParameters,
        key: LcPtr<EVP_PKEY>,
    ) -> Result<Self, Unspecified> {
        let bytes = key
            .as_const()
            .marshal_rfc5280_public_key()?
            .into_boxed_slice();
        Ok(ParsedPublicKey {
            algorithm: params,
            parsed_algorithm: params,
            key,
            bytes,
        })
    }

    /// Uses the public key to verify that `signature` is a valid signature of
    /// `message`.
    ///
    /// This method is more efficient than [`UnparsedPublicKey::verify`] when
    /// performing multiple signature verifications with the same public key,
    /// as the key parsing overhead is avoided.
    ///
    /// See the [`crate::signature`] module-level documentation for examples.
    ///
    // # FIPS
    // The following conditions must be met:
    // * RSA Key Sizes: 1024, 2048, 3072, 4096
    // * NIST Elliptic Curves: P256, P384, P521
    // * Digest Algorithms: SHA1, SHA256, SHA384, SHA512
    //
    /// # Errors
    /// `error::Unspecified` if the signature is invalid or verification fails.
    #[inline]
    pub fn verify_sig(&self, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified> {
        self.parsed_algorithm
            .parsed_verify_sig(self, message, signature)
    }

    /// Uses the public key to verify that `signature` is a valid signature of
    /// `digest`.
    ///
    /// This method is more efficient than [`UnparsedPublicKey::verify_digest`] when
    /// performing multiple signature verifications with the same public key,
    /// as the key parsing overhead is avoided.
    ///
    /// See the [`crate::signature`] module-level documentation for examples.
    ///
    // # FIPS
    // Not allowed
    //
    /// # Errors
    /// `error::Unspecified` if the signature is invalid or verification fails.
    #[inline]
    pub fn verify_digest_sig(
        &self,
        digest: &Digest,
        signature: &[u8],
    ) -> Result<(), error::Unspecified> {
        self.parsed_algorithm
            .parsed_verify_digest_sig(self, digest, signature)
    }
}

impl AsDer<PublicKeyX509Der<'static>> for ParsedPublicKey {
    fn as_der(&self) -> Result<PublicKeyX509Der<'static>, Unspecified> {
        Ok(PublicKeyX509Der::new(
            self.key.as_const().marshal_rfc5280_public_key()?,
        ))
    }
}

/// Provides the original bytes from which this key was parsed
impl AsRef<[u8]> for ParsedPublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Debug for ParsedPublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&format!(
            "ParsedPublicKey {{ algorithm: {:?}, bytes: \"{}\" }}",
            self.algorithm,
            hex::encode(self.bytes.as_ref())
        ))
    }
}

impl<B: AsRef<[u8]>> AsRef<[u8]> for UnparsedPublicKey<B> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl<B: Copy + AsRef<[u8]>> Copy for UnparsedPublicKey<B> {}

impl<B: AsRef<[u8]>> Debug for UnparsedPublicKey<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&format!(
            "UnparsedPublicKey {{ algorithm: {:?}, bytes: \"{}\" }}",
            self.algorithm,
            hex::encode(self.bytes.as_ref())
        ))
    }
}

impl<B: AsRef<[u8]>> UnparsedPublicKey<B> {
    /// Construct a new `UnparsedPublicKey`.
    ///
    /// No validation of `bytes` is done until `verify()` is called.
    #[inline]
    pub fn new(algorithm: &'static dyn VerificationAlgorithm, bytes: B) -> Self {
        Self { algorithm, bytes }
    }

    /// Parses the public key and verifies `signature` is a valid signature of
    /// `message` using it.
    ///
    /// See the [`crate::signature`] module-level documentation for examples.
    ///
    // # FIPS
    // The following conditions must be met:
    // * RSA Key Sizes: 1024, 2048, 3072, 4096
    // * NIST Elliptic Curves: P256, P384, P521
    // * Digest Algorithms: SHA1, SHA256, SHA384, SHA512
    //
    /// # Errors
    /// `error::Unspecified` if inputs not verified.
    #[inline]
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified> {
        self.algorithm
            .verify_sig(self.bytes.as_ref(), message, signature)
    }

    /// Parses the public key and verifies `signature` is a valid signature of
    /// `digest` using it.
    ///
    /// See the [`crate::signature`] module-level documentation for examples.
    ///
    // # FIPS
    // Not allowed
    //
    /// # Errors
    /// `error::Unspecified` if inputs not verified.
    #[inline]
    pub fn verify_digest(
        &self,
        digest: &Digest,
        signature: &[u8],
    ) -> Result<(), error::Unspecified> {
        self.algorithm
            .verify_digest_sig(self.bytes.as_ref(), digest, signature)
    }

    /// Parses the public key bytes and returns a `ParsedPublicKey`.
    ///
    /// This method validates the public key format and creates a `ParsedPublicKey`
    /// that can be used for more efficient signature verification operations.
    /// The parsing overhead is incurred once, making subsequent verifications
    /// faster compared to using `UnparsedPublicKey::verify` directly.
    ///
    /// This is equivalent to calling [`ParsedPublicKey::new`] with the same
    /// algorithm and bytes.
    ///
    /// # Errors
    /// `KeyRejected` if the public key bytes are malformed or incompatible
    /// with the specified algorithm.
    pub fn parse(&self) -> Result<ParsedPublicKey, KeyRejected> {
        parse_public_key(self.bytes.as_ref(), self.algorithm)
    }
}

pub(crate) fn parse_public_key(
    bytes: &[u8],
    algorithm: &'static dyn VerificationAlgorithm,
) -> Result<ParsedPublicKey, KeyRejected> {
    let parsed_algorithm: &'static dyn ParsedVerificationAlgorithm;

    let key = if algorithm.type_id() == TypeId::of::<EcdsaVerificationAlgorithm>() {
        #[allow(clippy::cast_ptr_alignment)]
        let ec_alg = unsafe {
            &*(algorithm as *const dyn VerificationAlgorithm).cast::<EcdsaVerificationAlgorithm>()
        };
        parsed_algorithm = ec_alg;
        parse_ec_public_key(bytes, ec_alg.id.nid())?
    } else if algorithm.type_id() == TypeId::of::<EdDSAParameters>() {
        #[allow(clippy::cast_ptr_alignment)]
        let ed_alg =
            unsafe { &*(algorithm as *const dyn VerificationAlgorithm).cast::<EdDSAParameters>() };
        parsed_algorithm = ed_alg;
        parse_ed25519_public_key(bytes)?
    } else if algorithm.type_id() == TypeId::of::<RsaParameters>() {
        #[allow(clippy::cast_ptr_alignment)]
        let rsa_alg =
            unsafe { &*(algorithm as *const dyn VerificationAlgorithm).cast::<RsaParameters>() };
        parsed_algorithm = rsa_alg;
        parse_rsa_public_key(bytes)?
    } else {
        #[cfg(all(feature = "unstable", not(feature = "fips")))]
        if algorithm.type_id() == TypeId::of::<PqdsaVerificationAlgorithm>() {
            #[allow(clippy::cast_ptr_alignment)]
            let pqdsa_alg = unsafe {
                &*(algorithm as *const dyn VerificationAlgorithm)
                    .cast::<PqdsaVerificationAlgorithm>()
            };
            parsed_algorithm = pqdsa_alg;
            parse_pqdsa_public_key(bytes, pqdsa_alg.id)?
        } else {
            unreachable!()
        }
        #[cfg(any(not(feature = "unstable"), feature = "fips"))]
        unreachable!()
    };

    let bytes = bytes.to_vec().into_boxed_slice();
    Ok(ParsedPublicKey {
        algorithm,
        parsed_algorithm,
        key,
        bytes,
    })
}

/// Verification of signatures using RSA keys of 1024-8192 bits, PKCS#1.5 padding, and SHA-1.
pub const RSA_PKCS1_1024_8192_SHA1_FOR_LEGACY_USE_ONLY: RsaParameters = RsaParameters::new(
    &digest::SHA1_FOR_LEGACY_USE_ONLY,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    1024..=8192,
    &RsaVerificationAlgorithmId::RSA_PKCS1_1024_8192_SHA1_FOR_LEGACY_USE_ONLY,
);

/// Verification of signatures using RSA keys of 1024-8192 bits, PKCS#1.5 padding, and SHA-256.
pub const RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY: RsaParameters = RsaParameters::new(
    &digest::SHA256,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    1024..=8192,
    &RsaVerificationAlgorithmId::RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY,
);

/// Verification of signatures using RSA keys of 1024-8192 bits, PKCS#1.5 padding, and SHA-512.
pub const RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY: RsaParameters = RsaParameters::new(
    &digest::SHA512,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    1024..=8192,
    &RsaVerificationAlgorithmId::RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY,
);

/// Verification of signatures using RSA keys of 2048-8192 bits, PKCS#1.5 padding, and SHA-1.
pub const RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY: RsaParameters = RsaParameters::new(
    &digest::SHA1_FOR_LEGACY_USE_ONLY,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    2048..=8192,
    &RsaVerificationAlgorithmId::RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY,
);

/// Verification of signatures using RSA keys of 2048-8192 bits, PKCS#1.5 padding, and SHA-256.
pub const RSA_PKCS1_2048_8192_SHA256: RsaParameters = RsaParameters::new(
    &digest::SHA256,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    2048..=8192,
    &RsaVerificationAlgorithmId::RSA_PKCS1_2048_8192_SHA256,
);

/// Verification of signatures using RSA keys of 2048-8192 bits, PKCS#1.5 padding, and SHA-384.
pub const RSA_PKCS1_2048_8192_SHA384: RsaParameters = RsaParameters::new(
    &digest::SHA384,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    2048..=8192,
    &RsaVerificationAlgorithmId::RSA_PKCS1_2048_8192_SHA384,
);

/// Verification of signatures using RSA keys of 2048-8192 bits, PKCS#1.5 padding, and SHA-512.
pub const RSA_PKCS1_2048_8192_SHA512: RsaParameters = RsaParameters::new(
    &digest::SHA512,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    2048..=8192,
    &RsaVerificationAlgorithmId::RSA_PKCS1_2048_8192_SHA512,
);

/// Verification of signatures using RSA keys of 3072-8192 bits, PKCS#1.5 padding, and SHA-384.
pub const RSA_PKCS1_3072_8192_SHA384: RsaParameters = RsaParameters::new(
    &digest::SHA384,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    3072..=8192,
    &RsaVerificationAlgorithmId::RSA_PKCS1_3072_8192_SHA384,
);

/// Verification of signatures using RSA keys of 2048-8192 bits, PSS padding, and SHA-256.
pub const RSA_PSS_2048_8192_SHA256: RsaParameters = RsaParameters::new(
    &digest::SHA256,
    &rsa::signature::RsaPadding::RSA_PKCS1_PSS_PADDING,
    2048..=8192,
    &RsaVerificationAlgorithmId::RSA_PSS_2048_8192_SHA256,
);

/// Verification of signatures using RSA keys of 2048-8192 bits, PSS padding, and SHA-384.
pub const RSA_PSS_2048_8192_SHA384: RsaParameters = RsaParameters::new(
    &digest::SHA384,
    &rsa::signature::RsaPadding::RSA_PKCS1_PSS_PADDING,
    2048..=8192,
    &RsaVerificationAlgorithmId::RSA_PSS_2048_8192_SHA384,
);

/// Verification of signatures using RSA keys of 2048-8192 bits, PSS padding, and SHA-512.
pub const RSA_PSS_2048_8192_SHA512: RsaParameters = RsaParameters::new(
    &digest::SHA512,
    &rsa::signature::RsaPadding::RSA_PKCS1_PSS_PADDING,
    2048..=8192,
    &RsaVerificationAlgorithmId::RSA_PSS_2048_8192_SHA512,
);

/// RSA PSS padding using SHA-256 for RSA signatures.
pub const RSA_PSS_SHA256: RsaSignatureEncoding = RsaSignatureEncoding::new(
    &digest::SHA256,
    &rsa::signature::RsaPadding::RSA_PKCS1_PSS_PADDING,
    &RsaSigningAlgorithmId::RSA_PSS_SHA256,
);

/// RSA PSS padding using SHA-384 for RSA signatures.
pub const RSA_PSS_SHA384: RsaSignatureEncoding = RsaSignatureEncoding::new(
    &digest::SHA384,
    &rsa::signature::RsaPadding::RSA_PKCS1_PSS_PADDING,
    &RsaSigningAlgorithmId::RSA_PSS_SHA384,
);

/// RSA PSS padding using SHA-512 for RSA signatures.
pub const RSA_PSS_SHA512: RsaSignatureEncoding = RsaSignatureEncoding::new(
    &digest::SHA512,
    &rsa::signature::RsaPadding::RSA_PKCS1_PSS_PADDING,
    &RsaSigningAlgorithmId::RSA_PSS_SHA512,
);

/// PKCS#1 1.5 padding using SHA-256 for RSA signatures.
pub const RSA_PKCS1_SHA256: RsaSignatureEncoding = RsaSignatureEncoding::new(
    &digest::SHA256,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    &RsaSigningAlgorithmId::RSA_PKCS1_SHA256,
);

/// PKCS#1 1.5 padding using SHA-384 for RSA signatures.
pub const RSA_PKCS1_SHA384: RsaSignatureEncoding = RsaSignatureEncoding::new(
    &digest::SHA384,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    &RsaSigningAlgorithmId::RSA_PKCS1_SHA384,
);

/// PKCS#1 1.5 padding using SHA-512 for RSA signatures.
pub const RSA_PKCS1_SHA512: RsaSignatureEncoding = RsaSignatureEncoding::new(
    &digest::SHA512,
    &rsa::signature::RsaPadding::RSA_PKCS1_PADDING,
    &RsaSigningAlgorithmId::RSA_PKCS1_SHA512,
);

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-256 curve and SHA-256.
pub const ECDSA_P256_SHA256_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P256,
    digest: &digest::SHA256,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-384 curve and SHA-384.
pub const ECDSA_P384_SHA384_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P384,
    digest: &digest::SHA384,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-384 curve and SHA3-384.
pub const ECDSA_P384_SHA3_384_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P384,
    digest: &digest::SHA3_384,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-1.
pub const ECDSA_P521_SHA1_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA1_FOR_LEGACY_USE_ONLY,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-224.
pub const ECDSA_P521_SHA224_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA224,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-256.
pub const ECDSA_P521_SHA256_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA256,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-384.
pub const ECDSA_P521_SHA384_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA384,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-512.
pub const ECDSA_P521_SHA512_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA512,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA3-512.
pub const ECDSA_P521_SHA3_512_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA3_512,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-256K1 curve and SHA-256.
pub const ECDSA_P256K1_SHA256_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P256K1,
    digest: &digest::SHA256,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the P-256K1 curve and SHA3-256.
pub const ECDSA_P256K1_SHA3_256_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P256K1,
    digest: &digest::SHA3_256,
    sig_format: EcdsaSignatureFormat::Fixed,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-256 curve and SHA-256.
pub const ECDSA_P256_SHA256_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P256,
    digest: &digest::SHA256,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// *Not recommended.* Verification of ASN.1 DER-encoded ECDSA signatures using the P-256 curve and SHA-384.
pub const ECDSA_P256_SHA384_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P256,
    digest: &digest::SHA384,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// *Not recommended.* Verification of ASN.1 DER-encoded ECDSA signatures using the P-256 curve and SHA-512.
pub const ECDSA_P256_SHA512_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P256,
    digest: &digest::SHA512,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// *Not recommended.* Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA-256.
pub const ECDSA_P384_SHA256_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P384,
    digest: &digest::SHA256,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA-384.
pub const ECDSA_P384_SHA384_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P384,
    digest: &digest::SHA384,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// *Not recommended.* Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA-512.
pub const ECDSA_P384_SHA512_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P384,
    digest: &digest::SHA512,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA3-384.
pub const ECDSA_P384_SHA3_384_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P384,
    digest: &digest::SHA3_384,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-1.
pub const ECDSA_P521_SHA1_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA1_FOR_LEGACY_USE_ONLY,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-224.
pub const ECDSA_P521_SHA224_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA224,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-256.
pub const ECDSA_P521_SHA256_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA256,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-384.
pub const ECDSA_P521_SHA384_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA384,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-512.
pub const ECDSA_P521_SHA512_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA512,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA3-512.
pub const ECDSA_P521_SHA3_512_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P521,
    digest: &digest::SHA3_512,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-256K1 curve and SHA-256.
pub const ECDSA_P256K1_SHA256_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P256K1,
    digest: &digest::SHA256,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-256K1 curve and SHA3-256.
pub const ECDSA_P256K1_SHA3_256_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    id: &ec::signature::AlgorithmID::ECDSA_P256K1,
    digest: &digest::SHA3_256,
    sig_format: EcdsaSignatureFormat::ASN1,
};

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-256 curve and SHA-256.
pub const ECDSA_P256_SHA256_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P256_SHA256_FIXED);

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-384 curve and SHA-384.
pub const ECDSA_P384_SHA384_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P384_SHA384_FIXED);

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-384 curve and SHA3-384.
pub const ECDSA_P384_SHA3_384_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P384_SHA3_384_FIXED);

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-224.
/// # ⚠️ Warning
/// The security design strength of SHA-224 digests is less then security strength of P-521.
/// This scheme should only be used for backwards compatibility purposes.
pub const ECDSA_P521_SHA224_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA224_FIXED);

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-256.
/// # ⚠️ Warning
/// The security design strength of SHA-256 digests is less then security strength of P-521.
/// This scheme should only be used for backwards compatibility purposes.
pub const ECDSA_P521_SHA256_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA256_FIXED);

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-384.
/// # ⚠️ Warning
/// The security design strength of SHA-384 digests is less then security strength of P-521.
/// This scheme should only be used for backwards compatibility purposes.
pub const ECDSA_P521_SHA384_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA384_FIXED);

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA-512.
pub const ECDSA_P521_SHA512_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA512_FIXED);

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-521 curve and SHA3-512.
pub const ECDSA_P521_SHA3_512_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA3_512_FIXED);

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-256K1 curve and SHA-256.
pub const ECDSA_P256K1_SHA256_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P256K1_SHA256_FIXED);

/// Signing of fixed-length (PKCS#11 style) ECDSA signatures using the P-256K1 curve and SHA3-256.
pub const ECDSA_P256K1_SHA3_256_FIXED_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P256K1_SHA3_256_FIXED);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-256 curve and SHA-256.
pub const ECDSA_P256_SHA256_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P256_SHA256_ASN1);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA-384.
pub const ECDSA_P384_SHA384_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P384_SHA384_ASN1);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-384 curve and SHA3-384.
pub const ECDSA_P384_SHA3_384_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P384_SHA3_384_ASN1);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-224.
/// # ⚠️ Warning
/// The security design strength of SHA-224 digests is less then security strength of P-521.
/// This scheme should only be used for backwards compatibility purposes.
pub const ECDSA_P521_SHA224_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA224_ASN1);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-256.
/// # ⚠️ Warning
/// The security design strength of SHA-256 digests is less then security strength of P-521.
/// This scheme should only be used for backwards compatibility purposes.
pub const ECDSA_P521_SHA256_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA256_ASN1);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-384.
/// # ⚠️ Warning
/// The security design strength of SHA-384 digests is less then security strength of P-521.
/// This scheme should only be used for backwards compatibility purposes.
pub const ECDSA_P521_SHA384_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA384_ASN1);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA-512.
pub const ECDSA_P521_SHA512_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA512_ASN1);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-521 curve and SHA3-512.
pub const ECDSA_P521_SHA3_512_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P521_SHA3_512_ASN1);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-256K1 curve and SHA-256.
pub const ECDSA_P256K1_SHA256_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P256K1_SHA256_ASN1);

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-256K1 curve and SHA3-256.
pub const ECDSA_P256K1_SHA3_256_ASN1_SIGNING: EcdsaSigningAlgorithm =
    EcdsaSigningAlgorithm(&ECDSA_P256K1_SHA3_256_ASN1);

/// Verification of Ed25519 signatures.
pub const ED25519: EdDSAParameters = EdDSAParameters {};

#[cfg(test)]
mod tests {
    use crate::rand::{generate, SystemRandom};
    use crate::signature::{ParsedPublicKey, UnparsedPublicKey, ED25519};
    use crate::test;
    use regex::Regex;

    #[cfg(feature = "fips")]
    mod fips;

    #[test]
    fn test_unparsed_public_key() {
        let random_pubkey: [u8; 32] = generate(&SystemRandom::new()).unwrap().expose();
        let unparsed_pubkey = UnparsedPublicKey::new(&ED25519, random_pubkey);
        let unparsed_pubkey_debug = format!("{unparsed_pubkey:?}");

        #[allow(clippy::clone_on_copy)]
        let unparsed_pubkey_clone = unparsed_pubkey.clone();
        assert_eq!(unparsed_pubkey_debug, format!("{unparsed_pubkey_clone:?}"));
        let pubkey_re = Regex::new(
            "UnparsedPublicKey \\{ algorithm: EdDSAParameters, bytes: \"[0-9a-f]{64}\" \\}",
        )
        .unwrap();

        assert!(pubkey_re.is_match(&unparsed_pubkey_debug));
    }
    #[test]
    fn test_types() {
        test::compile_time_assert_send::<UnparsedPublicKey<&[u8]>>();
        test::compile_time_assert_sync::<UnparsedPublicKey<&[u8]>>();
        test::compile_time_assert_send::<UnparsedPublicKey<Vec<u8>>>();
        test::compile_time_assert_sync::<UnparsedPublicKey<Vec<u8>>>();
        test::compile_time_assert_clone::<UnparsedPublicKey<&[u8]>>();
        test::compile_time_assert_clone::<UnparsedPublicKey<Vec<u8>>>();
        test::compile_time_assert_send::<ParsedPublicKey>();
        test::compile_time_assert_sync::<ParsedPublicKey>();
        test::compile_time_assert_clone::<ParsedPublicKey>();
    }
}
