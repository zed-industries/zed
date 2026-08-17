use super::*;

impl Trigonometry for f32 {
    #[inline]
    fn sin(self) -> Self {
        ::libm::sinf(self)
    }

    #[inline]
    fn cos(self) -> Self {
        ::libm::cosf(self)
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        ::libm::sincosf(self)
    }

    #[inline]
    fn tan(self) -> Self {
        ::libm::tanf(self)
    }

    #[inline]
    fn asin(self) -> Self {
        ::libm::asinf(self)
    }

    #[inline]
    fn acos(self) -> Self {
        ::libm::acosf(self)
    }

    #[inline]
    fn atan(self) -> Self {
        ::libm::atanf(self)
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        ::libm::atan2f(self, other)
    }
}

impl Trigonometry for f64 {
    #[inline]
    fn sin(self) -> Self {
        ::libm::sin(self)
    }

    #[inline]
    fn cos(self) -> Self {
        ::libm::cos(self)
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        ::libm::sincos(self)
    }

    #[inline]
    fn tan(self) -> Self {
        ::libm::tan(self)
    }

    #[inline]
    fn asin(self) -> Self {
        ::libm::asin(self)
    }

    #[inline]
    fn acos(self) -> Self {
        ::libm::acos(self)
    }

    #[inline]
    fn atan(self) -> Self {
        ::libm::atan(self)
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        ::libm::atan2(self, other)
    }
}

impl Abs for f32 {
    #[inline]
    fn abs(self) -> Self {
        ::libm::fabsf(self)
    }
}

impl Abs for f64 {
    #[inline]
    fn abs(self) -> Self {
        ::libm::fabs(self)
    }
}

impl Sqrt for f32 {
    #[inline]
    fn sqrt(self) -> Self {
        ::libm::sqrtf(self)
    }
}

impl Sqrt for f64 {
    #[inline]
    fn sqrt(self) -> Self {
        ::libm::sqrt(self)
    }
}

impl Cbrt for f32 {
    #[inline]
    fn cbrt(self) -> Self {
        ::libm::cbrtf(self)
    }
}

impl Cbrt for f64 {
    #[inline]
    fn cbrt(self) -> Self {
        ::libm::cbrt(self)
    }
}

impl Powf for f32 {
    #[inline]
    fn powf(self, exp: Self) -> Self {
        ::libm::powf(self, exp)
    }
}

impl Powf for f64 {
    #[inline]
    fn powf(self, exp: Self) -> Self {
        ::libm::pow(self, exp)
    }
}

impl Powi for f32 {
    #[inline]
    fn powi(mut self, mut exp: i32) -> Self {
        if exp < 0 {
            exp = exp.wrapping_neg();
            self = self.recip();
        }

        Powu::powu(self, exp as u32)
    }
}

impl Powi for f64 {
    #[inline]
    fn powi(mut self, mut exp: i32) -> Self {
        if exp < 0 {
            exp = exp.wrapping_neg();
            self = self.recip();
        }

        Powu::powu(self, exp as u32)
    }
}

impl Recip for f32 {
    #[inline]
    fn recip(self) -> Self {
        1.0 / self
    }
}

impl Recip for f64 {
    #[inline]
    fn recip(self) -> Self {
        1.0 / self
    }
}

impl Exp for f32 {
    #[inline]
    fn exp(self) -> Self {
        ::libm::expf(self)
    }
}

impl Exp for f64 {
    #[inline]
    fn exp(self) -> Self {
        ::libm::exp(self)
    }
}

impl Hypot for f32 {
    #[inline]
    fn hypot(self, other: Self) -> Self {
        ::libm::hypotf(self, other)
    }
}

impl Hypot for f64 {
    #[inline]
    fn hypot(self, other: Self) -> Self {
        ::libm::hypot(self, other)
    }
}

impl Round for f32 {
    #[inline]
    fn round(self) -> Self {
        ::libm::roundf(self)
    }

    #[inline]
    fn floor(self) -> Self {
        ::libm::floorf(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        ::libm::ceilf(self)
    }
}

impl Round for f64 {
    #[inline]
    fn round(self) -> Self {
        ::libm::round(self)
    }

    #[inline]
    fn floor(self) -> Self {
        ::libm::floor(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        ::libm::ceil(self)
    }
}

impl MulAdd for f32 {
    #[inline]
    fn mul_add(self, m: Self, a: Self) -> Self {
        ::libm::fmaf(self, m, a)
    }
}

impl MulAdd for f64 {
    #[inline]
    fn mul_add(self, m: Self, a: Self) -> Self {
        ::libm::fma(self, m, a)
    }
}

impl Signum for f32 {
    #[inline]
    fn signum(self) -> Self {
        if self.is_nan() {
            Self::NAN
        } else {
            ::libm::copysignf(1.0, self)
        }
    }
}

impl Signum for f64 {
    #[inline]
    fn signum(self) -> Self {
        if self.is_nan() {
            Self::NAN
        } else {
            ::libm::copysign(1.0, self)
        }
    }
}

impl Ln for f32 {
    #[inline]
    fn ln(self) -> Self {
        ::libm::logf(self)
    }
}

impl Ln for f64 {
    #[inline]
    fn ln(self) -> Self {
        ::libm::log(self)
    }
}
