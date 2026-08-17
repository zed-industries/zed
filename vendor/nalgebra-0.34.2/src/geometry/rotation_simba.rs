use simba::simd::SimdValue;

use crate::base::{OMatrix, Scalar};

use crate::geometry::Rotation;

impl<T, const D: usize> SimdValue for Rotation<T, D>
where
    T: Scalar + SimdValue,
    T::Element: Scalar,
{
    const LANES: usize = T::LANES;
    type Element = Rotation<T::Element, D>;
    type SimdBool = T::SimdBool;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        Rotation::from_matrix_unchecked(OMatrix::splat(val.into_inner()))
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        Rotation::from_matrix_unchecked(self.matrix().extract(i))
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        unsafe { Rotation::from_matrix_unchecked(self.matrix().extract_unchecked(i)) }
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
        Rotation::from_matrix_unchecked(self.into_inner().select(cond, other.into_inner()))
    }
}
