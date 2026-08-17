//! Traits for abstracting over numeric types.
//!
//! These traits describe various numeric properties and operations. They are
//! similar in purpose to the immensely helpful traits in
//! [`num-traits`](https://crates.io/crates/num-traits/), but the structure is
//! different. The philosophy behind this module is to focus on capabilities,
//! rather than categories, and to assume as little as possible. Within reason.
//!
//! Instead of having large traits with a lot of methods and dependencies, each
//! operation (or group of operations), are separated into their own traits.
//! This allows number types to have partial compatibility by only implementing
//! some of the traits, and new methods can be added as new traits without
//! affecting old functionality.

use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::bool_mask::HasBoolMask;

#[cfg(all(not(feature = "std"), feature = "libm"))]
mod libm;
#[cfg(feature = "wide")]
mod wide;

/// Numbers that belong to the real number set. It's both a semantic marker and
/// provides a constructor for number constants.
pub trait Real {
    /// Create a number from an `f64` value, mainly for converting constants.
    #[must_use]
    fn from_f64(n: f64) -> Self;
}

/// Trait for creating a vectorized value from a scalar value.
pub trait FromScalar {
    /// The scalar type that is stored in each lane of `Self`. Scalar types
    /// should set this to equal `Self`.
    type Scalar;

    /// Create a new vectorized value where each lane is `scalar`. This
    /// corresponds to `splat` for SIMD types.
    #[must_use]
    fn from_scalar(scalar: Self::Scalar) -> Self;
}

/// Conversion from an array of scalars to a vectorized value.
pub trait FromScalarArray<const N: usize>: FromScalar {
    /// Creates a vectorized value from an array of scalars.
    #[must_use]
    fn from_array(scalars: [Self::Scalar; N]) -> Self;
}

/// Conversion from a vectorized value to an array of scalars.
pub trait IntoScalarArray<const N: usize>: FromScalar {
    /// Creates an array of scalars from a vectorized value.
    #[must_use]
    fn into_array(self) -> [Self::Scalar; N];
}

/// Methods for the value `0`.
pub trait Zero {
    /// Create a new `0` value.
    #[must_use]
    fn zero() -> Self;
}

/// Methods for the value `1`.
pub trait One {
    /// Create a new `1` value.
    #[must_use]
    fn one() -> Self;
}

/// A helper trait that collects arithmetic traits under one name.
pub trait Arithmetics
where
    Self: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Neg<Output = Self>
        + Sized,
    for<'a> Self: Add<&'a Self, Output = Self>
        + Sub<&'a Self, Output = Self>
        + Mul<&'a Self, Output = Self>
        + Div<&'a Self, Output = Self>,
{
}

impl<T> Arithmetics for T
where
    T: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Neg<Output = Self>
        + Sized,
    for<'a> Self: Add<&'a Self, Output = Self>
        + Sub<&'a Self, Output = Self>
        + Mul<&'a Self, Output = Self>
        + Div<&'a Self, Output = Self>,
{
}

/// Methods for getting the largest or smallest of two values.
pub trait MinMax: Sized {
    /// Return the smallest of `self` and `other`.
    #[must_use]
    fn min(self, other: Self) -> Self;

    /// Return the largest of `self` and `other`.
    #[must_use]
    fn max(self, other: Self) -> Self;

    /// Return a pair of `self` and `other`, where the smallest is the first
    /// value and the largest is the second.
    #[must_use]
    fn min_max(self, other: Self) -> (Self, Self);
}

/// Trigonometry methods and their inverses.
pub trait Trigonometry: Sized {
    /// Compute the sine of `self` (in radians).
    #[must_use]
    fn sin(self) -> Self;

    /// Compute the cosine of `self` (in radians).
    #[must_use]
    fn cos(self) -> Self;

    /// Simultaneously compute the sine and cosine of `self` (in radians).
    /// Returns `(sin(self), cos(self))`.
    #[must_use]
    fn sin_cos(self) -> (Self, Self);

    /// Compute the tangent of `self` (in radians).
    #[must_use]
    fn tan(self) -> Self;

    /// Compute the arcsine in radians of `self`.
    #[must_use]
    fn asin(self) -> Self;

    /// Compute the arccosine in radians of `self`.
    #[must_use]
    fn acos(self) -> Self;

    /// Compute the arctangent in radians of `self`.
    #[must_use]
    fn atan(self) -> Self;

    /// Compute the arctangent in radians of `self` (y) and `other` (x).
    #[must_use]
    fn atan2(self, other: Self) -> Self;
}

