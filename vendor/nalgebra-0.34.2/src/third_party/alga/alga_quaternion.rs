use num::Zero;

use alga::general::{
    AbstractGroup, AbstractGroupAbelian, AbstractLoop, AbstractMagma, AbstractModule,
    AbstractMonoid, AbstractQuasigroup, AbstractSemigroup, Additive, Id, Identity, Module,
    Multiplicative, RealField, TwoSidedInverse,
};
use alga::linear::{
    AffineTransformation, DirectIsometry, FiniteDimVectorSpace, Isometry, NormedSpace,
    OrthogonalTransformation, ProjectiveTransformation, Rotation, Similarity, Transformation,
    VectorSpace,
};

use crate::base::{Vector3, Vector4};
use crate::geometry::{Point3, Quaternion, UnitQuaternion};

impl<T: RealField + simba::scalar::RealField> Identity<Multiplicative> for Quaternion<T> {
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField> Identity<Additive> for Quaternion<T> {
    #[inline]
    fn identity() -> Self {
        Self::zero()
    }
}

impl<T: RealField + simba::scalar::RealField> AbstractMagma<Multiplicative> for Quaternion<T> {
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl<T: RealField + simba::scalar::RealField> AbstractMagma<Additive> for Quaternion<T> {
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self + rhs
    }
}

impl<T: RealField + simba::scalar::RealField> TwoSidedInverse<Additive> for Quaternion<T> {
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        -self
    }
}

macro_rules! impl_structures(
    ($Quaternion: ident; $($marker: ident<$operator: ident>),* $(,)*) => {$(
        impl<T: RealField + simba::scalar::RealField> $marker<$operator> for $Quaternion<T> { }
    )*}
);

impl_structures!(
    Quaternion;
    AbstractSemigroup<Multiplicative>,
    AbstractMonoid<Multiplicative>,

    AbstractSemigroup<Additive>,
    AbstractQuasigroup<Additive>,
    AbstractMonoid<Additive>,
    AbstractLoop<Additive>,
    AbstractGroup<Additive>,
    AbstractGroupAbelian<Additive>
);

/*
 *
 * Vector space.
 *
 */
impl<T: RealField + simba::scalar::RealField> AbstractModule for Quaternion<T> {
    type AbstractRing = T;

    #[inline]
    fn multiply_by(&self, n: T) -> Self {
        self * n
    }
}

impl<T: RealField + simba::scalar::RealField> Module for Quaternion<T> {
    type Ring = T;
}

impl<T: RealField + simba::scalar::RealField> VectorSpace for Quaternion<T> {
    type Field = T;
}

impl<T: RealField + simba::scalar::RealField> FiniteDimVectorSpace for Quaternion<T> {
    #[inline]
    fn dimension() -> usize {
        4
    }

    #[inline]
    fn canonical_basis_element(i: usize) -> Self {
        Self::from(Vector4::canonical_basis_element(i))
    }

    #[inline]
    fn dot(&self, other: &Self) -> T {
        self.coords.dot(&other.coords)
    }

    #[inline]
    unsafe fn component_unchecked(&self, i: usize) -> &T {
        unsafe { self.coords.component_unchecked(i) }
    }

    #[inline]
    unsafe fn component_unchecked_mut(&mut self, i: usize) -> &mut T {
        unsafe { self.coords.component_unchecked_mut(i) }
    }
}

impl<T: RealField + simba::scalar::RealField> NormedSpace for Quaternion<T> {
    type RealField = T;
    type ComplexField = T;

    #[inline]
    fn norm_squared(&self) -> T {
        self.coords.norm_squared()
    }

    #[inline]
    fn norm(&self) -> T {
        self.as_vector().norm()
    }

    #[inline]
    fn normalize(&self) -> Self {
        let v = self.coords.normalize();
        Self::from(v)
    }

    #[inline]
    fn normalize_mut(&mut self) -> T {
        self.coords.normalize_mut()
    }

    #[inline]
    fn try_normalize(&self, min_norm: T) -> Option<Self> {
        self.coords.try_normalize(min_norm).map(Self::from)
    }

