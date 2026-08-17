#![no_std]
#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! Wrappers for total order on Floats.  See the [`OrderedFloat`] and [`NotNan`] docs for details.

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::error::Error;

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::hint::unreachable_unchecked;
use core::iter::{Product, Sum};
use core::num::FpCategory;
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};
use core::str::FromStr;

#[cfg(not(feature = "std"))]
use num_traits::float::FloatCore as Float;
#[cfg(feature = "std")]
pub use num_traits::Float;
use num_traits::{Bounded, FromPrimitive, Num, NumCast, One, Signed, ToPrimitive, Zero};

// masks for the parts of the IEEE 754 float
const SIGN_MASK: u64 = 0x8000000000000000u64;
const EXP_MASK: u64 = 0x7ff0000000000000u64;
const MAN_MASK: u64 = 0x000fffffffffffffu64;

// canonical raw bit patterns (for hashing)
const CANONICAL_NAN_BITS: u64 = 0x7ff8000000000000u64;

#[inline(always)]
fn canonicalize_signed_zero<T: Float>(x: T) -> T {
    // -0.0 + 0.0 == +0.0 under IEEE754 roundTiesToEven rounding mode,
    // which Rust guarantees. Thus by adding a positive zero we
    // canonicalize signed zero without any branches in one instruction.
    x + T::zero()
}

/// A wrapper around floats providing implementations of `Eq`, `Ord`, and `Hash`.
///
/// NaN is sorted as *greater* than all other values and *equal*
/// to itself, in contradiction with the IEEE standard.
///
/// ```
/// use ordered_float::OrderedFloat;
/// use std::f32::NAN;
///
/// let mut v = [OrderedFloat(NAN), OrderedFloat(2.0), OrderedFloat(1.0)];
/// v.sort();
/// assert_eq!(v, [OrderedFloat(1.0), OrderedFloat(2.0), OrderedFloat(NAN)]);
/// ```
///
/// Because `OrderedFloat` implements `Ord` and `Eq`, it can be used as a key in a `HashSet`,
/// `HashMap`, `BTreeMap`, or `BTreeSet` (unlike the primitive `f32` or `f64` types):
///
/// ```
/// # use ordered_float::OrderedFloat;
/// # use std::collections::HashSet;
/// # use std::f32::NAN;
///
/// let mut s: HashSet<OrderedFloat<f32>> = HashSet::new();
/// s.insert(OrderedFloat(NAN));
/// assert!(s.contains(&OrderedFloat(NAN)));
/// ```
#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct OrderedFloat<T>(pub T);

impl<T: Float> OrderedFloat<T> {
    /// Get the value out.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Float> AsRef<T> for OrderedFloat<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: Float> AsMut<T> for OrderedFloat<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<'a, T: Float> From<&'a T> for &'a OrderedFloat<T> {
    #[inline]
    fn from(t: &'a T) -> &'a OrderedFloat<T> {
        // Safety: OrderedFloat is #[repr(transparent)] and has no invalid values.
        unsafe { &*(t as *const T as *const OrderedFloat<T>) }
    }
}

impl<'a, T: Float> From<&'a mut T> for &'a mut OrderedFloat<T> {
    #[inline]
    fn from(t: &'a mut T) -> &'a mut OrderedFloat<T> {
        // Safety: OrderedFloat is #[repr(transparent)] and has no invalid values.
        unsafe { &mut *(t as *mut T as *mut OrderedFloat<T>) }
    }
}

impl<T: Float> PartialOrd for OrderedFloat<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        other.ge(self)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        !other.ge(self)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        // We consider all NaNs equal, and NaN is the largest possible
        // value. Thus if self is NaN we always return true. Otherwise
        // self >= other is correct. If other is also not NaN it is trivially
        // correct, and if it is we note that nothing can be greater or
        // equal to NaN except NaN itself, which we already handled earlier.
        self.0.is_nan() | (self.0 >= other.0)
    }
}

impl<T: Float> Ord for OrderedFloat<T> {
    #[inline]
    #[allow(clippy::comparison_chain)]
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            Ordering::Less
        } else if self > other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<T: Float> PartialEq for OrderedFloat<T> {
    #[inline]
    fn eq(&self, other: &OrderedFloat<T>) -> bool {
        if self.0.is_nan() {
            other.0.is_nan()
        } else {
            self.0 == other.0
        }
    }
}

impl<T: Float> PartialEq<T> for OrderedFloat<T> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: Float> Hash for OrderedFloat<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bits = if self.is_nan() {
            CANONICAL_NAN_BITS
        } else {
            raw_double_bits(&canonicalize_signed_zero(self.0))
        };

        bits.hash(state)
    }
}

impl<T: Float + fmt::Display> fmt::Display for OrderedFloat<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<OrderedFloat<f32>> for f32 {
    #[inline]
    fn from(f: OrderedFloat<f32>) -> f32 {
        f.0
    }
}

impl From<OrderedFloat<f64>> for f64 {
    #[inline]
    fn from(f: OrderedFloat<f64>) -> f64 {
        f.0
    }
}

impl<T: Float> From<T> for OrderedFloat<T> {
    #[inline]
    fn from(val: T) -> Self {
        OrderedFloat(val)
    }
}

impl<T: Float> Deref for OrderedFloat<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Float> DerefMut for OrderedFloat<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Float> Eq for OrderedFloat<T> {}

