// Copyright 2019 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A 2D point.

use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::common::FloatExt;
use crate::{Axis, Vec2};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// A 2D point.
///
/// This type represents a point in 2D space. It has the same layout as [`Vec2`], but
/// its meaning is different: `Vec2` represents a change in location (for example velocity).
///
/// In general, `kurbo` overloads math operators where it makes sense, for example implementing
/// `Affine * Point` as the point under the affine transformation. However `Point + Point` and
/// `f64 * Point` are not implemented, because the operations do not make geometric sense. If you
/// need to apply these operations, then 1) check what you're doing makes geometric sense, then 2)
/// use [`Point::to_vec2`] to convert the point to a `Vec2`.
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    /// The x coordinate.
    pub x: f64,
    /// The y coordinate.
    pub y: f64,
}

impl Point {
    /// The point (0, 0).
    pub const ZERO: Point = Point::new(0., 0.);

    /// The point at the origin; (0, 0).
    pub const ORIGIN: Point = Point::new(0., 0.);

    /// Create a new `Point` with the provided `x` and `y` coordinates.
    #[inline(always)]
    pub const fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Convert this point into a `Vec2`.
    #[inline(always)]
    pub const fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Linearly interpolate between two points.
    #[inline]
    pub fn lerp(self, other: Point, t: f64) -> Point {
        self.to_vec2().lerp(other.to_vec2(), t).to_point()
    }

    /// Determine the midpoint of two points.
    #[inline]
    pub const fn midpoint(self, other: Point) -> Point {
        Point::new(0.5 * (self.x + other.x), 0.5 * (self.y + other.y))
    }

    /// Euclidean distance.
    ///
    /// See [`Vec2::hypot`] for the same operation on [`Vec2`].
    #[inline]
    pub fn distance(self, other: Point) -> f64 {
        (self - other).hypot()
    }

    /// Squared Euclidean distance.
    ///
    /// See [`Vec2::hypot2`] for the same operation on [`Vec2`].
    #[inline]
    pub fn distance_squared(self, other: Point) -> f64 {
        (self - other).hypot2()
    }

    /// Returns a new `Point`, with `x` and `y` [rounded] to the nearest integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Point;
    /// let a = Point::new(3.3, 3.6).round();
    /// let b = Point::new(3.0, -3.1).round();
    /// assert_eq!(a.x, 3.0);
    /// assert_eq!(a.y, 4.0);
    /// assert_eq!(b.x, 3.0);
    /// assert_eq!(b.y, -3.0);
    /// ```
    ///
    /// [rounded]: f64::round
    #[inline]
    pub fn round(self) -> Point {
        Point::new(self.x.round(), self.y.round())
    }

    /// Returns a new `Point`,
    /// with `x` and `y` [rounded up] to the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Point;
    /// let a = Point::new(3.3, 3.6).ceil();
    /// let b = Point::new(3.0, -3.1).ceil();
    /// assert_eq!(a.x, 4.0);
    /// assert_eq!(a.y, 4.0);
    /// assert_eq!(b.x, 3.0);
    /// assert_eq!(b.y, -3.0);
    /// ```
    ///
    /// [rounded up]: f64::ceil
    #[inline]
    pub fn ceil(self) -> Point {
        Point::new(self.x.ceil(), self.y.ceil())
    }

    /// Returns a new `Point`,
    /// with `x` and `y` [rounded down] to the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Point;
    /// let a = Point::new(3.3, 3.6).floor();
    /// let b = Point::new(3.0, -3.1).floor();
    /// assert_eq!(a.x, 3.0);
    /// assert_eq!(a.y, 3.0);
    /// assert_eq!(b.x, 3.0);
    /// assert_eq!(b.y, -4.0);
    /// ```
    ///
    /// [rounded down]: f64::floor
    #[inline]
    pub fn floor(self) -> Point {
        Point::new(self.x.floor(), self.y.floor())
    }

    /// Returns a new `Point`,
    /// with `x` and `y` [rounded away] from zero to the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Point;
    /// let a = Point::new(3.3, 3.6).expand();
    /// let b = Point::new(3.0, -3.1).expand();
    /// assert_eq!(a.x, 4.0);
    /// assert_eq!(a.y, 4.0);
    /// assert_eq!(b.x, 3.0);
    /// assert_eq!(b.y, -4.0);
    /// ```
    ///
    /// [rounded away]: FloatExt::expand
    #[inline]
    pub fn expand(self) -> Point {
        Point::new(self.x.expand(), self.y.expand())
    }

