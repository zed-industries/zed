// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::UnknownUnit;
use crate::approxord::{max, min};
use crate::length::Length;
use crate::num::*;
use crate::scale::Scale;
use crate::vector::{vec2, BoolVector2D, Vector2D};
use crate::vector::{vec3, BoolVector3D, Vector3D};

use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::iter::Sum;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "mint")]
use mint;
use num_traits::{Float, NumCast, Signed};
#[cfg(feature = "serde")]
use serde;

/// A 2d size tagged with a unit.
#[repr(C)]
pub struct Size2D<T, U> {
    /// The extent of the element in the `U` units along the `x` axis (usually horizontal).
    pub width: T,
    /// The extent of the element in the `U` units along the `y` axis (usually vertical).
    pub height: T,
    #[doc(hidden)]
    pub _unit: PhantomData<U>,
}

impl<T: Copy, U> Copy for Size2D<T, U> {}

impl<T: Clone, U> Clone for Size2D<T, U> {
    fn clone(&self) -> Self {
        Size2D {
            width: self.width.clone(),
            height: self.height.clone(),
            _unit: PhantomData,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T, U> serde::Deserialize<'de> for Size2D<T, U>
where
    T: serde::Deserialize<'de>,
{
    /// Deserializes 2d size from tuple of width and height.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (width, height) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Size2D {
            width,
            height,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
impl<T, U> serde::Serialize for Size2D<T, U>
where
    T: serde::Serialize,
{
    /// Serializes 2d size to tuple of width and height.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.width, &self.height).serialize(serializer)
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T, U> arbitrary::Arbitrary<'a> for Size2D<T, U>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (width, height) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(Size2D {
            width,
            height,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, U> Zeroable for Size2D<T, U> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, U: 'static> Pod for Size2D<T, U> {}

impl<T, U> Eq for Size2D<T, U> where T: Eq {}

impl<T, U> PartialEq for Size2D<T, U>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl<T, U> Hash for Size2D<T, U>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.width.hash(h);
        self.height.hash(h);
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Size2D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.width, f)?;
        write!(f, "x")?;
        fmt::Debug::fmt(&self.height, f)
    }
}

impl<T: Default, U> Default for Size2D<T, U> {
    fn default() -> Self {
        Size2D::new(Default::default(), Default::default())
    }
}

impl<T, U> Size2D<T, U> {
    /// The same as [`Zero::zero`] but available without importing trait.
    ///
    /// [`Zero::zero`]: crate::num::Zero::zero
    #[inline]
    pub fn zero() -> Self
    where
        T: Zero,
    {
        Size2D::new(Zero::zero(), Zero::zero())
    }

    /// Constructor taking scalar values.
    #[inline]
    pub const fn new(width: T, height: T) -> Self {
        Size2D {
            width,
            height,
            _unit: PhantomData,
        }
    }
    /// Constructor taking scalar strongly typed lengths.
    #[inline]
    pub fn from_lengths(width: Length<T, U>, height: Length<T, U>) -> Self {
        Size2D::new(width.0, height.0)
    }

    /// Constructor setting all components to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Size2D {
            width: v.clone(),
            height: v,
            _unit: PhantomData,
        }
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(p: Size2D<T, UnknownUnit>) -> Self {
        Size2D::new(p.width, p.height)
    }
}

impl<T: Copy, U> Size2D<T, U> {
    /// Return this size as an array of two elements (width, then height).
    #[inline]
    pub fn to_array(self) -> [T; 2] {
        [self.width, self.height]
    }

    /// Return this size as a tuple of two elements (width, then height).
    #[inline]
    pub fn to_tuple(self) -> (T, T) {
        (self.width, self.height)
    }

    /// Return this size as a vector with width and height.
    #[inline]
    pub fn to_vector(self) -> Vector2D<T, U> {
        vec2(self.width, self.height)
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(self) -> Size2D<T, UnknownUnit> {
        self.cast_unit()
    }

    /// Cast the unit
    #[inline]
    pub fn cast_unit<V>(self) -> Size2D<T, V> {
        Size2D::new(self.width, self.height)
    }

    /// Rounds each component to the nearest integer value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    ///
    /// ```rust
    /// # use euclid::size2;
    /// enum Mm {}
    ///
    /// assert_eq!(size2::<_, Mm>(-0.1, -0.8).round(), size2::<_, Mm>(0.0, -1.0))
    /// ```
    #[inline]
    #[must_use]
    pub fn round(self) -> Self
    where
        T: Round,
    {
        Size2D::new(self.width.round(), self.height.round())
    }

    /// Rounds each component to the smallest integer equal or greater than the original value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    ///
    /// ```rust
    /// # use euclid::size2;
    /// enum Mm {}
    ///
    /// assert_eq!(size2::<_, Mm>(-0.1, -0.8).ceil(), size2::<_, Mm>(0.0, 0.0))
    /// ```
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self
    where
        T: Ceil,
    {
        Size2D::new(self.width.ceil(), self.height.ceil())
    }

    /// Rounds each component to the biggest integer equal or lower than the original value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    ///
    /// ```rust
    /// # use euclid::size2;
    /// enum Mm {}
    ///
    /// assert_eq!(size2::<_, Mm>(-0.1, -0.8).floor(), size2::<_, Mm>(-1.0, -1.0))
    /// ```
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self
    where
        T: Floor,
    {
        Size2D::new(self.width.floor(), self.height.floor())
    }

    /// Returns result of multiplication of both components
    pub fn area(self) -> T::Output
    where
        T: Mul,
    {
        self.width * self.height
    }

    /// Linearly interpolate each component between this size and another size.
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::size2;
    /// use euclid::default::Size2D;
    ///
    /// let from: Size2D<_> = size2(0.0, 10.0);
    /// let to:  Size2D<_> = size2(8.0, -4.0);
    ///
    /// assert_eq!(from.lerp(to, -1.0), size2(-8.0,  24.0));
    /// assert_eq!(from.lerp(to,  0.0), size2( 0.0,  10.0));
    /// assert_eq!(from.lerp(to,  0.5), size2( 4.0,   3.0));
    /// assert_eq!(from.lerp(to,  1.0), size2( 8.0,  -4.0));
    /// assert_eq!(from.lerp(to,  2.0), size2(16.0, -18.0));
    /// ```
    #[inline]
    pub fn lerp(self, other: Self, t: T) -> Self
    where
        T: One + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
    {
        let one_t = T::one() - t;
        self * one_t + other * t
    }
}

impl<T: NumCast + Copy, U> Size2D<T, U> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn cast<NewT: NumCast>(self) -> Size2D<NewT, U> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    pub fn try_cast<NewT: NumCast>(self) -> Option<Size2D<NewT, U>> {
        match (NumCast::from(self.width), NumCast::from(self.height)) {
            (Some(w), Some(h)) => Some(Size2D::new(w, h)),
            _ => None,
        }
    }