macro_rules! impl_ordered_float_binop {
    ($imp:ident, $method:ident, $assign_imp:ident, $assign_method:ident) => {
        impl<T: $imp> $imp for OrderedFloat<T> {
            type Output = OrderedFloat<T::Output>;

            #[inline]
            fn $method(self, other: Self) -> Self::Output {
                OrderedFloat((self.0).$method(other.0))
            }
        }

        impl<T: $imp> $imp<T> for OrderedFloat<T> {
            type Output = OrderedFloat<T::Output>;

            #[inline]
            fn $method(self, other: T) -> Self::Output {
                OrderedFloat((self.0).$method(other))
            }
        }

        impl<'a, T> $imp<&'a T> for OrderedFloat<T>
        where
            T: $imp<&'a T>,
        {
            type Output = OrderedFloat<<T as $imp<&'a T>>::Output>;

            #[inline]
            fn $method(self, other: &'a T) -> Self::Output {
                OrderedFloat((self.0).$method(other))
            }
        }

        impl<'a, T> $imp<&'a Self> for OrderedFloat<T>
        where
            T: $imp<&'a T>,
        {
            type Output = OrderedFloat<<T as $imp<&'a T>>::Output>;

            #[inline]
            fn $method(self, other: &'a Self) -> Self::Output {
                OrderedFloat((self.0).$method(&other.0))
            }
        }

        impl<'a, T> $imp for &'a OrderedFloat<T>
        where
            &'a T: $imp,
        {
            type Output = OrderedFloat<<&'a T as $imp>::Output>;

            #[inline]
            fn $method(self, other: Self) -> Self::Output {
                OrderedFloat((self.0).$method(&other.0))
            }
        }

        impl<'a, T> $imp<OrderedFloat<T>> for &'a OrderedFloat<T>
        where
            &'a T: $imp<T>,
        {
            type Output = OrderedFloat<<&'a T as $imp<T>>::Output>;

            #[inline]
            fn $method(self, other: OrderedFloat<T>) -> Self::Output {
                OrderedFloat((self.0).$method(other.0))
            }
        }

        impl<'a, T> $imp<T> for &'a OrderedFloat<T>
        where
            &'a T: $imp<T>,
        {
            type Output = OrderedFloat<<&'a T as $imp<T>>::Output>;

            #[inline]
            fn $method(self, other: T) -> Self::Output {
                OrderedFloat((self.0).$method(other))
            }
        }

        impl<'a, T> $imp<&'a T> for &'a OrderedFloat<T>
        where
            &'a T: $imp,
        {
            type Output = OrderedFloat<<&'a T as $imp>::Output>;

            #[inline]
            fn $method(self, other: &'a T) -> Self::Output {
                OrderedFloat((self.0).$method(other))
            }
        }

        #[doc(hidden)] // Added accidentally; remove in next major version
        impl<'a, T> $imp<&'a Self> for &'a OrderedFloat<T>
        where
            &'a T: $imp,
        {
            type Output = OrderedFloat<<&'a T as $imp>::Output>;

            #[inline]
            fn $method(self, other: &'a Self) -> Self::Output {
                OrderedFloat((self.0).$method(&other.0))
            }
        }

        impl<T: $assign_imp> $assign_imp<T> for OrderedFloat<T> {
            #[inline]
            fn $assign_method(&mut self, other: T) {
                (self.0).$assign_method(other);
            }
        }

        impl<'a, T: $assign_imp<&'a T>> $assign_imp<&'a T> for OrderedFloat<T> {
            #[inline]
            fn $assign_method(&mut self, other: &'a T) {
                (self.0).$assign_method(other);
            }
        }

        impl<T: $assign_imp> $assign_imp for OrderedFloat<T> {
            #[inline]
            fn $assign_method(&mut self, other: Self) {
                (self.0).$assign_method(other.0);
            }
        }

        impl<'a, T: $assign_imp<&'a T>> $assign_imp<&'a Self> for OrderedFloat<T> {
            #[inline]
            fn $assign_method(&mut self, other: &'a Self) {
                (self.0).$assign_method(&other.0);
            }
        }
    };
}

impl_ordered_float_binop! {Add, add, AddAssign, add_assign}
impl_ordered_float_binop! {Sub, sub, SubAssign, sub_assign}
impl_ordered_float_binop! {Mul, mul, MulAssign, mul_assign}
impl_ordered_float_binop! {Div, div, DivAssign, div_assign}
impl_ordered_float_binop! {Rem, rem, RemAssign, rem_assign}

/// Adds a float directly.
impl<T: Float + Sum> Sum for OrderedFloat<T> {
    fn sum<I: Iterator<Item = OrderedFloat<T>>>(iter: I) -> Self {
        OrderedFloat(iter.map(|v| v.0).sum())
    }
}

impl<'a, T: Float + Sum + 'a> Sum<&'a OrderedFloat<T>> for OrderedFloat<T> {
    #[inline]
    fn sum<I: Iterator<Item = &'a OrderedFloat<T>>>(iter: I) -> Self {
        iter.cloned().sum()
    }
}

impl<T: Float + Product> Product for OrderedFloat<T> {
    fn product<I: Iterator<Item = OrderedFloat<T>>>(iter: I) -> Self {
        OrderedFloat(iter.map(|v| v.0).product())
    }
}

impl<'a, T: Float + Product + 'a> Product<&'a OrderedFloat<T>> for OrderedFloat<T> {
    #[inline]
    fn product<I: Iterator<Item = &'a OrderedFloat<T>>>(iter: I) -> Self {
        iter.cloned().product()
    }
}

impl<T: Float + Signed> Signed for OrderedFloat<T> {
    #[inline]
    fn abs(&self) -> Self {
        OrderedFloat(self.0.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        OrderedFloat(Signed::abs_sub(&self.0, &other.0))
    }

    #[inline]
    fn signum(&self) -> Self {
        OrderedFloat(self.0.signum())
    }
    #[inline]
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }
    #[inline]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl<T: Bounded> Bounded for OrderedFloat<T> {
    #[inline]
    fn min_value() -> Self {
        OrderedFloat(T::min_value())
    }

    #[inline]
    fn max_value() -> Self {
        OrderedFloat(T::max_value())
    }
}

impl<T: FromStr> FromStr for OrderedFloat<T> {
    type Err = T::Err;

    /// Convert a &str to `OrderedFloat`. Returns an error if the string fails to parse.
    ///
    /// ```
    /// use ordered_float::OrderedFloat;
    ///
    /// assert!("-10".parse::<OrderedFloat<f32>>().is_ok());
    /// assert!("abc".parse::<OrderedFloat<f32>>().is_err());
    /// assert!("NaN".parse::<OrderedFloat<f32>>().is_ok());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(OrderedFloat)
    }
}

impl<T: Neg> Neg for OrderedFloat<T> {
    type Output = OrderedFloat<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        OrderedFloat(-self.0)
    }
}

impl<'a, T> Neg for &'a OrderedFloat<T>
where
    &'a T: Neg,
{
    type Output = OrderedFloat<<&'a T as Neg>::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        OrderedFloat(-(&self.0))
    }
}

