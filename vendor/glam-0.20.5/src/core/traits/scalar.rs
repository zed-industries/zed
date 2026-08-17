// Wait until this bug is fix and float cmp to zero don't report a warning.
// https://github.com/rust-lang/rust-clippy/issues/3804
#![allow(clippy::float_cmp)]
// num_traits is optional as it adds 70% to compile times. It is needed by no_std builds
#[cfg(feature = "libm")]
pub use num_traits::{Float, Num, Signed};

use core::{
    marker::Sized,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub},
};

// Stub the necessary parts of num traits
#[cfg(not(feature = "libm"))]
pub trait Num: PartialEq {}

#[cfg(not(feature = "libm"))]
pub trait Signed: Sized + Num + core::ops::Neg<Output = Self> {
    fn abs(self) -> Self;
    fn signum(self) -> Self;
}

#[cfg(not(feature = "libm"))]
pub trait Float: Num + Copy + core::ops::Neg<Output = Self> {
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn ceil(self) -> Self;
    fn exp(self) -> Self;
    fn floor(self) -> Self;
    fn is_finite(self) -> bool;
    fn is_nan(self) -> bool;
    fn mul_add(self, b: Self, c: Self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn recip(self) -> Self;
    fn round(self) -> Self;
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn tan(self) -> Self;
}

#[cfg(not(feature = "libm"))]
macro_rules! impl_num_trait {
    ($t:ident) => {
        impl Num for $t {}
    };
}

#[cfg(not(feature = "libm"))]
macro_rules! impl_signed_trait {
    ($t:ident) => {
        impl_num_trait!($t);

        impl Signed for $t {
            #[inline(always)]
            fn abs(self) -> Self {
                $t::abs(self)
            }
            #[inline(always)]
            fn signum(self) -> Self {
                $t::signum(self)
            }
        }
    };
}

#[cfg(not(feature = "libm"))]
macro_rules! impl_float_trait {
    ($t:ident) => {
        impl_signed_trait!($t);

        impl Float for $t {
            #[inline(always)]
            fn asin(self) -> Self {
                $t::asin(self)
            }
            #[inline(always)]
            fn acos(self) -> Self {
                $t::acos(self)
            }
            #[inline(always)]
            fn ceil(self) -> Self {
                $t::ceil(self)
            }
            #[inline(always)]
            fn exp(self) -> Self {
                $t::exp(self)
            }
            #[inline(always)]
            fn floor(self) -> Self {
                $t::floor(self)
            }
            #[inline(always)]
            fn is_finite(self) -> bool {
                $t::is_finite(self)
            }
            #[inline(always)]
            fn is_nan(self) -> bool {
                $t::is_nan(self)
            }
            #[inline(always)]
            fn mul_add(self, b: Self, c: Self) -> Self {
                $t::mul_add(self, b, c)
            }
            #[inline(always)]
            fn powf(self, n: Self) -> Self {
                $t::powf(self, n)
            }
            #[inline(always)]
            fn recip(self) -> Self {
                $t::recip(self)
            }
            #[inline(always)]
            fn round(self) -> Self {
                $t::round(self)
            }
            #[inline(always)]
            fn sin(self) -> Self {
                $t::sin(self)
            }
            #[inline(always)]
            fn sin_cos(self) -> (Self, Self) {
                $t::sin_cos(self)
            }
            #[inline(always)]
            fn sqrt(self) -> Self {
                $t::sqrt(self)
            }
            #[inline(always)]
            fn tan(self) -> Self {
                $t::tan(self)
            }
        }
    };
}

#[cfg(not(feature = "libm"))]
impl_float_trait!(f32);
#[cfg(not(feature = "libm"))]
impl_float_trait!(f64);
#[cfg(not(feature = "libm"))]
impl_signed_trait!(i32);
#[cfg(not(feature = "libm"))]
impl_num_trait!(u32);

pub trait MaskConst: Sized {
    const MASK: [Self; 2];
}

pub trait NumConstEx: Sized {
    const ZERO: Self;
    const ONE: Self;
}

pub trait FloatConstEx: Sized {
    const NEG_ONE: Self;
    const TWO: Self;
    const HALF: Self;
}

pub trait NanConstEx: Sized {
    const NAN: Self;
}

pub trait NumEx:
    Num
    + NumConstEx
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
{
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

pub trait SignedEx: Signed + NumEx {}

pub trait FloatEx: Float + FloatConstEx + SignedEx + NanConstEx {
    /// Returns a very close approximation of `self.clamp(-1.0, 1.0).acos()`.
    fn acos_approx(self) -> Self;
    fn from_f32(f: f32) -> Self;
    fn from_f64(f: f64) -> Self;
}

impl NumConstEx for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl NanConstEx for f32 {
    const NAN: Self = f32::NAN;
}

impl FloatConstEx for f32 {
    const NEG_ONE: Self = -1.0;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
}

impl NumEx for f32 {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        f32::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }
}

impl SignedEx for f32 {}

impl FloatEx for f32 {
    #[inline(always)]
    fn from_f32(v: f32) -> Self {
        v
    }
    #[inline(always)]
    fn from_f64(v: f64) -> Self {
        v as Self
    }
    #[inline(always)]
    fn acos_approx(self) -> Self {
        // Based on https://github.com/microsoft/DirectXMath `XMScalarAcos`
        // Clamp input to [-1,1].
        let nonnegative = self >= 0.0;
        let x = self.abs();
        let mut omx = 1.0 - x;
        if omx < 0.0 {
            omx = 0.0;
        }
        let root = omx.sqrt();

        // 7-degree minimax approximation
        #[allow(clippy::approx_constant)]
        let mut result = ((((((-0.001_262_491_1 * x + 0.006_670_09) * x - 0.017_088_126) * x
            + 0.030_891_88)
            * x
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
}

impl NumConstEx for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl NanConstEx for f64 {
    const NAN: Self = f64::NAN;
}

impl FloatConstEx for f64 {
    const NEG_ONE: Self = -1.0;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
}

impl NumEx for f64 {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }
}

impl SignedEx for f64 {}

impl FloatEx for f64 {
    #[inline(always)]
    fn from_f32(v: f32) -> Self {
        v as Self
    }
    #[inline(always)]
    fn from_f64(v: f64) -> Self {
        v
    }
    #[inline(always)]
    fn acos_approx(self) -> Self {
        f64::acos(self.max(-1.0).min(1.0))
    }
}

impl NumConstEx for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl NumEx for i32 {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        core::cmp::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        core::cmp::max(self, other)
    }
}

impl SignedEx for i32 {}

impl NumConstEx for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl NumEx for u32 {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        core::cmp::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        core::cmp::max(self, other)
    }
}

pub trait IntegerShiftOps<Rhs>: Sized + Shl<Rhs, Output = Self> + Shr<Rhs, Output = Self> {}

pub trait IntegerBitOps:
    Sized + Not<Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self>
{
}

impl IntegerShiftOps<i8> for i32 {}
impl IntegerShiftOps<i16> for i32 {}
impl IntegerShiftOps<i32> for i32 {}
impl IntegerShiftOps<u8> for i32 {}
impl IntegerShiftOps<u16> for i32 {}
impl IntegerShiftOps<u32> for i32 {}

impl IntegerShiftOps<i8> for u32 {}
impl IntegerShiftOps<i16> for u32 {}
impl IntegerShiftOps<i32> for u32 {}
impl IntegerShiftOps<u8> for u32 {}
impl IntegerShiftOps<u16> for u32 {}
impl IntegerShiftOps<u32> for u32 {}

impl IntegerBitOps for i32 {}
impl IntegerBitOps for u32 {}

#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        assert_approx_eq!($a, $b, core::f32::EPSILON);
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        assert!(
            (a - b).abs() <= eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (a - b).abs()
        );
    }};
}

