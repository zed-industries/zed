/*!
A collection of bounded numeric types.

Includes:

- [`FiniteF32`]
- [`FiniteF64`]
- [`NonZeroPositiveF32`]
- [`NonZeroPositiveF64`]
- [`PositiveF32`]
- [`PositiveF64`]
- [`NormalizedF32`]
- [`NormalizedF64`]

Unlike `f32`/`f64`, all float types implement `Ord`, `PartialOrd` and `Hash`,
since it's guaranteed that they all are finite.
*/

#![no_std]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]

macro_rules! impl_display {
    ($t:ident) => {
        impl core::fmt::Display for $t {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.get())
            }
        }
    };
}

#[cfg(feature = "approx-eq")]
pub use float_cmp::{ApproxEq, ApproxEqUlps, Ulps};

#[cfg(feature = "approx-eq")]
macro_rules! impl_approx_32 {
    ($t:ident) => {
        impl float_cmp::ApproxEq for $t {
            type Margin = float_cmp::F32Margin;

            #[inline]
            fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
                self.0.approx_eq(other.0, margin)
            }
        }

        impl float_cmp::ApproxEqUlps for $t {
            type Flt = f32;

            #[inline]
            fn approx_eq_ulps(&self, other: &Self, ulps: i32) -> bool {
                self.0.approx_eq_ulps(&other.0, ulps)
            }
        }
    };
}

#[cfg(not(feature = "approx-eq"))]
macro_rules! impl_approx_32 {
    ($t:ident) => {};
}

#[cfg(feature = "approx-eq")]
macro_rules! impl_approx_64 {
    ($t:ident) => {
        #[cfg(feature = "approx-eq")]
        impl float_cmp::ApproxEq for $t {
            type Margin = float_cmp::F64Margin;

            #[inline]
            fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
                self.0.approx_eq(other.0, margin)
            }
        }

        #[cfg(feature = "approx-eq")]
        impl float_cmp::ApproxEqUlps for $t {
            type Flt = f64;

            #[inline]
            fn approx_eq_ulps(&self, other: &Self, ulps: i64) -> bool {
                self.0.approx_eq_ulps(&other.0, ulps)
            }
        }
    };
}

#[cfg(not(feature = "approx-eq"))]
macro_rules! impl_approx_64 {
    ($t:ident) => {};
}

/// An immutable, finite `f32`.
///
/// Unlike `f32`, implements `Ord`, `PartialOrd` and `Hash`.
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct FiniteF32(f32);

impl FiniteF32 {
    /// Creates a finite `f32`.
    ///
    /// Returns `None` for NaN and infinity.
    #[inline]
    pub fn new(n: f32) -> Option<Self> {
        if n.is_finite() {
            Some(FiniteF32(n))
        } else {
            None
        }
    }

    /// Creates a finite `f32` without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite.
    #[inline]
    pub const unsafe fn new_unchecked(n: f32) -> Self {
        FiniteF32(n)
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f32 {
        self.0
    }
}

impl Eq for FiniteF32 {}

impl PartialEq for FiniteF32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for FiniteF32 {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.0 < other.0 {
            core::cmp::Ordering::Less
        } else if self.0 > other.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for FiniteF32 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::hash::Hash for FiniteF32 {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialEq<f32> for FiniteF32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.get() == *other
    }
}

impl_display!(FiniteF32);
impl_approx_32!(FiniteF32);

/// An immutable, finite `f64`.
///
/// Unlike `f64`, implements `Ord`, `PartialOrd` and `Hash`.
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct FiniteF64(f64);

impl FiniteF64 {
    /// Creates a finite `f64`.
    ///
    /// Returns `None` for NaN and infinity.
    #[inline]
    pub fn new(n: f64) -> Option<Self> {
        if n.is_finite() {
            Some(FiniteF64(n))
        } else {
            None
        }
    }