    // Convenience functions for common casts

    /// Cast into an `f32` size.
    #[inline]
    pub fn to_f32(self) -> Size2D<f32, U> {
        self.cast()
    }

    /// Cast into an `f64` size.
    #[inline]
    pub fn to_f64(self) -> Size2D<f64, U> {
        self.cast()
    }

    /// Cast into an `uint` size, truncating decimals if any.
    ///
    /// When casting from floating point sizes, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_usize(self) -> Size2D<usize, U> {
        self.cast()
    }

    /// Cast into an `u32` size, truncating decimals if any.
    ///
    /// When casting from floating point sizes, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_u32(self) -> Size2D<u32, U> {
        self.cast()
    }

    /// Cast into an `u64` size, truncating decimals if any.
    ///
    /// When casting from floating point sizes, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_u64(self) -> Size2D<u64, U> {
        self.cast()
    }

    /// Cast into an `i32` size, truncating decimals if any.
    ///
    /// When casting from floating point sizes, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i32(self) -> Size2D<i32, U> {
        self.cast()
    }

    /// Cast into an `i64` size, truncating decimals if any.
    ///
    /// When casting from floating point sizes, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i64(self) -> Size2D<i64, U> {
        self.cast()
    }
}

impl<T: Float, U> Size2D<T, U> {
    /// Returns `true` if all members are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.width.is_finite() && self.height.is_finite()
    }
}

impl<T: Signed, U> Size2D<T, U> {
    /// Computes the absolute value of each component.
    ///
    /// For `f32` and `f64`, `NaN` will be returned for component if the component is `NaN`.
    ///
    /// For signed integers, `::MIN` will be returned for component if the component is `::MIN`.
    pub fn abs(self) -> Self {
        size2(self.width.abs(), self.height.abs())
    }

    /// Returns `true` if both components is positive and `false` any component is zero or negative.
    pub fn is_positive(self) -> bool {
        self.width.is_positive() && self.height.is_positive()
    }
}

impl<T: PartialOrd, U> Size2D<T, U> {
    /// Returns the size each component of which are minimum of this size and another.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        size2(min(self.width, other.width), min(self.height, other.height))
    }

    /// Returns the size each component of which are maximum of this size and another.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        size2(max(self.width, other.width), max(self.height, other.height))
    }

    /// Returns the size each component of which clamped by corresponding
    /// components of `start` and `end`.
    ///
    /// Shortcut for `self.max(start).min(end)`.
    #[inline]
    pub fn clamp(self, start: Self, end: Self) -> Self
    where
        T: Copy,
    {
        self.max(start).min(end)
    }

    // Returns true if this size is larger or equal to the other size in all dimensions.
    #[inline]
    pub fn contains(self, other: Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    /// Returns vector with results of "greater then" operation on each component.
    pub fn greater_than(self, other: Self) -> BoolVector2D {
        BoolVector2D {
            x: self.width > other.width,
            y: self.height > other.height,
        }
    }

    /// Returns vector with results of "lower then" operation on each component.
    pub fn lower_than(self, other: Self) -> BoolVector2D {
        BoolVector2D {
            x: self.width < other.width,
            y: self.height < other.height,
        }
    }

    /// Returns `true` if any component of size is zero, negative, or NaN.
    pub fn is_empty(self) -> bool
    where
        T: Zero,
    {
        let zero = T::zero();
        // The condition is expressed this way so that we return true in
        // the presence of NaN.
        !(self.width > zero && self.height > zero)
    }
}

impl<T: PartialEq, U> Size2D<T, U> {
    /// Returns vector with results of "equal" operation on each component.
    pub fn equal(self, other: Self) -> BoolVector2D {
        BoolVector2D {
            x: self.width == other.width,
            y: self.height == other.height,
        }
    }

    /// Returns vector with results of "not equal" operation on each component.
    pub fn not_equal(self, other: Self) -> BoolVector2D {
        BoolVector2D {
            x: self.width != other.width,
            y: self.height != other.height,
        }
    }
}

impl<T: Round, U> Round for Size2D<T, U> {
    /// See [`Size2D::round`].
    #[inline]
    fn round(self) -> Self {
        self.round()
    }
}

impl<T: Ceil, U> Ceil for Size2D<T, U> {
    /// See [`Size2D::ceil`].
    #[inline]
    fn ceil(self) -> Self {
        self.ceil()
    }
}

impl<T: Floor, U> Floor for Size2D<T, U> {
    /// See [`Size2D::floor`].
    #[inline]
    fn floor(self) -> Self {
        self.floor()
    }
}

