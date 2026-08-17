use simba::simd::SimdValue;

use crate::base::{OVector, Scalar};

use crate::geometry::Point;

impl<T: Scalar + SimdValue, const D: usize> SimdValue for Point<T, D>
where
    T::Element: Scalar,
{
    const LANES: usize = T::LANES;
    type Element = Point<T::Element, D>;
    type SimdBool = T::SimdBool;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        OVector::splat(val.coords).into()
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        self.coords.extract(i).into()
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        unsafe { self.coords.extract_unchecked(i).into() }
    }

    #[inline]
    fn replace(&mut self, i: usize, val: Self::Element) {
        self.coords.replace(i, val.coords)
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        unsafe { self.coords.replace_unchecked(i, val.coords) }
    }

    #[inline]
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        self.coords.select(cond, other.coords).into()
    }
}