/// Method for getting the absolute value of a number.
pub trait Abs {
    /// Returns the absolute value of `self`.
    #[must_use]
    fn abs(self) -> Self;
}

/// Method for getting the square root of a number.
pub trait Sqrt {
    /// Returns the square root of `self`.
    #[must_use]
    fn sqrt(self) -> Self;
}

/// Method for getting the cube root of a number.
pub trait Cbrt {
    /// Returns the cube root of `self`.
    #[must_use]
    fn cbrt(self) -> Self;
}

/// Method for raising a number by a real number exponent.
///
/// The name "powf" is kept for familiarity, even though the exponent doesn't
/// have to be a floating point number.
pub trait Powf {
    /// Return `self` raised to the power of `exp`.
    #[must_use]
    fn powf(self, exp: Self) -> Self;
}

/// Method for raising a number by a signed integer exponent.
pub trait Powi {
    /// Return `self` raised to the power of `exp`.
    #[must_use]
    fn powi(self, exp: i32) -> Self;
}

/// Method for raising a number by a n unsigned integer exponent.
pub trait Powu {
    /// Return `self` raised to the power of `exp`.
    #[must_use]
    fn powu(self, exp: u32) -> Self;
}

/// Method for calculating `1 / x`.
pub trait Recip {
    /// Return `1 / self`.
    #[must_use]
    fn recip(self) -> Self;
}

/// Methods for calculating `e ^ x`,
pub trait Exp {
    /// Return `e ^ self`.
    #[must_use]
    fn exp(self) -> Self;
}

/// Methods for checking if a number can be used as a divisor.
pub trait IsValidDivisor: HasBoolMask {
    /// Return `true` if `self` can be used as a divisor in `x / self`.
    ///
    /// This checks that division by `self` will result in a finite and defined
    /// value. Integers check for `self != 0`, while floating point types call
    /// [`is_normal`][std::primitive::f32::is_normal].
    #[must_use]
    fn is_valid_divisor(&self) -> Self::Mask;
}

/// Methods for calculating the lengths of a hypotenuse.
pub trait Hypot {
    /// Returns the length of the hypotenuse formed by `self` and `other`, i.e.
    /// `sqrt(self * self + other * other)`.
    #[must_use]
    fn hypot(self, other: Self) -> Self;
}

/// Methods for rounding numbers to integers.
pub trait Round {
    /// Return the nearest integer to `self`. Round half-way cases away from 0.0.
    #[must_use]
    fn round(self) -> Self;

    /// Return the largest integer less than or equal to `self`.
    #[must_use]
    fn floor(self) -> Self;

    /// Return the smallest integer greater than or equal to `self`.
    #[must_use]
    fn ceil(self) -> Self;
}

/// Trait for clamping a value.
pub trait Clamp {
    /// Clamp self to be within the range `[min, max]`.
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;

    /// Clamp self to be within the range `[min, ∞)`.
    #[must_use]
    fn clamp_min(self, min: Self) -> Self;

    /// Clamp self to be within the range `(-∞, max]`.
    #[must_use]
    fn clamp_max(self, max: Self) -> Self;
}

/// Assigning trait for clamping a value.
pub trait ClampAssign {
    /// Clamp self to be within the range `[min, max]`.
    fn clamp_assign(&mut self, min: Self, max: Self);

    /// Clamp self to be within the range `[min, ∞)`.
    fn clamp_min_assign(&mut self, min: Self);

    /// Clamp self to be within the range `(-∞, max]`.
    fn clamp_max_assign(&mut self, max: Self);
}

/// Combined multiplication and addition operation.
pub trait MulAdd {
    /// Multiplies self with `m` and add `a`, as in `(self * m) + a`.
    #[must_use]
    fn mul_add(self, m: Self, a: Self) -> Self;
}

/// Combined multiplication and subtraction operation.
pub trait MulSub {
    /// Multiplies self with `m` and subtract `s`, as in `(self * m) - s`.
    #[must_use]
    fn mul_sub(self, m: Self, s: Self) -> Self;
}

/// Saturating addition operation.
pub trait SaturatingAdd<Rhs = Self> {
    /// The resulting type.
    type Output;

    /// Returns the sum of `self` and `other`, but saturates instead of overflowing.
    #[must_use]
    fn saturating_add(self, other: Rhs) -> Self::Output;
}

/// Saturating subtraction operation.
pub trait SaturatingSub<Rhs = Self> {
    /// The resulting type.
    type Output;

    /// Returns the difference of `self` and `other`, but saturates instead of overflowing.
    #[must_use]
    fn saturating_sub(self, other: Rhs) -> Self::Output;
}

