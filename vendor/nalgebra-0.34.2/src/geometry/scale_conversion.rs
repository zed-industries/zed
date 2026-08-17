use num::{One, Zero};

use simba::scalar::{RealField, SubsetOf, SupersetOf};
use simba::simd::PrimitiveSimdValue;

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, OMatrix, OVector, SVector, Scalar};

use crate::Point;
use crate::geometry::{Scale, SuperTCategoryOf, TAffine, Transform};

/*
 * This file provides the following conversions:
 * =============================================
 *
 * Scale -> Scale
 * Scale -> Transform
 * Scale -> Matrix (homogeneous)
 */

impl<T1, T2, const D: usize> SubsetOf<Scale<T2, D>> for Scale<T1, D>
where
    T1: Scalar,
    T2: Scalar + SupersetOf<T1>,
{
    #[inline]
    fn to_superset(&self) -> Scale<T2, D> {
        Scale::from(self.vector.to_superset())
    }

    #[inline]
    fn is_in_subset(rot: &Scale<T2, D>) -> bool {
        crate::is_convertible::<_, SVector<T1, D>>(&rot.vector)
    }

    #[inline]
    fn from_superset_unchecked(rot: &Scale<T2, D>) -> Self {
        Scale {
            vector: rot.vector.to_subset_unchecked(),
        }
    }
}

impl<T1, T2, C, const D: usize> SubsetOf<Transform<T2, C, D>> for Scale<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    C: SuperTCategoryOf<TAffine>,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<DimNameSum<Const<D>, U1>, U1>,
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
    SubsetOf<OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>> for Scale<T1, D>
where
    T1: RealField,
    T2: RealField + SupersetOf<T1>,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<DimNameSum<Const<D>, U1>, U1>,
{
    #[inline]
    fn to_superset(&self) -> OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        self.to_homogeneous().to_superset()
    }

    #[inline]
    fn is_in_subset(m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>) -> bool {
        if m[(D, D)] != T2::one() {
            return false;
        }
        for i in 0..D + 1 {
            for j in 0..D + 1 {
                if i != j && m[(i, j)] != T2::zero() {
                    return false;
                }
            }
        }
        true
    }

    #[inline]
    fn from_superset_unchecked(
        m: &OMatrix<T2, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    ) -> Self {
        let v = m.fixed_view::<D, D>(0, 0).diagonal();
        Self {
            vector: crate::convert_unchecked(v),
        }
    }
}

impl<T: Scalar + Zero + One, const D: usize> From<Scale<T, D>>
    for OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<DimNameSum<Const<D>, U1>, U1>
        + Allocator<Const<D>>,
{
    #[inline]
    fn from(t: Scale<T, D>) -> Self {
        t.to_homogeneous()
    }
}

impl<T: Scalar, const D: usize> From<OVector<T, Const<D>>> for Scale<T, D> {
    #[inline]
    fn from(vector: OVector<T, Const<D>>) -> Self {
        Scale { vector }
    }
}

impl<T: Scalar, const D: usize> From<[T; D]> for Scale<T, D> {
    #[inline]
    fn from(coords: [T; D]) -> Self {
        Scale {
            vector: coords.into(),
        }
    }
}

impl<T: Scalar, const D: usize> From<Point<T, D>> for Scale<T, D> {
    #[inline]
    fn from(pt: Point<T, D>) -> Self {
        Scale { vector: pt.coords }
    }
}

impl<T: Scalar, const D: usize> From<Scale<T, D>> for [T; D] {
    #[inline]
    fn from(t: Scale<T, D>) -> Self {
        t.vector.into()
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Scale<T::Element, D>; 2]> for Scale<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 2]>,
    T::Element: Scalar,
{
    #[inline]
    fn from(arr: [Scale<T::Element, D>; 2]) -> Self {
        Self::from(OVector::from([
            arr[0].vector.clone(),
            arr[1].vector.clone(),
        ]))
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Scale<T::Element, D>; 4]> for Scale<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 4]>,
    T::Element: Scalar,
{
    #[inline]
    fn from(arr: [Scale<T::Element, D>; 4]) -> Self {
        Self::from(OVector::from([
            arr[0].vector.clone(),
            arr[1].vector.clone(),
            arr[2].vector.clone(),
            arr[3].vector.clone(),
        ]))
    }
}

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Scale<T::Element, D>; 8]> for Scale<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 8]>,
    T::Element: Scalar,
{
    #[inline]
    fn from(arr: [Scale<T::Element, D>; 8]) -> Self {
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

impl<T: Scalar + PrimitiveSimdValue, const D: usize> From<[Scale<T::Element, D>; 16]>
    for Scale<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 16]>,
    T::Element: Scalar,
{
    #[inline]
    fn from(arr: [Scale<T::Element, D>; 16]) -> Self {
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
