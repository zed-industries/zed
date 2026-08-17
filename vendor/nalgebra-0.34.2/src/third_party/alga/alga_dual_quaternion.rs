use num::Zero;

use alga::general::{
    AbstractGroup, AbstractGroupAbelian, AbstractLoop, AbstractMagma, AbstractModule,
    AbstractMonoid, AbstractQuasigroup, AbstractSemigroup, Additive, Id, Identity, Module,
    Multiplicative, RealField, TwoSidedInverse,
};
use alga::linear::{
    AffineTransformation, DirectIsometry, FiniteDimVectorSpace, Isometry, NormedSpace,
    ProjectiveTransformation, Similarity, Transformation, VectorSpace,
};

use crate::base::Vector3;
use crate::geometry::{
    DualQuaternion, Point3, Quaternion, Translation3, UnitDualQuaternion, UnitQuaternion,
};

impl<T: RealField + simba::scalar::RealField> Identity<Multiplicative> for DualQuaternion<T> {
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField> Identity<Additive> for DualQuaternion<T> {
    #[inline]
    fn identity() -> Self {
        Self::zero()
    }
}

impl<T: RealField + simba::scalar::RealField> AbstractMagma<Multiplicative> for DualQuaternion<T> {
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl<T: RealField + simba::scalar::RealField> AbstractMagma<Additive> for DualQuaternion<T> {
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self + rhs
    }
}

impl<T: RealField + simba::scalar::RealField> TwoSidedInverse<Additive> for DualQuaternion<T> {
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        -self
    }
}

macro_rules! impl_structures(
    ($DualQuaternion: ident; $($marker: ident<$operator: ident>),* $(,)*) => {$(
        impl<T: RealField + simba::scalar::RealField> $marker<$operator> for $DualQuaternion<T> { }
    )*}
);

impl_structures!(
    DualQuaternion;
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
impl<T: RealField + simba::scalar::RealField> AbstractModule for DualQuaternion<T> {
    type AbstractRing = T;

    #[inline]
    fn multiply_by(&self, n: T) -> Self {
        self * n
    }
}

impl<T: RealField + simba::scalar::RealField> Module for DualQuaternion<T> {
    type Ring = T;
}

impl<T: RealField + simba::scalar::RealField> VectorSpace for DualQuaternion<T> {
    type Field = T;
}

impl<T: RealField + simba::scalar::RealField> FiniteDimVectorSpace for DualQuaternion<T> {
    #[inline]
    fn dimension() -> usize {
        8
    }

    #[inline]
    fn canonical_basis_element(i: usize) -> Self {
        if i < 4 {
            DualQuaternion::from_real_and_dual(
                Quaternion::canonical_basis_element(i),
                Quaternion::zero(),
            )
        } else {
            DualQuaternion::from_real_and_dual(
                Quaternion::zero(),
                Quaternion::canonical_basis_element(i - 4),
            )
        }
    }

    #[inline]
    fn dot(&self, other: &Self) -> T {
        self.real.dot(&other.real) + self.dual.dot(&other.dual)
    }

    #[inline]
    unsafe fn component_unchecked(&self, i: usize) -> &T {
        unsafe { self.as_ref().get_unchecked(i) }
    }

    #[inline]
    unsafe fn component_unchecked_mut(&mut self, i: usize) -> &mut T {
        unsafe { self.as_mut().get_unchecked_mut(i) }
    }
}

impl<T: RealField + simba::scalar::RealField> NormedSpace for DualQuaternion<T> {
    type RealField = T;
    type ComplexField = T;

    #[inline]
    fn norm_squared(&self) -> T {
        self.real.norm_squared()
    }

    #[inline]
    fn norm(&self) -> T {
        self.real.norm()
    }

    #[inline]
    fn normalize(&self) -> Self {
        self.normalize()
    }

    #[inline]
    fn normalize_mut(&mut self) -> T {
        self.normalize_mut()
    }

    #[inline]
    fn try_normalize(&self, min_norm: T) -> Option<Self> {
        let real_norm = self.real.norm();
        if real_norm > min_norm {
            Some(Self::from_real_and_dual(
                self.real / real_norm,
                self.dual / real_norm,
            ))
        } else {
            None
        }
    }

    #[inline]
    fn try_normalize_mut(&mut self, min_norm: T) -> Option<T> {
        let real_norm = self.real.norm();
        if real_norm > min_norm {
            self.real /= real_norm;
            self.dual /= real_norm;
            Some(real_norm)
        } else {
            None
        }
    }
}

/*
 *
 * Implementations for UnitDualQuaternion.
 *
 */
impl<T: RealField + simba::scalar::RealField> Identity<Multiplicative> for UnitDualQuaternion<T> {
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField> AbstractMagma<Multiplicative>
    for UnitDualQuaternion<T>
{
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl<T: RealField + simba::scalar::RealField> TwoSidedInverse<Multiplicative>
    for UnitDualQuaternion<T>
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
    UnitDualQuaternion;
    AbstractSemigroup<Multiplicative>,
    AbstractQuasigroup<Multiplicative>,
    AbstractMonoid<Multiplicative>,
    AbstractLoop<Multiplicative>,
    AbstractGroup<Multiplicative>
);

impl<T: RealField + simba::scalar::RealField> Transformation<Point3<T>> for UnitDualQuaternion<T> {
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
    for UnitDualQuaternion<T>
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
    for UnitDualQuaternion<T>
{
    type Rotation = UnitQuaternion<T>;
    type NonUniformScaling = Id;
    type Translation = Translation3<T>;

    #[inline]
    fn decompose(&self) -> (Self::Translation, Self::Rotation, Id, Self::Rotation) {
        (
            self.translation(),
            self.rotation(),
            Id::new(),
            UnitQuaternion::identity(),
        )
    }

    #[inline]
    fn append_translation(&self, translation: &Self::Translation) -> Self {
        self * Self::from_parts(*translation, UnitQuaternion::identity())
    }

    #[inline]
    fn prepend_translation(&self, translation: &Self::Translation) -> Self {
        Self::from_parts(*translation, UnitQuaternion::identity()) * self
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

impl<T: RealField + simba::scalar::RealField> Similarity<Point3<T>> for UnitDualQuaternion<T> {
    type Scaling = Id;

    #[inline]
    fn translation(&self) -> Translation3<T> {
        self.translation()
    }

    #[inline]
    fn rotation(&self) -> UnitQuaternion<T> {
        self.rotation()
    }

    #[inline]
    fn scaling(&self) -> Id {
        Id::new()
    }
}

macro_rules! marker_impl(
    ($($Trait: ident),*) => {$(
        impl<T: RealField + simba::scalar::RealField> $Trait<Point3<T>> for UnitDualQuaternion<T> { }
    )*}
);

marker_impl!(Isometry, DirectIsometry);
