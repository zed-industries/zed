use simba::simd::SimdValue;

use crate::SimdRealField;

use crate::geometry::{AbstractRotation, Isometry, Translation};

impl<T: SimdRealField, R, const D: usize> SimdValue for Isometry<T, R, D>
where
    T::Element: SimdRealField,
    R: SimdValue<SimdBool = T::SimdBool> + AbstractRotation<T, D>,
    R::Element: AbstractRotation<T::Element, D>,
{
    const LANES: usize = T::LANES;
    type Element = Isometry<T::Element, R::Element, D>;
    type SimdBool = T::SimdBool;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        Isometry::from_parts(Translation::splat(val.translation), R::splat(val.rotation))
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        Isometry::from_parts(self.translation.extract(i), self.rotation.extract(i))
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        unsafe {
            Isometry::from_parts(
                self.translation.extract_unchecked(i),
                self.rotation.extract_unchecked(i),
            )
        }
    }

    #[inline]
    fn replace(&mut self, i: usize, val: Self::Element) {
        self.translation.replace(i, val.translation);
        self.rotation.replace(i, val.rotation);
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        unsafe {
            self.translation.replace_unchecked(i, val.translation);
            self.rotation.replace_unchecked(i, val.rotation);
        }
    }

    #[inline]
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        Isometry::from_parts(
            self.translation.select(cond, other.translation),
            self.rotation.select(cond, other.rotation),
        )
    }
}