impl<T: Zero, U> Zero for Size2D<T, U> {
    #[inline]
    fn zero() -> Self {
        Size2D::new(Zero::zero(), Zero::zero())
    }
}

impl<T: Neg, U> Neg for Size2D<T, U> {
    type Output = Size2D<T::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Size2D::new(-self.width, -self.height)
    }
}

impl<T: Add, U> Add for Size2D<T, U> {
    type Output = Size2D<T::Output, U>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Size2D::new(self.width + other.width, self.height + other.height)
    }
}

impl<T: Copy + Add<T, Output = T>, U> Add<&Self> for Size2D<T, U> {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        Size2D::new(self.width + other.width, self.height + other.height)
    }
}

impl<T: Add<Output = T> + Zero, U> Sum for Size2D<T, U> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T: 'a + Add<Output = T> + Copy + Zero, U: 'a> Sum<&'a Self> for Size2D<T, U> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<T: AddAssign, U> AddAssign for Size2D<T, U> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl<T: Sub, U> Sub for Size2D<T, U> {
    type Output = Size2D<T::Output, U>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Size2D::new(self.width - other.width, self.height - other.height)
    }
}

impl<T: SubAssign, U> SubAssign for Size2D<T, U> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.width -= other.width;
        self.height -= other.height;
    }
}

impl<T: Copy + Mul, U> Mul<T> for Size2D<T, U> {
    type Output = Size2D<T::Output, U>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Size2D::new(self.width * scale, self.height * scale)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Size2D<T, U> {
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.width *= other;
        self.height *= other;
    }
}

impl<T: Copy + Mul, U1, U2> Mul<Scale<T, U1, U2>> for Size2D<T, U1> {
    type Output = Size2D<T::Output, U2>;

