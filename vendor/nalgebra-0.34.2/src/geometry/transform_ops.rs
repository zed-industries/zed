// The macros break if the references are taken out, for some reason.
#![allow(clippy::op_ref)]

use num::{One, Zero};
use std::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign};

use simba::scalar::{ClosedAddAssign, ClosedMulAssign, RealField, SubsetOf};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, OMatrix, SVector, Scalar};

use crate::geometry::{
    Isometry, Point, Rotation, Similarity, SubTCategoryOf, SuperTCategoryOf, TAffine, TCategory,
    TCategoryMul, TGeneral, TProjective, Transform, Translation, UnitComplex, UnitQuaternion,
};

/*
 *
 * In the following, we provide:
 * =========================
 *
 * Index<(usize, usize)>
 * IndexMut<(usize, usize)> (where TCategory == TGeneral)
 *
 * (Operators)
 *
 * Transform × Isometry
 * Transform × Rotation
 * Transform × Similarity
 * Transform × Transform
 * Transform × UnitQuaternion
 * Transform × UnitComplex
 * Transform × Translation
 * Transform × Vector
 * Transform × Point
 *
 * Isometry       × Transform
 * Rotation       × Transform
 * Similarity     × Transform
 * Translation    × Transform
 * UnitQuaternion × Transform
 * UnitComplex    × Transform
 *
 * TODO: Transform ÷ Isometry
 * Transform ÷ Rotation
 * TODO: Transform ÷ Similarity
 * Transform ÷ Transform
 * Transform ÷ UnitQuaternion
 * Transform ÷ Translation
 *
 * TODO: Isometry       ÷ Transform
 * Rotation       ÷ Transform
 * TODO: Similarity     ÷ Transform
 * Translation    ÷ Transform
 * UnitQuaternion ÷ Transform
 * TODO: UnitComplex ÷ Transform
 *
 *
 * (Assignment Operators)
 *
 *
 * Transform ×= Transform
 * Transform ×= Similarity
 * Transform ×= Isometry
 * Transform ×= Rotation
 * Transform ×= UnitQuaternion
 * Transform ×= UnitComplex
 * Transform ×= Translation
 *
 * Transform ÷= Transform
 * TODO: Transform ÷= Similarity
 * TODO: Transform ÷= Isometry
 * Transform ÷= Rotation
 * Transform ÷= UnitQuaternion
 * Transform ÷= UnitComplex
 *
 */

/*
 *
 * Indexing.
 *
 */
impl<T: RealField, C: TCategory, const D: usize> Index<(usize, usize)> for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    type Output = T;

    #[inline]
    fn index(&self, ij: (usize, usize)) -> &T {
        self.matrix().index(ij)
    }
}

// Only general transformations are mutably indexable.
impl<T: RealField, const D: usize> IndexMut<(usize, usize)> for Transform<T, TGeneral, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn index_mut(&mut self, ij: (usize, usize)) -> &mut T {
        self.matrix_mut().index_mut(ij)
    }
}

// Transform × Vector
md_impl_all!(
    Mul, mul where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategory,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: SVector<T, D>, Output = SVector<T, D>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] => {
        let transform = self.matrix().fixed_view::<D, D>(0, 0);

        if C::has_normalizer() {
            let normalizer = self.matrix().fixed_view::<1, D>(D, 0);
            let n = normalizer.tr_dot(rhs);

            if !n.is_zero() {
                return transform * (rhs / n);
            }
        }

        transform * rhs
    };
);

// Transform × Point
md_impl_all!(
    Mul, mul where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategory,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Point<T, D>, Output = Point<T, D>;
    [val val] => &self * &rhs;
    [ref val] =>  self * &rhs;
    [val ref] => &self *  rhs;
    [ref ref] => {
        let transform   = self.matrix().fixed_view::<D, D>(0, 0);
        let translation = self.matrix().fixed_view::<D, 1>(0, D);

        if C::has_normalizer() {
            let normalizer = self.matrix().fixed_view::<1, D>(D, 0);
            #[allow(clippy::suspicious_arithmetic_impl)]
            let n = normalizer.tr_dot(&rhs.coords) + unsafe { self.matrix().get_unchecked((D, D)).clone() };

            if !n.is_zero() {
                return (transform * rhs + translation) / n;
            }
        }

        transform * rhs + translation
    };
);

// Transform × Transform
md_impl_all!(
    Mul, mul where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for CA, CB;
    where Const<D>: DimNameAdd<U1>, CA: TCategoryMul<CB>, CB: TCategory,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, CA, D>, rhs: Transform<T, CB, D>, Output = Transform<T, CA::Representative, D>;
    [val val] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.into_inner());
    [ref val] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.into_inner());
    [val ref] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.matrix());
    [ref ref] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.matrix());
);