    /// Creates a finite `f64` without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite.
    #[inline]
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        FiniteF64(n)
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f64 {
        self.0
    }
}

impl Eq for FiniteF64 {}

impl PartialEq for FiniteF64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for FiniteF64 {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.0 < other.0 {
            core::cmp::Ordering::Less
        } else if self.0 > other.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for FiniteF64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::hash::Hash for FiniteF64 {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialEq<f64> for FiniteF64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.get() == *other
    }
}

impl_display!(FiniteF64);
impl_approx_64!(FiniteF64);

/// An immutable, finite `f32` that is known to be >= 0.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
#[repr(transparent)]
pub struct PositiveF32(FiniteF32);

impl PositiveF32 {
    /// A `PositiveF32` value initialized with zero.
    pub const ZERO: Self = PositiveF32(FiniteF32(0.0));

    /// Creates a new `PositiveF32` if the given value is >= 0.
    ///
    /// Returns `None` for negative, NaN and infinity.
    #[inline]
    pub fn new(n: f32) -> Option<Self> {
        if n.is_finite() && n >= 0.0 {
            Some(PositiveF32(FiniteF32(n)))
        } else {
            None
        }
    }

    /// Creates a new `PositiveF32` without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite and >= 0.
    #[inline]
    pub const unsafe fn new_unchecked(n: f32) -> Self {
        PositiveF32(FiniteF32(n))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f32 {
        self.0.get()
    }

    /// Returns the value as a `FiniteF32`.
    #[inline]
    pub const fn get_finite(&self) -> FiniteF32 {
        self.0
    }
}

impl PartialEq<f32> for PositiveF32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.get() == *other
    }
}

impl_display!(PositiveF32);
impl_approx_32!(PositiveF32);

/// An immutable, finite `f64` that is known to be >= 0.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
#[repr(transparent)]
pub struct PositiveF64(FiniteF64);

impl PositiveF64 {
    /// A `PositiveF64` value initialized with zero.
    pub const ZERO: Self = PositiveF64(FiniteF64(0.0));

    /// Creates a new `PositiveF64` if the given value is >= 0.
    ///
    /// Returns `None` for negative, NaN and infinity.
    #[inline]
    pub fn new(n: f64) -> Option<Self> {
        if n.is_finite() && n >= 0.0 {
            Some(PositiveF64(FiniteF64(n)))
        } else {
            None
        }
    }

    /// Creates a new `PositiveF64` without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite and >= 0.
    #[inline]
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        PositiveF64(FiniteF64(n))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f64 {
        self.0.get()
    }

    /// Returns the value as a `FiniteF64`.
    #[inline]
    pub const fn get_finite(&self) -> FiniteF64 {
        self.0
    }
}

impl PartialEq<f64> for PositiveF64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.get() == *other
    }
}

impl_display!(PositiveF64);
impl_approx_64!(PositiveF64);

/// An immutable, finite `f32` that is known to be > 0.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NonZeroPositiveF32(FiniteF32);

impl NonZeroPositiveF32 {
    /// Creates a new `NonZeroPositiveF32` if the given value is > 0.
    ///
    /// Returns `None` for negative, zero, NaN and infinity.
    #[inline]
    pub fn new(n: f32) -> Option<Self> {
        if n.is_finite() && n > 0.0 {
            Some(NonZeroPositiveF32(FiniteF32(n)))
        } else {
            None
        }
    }

    /// Creates a new `NonZeroPositiveF32` without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite and > 0.
    #[inline]
    pub const unsafe fn new_unchecked(n: f32) -> Self {
        NonZeroPositiveF32(FiniteF32(n))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f32 {
        self.0.get()
    }

    /// Returns the value as a `FiniteF32`.
    #[inline]
    pub const fn get_finite(&self) -> FiniteF32 {
        self.0
    }
}

impl PartialEq<f32> for NonZeroPositiveF32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.get() == *other
    }
}

impl_display!(NonZeroPositiveF32);
impl_approx_32!(NonZeroPositiveF32);

/// An immutable, finite `f64` that is known to be > 0.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NonZeroPositiveF64(FiniteF64);

