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

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod error;
mod params;
mod private_key;
mod public_key;
mod traits;
mod version;

pub use der::{
    self,
    asn1::{ObjectIdentifier, UintRef},
};

pub use crate::{
    error::{Error, Result},
    params::{RsaOaepParams, RsaPssParams, TrailerField},
    private_key::RsaPrivateKey,
    public_key::RsaPublicKey,
    traits::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    version::Version,
};

#[cfg(feature = "alloc")]
pub use crate::{
    private_key::{other_prime_info::OtherPrimeInfo, OtherPrimeInfos},
    traits::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
};

#[cfg(feature = "pem")]
pub use der::pem::{self, LineEnding};

/// `rsaEncryption` Object Identifier (OID)
#[cfg(feature = "pkcs8")]
pub const ALGORITHM_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.1");

/// `AlgorithmIdentifier` for RSA.
#[cfg(feature = "pkcs8")]
pub const ALGORITHM_ID: pkcs8::AlgorithmIdentifierRef<'static> = pkcs8::AlgorithmIdentifierRef {
    oid: ALGORITHM_OID,
    parameters: Some(der::asn1::AnyRef::NULL),
};
