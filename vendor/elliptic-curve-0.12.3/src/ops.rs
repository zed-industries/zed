//! Traits for arithmetic operations on elliptic curve field elements.

pub use core::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crypto_bigint::{ArrayEncoding, ByteArray, Integer};

#[cfg(feature = "arithmetic")]
use {group::Group, subtle::CtOption};

#[cfg(feature = "digest")]
use digest::FixedOutput;

/// Perform an inversion on a field element (i.e. base field element or scalar)
pub trait Invert {
    /// Field element type
    type Output;

    /// Invert a field element.
    fn invert(&self) -> Self::Output;
}

#[cfg(feature = "arithmetic")]
impl<F: ff::Field> Invert for F {
    type Output = CtOption<F>;

    fn invert(&self) -> CtOption<F> {
        ff::Field::invert(self)
    }
}

/// Linear combination.
///
/// This trait enables crates to provide an optimized implementation of
/// linear combinations (e.g. Shamir's Trick), or otherwise provides a default
/// non-optimized implementation.
// TODO(tarcieri): replace this with a trait from the `group` crate? (see zkcrypto/group#25)
#[cfg(feature = "arithmetic")]
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
pub trait LinearCombination: Group {
    /// Calculates `x * k + y * l`.
    fn lincomb(x: &Self, k: &Self::Scalar, y: &Self, l: &Self::Scalar) -> Self {
        (*x * k) + (*y * l)
    }
}

/// Modular reduction.
pub trait Reduce<UInt: Integer + ArrayEncoding>: Sized {
    /// Perform a modular reduction, returning a field element.
    fn from_uint_reduced(n: UInt) -> Self;

    /// Interpret the given byte array as a big endian integer and perform
    /// a modular reduction.
    fn from_be_bytes_reduced(bytes: ByteArray<UInt>) -> Self {
        Self::from_uint_reduced(UInt::from_be_byte_array(bytes))
    }

    /// Interpret the given byte array as a little endian integer and perform a
    /// modular reduction.
    fn from_le_bytes_reduced(bytes: ByteArray<UInt>) -> Self {
        Self::from_uint_reduced(UInt::from_le_byte_array(bytes))
    }

    /// Interpret a digest as a big endian integer and perform a modular
    /// reduction.
    #[cfg(feature = "digest")]
    #[cfg_attr(docsrs, doc(cfg(feature = "digest")))]
    fn from_be_digest_reduced<D>(digest: D) -> Self
    where
        D: FixedOutput<OutputSize = UInt::ByteSize>,
    {
        Self::from_be_bytes_reduced(digest.finalize_fixed())
    }

    /// Interpret a digest as a little endian integer and perform a modular
    /// reduction.
    #[cfg(feature = "digest")]
    #[cfg_attr(docsrs, doc(cfg(feature = "digest")))]
    fn from_le_digest_reduced<D>(digest: D) -> Self
    where
        D: FixedOutput<OutputSize = UInt::ByteSize>,
    {
        Self::from_le_bytes_reduced(digest.finalize_fixed())
    }
}

/// Modular reduction to a non-zero output.
///
/// This trait is primarily intended for use by curve implementations such
/// as the `k256` and `p256` crates.
///
/// End users should use the [`Reduce`] impl on
/// [`NonZeroScalar`][`crate::NonZeroScalar`] instead.
pub trait ReduceNonZero<UInt: Integer + ArrayEncoding>: Sized {
    /// Perform a modular reduction, returning a field element.
    fn from_uint_reduced_nonzero(n: UInt) -> Self;
}