impl NonZeroPositiveF64 {
    /// Creates a new `NonZeroPositiveF64` if the given value is > 0.
    ///
    /// Returns `None` for negative, zero, NaN and infinity.
    #[inline]
    pub fn new(n: f64) -> Option<Self> {
        if n.is_finite() && n > 0.0 {
            Some(NonZeroPositiveF64(FiniteF64(n)))
        } else {
            None
        }
    }

    /// Creates a new `NonZeroPositiveF64` without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be finite and > 0.
    #[inline]
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        NonZeroPositiveF64(FiniteF64(n))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(&self) -> f64 {
        self.0.get()
    }

    /// Returns the value as a `FiniteF64`.
    #[inline]
    pub const fn get_finite(&self) -> FiniteF64 {
        self.0
    }
}

impl PartialEq<f64> for NonZeroPositiveF64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.get() == *other
    }
}

impl_display!(NonZeroPositiveF64);
impl_approx_64!(NonZeroPositiveF64);

/// An immutable, finite `f32` in a 0..=1 range.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NormalizedF32(FiniteF32);

impl NormalizedF32 {
    /// A `NormalizedF32` value initialized with zero.
    pub const ZERO: Self = NormalizedF32(FiniteF32(0.0));
    /// A `NormalizedF32` value initialized with one.
    pub const ONE: Self = NormalizedF32(FiniteF32(1.0));

    /// Creates a `NormalizedF32` if the given value is in a 0..=1 range.
    #[inline]
    pub fn new(n: f32) -> Option<Self> {
        if n.is_finite() && n >= 0.0 && n <= 1.0 {
            Some(NormalizedF32(FiniteF32(n)))
        } else {
            None
        }
    }

    /// Creates a new `NormalizedF32` without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be in 0..=1 range.
    #[inline]
    pub const unsafe fn new_unchecked(n: f32) -> Self {
        NormalizedF32(FiniteF32(n))
    }

    /// Creates a `NormalizedF32` clamping the given value to a 0..=1 range.
    ///
    /// Returns zero in case of NaN or infinity.
    #[inline]
    pub fn new_clamped(n: f32) -> Self {
        if n.is_finite() {
            NormalizedF32(FiniteF32(clamp_f32(0.0, n, 1.0)))
        } else {
            Self::ZERO
        }
    }

    /// Creates a `NormalizedF32` by dividing the given value by 255.
    #[inline]
    pub fn new_u8(n: u8) -> Self {
        NormalizedF32(FiniteF32(f32::from(n) / 255.0))
    }

    /// Creates a `NormalizedF64` by dividing the given value by 65535.
    #[inline]
    pub fn new_u16(n: u16) -> Self {
        NormalizedF32(FiniteF32(f32::from(n) / 65535.0))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(self) -> f32 {
        self.0.get()
    }

    /// Returns the value as a `FiniteF32`.
    #[inline]
    pub const fn get_finite(&self) -> FiniteF32 {
        self.0
    }

    /// Returns the value as a `u8`.
    #[inline]
    pub fn to_u8(&self) -> u8 {
        ((self.0).0 * 255.0 + 0.5) as u8
    }

    /// Returns the value as a `u16`.
    #[inline]
    pub fn to_u16(&self) -> u16 {
        ((self.0).0 * 65535.0 + 0.5) as u16
    }
}

impl core::ops::Mul<NormalizedF32> for NormalizedF32 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_clamped((self.0).0 * (rhs.0).0)
    }
}

impl PartialEq<f32> for NormalizedF32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.get() == *other
    }
}

impl_display!(NormalizedF32);
impl_approx_32!(NormalizedF32);

/// An immutable, finite `f64` in a 0..=1 range.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NormalizedF64(FiniteF64);

impl NormalizedF64 {
    /// A `NormalizedF64` value initialized with zero.
    pub const ZERO: Self = NormalizedF64(FiniteF64(0.0));
    /// A `NormalizedF64` value initialized with one.
    pub const ONE: Self = NormalizedF64(FiniteF64(1.0));