/// Trait for getting a number that represents the sign of `self`.
pub trait Signum {
    /// Returns a number that represents the sign of `self`. For floating point:
    ///
    /// * `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// * NaN if the number is NaN
    fn signum(self) -> Self;
}

/// Trait for getting the natural logarithm of `self`.
pub trait Ln {
    /// Returns the natural logarithm of `self`.
    fn ln(self) -> Self;
}

macro_rules! impl_uint {
    ($($ty: ident),+) => {
        $(
            impl FromScalar for $ty {
                type Scalar = Self;

                #[inline]
                fn from_scalar(scalar: Self) -> Self {
                    scalar
                }
            }

            impl FromScalarArray<1> for $ty {
                #[inline]
                fn from_array(scalars: [Self; 1]) -> Self {
                    let [scalar] = scalars;
                    scalar
                }
            }

            impl IntoScalarArray<1> for $ty {
                #[inline]
                fn into_array(self) -> [Self; 1] {
                    [self]
                }
            }

            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl MinMax for $ty {
                #[inline]
                fn min(self, other: Self) -> Self {
                    core::cmp::Ord::min(self, other)
                }

                #[inline]
                fn max(self, other: Self) -> Self {
                    core::cmp::Ord::max(self, other)
                }

                #[inline]
                fn min_max(self, other: Self) -> (Self, Self) {
                    if self > other {
                        (other, self)
                    } else {
                        (self, other)
                    }
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
                fn is_valid_divisor(&self) -> bool {
                    *self != 0
                }
            }

            impl Clamp for $ty {
                #[inline]
                fn clamp(self, min: Self, max: Self) -> Self {
                    core::cmp::Ord::clamp(self, min, max)
                }

                #[inline]
                fn clamp_min(self, min: Self) -> Self {
                    core::cmp::Ord::max(self, min)
                }

                #[inline]
                fn clamp_max(self, max: Self) -> Self {
                    core::cmp::Ord::min(self, max)
                }
            }

            impl ClampAssign for $ty {
                #[inline]
                fn clamp_assign(&mut self, min: Self, max: Self) {
                    *self = core::cmp::Ord::clamp(*self, min, max);
                }

                #[inline]
                fn clamp_min_assign(&mut self, min: Self) {
                    *self = core::cmp::Ord::max(*self, min);
                }

                #[inline]
                fn clamp_max_assign(&mut self, max: Self) {
                    *self = core::cmp::Ord::min(*self, max);
                }
            }

            impl MulAdd for $ty {
                #[inline]
                fn mul_add(self, m: Self, a: Self) -> Self {
                    (self * m) + a
                }
            }

            impl MulSub for $ty {
                #[inline]
                fn mul_sub(self, m: Self, s: Self) -> Self {
                    (self * m) - s
                }
            }

            impl SaturatingAdd for $ty {
                type Output = $ty;
                #[inline]
                fn saturating_add(self, other: Self) -> Self{
                    <$ty>::saturating_add(self, other)
                }
            }

            impl SaturatingSub for $ty {
                type Output = $ty;
                #[inline]
                fn saturating_sub(self, other: Self) -> Self{
                    <$ty>::saturating_sub(self, other)
                }
            }
        )+
    };
}

macro_rules! impl_float {
    ($($ty: ident),+) => {
        $(
            impl Real for $ty {
                #[inline]
                fn from_f64(n: f64) -> $ty {
                    n as $ty
                }
            }

            impl FromScalar for $ty {
                type Scalar = Self;

                #[inline]
                fn from_scalar(scalar: Self) -> Self {
                    scalar
                }
            }

            impl FromScalarArray<1> for $ty {
                #[inline]
                fn from_array(scalars: [Self; 1]) -> Self {
                    let [scalar] = scalars;
                    scalar
                }
            }

            impl IntoScalarArray<1> for $ty {
                #[inline]
                fn into_array(self) -> [Self; 1] {
                    [self]
                }
            }

            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0.0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1.0
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
                    if self > other {
                        (other, self)
                    } else {
                        (self, other)
                    }
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
                fn is_valid_divisor(&self) -> bool {
                    $ty::is_normal(*self)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
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

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Abs for $ty {
                #[inline]
                fn abs(self) -> Self {
                    $ty::abs(self)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Sqrt for $ty {
                #[inline]
                fn sqrt(self) -> Self {
                    $ty::sqrt(self)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Cbrt for $ty {
                #[inline]
                fn cbrt(self) -> Self {
                    $ty::cbrt(self)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Powf for $ty {
                #[inline]
                fn powf(self, exp: Self) -> Self {
                    $ty::powf(self, exp)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Powi for $ty {
                #[inline]
                fn powi(self, exp: i32) -> Self {
                    $ty::powi(self, exp)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Recip for $ty {
                #[inline]
                fn recip(self) -> Self {
                    $ty::recip(self)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Exp for $ty {
                #[inline]
                fn exp(self) -> Self {
                    $ty::exp(self)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Hypot for $ty {
                #[inline]
                fn hypot(self, other: Self) -> Self {
                    $ty::hypot(self, other)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Round for $ty {
                #[inline]
                fn round(self) -> Self {
                    $ty::round(self)
                }

                #[inline]
                fn floor(self) -> Self {
                    $ty::floor(self)
                }

                #[inline]
                fn ceil(self) -> Self {
                    $ty::ceil(self)
                }
            }

            impl Clamp for $ty {
                #[inline]
                fn clamp(self, min: Self, max: Self) -> Self {
                    $ty::clamp(self, min, max)
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

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl MulAdd for $ty {
                #[inline]
                fn mul_add(self, m: Self, a: Self) -> Self {
                    $ty::mul_add(self, m, a)
                }
            }

            impl MulSub for $ty {
                #[inline]
                fn mul_sub(self, m: Self, s: Self) -> Self {
                    (self * m) - s
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Signum for $ty {
                #[inline]
                fn signum(self) -> Self {
                    $ty::signum(self)
                }
            }

            #[cfg(any(feature = "std", all(test, not(feature = "libm"))))]
            impl Ln for $ty {
                #[inline]
                fn ln(self) -> Self {
                    $ty::ln(self)
                }
            }
        )+
    };
}

impl_uint!(u8, u16, u32, u64, u128);
impl_float!(f32, f64);

/// "borrowed" from num_traits
///
/// Raises a value to the power of exp, using exponentiation by squaring.
///
/// Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined.
//
// # Example
//
// ```rust
// use num_traits::pow;
//
// assert_eq!(pow(2i8, 4), 16);
// assert_eq!(pow(6u8, 3), 216);
// assert_eq!(pow(0u8, 0), 1); // Be aware if this case affects you
// ```
#[inline]
fn pow<T: Clone + One + Mul<T, Output = T>>(mut base: T, mut exp: u32) -> T {
    if exp == 0 {
        return T::one();
    }

    while exp & 1 == 0 {
        base = base.clone() * base;
        exp >>= 1;
    }
    if exp == 1 {
        return base;
    }

    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = base.clone() * base;
        if exp & 1 == 1 {
            acc = acc * base.clone();
        }
    }
    acc
}

/// Trait for lanewise comparison of two values.
///
/// This is similar to `PartialEq` and `PartialOrd`, except that it returns a
/// Boolean mask instead of `bool` or [`Ordering`][core::cmp::Ordering].
pub trait PartialCmp: HasBoolMask {
    /// Compares `self < other`.
    #[must_use]
    fn lt(&self, other: &Self) -> Self::Mask;

    /// Compares `self <= other`.
    #[must_use]
    fn lt_eq(&self, other: &Self) -> Self::Mask;

    /// Compares `self == other`.
    #[must_use]
    fn eq(&self, other: &Self) -> Self::Mask;

    /// Compares `self != other`.
    #[must_use]
    fn neq(&self, other: &Self) -> Self::Mask;

    /// Compares `self >= other`.
    #[must_use]
    fn gt_eq(&self, other: &Self) -> Self::Mask;

    /// Compares `self > other`.
    #[must_use]
    fn gt(&self, other: &Self) -> Self::Mask;
}

macro_rules! impl_partial_cmp {
    ($($ty:ident),+) => {
        $(
            impl PartialCmp for $ty {
                #[inline]
                fn lt(&self, other: &Self) -> Self::Mask {
                    self < other
                }

                #[inline]
                fn lt_eq(&self, other: &Self) -> Self::Mask {
                    self <= other
                }

                #[inline]
                fn eq(&self, other: &Self) -> Self::Mask {
                    self == other
                }

                #[inline]
                fn neq(&self, other: &Self) -> Self::Mask {
                    self != other
                }

                #[inline]
                fn gt_eq(&self, other: &Self) -> Self::Mask {
                    self >= other
                }

                #[inline]
                fn gt(&self, other: &Self) -> Self::Mask {
                    self > other
                }
            }
        )+
    };
}

impl_partial_cmp!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