impl<T: Zero> Zero for OrderedFloat<T> {
    #[inline]
    fn zero() -> Self {
        OrderedFloat(T::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: One> One for OrderedFloat<T> {
    #[inline]
    fn one() -> Self {
        OrderedFloat(T::one())
    }
}

impl<T: NumCast> NumCast for OrderedFloat<T> {
    #[inline]
    fn from<F: ToPrimitive>(n: F) -> Option<Self> {
        T::from(n).map(OrderedFloat)
    }
}

impl<T: FromPrimitive> FromPrimitive for OrderedFloat<T> {
    fn from_i64(n: i64) -> Option<Self> {
        T::from_i64(n).map(OrderedFloat)
    }
    fn from_u64(n: u64) -> Option<Self> {
        T::from_u64(n).map(OrderedFloat)
    }
    fn from_isize(n: isize) -> Option<Self> {
        T::from_isize(n).map(OrderedFloat)
    }
    fn from_i8(n: i8) -> Option<Self> {
        T::from_i8(n).map(OrderedFloat)
    }
    fn from_i16(n: i16) -> Option<Self> {
        T::from_i16(n).map(OrderedFloat)
    }
    fn from_i32(n: i32) -> Option<Self> {
        T::from_i32(n).map(OrderedFloat)
    }
    fn from_usize(n: usize) -> Option<Self> {
        T::from_usize(n).map(OrderedFloat)
    }
    fn from_u8(n: u8) -> Option<Self> {
        T::from_u8(n).map(OrderedFloat)
    }
    fn from_u16(n: u16) -> Option<Self> {
        T::from_u16(n).map(OrderedFloat)
    }
    fn from_u32(n: u32) -> Option<Self> {
        T::from_u32(n).map(OrderedFloat)
    }
    fn from_f32(n: f32) -> Option<Self> {
        T::from_f32(n).map(OrderedFloat)
    }
    fn from_f64(n: f64) -> Option<Self> {
        T::from_f64(n).map(OrderedFloat)
    }
}

impl<T: ToPrimitive> ToPrimitive for OrderedFloat<T> {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }
    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
    fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }
    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }
    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }
    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }
    fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }
    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }
    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }
    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }
    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }
    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl<T: Float> num_traits::float::FloatCore for OrderedFloat<T> {
    fn nan() -> Self {
        OrderedFloat(T::nan())
    }
    fn infinity() -> Self {
        OrderedFloat(T::infinity())
    }
    fn neg_infinity() -> Self {
        OrderedFloat(T::neg_infinity())
    }
    fn neg_zero() -> Self {
        OrderedFloat(T::neg_zero())
    }
    fn min_value() -> Self {
        OrderedFloat(T::min_value())
    }
    fn min_positive_value() -> Self {
        OrderedFloat(T::min_positive_value())
    }
    fn max_value() -> Self {
        OrderedFloat(T::max_value())
    }
    fn is_nan(self) -> bool {
        self.0.is_nan()
    }
    fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }
    fn is_finite(self) -> bool {
        self.0.is_finite()
    }
    fn is_normal(self) -> bool {
        self.0.is_normal()
    }
    fn classify(self) -> FpCategory {
        self.0.classify()
    }
    fn floor(self) -> Self {
        OrderedFloat(self.0.floor())
    }
    fn ceil(self) -> Self {
        OrderedFloat(self.0.ceil())
    }
    fn round(self) -> Self {
        OrderedFloat(self.0.round())
    }
    fn trunc(self) -> Self {
        OrderedFloat(self.0.trunc())
    }
    fn fract(self) -> Self {
        OrderedFloat(self.0.fract())
    }
    fn abs(self) -> Self {
        OrderedFloat(self.0.abs())
    }
    fn signum(self) -> Self {
        OrderedFloat(self.0.signum())
    }
    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }
    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }
    fn recip(self) -> Self {
        OrderedFloat(self.0.recip())
    }
    fn powi(self, n: i32) -> Self {
        OrderedFloat(self.0.powi(n))
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        self.0.integer_decode()
    }
    fn epsilon() -> Self {
        OrderedFloat(T::epsilon())
    }
    fn to_degrees(self) -> Self {
        OrderedFloat(self.0.to_degrees())
    }
    fn to_radians(self) -> Self {
        OrderedFloat(self.0.to_radians())
    }
}

#[cfg(feature = "std")]
impl<T: Float> Float for OrderedFloat<T> {
    fn nan() -> Self {
        OrderedFloat(T::nan())
    }
    fn infinity() -> Self {
        OrderedFloat(T::infinity())
    }
    fn neg_infinity() -> Self {
        OrderedFloat(T::neg_infinity())
    }
    fn neg_zero() -> Self {
        OrderedFloat(T::neg_zero())
    }
    fn min_value() -> Self {
        OrderedFloat(T::min_value())
    }
    fn min_positive_value() -> Self {
        OrderedFloat(T::min_positive_value())
    }
    fn max_value() -> Self {
        OrderedFloat(T::max_value())
    }
    fn is_nan(self) -> bool {
        self.0.is_nan()
    }
    fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }
    fn is_finite(self) -> bool {
        self.0.is_finite()
    }
    fn is_normal(self) -> bool {
        self.0.is_normal()
    }
    fn classify(self) -> FpCategory {
        self.0.classify()
    }
    fn floor(self) -> Self {
        OrderedFloat(self.0.floor())
    }
    fn ceil(self) -> Self {
        OrderedFloat(self.0.ceil())
    }
    fn round(self) -> Self {
        OrderedFloat(self.0.round())
    }
    fn trunc(self) -> Self {
        OrderedFloat(self.0.trunc())
    }
    fn fract(self) -> Self {
        OrderedFloat(self.0.fract())
    }
    fn abs(self) -> Self {
        OrderedFloat(self.0.abs())
    }
    fn signum(self) -> Self {
        OrderedFloat(self.0.signum())
    }
    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }
    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        OrderedFloat(self.0.mul_add(a.0, b.0))
    }
    fn recip(self) -> Self {
        OrderedFloat(self.0.recip())
    }
    fn powi(self, n: i32) -> Self {
        OrderedFloat(self.0.powi(n))
    }
    fn powf(self, n: Self) -> Self {
        OrderedFloat(self.0.powf(n.0))
    }
    fn sqrt(self) -> Self {
        OrderedFloat(self.0.sqrt())
    }
    fn exp(self) -> Self {
        OrderedFloat(self.0.exp())
    }
    fn exp2(self) -> Self {
        OrderedFloat(self.0.exp2())
    }
    fn ln(self) -> Self {
        OrderedFloat(self.0.ln())
    }
    fn log(self, base: Self) -> Self {
        OrderedFloat(self.0.log(base.0))
    }
    fn log2(self) -> Self {
        OrderedFloat(self.0.log2())
    }
    fn log10(self) -> Self {
        OrderedFloat(self.0.log10())
    }
    fn max(self, other: Self) -> Self {
        OrderedFloat(self.0.max(other.0))
    }
    fn min(self, other: Self) -> Self {
        OrderedFloat(self.0.min(other.0))
    }
    fn abs_sub(self, other: Self) -> Self {
        OrderedFloat(self.0.abs_sub(other.0))
    }
    fn cbrt(self) -> Self {
        OrderedFloat(self.0.cbrt())
    }
    fn hypot(self, other: Self) -> Self {
        OrderedFloat(self.0.hypot(other.0))
    }
    fn sin(self) -> Self {
        OrderedFloat(self.0.sin())
    }
    fn cos(self) -> Self {
        OrderedFloat(self.0.cos())
    }
    fn tan(self) -> Self {
        OrderedFloat(self.0.tan())
    }
    fn asin(self) -> Self {
        OrderedFloat(self.0.asin())
    }
    fn acos(self) -> Self {
        OrderedFloat(self.0.acos())
    }
    fn atan(self) -> Self {
        OrderedFloat(self.0.atan())
    }
    fn atan2(self, other: Self) -> Self {
        OrderedFloat(self.0.atan2(other.0))
    }
    fn sin_cos(self) -> (Self, Self) {
        let (a, b) = self.0.sin_cos();
        (OrderedFloat(a), OrderedFloat(b))
    }
    fn exp_m1(self) -> Self {
        OrderedFloat(self.0.exp_m1())
    }
    fn ln_1p(self) -> Self {
        OrderedFloat(self.0.ln_1p())
    }
    fn sinh(self) -> Self {
        OrderedFloat(self.0.sinh())
    }
    fn cosh(self) -> Self {
        OrderedFloat(self.0.cosh())
    }
    fn tanh(self) -> Self {
        OrderedFloat(self.0.tanh())
    }
    fn asinh(self) -> Self {
        OrderedFloat(self.0.asinh())
    }
    fn acosh(self) -> Self {
        OrderedFloat(self.0.acosh())
    }
    fn atanh(self) -> Self {
        OrderedFloat(self.0.atanh())
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        self.0.integer_decode()
    }
    fn epsilon() -> Self {
        OrderedFloat(T::epsilon())
    }
    fn to_degrees(self) -> Self {
        OrderedFloat(self.0.to_degrees())
    }
    fn to_radians(self) -> Self {
        OrderedFloat(self.0.to_radians())
    }
}

