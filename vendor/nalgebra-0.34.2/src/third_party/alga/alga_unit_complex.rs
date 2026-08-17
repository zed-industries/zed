use alga::general::{
    AbstractGroup, AbstractLoop, AbstractMagma, AbstractMonoid, AbstractQuasigroup,
    AbstractSemigroup, Id, Identity, Multiplicative, RealField, TwoSidedInverse,
};
use alga::linear::{
    AffineTransformation, DirectIsometry, Isometry, OrthogonalTransformation,
    ProjectiveTransformation, Rotation, Similarity, Transformation,
};

use crate::base::Vector2;
use crate::geometry::{Point2, UnitComplex};

/*
 *
 * Implementations for UnitComplex.
 *
 */
impl<T: RealField + simba::scalar::RealField> Identity<Multiplicative> for UnitComplex<T> {
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField> AbstractMagma<Multiplicative> for UnitComplex<T> {
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl<T: RealField + simba::scalar::RealField> TwoSidedInverse<Multiplicative> for UnitComplex<T> {
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        self.inverse()
    }

    #[inline]
    fn two_sided_inverse_mut(&mut self) {
        self.inverse_mut()
    }
}

macro_rules! impl_structures(
    ($($marker: ident<$operator: ident>),* $(,)*) => {$(
        impl<T: RealField + simba::scalar::RealField> $marker<$operator> for UnitComplex<T> {
        }
    )*}
);

impl_structures!(
    AbstractSemigroup<Multiplicative>,
    AbstractQuasigroup<Multiplicative>,
    AbstractMonoid<Multiplicative>,
    AbstractLoop<Multiplicative>,
    AbstractGroup<Multiplicative>
);

impl<T: RealField + simba::scalar::RealField> Transformation<Point2<T>> for UnitComplex<T> {
    #[inline]
    fn transform_point(&self, pt: &Point2<T>) -> Point2<T> {
        self.transform_point(pt)
    }

    #[inline]
    fn transform_vector(&self, v: &Vector2<T>) -> Vector2<T> {
        self.transform_vector(v)
    }
}

impl<T: RealField + simba::scalar::RealField> ProjectiveTransformation<Point2<T>>
    for UnitComplex<T>
{
    #[inline]
    fn inverse_transform_point(&self, pt: &Point2<T>) -> Point2<T> {
        self.inverse_transform_point(pt)
    }

    #[inline]
    fn inverse_transform_vector(&self, v: &Vector2<T>) -> Vector2<T> {
        self.inverse_transform_vector(v)
    }
}

impl<T: RealField + simba::scalar::RealField> AffineTransformation<Point2<T>> for UnitComplex<T> {
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

impl<T: RealField + simba::scalar::RealField> Similarity<Point2<T>> for UnitComplex<T> {
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
        impl<T: RealField + simba::scalar::RealField> $Trait<Point2<T>> for UnitComplex<T>
        { }
    )*}
);

marker_impl!(Isometry, DirectIsometry, OrthogonalTransformation);

impl<T: RealField + simba::scalar::RealField> Rotation<Point2<T>> for UnitComplex<T> {
    #[inline]
    fn powf(&self, n: T) -> Option<Self> {
        Some(self.powf(n))
    }

    #[inline]
    fn rotation_between(a: &Vector2<T>, b: &Vector2<T>) -> Option<Self> {
        Some(Self::rotation_between(a, b))
    }

    #[inline]
    fn scaled_rotation_between(a: &Vector2<T>, b: &Vector2<T>, s: T) -> Option<Self> {
        Some(Self::scaled_rotation_between(a, b, s))
    }
}
