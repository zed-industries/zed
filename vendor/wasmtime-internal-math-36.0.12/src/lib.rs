//! A minimal helper crate for implementing float-related operations for
//! WebAssembly in terms of the native platform primitives.
//!
//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.
//!
//! This crate is intended to assist with solving the portability issues such
//! as:
//!
//! * Functions like `f32::trunc` are not available in `#![no_std]` targets.
//! * The `f32::trunc` function is likely faster than the `libm` fallback.
//! * Behavior of `f32::trunc` differs across platforms, for example it's
//!   different on Windows and glibc on Linux. Additionally riscv64's
//!   implementation of `libm` seems to have different NaN behavior than other
//!   platforms.
//! * Some wasm functions are in the Rust standard library, but not stable yet.
//!
//! There are a few locations throughout the codebase that these functions are
//! needed so they're implemented only in a single location here rather than
//! multiple.

#![no_std]

#[cfg(feature = "std")]
extern crate std;

/// Returns the bounds for guarding a trapping f32-to-int conversion.
///
/// This function will return two floats, a lower bound and an upper bound,
/// which can be used to test whether a WebAssembly f32-to-int conversion
/// should trap. The float being converted must be greater than the lower bound
/// and less than the upper bound for the conversion to proceed, otherwise a
/// trap or infinity value should be generated.
///
/// The `signed` argument indicates whether a conversion to a signed integer is
/// happening. If `false` a conversion to an unsigned integer is happening. The
/// `out_bits` argument indicates how many bits are in the integer being
/// converted to.
pub const fn f32_cvt_to_int_bounds(signed: bool, out_bits: u32) -> (f32, f32) {
    match (signed, out_bits) {
        (true, 8) => (i8::min_value() as f32 - 1., i8::max_value() as f32 + 1.),
        (true, 16) => (i16::min_value() as f32 - 1., i16::max_value() as f32 + 1.),
        (true, 32) => (-2147483904.0, 2147483648.0),
        (true, 64) => (-9223373136366403584.0, 9223372036854775808.0),
        (false, 8) => (-1., u8::max_value() as f32 + 1.),
        (false, 16) => (-1., u16::max_value() as f32 + 1.),
        (false, 32) => (-1., 4294967296.0),
        (false, 64) => (-1., 18446744073709551616.0),
        _ => unreachable!(),
    }
}

/// Same as [`f32_cvt_to_int_bounds`] but used for f64-to-int conversions.
pub const fn f64_cvt_to_int_bounds(signed: bool, out_bits: u32) -> (f64, f64) {
    match (signed, out_bits) {
        (true, 8) => (i8::min_value() as f64 - 1., i8::max_value() as f64 + 1.),
        (true, 16) => (i16::min_value() as f64 - 1., i16::max_value() as f64 + 1.),
        (true, 32) => (-2147483649.0, 2147483648.0),
        (true, 64) => (-9223372036854777856.0, 9223372036854775808.0),
        (false, 8) => (-1., u8::max_value() as f64 + 1.),
        (false, 16) => (-1., u16::max_value() as f64 + 1.),
        (false, 32) => (-1., 4294967296.0),
        (false, 64) => (-1., 18446744073709551616.0),
        _ => unreachable!(),
    }
}

pub trait WasmFloat {
    fn wasm_trunc(self) -> Self;
    fn wasm_copysign(self, sign: Self) -> Self;
    fn wasm_floor(self) -> Self;
    fn wasm_ceil(self) -> Self;
    fn wasm_sqrt(self) -> Self;
    fn wasm_abs(self) -> Self;
    fn wasm_nearest(self) -> Self;
    fn wasm_minimum(self, other: Self) -> Self;
    fn wasm_maximum(self, other: Self) -> Self;
    fn wasm_mul_add(self, b: Self, c: Self) -> Self;
}

