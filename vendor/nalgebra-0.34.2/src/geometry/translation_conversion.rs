use num::{One, Zero};

use simba::scalar::{RealField, SubsetOf, SupersetOf};
use simba::simd::PrimitiveSimdValue;

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, DimName, OMatrix, OVector, SVector, Scalar};

use crate::geometry::{
    AbstractRotation, Isometry, Similarity, SuperTCategoryOf, TAffine, Transform, Translation,
    Translation3, UnitDualQuaternion, UnitQuaternion,
};
use crate::{ArrayStorage, Point};

/*
 * This file provides the following conversions:
 * =============================================
 *
 * Translation -> Translation
 * Translation -> Isometry
 * Translation3 -> UnitDualQuaternion
 * Translation -> Similarity
 * Translation -> Transform
 * Translation -> Matrix (homogeneous)
 */

impl<T1, T2, const D: usize> SubsetOf<Translation<T2, D>> for Translation<T1, D>
where
    T1: Scalar,
    T2: Scalar + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> Translation<T2, D> {
        Translation::from(self.vector.to_superset())
    }

    #[inline]
    fn is_in_subset(rot: &Translation<T2, D>) -> bool {
        crate::is_convertible::<_, SVector<T1, D>>(&rot.vector)
    }

    #[inline]
    fn from_superset_unchecked(rot: &Translation<T2, D>) -> Self {
        Translation {
            vector: rot.vector.to_subset_unchecked(),
        }
    }
}

impl<T1, T2, R, const D: usize> SubsetOf<Isometry<T2, R, D>> for Translation<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T2, D>,
{
    #[inline]
    fn to_superset(&self) -> Isometry<T2, R, D> {
        Isometry::from_parts(self.to_superset(), R::identity())
    }

    #[inline]
    fn is_in_subset(iso: &Isometry<T2, R, D>) -> bool {
        iso.rotation == R::identity()
    }

    #[inline]
    fn from_superset_unchecked(iso: &Isometry<T2, R, D>) -> Self {
        Self::from_superset_unchecked(&iso.translation)
    }
}

impl<T1, T2> SubsetOf<UnitDualQuaternion<T2>> for Translation3<T1>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> UnitDualQuaternion<T2> {
        let dq = UnitDualQuaternion::<T1>::from_parts(self.clone(), UnitQuaternion::identity());
        dq.to_superset()
    }

    #[inline]
    fn is_in_subset(dq: &UnitDualQuaternion<T2>) -> bool {
        crate::is_convertible::<_, Translation<T1, 3>>(&dq.translation())
            && dq.rotation() == UnitQuaternion::identity()
    }

    #[inline]
    fn from_superset_unchecked(dq: &UnitDualQuaternion<T2>) -> Self {
        let dq: UnitDualQuaternion<T1> = crate::convert_ref_unchecked(dq);
        dq.translation()
    }
}

impl<T1, T2, R, const D: usize> SubsetOf<Similarity<T2, R, D>> for Translation<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    R: AbstractRotation<T2, D>,
{
    #[inline]
    fn to_superset(&self) -> Similarity<T2, R, D> {
        Similarity::from_parts(self.to_superset(), R::identity(), T2::one())
    }

    #[inline]
    fn is_in_subset(sim: &Similarity<T2, R, D>) -> bool {
        sim.isometry.rotation == R::identity() && sim.scaling() == T2::one()
    }

    #[inline]
    fn from_superset_unchecked(sim: &Similarity<T2, R, D>) -> Self {
        Self::from_superset_unchecked(&sim.isometry.translation)
    }
}

impl<T1, T2, C, const D: usize> SubsetOf<Transform<T2, C, D>> for Translation<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    C: SuperTCategoryOf<TAffine>,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn to_superset(&self) -> Transform<T2, C, D> {
        Transform::from_matrix_unchecked(self.to_homogeneous().to_superset())
    }

    #[inline]
    fn is_in_subset(t: &Transform<T2, C, D>) -> bool {
        <Self as SubsetOf<_>>::is_in_subset(t.matrix())
    }

    #[inline]
    fn from_superset_unchecked(t: &Transform<T2, C, D>) -> Self {
        Self::from_superset_unchecked(t.matrix())
    }
}