    /// Creates a `NormalizedF64` if the given value is in a 0..=1 range.
    #[inline]
    pub fn new(n: f64) -> Option<Self> {
        if n >= 0.0 && n <= 1.0 {
            Some(NormalizedF64(FiniteF64(n)))
        } else {
            None
        }
    }

    /// Creates a new `NormalizedF64` without checking the value.
    ///
    /// # Safety
    ///
    /// `n` must be in 0..=1 range.
    #[inline]
    pub const unsafe fn new_unchecked(n: f64) -> Self {
        NormalizedF64(FiniteF64(n))
    }

    /// Creates a `NormalizedF64` clamping the given value to a 0..=1 range.
    ///
    /// Returns zero in case of NaN or infinity.
    #[inline]
    pub fn new_clamped(n: f64) -> Self {
        if n.is_finite() {
            NormalizedF64(FiniteF64(clamp_f64(0.0, n, 1.0)))
        } else {
            Self::ZERO
        }
    }

    /// Creates a `NormalizedF64` by dividing the given value by 255.
    #[inline]
    pub fn new_u8(n: u8) -> Self {
        NormalizedF64(FiniteF64(f64::from(n) / 255.0))
    }

    /// Creates a `NormalizedF64` by dividing the given value by 65535.
    #[inline]
    pub fn new_u16(n: u16) -> Self {
        NormalizedF64(FiniteF64(f64::from(n) / 65535.0))
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub const fn get(self) -> f64 {
        self.0.get()
    }

    /// Returns the value as a `FiniteF64`.
    #[inline]
    pub const fn get_finite(&self) -> FiniteF64 {
        self.0
    }

    /// Returns the value as a `u8`.
    #[inline]
    pub fn to_u8(&self) -> u8 {
        ((self.0).0 * 255.0 + 0.5) as u8
    }

    /// Returns the value as a `u16`.
    #[inline]
    pub fn to_u16(&self) -> u16 {
        ((self.0).0 * 65535.0 + 0.5) as u16
    }
}

impl core::ops::Mul<NormalizedF64> for NormalizedF64 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_clamped((self.0).0 * (rhs.0).0)
    }
}

impl PartialEq<f64> for NormalizedF64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.get() == *other
    }
}

impl_display!(NormalizedF64);
impl_approx_64!(NormalizedF64);

#[inline]
fn clamp_f32(min: f32, val: f32, max: f32) -> f32 {
    max.min(val).max(min)
}