#[cfg(test)]
macro_rules! assert_relative_eq {
    ($a:expr, $b:expr) => {{
        assert_relative_eq!($a, $b, core::f32::EPSILON);
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        let diff = (a - b).abs();
        let largest = a.abs().max(b.abs());
        assert!(
            diff <= largest * eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            largest * eps,
            diff
        );
    }};
}

#[test]
fn test_scalar_acos() {
    fn test_scalar_acos_angle(a: f32) {
        // 1e-6 is the lowest epsilon that will pass
        assert_relative_eq!(a.acos_approx(), a.acos(), 1e-6);
        // assert_approx_eq!(scalar_acos(a), a.acos(), 1e-6);
    }

    // test 1024 floats between -1.0 and 1.0 inclusive
    const MAX_TESTS: u32 = 1024 / 2;
    const SIGN: u32 = 0x80_00_00_00;
    const PTVE_ONE: u32 = 0x3f_80_00_00; // 1.0_f32.to_bits();
    const NGVE_ONE: u32 = SIGN | PTVE_ONE;
    const STEP_SIZE: usize = (PTVE_ONE / MAX_TESTS) as usize;
    for f in (SIGN..=NGVE_ONE).step_by(STEP_SIZE).map(f32::from_bits) {
        test_scalar_acos_angle(f);
    }
    for f in (0..=PTVE_ONE).step_by(STEP_SIZE).map(f32::from_bits) {
        test_scalar_acos_angle(f);
    }

    // input is clamped to -1.0..1.0
    assert_approx_eq!(2.0_f32.acos_approx(), 0.0);
    assert_approx_eq!((-2.0_f32).acos_approx(), core::f32::consts::PI);

    // input is clamped to -1.0..1.0
    assert_eq!(2.0_f64.acos_approx(), 0.0);
    assert!(((-2.0_f64).acos_approx() - core::f64::consts::PI).abs() < f64::EPSILON);
}
