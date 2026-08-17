#[cfg(any(feature = "libm", all(feature = "nostd-libm", not(feature = "std"))))]
mod libm_math {
    #[inline(always)]
    pub(crate) fn abs(f: f64) -> f64 {
        libm::fabs(f)
    }

    #[inline(always)]
    pub(crate) fn acos_approx(f: f64) -> f64 {
        libm::acos(f.clamp(-1.0, 1.0))
    }

    #[inline(always)]
    pub(crate) fn atan2(f: f64, other: f64) -> f64 {
        libm::atan2(f, other)
    }

    #[inline(always)]
    pub(crate) fn sin(f: f64) -> f64 {
        libm::sin(f)
    }

    #[inline(always)]
    pub(crate) fn sin_cos(f: f64) -> (f64, f64) {
        libm::sincos(f)
    }

    #[inline(always)]
    pub(crate) fn tan(f: f64) -> f64 {
        libm::tan(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f64) -> f64 {
        libm::sqrt(f)
    }

    #[inline(always)]
    pub(crate) fn copysign(f: f64, sign: f64) -> f64 {
        libm::copysign(f, sign)
    }

    #[inline(always)]
    pub(crate) fn signum(f: f64) -> f64 {
        if f.is_nan() {
            f64::NAN
        } else {
            copysign(1.0, f)
        }
    }

    #[inline(always)]
    pub(crate) fn round(f: f64) -> f64 {
        libm::round(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f64) -> f64 {
        libm::trunc(f)
    }

    #[inline(always)]
    pub(crate) fn ceil(f: f64) -> f64 {
        libm::ceil(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f64) -> f64 {
        libm::floor(f)
    }

    #[inline(always)]
    pub(crate) fn exp(f: f64) -> f64 {
        libm::exp(f)
    }

    #[inline(always)]
    pub(crate) fn powf(f: f64, n: f64) -> f64 {
        libm::pow(f, n)
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f64, b: f64, c: f64) -> f64 {
        libm::fma(a, b, c)
    }

    #[inline]
    pub fn div_euclid(a: f64, b: f64) -> f64 {
        // Based on https://doc.rust-lang.org/src/std/f64.rs.html#293
        let q = libm::trunc(a / b);
        if a % b < 0.0 {
            return if b > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    #[inline]
    pub fn rem_euclid(a: f64, b: f64) -> f64 {
        let r = a % b;
        if r < 0.0 {
            r + abs(b)
        } else {
            r
        }
    }
}

#[cfg(all(not(feature = "libm"), feature = "std"))]
mod std_math {
    #[inline(always)]
    pub(crate) fn abs(f: f64) -> f64 {
        f64::abs(f)
    }

    #[inline(always)]
    pub(crate) fn acos_approx(f: f64) -> f64 {
        f64::acos(f64::clamp(f, -1.0, 1.0))
    }

    #[inline(always)]
    pub(crate) fn atan2(f: f64, other: f64) -> f64 {
        f64::atan2(f, other)
    }

    #[inline(always)]
    pub(crate) fn sin(f: f64) -> f64 {
        f64::sin(f)
    }

    #[inline(always)]
    pub(crate) fn sin_cos(f: f64) -> (f64, f64) {
        f64::sin_cos(f)
    }

    #[inline(always)]
    pub(crate) fn tan(f: f64) -> f64 {
        f64::tan(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f64) -> f64 {
        f64::sqrt(f)
    }

    #[inline(always)]
    pub(crate) fn copysign(f: f64, sign: f64) -> f64 {
        f64::copysign(f, sign)
    }

    #[inline(always)]
    pub(crate) fn signum(f: f64) -> f64 {
        f64::signum(f)
    }

    #[inline(always)]
    pub(crate) fn round(f: f64) -> f64 {
        f64::round(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f64) -> f64 {
        f64::trunc(f)
    }

    #[inline(always)]
    pub(crate) fn ceil(f: f64) -> f64 {
        f64::ceil(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f64) -> f64 {
        f64::floor(f)
    }

    #[inline(always)]
    pub(crate) fn exp(f: f64) -> f64 {
        f64::exp(f)
    }

    #[inline(always)]
    pub(crate) fn powf(f: f64, n: f64) -> f64 {
        f64::powf(f, n)
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f64, b: f64, c: f64) -> f64 {
        f64::mul_add(a, b, c)
    }

    #[inline]
    pub fn div_euclid(a: f64, b: f64) -> f64 {
        f64::div_euclid(a, b)
    }

    #[inline]
    pub fn rem_euclid(a: f64, b: f64) -> f64 {
        f64::rem_euclid(a, b)
    }
}

#[cfg(any(feature = "libm", all(feature = "nostd-libm", not(feature = "std"))))]
pub(crate) use libm_math::*;

#[cfg(all(not(feature = "libm"), feature = "std"))]
pub(crate) use std_math::*;
