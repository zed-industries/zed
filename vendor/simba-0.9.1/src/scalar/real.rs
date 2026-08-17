use num::Signed;
use std::{f32, f64};

use approx::{RelativeEq, UlpsEq};

use crate::scalar::ComplexField;

#[cfg(all(not(feature = "std"), not(feature = "libm_force"), feature = "libm"))]
use num::Float;
//#[cfg(feature = "decimal")]
//use decimal::d128;

/// Trait shared by all reals.
#[allow(missing_docs)]
pub trait RealField:
    ComplexField<RealField = Self>
    + RelativeEq<Epsilon = Self>
    + UlpsEq<Epsilon = Self>
    + Signed
    + PartialOrd
{
    /// Is the sign of this real number positive?
    fn is_sign_positive(&self) -> bool;
    /// Is the sign of this real number negative?
    fn is_sign_negative(&self) -> bool;
    /// Copies the sign of `sign` to `self`.
    ///
    /// - Returns `self.simd_abs()` if `sign` is positive or positive-zero.
    /// - Returns `-self.simd_abs()` if `sign` is negative or negative-zero.
    fn copysign(self, sign: Self) -> Self;

    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn atan2(self, other: Self) -> Self;

    /// The smallest finite positive value representable using this type.
    fn min_value() -> Option<Self>;
    /// The largest finite positive value representable using this type.
    fn max_value() -> Option<Self>;

    fn pi() -> Self;
    fn two_pi() -> Self;
    fn frac_pi_2() -> Self;
    fn frac_pi_3() -> Self;
    fn frac_pi_4() -> Self;
    fn frac_pi_6() -> Self;
    fn frac_pi_8() -> Self;
    fn frac_1_pi() -> Self;
    fn frac_2_pi() -> Self;
    fn frac_2_sqrt_pi() -> Self;

    fn e() -> Self;
    fn log2_e() -> Self;
    fn log10_e() -> Self;
    fn ln_2() -> Self;
    fn ln_10() -> Self;
}

macro_rules! impl_real (
    ($($T:ty, $M:ident, $cpysgn_mod: ident, $atan_mod: ident);*) => ($(
        impl RealField for $T {
            #[inline]
            fn is_sign_positive(&self) -> bool {
                $M::is_sign_positive(*self)
            }

            #[inline]
            fn is_sign_negative(&self) -> bool {
                $M::is_sign_negative(*self)
            }

            #[inline(always)]
            fn copysign(self, sign: Self) -> Self {
                $cpysgn_mod::copysign(self, sign)
            }

            #[inline]
            fn max(self, other: Self) -> Self {
                $M::max(self, other)
            }

            #[inline]
            fn min(self, other: Self) -> Self {
                $M::min(self, other)
            }

            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self {
                if self < min {
                    min
                } else if self > max {
                    max
                } else {
                    self
                }
            }

            #[inline]
            fn atan2(self, other: Self) -> Self {
                $atan_mod::atan2(self, other)
            }


            /// The smallest finite positive value representable using this type.
            #[inline]
            fn min_value() -> Option<Self> {
                Some($M::MIN)
            }

            /// The largest finite positive value representable using this type.
            #[inline]
            fn max_value() -> Option<Self> {
                Some($M::MAX)
            }

            /// Archimedes' constant.
            #[inline]
            fn pi() -> Self {
                $M::consts::PI
            }

            /// 2.0 * pi.
            #[inline]
            fn two_pi() -> Self {
                $M::consts::PI + $M::consts::PI
            }

            /// pi / 2.0.
            #[inline]
            fn frac_pi_2() -> Self {
                $M::consts::FRAC_PI_2
            }

            /// pi / 3.0.
            #[inline]
            fn frac_pi_3() -> Self {
                $M::consts::FRAC_PI_3
            }

            /// pi / 4.0.
            #[inline]
            fn frac_pi_4() -> Self {
                $M::consts::FRAC_PI_4
            }

            /// pi / 6.0.
            #[inline]
            fn frac_pi_6() -> Self {
                $M::consts::FRAC_PI_6
            }

            /// pi / 8.0.
            #[inline]
            fn frac_pi_8() -> Self {
                $M::consts::FRAC_PI_8
            }

            /// 1.0 / pi.
            #[inline]
            fn frac_1_pi() -> Self {
                $M::consts::FRAC_1_PI
            }

            /// 2.0 / pi.
            #[inline]
            fn frac_2_pi() -> Self {
                $M::consts::FRAC_2_PI
            }

            /// 2.0 / sqrt(pi).
            #[inline]
            fn frac_2_sqrt_pi() -> Self {
                $M::consts::FRAC_2_SQRT_PI
            }


            /// Euler's number.
            #[inline]
            fn e() -> Self {
                $M::consts::E
            }

            /// log2(e).
            #[inline]
            fn log2_e() -> Self {
                $M::consts::LOG2_E
            }

            /// log10(e).
            #[inline]
            fn log10_e() -> Self {
                $M::consts::LOG10_E
            }

            /// ln(2.0).
            #[inline]
            fn ln_2() -> Self {
                $M::consts::LN_2
            }

            /// ln(10.0).
            #[inline]
            fn ln_10() -> Self {
                $M::consts::LN_10
            }
        }
    )*)
);

#[cfg(all(not(feature = "std"), not(feature = "libm_force"), feature = "libm"))]
impl_real!(f32, f32, Float, Float; f64, f64, Float, Float);
#[cfg(all(feature = "std", not(feature = "libm_force")))]
impl_real!(f32, f32, f32, f32; f64, f64, f64, f64);
#[cfg(feature = "libm_force")]
impl_real!(f32, f32, libm_force_f32, libm_force_f32; f64, f64, libm_force, libm_force);

// We use this dummy module to remove the 'f' suffix at the end of
// each libm functions to make our generic Real/ComplexField impl
// macros work.
#[cfg(feature = "libm_force")]
mod libm_force_f32 {
    #[inline(always)]
    pub fn atan2(y: f32, x: f32) -> f32 {
        libm_force::atan2f(y, x)
    }

    #[inline(always)]
    pub fn copysign(x: f32, y: f32) -> f32 {
        libm_force::copysignf(x, y)
    }
}

//#[cfg(feature = "decimal")]
//impl_real!(d128, d128, d128);
