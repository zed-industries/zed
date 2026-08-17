use core::fmt;
use core::ops::{Mul, Neg};
use ff::PrimeField;
use subtle::{Choice, CtOption};

use crate::{prime::PrimeGroup, Curve, Group, GroupEncoding, GroupOps, GroupOpsOwned};

/// This trait represents an element of a cryptographic group with a large prime-order
/// subgroup and a comparatively-small cofactor.
pub trait CofactorGroup:
    Group
    + GroupEncoding
    + GroupOps<<Self as CofactorGroup>::Subgroup>
    + GroupOpsOwned<<Self as CofactorGroup>::Subgroup>
{
    /// The large prime-order subgroup in which cryptographic operations are performed.
    /// If `Self` implements `PrimeGroup`, then `Self::Subgroup` may be `Self`.
    type Subgroup: PrimeGroup<Scalar = Self::Scalar> + Into<Self>;

    /// Maps `self` to the prime-order subgroup by multiplying this element by some
    /// `k`-multiple of the cofactor.
    ///
    /// The value `k` does not vary between inputs for a given implementation, but may
    /// vary between different implementations of `CofactorGroup` because some groups have
    /// more efficient methods of clearing the cofactor when `k` is allowed to be
    /// different than `1`.
    ///
    /// If `Self` implements [`PrimeGroup`], this returns `self`.
    fn clear_cofactor(&self) -> Self::Subgroup;

    /// Returns `self` if it is contained in the prime-order subgroup.
    ///
    /// If `Self` implements [`PrimeGroup`], this returns `Some(self)`.
    fn into_subgroup(self) -> CtOption<Self::Subgroup>;

    /// Determines if this element is of small order.
    ///
    /// Returns:
    /// - `true` if `self` is in the torsion subgroup.
    /// - `false` if `self` is not in the torsion subgroup.
    fn is_small_order(&self) -> Choice {
        self.clear_cofactor().is_identity()
    }

    /// Determines if this element is "torsion free", i.e., is contained in the
    /// prime-order subgroup.
    ///
    /// Returns:
    /// - `true` if `self` has trivial torsion and is in the prime-order subgroup.
    /// - `false` if `self` has non-zero torsion component and is not in the prime-order
    ///   subgroup.
    fn is_torsion_free(&self) -> Choice;
}

/// Efficient representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait CofactorCurve:
    Curve<AffineRepr = <Self as CofactorCurve>::Affine> + CofactorGroup
{
    type Affine: CofactorCurveAffine<Curve = Self, Scalar = Self::Scalar>
        + Mul<Self::Scalar, Output = Self>
        + for<'r> Mul<&'r Self::Scalar, Output = Self>;
}

/// Affine representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait CofactorCurveAffine:
    GroupEncoding
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
    + Mul<<Self as CofactorCurveAffine>::Scalar, Output = <Self as CofactorCurveAffine>::Curve>
    + for<'r> Mul<
        &'r <Self as CofactorCurveAffine>::Scalar,
        Output = <Self as CofactorCurveAffine>::Curve,
    >
{
    type Scalar: PrimeField;
    type Curve: CofactorCurve<Affine = Self, Scalar = Self::Scalar>;

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
