use ::wide::{f32x4, f32x8, f64x2, f64x4, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

use super::*;

macro_rules! impl_wide_float {
    ($($ty: ident { array: [$scalar: ident; $n: expr], powf: $pow_self: ident, }),+) => {
        $(
            impl Real for $ty {
                #[inline]
                fn from_f64(n: f64) -> $ty {
                    $ty::splat(n as $scalar)
                }

            }

            impl FromScalar for $ty {
                type Scalar = $scalar;

                #[inline]
                fn from_scalar(scalar: $scalar) -> Self {
                    $ty::splat(scalar)
                }
            }

            impl FromScalarArray<$n> for $ty {
                #[inline]
                fn from_array(scalars: [$scalar; $n]) -> Self {
                    scalars.into()
                }
            }

            impl IntoScalarArray<$n> for $ty {
                #[inline]
                fn into_array(self) -> [$scalar; $n] {
                    self.into()
                }
            }

            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    $ty::ZERO
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    $ty::ONE
                }
            }

            impl MinMax for $ty {
                #[inline]
                fn max(self, other: Self) -> Self {
                    $ty::max(self, other)
                }

                #[inline]
                fn min(self, other: Self) -> Self {
                    $ty::min(self, other)
                }

                #[inline]
                fn min_max(self, other: Self) -> (Self, Self) {
                    ($ty::min(self, other), $ty::max(self, other))
                }
            }

            impl Powu for $ty {
                #[inline]
                fn powu(self, exp: u32) -> Self {
                    pow(self, exp)
                }
            }

            impl IsValidDivisor for $ty {
                #[inline]
                fn is_valid_divisor(&self) -> Self {
                    !self.cmp_eq($ty::ZERO)
                }
            }

            impl Trigonometry for $ty {
                #[inline]
                fn sin(self) -> Self {
                    $ty::sin(self)
                }

                #[inline]
                fn cos(self) -> Self {
                    $ty::cos(self)
                }

                #[inline]
                fn sin_cos(self) -> (Self, Self) {
                    $ty::sin_cos(self)
                }

                #[inline]
                fn tan(self) -> Self {
                    $ty::tan(self)
                }

                #[inline]
                fn asin(self) -> Self {
                    $ty::asin(self)
                }

                #[inline]
                fn acos(self) -> Self {
                    $ty::acos(self)
                }

                #[inline]
                fn atan(self) -> Self {
                    $ty::atan(self)
                }

                #[inline]
                fn atan2(self, other: Self) -> Self {
                    $ty::atan2(self, other)
                }
            }

            impl Abs for $ty {
                #[inline]
                fn abs(self) -> Self {
                    $ty::abs(self)
                }
            }

            impl Sqrt for $ty {
                #[inline]
                fn sqrt(self) -> Self {
                    $ty::sqrt(self)
                }
            }

            impl Cbrt for $ty {
                #[inline]
                fn cbrt(self) -> Self {
                    let mut array = self.into_array();

                    for scalar in &mut array {
                        *scalar = scalar.cbrt();
                    }

                    array.into()
                }
            }

            impl Powf for $ty {
                #[inline]
                fn powf(self, exp: Self) -> Self {
                    $ty::$pow_self(self, exp)
                }
            }

            impl Powi for $ty {
                #[inline]
                fn powi(mut self, mut exp: i32) -> Self {
                    if exp < 0 {
                        exp = exp.wrapping_neg();
                        self = self.recip();
                    }

                    Powu::powu(self, exp as u32)
                }
            }

            // impl Recip for $ty {
            //     #[inline]
            //     fn recip(self) -> Self {
            //         $ty::recip(self)
            //     }
            // }

            impl Exp for $ty {
                #[inline]
                fn exp(self) -> Self {
                    $ty::exp(self)
                }
            }

            impl Hypot for $ty {
                #[inline]
                fn hypot(self, other: Self) -> Self {
                    (self * self + other * other).sqrt()
                }
            }

            impl Round for $ty {
                #[inline]
                fn round(self) -> Self {
                    $ty::round(self)
                }

                #[inline]
                fn floor(self) -> Self {
                    let mut array = self.into_array();

                    for scalar in &mut array {
                        *scalar = scalar.floor();
                    }

                    array.into()
                }

                #[inline]
                fn ceil(self) -> Self {
                    let mut array = self.into_array();

                    for scalar in &mut array {
                        *scalar = scalar.ceil();
                    }

                    array.into()
                }
            }

            impl Clamp for $ty {
                #[inline]
                fn clamp(self, min: Self, max: Self) -> Self {
                    self.min(max).max(min)
                }

                #[inline]
                fn clamp_min(self, min: Self) -> Self {
                    $ty::max(self, min)
                }

                #[inline]
                fn clamp_max(self, max: Self) -> Self {
                    $ty::min(self, max)
                }
            }

            impl ClampAssign for $ty {
                #[inline]
                fn clamp_assign(&mut self, min: Self, max: Self) {
                    *self = $ty::clamp(*self, min, max);
                }

                #[inline]
                fn clamp_min_assign(&mut self, min: Self) {
                    *self = $ty::max(*self, min);
                }

                #[inline]
                fn clamp_max_assign(&mut self, max: Self) {
                    *self = $ty::min(*self, max);
                }
            }

            impl PartialCmp for $ty {
                #[inline]
                fn lt(&self, other: &Self) -> Self::Mask {
                    self.cmp_lt(*other)
                }

                #[inline]
                fn lt_eq(&self, other: &Self) -> Self::Mask {
                    self.cmp_le(*other)
                }

                #[inline]
                fn eq(&self, other: &Self) -> Self::Mask {
                    self.cmp_eq(*other)
                }

                #[inline]
                fn neq(&self, other: &Self) -> Self::Mask {
                    self.cmp_ne(*other)
                }

                #[inline]
                fn gt_eq(&self, other: &Self) -> Self::Mask {
                    self.cmp_ge(*other)
                }

                #[inline]
                fn gt(&self, other: &Self) -> Self::Mask {
                    self.cmp_gt(*other)
                }
            }

            impl MulAdd for $ty {
                #[inline]
                fn mul_add(self, m: Self, a: Self) -> Self {
                    $ty::mul_add(self, m, a)
                }
            }

            impl MulSub for $ty {
                #[inline]
                fn mul_sub(self, m: Self, s: Self) -> Self {
                    $ty::mul_sub(self, m, s)
                }
            }

            impl Signum for $ty {
                #[inline]
                fn signum(self) -> Self {
                    self.is_nan().blend(Self::from($scalar::NAN), $ty::copysign(Self::from(1.0), self))
                }
            }

            impl Ln for $ty {
                #[inline]
                fn ln(self) -> Self {
                    self.ln()
                }
            }
        )+
    };
}

impl_wide_float!(
    f32x4 {
        array: [f32; 4],
        powf: pow_f32x4,
    },
    f32x8 {
        array: [f32; 8],
        powf: pow_f32x8,
    },
    f64x2 {
        array: [f64; 2],
        powf: pow_f64x2,
    },
    f64x4 {
        array: [f64; 4],
        powf: pow_f64x4,
    }
);

impl Recip for f32x4 {
    #[inline]
    fn recip(self) -> Self {
        f32x4::recip(self)
    }
}

impl Recip for f32x8 {
    #[inline]
    fn recip(self) -> Self {
        f32x8::recip(self)
    }
}

impl Recip for f64x2 {
    #[inline]
    fn recip(self) -> Self {
        f64x2::ONE / self
    }
}

impl Recip for f64x4 {
    #[inline]
    fn recip(self) -> Self {
        f64x4::ONE / self
    }
}
