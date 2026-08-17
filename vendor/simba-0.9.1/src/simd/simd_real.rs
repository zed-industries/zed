use crate::scalar::RealField;
use crate::simd::{SimdComplexField, SimdPartialOrd, SimdSigned};

/// Lanewise generalization of `RealField` for SIMD reals.
///
/// Each lane of an SIMD real field should contain one real field.
/// This is implemented by scalar reals like `f32` and `f64` as well as SIMD reals like `portable_simd::f32x4`.
#[allow(missing_docs)]
pub trait SimdRealField:
    SimdPartialOrd + SimdSigned + SimdComplexField<SimdRealField = Self>
{
    /// Copies the sign of `sign` to `self`.
    ///
    /// - Returns `self.simd_abs()` if `sign` is positive or positive-zero.
    /// - Returns `-self.simd_abs()` if `sign` is negative or negative-zero.
    fn simd_copysign(self, sign: Self) -> Self;
    fn simd_atan2(self, other: Self) -> Self;

    fn simd_default_epsilon() -> Self;
    fn simd_pi() -> Self;
    fn simd_two_pi() -> Self;
    fn simd_frac_pi_2() -> Self;
    fn simd_frac_pi_3() -> Self;
    fn simd_frac_pi_4() -> Self;
    fn simd_frac_pi_6() -> Self;
    fn simd_frac_pi_8() -> Self;
    fn simd_frac_1_pi() -> Self;
    fn simd_frac_2_pi() -> Self;
    fn simd_frac_2_sqrt_pi() -> Self;

    fn simd_e() -> Self;
    fn simd_log2_e() -> Self;
    fn simd_log10_e() -> Self;
    fn simd_ln_2() -> Self;
    fn simd_ln_10() -> Self;
}

// Blanket impl RealField => SimdRealField
impl<T: RealField> SimdRealField for T {
    #[inline(always)]
    fn simd_atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    #[inline(always)]
    fn simd_default_epsilon() -> Self {
        Self::default_epsilon()
    }
    #[inline(always)]
    fn simd_copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }
    #[inline(always)]
    fn simd_pi() -> Self {
        Self::pi()
    }
    #[inline(always)]
    fn simd_two_pi() -> Self {
        Self::two_pi()
    }
    #[inline(always)]
    fn simd_frac_pi_2() -> Self {
        Self::frac_pi_2()
    }
    #[inline(always)]
    fn simd_frac_pi_3() -> Self {
        Self::frac_pi_3()
    }
    #[inline(always)]
    fn simd_frac_pi_4() -> Self {
        Self::frac_pi_4()
    }
    #[inline(always)]
    fn simd_frac_pi_6() -> Self {
        Self::frac_pi_6()
    }
    #[inline(always)]
    fn simd_frac_pi_8() -> Self {
        Self::frac_pi_8()
    }
    #[inline(always)]
    fn simd_frac_1_pi() -> Self {
        Self::frac_1_pi()
    }
    #[inline(always)]
    fn simd_frac_2_pi() -> Self {
        Self::frac_2_pi()
    }
    #[inline(always)]
    fn simd_frac_2_sqrt_pi() -> Self {
        Self::frac_2_sqrt_pi()
    }

    #[inline(always)]
    fn simd_e() -> Self {
        Self::e()
    }
    #[inline(always)]
    fn simd_log2_e() -> Self {
        Self::log2_e()
    }
    #[inline(always)]
    fn simd_log10_e() -> Self {
        Self::log10_e()
    }
    #[inline(always)]
    fn simd_ln_2() -> Self {
        Self::ln_2()
    }
    #[inline(always)]
    fn simd_ln_10() -> Self {
        Self::ln_10()
    }
}
