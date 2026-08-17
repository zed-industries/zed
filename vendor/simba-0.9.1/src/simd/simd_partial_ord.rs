use crate::simd::SimdValue;

/// Lane-wise generalization of the standard `PartialOrd` for SIMD values.
pub trait SimdPartialOrd: SimdValue {
    /// Lanewise _greater than_ `>` comparison.
    fn simd_gt(self, other: Self) -> Self::SimdBool;
    /// Lanewise _less than_ `<` comparison.
    fn simd_lt(self, other: Self) -> Self::SimdBool;
    /// Lanewise _greater or equal_ `>=` comparison.
    fn simd_ge(self, other: Self) -> Self::SimdBool;
    /// Lanewise _less or equal_ `<=` comparison.
    fn simd_le(self, other: Self) -> Self::SimdBool;
    /// Lanewise _equal_ `==` comparison.
    fn simd_eq(self, other: Self) -> Self::SimdBool;
    /// Lanewise _not equal_ `!=` comparison.
    fn simd_ne(self, other: Self) -> Self::SimdBool;

    /// Lanewise max value.
    fn simd_max(self, other: Self) -> Self;
    /// Lanewise min value.
    fn simd_min(self, other: Self) -> Self;
    /// Clamps each lane of `self` between the corresponding lane of `min` and `max`.
    fn simd_clamp(self, min: Self, max: Self) -> Self;

    /// The min value among all lanes of `self`.
    fn simd_horizontal_min(self) -> Self::Element;
    /// The max value among all lanes of `self`.
    fn simd_horizontal_max(self) -> Self::Element;
}

impl<T: PartialOrd + SimdValue<Element = T, SimdBool = bool>> SimdPartialOrd for T {
    #[inline(always)]
    fn simd_gt(self, other: Self) -> Self::SimdBool {
        self > other
    }

    #[inline(always)]
    fn simd_lt(self, other: Self) -> Self::SimdBool {
        self < other
    }

    #[inline(always)]
    fn simd_ge(self, other: Self) -> Self::SimdBool {
        self >= other
    }

    #[inline(always)]
    fn simd_le(self, other: Self) -> Self::SimdBool {
        self <= other
    }

    #[inline(always)]
    fn simd_eq(self, other: Self) -> Self::SimdBool {
        self == other
    }

    #[inline(always)]
    fn simd_ne(self, other: Self) -> Self::SimdBool {
        self != other
    }

    #[inline(always)]
    fn simd_max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    #[inline(always)]
    fn simd_min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }

    #[inline(always)]
    fn simd_clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    #[inline(always)]
    fn simd_horizontal_min(self) -> Self::Element {
        self
    }

    #[inline(always)]
    fn simd_horizontal_max(self) -> Self::Element {
        self
    }
}
