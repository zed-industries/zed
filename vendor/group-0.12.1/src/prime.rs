use core::fmt;
use core::ops::{Mul, Neg};
use ff::PrimeField;
use subtle::Choice;

use crate::{Curve, Group, GroupEncoding};

/// This trait represents an element of a prime-order cryptographic group.
pub trait PrimeGroup: Group + GroupEncoding {}

/// Efficient representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait PrimeCurve: Curve<AffineRepr = <Self as PrimeCurve>::Affine> + PrimeGroup {
    type Affine: PrimeCurveAffine<Curve = Self, Scalar = Self::Scalar>
        + Mul<Self::Scalar, Output = Self>
        + for<'r> Mul<&'r Self::Scalar, Output = Self>;
}

/// Affine representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait PrimeCurveAffine: GroupEncoding
    + Copy
    + Clone
    + Sized
    + Send
    + Sync
    + fmt::Debug
    + PartialEq
    + Eq
    + 'static
    + Neg<Output = Self>
    + Mul<<Self as PrimeCurveAffine>::Scalar, Output = <Self as PrimeCurveAffine>::Curve>
    + for<'r> Mul<&'r <Self as PrimeCurveAffine>::Scalar, Output = <Self as PrimeCurveAffine>::Curve>
{
    type Scalar: PrimeField;
    type Curve: PrimeCurve<Affine = Self, Scalar = Self::Scalar>;

    /// Returns the additive identity.
    fn identity() -> Self;

    /// Returns a fixed generator of unknown exponent.
    fn generator() -> Self;

    /// Determines if this point represents the point at infinity; the
    /// additive identity.
    fn is_identity(&self) -> Choice;

    /// Converts this element to its curve representation.
    fn to_curve(&self) -> Self::Curve;
}