#[inline]
fn clamp_f64(min: f64, val: f64, max: f64) -> f64 {
    max.min(val).max(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_f32() {
        assert_eq!(FiniteF32::new(0.0).map(|n| n.get()), Some(0.0));
        assert_eq!(FiniteF32::new(core::f32::NAN), None);
        assert_eq!(FiniteF32::new(core::f32::INFINITY), None);
        assert_eq!(FiniteF32::new(core::f32::NEG_INFINITY), None);
    }

    #[test]
    fn positive_f32() {
        assert_eq!(NonZeroPositiveF32::new(-1.0).map(|n| n.get()), None);
        assert_eq!(NonZeroPositiveF32::new(0.0).map(|n| n.get()), None);
        assert_eq!(NonZeroPositiveF32::new(1.0).map(|n| n.get()), Some(1.0));
        assert_eq!(
            NonZeroPositiveF32::new(core::f32::EPSILON).map(|n| n.get()),
            Some(core::f32::EPSILON)
        );
        assert_eq!(
            NonZeroPositiveF32::new(-core::f32::EPSILON).map(|n| n.get()),
            None
        );
        assert_eq!(NonZeroPositiveF32::new(core::f32::NAN), None);
        assert_eq!(NonZeroPositiveF32::new(core::f32::INFINITY), None);
        assert_eq!(NonZeroPositiveF32::new(core::f32::NEG_INFINITY), None);
    }

    #[test]
    fn positive_f64() {
        assert_eq!(NonZeroPositiveF32::new(-1.0).map(|n| n.get()), None);
        assert_eq!(NonZeroPositiveF64::new(0.0).map(|n| n.get()), None);
        assert_eq!(NonZeroPositiveF64::new(1.0).map(|n| n.get()), Some(1.0));
        assert_eq!(
            NonZeroPositiveF64::new(core::f64::EPSILON).map(|n| n.get()),
            Some(core::f64::EPSILON)
        );
        assert_eq!(
            NonZeroPositiveF64::new(-core::f64::EPSILON).map(|n| n.get()),
            None
        );
        assert_eq!(NonZeroPositiveF64::new(core::f64::NAN), None);
        assert_eq!(NonZeroPositiveF64::new(core::f64::INFINITY), None);
        assert_eq!(NonZeroPositiveF64::new(core::f64::NEG_INFINITY), None);
    }

    #[test]
    fn norm_f32() {
        assert_eq!(NormalizedF32::new(-0.5), None);
        assert_eq!(
            NormalizedF32::new(-core::f32::EPSILON).map(|n| n.get()),
            None
        );
        assert_eq!(NormalizedF32::new(0.0).map(|n| n.get()), Some(0.0));
        assert_eq!(NormalizedF32::new(0.5).map(|n| n.get()), Some(0.5));
        assert_eq!(NormalizedF32::new(1.0).map(|n| n.get()), Some(1.0));
        assert_eq!(NormalizedF32::new(1.5), None);
        assert_eq!(NormalizedF32::new(core::f32::NAN), None);
        assert_eq!(NormalizedF32::new(core::f32::INFINITY), None);
        assert_eq!(NormalizedF32::new(core::f32::NEG_INFINITY), None);
    }

    #[test]
    fn clamped_norm_f32() {
        assert_eq!(NormalizedF32::new_clamped(-0.5).get(), 0.0);
        assert_eq!(NormalizedF32::new_clamped(0.5).get(), 0.5);
        assert_eq!(NormalizedF32::new_clamped(1.5).get(), 1.0);
        assert_eq!(NormalizedF32::new_clamped(core::f32::NAN).get(), 0.0);
        assert_eq!(NormalizedF32::new_clamped(core::f32::INFINITY).get(), 0.0);
        assert_eq!(
            NormalizedF32::new_clamped(core::f32::NEG_INFINITY).get(),
            0.0
        );
    }

    #[test]
    fn norm_f64() {
        assert_eq!(NormalizedF64::new(-0.5), None);
        assert_eq!(
            NormalizedF64::new(-core::f64::EPSILON).map(|n| n.get()),
            None
        );
        assert_eq!(NormalizedF64::new(0.0).map(|n| n.get()), Some(0.0));
        assert_eq!(NormalizedF64::new(0.5).map(|n| n.get()), Some(0.5));
        assert_eq!(NormalizedF64::new(1.0).map(|n| n.get()), Some(1.0));
        assert_eq!(NormalizedF64::new(1.5), None);
        assert_eq!(NormalizedF64::new(core::f64::NAN), None);
        assert_eq!(NormalizedF64::new(core::f64::INFINITY), None);
        assert_eq!(NormalizedF64::new(core::f64::NEG_INFINITY), None);
    }

    #[test]
    fn clamped_norm_f64() {
        assert_eq!(NormalizedF64::new_clamped(-0.5).get(), 0.0);
        assert_eq!(NormalizedF64::new_clamped(0.5).get(), 0.5);
        assert_eq!(NormalizedF64::new_clamped(1.5).get(), 1.0);
        assert_eq!(NormalizedF64::new_clamped(core::f64::NAN).get(), 0.0);
        assert_eq!(NormalizedF64::new_clamped(core::f64::INFINITY).get(), 0.0);
        assert_eq!(
            NormalizedF64::new_clamped(core::f64::NEG_INFINITY).get(),
            0.0
        );
    }
}