    /// Returns a new `Point`,
    /// with `x` and `y` [rounded towards] zero to the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Point;
    /// let a = Point::new(3.3, 3.6).trunc();
    /// let b = Point::new(3.0, -3.1).trunc();
    /// assert_eq!(a.x, 3.0);
    /// assert_eq!(a.y, 3.0);
    /// assert_eq!(b.x, 3.0);
    /// assert_eq!(b.y, -3.0);
    /// ```
    ///
    /// [rounded towards]: f64::trunc
    #[inline]
    pub fn trunc(self) -> Point {
        Point::new(self.x.trunc(), self.y.trunc())
    }

    /// Is this point [finite]?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Is this point [`NaN`]?
    ///
    /// [`NaN`]: f64::is_nan
    #[inline]
    pub const fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    /// Get the member matching the given axis.
    #[inline]
    pub const fn get_coord(self, axis: Axis) -> f64 {
        match axis {
            Axis::Horizontal => self.x,
            Axis::Vertical => self.y,
        }
    }

    /// Get a mutable reference to the member matching the given axis.
    #[inline]
    pub const fn get_coord_mut(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            Axis::Horizontal => &mut self.x,
            Axis::Vertical => &mut self.y,
        }
    }

    /// Set the member matching the given axis to the given value.
    #[inline]
    pub const fn set_coord(&mut self, axis: Axis, value: f64) {
        match axis {
            Axis::Horizontal => self.x = value,
            Axis::Vertical => self.y = value,
        }
    }
}

impl From<(f32, f32)> for Point {
    #[inline(always)]
    fn from(v: (f32, f32)) -> Point {
        Point {
            x: v.0 as f64,
            y: v.1 as f64,
        }
    }
}

impl From<(f64, f64)> for Point {
    #[inline(always)]
    fn from(v: (f64, f64)) -> Point {
        Point { x: v.0, y: v.1 }
    }
}

impl From<Point> for (f64, f64) {
    #[inline(always)]
    fn from(v: Point) -> (f64, f64) {
        (v.x, v.y)
    }
}

impl Add<Vec2> for Point {
    type Output = Point;

    #[inline]
    fn add(self, other: Vec2) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign<Vec2> for Point {
    #[inline]
    fn add_assign(&mut self, other: Vec2) {
        *self = Point::new(self.x + other.x, self.y + other.y);
    }
}

impl Sub<Vec2> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, other: Vec2) -> Self {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign<Vec2> for Point {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        *self = Point::new(self.x - other.x, self.y - other.y);
    }
}

impl Add<(f64, f64)> for Point {
    type Output = Point;

    #[inline]
    fn add(self, (x, y): (f64, f64)) -> Self {
        Point::new(self.x + x, self.y + y)
    }
}

impl AddAssign<(f64, f64)> for Point {
    #[inline]
    fn add_assign(&mut self, (x, y): (f64, f64)) {
        *self = Point::new(self.x + x, self.y + y);
    }
}

impl Sub<(f64, f64)> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, (x, y): (f64, f64)) -> Self {
        Point::new(self.x - x, self.y - y)
    }
}

impl SubAssign<(f64, f64)> for Point {
    #[inline]
    fn sub_assign(&mut self, (x, y): (f64, f64)) {
        *self = Point::new(self.x - x, self.y - y);
    }
}

impl Sub<Point> for Point {
    type Output = Vec2;

    #[inline]
    fn sub(self, other: Point) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "(")?;
        fmt::Display::fmt(&self.x, formatter)?;
        write!(formatter, ", ")?;
        fmt::Display::fmt(&self.y, formatter)?;
        write!(formatter, ")")
    }
}

#[cfg(feature = "mint")]
impl From<Point> for mint::Point2<f64> {
    #[inline(always)]
    fn from(p: Point) -> mint::Point2<f64> {
        mint::Point2 { x: p.x, y: p.y }
    }
}

#[cfg(feature = "mint")]
impl From<mint::Point2<f64>> for Point {
    #[inline(always)]
    fn from(p: mint::Point2<f64>) -> Point {
        Point { x: p.x, y: p.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_arithmetic() {
        assert_eq!(
            Point::new(0., 0.) - Vec2::new(10., 0.),
            Point::new(-10., 0.)
        );
        assert_eq!(
            Point::new(0., 0.) - Point::new(-5., 101.),
            Vec2::new(5., -101.)
        );
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn distance() {
        let p1 = Point::new(0., 10.);
        let p2 = Point::new(0., 5.);
        assert_eq!(p1.distance(p2), 5.);

        let p1 = Point::new(-11., 1.);
        let p2 = Point::new(-7., -2.);
        assert_eq!(p1.distance(p2), 5.);
    }

    #[test]
    fn display() {
        let p = Point::new(0.12345, 9.87654);
        assert_eq!(format!("{p}"), "(0.12345, 9.87654)");

        let p = Point::new(0.12345, 9.87654);
        assert_eq!(format!("{p:.2}"), "(0.12, 9.88)");
    }
}
