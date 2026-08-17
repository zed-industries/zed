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
use crate::num::*;
use crate::point::{point2, Point2D};
use crate::rect::Rect;
use crate::scale::Scale;
use crate::side_offsets::SideOffsets2D;
use crate::size::Size2D;
use crate::vector::{vec2, Vector2D};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
use num_traits::{Float, NumCast};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use core::borrow::Borrow;
use core::cmp::PartialOrd;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Add, Div, DivAssign, Mul, MulAssign, Range, Sub};

/// A 2d axis aligned rectangle represented by its minimum and maximum coordinates.
///
/// # Representation
///
/// This struct is similar to [`Rect`], but stores rectangle as two endpoints
/// instead of origin point and size. Such representation has several advantages over
/// [`Rect`] representation:
/// - Several operations are more efficient with `Box2D`, including [`intersection`],
///   [`union`], and point-in-rect.
/// - The representation is less susceptible to overflow. With [`Rect`], computation
///   of second point can overflow for a large range of values of origin and size.
///   However, with `Box2D`, computation of [`size`] cannot overflow if the coordinates
///   are signed and the resulting size is unsigned.
///
/// A known disadvantage of `Box2D` is that translating the rectangle requires translating
/// both points, whereas translating [`Rect`] only requires translating one point.
///
/// # Empty box
///
/// A box is considered empty (see [`is_empty`]) if any of the following is true:
/// - it's area is empty,
/// - it's area is negative (`min.x > max.x` or `min.y > max.y`),
/// - it contains NaNs.
///
/// [`intersection`]: Self::intersection
/// [`is_empty`]: Self::is_empty
/// [`union`]: Self::union
/// [`size`]: Self::size
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
)]
pub struct Box2D<T, U> {
    pub min: Point2D<T, U>,
    pub max: Point2D<T, U>,
}

impl<T: Hash, U> Hash for Box2D<T, U> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.min.hash(h);
        self.max.hash(h);
    }
}

impl<T: Copy, U> Copy for Box2D<T, U> {}

impl<T: Clone, U> Clone for Box2D<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.min.clone(), self.max.clone())
    }
}

impl<T: PartialEq, U> PartialEq for Box2D<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.min.eq(&other.min) && self.max.eq(&other.max)
    }
}

impl<T: Eq, U> Eq for Box2D<T, U> {}

impl<T: fmt::Debug, U> fmt::Debug for Box2D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Box2D")
            .field(&self.min)
            .field(&self.max)
            .finish()
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T, U> arbitrary::Arbitrary<'a> for Box2D<T, U>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Box2D::new(
            arbitrary::Arbitrary::arbitrary(u)?,
            arbitrary::Arbitrary::arbitrary(u)?,
        ))
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, U> Zeroable for Box2D<T, U> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, U: 'static> Pod for Box2D<T, U> {}

impl<T, U> Box2D<T, U> {
    /// Constructor.
    #[inline]
    pub const fn new(min: Point2D<T, U>, max: Point2D<T, U>) -> Self {
        Box2D { min, max }
    }

    /// Constructor.
    #[inline]
    pub fn from_origin_and_size(origin: Point2D<T, U>, size: Size2D<T, U>) -> Self
    where
        T: Copy + Add<T, Output = T>,
    {
        Box2D {
            min: origin,
            max: point2(origin.x + size.width, origin.y + size.height),
        }
    }

    /// Creates a `Box2D` of the given size, at offset zero.
    #[inline]
    pub fn from_size(size: Size2D<T, U>) -> Self
    where
        T: Zero,
    {
        Box2D {
            min: Point2D::zero(),
            max: point2(size.width, size.height),
        }
    }
}

