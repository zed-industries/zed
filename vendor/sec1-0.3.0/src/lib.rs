#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"
)]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

//! ## `serde` support
//!
//! When the `serde` feature of this crate is enabled, the [`EncodedPoint`]
//! type receives impls of [`serde::Serialize`] and [`serde::Deserialize`].
//!
//! Additionally, when both the `alloc` and `serde` features are enabled, the
//! serializers/deserializers will autodetect if a "human friendly" textual
//! encoding is being used, and if so encode the points as hexadecimal.

#[cfg(feature = "alloc")]
#[allow(unused_extern_crates)]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "point")]
pub mod point;

mod error;
#[cfg(feature = "der")]
mod parameters;
#[cfg(feature = "der")]
mod private_key;
#[cfg(feature = "der")]
mod traits;

#[cfg(feature = "der")]
pub use der;

pub use crate::error::{Error, Result};

#[cfg(feature = "point")]
pub use crate::point::EncodedPoint;

#[cfg(feature = "point")]
pub use generic_array::typenum::consts;

#[cfg(feature = "der")]
pub use crate::{parameters::EcParameters, private_key::EcPrivateKey, traits::DecodeEcPrivateKey};

#[cfg(feature = "alloc")]
pub use crate::traits::EncodeEcPrivateKey;

#[cfg(feature = "pem")]
#[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
pub use der::pem::{self, LineEnding};

#[cfg(feature = "pkcs8")]
#[cfg_attr(docsrs, doc(cfg(feature = "pkcs8")))]
pub use pkcs8;

#[cfg(feature = "pkcs8")]
use pkcs8::ObjectIdentifier;

#[cfg(all(doc, feature = "serde"))]
use serdect::serde;

/// Algorithm [`ObjectIdentifier`] for elliptic curve public key cryptography
/// (`id-ecPublicKey`).
///
/// <http://oid-info.com/get/1.2.840.10045.2.1>
#[cfg(feature = "pkcs8")]
#[cfg_attr(docsrs, doc(cfg(feature = "pkcs8")))]
pub const ALGORITHM_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.10045.2.1");