impl<T: Float + Num> Num for OrderedFloat<T> {
    type FromStrRadixErr = T::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(OrderedFloat)
    }
}

/// A wrapper around floats providing an implementation of `Eq`, `Ord` and `Hash`.
///
/// A NaN value cannot be stored in this type.
///
/// ```
/// use ordered_float::NotNan;
///
/// let mut v = [
///     NotNan::new(2.0).unwrap(),
///     NotNan::new(1.0).unwrap(),
/// ];
/// v.sort();
/// assert_eq!(v, [1.0, 2.0]);
/// ```
///
/// Because `NotNan` implements `Ord` and `Eq`, it can be used as a key in a `HashSet`,
/// `HashMap`, `BTreeMap`, or `BTreeSet` (unlike the primitive `f32` or `f64` types):
///
/// ```
/// # use ordered_float::NotNan;
/// # use std::collections::HashSet;
///
/// let mut s: HashSet<NotNan<f32>> = HashSet::new();
/// let key = NotNan::new(1.0).unwrap();
/// s.insert(key);
/// assert!(s.contains(&key));
/// ```
///
/// Arithmetic on NotNan values will panic if it produces a NaN value:
///
/// ```should_panic
/// # use ordered_float::NotNan;
/// let a = NotNan::new(std::f32::INFINITY).unwrap();
/// let b = NotNan::new(std::f32::NEG_INFINITY).unwrap();
///
/// // This will panic:
/// let c = a + b;
/// ```
#[derive(PartialOrd, PartialEq, Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct NotNan<T>(T);

impl<T: Float> NotNan<T> {
    /// Create a `NotNan` value.
    ///
    /// Returns `Err` if `val` is NaN
    pub fn new(val: T) -> Result<Self, FloatIsNan> {
        match val {
            ref val if val.is_nan() => Err(FloatIsNan),
            val => Ok(NotNan(val)),
        }
    }
}

impl<T> NotNan<T> {
    /// Get the value out.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Create a `NotNan` value from a value that is guaranteed to not be NaN
    ///
    /// # Safety
    ///
    /// Behaviour is undefined if `val` is NaN
    #[inline]
    pub const unsafe fn new_unchecked(val: T) -> Self {
        NotNan(val)
    }

    /// Create a `NotNan` value from a value that is guaranteed to not be NaN
    ///
    /// # Safety
    ///
    /// Behaviour is undefined if `val` is NaN
    #[deprecated(
        since = "2.5.0",
        note = "Please use the new_unchecked function instead."
    )]
    #[inline]
    pub const unsafe fn unchecked_new(val: T) -> Self {
        Self::new_unchecked(val)
    }
}

impl<T: Float> AsRef<T> for NotNan<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl Borrow<f32> for NotNan<f32> {
    #[inline]
    fn borrow(&self) -> &f32 {
        &self.0
    }
}

impl Borrow<f64> for NotNan<f64> {
    #[inline]
    fn borrow(&self) -> &f64 {
        &self.0
    }
}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl<T: Float> Ord for NotNan<T> {
    fn cmp(&self, other: &NotNan<T>) -> Ordering {
        match self.partial_cmp(&other) {
            Some(ord) => ord,
            None => unsafe { unreachable_unchecked() },
        }
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl<T: Float> Hash for NotNan<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bits = raw_double_bits(&canonicalize_signed_zero(self.0));
        bits.hash(state)
    }
}

impl<T: Float + fmt::Display> fmt::Display for NotNan<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<NotNan<f32>> for f32 {
    #[inline]
    fn from(value: NotNan<f32>) -> Self {
        value.0
    }
}

impl From<NotNan<f64>> for f64 {
    #[inline]
    fn from(value: NotNan<f64>) -> Self {
        value.0
    }
}

impl TryFrom<f32> for NotNan<f32> {
    type Error = FloatIsNan;
    #[inline]
    fn try_from(v: f32) -> Result<Self, Self::Error> {
        NotNan::new(v)
    }
}

impl TryFrom<f64> for NotNan<f64> {
    type Error = FloatIsNan;
    #[inline]
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        NotNan::new(v)
    }
}

macro_rules! impl_from_int_primitive {
    ($primitive:ty, $inner:ty) => {
        impl From<$primitive> for NotNan<$inner> {
            fn from(source: $primitive) -> Self {
                // the primitives with which this macro will be called cannot hold a value that
                // f64::from would convert to NaN, so this does not hurt invariants
                NotNan(<$inner as From<$primitive>>::from(source))
            }
        }
    };
}

impl_from_int_primitive!(i8, f64);
impl_from_int_primitive!(i16, f64);
impl_from_int_primitive!(i32, f64);
impl_from_int_primitive!(u8, f64);
impl_from_int_primitive!(u16, f64);
impl_from_int_primitive!(u32, f64);

impl_from_int_primitive!(i8, f32);
impl_from_int_primitive!(i16, f32);
impl_from_int_primitive!(u8, f32);
impl_from_int_primitive!(u16, f32);

impl From<NotNan<f32>> for NotNan<f64> {
    #[inline]
    fn from(v: NotNan<f32>) -> NotNan<f64> {
        unsafe { NotNan::new_unchecked(v.0 as f64) }
    }
}

impl<T: Float> Deref for NotNan<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Float + PartialEq> Eq for NotNan<T> {}

impl<T: Float> PartialEq<T> for NotNan<T> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

/// Adds a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float> Add<T> for NotNan<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: T) -> Self {
        NotNan::new(self.0 + other).expect("Addition resulted in NaN")
    }
}

/// Adds a float directly.
///
/// Panics if the provided value is NaN.
impl<T: Float + Sum> Sum for NotNan<T> {
    fn sum<I: Iterator<Item = NotNan<T>>>(iter: I) -> Self {
        NotNan::new(iter.map(|v| v.0).sum()).expect("Sum resulted in NaN")
    }
}