    #[inline]
    fn mul(self, scale: Scale<T, U1, U2>) -> Self::Output {
        Size2D::new(self.width * scale.0, self.height * scale.0)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<Scale<T, U, U>> for Size2D<T, U> {
    #[inline]
    fn mul_assign(&mut self, other: Scale<T, U, U>) {
        *self *= other.0;
    }
}

impl<T: Copy + Div, U> Div<T> for Size2D<T, U> {
    type Output = Size2D<T::Output, U>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Size2D::new(self.width / scale, self.height / scale)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Size2D<T, U> {
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.width /= other;
        self.height /= other;
    }
}

impl<T: Copy + Div, U1, U2> Div<Scale<T, U1, U2>> for Size2D<T, U2> {
    type Output = Size2D<T::Output, U1>;

    #[inline]
    fn div(self, scale: Scale<T, U1, U2>) -> Self::Output {
        Size2D::new(self.width / scale.0, self.height / scale.0)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<Scale<T, U, U>> for Size2D<T, U> {
    #[inline]
    fn div_assign(&mut self, other: Scale<T, U, U>) {
        *self /= other.0;
    }
}

/// Shorthand for `Size2D::new(w, h)`.
#[inline]
pub const fn size2<T, U>(w: T, h: T) -> Size2D<T, U> {
    Size2D::new(w, h)
}

#[cfg(feature = "mint")]
impl<T, U> From<mint::Vector2<T>> for Size2D<T, U> {
    #[inline]
    fn from(v: mint::Vector2<T>) -> Self {
        Size2D {
            width: v.x,
            height: v.y,
            _unit: PhantomData,
        }
    }
}
#[cfg(feature = "mint")]
impl<T, U> From<Size2D<T, U>> for mint::Vector2<T> {
    #[inline]
    fn from(s: Size2D<T, U>) -> Self {
        mint::Vector2 {
            x: s.width,
            y: s.height,
        }
    }
}

impl<T, U> From<Vector2D<T, U>> for Size2D<T, U> {
    #[inline]
    fn from(v: Vector2D<T, U>) -> Self {
        size2(v.x, v.y)
    }
}

impl<T, U> From<Size2D<T, U>> for [T; 2] {
    #[inline]
    fn from(s: Size2D<T, U>) -> Self {
        [s.width, s.height]
    }
}

impl<T, U> From<[T; 2]> for Size2D<T, U> {
    #[inline]
    fn from([w, h]: [T; 2]) -> Self {
        size2(w, h)
    }
}

impl<T, U> From<Size2D<T, U>> for (T, T) {
    #[inline]
    fn from(s: Size2D<T, U>) -> Self {
        (s.width, s.height)
    }
}

impl<T, U> From<(T, T)> for Size2D<T, U> {
    #[inline]
    fn from(tuple: (T, T)) -> Self {
        size2(tuple.0, tuple.1)
    }
}

#[cfg(test)]
mod size2d {
    use crate::default::Size2D;
    #[cfg(feature = "mint")]
    use mint;

    #[test]
    pub fn test_area() {
        let p = Size2D::new(1.5, 2.0);
        assert_eq!(p.area(), 3.0);
    }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let s1 = Size2D::new(1.0, 2.0);
        let sm: mint::Vector2<_> = s1.into();
        let s2 = Size2D::from(sm);

        assert_eq!(s1, s2);
    }

    mod ops {
        use crate::default::Size2D;
        use crate::scale::Scale;

        pub enum Mm {}
        pub enum Cm {}

        pub type Size2DMm<T> = crate::Size2D<T, Mm>;
        pub type Size2DCm<T> = crate::Size2D<T, Cm>;

        #[test]
        pub fn test_neg() {
            assert_eq!(-Size2D::new(1.0, 2.0), Size2D::new(-1.0, -2.0));
            assert_eq!(-Size2D::new(0.0, 0.0), Size2D::new(-0.0, -0.0));
            assert_eq!(-Size2D::new(-1.0, -2.0), Size2D::new(1.0, 2.0));
        }

        #[test]
        pub fn test_add() {
            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(3.0, 4.0);
            assert_eq!(s1 + s2, Size2D::new(4.0, 6.0));
            assert_eq!(s1 + &s2, Size2D::new(4.0, 6.0));

            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(0.0, 0.0);
            assert_eq!(s1 + s2, Size2D::new(1.0, 2.0));
            assert_eq!(s1 + &s2, Size2D::new(1.0, 2.0));

            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(-3.0, -4.0);
            assert_eq!(s1 + s2, Size2D::new(-2.0, -2.0));
            assert_eq!(s1 + &s2, Size2D::new(-2.0, -2.0));

            let s1 = Size2D::new(0.0, 0.0);
            let s2 = Size2D::new(0.0, 0.0);
            assert_eq!(s1 + s2, Size2D::new(0.0, 0.0));
            assert_eq!(s1 + &s2, Size2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_add_assign() {
            let mut s = Size2D::new(1.0, 2.0);
            s += Size2D::new(3.0, 4.0);
            assert_eq!(s, Size2D::new(4.0, 6.0));

            let mut s = Size2D::new(1.0, 2.0);
            s += Size2D::new(0.0, 0.0);
            assert_eq!(s, Size2D::new(1.0, 2.0));

            let mut s = Size2D::new(1.0, 2.0);
            s += Size2D::new(-3.0, -4.0);
            assert_eq!(s, Size2D::new(-2.0, -2.0));

            let mut s = Size2D::new(0.0, 0.0);
            s += Size2D::new(0.0, 0.0);
            assert_eq!(s, Size2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_sum() {
            let sizes = [
                Size2D::new(0.0, 1.0),
                Size2D::new(1.0, 2.0),
                Size2D::new(2.0, 3.0),
            ];
            let sum = Size2D::new(3.0, 6.0);
            assert_eq!(sizes.iter().sum::<Size2D<_>>(), sum);
        }

        #[test]
        pub fn test_sub() {
            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(3.0, 4.0);
            assert_eq!(s1 - s2, Size2D::new(-2.0, -2.0));

            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(0.0, 0.0);
            assert_eq!(s1 - s2, Size2D::new(1.0, 2.0));

            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(-3.0, -4.0);
            assert_eq!(s1 - s2, Size2D::new(4.0, 6.0));

            let s1 = Size2D::new(0.0, 0.0);
            let s2 = Size2D::new(0.0, 0.0);
            assert_eq!(s1 - s2, Size2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_sub_assign() {
            let mut s = Size2D::new(1.0, 2.0);
            s -= Size2D::new(3.0, 4.0);
            assert_eq!(s, Size2D::new(-2.0, -2.0));

            let mut s = Size2D::new(1.0, 2.0);
            s -= Size2D::new(0.0, 0.0);
            assert_eq!(s, Size2D::new(1.0, 2.0));

            let mut s = Size2D::new(1.0, 2.0);
            s -= Size2D::new(-3.0, -4.0);
            assert_eq!(s, Size2D::new(4.0, 6.0));

            let mut s = Size2D::new(0.0, 0.0);
            s -= Size2D::new(0.0, 0.0);
            assert_eq!(s, Size2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_mul_scalar() {
            let s1: Size2D<f32> = Size2D::new(3.0, 5.0);

            let result = s1 * 5.0;

            assert_eq!(result, Size2D::new(15.0, 25.0));
        }

        #[test]
        pub fn test_mul_assign_scalar() {
            let mut s1 = Size2D::new(3.0, 5.0);

            s1 *= 5.0;

            assert_eq!(s1, Size2D::new(15.0, 25.0));
        }

        #[test]
        pub fn test_mul_scale() {
            let s1 = Size2DMm::new(1.0, 2.0);
            let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

            let result = s1 * cm_per_mm;

            assert_eq!(result, Size2DCm::new(0.1, 0.2));
        }

        #[test]
        pub fn test_mul_assign_scale() {
            let mut s1 = Size2DMm::new(1.0, 2.0);
            let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

            s1 *= scale;

            assert_eq!(s1, Size2DMm::new(0.1, 0.2));
        }

        #[test]
        pub fn test_div_scalar() {
            let s1: Size2D<f32> = Size2D::new(15.0, 25.0);

            let result = s1 / 5.0;

            assert_eq!(result, Size2D::new(3.0, 5.0));
        }

        #[test]
        pub fn test_div_assign_scalar() {
            let mut s1: Size2D<f32> = Size2D::new(15.0, 25.0);

            s1 /= 5.0;

            assert_eq!(s1, Size2D::new(3.0, 5.0));
        }

        #[test]
        pub fn test_div_scale() {
            let s1 = Size2DCm::new(0.1, 0.2);
            let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

            let result = s1 / cm_per_mm;

            assert_eq!(result, Size2DMm::new(1.0, 2.0));
        }

        #[test]
        pub fn test_div_assign_scale() {
            let mut s1 = Size2DMm::new(0.1, 0.2);
            let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

            s1 /= scale;

            assert_eq!(s1, Size2DMm::new(1.0, 2.0));
        }

        #[test]
        pub fn test_nan_empty() {
            use std::f32::NAN;
            assert!(Size2D::new(NAN, 2.0).is_empty());
            assert!(Size2D::new(0.0, NAN).is_empty());
            assert!(Size2D::new(NAN, -2.0).is_empty());
        }
    }
}

/// A 3d size tagged with a unit.
#[repr(C)]
pub struct Size3D<T, U> {
    /// The extent of the element in the `U` units along the `x` axis.
    pub width: T,
    /// The extent of the element in the `U` units along the `y` axis.
    pub height: T,
    /// The extent of the element in the `U` units along the `z` axis.
    pub depth: T,
    #[doc(hidden)]
    pub _unit: PhantomData<U>,
}

impl<T: Copy, U> Copy for Size3D<T, U> {}

impl<T: Clone, U> Clone for Size3D<T, U> {
    fn clone(&self) -> Self {
        Size3D {
            width: self.width.clone(),
            height: self.height.clone(),
            depth: self.depth.clone(),
            _unit: PhantomData,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T, U> serde::Deserialize<'de> for Size3D<T, U>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (width, height, depth) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Size3D {
            width,
            height,
            depth,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
impl<T, U> serde::Serialize for Size3D<T, U>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.width, &self.height, &self.depth).serialize(serializer)
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T, U> arbitrary::Arbitrary<'a> for Size3D<T, U>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (width, height, depth) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(Size3D {
            width,
            height,
            depth,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, U> Zeroable for Size3D<T, U> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, U: 'static> Pod for Size3D<T, U> {}

impl<T, U> Eq for Size3D<T, U> where T: Eq {}

impl<T, U> PartialEq for Size3D<T, U>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.depth == other.depth
    }
}

impl<T, U> Hash for Size3D<T, U>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.width.hash(h);
        self.height.hash(h);
        self.depth.hash(h);
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Size3D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.width, f)?;
        write!(f, "x")?;
        fmt::Debug::fmt(&self.height, f)?;
        write!(f, "x")?;
        fmt::Debug::fmt(&self.depth, f)
    }
}

impl<T: Default, U> Default for Size3D<T, U> {
    fn default() -> Self {
        Size3D::new(Default::default(), Default::default(), Default::default())
    }
}

impl<T, U> Size3D<T, U> {
    /// The same as [`Zero::zero`] but available without importing trait.
    ///
    /// [`Zero::zero`]: crate::num::Zero::zero
    pub fn zero() -> Self
    where
        T: Zero,
    {
        Size3D::new(Zero::zero(), Zero::zero(), Zero::zero())
    }

    /// Constructor taking scalar values.
    #[inline]
    pub const fn new(width: T, height: T, depth: T) -> Self {
        Size3D {
            width,
            height,
            depth,
            _unit: PhantomData,
        }
    }
    /// Constructor taking scalar strongly typed lengths.
    #[inline]
    pub fn from_lengths(width: Length<T, U>, height: Length<T, U>, depth: Length<T, U>) -> Self {
        Size3D::new(width.0, height.0, depth.0)
    }

    /// Constructor setting all components to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Size3D {
            width: v.clone(),
            height: v.clone(),
            depth: v,
            _unit: PhantomData,
        }
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(p: Size3D<T, UnknownUnit>) -> Self {
        Size3D::new(p.width, p.height, p.depth)
    }
}

impl<T: Copy, U> Size3D<T, U> {
    /// Return this size as an array of three elements (width, then height, then depth).
    #[inline]
    pub fn to_array(self) -> [T; 3] {
        [self.width, self.height, self.depth]
    }

    /// Return this size as an array of three elements (width, then height, then depth).
    #[inline]
    pub fn to_tuple(self) -> (T, T, T) {
        (self.width, self.height, self.depth)
    }

    /// Return this size as a vector with width, height and depth.
    #[inline]
    pub fn to_vector(self) -> Vector3D<T, U> {
        vec3(self.width, self.height, self.depth)
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(self) -> Size3D<T, UnknownUnit> {
        self.cast_unit()
    }

    /// Cast the unit
    #[inline]
    pub fn cast_unit<V>(self) -> Size3D<T, V> {
        Size3D::new(self.width, self.height, self.depth)
    }

    /// Rounds each component to the nearest integer value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    ///
    /// ```rust
    /// # use euclid::size3;
    /// enum Mm {}
    ///
    /// assert_eq!(size3::<_, Mm>(-0.1, -0.8, 0.4).round(), size3::<_, Mm>(0.0, -1.0, 0.0))
    /// ```
    #[inline]
    #[must_use]
    pub fn round(self) -> Self
    where
        T: Round,
    {
        Size3D::new(self.width.round(), self.height.round(), self.depth.round())
    }

    /// Rounds each component to the smallest integer equal or greater than the original value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    ///
    /// ```rust
    /// # use euclid::size3;
    /// enum Mm {}
    ///
    /// assert_eq!(size3::<_, Mm>(-0.1, -0.8, 0.4).ceil(), size3::<_, Mm>(0.0, 0.0, 1.0))
    /// ```
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self
    where
        T: Ceil,
    {
        Size3D::new(self.width.ceil(), self.height.ceil(), self.depth.ceil())
    }

    /// Rounds each component to the biggest integer equal or lower than the original value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    ///
    /// ```rust
    /// # use euclid::size3;
    /// enum Mm {}
    ///
    /// assert_eq!(size3::<_, Mm>(-0.1, -0.8, 0.4).floor(), size3::<_, Mm>(-1.0, -1.0, 0.0))
    /// ```
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self
    where
        T: Floor,
    {
        Size3D::new(self.width.floor(), self.height.floor(), self.depth.floor())
    }

    /// Returns result of multiplication of all components
    pub fn volume(self) -> T
    where
        T: Mul<Output = T>,
    {
        self.width * self.height * self.depth
    }

    /// Linearly interpolate between this size and another size.
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::size3;
    /// use euclid::default::Size3D;
    ///
    /// let from: Size3D<_> = size3(0.0, 10.0, -1.0);
    /// let to:  Size3D<_> = size3(8.0, -4.0,  0.0);
    ///
    /// assert_eq!(from.lerp(to, -1.0), size3(-8.0,  24.0, -2.0));
    /// assert_eq!(from.lerp(to,  0.0), size3( 0.0,  10.0, -1.0));
    /// assert_eq!(from.lerp(to,  0.5), size3( 4.0,   3.0, -0.5));
    /// assert_eq!(from.lerp(to,  1.0), size3( 8.0,  -4.0,  0.0));
    /// assert_eq!(from.lerp(to,  2.0), size3(16.0, -18.0,  1.0));
    /// ```
    #[inline]
    pub fn lerp(self, other: Self, t: T) -> Self
    where
        T: One + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
    {
        let one_t = T::one() - t;
        self * one_t + other * t
    }
}

impl<T: NumCast + Copy, U> Size3D<T, U> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn cast<NewT: NumCast>(self) -> Size3D<NewT, U> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    pub fn try_cast<NewT: NumCast>(self) -> Option<Size3D<NewT, U>> {
        match (
            NumCast::from(self.width),
            NumCast::from(self.height),
            NumCast::from(self.depth),
        ) {
            (Some(w), Some(h), Some(d)) => Some(Size3D::new(w, h, d)),
            _ => None,
        }
    }

    // Convenience functions for common casts

    /// Cast into an `f32` size.
    #[inline]
    pub fn to_f32(self) -> Size3D<f32, U> {
        self.cast()
    }

    /// Cast into an `f64` size.
    #[inline]
    pub fn to_f64(self) -> Size3D<f64, U> {
        self.cast()
    }

    /// Cast into an `uint` size, truncating decimals if any.
    ///
    /// When casting from floating point sizes, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_usize(self) -> Size3D<usize, U> {
        self.cast()
    }

    /// Cast into an `u32` size, truncating decimals if any.
    ///
    /// When casting from floating point sizes, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_u32(self) -> Size3D<u32, U> {
        self.cast()
    }

    /// Cast into an `i32` size, truncating decimals if any.
    ///
    /// When casting from floating point sizes, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i32(self) -> Size3D<i32, U> {
        self.cast()
    }

    /// Cast into an `i64` size, truncating decimals if any.
    ///
    /// When casting from floating point sizes, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i64(self) -> Size3D<i64, U> {
        self.cast()
    }
}

impl<T: Float, U> Size3D<T, U> {
    /// Returns `true` if all members are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.width.is_finite() && self.height.is_finite() && self.depth.is_finite()
    }
}

impl<T: Signed, U> Size3D<T, U> {
    /// Computes the absolute value of each component.
    ///
    /// For `f32` and `f64`, `NaN` will be returned for component if the component is `NaN`.
    ///
    /// For signed integers, `::MIN` will be returned for component if the component is `::MIN`.
    pub fn abs(self) -> Self {
        size3(self.width.abs(), self.height.abs(), self.depth.abs())
    }

    /// Returns `true` if all components is positive and `false` any component is zero or negative.
    pub fn is_positive(self) -> bool {
        self.width.is_positive() && self.height.is_positive() && self.depth.is_positive()
    }
}

impl<T: PartialOrd, U> Size3D<T, U> {
    /// Returns the size each component of which are minimum of this size and another.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        size3(
            min(self.width, other.width),
            min(self.height, other.height),
            min(self.depth, other.depth),
        )
    }