impl<T, U> Box2D<T, U>
where
    T: PartialOrd,
{
    /// Returns `true` if the box has a negative area.
    ///
    /// The common interpretation for a negative box is to consider it empty. It can be obtained
    /// by calculating the intersection of two boxes that do not intersect.
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.max.x < self.min.x || self.max.y < self.min.y
    }

    /// Returns `true` if the size is zero, negative or NaN.
    #[inline]
    pub fn is_empty(&self) -> bool {
        !(self.max.x > self.min.x && self.max.y > self.min.y)
    }

    /// Returns `true` if the two boxes intersect.
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        // Use bitwise and instead of && to avoid emitting branches.
        (self.min.x < other.max.x)
            & (self.max.x > other.min.x)
            & (self.min.y < other.max.y)
            & (self.max.y > other.min.y)
    }

    /// Returns `true` if this box2d contains the point `p`. A point is considered
    /// in the box2d if it lies on the left or top edges, but outside if it lies
    /// on the right or bottom edges.
    #[inline]
    pub fn contains(&self, p: Point2D<T, U>) -> bool {
        // Use bitwise and instead of && to avoid emitting branches.
        (self.min.x <= p.x) & (p.x < self.max.x) & (self.min.y <= p.y) & (p.y < self.max.y)
    }

    /// Returns `true` if this box contains the point `p`. A point is considered
    /// in the box2d if it lies on any edge of the box2d.
    #[inline]
    pub fn contains_inclusive(&self, p: Point2D<T, U>) -> bool {
        // Use bitwise and instead of && to avoid emitting branches.
        (self.min.x <= p.x) & (p.x <= self.max.x) & (self.min.y <= p.y) & (p.y <= self.max.y)
    }

    /// Returns `true` if this box contains the interior of the other box. Always
    /// returns `true` if other is empty, and always returns `false` if other is
    /// nonempty but this box is empty.
    #[inline]
    pub fn contains_box(&self, other: &Self) -> bool {
        other.is_empty()
            || ((self.min.x <= other.min.x)
                & (other.max.x <= self.max.x)
                & (self.min.y <= other.min.y)
                & (other.max.y <= self.max.y))
    }
}

impl<T, U> Box2D<T, U>
where
    T: Copy + PartialOrd,
{
    #[inline]
    pub fn to_non_empty(&self) -> Option<Self> {
        if self.is_empty() {
            return None;
        }

        Some(*self)
    }

    /// Computes the intersection of two boxes, returning `None` if the boxes do not intersect.
    #[inline]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let b = self.intersection_unchecked(other);

        if b.is_empty() {
            return None;
        }

        Some(b)
    }

    /// Computes the intersection of two boxes without check whether they do intersect.
    ///
    /// The result is a negative box if the boxes do not intersect.
    /// This can be useful for computing the intersection of more than two boxes, as
    /// it is possible to chain multiple `intersection_unchecked` calls and check for
    /// empty/negative result at the end.
    #[inline]
    pub fn intersection_unchecked(&self, other: &Self) -> Self {
        Box2D {
            min: point2(max(self.min.x, other.min.x), max(self.min.y, other.min.y)),
            max: point2(min(self.max.x, other.max.x), min(self.max.y, other.max.y)),
        }
    }

    /// Computes the union of two boxes.
    ///
    /// If either of the boxes is empty, the other one is returned.
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        if other.is_empty() {
            return *self;
        }
        if self.is_empty() {
            return *other;
        }

        Box2D {
            min: point2(min(self.min.x, other.min.x), min(self.min.y, other.min.y)),
            max: point2(max(self.max.x, other.max.x), max(self.max.y, other.max.y)),
        }
    }
}

impl<T, U> Box2D<T, U>
where
    T: Copy + Add<T, Output = T>,
{
    /// Returns the same box, translated by a vector.
    #[inline]
    pub fn translate(&self, by: Vector2D<T, U>) -> Self {
        Box2D {
            min: self.min + by,
            max: self.max + by,
        }
    }
}