impl<'a, T: Float + Sum + 'a> Sum<&'a NotNan<T>> for NotNan<T> {
    #[inline]
    fn sum<I: Iterator<Item = &'a NotNan<T>>>(iter: I) -> Self {
        iter.cloned().sum()
    }
}

/// Subtracts a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float> Sub<T> for NotNan<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: T) -> Self {
        NotNan::new(self.0 - other).expect("Subtraction resulted in NaN")
    }
}

/// Multiplies a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float> Mul<T> for NotNan<T> {
    type Output = Self;

    #[inline]
    fn mul(self, other: T) -> Self {
        NotNan::new(self.0 * other).expect("Multiplication resulted in NaN")
    }
}

impl<T: Float + Product> Product for NotNan<T> {
    fn product<I: Iterator<Item = NotNan<T>>>(iter: I) -> Self {
        NotNan::new(iter.map(|v| v.0).product()).expect("Product resulted in NaN")
    }
}

impl<'a, T: Float + Product + 'a> Product<&'a NotNan<T>> for NotNan<T> {
    #[inline]
    fn product<I: Iterator<Item = &'a NotNan<T>>>(iter: I) -> Self {
        iter.cloned().product()
    }
}

/// Divides a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float> Div<T> for NotNan<T> {
    type Output = Self;

    #[inline]
    fn div(self, other: T) -> Self {
        NotNan::new(self.0 / other).expect("Division resulted in NaN")
    }
}

/// Calculates `%` with a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float> Rem<T> for NotNan<T> {
    type Output = Self;

    #[inline]
    fn rem(self, other: T) -> Self {
        NotNan::new(self.0 % other).expect("Rem resulted in NaN")
    }
}

macro_rules! impl_not_nan_binop {
    ($imp:ident, $method:ident, $assign_imp:ident, $assign_method:ident) => {
        impl<T: Float> $imp for NotNan<T> {
            type Output = Self;

            #[inline]
            fn $method(self, other: Self) -> Self {
                self.$method(other.0)
            }
        }

        impl<T: Float> $imp<&T> for NotNan<T> {
            type Output = NotNan<T>;

            #[inline]
            fn $method(self, other: &T) -> Self::Output {
                self.$method(*other)
            }
        }

        impl<T: Float> $imp<&Self> for NotNan<T> {
            type Output = NotNan<T>;

            #[inline]
            fn $method(self, other: &Self) -> Self::Output {
                self.$method(other.0)
            }
        }

        impl<T: Float> $imp for &NotNan<T> {
            type Output = NotNan<T>;

            #[inline]
            fn $method(self, other: Self) -> Self::Output {
                (*self).$method(other.0)
            }
        }

        impl<T: Float> $imp<NotNan<T>> for &NotNan<T> {
            type Output = NotNan<T>;

            #[inline]
            fn $method(self, other: NotNan<T>) -> Self::Output {
                (*self).$method(other.0)
            }
        }

        impl<T: Float> $imp<T> for &NotNan<T> {
            type Output = NotNan<T>;

            #[inline]
            fn $method(self, other: T) -> Self::Output {
                (*self).$method(other)
            }
        }

        impl<T: Float> $imp<&T> for &NotNan<T> {
            type Output = NotNan<T>;

            #[inline]
            fn $method(self, other: &T) -> Self::Output {
                (*self).$method(*other)
            }
        }

        impl<T: Float + $assign_imp> $assign_imp<T> for NotNan<T> {
            #[inline]
            fn $assign_method(&mut self, other: T) {
                *self = (*self).$method(other);
            }
        }

        impl<T: Float + $assign_imp> $assign_imp<&T> for NotNan<T> {
            #[inline]
            fn $assign_method(&mut self, other: &T) {
                *self = (*self).$method(*other);
            }
        }

        impl<T: Float + $assign_imp> $assign_imp for NotNan<T> {
            #[inline]
            fn $assign_method(&mut self, other: Self) {
                (*self).$assign_method(other.0);
            }
        }

        impl<T: Float + $assign_imp> $assign_imp<&Self> for NotNan<T> {
            #[inline]
            fn $assign_method(&mut self, other: &Self) {
                (*self).$assign_method(other.0);
            }
        }
    };
}

impl_not_nan_binop! {Add, add, AddAssign, add_assign}
impl_not_nan_binop! {Sub, sub, SubAssign, sub_assign}
impl_not_nan_binop! {Mul, mul, MulAssign, mul_assign}
impl_not_nan_binop! {Div, div, DivAssign, div_assign}
impl_not_nan_binop! {Rem, rem, RemAssign, rem_assign}

impl<T: Float> Neg for NotNan<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        NotNan(-self.0)
    }
}

impl<T: Float> Neg for &NotNan<T> {
    type Output = NotNan<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        NotNan(-self.0)
    }
}

/// An error indicating an attempt to construct NotNan from a NaN
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FloatIsNan;

#[cfg(feature = "std")]
impl Error for FloatIsNan {
    fn description(&self) -> &str {
        "NotNan constructed with NaN"
    }
}

impl fmt::Display for FloatIsNan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NotNan constructed with NaN")
    }
}

#[cfg(feature = "std")]
impl From<FloatIsNan> for std::io::Error {
    #[inline]
    fn from(e: FloatIsNan) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
    }
}

#[inline]
fn raw_double_bits<F: Float>(f: &F) -> u64 {
    let (man, exp, sign) = f.integer_decode();
    let exp_u64 = exp as u16 as u64;
    let sign_u64 = if sign > 0 { 1u64 } else { 0u64 };
    (man & MAN_MASK) | ((exp_u64 << 52) & EXP_MASK) | ((sign_u64 << 63) & SIGN_MASK)
}

