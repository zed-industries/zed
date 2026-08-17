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
    pub(crate) fn exp2(f: f64) -> f64 {
        libm::exp2(f)
    }

    #[inline(always)]
    pub(crate) fn ln(f: f64) -> f64 {
        libm::log(f)
    }

    #[inline(always)]
    pub(crate) fn log2(f: f64) -> f64 {
        libm::log2(f)
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
    pub(crate) fn exp2(f: f64) -> f64 {
        f64::exp2(f)
    }

    #[inline(always)]
    pub(crate) fn ln(f: f64) -> f64 {
        f64::ln(f)
    }

    #[inline(always)]
    pub(crate) fn log2(f: f64) -> f64 {
        f64::log2(f)
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

// Used to reduce the number of compilation errors, in the event that no other
// math backend is specified.
#[cfg(all(
    not(feature = "libm"),
    not(feature = "std"),
    not(feature = "nostd-libm")
))]
mod no_backend_math {
    pub(crate) fn abs(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn acos_approx(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn atan2(_: f64, _: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn sin(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn sin_cos(_: f64) -> (f64, f64) {
        unimplemented!()
    }

    pub(crate) fn tan(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn sqrt(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn copysign(_: f64, _: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn signum(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn round(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn trunc(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn ceil(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn floor(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn exp(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn exp2(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn ln(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn log2(_: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn powf(_: f64, _: f64) -> f64 {
        unimplemented!()
    }

    pub(crate) fn mul_add(_: f64, _: f64, _: f64) -> f64 {
        unimplemented!()
    }

    pub fn div_euclid(_: f64, _: f64) -> f64 {
        unimplemented!()
    }

    pub fn rem_euclid(_: f64, _: f64) -> f64 {
        unimplemented!()
    }
}

#[cfg(any(feature = "libm", all(feature = "nostd-libm", not(feature = "std"))))]
pub(crate) use libm_math::*;

#[cfg(all(not(feature = "libm"), feature = "std"))]
pub(crate) use std_math::*;

#[cfg(all(
    not(feature = "libm"),
    not(feature = "std"),
    not(feature = "nostd-libm")
))]
pub(crate) use no_backend_math::*;
