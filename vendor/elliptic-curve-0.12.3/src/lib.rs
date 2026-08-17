#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg"
)]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

//! ## Usage
//!
//! This crate provides traits for describing elliptic curves, along with
//! types which are generic over elliptic curves which can be used as the
//! basis of curve-agnostic code.
//!
//! It's intended to be used with the following concrete elliptic curve
//! implementations from the [`RustCrypto/elliptic-curves`] project:
//!
//! - [`bp256`]: brainpoolP256r1 and brainpoolP256t1
//! - [`bp384`]: brainpoolP384r1 and brainpoolP384t1
//! - [`k256`]: secp256k1 a.k.a. K-256
//! - [`p256`]: NIST P-256 a.k.a secp256r1, prime256v1
//! - [`p384`]: NIST P-384 a.k.a. secp384r1
//!
//! The [`ecdsa`] crate provides a generic implementation of the
//! Elliptic Curve Digital Signature Algorithm which can be used with any of
//! the above crates, either via an external ECDSA implementation, or
//! using native curve arithmetic where applicable.
//!
//! ## Type conversions
//!
//! The following chart illustrates the various conversions possible between
//! the various types defined by this crate.
//!
//! ![Type Conversion Map](https://raw.githubusercontent.com/RustCrypto/media/master/img/elliptic-curve/type-transforms.svg)
//!
//! ## `serde` support
//!
//! When the `serde` feature of this crate is enabled, `Serialize` and
//! `Deserialize` impls are provided for the following types:
//!
//! - [`JwkEcKey`]
//! - [`PublicKey`]
//! - [`ScalarCore`]
//!
//! Please see type-specific documentation for more information.
//!
//! [`RustCrypto/elliptic-curves`]: https://github.com/RustCrypto/elliptic-curves
//! [`bp256`]: https://github.com/RustCrypto/elliptic-curves/tree/master/bp256
//! [`bp384`]: https://github.com/RustCrypto/elliptic-curves/tree/master/bp384
//! [`k256`]: https://github.com/RustCrypto/elliptic-curves/tree/master/k256
//! [`p256`]: https://github.com/RustCrypto/elliptic-curves/tree/master/p256
//! [`p384`]: https://github.com/RustCrypto/elliptic-curves/tree/master/p384
//! [`ecdsa`]: https://github.com/RustCrypto/signatures/tree/master/ecdsa

#[cfg(feature = "alloc")]
#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "rand_core")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
pub use rand_core;

#[macro_use]
mod macros;

pub mod ops;

#[cfg(feature = "dev")]
#[cfg_attr(docsrs, doc(cfg(feature = "dev")))]
pub mod dev;

#[cfg(feature = "ecdh")]
#[cfg_attr(docsrs, doc(cfg(feature = "ecdh")))]
pub mod ecdh;

#[cfg(feature = "hash2curve")]
#[cfg_attr(docsrs, doc(cfg(feature = "hash2curve")))]
pub mod hash2curve;

#[cfg(feature = "sec1")]
#[cfg_attr(docsrs, doc(cfg(feature = "sec1")))]
pub mod sec1;

#[cfg(feature = "arithmetic")]
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
pub mod weierstrass;

mod error;
mod point;
mod scalar;
mod secret_key;

#[cfg(feature = "arithmetic")]
mod arithmetic;
#[cfg(feature = "arithmetic")]
mod public_key;

#[cfg(feature = "jwk")]
mod jwk;

pub use crate::{
    error::{Error, Result},
    point::{
        AffineXCoordinate, DecompactPoint, DecompressPoint, PointCompaction, PointCompression,
    },
    scalar::{core::ScalarCore, IsHigh},
    secret_key::SecretKey,
};
pub use crypto_bigint as bigint;
pub use generic_array::{self, typenum::consts};
pub use rand_core;
pub use subtle;
pub use zeroize;