impl<T, U> Box2D<T, U>
where
    T: Copy + Sub<T, Output = T>,
{
    #[inline]
    pub fn size(&self) -> Size2D<T, U> {
        (self.max - self.min).to_size()
    }

    /// Change the size of the box by adjusting the max endpoint
    /// without modifying the min endpoint.
    #[inline]
    pub fn set_size(&mut self, size: Size2D<T, U>) {
        let diff = (self.size() - size).to_vector();
        self.max -= diff;
    }

    #[inline]
    pub fn width(&self) -> T {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> T {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn to_rect(&self) -> Rect<T, U> {
        Rect {
            origin: self.min,
            size: self.size(),
        }
    }
}

impl<T, U> Box2D<T, U>
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T>,
{
    /// Inflates the box by the specified sizes on each side respectively.
    #[inline]
    #[must_use]
    pub fn inflate(&self, width: T, height: T) -> Self {
        Box2D {
            min: point2(self.min.x - width, self.min.y - height),
            max: point2(self.max.x + width, self.max.y + height),
        }
    }

    /// Calculate the size and position of an inner box.
    ///
    /// Subtracts the side offsets from all sides. The horizontal, vertical
    /// and applicate offsets must not be larger than the original side length.
    pub fn inner_box(&self, offsets: SideOffsets2D<T, U>) -> Self {
        Box2D {
            min: self.min + vec2(offsets.left, offsets.top),
            max: self.max - vec2(offsets.right, offsets.bottom),
        }
    }

    /// Calculate the b and position of an outer box.
    ///
    /// Add the offsets to all sides. The expanded box is returned.
    pub fn outer_box(&self, offsets: SideOffsets2D<T, U>) -> Self {
        Box2D {
            min: self.min - vec2(offsets.left, offsets.top),
            max: self.max + vec2(offsets.right, offsets.bottom),
        }
    }
}

impl<T, U> Box2D<T, U>
where
    T: Copy + Zero + PartialOrd,
{
    /// Returns the smallest box containing all of the provided points.
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Point2D<T, U>>,
    {
        let mut points = points.into_iter();

        let (mut min_x, mut min_y) = match points.next() {
            Some(first) => first.borrow().to_tuple(),
            None => return Box2D::zero(),
        };

        let (mut max_x, mut max_y) = (min_x, min_y);
        for point in points {
            let p = point.borrow();
            if p.x < min_x {
                min_x = p.x
            }
            if p.x > max_x {
                max_x = p.x
            }
            if p.y < min_y {
                min_y = p.y
            }
            if p.y > max_y {
                max_y = p.y
            }
        }

        Box2D {
            min: point2(min_x, min_y),
            max: point2(max_x, max_y),
        }
    }
}

impl<T, U> Box2D<T, U>
where
    T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// Linearly interpolate between this box and another box.
    #[inline]
    pub fn lerp(&self, other: Self, t: T) -> Self {
        Self::new(self.min.lerp(other.min, t), self.max.lerp(other.max, t))
    }
}

impl<T, U> Box2D<T, U>
where
    T: Copy + One + Add<Output = T> + Div<Output = T>,
{
    pub fn center(&self) -> Point2D<T, U> {
        let two = T::one() + T::one();
        (self.min + self.max.to_vector()) / two
    }
}

impl<T, U> Box2D<T, U>
where
    T: Copy + Mul<T, Output = T> + Sub<T, Output = T>,
{
    #[inline]
    pub fn area(&self) -> T {
        let size = self.size();
        size.width * size.height
    }
}

impl<T, U> Box2D<T, U>
where
    T: Zero,
{
    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self {
        Box2D::new(Point2D::zero(), Point2D::zero())
    }
}

impl<T: Copy + Mul, U> Mul<T> for Box2D<T, U> {
    type Output = Box2D<T::Output, U>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Box2D::new(self.min * scale, self.max * scale)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Box2D<T, U> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        *self *= Scale::new(scale);
    }
}

impl<T: Copy + Div, U> Div<T> for Box2D<T, U> {
    type Output = Box2D<T::Output, U>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Box2D::new(self.min / scale, self.max / scale)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Box2D<T, U> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        *self /= Scale::new(scale);
    }
}