    /// Returns the size each component of which are maximum of this size and another.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        size3(
            max(self.width, other.width),
            max(self.height, other.height),
            max(self.depth, other.depth),
        )
    }

    /// Returns the size each component of which clamped by corresponding
    /// components of `start` and `end`.
    ///
    /// Shortcut for `self.max(start).min(end)`.
    #[inline]
    pub fn clamp(self, start: Self, end: Self) -> Self
    where
        T: Copy,
    {
        self.max(start).min(end)
    }

    // Returns true if this size is larger or equal to the other size in all dimensions.
    #[inline]
    pub fn contains(self, other: Self) -> bool {
        self.width >= other.width && self.height >= other.height && self.depth >= other.depth
    }

    /// Returns vector with results of "greater than" operation on each component.
    pub fn greater_than(self, other: Self) -> BoolVector3D {
        BoolVector3D {
            x: self.width > other.width,
            y: self.height > other.height,
            z: self.depth > other.depth,
        }
    }

    /// Returns vector with results of "lower than" operation on each component.
    pub fn lower_than(self, other: Self) -> BoolVector3D {
        BoolVector3D {
            x: self.width < other.width,
            y: self.height < other.height,
            z: self.depth < other.depth,
        }
    }

    /// Returns `true` if any component of size is zero, negative or NaN.
    pub fn is_empty(self) -> bool
    where
        T: Zero,
    {
        let zero = T::zero();
        !(self.width > zero && self.height > zero && self.depth > zero)
    }
}

