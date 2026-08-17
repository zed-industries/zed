use alga::general::{
    AbstractGroup, AbstractLoop, AbstractMagma, AbstractMonoid, AbstractQuasigroup,
    AbstractSemigroup, Identity, Multiplicative, RealField, TwoSidedInverse,
};
use alga::linear::{ProjectiveTransformation, Transformation};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, SVector};

use crate::geometry::{Point, SubTCategoryOf, TCategory, TProjective, Transform};

/*
 *
 * Algebraic structures.
 *
 */
impl<T: RealField + simba::scalar::RealField, C, const D: usize> Identity<Multiplicative>
    for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    C: TCategory,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn identity() -> Self {
        Self::identity()
    }
}

impl<T: RealField + simba::scalar::RealField, C, const D: usize> TwoSidedInverse<Multiplicative>
    for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    C: SubTCategoryOf<TProjective>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        self.clone().inverse()
    }

    #[inline]
    fn two_sided_inverse_mut(&mut self) {
        self.inverse_mut()
    }
}

impl<T: RealField + simba::scalar::RealField, C, const D: usize> AbstractMagma<Multiplicative>
    for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    C: TCategory,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn operate(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

macro_rules! impl_multiplicative_structures(
    ($($marker: ident<$operator: ident>),* $(,)*) => {$(
        impl<T: RealField + simba::scalar::RealField, C, const D: usize> $marker<$operator> for Transform<T, C, D>
            where
                  Const<D>: DimNameAdd<U1>,
                  C: TCategory,
                  DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> { }
    )*}
);

macro_rules! impl_inversible_multiplicative_structures(
    ($($marker: ident<$operator: ident>),* $(,)*) => {$(
        impl<T: RealField + simba::scalar::RealField, C, const D: usize> $marker<$operator> for Transform<T, C, D>
            where
                  Const<D>: DimNameAdd<U1>,
                  C: SubTCategoryOf<TProjective>,
                  DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> { }
    )*}
);

impl_multiplicative_structures!(
    AbstractSemigroup<Multiplicative>,
    AbstractMonoid<Multiplicative>,
);

impl_inversible_multiplicative_structures!(
    AbstractQuasigroup<Multiplicative>,
    AbstractLoop<Multiplicative>,
    AbstractGroup<Multiplicative>
);

/*
 *
 * Transformation groups.
 *
 */
impl<T, C, const D: usize> Transformation<Point<T, D>> for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    T: RealField + simba::scalar::RealField,
    C: TCategory,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.transform_point(pt)
    }

    #[inline]
    fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.transform_vector(v)
    }
}

impl<T, C, const D: usize> ProjectiveTransformation<Point<T, D>> for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    T: RealField + simba::scalar::RealField,
    C: SubTCategoryOf<TProjective>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.inverse_transform_point(pt)
    }

    #[inline]
    fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.inverse_transform_vector(v)
    }
}

// TODO: we need to implement an SVD for this.
//
// impl<T, C, const D: usize> AffineTransformation<Point<T, D>> for Transform<T, C, D>
//     where T:  RealField,
//           C: SubTCategoryOf<TAffine>,
//           DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
//     type PreRotation       = Rotation<T, D>;
//     type NonUniformScaling = OVector<T, D>;
//     type PostRotation      = Rotation<T, D>;
//     type Translation       = Translation<T, D>;
//
//     #[inline]
//     fn decompose(&self) -> (Self::Translation, Self::PostRotation, Self::NonUniformScaling, Self::PreRotation) {
//         unimplemented!()
//     }
// }