impl<T: Float> Zero for NotNan<T> {
    #[inline]
    fn zero() -> Self {
        NotNan(T::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: Float> One for NotNan<T> {
    #[inline]
    fn one() -> Self {
        NotNan(T::one())
    }
}

impl<T: Float> Bounded for NotNan<T> {
    #[inline]
    fn min_value() -> Self {
        NotNan(T::min_value())
    }

    #[inline]
    fn max_value() -> Self {
        NotNan(T::max_value())
    }
}

impl<T: Float + FromStr> FromStr for NotNan<T> {
    type Err = ParseNotNanError<T::Err>;

    /// Convert a &str to `NotNan`. Returns an error if the string fails to parse,
    /// or if the resulting value is NaN
    ///
    /// ```
    /// use ordered_float::NotNan;
    ///
    /// assert!("-10".parse::<NotNan<f32>>().is_ok());
    /// assert!("abc".parse::<NotNan<f32>>().is_err());
    /// assert!("NaN".parse::<NotNan<f32>>().is_err());
    /// ```
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        src.parse()
            .map_err(ParseNotNanError::ParseFloatError)
            .and_then(|f| NotNan::new(f).map_err(|_| ParseNotNanError::IsNaN))
    }
}

impl<T: Float + FromPrimitive> FromPrimitive for NotNan<T> {
    fn from_i64(n: i64) -> Option<Self> {
        T::from_i64(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_u64(n: u64) -> Option<Self> {
        T::from_u64(n).and_then(|n| NotNan::new(n).ok())
    }

    fn from_isize(n: isize) -> Option<Self> {
        T::from_isize(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_i8(n: i8) -> Option<Self> {
        T::from_i8(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_i16(n: i16) -> Option<Self> {
        T::from_i16(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_i32(n: i32) -> Option<Self> {
        T::from_i32(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_usize(n: usize) -> Option<Self> {
        T::from_usize(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_u8(n: u8) -> Option<Self> {
        T::from_u8(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_u16(n: u16) -> Option<Self> {
        T::from_u16(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_u32(n: u32) -> Option<Self> {
        T::from_u32(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_f32(n: f32) -> Option<Self> {
        T::from_f32(n).and_then(|n| NotNan::new(n).ok())
    }
    fn from_f64(n: f64) -> Option<Self> {
        T::from_f64(n).and_then(|n| NotNan::new(n).ok())
    }
}

impl<T: Float> ToPrimitive for NotNan<T> {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }
    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }
    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }
    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }
    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }
    fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }
    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }
    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }
    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }
    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }
    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

/// An error indicating a parse error from a string for `NotNan`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ParseNotNanError<E> {
    /// A plain parse error from the underlying float type.
    ParseFloatError(E),
    /// The parsed float value resulted in a NaN.
    IsNaN,
}

#[cfg(feature = "std")]
impl<E: fmt::Debug + Error + 'static> Error for ParseNotNanError<E> {
    fn description(&self) -> &str {
        "Error parsing a not-NaN floating point value"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseNotNanError::ParseFloatError(e) => Some(e),
            ParseNotNanError::IsNaN => None,
        }
    }
}

impl<E: fmt::Display> fmt::Display for ParseNotNanError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseNotNanError::ParseFloatError(e) => write!(f, "Parse error: {}", e),
            ParseNotNanError::IsNaN => write!(f, "NotNan parser encounter a NaN"),
        }
    }
}

impl<T: Float> Num for NotNan<T> {
    type FromStrRadixErr = ParseNotNanError<T::FromStrRadixErr>;

    fn from_str_radix(src: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(src, radix)
            .map_err(ParseNotNanError::ParseFloatError)
            .and_then(|n| NotNan::new(n).map_err(|_| ParseNotNanError::IsNaN))
    }
}

impl<T: Float + Signed> Signed for NotNan<T> {
    #[inline]
    fn abs(&self) -> Self {
        NotNan(self.0.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        NotNan::new(Signed::abs_sub(&self.0, &other.0)).expect("Subtraction resulted in NaN")
    }

    #[inline]
    fn signum(&self) -> Self {
        NotNan(self.0.signum())
    }
    #[inline]
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }
    #[inline]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl<T: Float> NumCast for NotNan<T> {
    fn from<F: ToPrimitive>(n: F) -> Option<Self> {
        T::from(n).and_then(|n| NotNan::new(n).ok())
    }
}

#[cfg(feature = "serde")]
mod impl_serde {
    extern crate serde;
    use self::serde::de::{Error, Unexpected};
    use self::serde::{Deserialize, Deserializer, Serialize, Serializer};
    use super::{NotNan, OrderedFloat};
    use core::f64;
    #[cfg(not(feature = "std"))]
    use num_traits::float::FloatCore as Float;
    #[cfg(feature = "std")]
    use num_traits::Float;

    #[cfg(test)]
    extern crate serde_test;
    #[cfg(test)]
    use self::serde_test::{assert_de_tokens_error, assert_tokens, Token};

    impl<T: Float + Serialize> Serialize for OrderedFloat<T> {
        #[inline]
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(s)
        }
    }

    impl<'de, T: Float + Deserialize<'de>> Deserialize<'de> for OrderedFloat<T> {
        #[inline]
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            T::deserialize(d).map(OrderedFloat)
        }
    }

    impl<T: Float + Serialize> Serialize for NotNan<T> {
        #[inline]
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(s)
        }
    }

    impl<'de, T: Float + Deserialize<'de>> Deserialize<'de> for NotNan<T> {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let float = T::deserialize(d)?;
            NotNan::new(float).map_err(|_| {
                Error::invalid_value(Unexpected::Float(f64::NAN), &"float (but not NaN)")
            })
        }
    }

    #[test]
    fn test_ordered_float() {
        let float = OrderedFloat(1.0f64);
        assert_tokens(&float, &[Token::F64(1.0)]);
    }

    #[test]
    fn test_not_nan() {
        let float = NotNan(1.0f64);
        assert_tokens(&float, &[Token::F64(1.0)]);
    }

    #[test]
    fn test_fail_on_nan() {
        assert_de_tokens_error::<NotNan<f64>>(
            &[Token::F64(f64::NAN)],
            "invalid value: floating point `NaN`, expected float (but not NaN)",
        );
    }
}

#[cfg(feature = "rkyv")]
mod impl_rkyv {
    use super::{NotNan, OrderedFloat};
    #[cfg(not(feature = "std"))]
    use num_traits::float::FloatCore as Float;
    #[cfg(feature = "std")]
    use num_traits::Float;
    #[cfg(test)]
    use rkyv::{archived_root, ser::Serializer};
    use rkyv::{from_archived, Archive, Deserialize, Fallible, Serialize};

    #[cfg(test)]
    type DefaultSerializer = rkyv::ser::serializers::CoreSerializer<16, 16>;
    #[cfg(test)]
    type DefaultDeserializer = rkyv::Infallible;

    impl<T: Float + Archive> Archive for OrderedFloat<T> {
        type Archived = OrderedFloat<T>;

        type Resolver = ();

        unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
            out.write(*self);
        }
    }

    impl<T: Float + Serialize<S>, S: Fallible + ?Sized> Serialize<S> for OrderedFloat<T> {
        fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
            Ok(())
        }
    }

    impl<T: Float + Deserialize<T, D>, D: Fallible + ?Sized> Deserialize<OrderedFloat<T>, D>
        for OrderedFloat<T>
    {
        fn deserialize(&self, _: &mut D) -> Result<OrderedFloat<T>, D::Error> {
            Ok(from_archived!(*self))
        }
    }

    impl<T: Float + Archive> Archive for NotNan<T> {
        type Archived = NotNan<T>;

        type Resolver = ();

        unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
            out.write(*self);
        }
    }

    impl<T: Float + Serialize<S>, S: Fallible + ?Sized> Serialize<S> for NotNan<T> {
        fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
            Ok(())
        }
    }

    impl<T: Float + Deserialize<T, D>, D: Fallible + ?Sized> Deserialize<NotNan<T>, D> for NotNan<T> {
        fn deserialize(&self, _: &mut D) -> Result<NotNan<T>, D::Error> {
            Ok(from_archived!(*self))
        }
    }

    #[test]
    fn test_ordered_float() {
        let float = OrderedFloat(1.0f64);
        let mut serializer = DefaultSerializer::default();
        serializer
            .serialize_value(&float)
            .expect("failed to archive value");
        let len = serializer.pos();
        let buffer = serializer.into_serializer().into_inner();

        let archived_value = unsafe { archived_root::<OrderedFloat<f64>>(&buffer[0..len]) };
        assert_eq!(archived_value, &float);
        let mut deserializer = DefaultDeserializer::default();
        let deser_float: OrderedFloat<f64> = archived_value.deserialize(&mut deserializer).unwrap();
        assert_eq!(deser_float, float);
    }

    #[test]
    fn test_not_nan() {
        let float = NotNan(1.0f64);
        let mut serializer = DefaultSerializer::default();
        serializer
            .serialize_value(&float)
            .expect("failed to archive value");
        let len = serializer.pos();
        let buffer = serializer.into_serializer().into_inner();

        let archived_value = unsafe { archived_root::<NotNan<f64>>(&buffer[0..len]) };
        assert_eq!(archived_value, &float);
        let mut deserializer = DefaultDeserializer::default();
        let deser_float: NotNan<f64> = archived_value.deserialize(&mut deserializer).unwrap();
        assert_eq!(deser_float, float);
    }
}

#[cfg(all(feature = "std", feature = "schemars"))]
mod impl_schemars {
    extern crate schemars;
    use self::schemars::gen::SchemaGenerator;
    use self::schemars::schema::{InstanceType, Schema, SchemaObject};
    use super::{NotNan, OrderedFloat};

    macro_rules! primitive_float_impl {
        ($type:ty, $schema_name:literal) => {
            impl schemars::JsonSchema for $type {
                fn is_referenceable() -> bool {
                    false
                }

                fn schema_name() -> std::string::String {
                    std::string::String::from($schema_name)
                }

                fn json_schema(_: &mut SchemaGenerator) -> Schema {
                    SchemaObject {
                        instance_type: Some(InstanceType::Number.into()),
                        format: Some(std::string::String::from($schema_name)),
                        ..Default::default()
                    }
                    .into()
                }
            }
        };
    }

    primitive_float_impl!(OrderedFloat<f32>, "float");
    primitive_float_impl!(OrderedFloat<f64>, "double");
    primitive_float_impl!(NotNan<f32>, "float");
    primitive_float_impl!(NotNan<f64>, "double");

    #[test]
    fn schema_generation_does_not_panic_for_common_floats() {
        {
            let schema = schemars::gen::SchemaGenerator::default()
                .into_root_schema_for::<OrderedFloat<f32>>();
            assert_eq!(
                schema.schema.instance_type,
                Some(schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                    schemars::schema::InstanceType::Number
                )))
            );
            assert_eq!(
                schema.schema.metadata.unwrap().title.unwrap(),
                std::string::String::from("float")
            );
        }
        {
            let schema = schemars::gen::SchemaGenerator::default()
                .into_root_schema_for::<OrderedFloat<f64>>();
            assert_eq!(
                schema.schema.instance_type,
                Some(schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                    schemars::schema::InstanceType::Number
                )))
            );
            assert_eq!(
                schema.schema.metadata.unwrap().title.unwrap(),
                std::string::String::from("double")
            );
        }
        {
            let schema =
                schemars::gen::SchemaGenerator::default().into_root_schema_for::<NotNan<f32>>();
            assert_eq!(
                schema.schema.instance_type,
                Some(schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                    schemars::schema::InstanceType::Number
                )))
            );
            assert_eq!(
                schema.schema.metadata.unwrap().title.unwrap(),
                std::string::String::from("float")
            );
        }
        {
            let schema =
                schemars::gen::SchemaGenerator::default().into_root_schema_for::<NotNan<f64>>();
            assert_eq!(
                schema.schema.instance_type,
                Some(schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                    schemars::schema::InstanceType::Number
                )))
            );
            assert_eq!(
                schema.schema.metadata.unwrap().title.unwrap(),
                std::string::String::from("double")
            );
        }
    }
    #[test]
    fn ordered_float_schema_match_primitive_schema() {
        {
            let of_schema = schemars::gen::SchemaGenerator::default()
                .into_root_schema_for::<OrderedFloat<f32>>();
            let prim_schema =
                schemars::gen::SchemaGenerator::default().into_root_schema_for::<f32>();
            assert_eq!(of_schema, prim_schema);
        }
        {
            let of_schema = schemars::gen::SchemaGenerator::default()
                .into_root_schema_for::<OrderedFloat<f64>>();
            let prim_schema =
                schemars::gen::SchemaGenerator::default().into_root_schema_for::<f64>();
            assert_eq!(of_schema, prim_schema);
        }
        {
            let of_schema =
                schemars::gen::SchemaGenerator::default().into_root_schema_for::<NotNan<f32>>();
            let prim_schema =
                schemars::gen::SchemaGenerator::default().into_root_schema_for::<f32>();
            assert_eq!(of_schema, prim_schema);
        }
        {
            let of_schema =
                schemars::gen::SchemaGenerator::default().into_root_schema_for::<NotNan<f64>>();
            let prim_schema =
                schemars::gen::SchemaGenerator::default().into_root_schema_for::<f64>();
            assert_eq!(of_schema, prim_schema);
        }
    }
}

#[cfg(feature = "rand")]
mod impl_rand {
    use super::{NotNan, OrderedFloat};
    use rand::distributions::uniform::*;
    use rand::distributions::{Distribution, Open01, OpenClosed01, Standard};
    use rand::Rng;

    macro_rules! impl_distribution {
        ($dist:ident, $($f:ty),+) => {
            $(
            impl Distribution<NotNan<$f>> for $dist {
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NotNan<$f> {
                    // 'rand' never generates NaN values in the Standard, Open01, or
                    // OpenClosed01 distributions. Using 'new_unchecked' is therefore
                    // safe.
                    unsafe { NotNan::new_unchecked(self.sample(rng)) }
                }
            }

            impl Distribution<OrderedFloat<$f>> for $dist {
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OrderedFloat<$f> {
                    OrderedFloat(self.sample(rng))
                }
            }
            )*
        }
    }

    impl_distribution! { Standard, f32, f64 }
    impl_distribution! { Open01, f32, f64 }
    impl_distribution! { OpenClosed01, f32, f64 }

    pub struct UniformNotNan<T>(UniformFloat<T>);
    impl SampleUniform for NotNan<f32> {
        type Sampler = UniformNotNan<f32>;
    }
    impl SampleUniform for NotNan<f64> {
        type Sampler = UniformNotNan<f64>;
    }

