use alga::general::{
    AbstractGroup, AbstractLoop, AbstractMagma, AbstractMonoid, AbstractQuasigroup,
    AbstractSemigroup, Id, Identity, Multiplicative, RealField, TwoSidedInverse,
};
use alga::linear::Translation as AlgaTranslation;
use alga::linear::{
    AffineTransformation, DirectIsometry, Isometry, ProjectiveTransformation, Similarity,
    Transformation,
};

use crate::base::SVector;
use crate::geometry::{Point, Translation};

/*
 *
 * Algebraic structures.
 *
 */
impl<T: RealField + simba::scalar::RealField, const D: usize> Identity<Multiplicative>
    for Translation<T, D>
{
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField, const D: usize> TwoSidedInverse<Multiplicative>
    for Translation<T, D>
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

impl<T: RealField + simba::scalar::RealField, const D: usize> AbstractMagma<Multiplicative>
    for Translation<T, D>
{
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

macro_rules! impl_multiplicative_structures(
    ($($marker: ident<$operator: ident>),* $(,)*) => {$(
        impl<T: RealField + simba::scalar::RealField, const D: usize> $marker<$operator> for Translation<T, D>
            { }
    )*}
);

impl_multiplicative_structures!(
    AbstractSemigroup<Multiplicative>,
    AbstractMonoid<Multiplicative>,
    AbstractQuasigroup<Multiplicative>,
    AbstractLoop<Multiplicative>,
    AbstractGroup<Multiplicative>
);

/*
 *
 * Transformation groups.
 *
 */
impl<T: RealField + simba::scalar::RealField, const D: usize> Transformation<Point<T, D>>
    for Translation<T, D>
{
    #[inline]
    fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.transform_point(pt)
    }

    #[inline]
    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        *v
    }
}

impl<T: RealField + simba::scalar::RealField, const D: usize> ProjectiveTransformation<Point<T, D>>
    for Translation<T, D>
{
    #[inline]
    fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.inverse_transform_point(pt)
    }

    #[inline]
    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        *v
    }
}

impl<T: RealField + simba::scalar::RealField, const D: usize> AffineTransformation<Point<T, D>>
    for Translation<T, D>
{
    type Rotation = Id;
    type NonUniformScaling = Id;
    type Translation = Self;

    #[inline]
    fn decompose(&self) -> (Self, Id, Id, Id) {
        (*self, Id::new(), Id::new(), Id::new())
    }

    #[inline]
    fn append_translation(&self, t: &Self::Translation) -> Self {
        t * self
    }

    #[inline]
    fn prepend_translation(&self, t: &Self::Translation) -> Self {
        self * t
    }

    #[inline]
    fn append_rotation(&self, _: &Self::Rotation) -> Self {
        *self
    }

    #[inline]
    fn prepend_rotation(&self, _: &Self::Rotation) -> Self {
        *self
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

impl<T: RealField + simba::scalar::RealField, const D: usize> Similarity<Point<T, D>>
    for Translation<T, D>
{
    type Scaling = Id;

    #[inline]
    fn translation(&self) -> Self {
        *self
    }

    #[inline]
    fn rotation(&self) -> Id {
        Id::new()
    }

    #[inline]
    fn scaling(&self) -> Id {
        Id::new()
    }
}

macro_rules! marker_impl(
    ($($Trait: ident),*) => {$(
        impl<T: RealField + simba::scalar::RealField, const D: usize> $Trait<Point<T, D>> for Translation<T, D>
            { }
    )*}
);

marker_impl!(Isometry, DirectIsometry);

/// Subgroups of the n-dimensional translation group `T(n)`.
impl<T: RealField + simba::scalar::RealField, const D: usize> AlgaTranslation<Point<T, D>>
    for Translation<T, D>
{
    #[inline]
    fn to_vector(&self) -> SVector<T, D> {
        self.vector
    }

    #[inline]
    fn from_vector(v: SVector<T, D>) -> Option<Self> {
        Some(Self::from(v))
    }

    #[inline]
    fn powf(&self, n: T) -> Option<Self> {
        Some(Self::from(self.vector * n))
    }

    #[inline]
    fn translation_between(a: &Point<T, D>, b: &Point<T, D>) -> Option<Self> {
        Some(Self::from(b - a))
    }
}
