#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::checked_conversions,
    clippy::implicit_saturating_sub,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

//! ## `serde` support
//!
//! When the `serde` feature of this crate is enabled, `Serialize` and
//! `Deserialize` impls are provided for the [`Signature`] and [`VerifyingKey`]
//! types.
//!
//! Please see type-specific documentation for more information.
//!
//! ## Interop
//!
//! Any crates which provide an implementation of ECDSA for a particular
//! elliptic curve can leverage the types from this crate, along with the
//! [`k256`], [`p256`], and/or [`p384`] crates to expose ECDSA functionality in
//! a generic, interoperable way by leveraging the [`Signature`] type with in
//! conjunction with the [`signature::Signer`] and [`signature::Verifier`]
//! traits.
//!
//! For example, the [`ring-compat`] crate implements the [`signature::Signer`]
//! and [`signature::Verifier`] traits in conjunction with the
//! [`p256::ecdsa::Signature`] and [`p384::ecdsa::Signature`] types to
//! wrap the ECDSA implementations from [*ring*] in a generic, interoperable
//! API.
//!
//! [`k256`]: https://docs.rs/k256
//! [`p256`]: https://docs.rs/p256
//! [`p256::ecdsa::Signature`]: https://docs.rs/p256/latest/p256/ecdsa/type.Signature.html
//! [`p384`]: https://docs.rs/p384
//! [`p384::ecdsa::Signature`]: https://docs.rs/p384/latest/p384/ecdsa/type.Signature.html
//! [`ring-compat`]: https://docs.rs/ring-compat
//! [*ring*]: https://docs.rs/ring

#[cfg(feature = "alloc")]
extern crate alloc;

mod recovery;

#[cfg(feature = "der")]
#[cfg_attr(docsrs, doc(cfg(feature = "der")))]
pub mod der;

#[cfg(feature = "dev")]
#[cfg_attr(docsrs, doc(cfg(feature = "dev")))]
pub mod dev;

#[cfg(feature = "hazmat")]
#[cfg_attr(docsrs, doc(cfg(feature = "hazmat")))]
pub mod hazmat;

#[cfg(feature = "sign")]
mod sign;

#[cfg(feature = "verify")]
mod verify;

pub use crate::recovery::RecoveryId;

// Re-export the `elliptic-curve` crate (and select types)
pub use elliptic_curve::{self, sec1::EncodedPoint, PrimeCurve};

// Re-export the `signature` crate (and select types)
pub use signature::{self, Error, Result};

#[cfg(feature = "sign")]
#[cfg_attr(docsrs, doc(cfg(feature = "sign")))]
pub use crate::sign::SigningKey;

#[cfg(feature = "verify")]
#[cfg_attr(docsrs, doc(cfg(feature = "verify")))]
pub use crate::verify::VerifyingKey;

use core::{
    fmt::{self, Debug},
    ops::Add,
};
use elliptic_curve::{
    bigint::Encoding as _,
    generic_array::{sequence::Concat, ArrayLength, GenericArray},
    FieldBytes, FieldSize, ScalarCore,
};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "arithmetic")]
use {
    core::str,
    elliptic_curve::{ff::PrimeField, IsHigh, NonZeroScalar, ScalarArithmetic},
};

#[cfg(feature = "serde")]
use serdect::serde::{de, ser, Deserialize, Serialize};

/// Size of a fixed sized signature for the given elliptic curve.
pub type SignatureSize<C> = <FieldSize<C> as Add>::Output;

/// Fixed-size byte array containing an ECDSA signature
pub type SignatureBytes<C> = GenericArray<u8, SignatureSize<C>>;

/// ECDSA signature (fixed-size). Generic over elliptic curve types.
///
/// Serialized as fixed-sized big endian scalar values with no added framing:
///
/// - `r`: field element size for the given curve, big-endian
/// - `s`: field element size for the given curve, big-endian
///
/// For example, in a curve with a 256-bit modulus like NIST P-256 or
/// secp256k1, `r` and `s` will both be 32-bytes, resulting in a signature
/// with a total of 64-bytes.
///
/// ASN.1 DER-encoded signatures also supported via the
/// [`Signature::from_der`] and [`Signature::to_der`] methods.
///
/// # `serde` support
///
/// When the `serde` feature of this crate is enabled, it provides support for
/// serializing and deserializing ECDSA signatures using the `Serialize` and
/// `Deserialize` traits.
///
/// The serialization uses a 64-byte fixed encoding when used with binary
/// formats, and a hexadecimal encoding when used with text formats.
#[derive(Clone, Eq, PartialEq)]
pub struct Signature<C: PrimeCurve>
where
    SignatureSize<C>: ArrayLength<u8>,
{
    bytes: SignatureBytes<C>,
}