impl WasmFloat for f32 {
    #[inline]
    fn wasm_trunc(self) -> f32 {
        if self.is_nan() {
            return f32::NAN;
        }
        #[cfg(feature = "std")]
        if !cfg!(windows) && !cfg!(target_arch = "riscv64") {
            return self.trunc();
        }
        libm::truncf(self)
    }
    #[inline]
    fn wasm_copysign(self, sign: f32) -> f32 {
        #[cfg(feature = "std")]
        if true {
            return self.copysign(sign);
        }
        libm::copysignf(self, sign)
    }
    #[inline]
    fn wasm_floor(self) -> f32 {
        if self.is_nan() {
            return f32::NAN;
        }
        #[cfg(feature = "std")]
        if !cfg!(target_arch = "riscv64") {
            return self.floor();
        }
        libm::floorf(self)
    }
    #[inline]
    fn wasm_ceil(self) -> f32 {
        if self.is_nan() {
            return f32::NAN;
        }
        #[cfg(feature = "std")]
        if !cfg!(target_arch = "riscv64") {
            return self.ceil();
        }
        libm::ceilf(self)
    }
    #[inline]
    fn wasm_sqrt(self) -> f32 {
        #[cfg(feature = "std")]
        if true {
            return self.sqrt();
        }
        libm::sqrtf(self)
    }
    #[inline]
    fn wasm_abs(self) -> f32 {
        #[cfg(feature = "std")]
        if true {
            return self.abs();
        }
        libm::fabsf(self)
    }
    #[inline]
    fn wasm_nearest(self) -> f32 {
        if self.is_nan() {
            return f32::NAN;
        }
        #[cfg(feature = "std")]
        if !cfg!(windows) && !cfg!(target_arch = "riscv64") {
            return self.round_ties_even();
        }
        let round = libm::roundf(self);
        if libm::fabsf(self - round) != 0.5 {
            return round;
        }
        match round % 2.0 {
            1.0 => libm::floorf(self),
            -1.0 => libm::ceilf(self),
            _ => round,
        }
    }
    #[inline]
    fn wasm_maximum(self, other: f32) -> f32 {
        // FIXME: replace this with `a.maximum(b)` when rust-lang/rust#91079 is
        // stabilized
        if self > other {
            self
        } else if other > self {
            other
        } else if self == other {
            if self.is_sign_positive() && other.is_sign_negative() {
                self
            } else {
                other
            }
        } else {
            self + other
        }
    }
    #[inline]
    fn wasm_minimum(self, other: f32) -> f32 {
        // FIXME: replace this with `self.minimum(other)` when
        // rust-lang/rust#91079 is stabilized
        if self < other {
            self
        } else if other < self {
            other
        } else if self == other {
            if self.is_sign_negative() && other.is_sign_positive() {
                self
            } else {
                other
            }
        } else {
            self + other
        }
    }
    #[inline]
    fn wasm_mul_add(self, b: f32, c: f32) -> f32 {
        // The MinGW implementation of `fma` differs from other platforms, so
        // favor `libm` there instead.
        #[cfg(feature = "std")]
        if !(cfg!(windows) && cfg!(target_env = "gnu")) {
            return self.mul_add(b, c);
        }
        libm::fmaf(self, b, c)
    }
}

impl WasmFloat for f64 {
    #[inline]
    fn wasm_trunc(self) -> f64 {
        if self.is_nan() {
            return f64::NAN;
        }
        #[cfg(feature = "std")]
        if !cfg!(windows) && !cfg!(target_arch = "riscv64") {
            return self.trunc();
        }
        libm::trunc(self)
    }
    #[inline]
    fn wasm_copysign(self, sign: f64) -> f64 {
        #[cfg(feature = "std")]
        if true {
            return self.copysign(sign);
        }
        libm::copysign(self, sign)
    }
    #[inline]
    fn wasm_floor(self) -> f64 {
        if self.is_nan() {
            return f64::NAN;
        }
        #[cfg(feature = "std")]
        if !cfg!(target_arch = "riscv64") {
            return self.floor();
        }
        libm::floor(self)
    }
    #[inline]
    fn wasm_ceil(self) -> f64 {
        if self.is_nan() {
            return f64::NAN;
        }
        #[cfg(feature = "std")]
        if !cfg!(target_arch = "riscv64") {
            return self.ceil();
        }
        libm::ceil(self)
    }
    #[inline]
    fn wasm_sqrt(self) -> f64 {
        #[cfg(feature = "std")]
        if true {
            return self.sqrt();
        }
        libm::sqrt(self)
    }
    #[inline]
    fn wasm_abs(self) -> f64 {
        #[cfg(feature = "std")]
        if true {
            return self.abs();
        }
        libm::fabs(self)
    }
    #[inline]
    fn wasm_nearest(self) -> f64 {
        if self.is_nan() {
            return f64::NAN;
        }
        #[cfg(feature = "std")]
        if !cfg!(windows) && !cfg!(target_arch = "riscv64") {
            return self.round_ties_even();
        }
        let round = libm::round(self);
        if libm::fabs(self - round) != 0.5 {
            return round;
        }
        match round % 2.0 {
            1.0 => libm::floor(self),
            -1.0 => libm::ceil(self),
            _ => round,
        }
    }
    #[inline]
    fn wasm_maximum(self, other: f64) -> f64 {
        // FIXME: replace this with `a.maximum(b)` when rust-lang/rust#91079 is
        // stabilized
        if self > other {
            self
        } else if other > self {
            other
        } else if self == other {
            if self.is_sign_positive() && other.is_sign_negative() {
                self
            } else {
                other
            }
        } else {
            self + other
        }
    }
    #[inline]
    fn wasm_minimum(self, other: f64) -> f64 {
        // FIXME: replace this with `self.minimum(other)` when
        // rust-lang/rust#91079 is stabilized
        if self < other {
            self
        } else if other < self {
            other
        } else if self == other {
            if self.is_sign_negative() && other.is_sign_positive() {
                self
            } else {
                other
            }
        } else {
            self + other
        }
    }
    #[inline]
    fn wasm_mul_add(self, b: f64, c: f64) -> f64 {
        // The MinGW implementation of `fma` differs from other platforms, so
        // favor `libm` there instead.
        #[cfg(feature = "std")]
        if !(cfg!(windows) && cfg!(target_env = "gnu")) {
            return self.mul_add(b, c);
        }
        libm::fma(self, b, c)
    }
}