// Transform × Rotation
md_impl_all!(
    Mul, mul
    where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, Const<D>)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Rotation<T, D>, Output = Transform<T, C::Representative, D>;
    [val val] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref val] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
    [val ref] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref ref] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
);

// Rotation × Transform
md_impl_all!(
    Mul, mul where T: RealField;
    (Const<D>, Const<D>), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Rotation<T, D>, rhs: Transform<T, C, D>, Output = Transform<T, C::Representative, D>;
    [val val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [ref val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [val ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
    [ref ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
);

// Transform × UnitQuaternion
md_impl_all!(
    Mul, mul where T: RealField;
    (U4, U4), (U4, U1)
    const;
    for C;
    where C: TCategoryMul<TAffine>;
    self: Transform<T, C, 3>, rhs: UnitQuaternion<T>, Output = Transform<T, C::Representative, 3>;
    [val val] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref val] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
    [val ref] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.clone().to_homogeneous());
    [ref ref] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.clone().to_homogeneous());
);

// Transform × UnitComplex
md_impl_all!(
    Mul, mul where T: RealField;
    (U3, U3), (U2, U1)
    const;
    for C;
    where C: TCategoryMul<TAffine>;
    self: Transform<T, C, 2>, rhs: UnitComplex<T>, Output = Transform<T, C::Representative, 2>;
    [val val] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref val] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
    [val ref] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.clone().to_homogeneous());
    [ref ref] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.clone().to_homogeneous());
);