    pub struct UniformOrdered<T>(UniformFloat<T>);
    impl SampleUniform for OrderedFloat<f32> {
        type Sampler = UniformOrdered<f32>;
    }
    impl SampleUniform for OrderedFloat<f64> {
        type Sampler = UniformOrdered<f64>;
    }

    macro_rules! impl_uniform_sampler {
        ($f:ty) => {
            impl UniformSampler for UniformNotNan<$f> {
                type X = NotNan<$f>;
                fn new<B1, B2>(low: B1, high: B2) -> Self
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    UniformNotNan(UniformFloat::<$f>::new(low.borrow().0, high.borrow().0))
                }
                fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    UniformSampler::new(low, high)
                }
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                    // UniformFloat.sample() will never return NaN.
                    unsafe { NotNan::new_unchecked(self.0.sample(rng)) }
                }
            }

            impl UniformSampler for UniformOrdered<$f> {
                type X = OrderedFloat<$f>;
                fn new<B1, B2>(low: B1, high: B2) -> Self
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    UniformOrdered(UniformFloat::<$f>::new(low.borrow().0, high.borrow().0))
                }
                fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    UniformSampler::new(low, high)
                }
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                    OrderedFloat(self.0.sample(rng))
                }
            }
        };
    }

    impl_uniform_sampler! { f32 }
    impl_uniform_sampler! { f64 }

    #[cfg(all(test, feature = "randtest"))]
    mod tests {
        use super::*;

        fn sample_fuzz<T>()
        where
            Standard: Distribution<NotNan<T>>,
            Open01: Distribution<NotNan<T>>,
            OpenClosed01: Distribution<NotNan<T>>,
            Standard: Distribution<OrderedFloat<T>>,
            Open01: Distribution<OrderedFloat<T>>,
            OpenClosed01: Distribution<OrderedFloat<T>>,
            T: crate::Float,
        {
            let mut rng = rand::thread_rng();
            let f1: NotNan<T> = rng.sample(Standard);
            let f2: NotNan<T> = rng.sample(Open01);
            let f3: NotNan<T> = rng.sample(OpenClosed01);
            let _: OrderedFloat<T> = rng.sample(Standard);
            let _: OrderedFloat<T> = rng.sample(Open01);
            let _: OrderedFloat<T> = rng.sample(OpenClosed01);
            assert!(!f1.into_inner().is_nan());
            assert!(!f2.into_inner().is_nan());
            assert!(!f3.into_inner().is_nan());
        }

        #[test]
        fn sampling_f32_does_not_panic() {
            sample_fuzz::<f32>();
        }

        #[test]
        fn sampling_f64_does_not_panic() {
            sample_fuzz::<f64>();
        }

        #[test]
        #[should_panic]
        fn uniform_sampling_panic_on_infinity_notnan() {
            let (low, high) = (
                NotNan::new(0f64).unwrap(),
                NotNan::new(core::f64::INFINITY).unwrap(),
            );
            let uniform = Uniform::new(low, high);
            let _ = uniform.sample(&mut rand::thread_rng());
        }

        #[test]
        #[should_panic]
        fn uniform_sampling_panic_on_infinity_ordered() {
            let (low, high) = (OrderedFloat(0f64), OrderedFloat(core::f64::INFINITY));
            let uniform = Uniform::new(low, high);
            let _ = uniform.sample(&mut rand::thread_rng());
        }

        #[test]
        #[should_panic]
        fn uniform_sampling_panic_on_nan_ordered() {
            let (low, high) = (OrderedFloat(0f64), OrderedFloat(core::f64::NAN));
            let uniform = Uniform::new(low, high);
            let _ = uniform.sample(&mut rand::thread_rng());
        }
    }
}

#[cfg(feature = "proptest")]
mod impl_proptest {
    use super::{NotNan, OrderedFloat};
    use proptest::arbitrary::{Arbitrary, StrategyFor};
    use proptest::num::{f32, f64};
    use proptest::strategy::{FilterMap, Map, Strategy};
    use std::convert::TryFrom;

    macro_rules! impl_arbitrary {
        ($($f:ident),+) => {
            $(
                impl Arbitrary for NotNan<$f> {
                    type Strategy = FilterMap<StrategyFor<$f>, fn(_: $f) -> Option<NotNan<$f>>>;
                    type Parameters = <$f as Arbitrary>::Parameters;
                    fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
                        <$f>::arbitrary_with(params)
                            .prop_filter_map("filter nan values", |f| NotNan::try_from(f).ok())
                    }
                }

                impl Arbitrary for OrderedFloat<$f> {
                    type Strategy = Map<StrategyFor<$f>, fn(_: $f) -> OrderedFloat<$f>>;
                    type Parameters = <$f as Arbitrary>::Parameters;
                    fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
                        <$f>::arbitrary_with(params).prop_map(|f| OrderedFloat::from(f))
                    }
                }
            )*
        }
    }
    impl_arbitrary! { f32, f64 }
}

#[cfg(feature = "arbitrary")]
mod impl_arbitrary {
    use super::{FloatIsNan, NotNan, OrderedFloat};
    use arbitrary::{Arbitrary, Unstructured};
    use num_traits::FromPrimitive;

    macro_rules! impl_arbitrary {
        ($($f:ident),+) => {
            $(
                impl<'a> Arbitrary<'a> for NotNan<$f> {
                    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
                        let float: $f = u.arbitrary()?;
                        match NotNan::new(float) {
                            Ok(notnan_value) => Ok(notnan_value),
                            Err(FloatIsNan) => {
                                // If our arbitrary float input was a NaN (encoded by exponent = max
                                // value), then replace it with a finite float, reusing the mantissa
                                // bits.
                                //
                                // This means the output is not uniformly distributed among all
                                // possible float values, but Arbitrary makes no promise that that
                                // is true.
                                //
                                // An alternative implementation would be to return an
                                // `arbitrary::Error`, but that is not as useful since it forces the
                                // caller to retry with new random/fuzzed data; and the precendent of
                                // `arbitrary`'s built-in implementations is to prefer the approach of
                                // mangling the input bits to fit.

                                let (mantissa, _exponent, sign) =
                                    num_traits::Float::integer_decode(float);
                                let revised_float = <$f>::from_i64(
                                    i64::from(sign) * mantissa as i64
                                ).unwrap();

                                // If this unwrap() fails, then there is a bug in the above code.
                                Ok(NotNan::new(revised_float).unwrap())
                            }
                        }
                    }

                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        <$f as Arbitrary>::size_hint(depth)
                    }
                }

                impl<'a> Arbitrary<'a> for OrderedFloat<$f> {
                    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
                        let float: $f = u.arbitrary()?;
                        Ok(OrderedFloat::from(float))
                    }

                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        <$f as Arbitrary>::size_hint(depth)
                    }
                }
            )*
        }
    }
    impl_arbitrary! { f32, f64 }
}
