use num::{NumAssignOps, NumOps, Zero};
use std::any::Any;
use std::f64;
use std::fmt::Debug;
use std::ops::Neg;

use crate::scalar::{ComplexField, Field, SubsetOf, SupersetOf};
use crate::simd::{SimdRealField, SimdValue};

/// Lane-wise generalisation of `ComplexField` for SIMD complex fields.
///
/// Each lane of an SIMD complex field should contain one complex field.
#[allow(missing_docs)]
pub trait SimdComplexField:
SubsetOf<Self>
+ SupersetOf<f32>
+ SupersetOf<f64>
+ Field
+ Clone
+ Neg<Output=Self>
//    + MeetSemilattice
//    + JoinSemilattice
+ Send
+ Sync
+ Any
+ 'static
+ Debug
+ NumAssignOps
+ NumOps
+ PartialEq
{
    /// Type of the coefficients of a complex number.
    type SimdRealField: SimdRealField<SimdBool=<Self as SimdValue>::SimdBool>;
    complex_trait_methods!(SimdRealField, simd_);

    /// Computes the sum of all the lanes of `self`.
    fn simd_horizontal_sum(self) -> Self::Element;

    /// Computes the product of all the lanes of `self`.
    fn simd_horizontal_product(self) -> Self::Element;
}

// Blanket impl: ComplexField => SimdComplexField
impl<T: ComplexField> SimdComplexField for T {
    type SimdRealField = T::RealField;

    #[inline(always)]
    fn from_simd_real(re: Self::SimdRealField) -> Self {
        Self::from_real(re)
    }
    #[inline(always)]
    fn simd_real(self) -> Self::SimdRealField {
        self.real()
    }
    #[inline(always)]
    fn simd_imaginary(self) -> Self::SimdRealField {
        self.imaginary()
    }
    #[inline(always)]
    fn simd_modulus(self) -> Self::SimdRealField {
        self.modulus()
    }
    #[inline(always)]
    fn simd_modulus_squared(self) -> Self::SimdRealField {
        self.modulus_squared()
    }
    #[inline(always)]
    fn simd_argument(self) -> Self::SimdRealField {
        self.argument()
    }
    #[inline(always)]
    fn simd_norm1(self) -> Self::SimdRealField {
        self.norm1()
    }
    #[inline(always)]
    fn simd_scale(self, factor: Self::SimdRealField) -> Self {
        self.scale(factor)
    }
    #[inline(always)]
    fn simd_unscale(self, factor: Self::SimdRealField) -> Self {
        self.unscale(factor)
    }
    #[inline(always)]
    fn simd_to_polar(self) -> (Self::SimdRealField, Self::SimdRealField) {
        self.to_polar()
    }
    #[inline(always)]
    fn simd_to_exp(self) -> (Self::SimdRealField, Self) {
        self.to_exp()
    }
    #[inline(always)]
    fn simd_signum(self) -> Self {
        self.signum()
    }

    #[inline(always)]
    fn simd_floor(self) -> Self {
        self.floor()
    }
    #[inline(always)]
    fn simd_ceil(self) -> Self {
        self.ceil()
    }
    #[inline(always)]
    fn simd_round(self) -> Self {
        self.round()
    }
    #[inline(always)]
    fn simd_trunc(self) -> Self {
        self.trunc()
    }
    #[inline(always)]
    fn simd_fract(self) -> Self {
        self.fract()
    }
    #[inline(always)]
    fn simd_mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }

    #[inline(always)]
    fn simd_abs(self) -> Self::SimdRealField {
        self.abs()
    }
    #[inline(always)]
    fn simd_hypot(self, other: Self) -> Self::SimdRealField {
        self.hypot(other)
    }

    #[inline(always)]
    fn simd_recip(self) -> Self {
        self.recip()
    }
    #[inline(always)]
    fn simd_conjugate(self) -> Self {
        self.conjugate()
    }
    #[inline(always)]
    fn simd_sin(self) -> Self {
        self.sin()
    }
    #[inline(always)]
    fn simd_cos(self) -> Self {
        self.cos()
    }
    #[inline(always)]
    fn simd_sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
    #[inline(always)]
    fn simd_sinh_cosh(self) -> (Self, Self) {
        self.sinh_cosh()
    }
    #[inline(always)]
    fn simd_tan(self) -> Self {
        self.tan()
    }
    #[inline(always)]
    fn simd_asin(self) -> Self {
        self.asin()
    }
    #[inline(always)]
    fn simd_acos(self) -> Self {
        self.acos()
    }
    #[inline(always)]
    fn simd_atan(self) -> Self {
        self.atan()
    }
    #[inline(always)]
    fn simd_sinh(self) -> Self {
        self.sinh()
    }
    #[inline(always)]
    fn simd_cosh(self) -> Self {
        self.cosh()
    }
    #[inline(always)]
    fn simd_tanh(self) -> Self {
        self.tanh()
    }
    #[inline(always)]
    fn simd_asinh(self) -> Self {
        self.asinh()
    }
    #[inline(always)]
    fn simd_acosh(self) -> Self {
        self.acosh()
    }
    #[inline(always)]
    fn simd_atanh(self) -> Self {
        self.atanh()
    }

    #[inline(always)]
    fn simd_sinc(self) -> Self {
        self.sinc()
    }
    #[inline(always)]
    fn simd_sinhc(self) -> Self {
        self.sinhc()
    }

    #[inline(always)]
    fn simd_cosc(self) -> Self {
        self.cosc()
    }
    #[inline(always)]
    fn simd_coshc(self) -> Self {
        self.coshc()
    }

    #[inline(always)]
    fn simd_log(self, base: Self::SimdRealField) -> Self {
        self.log(base)
    }
    #[inline(always)]
    fn simd_log2(self) -> Self {
        self.log2()
    }
    #[inline(always)]
    fn simd_log10(self) -> Self {
        self.log10()
    }
    #[inline(always)]
    fn simd_ln(self) -> Self {
        self.ln()
    }
    #[inline(always)]
    fn simd_ln_1p(self) -> Self {
        self.ln_1p()
    }
    #[inline(always)]
    fn simd_sqrt(self) -> Self {
        self.sqrt()
    }
    #[inline(always)]
    fn simd_exp(self) -> Self {
        self.exp()
    }
    #[inline(always)]
    fn simd_exp2(self) -> Self {
        self.exp2()
    }
    #[inline(always)]
    fn simd_exp_m1(self) -> Self {
        self.exp_m1()
    }
    #[inline(always)]
    fn simd_powi(self, n: i32) -> Self {
        self.powi(n)
    }
    #[inline(always)]
    fn simd_powf(self, n: Self::SimdRealField) -> Self {
        self.powf(n)
    }
    #[inline(always)]
    fn simd_powc(self, n: Self) -> Self {
        self.powc(n)
    }
    #[inline(always)]
    fn simd_cbrt(self) -> Self {
        self.cbrt()
    }

    #[inline(always)]
    fn simd_horizontal_sum(self) -> Self::Element {
        self
    }
    #[inline(always)]
    fn simd_horizontal_product(self) -> Self::Element {
        self
    }
}