impl<T: PartialEq, U> Size3D<T, U> {
    /// Returns vector with results of "equal" operation on each component.
    pub fn equal(self, other: Self) -> BoolVector3D {
        BoolVector3D {
            x: self.width == other.width,
            y: self.height == other.height,
            z: self.depth == other.depth,
        }
    }

    /// Returns vector with results of "not equal" operation on each component.
    pub fn not_equal(self, other: Self) -> BoolVector3D {
        BoolVector3D {
            x: self.width != other.width,
            y: self.height != other.height,
            z: self.depth != other.depth,
        }
    }
}

impl<T: Round, U> Round for Size3D<T, U> {
    /// See [`Size3D::round`].
    #[inline]
    fn round(self) -> Self {
        self.round()
    }
}

impl<T: Ceil, U> Ceil for Size3D<T, U> {
    /// See [`Size3D::ceil`].
    #[inline]
    fn ceil(self) -> Self {
        self.ceil()
    }
}

impl<T: Floor, U> Floor for Size3D<T, U> {
    /// See [`Size3D::floor`].
    #[inline]
    fn floor(self) -> Self {
        self.floor()
    }
}

impl<T: Zero, U> Zero for Size3D<T, U> {
    #[inline]
    fn zero() -> Self {
        Size3D::new(Zero::zero(), Zero::zero(), Zero::zero())
    }
}

