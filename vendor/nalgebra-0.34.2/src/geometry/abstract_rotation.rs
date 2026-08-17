use crate::geometry::{Rotation, UnitComplex, UnitQuaternion};
use crate::{Const, OVector, Point, SVector, Scalar, SimdRealField, Unit};

use simba::scalar::ClosedMulAssign;

/// Trait implemented by rotations that can be used inside of an `Isometry` or `Similarity`.
pub trait AbstractRotation<T: Scalar, const D: usize>: PartialEq + ClosedMulAssign + Clone {
    /// The rotation identity.
    fn identity() -> Self;
    /// The rotation inverse.
    fn inverse(&self) -> Self;
    /// Change `self` to its inverse.
    fn inverse_mut(&mut self);
    /// Apply the rotation to the given vector.
    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D>;
    /// Apply the rotation to the given point.
    fn transform_point(&self, p: &Point<T, D>) -> Point<T, D>;
    /// Apply the inverse rotation to the given vector.
    fn inverse_transform_vector(&self, v: &OVector<T, Const<D>>) -> OVector<T, Const<D>>;
    /// Apply the inverse rotation to the given unit vector.
    fn inverse_transform_unit_vector(&self, v: &Unit<SVector<T, D>>) -> Unit<SVector<T, D>> {
        Unit::new_unchecked(self.inverse_transform_vector(&**v))
    }
    /// Apply the inverse rotation to the given point.
    fn inverse_transform_point(&self, p: &Point<T, D>) -> Point<T, D>;
}

impl<T: SimdRealField, const D: usize> AbstractRotation<T, D> for Rotation<T, D>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }

    #[inline]
    fn inverse(&self) -> Self {
        self.inverse()
    }

    #[inline]
    fn inverse_mut(&mut self) {
        self.inverse_mut()
    }

    #[inline]
    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self * v
    }

    #[inline]
    fn transform_point(&self, p: &Point<T, D>) -> Point<T, D> {
        self * p
    }

    #[inline]
    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.inverse_transform_vector(v)
    }

    #[inline]
    fn inverse_transform_unit_vector(&self, v: &Unit<SVector<T, D>>) -> Unit<SVector<T, D>> {
        self.inverse_transform_unit_vector(v)
    }

    #[inline]
    fn inverse_transform_point(&self, p: &Point<T, D>) -> Point<T, D> {
        self.inverse_transform_point(p)
    }
}

impl<T: SimdRealField> AbstractRotation<T, 3> for UnitQuaternion<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }

    #[inline]
    fn inverse(&self) -> Self {
        self.inverse()
    }

    #[inline]
    fn inverse_mut(&mut self) {
        self.inverse_mut()
    }

    #[inline]
    fn transform_vector(&self, v: &SVector<T, 3>) -> SVector<T, 3> {
        self * v
    }

    #[inline]
    fn transform_point(&self, p: &Point<T, 3>) -> Point<T, 3> {
        self * p
    }

    #[inline]
    fn inverse_transform_vector(&self, v: &SVector<T, 3>) -> SVector<T, 3> {
        self.inverse_transform_vector(v)
    }

    #[inline]
    fn inverse_transform_point(&self, p: &Point<T, 3>) -> Point<T, 3> {
        self.inverse_transform_point(p)
    }
}

impl<T: SimdRealField> AbstractRotation<T, 2> for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }

    #[inline]
    fn inverse(&self) -> Self {
        self.inverse()
    }

    #[inline]
    fn inverse_mut(&mut self) {
        self.inverse_mut()
    }

    #[inline]
    fn transform_vector(&self, v: &SVector<T, 2>) -> SVector<T, 2> {
        self * v
    }

    #[inline]
    fn transform_point(&self, p: &Point<T, 2>) -> Point<T, 2> {
        self * p
    }

    #[inline]
    fn inverse_transform_vector(&self, v: &SVector<T, 2>) -> SVector<T, 2> {
        self.inverse_transform_vector(v)
    }

    #[inline]
    fn inverse_transform_point(&self, p: &Point<T, 2>) -> Point<T, 2> {
        self.inverse_transform_point(p)
    }
}