    #[inline]
    fn try_normalize_mut(&mut self, min_norm: T) -> Option<T> {
        self.coords.try_normalize_mut(min_norm)
    }
}

/*
 *
 * Implementations for UnitQuaternion.
 *
 */
impl<T: RealField + simba::scalar::RealField> Identity<Multiplicative> for UnitQuaternion<T> {
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField> AbstractMagma<Multiplicative> for UnitQuaternion<T> {
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl<T: RealField + simba::scalar::RealField> TwoSidedInverse<Multiplicative>
    for UnitQuaternion<T>
{
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        self.inverse()
    }

    #[inline]
    fn two_sided_inverse_mut(&mut self) {
        self.inverse_mut()
    }
}

impl_structures!(
    UnitQuaternion;
    AbstractSemigroup<Multiplicative>,
    AbstractQuasigroup<Multiplicative>,
    AbstractMonoid<Multiplicative>,
    AbstractLoop<Multiplicative>,
    AbstractGroup<Multiplicative>
);

impl<T: RealField + simba::scalar::RealField> Transformation<Point3<T>> for UnitQuaternion<T> {
    #[inline]
    fn transform_point(&self, pt: &Point3<T>) -> Point3<T> {
        self.transform_point(pt)
    }

    #[inline]
    fn transform_vector(&self, v: &Vector3<T>) -> Vector3<T> {
        self.transform_vector(v)
    }
}

impl<T: RealField + simba::scalar::RealField> ProjectiveTransformation<Point3<T>>
    for UnitQuaternion<T>
{
    #[inline]
    fn inverse_transform_point(&self, pt: &Point3<T>) -> Point3<T> {
        self.inverse_transform_point(pt)
    }

    #[inline]
    fn inverse_transform_vector(&self, v: &Vector3<T>) -> Vector3<T> {
        self.inverse_transform_vector(v)
    }
}

impl<T: RealField + simba::scalar::RealField> AffineTransformation<Point3<T>>
    for UnitQuaternion<T>
{
    type Rotation = Self;
    type NonUniformScaling = Id;
    type Translation = Id;

    #[inline]
    fn decompose(&self) -> (Id, Self, Id, Self) {
        (Id::new(), *self, Id::new(), Self::identity())
    }

    #[inline]
    fn append_translation(&self, _: &Self::Translation) -> Self {
        *self
    }

    #[inline]
    fn prepend_translation(&self, _: &Self::Translation) -> Self {
        *self
    }

    #[inline]
    fn append_rotation(&self, r: &Self::Rotation) -> Self {
        r * self
    }

    #[inline]
    fn prepend_rotation(&self, r: &Self::Rotation) -> Self {
        self * r
    }

    #[inline]
    fn append_scaling(&self, _: &Self::NonUniformScaling) -> Self {
        *self
    }

    #[inline]
    fn prepend_scaling(&self, _: &Self::NonUniformScaling) -> Self {
        *self
    }
}

impl<T: RealField + simba::scalar::RealField> Similarity<Point3<T>> for UnitQuaternion<T> {
    type Scaling = Id;

    #[inline]
    fn translation(&self) -> Id {
        Id::new()
    }

    #[inline]
    fn rotation(&self) -> Self {
        *self
    }

    #[inline]
    fn scaling(&self) -> Id {
        Id::new()
    }
}

macro_rules! marker_impl(
    ($($Trait: ident),*) => {$(
        impl<T: RealField + simba::scalar::RealField> $Trait<Point3<T>> for UnitQuaternion<T> { }
    )*}
);

marker_impl!(Isometry, DirectIsometry, OrthogonalTransformation);

impl<T: RealField + simba::scalar::RealField> Rotation<Point3<T>> for UnitQuaternion<T> {
    #[inline]
    fn powf(&self, n: T) -> Option<Self> {
        Some(self.powf(n))
    }

    #[inline]
    fn rotation_between(a: &Vector3<T>, b: &Vector3<T>) -> Option<Self> {
        Self::rotation_between(a, b)
    }

    #[inline]
    fn scaled_rotation_between(a: &Vector3<T>, b: &Vector3<T>, s: T) -> Option<Self> {
        Self::scaled_rotation_between(a, b, s)
    }
}
