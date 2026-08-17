use simba::simd::SimdValue;

use crate::RealField;
use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, OMatrix, Scalar};

use crate::geometry::{TCategory, Transform};

impl<T: RealField, C, const D: usize> SimdValue for Transform<T, C, D>
where
    T::Element: Scalar,
    C: TCategory,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    const LANES: usize = T::LANES;
    type Element = Transform<T::Element, C, D>;
    type SimdBool = T::SimdBool;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        Transform::from_matrix_unchecked(OMatrix::splat(val.into_inner()))
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        Transform::from_matrix_unchecked(self.matrix().extract(i))
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        unsafe { Transform::from_matrix_unchecked(self.matrix().extract_unchecked(i)) }
    }

    #[inline]
    fn replace(&mut self, i: usize, val: Self::Element) {
        self.matrix_mut_unchecked().replace(i, val.into_inner())
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        unsafe {
            self.matrix_mut_unchecked()
                .replace_unchecked(i, val.into_inner())
        }
    }

    #[inline]
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        Transform::from_matrix_unchecked(self.into_inner().select(cond, other.into_inner()))
    }
}
