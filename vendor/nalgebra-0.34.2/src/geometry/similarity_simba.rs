use simba::simd::{SimdRealField, SimdValue};

use crate::geometry::{AbstractRotation, Isometry, Similarity};

impl<T: SimdRealField, R, const D: usize> SimdValue for Similarity<T, R, D>
where
    T::Element: SimdRealField,
    R: SimdValue<SimdBool = T::SimdBool> + AbstractRotation<T, D>,
    R::Element: AbstractRotation<T::Element, D>,
{
    const LANES: usize = T::LANES;
    type Element = Similarity<T::Element, R::Element, D>;
    type SimdBool = T::SimdBool;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        let scaling = T::splat(val.scaling());
        Similarity::from_isometry(Isometry::splat(val.isometry), scaling)
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        Similarity::from_isometry(self.isometry.extract(i), self.scaling().extract(i))
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        unsafe {
            Similarity::from_isometry(
                self.isometry.extract_unchecked(i),
                self.scaling().extract_unchecked(i),
            )
        }
    }

    #[inline]
    fn replace(&mut self, i: usize, val: Self::Element) {
        let mut s = self.scaling();
        s.replace(i, val.scaling());
        self.set_scaling(s);
        self.isometry.replace(i, val.isometry);
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        unsafe {
            let mut s = self.scaling();
            s.replace_unchecked(i, val.scaling());
            self.set_scaling(s);
            self.isometry.replace_unchecked(i, val.isometry);
        }
    }

    #[inline]
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        let scaling = self.scaling().select(cond, other.scaling());
        Similarity::from_isometry(self.isometry.select(cond, other.isometry), scaling)
    }
}
