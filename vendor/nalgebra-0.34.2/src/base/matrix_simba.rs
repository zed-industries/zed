use simba::simd::SimdValue;

use crate::base::allocator::Allocator;
use crate::base::dimension::Dim;
use crate::base::{DefaultAllocator, OMatrix, Scalar};

/*
 *
 * Simd structures.
 *
 */
impl<T, R, C> SimdValue for OMatrix<T, R, C>
where
    T: Scalar + SimdValue,
    R: Dim,
    C: Dim,
    T::Element: Scalar,
    DefaultAllocator: Allocator<R, C>,
{
    const LANES: usize = T::LANES;
    type Element = OMatrix<T::Element, R, C>;
    type SimdBool = T::SimdBool;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        val.map(T::splat)
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        self.map(|e| e.extract(i))
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        unsafe { self.map(|e| e.extract_unchecked(i)) }
    }

    #[inline]
    fn replace(&mut self, i: usize, val: Self::Element) {
        self.zip_apply(&val, |a, b| {
            a.replace(i, b);
        })
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        unsafe {
            self.zip_apply(&val, |a, b| {
                a.replace_unchecked(i, b);
            })
        }
    }

    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        self.zip_map(&other, |a, b| a.select(cond, b))
    }
}
