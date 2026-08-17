use simba::simd::SimdValue;

use crate::Scalar;
use crate::base::OVector;

use crate::geometry::Translation;

impl<T: Scalar + SimdValue, const D: usize> SimdValue for Translation<T, D>
where
    T::Element: Scalar,
{
    const LANES: usize = T::LANES;
    type Element = Translation<T::Element, D>;
    type SimdBool = T::SimdBool;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        OVector::splat(val.vector).into()
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        self.vector.extract(i).into()
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        unsafe { self.vector.extract_unchecked(i).into() }
    }

    #[inline]
    fn replace(&mut self, i: usize, val: Self::Element) {
        self.vector.replace(i, val.vector)
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        unsafe { self.vector.replace_unchecked(i, val.vector) }
    }

    #[inline]
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        self.vector.select(cond, other.vector).into()
    }
}