impl<T: Neg, U> Neg for Size3D<T, U> {
    type Output = Size3D<T::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Size3D::new(-self.width, -self.height, -self.depth)
    }
}

impl<T: Add, U> Add for Size3D<T, U> {
    type Output = Size3D<T::Output, U>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Size3D::new(
            self.width + other.width,
            self.height + other.height,
            self.depth + other.depth,
        )
    }
}

impl<T: Copy + Add<T, Output = T>, U> Add<&Self> for Size3D<T, U> {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        Size3D::new(
            self.width + other.width,
            self.height + other.height,
            self.depth + other.depth,
        )
    }
}

impl<T: Add<Output = T> + Zero, U> Sum for Size3D<T, U> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T: 'a + Add<Output = T> + Copy + Zero, U: 'a> Sum<&'a Self> for Size3D<T, U> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<T: AddAssign, U> AddAssign for Size3D<T, U> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.width += other.width;
        self.height += other.height;
        self.depth += other.depth;
    }
}

impl<T: Sub, U> Sub for Size3D<T, U> {
    type Output = Size3D<T::Output, U>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Size3D::new(
            self.width - other.width,
            self.height - other.height,
            self.depth - other.depth,
        )
    }
}

impl<T: SubAssign, U> SubAssign for Size3D<T, U> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.width -= other.width;
        self.height -= other.height;
        self.depth -= other.depth;
    }
}

impl<T: Copy + Mul, U> Mul<T> for Size3D<T, U> {
    type Output = Size3D<T::Output, U>;

    #[inline]
    #[rustfmt::skip]
    fn mul(self, scale: T) -> Self::Output {
        Size3D::new(
            self.width * scale,
            self.height * scale,
            self.depth * scale,
        )
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Size3D<T, U> {
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.width *= other;
        self.height *= other;
        self.depth *= other;
    }
}

impl<T: Copy + Mul, U1, U2> Mul<Scale<T, U1, U2>> for Size3D<T, U1> {
    type Output = Size3D<T::Output, U2>;

    #[inline]
    fn mul(self, scale: Scale<T, U1, U2>) -> Self::Output {
        Size3D::new(
            self.width * scale.0,
            self.height * scale.0,
            self.depth * scale.0,
        )
    }
}

impl<T: Copy + MulAssign, U> MulAssign<Scale<T, U, U>> for Size3D<T, U> {
    #[inline]
    fn mul_assign(&mut self, other: Scale<T, U, U>) {
        *self *= other.0;
    }
}

impl<T: Copy + Div, U> Div<T> for Size3D<T, U> {
    type Output = Size3D<T::Output, U>;

    #[inline]
    #[rustfmt::skip]
    fn div(self, scale: T) -> Self::Output {
        Size3D::new(
            self.width / scale,
            self.height / scale,
            self.depth / scale,
        )
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Size3D<T, U> {
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.width /= other;
        self.height /= other;
        self.depth /= other;
    }
}

impl<T: Copy + Div, U1, U2> Div<Scale<T, U1, U2>> for Size3D<T, U2> {
    type Output = Size3D<T::Output, U1>;

    #[inline]
    fn div(self, scale: Scale<T, U1, U2>) -> Self::Output {
        Size3D::new(
            self.width / scale.0,
            self.height / scale.0,
            self.depth / scale.0,
        )
    }
}

impl<T: Copy + DivAssign, U> DivAssign<Scale<T, U, U>> for Size3D<T, U> {
    #[inline]
    fn div_assign(&mut self, other: Scale<T, U, U>) {
        *self /= other.0;
    }
}

#[cfg(feature = "mint")]
impl<T, U> From<mint::Vector3<T>> for Size3D<T, U> {
    #[inline]
    fn from(v: mint::Vector3<T>) -> Self {
        size3(v.x, v.y, v.z)
    }
}
#[cfg(feature = "mint")]
impl<T, U> From<Size3D<T, U>> for mint::Vector3<T> {
    #[inline]
    fn from(s: Size3D<T, U>) -> Self {
        mint::Vector3 {
            x: s.width,
            y: s.height,
            z: s.depth,
        }
    }
}

impl<T, U> From<Vector3D<T, U>> for Size3D<T, U> {
    #[inline]
    fn from(v: Vector3D<T, U>) -> Self {
        size3(v.x, v.y, v.z)
    }
}

impl<T, U> From<Size3D<T, U>> for [T; 3] {
    #[inline]
    fn from(s: Size3D<T, U>) -> Self {
        [s.width, s.height, s.depth]
    }
}

impl<T, U> From<[T; 3]> for Size3D<T, U> {
    #[inline]
    fn from([w, h, d]: [T; 3]) -> Self {
        size3(w, h, d)
    }
}

impl<T, U> From<Size3D<T, U>> for (T, T, T) {
    #[inline]
    fn from(s: Size3D<T, U>) -> Self {
        (s.width, s.height, s.depth)
    }
}

impl<T, U> From<(T, T, T)> for Size3D<T, U> {
    #[inline]
    fn from(tuple: (T, T, T)) -> Self {
        size3(tuple.0, tuple.1, tuple.2)
    }
}

/// Shorthand for `Size3D::new(w, h, d)`.
#[inline]
pub const fn size3<T, U>(w: T, h: T, d: T) -> Size3D<T, U> {
    Size3D::new(w, h, d)
}

#[cfg(test)]
mod size3d {
    mod ops {
        use crate::default::{Size2D, Size3D};
        use crate::scale::Scale;

        pub enum Mm {}
        pub enum Cm {}

        pub type Size3DMm<T> = crate::Size3D<T, Mm>;
        pub type Size3DCm<T> = crate::Size3D<T, Cm>;

