#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(
    clippy::mod_module_files,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

//! ## About this crate
//! This library provides generalized PKCS#8 support designed to work with a
//! number of different algorithms. It supports `no_std` platforms including
//! ones without a heap (albeit with reduced functionality).
//!
//! It supports decoding/encoding the following types:
//!
//! - [`EncryptedPrivateKeyInfo`]: (with `pkcs5` feature) encrypted key.
//! - [`PrivateKeyInfo`]: algorithm identifier and data representing a private key.
//!   Optionally also includes public key data for asymmetric keys.
//! - [`SubjectPublicKeyInfo`]: algorithm identifier and data representing a public key
//!   (re-exported from the [`spki`] crate)
//!
//! When the `pem` feature is enabled, it also supports decoding/encoding
//! documents from "PEM encoding" format as defined in RFC 7468.
//!
//! ## Encrypted Private Key Support
//! [`EncryptedPrivateKeyInfo`] supports decoding/encoding encrypted PKCS#8
//! private keys and is gated under the `pkcs5` feature.
//!
//! When the `encryption` feature of this crate is enabled, it provides
//! [`EncryptedPrivateKeyInfo::decrypt`] and [`PrivateKeyInfo::encrypt`]
//! functions which are able to decrypt/encrypt keys using the following
//! algorithms:
//!
//! - [PKCS#5v2 Password Based Encryption Scheme 2 (RFC 8018)]
//!   - Key derivation functions:
//!     - [scrypt] ([RFC 7914])
//!     - PBKDF2 ([RFC 8018](https://datatracker.ietf.org/doc/html/rfc8018#section-5.2))
//!       - SHA-2 based PRF with HMAC-SHA224, HMAC-SHA256, HMAC-SHA384, or HMAC-SHA512
//!       - SHA-1 based PRF with HMAC-SHA1, when the `sha1` feature of this crate is enabled.
//!   - Symmetric encryption: AES-128-CBC, AES-192-CBC, or AES-256-CBC
//!     (best available options for PKCS#5v2)
//!  
//! ## Legacy DES-CBC and DES-EDE3-CBC (3DES) support (optional)
//! When the `des-insecure` and/or `3des` features are enabled this crate provides support for
//! private keys encrypted with with DES-CBC and DES-EDE3-CBC (3DES or Triple DES) symmetric
//! encryption, respectively.
//!
//! ⚠️ WARNING ⚠️
//!
//! DES support (gated behind the `des-insecure` feature) is implemented to
//! allow for decryption of legacy PKCS#8 files only.
//!
//! Such PKCS#8 documents should be considered *INSECURE* due to the short
//! 56-bit key size of DES.
//!
//! New keys should use AES instead.
//!
//! [RFC 5208]: https://tools.ietf.org/html/rfc5208
//! [RFC 5958]: https://tools.ietf.org/html/rfc5958
//! [RFC 7914]: https://datatracker.ietf.org/doc/html/rfc7914
//! [PKCS#5v2 Password Based Encryption Scheme 2 (RFC 8018)]: https://tools.ietf.org/html/rfc8018#section-6.2
//! [scrypt]: https://en.wikipedia.org/wiki/Scrypt

#[cfg(feature = "pem")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod error;
mod private_key_info;
mod traits;
mod version;

#[cfg(feature = "pkcs5")]
pub(crate) mod encrypted_private_key_info;

pub use crate::{
    error::{Error, Result},
    private_key_info::PrivateKeyInfo,
    traits::DecodePrivateKey,
    version::Version,
};
pub use der::{self, asn1::ObjectIdentifier, oid::AssociatedOid};
pub use spki::{
    self, AlgorithmIdentifierRef, DecodePublicKey, SubjectPublicKeyInfo, SubjectPublicKeyInfoRef,
};

#[cfg(feature = "alloc")]
pub use {
    crate::traits::EncodePrivateKey,
    der::{Document, SecretDocument},
    spki::EncodePublicKey,
};

#[cfg(feature = "pem")]
pub use der::pem::LineEnding;

#[cfg(feature = "pkcs5")]
pub use {encrypted_private_key_info::EncryptedPrivateKeyInfo, pkcs5};

#[cfg(feature = "rand_core")]
pub use rand_core;