impl<T1, T2, const D: usize>
    SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>> for Translation<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<Const<D>, Buffer<T1> = ArrayStorage<T1, D, 1>>
        + Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn to_superset(&self) -> OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        self.to_homogeneous().to_superset()
    }

    #[inline]
    fn is_in_subset(m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>) -> bool {
        let id = m.generic_view((0, 0), (DimNameSum::<Const<D>, U1>::name(), Const::<D>));

        // Scalar types agree.
        m.iter().all(|e| SupersetOf::<T1>::is_in_subset(e)) &&
        // The block part does nothing.
        id.is_identity(T2::zero()) &&
        // The normalization factor is one.
        m[(D, D)] == T2::one()
    }

    #[inline]
    fn from_superset_unchecked(
        m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    ) -> Self {
        let t: OVector<T2, Const<D>> = m.fixed_view::<D, 1>(0, D).into_owned();
        Self {
            vector: crate::convert_unchecked(t),
        }
    }
}

impl<T: Scalar + Zero + One, const D: usize> From<Translation<T, D>>
    for OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator:
        Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> + Allocator<Const<D>>,
{
    #[inline]
    fn from(t: Translation<T, D>) -> Self {
        t.to_homogeneous()
    }
}

impl<T: Scalar, const D: usize> From<OVector<T, Const<D>>> for Translation<T, D> {
    #[inline]
    fn from(vector: OVector<T, Const<D>>) -> Self {
        Translation { vector }
    }
}

impl<T: Scalar, const D: usize> From<[T; D]> for Translation<T, D> {
    #[inline]
    fn from(coords: [T; D]) -> Self {
        Translation {
            vector: coords.into(),
        }
    }
}

impl<T: Scalar, const D: usize> From<Point<T, D>> for Translation<T, D> {
    #[inline]
    fn from(pt: Point<T, D>) -> Self {
        Translation { vector: pt.coords }
    }
}

impl<T: Scalar, const D: usize> From<Translation<T, D>> for [T; D] {
    #[inline]
    fn from(t: Translation<T, D>) -> Self {
        t.vector.into()
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Translation<T::Element, D>; 2]>
    for Translation<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 2]>,
    T::Element: Scalar,
{
    #[inline]
    fn from(arr: [Translation<T::Element, D>; 2]) -> Self {
        Self::from(OVector::from([
            arr[0].vector.clone(),
            arr[1].vector.clone(),
        ]))
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Translation<T::Element, D>; 4]>
    for Translation<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 4]>,
    T::Element: Scalar,
{
    #[inline]
    fn from(arr: [Translation<T::Element, D>; 4]) -> Self {
        Self::from(OVector::from([
            arr[0].vector.clone(),
            arr[1].vector.clone(),
            arr[2].vector.clone(),
            arr[3].vector.clone(),
        ]))
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Translation<T::Element, D>; 8]>
    for Translation<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 8]>,
    T::Element: Scalar,
{
    #[inline]
    fn from(arr: [Translation<T::Element, D>; 8]) -> Self {
        Self::from(OVector::from([
            arr[0].vector.clone(),
            arr[1].vector.clone(),
            arr[2].vector.clone(),
            arr[3].vector.clone(),
            arr[4].vector.clone(),
            arr[5].vector.clone(),
            arr[6].vector.clone(),
            arr[7].vector.clone(),
        ]))
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Translation<T::Element, D>; 16]>
    for Translation<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 16]>,
    T::Element: Scalar,
{
    #[inline]
    fn from(arr: [Translation<T::Element, D>; 16]) -> Self {
        Self::from(OVector::from([
            arr[0].vector.clone(),
            arr[1].vector.clone(),
            arr[2].vector.clone(),
            arr[3].vector.clone(),
            arr[4].vector.clone(),
            arr[5].vector.clone(),
            arr[6].vector.clone(),
            arr[7].vector.clone(),
            arr[8].vector.clone(),
            arr[9].vector.clone(),
            arr[10].vector.clone(),
            arr[11].vector.clone(),
            arr[12].vector.clone(),
            arr[13].vector.clone(),
            arr[14].vector.clone(),
            arr[15].vector.clone(),
        ]))
    }
}