        #[test]
        pub fn test_neg() {
            assert_eq!(-Size3D::new(1.0, 2.0, 3.0), Size3D::new(-1.0, -2.0, -3.0));
            assert_eq!(-Size3D::new(0.0, 0.0, 0.0), Size3D::new(-0.0, -0.0, -0.0));
            assert_eq!(-Size3D::new(-1.0, -2.0, -3.0), Size3D::new(1.0, 2.0, 3.0));
        }

        #[test]
        pub fn test_add() {
            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(4.0, 5.0, 6.0);
            assert_eq!(s1 + s2, Size3D::new(5.0, 7.0, 9.0));
            assert_eq!(s1 + &s2, Size3D::new(5.0, 7.0, 9.0));

            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s1 + s2, Size3D::new(1.0, 2.0, 3.0));
            assert_eq!(s1 + &s2, Size3D::new(1.0, 2.0, 3.0));

            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(-4.0, -5.0, -6.0);
            assert_eq!(s1 + s2, Size3D::new(-3.0, -3.0, -3.0));
            assert_eq!(s1 + &s2, Size3D::new(-3.0, -3.0, -3.0));

            let s1 = Size3D::new(0.0, 0.0, 0.0);
            let s2 = Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s1 + s2, Size3D::new(0.0, 0.0, 0.0));
            assert_eq!(s1 + &s2, Size3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_sum() {
            let sizes = [
                Size3D::new(0.0, 1.0, 2.0),
                Size3D::new(1.0, 2.0, 3.0),
                Size3D::new(2.0, 3.0, 4.0),
            ];
            let sum = Size3D::new(3.0, 6.0, 9.0);
            assert_eq!(sizes.iter().sum::<Size3D<_>>(), sum);
        }

        #[test]
        pub fn test_add_assign() {
            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s += Size3D::new(4.0, 5.0, 6.0);
            assert_eq!(s, Size3D::new(5.0, 7.0, 9.0));

            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s += Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s, Size3D::new(1.0, 2.0, 3.0));

            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s += Size3D::new(-4.0, -5.0, -6.0);
            assert_eq!(s, Size3D::new(-3.0, -3.0, -3.0));

            let mut s = Size3D::new(0.0, 0.0, 0.0);
            s += Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s, Size3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_sub() {
            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(4.0, 5.0, 6.0);
            assert_eq!(s1 - s2, Size3D::new(-3.0, -3.0, -3.0));

            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s1 - s2, Size3D::new(1.0, 2.0, 3.0));

            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(-4.0, -5.0, -6.0);
            assert_eq!(s1 - s2, Size3D::new(5.0, 7.0, 9.0));

            let s1 = Size3D::new(0.0, 0.0, 0.0);
            let s2 = Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s1 - s2, Size3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_sub_assign() {
            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s -= Size3D::new(4.0, 5.0, 6.0);
            assert_eq!(s, Size3D::new(-3.0, -3.0, -3.0));

            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s -= Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s, Size3D::new(1.0, 2.0, 3.0));

            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s -= Size3D::new(-4.0, -5.0, -6.0);
            assert_eq!(s, Size3D::new(5.0, 7.0, 9.0));

            let mut s = Size3D::new(0.0, 0.0, 0.0);
            s -= Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s, Size3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_mul_scalar() {
            let s1: Size3D<f32> = Size3D::new(3.0, 5.0, 7.0);

            let result = s1 * 5.0;

            assert_eq!(result, Size3D::new(15.0, 25.0, 35.0));
        }

        #[test]
        pub fn test_mul_assign_scalar() {
            let mut s1: Size3D<f32> = Size3D::new(3.0, 5.0, 7.0);

            s1 *= 5.0;

            assert_eq!(s1, Size3D::new(15.0, 25.0, 35.0));
        }

        #[test]
        pub fn test_mul_scale() {
            let s1 = Size3DMm::new(1.0, 2.0, 3.0);
            let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

            let result = s1 * cm_per_mm;

            assert_eq!(result, Size3DCm::new(0.1, 0.2, 0.3));
        }

        #[test]
        pub fn test_mul_assign_scale() {
            let mut s1 = Size3DMm::new(1.0, 2.0, 3.0);
            let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

            s1 *= scale;

            assert_eq!(s1, Size3DMm::new(0.1, 0.2, 0.3));
        }

        #[test]
        pub fn test_div_scalar() {
            let s1: Size3D<f32> = Size3D::new(15.0, 25.0, 35.0);

            let result = s1 / 5.0;

            assert_eq!(result, Size3D::new(3.0, 5.0, 7.0));
        }

        #[test]
        pub fn test_div_assign_scalar() {
            let mut s1: Size3D<f32> = Size3D::new(15.0, 25.0, 35.0);

            s1 /= 5.0;

            assert_eq!(s1, Size3D::new(3.0, 5.0, 7.0));
        }

        #[test]
        pub fn test_div_scale() {
            let s1 = Size3DCm::new(0.1, 0.2, 0.3);
            let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

            let result = s1 / cm_per_mm;

            assert_eq!(result, Size3DMm::new(1.0, 2.0, 3.0));
        }

        #[test]
        pub fn test_div_assign_scale() {
            let mut s1 = Size3DMm::new(0.1, 0.2, 0.3);
            let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

            s1 /= scale;

            assert_eq!(s1, Size3DMm::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn test_nonempty() {
            assert!(!Size2D::new(1.0, 1.0).is_empty());
            assert!(!Size3D::new(1.0, 1.0, 1.0).is_empty());
        }

        #[test]
        pub fn test_nan_empty() {
            use std::f32::NAN;
            assert!(Size3D::new(NAN, 2.0, 3.0).is_empty());
            assert!(Size3D::new(0.0, NAN, 0.0).is_empty());
            assert!(Size3D::new(1.0, 2.0, NAN).is_empty());
        }
    }
}
