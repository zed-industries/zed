/// Returns a very close approximation of `self.clamp(-1.0, 1.0).acos()`.
#[inline]
fn acos_approx_f32(v: f32) -> f32 {
    // Based on https://github.com/microsoft/DirectXMath `XMScalarAcos`
    // Clamp input to [-1,1].
    let nonnegative = v >= 0.0;
    let x = abs(v);
    let mut omx = 1.0 - x;
    if omx < 0.0 {
        omx = 0.0;
    }
    let root = sqrt(omx);

    // 7-degree minimax approximation
    #[allow(clippy::approx_constant)]
    let mut result =
        ((((((-0.001_262_491_1 * x + 0.006_670_09) * x - 0.017_088_126) * x + 0.030_891_88) * x
            - 0.050_174_303)
            * x
            + 0.088_978_99)
            * x
            - 0.214_598_8)
            * x
            + 1.570_796_3;
    result *= root;

    // acos(x) = pi - acos(-x) when x < 0
    if nonnegative {
        result
    } else {
        core::f32::consts::PI - result
    }
}

#[cfg(feature = "libm")]
mod libm_math {
    #[inline(always)]
    pub(crate) fn abs(f: f32) -> f32 {
        libm::fabsf(f)
    }

    #[inline(always)]
    pub(crate) fn acos_approx(f: f32) -> f32 {
        super::acos_approx_f32(f)
    }

    #[inline(always)]
    pub(crate) fn asin(f: f32) -> f32 {
        libm::asinf(f)
    }

    #[inline(always)]
    pub(crate) fn atan2(f: f32, other: f32) -> f32 {
        libm::atan2f(f, other)
    }

    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn sin(f: f32) -> f32 {
        libm::sinf(f)
    }

    #[inline(always)]
    pub(crate) fn sin_cos(f: f32) -> (f32, f32) {
        libm::sincosf(f)
    }

    #[inline(always)]
    pub(crate) fn tan(f: f32) -> f32 {
        libm::tanf(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f32) -> f32 {
        libm::sqrtf(f)
    }

    #[inline(always)]
    pub(crate) fn copysign(f: f32, sign: f32) -> f32 {
        libm::copysignf(f, sign)
    }

    #[inline(always)]
    pub(crate) fn signum(f: f32) -> f32 {
        if f.is_nan() {
            f32::NAN
        } else {
            copysign(1.0, f)
        }
    }

    #[inline(always)]
    pub(crate) fn round(f: f32) -> f32 {
        libm::roundf(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f32) -> f32 {
        libm::truncf(f)
    }

    #[inline(always)]
    pub(crate) fn ceil(f: f32) -> f32 {
        libm::ceilf(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f32) -> f32 {
        libm::floorf(f)
    }

    #[inline(always)]
    pub(crate) fn exp(f: f32) -> f32 {
        libm::expf(f)
    }

    #[inline(always)]
    pub(crate) fn powf(f: f32, n: f32) -> f32 {
        libm::powf(f, n)
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f32, b: f32, c: f32) -> f32 {
        libm::fmaf(a, b, c)
    }

    #[inline]
    pub fn div_euclid(a: f32, b: f32) -> f32 {
        // Based on https://doc.rust-lang.org/src/std/f32.rs.html#293
        let q = libm::truncf(a / b);
        if a % b < 0.0 {
            return if b > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    #[inline]
    pub fn rem_euclid(a: f32, b: f32) -> f32 {
        let r = a % b;
        if r < 0.0 {
            r + abs(b)
        } else {
            r
        }
    }
}

#[cfg(not(feature = "libm"))]
mod std_math {
    #[inline(always)]
    pub(crate) fn abs(f: f32) -> f32 {
        f32::abs(f)
    }

    #[inline(always)]
    pub(crate) fn acos_approx(f: f32) -> f32 {
        super::acos_approx_f32(f)
    }

    #[inline(always)]
    pub(crate) fn asin(f: f32) -> f32 {
        f32::asin(f)
    }

    #[inline(always)]
    pub(crate) fn atan2(f: f32, other: f32) -> f32 {
        f32::atan2(f, other)
    }

    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn sin(f: f32) -> f32 {
        f32::sin(f)
    }

    #[inline(always)]
    pub(crate) fn sin_cos(f: f32) -> (f32, f32) {
        f32::sin_cos(f)
    }

    #[inline(always)]
    pub(crate) fn tan(f: f32) -> f32 {
        f32::tan(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f32) -> f32 {
        f32::sqrt(f)
    }

    #[inline(always)]
    pub(crate) fn copysign(f: f32, sign: f32) -> f32 {
        f32::copysign(f, sign)
    }

    #[inline(always)]
    pub(crate) fn signum(f: f32) -> f32 {
        f32::signum(f)
    }

    #[inline(always)]
    pub(crate) fn round(f: f32) -> f32 {
        f32::round(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f32) -> f32 {
        f32::trunc(f)
    }

    #[inline(always)]
    pub(crate) fn ceil(f: f32) -> f32 {
        f32::ceil(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f32) -> f32 {
        f32::floor(f)
    }

    #[inline(always)]
    pub(crate) fn exp(f: f32) -> f32 {
        f32::exp(f)
    }

    #[inline(always)]
    pub(crate) fn powf(f: f32, n: f32) -> f32 {
        f32::powf(f, n)
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f32, b: f32, c: f32) -> f32 {
        f32::mul_add(a, b, c)
    }

    #[inline]
    pub fn div_euclid(a: f32, b: f32) -> f32 {
        f32::div_euclid(a, b)
    }

    #[inline]
    pub fn rem_euclid(a: f32, b: f32) -> f32 {
        f32::rem_euclid(a, b)
    }
}

#[cfg(feature = "libm")]
pub(crate) use libm_math::*;

#[cfg(not(feature = "libm"))]
pub(crate) use std_math::*;
