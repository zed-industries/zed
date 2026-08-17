//! Elliptic curve arithmetic traits.

use crate::{
    ops::LinearCombination, AffineXCoordinate, Curve, FieldBytes, IsHigh, PrimeCurve, ScalarCore,
};
use core::fmt::Debug;
use subtle::{ConditionallySelectable, ConstantTimeEq};
use zeroize::DefaultIsZeroes;

/// Elliptic curve with affine arithmetic implementation.
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
pub trait AffineArithmetic: Curve + ScalarArithmetic {
    /// Elliptic curve point in affine coordinates.
    type AffinePoint: 'static
        + AffineXCoordinate<Self>
        + Copy
        + Clone
        + ConditionallySelectable
        + ConstantTimeEq
        + Debug
        + Default
        + DefaultIsZeroes
        + Eq
        + PartialEq
        + Sized
        + Send
        + Sync;
}

/// Prime order elliptic curve with projective arithmetic implementation.
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
pub trait PrimeCurveArithmetic:
    PrimeCurve + ProjectiveArithmetic<ProjectivePoint = Self::CurveGroup>
{
    /// Prime order elliptic curve group.
    type CurveGroup: group::prime::PrimeCurve<Affine = <Self as AffineArithmetic>::AffinePoint>;
}

/// Elliptic curve with projective arithmetic implementation.
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
pub trait ProjectiveArithmetic: Curve + AffineArithmetic {
    /// Elliptic curve point in projective coordinates.
    ///
    /// Note: the following bounds are provided by [`group::Group`]:
    /// - `'static`
    /// - [`Copy`]
    /// - [`Clone`]
    /// - [`Debug`]
    /// - [`Eq`]
    /// - [`Sized`]
    /// - [`Send`]
    /// - [`Sync`]
    type ProjectivePoint: ConditionallySelectable
        + ConstantTimeEq
        + Default
        + DefaultIsZeroes
        + From<Self::AffinePoint>
        + Into<Self::AffinePoint>
        + LinearCombination
        + group::Curve<AffineRepr = Self::AffinePoint>
        + group::Group<Scalar = Self::Scalar>;
}

/// Scalar arithmetic.
#[cfg(feature = "arithmetic")]
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
pub trait ScalarArithmetic: Curve {
    /// Scalar field type.
    ///
    /// Note: the following bounds are provided by [`ff::Field`]:
    /// - `'static`
    /// - [`Copy`]
    /// - [`Clone`]
    /// - [`ConditionallySelectable`]
    /// - [`ConstantTimeEq`]
    /// - [`Debug`]
    /// - [`Default`]
    /// - [`Send`]
    /// - [`Sync`]
    type Scalar: DefaultIsZeroes
        + From<ScalarCore<Self>>
        + Into<FieldBytes<Self>>
        + Into<Self::UInt>
        + IsHigh
        + ff::Field
        + ff::PrimeField<Repr = FieldBytes<Self>>;
}