impl<T: Copy + Mul, U1, U2> Mul<Scale<T, U1, U2>> for Box2D<T, U1> {
    type Output = Box2D<T::Output, U2>;

    #[inline]
    fn mul(self, scale: Scale<T, U1, U2>) -> Self::Output {
        Box2D::new(self.min * scale, self.max * scale)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<Scale<T, U, U>> for Box2D<T, U> {
    #[inline]
    fn mul_assign(&mut self, scale: Scale<T, U, U>) {
        self.min *= scale;
        self.max *= scale;
    }
}

impl<T: Copy + Div, U1, U2> Div<Scale<T, U1, U2>> for Box2D<T, U2> {
    type Output = Box2D<T::Output, U1>;

    #[inline]
    fn div(self, scale: Scale<T, U1, U2>) -> Self::Output {
        Box2D::new(self.min / scale, self.max / scale)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<Scale<T, U, U>> for Box2D<T, U> {
    #[inline]
    fn div_assign(&mut self, scale: Scale<T, U, U>) {
        self.min /= scale;
        self.max /= scale;
    }
}

impl<T, U> Box2D<T, U>
where
    T: Copy,
{
    #[inline]
    pub fn x_range(&self) -> Range<T> {
        self.min.x..self.max.x
    }

    #[inline]
    pub fn y_range(&self) -> Range<T> {
        self.min.y..self.max.y
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(&self) -> Box2D<T, UnknownUnit> {
        Box2D::new(self.min.to_untyped(), self.max.to_untyped())
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(c: &Box2D<T, UnknownUnit>) -> Box2D<T, U> {
        Box2D::new(Point2D::from_untyped(c.min), Point2D::from_untyped(c.max))
    }

    /// Cast the unit
    #[inline]
    pub fn cast_unit<V>(&self) -> Box2D<T, V> {
        Box2D::new(self.min.cast_unit(), self.max.cast_unit())
    }

    #[inline]
    pub fn scale<S: Copy>(&self, x: S, y: S) -> Self
    where
        T: Mul<S, Output = T>,
    {
        Box2D {
            min: point2(self.min.x * x, self.min.y * y),
            max: point2(self.max.x * x, self.max.y * y),
        }
    }
}

impl<T: NumCast + Copy, U> Box2D<T, U> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using [`round`], [`round_in`] or [`round_out`] before casting.
    ///
    /// [`round`]: Self::round
    /// [`round_in`]: Self::round_in
    /// [`round_out`]: Self::round_out
    #[inline]
    pub fn cast<NewT: NumCast>(&self) -> Box2D<NewT, U> {
        Box2D::new(self.min.cast(), self.max.cast())
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating point to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using [`round`], [`round_in`] or [`round_out`] before casting.
    ///
    /// [`round`]: Self::round
    /// [`round_in`]: Self::round_in
    /// [`round_out`]: Self::round_out
    pub fn try_cast<NewT: NumCast>(&self) -> Option<Box2D<NewT, U>> {
        match (self.min.try_cast(), self.max.try_cast()) {
            (Some(a), Some(b)) => Some(Box2D::new(a, b)),
            _ => None,
        }
    }

    // Convenience functions for common casts

    /// Cast into an `f32` box.
    #[inline]
    pub fn to_f32(&self) -> Box2D<f32, U> {
        self.cast()
    }

    /// Cast into an `f64` box.
    #[inline]
    pub fn to_f64(&self) -> Box2D<f64, U> {
        self.cast()
    }

    /// Cast into an `usize` box, truncating decimals if any.
    ///
    /// When casting from floating point boxes, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    #[inline]
    pub fn to_usize(&self) -> Box2D<usize, U> {
        self.cast()
    }

    /// Cast into an `u32` box, truncating decimals if any.
    ///
    /// When casting from floating point boxes, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    #[inline]
    pub fn to_u32(&self) -> Box2D<u32, U> {
        self.cast()
    }

    /// Cast into an `i32` box, truncating decimals if any.
    ///
    /// When casting from floating point boxes, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    #[inline]
    pub fn to_i32(&self) -> Box2D<i32, U> {
        self.cast()
    }

    /// Cast into an `i64` box, truncating decimals if any.
    ///
    /// When casting from floating point boxes, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    #[inline]
    pub fn to_i64(&self) -> Box2D<i64, U> {
        self.cast()
    }
}

impl<T: Float, U> Box2D<T, U> {
    /// Returns `true` if all members are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.min.is_finite() && self.max.is_finite()
    }
}

impl<T, U> Box2D<T, U>
where
    T: Round,
{
    /// Return a box with edges rounded to integer coordinates, such that
    /// the returned box has the same set of pixel centers as the original
    /// one.
    /// Values equal to 0.5 round up.
    /// Suitable for most places where integral device coordinates
    /// are needed, but note that any translation should be applied first to
    /// avoid pixel rounding errors.
    /// Note that this is *not* rounding to nearest integer if the values are negative.
    /// They are always rounding as floor(n + 0.5).
    #[must_use]
    pub fn round(&self) -> Self {
        Box2D::new(self.min.round(), self.max.round())
    }
}

impl<T, U> Box2D<T, U>
where
    T: Floor + Ceil,
{
    /// Return a box with faces/edges rounded to integer coordinates, such that
    /// the original box contains the resulting box.
    #[must_use]
    pub fn round_in(&self) -> Self {
        let min = self.min.ceil();
        let max = self.max.floor();
        Box2D { min, max }
    }

    /// Return a box with faces/edges rounded to integer coordinates, such that
    /// the original box is contained in the resulting box.
    #[must_use]
    pub fn round_out(&self) -> Self {
        let min = self.min.floor();
        let max = self.max.ceil();
        Box2D { min, max }
    }
}

impl<T, U> From<Size2D<T, U>> for Box2D<T, U>
where
    T: Copy + Zero + PartialOrd,
{
    fn from(b: Size2D<T, U>) -> Self {
        Self::from_size(b)
    }
}

impl<T: Default, U> Default for Box2D<T, U> {
    fn default() -> Self {
        Box2D {
            min: Default::default(),
            max: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::default::Box2D;
    use crate::side_offsets::SideOffsets2D;
    use crate::{point2, size2, vec2, Point2D};
    //use super::*;

    #[test]
    fn test_size() {
        let b = Box2D::new(point2(-10.0, -10.0), point2(10.0, 10.0));
        assert_eq!(b.size().width, 20.0);
        assert_eq!(b.size().height, 20.0);
    }

    #[test]
    fn test_width_height() {
        let b = Box2D::new(point2(-10.0, -10.0), point2(10.0, 10.0));
        assert!(b.width() == 20.0);
        assert!(b.height() == 20.0);
    }

    #[test]
    fn test_center() {
        let b = Box2D::new(point2(-10.0, -10.0), point2(10.0, 10.0));
        assert_eq!(b.center(), Point2D::zero());
    }

    #[test]
    fn test_area() {
        let b = Box2D::new(point2(-10.0, -10.0), point2(10.0, 10.0));
        assert_eq!(b.area(), 400.0);
    }

    #[test]
    fn test_from_points() {
        let b = Box2D::from_points(&[point2(50.0, 160.0), point2(100.0, 25.0)]);
        assert_eq!(b.min, point2(50.0, 25.0));
        assert_eq!(b.max, point2(100.0, 160.0));
    }

    #[test]
    fn test_round_in() {
        let b = Box2D::from_points(&[point2(-25.5, -40.4), point2(60.3, 36.5)]).round_in();
        assert_eq!(b.min.x, -25.0);
        assert_eq!(b.min.y, -40.0);
        assert_eq!(b.max.x, 60.0);
        assert_eq!(b.max.y, 36.0);
    }

    #[test]
    fn test_round_out() {
        let b = Box2D::from_points(&[point2(-25.5, -40.4), point2(60.3, 36.5)]).round_out();
        assert_eq!(b.min.x, -26.0);
        assert_eq!(b.min.y, -41.0);
        assert_eq!(b.max.x, 61.0);
        assert_eq!(b.max.y, 37.0);
    }

    #[test]
    fn test_round() {
        let b = Box2D::from_points(&[point2(-25.5, -40.4), point2(60.3, 36.5)]).round();
        assert_eq!(b.min.x, -25.0);
        assert_eq!(b.min.y, -40.0);
        assert_eq!(b.max.x, 60.0);
        assert_eq!(b.max.y, 37.0);
    }

    #[test]
    fn test_from_size() {
        let b = Box2D::from_size(size2(30.0, 40.0));
        assert!(b.min == Point2D::zero());
        assert!(b.size().width == 30.0);
        assert!(b.size().height == 40.0);
    }

    #[test]
    fn test_inner_box() {
        let b = Box2D::from_points(&[point2(50.0, 25.0), point2(100.0, 160.0)]);
        let b = b.inner_box(SideOffsets2D::new(10.0, 20.0, 5.0, 10.0));
        assert_eq!(b.max.x, 80.0);
        assert_eq!(b.max.y, 155.0);
        assert_eq!(b.min.x, 60.0);
        assert_eq!(b.min.y, 35.0);
    }

    #[test]
    fn test_outer_box() {
        let b = Box2D::from_points(&[point2(50.0, 25.0), point2(100.0, 160.0)]);
        let b = b.outer_box(SideOffsets2D::new(10.0, 20.0, 5.0, 10.0));
        assert_eq!(b.max.x, 120.0);
        assert_eq!(b.max.y, 165.0);
        assert_eq!(b.min.x, 40.0);
        assert_eq!(b.min.y, 15.0);
    }

    #[test]
    fn test_translate() {
        let size = size2(15.0, 15.0);
        let mut center = (size / 2.0).to_vector().to_point();
        let b = Box2D::from_size(size);
        assert_eq!(b.center(), center);
        let translation = vec2(10.0, 2.5);
        let b = b.translate(translation);
        center += translation;
        assert_eq!(b.center(), center);
        assert_eq!(b.max.x, 25.0);
        assert_eq!(b.max.y, 17.5);
        assert_eq!(b.min.x, 10.0);
        assert_eq!(b.min.y, 2.5);
    }

    #[test]
    fn test_union() {
        let b1 = Box2D::from_points(&[point2(-20.0, -20.0), point2(0.0, 20.0)]);
        let b2 = Box2D::from_points(&[point2(0.0, 20.0), point2(20.0, -20.0)]);
        let b = b1.union(&b2);
        assert_eq!(b.max.x, 20.0);
        assert_eq!(b.max.y, 20.0);
        assert_eq!(b.min.x, -20.0);
        assert_eq!(b.min.y, -20.0);
    }

    #[test]
    fn test_intersects() {
        let b1 = Box2D::from_points(&[point2(-15.0, -20.0), point2(10.0, 20.0)]);
        let b2 = Box2D::from_points(&[point2(-10.0, 20.0), point2(15.0, -20.0)]);
        assert!(b1.intersects(&b2));
    }

    #[test]
    fn test_intersection_unchecked() {
        let b1 = Box2D::from_points(&[point2(-15.0, -20.0), point2(10.0, 20.0)]);
        let b2 = Box2D::from_points(&[point2(-10.0, 20.0), point2(15.0, -20.0)]);
        let b = b1.intersection_unchecked(&b2);
        assert_eq!(b.max.x, 10.0);
        assert_eq!(b.max.y, 20.0);
        assert_eq!(b.min.x, -10.0);
        assert_eq!(b.min.y, -20.0);
    }

    #[test]
    fn test_intersection() {
        let b1 = Box2D::from_points(&[point2(-15.0, -20.0), point2(10.0, 20.0)]);
        let b2 = Box2D::from_points(&[point2(-10.0, 20.0), point2(15.0, -20.0)]);
        assert!(b1.intersection(&b2).is_some());

        let b1 = Box2D::from_points(&[point2(-15.0, -20.0), point2(-10.0, 20.0)]);
        let b2 = Box2D::from_points(&[point2(10.0, 20.0), point2(15.0, -20.0)]);
        assert!(b1.intersection(&b2).is_none());
    }

    #[test]
    fn test_scale() {
        let b = Box2D::from_points(&[point2(-10.0, -10.0), point2(10.0, 10.0)]);
        let b = b.scale(0.5, 0.5);
        assert_eq!(b.max.x, 5.0);
        assert_eq!(b.max.y, 5.0);
        assert_eq!(b.min.x, -5.0);
        assert_eq!(b.min.y, -5.0);
    }

    #[test]
    fn test_lerp() {
        let b1 = Box2D::from_points(&[point2(-20.0, -20.0), point2(-10.0, -10.0)]);
        let b2 = Box2D::from_points(&[point2(10.0, 10.0), point2(20.0, 20.0)]);
        let b = b1.lerp(b2, 0.5);
        assert_eq!(b.center(), Point2D::zero());
        assert_eq!(b.size().width, 10.0);
        assert_eq!(b.size().height, 10.0);
    }

    #[test]
    fn test_contains() {
        let b = Box2D::from_points(&[point2(-20.0, -20.0), point2(20.0, 20.0)]);
        assert!(b.contains(point2(-15.3, 10.5)));
    }

    #[test]
    fn test_contains_box() {
        let b1 = Box2D::from_points(&[point2(-20.0, -20.0), point2(20.0, 20.0)]);
        let b2 = Box2D::from_points(&[point2(-14.3, -16.5), point2(6.7, 17.6)]);
        assert!(b1.contains_box(&b2));
    }

    #[test]
    fn test_inflate() {
        let b = Box2D::from_points(&[point2(-20.0, -20.0), point2(20.0, 20.0)]);
        let b = b.inflate(10.0, 5.0);
        assert_eq!(b.size().width, 60.0);
        assert_eq!(b.size().height, 50.0);
        assert_eq!(b.center(), Point2D::zero());
    }

    #[test]
    fn test_is_empty() {
        for i in 0..2 {
            let mut coords_neg = [-20.0, -20.0];
            let mut coords_pos = [20.0, 20.0];
            coords_neg[i] = 0.0;
            coords_pos[i] = 0.0;
            let b = Box2D::from_points(&[Point2D::from(coords_neg), Point2D::from(coords_pos)]);
            assert!(b.is_empty());
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_nan_empty() {
        use std::f32::NAN;
        assert!(Box2D { min: point2(NAN, 2.0), max: point2(1.0, 3.0) }.is_empty());
        assert!(Box2D { min: point2(0.0, NAN), max: point2(1.0, 2.0) }.is_empty());
        assert!(Box2D { min: point2(1.0, -2.0), max: point2(NAN, 2.0) }.is_empty());
        assert!(Box2D { min: point2(1.0, -2.0), max: point2(0.0, NAN) }.is_empty());
    }

    #[test]
    fn test_from_origin_and_size() {
        let b = Box2D::from_origin_and_size(point2(1.0, 2.0), size2(3.0, 4.0));
        assert_eq!(b.min, point2(1.0, 2.0));
        assert_eq!(b.size(), size2(3.0, 4.0));
    }

    #[test]
    fn test_set_size() {
        let mut b = Box2D {
            min: point2(1.0, 2.0),
            max: point2(3.0, 4.0),
        };
        b.set_size(size2(5.0, 6.0));

        assert_eq!(b.min, point2(1.0, 2.0));
        assert_eq!(b.size(), size2(5.0, 6.0));
    }
}
