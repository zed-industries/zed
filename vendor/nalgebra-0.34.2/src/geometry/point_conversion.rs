use num::{One, Zero};
use simba::scalar::{ClosedDivAssign, SubsetOf, SupersetOf};
use simba::simd::PrimitiveSimdValue;

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, Matrix, OVector, Scalar};

use crate::geometry::Point;
use crate::{DimName, OPoint};

/*
 * This file provides the following conversions:
 * =============================================
 *
 * Point -> Point
 * Point -> Vector (homogeneous)
 */

impl<T1, T2, D: DimName> SubsetOf<OPoint<T2, D>> for OPoint<T1, D>
where
    T1: Scalar,
    T2: Scalar + SupersetOf<T1>,
    DefaultAllocator: Allocator<D>,
{
    #[inline]
    fn to_superset(&self) -> OPoint<T2, D> {
        OPoint::from(self.coords.to_superset())
    }

    #[inline]
    fn is_in_subset(m: &OPoint<T2, D>) -> bool {
        // TODO: is there a way to reuse the `.is_in_subset` from the matrix implementation of
        // SubsetOf?
        m.iter().all(|e| e.is_in_subset())
    }

    #[inline]
    fn from_superset_unchecked(m: &OPoint<T2, D>) -> Self {
        Self::from(Matrix::from_superset_unchecked(&m.coords))
    }
}

impl<T1, T2, D> SubsetOf<OVector<T2, DimNameSum<D, U1>>> for OPoint<T1, D>
where
    D: DimNameAdd<U1>,
    T1: Scalar,
    T2: Scalar + Zero + One + ClosedDivAssign + SupersetOf<T1>,
    DefaultAllocator: Allocator<D> + Allocator<DimNameSum<D, U1>>,
    // + Allocator<T1, D>
    // + Allocator<D>,
{
    #[inline]
    fn to_superset(&self) -> OVector<T2, DimNameSum<D, U1>> {
        let p: OPoint<T2, D> = self.to_superset();
        p.to_homogeneous()
    }

    #[inline]
    fn is_in_subset(v: &OVector<T2, DimNameSum<D, U1>>) -> bool {
        crate::is_convertible::<_, OVector<T1, DimNameSum<D, U1>>>(v) && !v[D::DIM].is_zero()
    }

    #[inline]
    fn from_superset_unchecked(v: &OVector<T2, DimNameSum<D, U1>>) -> Self {
        let coords = v.generic_view((0, 0), (D::name(), Const::<1>)) / v[D::DIM].clone();
        Self {
            coords: crate::convert_unchecked(coords),
        }
    }
}

impl<T: Scalar + Zero + One, D: DimName> From<OPoint<T, D>> for OVector<T, DimNameSum<D, U1>>
where
    D: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<D, U1>> + Allocator<D>,
{
    #[inline]
    fn from(t: OPoint<T, D>) -> Self {
        t.to_homogeneous()
    }
}

impl<T: Scalar, const D: usize> From<[T; D]> for Point<T, D> {
    #[inline]
    fn from(coords: [T; D]) -> Self {
        Point {
            coords: coords.into(),
        }
    }
}

impl<T: Scalar, const D: usize> From<Point<T, D>> for [T; D] {
    #[inline]
    fn from(p: Point<T, D>) -> Self {
        p.coords.into()
    }
}

impl<T: Scalar, D: DimName> From<OVector<T, D>> for OPoint<T, D>
where
    DefaultAllocator: Allocator<D>,
{
    #[inline]
    fn from(coords: OVector<T, D>) -> Self {
        OPoint { coords }
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue, const D: usize> From<[Point<T::Element, D>; 2]>
    for Point<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 2]>,
    T::Element: Scalar + Copy,
    <DefaultAllocator as Allocator<Const<D>>>::Buffer<T::Element>: Copy,
{
    #[inline]
    fn from(arr: [Point<T::Element, D>; 2]) -> Self {
        Self::from(OVector::from([arr[0].coords, arr[1].coords]))
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue, const D: usize> From<[Point<T::Element, D>; 4]>
    for Point<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 4]>,
    T::Element: Scalar + Copy,
    <DefaultAllocator as Allocator<Const<D>>>::Buffer<T::Element>: Copy,
{
    #[inline]
    fn from(arr: [Point<T::Element, D>; 4]) -> Self {
        Self::from(OVector::from([
            arr[0].coords,
            arr[1].coords,
            arr[2].coords,
            arr[3].coords,
        ]))
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue, const D: usize> From<[Point<T::Element, D>; 8]>
    for Point<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 8]>,
    T::Element: Scalar + Copy,
    <DefaultAllocator as Allocator<Const<D>>>::Buffer<T::Element>: Copy,
{
    #[inline]
    fn from(arr: [Point<T::Element, D>; 8]) -> Self {
        Self::from(OVector::from([
            arr[0].coords,
            arr[1].coords,
            arr[2].coords,
            arr[3].coords,
            arr[4].coords,
            arr[5].coords,
            arr[6].coords,
            arr[7].coords,
        ]))
    }
}

impl<T: Scalar + Copy + PrimitiveSimdValue, const D: usize> From<[Point<T::Element, D>; 16]>
    for Point<T, D>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 16]>,
    T::Element: Scalar + Copy,
    <DefaultAllocator as Allocator<Const<D>>>::Buffer<T::Element>: Copy,
{
    #[inline]
    fn from(arr: [Point<T::Element, D>; 16]) -> Self {
        Self::from(OVector::from([
            arr[0].coords,
            arr[1].coords,
            arr[2].coords,
            arr[3].coords,
            arr[4].coords,
            arr[5].coords,
            arr[6].coords,
            arr[7].coords,
            arr[8].coords,
            arr[9].coords,
            arr[10].coords,
            arr[11].coords,
            arr[12].coords,
            arr[13].coords,
            arr[14].coords,
            arr[15].coords,
        ]))
    }
}