// UnitQuaternion × Transform
md_impl_all!(
    Mul, mul where T: RealField;
    (U4, U1), (U4, U4)
    const;
    for C;
    where C: TCategoryMul<TAffine>;
    self: UnitQuaternion<T>, rhs: Transform<T, C, 3>, Output = Transform<T, C::Representative, 3>;
    [val val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [ref val] => Self::Output::from_matrix_unchecked(self.clone().to_homogeneous() * rhs.into_inner());
    [val ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
    [ref ref] => Self::Output::from_matrix_unchecked(self.clone().to_homogeneous() * rhs.matrix());
);

// UnitComplex × Transform
md_impl_all!(
    Mul, mul where T: RealField;
    (U2, U1), (U3, U3)
    const;
    for C;
    where C: TCategoryMul<TAffine>;
    self: UnitComplex<T>, rhs: Transform<T, C, 2>, Output = Transform<T, C::Representative, 2>;
    [val val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [ref val] => Self::Output::from_matrix_unchecked(self.clone().to_homogeneous() * rhs.into_inner());
    [val ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
    [ref ref] => Self::Output::from_matrix_unchecked(self.clone().to_homogeneous() * rhs.matrix());
);

// Transform × Isometry
md_impl_all!(
    Mul, mul where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C, R;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Isometry<T, R, D>, Output = Transform<T, C::Representative, D>;
    [val val] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref val] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
    [val ref] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref ref] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
);

// Isometry × Transform
md_impl_all!(
    Mul, mul where T: RealField;
    (Const<D>, U1), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for C, R;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Isometry<T, R, D>, rhs: Transform<T, C, D>, Output = Transform<T, C::Representative, D>;
    [val val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [ref val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [val ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
    [ref ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
);

// Transform × Similarity
md_impl_all!(
    Mul, mul where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C, R;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Similarity<T, R, D>, Output = Transform<T, C::Representative, D>;
    [val val] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref val] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
    [val ref] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref ref] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
);

// Similarity × Transform
md_impl_all!(
    Mul, mul where T: RealField;
    (Const<D>, U1), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for C, R;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Similarity<T, R, D>, rhs: Transform<T, C, D>, Output = Transform<T, C::Representative, D>;
    [val val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [ref val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [val ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
    [ref ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
);

/*
 *
 * TODO: don't explicitly build the homogeneous translation matrix.
 * Directly apply the translation, just as in `Matrix::{append,prepend}_translation`. This has not
 * been done yet because of the `DimNameDiff` requirement (which is not automatically deduced from
 * `DimNameAdd` requirement).
 *
 */
// Transform × Translation
md_impl_all!(
    Mul, mul where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Translation<T, D>, Output = Transform<T, C::Representative, D>;
    [val val] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref val] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
    [val ref] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
    [ref ref] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
);

// Translation × Transform
md_impl_all!(
    Mul, mul where T: RealField;
    (Const<D>, U1), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Translation<T, D>, rhs: Transform<T, C, D>, Output = Transform<T, C::Representative, D>;
    [val val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [ref val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
    [val ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
    [ref ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
);

// Transform ÷ Transform
md_impl_all!(
    Div, div where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for CA, CB;
    where Const<D>: DimNameAdd<U1>, CA: TCategoryMul<CB>, CB: SubTCategoryOf<TProjective>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, CA, D>, rhs: Transform<T, CB, D>, Output = Transform<T, CA::Representative, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.clone().inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.clone().inverse() };
);

// Transform ÷ Rotation
md_impl_all!(
    Div, div where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, Const<D>)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Rotation<T, D>, Output = Transform<T, C::Representative, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
);

// Rotation ÷ Transform
md_impl_all!(
    Div, div where T: RealField;
    (Const<D>, Const<D>), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Rotation<T, D>, rhs: Transform<T, C, D>, Output = Transform<T, C::Representative, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
);

// Transform ÷ UnitQuaternion
md_impl_all!(
    Div, div where T: RealField;
    (U4, U4), (U4, U1)
    const;
    for C;
    where C: TCategoryMul<TAffine>;
    self: Transform<T, C, 3>, rhs: UnitQuaternion<T>, Output = Transform<T, C::Representative, 3>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
);

// UnitQuaternion ÷ Transform
md_impl_all!(
    Div, div where T: RealField;
    (U4, U1), (U4, U4)
    const;
    for C;
    where C: TCategoryMul<TAffine>;
    self: UnitQuaternion<T>, rhs: Transform<T, C, 3>, Output = Transform<T, C::Representative, 3>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
);

//      // Transform ÷ Isometry
//      md_impl_all!(
//          Div, div where T: RealField;
//          (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
//          for Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >
//          where SB::Alloc: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1> >;
//          self: Transform<T, C, D>, rhs: Isometry<T, R, D>, Output = Transform<T, C::Representative, D>;
//          [val val] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.inverse().to_homogeneous());
//          [ref val] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.inverse().to_homogeneous());
//          [val ref] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.inverse().to_homogeneous());
//          [ref ref] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.inverse().to_homogeneous());
//      );

//      // Isometry ÷ Transform
//      md_impl_all!(
//          Div, div where T: RealField;
//          (Const<D>, U1), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
//          for Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >
//          where SA::Alloc: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1> >;
//          self: Isometry<T, R, D>, rhs: Transform<T, C, D>, Output = Transform<T, C::Representative, D>;
//          [val val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
//          [ref val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
//          [val ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
//          [ref ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
//      );

//      // Transform ÷ Similarity
//      md_impl_all!(
//          Div, div where T: RealField;
//          (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
//          for Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >
//          where SB::Alloc: Allocator<D, D >
//          where SB::Alloc: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1> >;
//          self: Transform<T, C, D>, rhs: Similarity<T, R, D>, Output = Transform<T, C::Representative, D>;
//          [val val] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
//          [ref val] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
//          [val ref] => Self::Output::from_matrix_unchecked(self.into_inner() * rhs.to_homogeneous());
//          [ref ref] => Self::Output::from_matrix_unchecked(self.matrix() * rhs.to_homogeneous());
//      );

//      // Similarity ÷ Transform
//      md_impl_all!(
//          Div, div where T: RealField;
//          (Const<D>, U1), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
//          for Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >
//          where SA::Alloc: Allocator<D, D >
//          where SA::Alloc: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1> >;
//          self: Similarity<T, R, D>, rhs: Transform<T, C, D>, Output = Transform<T, C::Representative, D>;
//          [val val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
//          [ref val] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.into_inner());
//          [val ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
//          [ref ref] => Self::Output::from_matrix_unchecked(self.to_homogeneous() * rhs.matrix());
//      );

// Transform ÷ Translation
md_impl_all!(
    Div, div where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Translation<T, D>, Output = Transform<T, C::Representative, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self * rhs.inverse() };
);

// Translation ÷ Transform
md_impl_all!(
    Div, div where T: RealField;
    (Const<D>, U1), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategoryMul<TAffine>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Translation<T, D>, rhs: Transform<T, C, D>, Output = Transform<T, C::Representative, D>;
    [val val] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
    [ref val] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
    [val ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
    [ref ref] => #[allow(clippy::suspicious_arithmetic_impl)] { self.inverse() * rhs };
);

// Transform ×= Transform
md_assign_impl_all!(
    MulAssign, mul_assign where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for CA, CB;
    where Const<D>: DimNameAdd<U1>, CA: TCategory, CB: SubTCategoryOf<CA>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, CA, D>, rhs: Transform<T, CB, D>;
    [val] => *self.matrix_mut_unchecked() *= rhs.into_inner();
    [ref] => *self.matrix_mut_unchecked() *= rhs.matrix();
);

// Transform ×= Similarity
md_assign_impl_all!(
    MulAssign, mul_assign where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C, R;
    where Const<D>: DimNameAdd<U1>, C: TCategory, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Similarity<T, R, D>;
    [val] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
    [ref] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
);

// Transform ×= Isometry
md_assign_impl_all!(
    MulAssign, mul_assign where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C, R;
    where Const<D>: DimNameAdd<U1>, C: TCategory, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Isometry<T, R, D>;
    [val] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
    [ref] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
);

/*
 *
 * TODO: don't explicitly build the homogeneous translation matrix.
 * Directly apply the translation, just as in `Matrix::{append,prepend}_translation`. This has not
 * been done yet because of the `DimNameDiff` requirement (which is not automatically deduced from
 * `DimNameAdd` requirement).
 *
 */
// Transform ×= Translation
md_assign_impl_all!(
    MulAssign, mul_assign where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategory,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Translation<T, D>;
    [val] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
    [ref] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
);

// Transform ×= Rotation
md_assign_impl_all!(
    MulAssign, mul_assign where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, Const<D>)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategory,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Rotation<T, D>;
    [val] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
    [ref] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
);

// Transform ×= UnitQuaternion
md_assign_impl_all!(
    MulAssign, mul_assign where T: RealField;
    (U4, U4), (U4, U1)
    const;
    for C;
    where C: TCategory;
    self: Transform<T, C, 3>, rhs: UnitQuaternion<T>;
    [val] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
    [ref] => *self.matrix_mut_unchecked() *= rhs.clone().to_homogeneous();
);

// Transform ×= UnitComplex
md_assign_impl_all!(
    MulAssign, mul_assign where T: RealField;
    (U3, U3), (U2, U1)
    const;
    for C;
    where C: TCategory;
    self: Transform<T, C, 2>, rhs: UnitComplex<T>;
    [val] => *self.matrix_mut_unchecked() *= rhs.to_homogeneous();
    [ref] => *self.matrix_mut_unchecked() *= rhs.clone().to_homogeneous();
);

// Transform ÷= Transform
md_assign_impl_all!(
    DivAssign, div_assign where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>)
    const D;
    for CA, CB;
    where Const<D>: DimNameAdd<U1>, CA: SuperTCategoryOf<CB>, CB: SubTCategoryOf<TProjective>,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, CA, D>, rhs: Transform<T, CB, D>;
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.clone().inverse() };
);

//      // Transform ÷= Similarity
//      md_assign_impl_all!(
//          DivAssign, div_assign;
//          (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
//          for Const<D>: DimNameAdd<U1>, C: TCategory, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >;
//          self: Transform<T, C, D>, rhs: Similarity<T, R, D>;
//          [val] => *self *= rhs.inverse();
//          [ref] => *self *= rhs.inverse();
//      );
//
//
//      // Transform ÷= Isometry
//      md_assign_impl_all!(
//          DivAssign, div_assign;
//          (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
//          for Const<D>: DimNameAdd<U1>, C: TCategory, R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> >;
//          self: Transform<T, C, D>, rhs: Isometry<T, R, D>;
//          [val] => *self *= rhs.inverse();
//          [ref] => *self *= rhs.inverse();
//      );

// Transform ÷= Translation
md_assign_impl_all!(
    DivAssign, div_assign where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, U1)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategory,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Translation<T, D>;
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

// Transform ÷= Rotation
md_assign_impl_all!(
    DivAssign, div_assign where T: RealField;
    (DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>), (Const<D>, Const<D>)
    const D;
    for C;
    where Const<D>: DimNameAdd<U1>, C: TCategory,
          DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>;
    self: Transform<T, C, D>, rhs: Rotation<T, D>;
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

// Transform ÷= UnitQuaternion
md_assign_impl_all!(
    DivAssign, div_assign where T: RealField;
    (U4, U4), (U4, U1)
    const;
    for C;
    where C: TCategory;
    self: Transform<T, C, 3>, rhs: UnitQuaternion<T>;
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);

// Transform ÷= UnitComplex
md_assign_impl_all!(
    DivAssign, div_assign where T: RealField;
    (U3, U3), (U2, U1)
    const;
    for C;
    where C: TCategory;
    self: Transform<T, C, 2>, rhs: UnitComplex<T>;
    [val] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
    [ref] => #[allow(clippy::suspicious_op_assign_impl)] { *self *= rhs.inverse() };
);
