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
//! # Usage
//! The following example demonstrates how to use an OID as the `parameters`
//! of an [`AlgorithmIdentifier`].
//!
//! Borrow the [`ObjectIdentifier`] first then use [`der::AnyRef::from`] or `.into()`:
//!
//! ```
//! use spki::{AlgorithmIdentifier, ObjectIdentifier};
//!
//! let alg_oid = "1.2.840.10045.2.1".parse::<ObjectIdentifier>().unwrap();
//! let params_oid = "1.2.840.10045.3.1.7".parse::<ObjectIdentifier>().unwrap();
//!
//! let alg_id = AlgorithmIdentifier {
//!     oid: alg_oid,
//!     parameters: Some(params_oid)
//! };
//! ```

#[cfg(feature = "alloc")]
#[allow(unused_extern_crates)]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod algorithm;
mod error;
mod spki;
mod traits;

#[cfg(feature = "fingerprint")]
mod fingerprint;

pub use crate::{
    algorithm::{AlgorithmIdentifier, AlgorithmIdentifierRef, AlgorithmIdentifierWithOid},
    error::{Error, Result},
    spki::{SubjectPublicKeyInfo, SubjectPublicKeyInfoRef},
    traits::{AssociatedAlgorithmIdentifier, DecodePublicKey, SignatureAlgorithmIdentifier},
};
pub use der::{self, asn1::ObjectIdentifier};

#[cfg(feature = "alloc")]
pub use {
    crate::{
        algorithm::AlgorithmIdentifierOwned,
        spki::SubjectPublicKeyInfoOwned,
        traits::{
            DynAssociatedAlgorithmIdentifier, DynSignatureAlgorithmIdentifier, EncodePublicKey,
            SignatureBitStringEncoding,
        },
    },
    der::Document,
};

#[cfg(feature = "fingerprint")]
pub use crate::fingerprint::FingerprintBytes;