impl<C> Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    /// Parse a signature from ASN.1 DER
    #[cfg(feature = "der")]
    #[cfg_attr(docsrs, doc(cfg(feature = "der")))]
    pub fn from_der(bytes: &[u8]) -> Result<Self>
    where
        der::MaxSize<C>: ArrayLength<u8>,
        <FieldSize<C> as Add>::Output: Add<der::MaxOverhead> + ArrayLength<u8>,
    {
        der::Signature::<C>::try_from(bytes).and_then(Self::try_from)
    }

    /// Create a [`Signature`] from the serialized `r` and `s` scalar values
    /// which comprise the signature.
    pub fn from_scalars(r: impl Into<FieldBytes<C>>, s: impl Into<FieldBytes<C>>) -> Result<Self> {
        Self::try_from(r.into().concat(s.into()).as_slice())
    }

    /// Split the signature into its `r` and `s` components, represented as bytes.
    pub fn split_bytes(&self) -> (FieldBytes<C>, FieldBytes<C>) {
        let (r_bytes, s_bytes) = self.bytes.split_at(C::UInt::BYTE_SIZE);

        (
            GenericArray::clone_from_slice(r_bytes),
            GenericArray::clone_from_slice(s_bytes),
        )
    }

    /// Serialize this signature as ASN.1 DER
    #[cfg(feature = "der")]
    #[cfg_attr(docsrs, doc(cfg(feature = "der")))]
    pub fn to_der(&self) -> der::Signature<C>
    where
        der::MaxSize<C>: ArrayLength<u8>,
        <FieldSize<C> as Add>::Output: Add<der::MaxOverhead> + ArrayLength<u8>,
    {
        let (r, s) = self.bytes.split_at(C::UInt::BYTE_SIZE);
        der::Signature::from_scalar_bytes(r, s).expect("DER encoding error")
    }

    /// Convert this signature into a byte vector.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}

#[cfg(feature = "arithmetic")]
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
impl<C> Signature<C>
where
    C: PrimeCurve + ScalarArithmetic,
    SignatureSize<C>: ArrayLength<u8>,
{
    /// Get the `r` component of this signature
    pub fn r(&self) -> NonZeroScalar<C> {
        NonZeroScalar::try_from(self.split_bytes().0.as_slice())
            .expect("r-component ensured valid in constructor")
    }

    /// Get the `s` component of this signature
    pub fn s(&self) -> NonZeroScalar<C> {
        NonZeroScalar::try_from(self.split_bytes().1.as_slice())
            .expect("s-component ensured valid in constructor")
    }

    /// Split the signature into its `r` and `s` scalars.
    pub fn split_scalars(&self) -> (NonZeroScalar<C>, NonZeroScalar<C>) {
        (self.r(), self.s())
    }

    /// Normalize signature into "low S" form as described in
    /// [BIP 0062: Dealing with Malleability][1].
    ///
    /// [1]: https://github.com/bitcoin/bips/blob/master/bip-0062.mediawiki
    pub fn normalize_s(&self) -> Option<Self> {
        let s = self.s();

        if s.is_high().into() {
            let neg_s = -s;
            let mut result = self.clone();
            result.bytes[C::UInt::BYTE_SIZE..].copy_from_slice(&neg_s.to_repr());
            Some(result)
        } else {
            None
        }
    }
}

impl<C> signature::Signature for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Self::try_from(bytes)
    }
}

impl<C> AsRef<[u8]> for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

impl<C> Copy for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
    <SignatureSize<C> as ArrayLength<u8>>::ArrayType: Copy,
{
}

impl<C> Debug for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ecdsa::Signature<{:?}>(", C::default())?;

        for &byte in self.as_ref() {
            write!(f, "{:02X}", byte)?;
        }

        write!(f, ")")
    }
}

impl<C> TryFrom<&[u8]> for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != C::UInt::BYTE_SIZE * 2 {
            return Err(Error::new());
        }

        for scalar_bytes in bytes.chunks_exact(C::UInt::BYTE_SIZE) {
            let scalar = ScalarCore::<C>::from_be_slice(scalar_bytes).map_err(|_| Error::new())?;

            if scalar.is_zero().into() {
                return Err(Error::new());
            }
        }

        Ok(Self {
            bytes: GenericArray::clone_from_slice(bytes),
        })
    }
}

impl<C> fmt::Display for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<C> fmt::LowerHex for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl<C> fmt::UpperHex for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

#[cfg(feature = "arithmetic")]
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
impl<C> str::FromStr for Signature<C>
where
    C: PrimeCurve + ScalarArithmetic,
    SignatureSize<C>: ArrayLength<u8>,
{
    type Err = Error;

    fn from_str(hex: &str) -> Result<Self> {
        if hex.as_bytes().len() != C::UInt::BYTE_SIZE * 4 {
            return Err(Error::new());
        }

        // This check is mainly to ensure `hex.split_at` below won't panic
        if !hex
            .as_bytes()
            .iter()
            .all(|&byte| matches!(byte, b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z'))
        {
            return Err(Error::new());
        }

        let (r_hex, s_hex) = hex.split_at(C::UInt::BYTE_SIZE * 2);

        let r = r_hex
            .parse::<NonZeroScalar<C>>()
            .map_err(|_| Error::new())?;

        let s = s_hex
            .parse::<NonZeroScalar<C>>()
            .map_err(|_| Error::new())?;

        Self::from_scalars(r, s)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<C> Serialize for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serdect::array::serialize_hex_upper_or_bin(&self.bytes, serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de, C> Deserialize<'de> for Signature<C>
where
    C: PrimeCurve,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let mut bytes = SignatureBytes::<C>::default();
        serdect::array::deserialize_hex_or_bin(&mut bytes, deserializer)?;
        Self::try_from(bytes.as_slice()).map_err(de::Error::custom)
    }
}
