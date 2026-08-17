use crate::simd::SimdValue;
use num::Signed;

/// A lane-wise generalization of [`num::Signed`](https://rust-num.github.io/num/num/trait.Signed.html) for SIMD values.
pub trait SimdSigned: SimdValue {
    /// The absolute value of each lane of `self`.
    fn simd_abs(&self) -> Self;
    /// The absolute difference of each lane of `self`.
    ///
    /// For each lane, this zero if the lane of self is less than or equal to the corresponding lane of other
    /// otherwise the difference between the lane of self and the lane of other is returned.
    fn simd_abs_sub(&self, other: &Self) -> Self;
    /// The signum of each lane of `Self`.
    fn simd_signum(&self) -> Self;
    /// Tests which lane is positive.
    fn is_simd_positive(&self) -> Self::SimdBool;
    /// Tests which lane is negative.
    fn is_simd_negative(&self) -> Self::SimdBool;
}

impl<T: Signed + SimdValue<SimdBool = bool>> SimdSigned for T {
    #[inline(always)]
    fn simd_abs(&self) -> Self {
        self.abs()
    }

    #[inline(always)]
    fn simd_abs_sub(&self, other: &Self) -> Self {
        self.abs_sub(other)
    }

    #[inline(always)]
    fn simd_signum(&self) -> Self {
        self.signum()
    }

    #[inline(always)]
    fn is_simd_positive(&self) -> Self::SimdBool {
        self.is_positive()
    }

    #[inline(always)]
    fn is_simd_negative(&self) -> Self::SimdBool {
        self.is_negative()
    }
}