#[cfg(feature = "arithmetic")]
pub use {
    crate::{
        arithmetic::{
            AffineArithmetic, PrimeCurveArithmetic, ProjectiveArithmetic, ScalarArithmetic,
        },
        public_key::PublicKey,
        scalar::{nonzero::NonZeroScalar, Scalar},
    },
    ff::{self, Field, PrimeField},
    group::{self, Group},
};

#[cfg(feature = "bits")]
pub use crate::scalar::ScalarBits;

#[cfg(feature = "jwk")]
pub use crate::jwk::{JwkEcKey, JwkParameters};

#[cfg(feature = "pkcs8")]
pub use pkcs8;

use core::fmt::Debug;
use generic_array::GenericArray;

/// Algorithm [`ObjectIdentifier`][`pkcs8::ObjectIdentifier`] for elliptic
/// curve public key cryptography (`id-ecPublicKey`).
///
/// <http://oid-info.com/get/1.2.840.10045.2.1>
#[cfg(feature = "pkcs8")]
#[cfg_attr(docsrs, doc(cfg(feature = "pkcs8")))]
pub const ALGORITHM_OID: pkcs8::ObjectIdentifier =
    pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.2.1");

/// Elliptic curve.
///
/// This trait is intended to be impl'd by a ZST which represents a concrete
/// elliptic curve.
///
/// Other traits in this crate which are bounded by [`Curve`] are intended to
/// be impl'd by these ZSTs, facilitating types which are generic over elliptic
/// curves (e.g. [`SecretKey`]).
pub trait Curve: 'static + Copy + Clone + Debug + Default + Eq + Ord + Send + Sync {
    /// Integer type used to represent field elements of this elliptic curve.
    // TODO(tarcieri): replace this with an e.g. `const Curve::MODULUS: UInt`.
    // Requires rust-lang/rust#60551, i.e. `const_evaluatable_checked`
    type UInt: bigint::AddMod<Output = Self::UInt>
        + bigint::ArrayEncoding
        + bigint::Encoding
        + bigint::Integer
        + bigint::NegMod<Output = Self::UInt>
        + bigint::Random
        + bigint::RandomMod
        + bigint::SubMod<Output = Self::UInt>
        + zeroize::Zeroize;

    /// Order constant.
    ///
    /// Subdivided into either 32-bit or 64-bit "limbs" (depending on the
    /// target CPU's word size), specified from least to most significant.
    const ORDER: Self::UInt;
}

/// Marker trait for elliptic curves with prime order.
pub trait PrimeCurve: Curve {}

/// Size of field elements of this elliptic curve.
pub type FieldSize<C> = <<C as Curve>::UInt as bigint::ArrayEncoding>::ByteSize;

/// Byte representation of a base/scalar field element of a given curve.
pub type FieldBytes<C> = GenericArray<u8, FieldSize<C>>;

/// Affine point type for a given curve with a [`ProjectiveArithmetic`]
/// implementation.
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
#[cfg(feature = "arithmetic")]
pub type AffinePoint<C> = <C as AffineArithmetic>::AffinePoint;

/// Projective point type for a given curve with a [`ProjectiveArithmetic`]
/// implementation.
#[cfg(feature = "arithmetic")]
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
pub type ProjectivePoint<C> = <C as ProjectiveArithmetic>::ProjectivePoint;

/// Elliptic curve parameters used by VOPRF.
#[cfg(feature = "voprf")]
#[cfg_attr(docsrs, doc(cfg(feature = "voprf")))]
pub trait VoprfParameters: Curve {
    /// The `ID` parameter which identifies a particular elliptic curve
    /// as defined in [section 4 of `draft-irtf-cfrg-voprf-08`][voprf].
    ///
    /// [voprf]: https://www.ietf.org/archive/id/draft-irtf-cfrg-voprf-08.html#section-4
    const ID: u16;

    /// The `Hash` parameter which assigns a particular hash function to this
    /// ciphersuite as defined in [section 4 of `draft-irtf-cfrg-voprf-08`][voprf].
    ///
    /// [voprf]: https://www.ietf.org/archive/id/draft-irtf-cfrg-voprf-08.html#section-4
    type Hash: digest::Digest;
}
