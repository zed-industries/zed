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
use crate::point::{point3, Point3D};
use crate::scale::Scale;
use crate::size::Size3D;
use crate::vector::Vector3D;

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

/// An axis aligned 3D box represented by its minimum and maximum coordinates.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
)]
pub struct Box3D<T, U> {
    pub min: Point3D<T, U>,
    pub max: Point3D<T, U>,
}

impl<T: Hash, U> Hash for Box3D<T, U> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.min.hash(h);
        self.max.hash(h);
    }
}

impl<T: Copy, U> Copy for Box3D<T, U> {}

impl<T: Clone, U> Clone for Box3D<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.min.clone(), self.max.clone())
    }
}

impl<T: PartialEq, U> PartialEq for Box3D<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.min.eq(&other.min) && self.max.eq(&other.max)
    }
}

impl<T: Eq, U> Eq for Box3D<T, U> {}

impl<T: fmt::Debug, U> fmt::Debug for Box3D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Box3D")
            .field(&self.min)
            .field(&self.max)
            .finish()
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T, U> arbitrary::Arbitrary<'a> for Box3D<T, U>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Box3D::new(
            arbitrary::Arbitrary::arbitrary(u)?,
            arbitrary::Arbitrary::arbitrary(u)?,
        ))
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, U> Zeroable for Box3D<T, U> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, U: 'static> Pod for Box3D<T, U> {}

impl<T, U> Box3D<T, U> {
    /// Constructor.
    #[inline]
    pub const fn new(min: Point3D<T, U>, max: Point3D<T, U>) -> Self {
        Box3D { min, max }
    }

    /// Constructor.
    #[inline]
    pub fn from_origin_and_size(origin: Point3D<T, U>, size: Size3D<T, U>) -> Self
    where
        T: Copy + Add<T, Output = T>,
    {
        Box3D {
            min: origin,
            max: point3(
                origin.x + size.width,
                origin.y + size.height,
                origin.z + size.depth,
            ),
        }
    }

    /// Creates a `Box3D` of the given size, at offset zero.
    #[inline]
    pub fn from_size(size: Size3D<T, U>) -> Self
    where
        T: Zero,
    {
        Box3D {
            min: Point3D::zero(),
            max: point3(size.width, size.height, size.depth),
        }
    }
}

impl<T, U> Box3D<T, U>
where
    T: PartialOrd,
{
    /// Returns `true` if the box has a negative volume.
    ///
    /// The common interpretation for a negative box is to consider it empty. It can be obtained
    /// by calculating the intersection of two boxes that do not intersect.
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.max.x < self.min.x || self.max.y < self.min.y || self.max.z < self.min.z
    }

    /// Returns `true` if the size is zero, negative or NaN.
    #[inline]
    pub fn is_empty(&self) -> bool {
        !(self.max.x > self.min.x && self.max.y > self.min.y && self.max.z > self.min.z)
    }

    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    /// Returns `true` if this box3d contains the point `p`. A point is considered
    /// in the box3d if it lies on the front, left or top faces, but outside if it lies
    /// on the back, right or bottom faces.
    #[inline]
    pub fn contains(&self, other: Point3D<T, U>) -> bool {
        (self.min.x <= other.x)
            & (other.x < self.max.x)
            & (self.min.y <= other.y)
            & (other.y < self.max.y)
            & (self.min.z <= other.z)
            & (other.z < self.max.z)
    }

    /// Returns `true` if this box3d contains the point `p`. A point is considered
    /// in the box3d if it lies on any face of the box3d.
    #[inline]
    pub fn contains_inclusive(&self, other: Point3D<T, U>) -> bool {
        (self.min.x <= other.x)
            & (other.x <= self.max.x)
            & (self.min.y <= other.y)
            & (other.y <= self.max.y)
            & (self.min.z <= other.z)
            & (other.z <= self.max.z)
    }

    /// Returns `true` if this box3d contains the interior of the other box3d. Always
    /// returns `true` if other is empty, and always returns `false` if other is
    /// nonempty but this box3d is empty.
    #[inline]
    pub fn contains_box(&self, other: &Self) -> bool {
        other.is_empty()
            || ((self.min.x <= other.min.x)
                & (other.max.x <= self.max.x)
                & (self.min.y <= other.min.y)
                & (other.max.y <= self.max.y)
                & (self.min.z <= other.min.z)
                & (other.max.z <= self.max.z))
    }
}

impl<T, U> Box3D<T, U>
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

    #[inline]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let b = self.intersection_unchecked(other);

        if b.is_empty() {
            return None;
        }

        Some(b)
    }

    pub fn intersection_unchecked(&self, other: &Self) -> Self {
        let intersection_min = Point3D::new(
            max(self.min.x, other.min.x),
            max(self.min.y, other.min.y),
            max(self.min.z, other.min.z),
        );

        let intersection_max = Point3D::new(
            min(self.max.x, other.max.x),
            min(self.max.y, other.max.y),
            min(self.max.z, other.max.z),
        );

        Box3D::new(intersection_min, intersection_max)
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

        Box3D::new(
            Point3D::new(
                min(self.min.x, other.min.x),
                min(self.min.y, other.min.y),
                min(self.min.z, other.min.z),
            ),
            Point3D::new(
                max(self.max.x, other.max.x),
                max(self.max.y, other.max.y),
                max(self.max.z, other.max.z),
            ),
        )
    }
}

impl<T, U> Box3D<T, U>
where
    T: Copy + Add<T, Output = T>,
{
    /// Returns the same box3d, translated by a vector.
    #[inline]
    #[must_use]
    pub fn translate(&self, by: Vector3D<T, U>) -> Self {
        Box3D {
            min: self.min + by,
            max: self.max + by,
        }
    }
}

impl<T, U> Box3D<T, U>
where
    T: Copy + Sub<T, Output = T>,
{
    #[inline]
    pub fn size(&self) -> Size3D<T, U> {
        Size3D::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
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
    pub fn depth(&self) -> T {
        self.max.z - self.min.z
    }
}

impl<T, U> Box3D<T, U>
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T>,
{
    /// Inflates the box by the specified sizes on each side respectively.
    #[inline]
    #[must_use]
    pub fn inflate(&self, width: T, height: T, depth: T) -> Self {
        Box3D::new(
            Point3D::new(self.min.x - width, self.min.y - height, self.min.z - depth),
            Point3D::new(self.max.x + width, self.max.y + height, self.max.z + depth),
        )
    }
}

impl<T, U> Box3D<T, U>
where
    T: Copy + Zero + PartialOrd,
{
    /// Returns the smallest box containing all of the provided points.
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Point3D<T, U>>,
    {
        let mut points = points.into_iter();

        let (mut min_x, mut min_y, mut min_z) = match points.next() {
            Some(first) => first.borrow().to_tuple(),
            None => return Box3D::zero(),
        };
        let (mut max_x, mut max_y, mut max_z) = (min_x, min_y, min_z);

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
            if p.z < min_z {
                min_z = p.z
            }
            if p.z > max_z {
                max_z = p.z
            }
        }

        Box3D {
            min: point3(min_x, min_y, min_z),
            max: point3(max_x, max_y, max_z),
        }
    }
}

impl<T, U> Box3D<T, U>
where
    T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// Linearly interpolate between this box3d and another box3d.
    #[inline]
    pub fn lerp(&self, other: Self, t: T) -> Self {
        Self::new(self.min.lerp(other.min, t), self.max.lerp(other.max, t))
    }
}

impl<T, U> Box3D<T, U>
where
    T: Copy + One + Add<Output = T> + Div<Output = T>,
{
    pub fn center(&self) -> Point3D<T, U> {
        let two = T::one() + T::one();
        (self.min + self.max.to_vector()) / two
    }
}

impl<T, U> Box3D<T, U>
where
    T: Copy + Mul<T, Output = T> + Sub<T, Output = T>,
{
    #[inline]
    pub fn volume(&self) -> T {
        let size = self.size();
        size.width * size.height * size.depth
    }

    #[inline]
    pub fn xy_area(&self) -> T {
        let size = self.size();
        size.width * size.height
    }

    #[inline]
    pub fn yz_area(&self) -> T {
        let size = self.size();
        size.depth * size.height
    }

    #[inline]
    pub fn xz_area(&self) -> T {
        let size = self.size();
        size.depth * size.width
    }
}

impl<T, U> Box3D<T, U>
where
    T: Zero,
{
    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self {
        Box3D::new(Point3D::zero(), Point3D::zero())
    }
}

impl<T: Copy + Mul, U> Mul<T> for Box3D<T, U> {
    type Output = Box3D<T::Output, U>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Box3D::new(self.min * scale, self.max * scale)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Box3D<T, U> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        self.min *= scale;
        self.max *= scale;
    }
}

impl<T: Copy + Div, U> Div<T> for Box3D<T, U> {
    type Output = Box3D<T::Output, U>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Box3D::new(self.min / scale.clone(), self.max / scale)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Box3D<T, U> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        self.min /= scale;
        self.max /= scale;
    }
}

impl<T: Copy + Mul, U1, U2> Mul<Scale<T, U1, U2>> for Box3D<T, U1> {
    type Output = Box3D<T::Output, U2>;

    #[inline]
    fn mul(self, scale: Scale<T, U1, U2>) -> Self::Output {
        Box3D::new(self.min * scale.clone(), self.max * scale)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<Scale<T, U, U>> for Box3D<T, U> {
    #[inline]
    fn mul_assign(&mut self, scale: Scale<T, U, U>) {
        self.min *= scale.clone();
        self.max *= scale;
    }
}

impl<T: Copy + Div, U1, U2> Div<Scale<T, U1, U2>> for Box3D<T, U2> {
    type Output = Box3D<T::Output, U1>;

    #[inline]
    fn div(self, scale: Scale<T, U1, U2>) -> Self::Output {
        Box3D::new(self.min / scale.clone(), self.max / scale)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<Scale<T, U, U>> for Box3D<T, U> {
    #[inline]
    fn div_assign(&mut self, scale: Scale<T, U, U>) {
        self.min /= scale.clone();
        self.max /= scale;
    }
}

impl<T, U> Box3D<T, U>
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

    #[inline]
    pub fn z_range(&self) -> Range<T> {
        self.min.z..self.max.z
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(&self) -> Box3D<T, UnknownUnit> {
        Box3D {
            min: self.min.to_untyped(),
            max: self.max.to_untyped(),
        }
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(c: &Box3D<T, UnknownUnit>) -> Box3D<T, U> {
        Box3D {
            min: Point3D::from_untyped(c.min),
            max: Point3D::from_untyped(c.max),
        }
    }

    /// Cast the unit
    #[inline]
    pub fn cast_unit<V>(&self) -> Box3D<T, V> {
        Box3D::new(self.min.cast_unit(), self.max.cast_unit())
    }

    #[inline]
    pub fn scale<S: Copy>(&self, x: S, y: S, z: S) -> Self
    where
        T: Mul<S, Output = T>,
    {
        Box3D::new(
            Point3D::new(self.min.x * x, self.min.y * y, self.min.z * z),
            Point3D::new(self.max.x * x, self.max.y * y, self.max.z * z),
        )
    }
}

impl<T: NumCast + Copy, U> Box3D<T, U> {
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
    pub fn cast<NewT: NumCast>(&self) -> Box3D<NewT, U> {
        Box3D::new(self.min.cast(), self.max.cast())
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
    pub fn try_cast<NewT: NumCast>(&self) -> Option<Box3D<NewT, U>> {
        match (self.min.try_cast(), self.max.try_cast()) {
            (Some(a), Some(b)) => Some(Box3D::new(a, b)),
            _ => None,
        }
    }

    // Convenience functions for common casts

    /// Cast into an `f32` box3d.
    #[inline]
    pub fn to_f32(&self) -> Box3D<f32, U> {
        self.cast()
    }

    /// Cast into an `f64` box3d.
    #[inline]
    pub fn to_f64(&self) -> Box3D<f64, U> {
        self.cast()
    }

    /// Cast into an `usize` box3d, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    #[inline]
    pub fn to_usize(&self) -> Box3D<usize, U> {
        self.cast()
    }

    /// Cast into an `u32` box3d, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    #[inline]
    pub fn to_u32(&self) -> Box3D<u32, U> {
        self.cast()
    }

    /// Cast into an `i32` box3d, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    #[inline]
    pub fn to_i32(&self) -> Box3D<i32, U> {
        self.cast()
    }

    /// Cast into an `i64` box3d, truncating decimals if any.
    ///
    /// When casting from floating point cuboids, it is worth considering whether
    /// to `round()`, `round_in()` or `round_out()` before the cast in order to
    /// obtain the desired conversion behavior.
    #[inline]
    pub fn to_i64(&self) -> Box3D<i64, U> {
        self.cast()
    }
}

impl<T: Float, U> Box3D<T, U> {
    /// Returns `true` if all members are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.min.is_finite() && self.max.is_finite()
    }
}

impl<T, U> Box3D<T, U>
where
    T: Round,
{
    /// Return a box3d with edges rounded to integer coordinates, such that
    /// the returned box3d has the same set of pixel centers as the original
    /// one.
    /// Values equal to 0.5 round up.
    /// Suitable for most places where integral device coordinates
    /// are needed, but note that any translation should be applied first to
    /// avoid pixel rounding errors.
    /// Note that this is *not* rounding to nearest integer if the values are negative.
    /// They are always rounding as floor(n + 0.5).
    #[must_use]
    pub fn round(&self) -> Self {
        Box3D::new(self.min.round(), self.max.round())
    }
}

impl<T, U> Box3D<T, U>
where
    T: Floor + Ceil,
{
    /// Return a box3d with faces/edges rounded to integer coordinates, such that
    /// the original box3d contains the resulting box3d.
    #[must_use]
    pub fn round_in(&self) -> Self {
        Box3D {
            min: self.min.ceil(),
            max: self.max.floor(),
        }
    }

    /// Return a box3d with faces/edges rounded to integer coordinates, such that
    /// the original box3d is contained in the resulting box3d.
    #[must_use]
    pub fn round_out(&self) -> Self {
        Box3D {
            min: self.min.floor(),
            max: self.max.ceil(),
        }
    }
}

impl<T, U> From<Size3D<T, U>> for Box3D<T, U>
where
    T: Copy + Zero + PartialOrd,
{
    fn from(b: Size3D<T, U>) -> Self {
        Self::from_size(b)
    }
}

impl<T: Default, U> Default for Box3D<T, U> {
    fn default() -> Self {
        Box3D {
            min: Default::default(),
            max: Default::default(),
        }
    }
}

/// Shorthand for `Box3D::new(Point3D::new(x1, y1, z1), Point3D::new(x2, y2, z2))`.
pub fn box3d<T: Copy, U>(
    min_x: T,
    min_y: T,
    min_z: T,
    max_x: T,
    max_y: T,
    max_z: T,
) -> Box3D<T, U> {
    Box3D::new(
        Point3D::new(min_x, min_y, min_z),
        Point3D::new(max_x, max_y, max_z),
    )
}

#[cfg(test)]
mod tests {
    use crate::default::{Box3D, Point3D};
    use crate::{point3, size3, vec3};

    #[test]
    fn test_new() {
        let b = Box3D::new(point3(-1.0, -1.0, -1.0), point3(1.0, 1.0, 1.0));
        assert!(b.min.x == -1.0);
        assert!(b.min.y == -1.0);
        assert!(b.min.z == -1.0);
        assert!(b.max.x == 1.0);
        assert!(b.max.y == 1.0);
        assert!(b.max.z == 1.0);
    }

    #[test]
    fn test_size() {
        let b = Box3D::new(point3(-10.0, -10.0, -10.0), point3(10.0, 10.0, 10.0));
        assert!(b.size().width == 20.0);
        assert!(b.size().height == 20.0);
        assert!(b.size().depth == 20.0);
    }

    #[test]
    fn test_width_height_depth() {
        let b = Box3D::new(point3(-10.0, -10.0, -10.0), point3(10.0, 10.0, 10.0));
        assert!(b.width() == 20.0);
        assert!(b.height() == 20.0);
        assert!(b.depth() == 20.0);
    }

    #[test]
    fn test_center() {
        let b = Box3D::new(point3(-10.0, -10.0, -10.0), point3(10.0, 10.0, 10.0));
        assert!(b.center() == Point3D::zero());
    }

    #[test]
    fn test_volume() {
        let b = Box3D::new(point3(-10.0, -10.0, -10.0), point3(10.0, 10.0, 10.0));
        assert!(b.volume() == 8000.0);
    }

    #[test]
    fn test_area() {
        let b = Box3D::new(point3(-10.0, -10.0, -10.0), point3(10.0, 10.0, 10.0));
        assert!(b.xy_area() == 400.0);
        assert!(b.yz_area() == 400.0);
        assert!(b.xz_area() == 400.0);
    }

    #[test]
    fn test_from_points() {
        let b = Box3D::from_points(&[point3(50.0, 160.0, 12.5), point3(100.0, 25.0, 200.0)]);
        assert!(b.min == point3(50.0, 25.0, 12.5));
        assert!(b.max == point3(100.0, 160.0, 200.0));
    }

    #[test]
    fn test_min_max() {
        let b = Box3D::from_points(&[point3(50.0, 25.0, 12.5), point3(100.0, 160.0, 200.0)]);
        assert!(b.min.x == 50.0);
        assert!(b.min.y == 25.0);
        assert!(b.min.z == 12.5);
        assert!(b.max.x == 100.0);
        assert!(b.max.y == 160.0);
        assert!(b.max.z == 200.0);
    }

    #[test]
    fn test_round_in() {
        let b =
            Box3D::from_points(&[point3(-25.5, -40.4, -70.9), point3(60.3, 36.5, 89.8)]).round_in();
        assert!(b.min.x == -25.0);
        assert!(b.min.y == -40.0);
        assert!(b.min.z == -70.0);
        assert!(b.max.x == 60.0);
        assert!(b.max.y == 36.0);
        assert!(b.max.z == 89.0);
    }

    #[test]
    fn test_round_out() {
        let b = Box3D::from_points(&[point3(-25.5, -40.4, -70.9), point3(60.3, 36.5, 89.8)])
            .round_out();
        assert!(b.min.x == -26.0);
        assert!(b.min.y == -41.0);
        assert!(b.min.z == -71.0);
        assert!(b.max.x == 61.0);
        assert!(b.max.y == 37.0);
        assert!(b.max.z == 90.0);
    }

    #[test]
    fn test_round() {
        let b =
            Box3D::from_points(&[point3(-25.5, -40.4, -70.9), point3(60.3, 36.5, 89.8)]).round();
        assert!(b.min.x == -25.0);
        assert!(b.min.y == -40.0);
        assert!(b.min.z == -71.0);
        assert!(b.max.x == 60.0);
        assert!(b.max.y == 37.0);
        assert!(b.max.z == 90.0);
    }

    #[test]
    fn test_from_size() {
        let b = Box3D::from_size(size3(30.0, 40.0, 50.0));
        assert!(b.min == Point3D::zero());
        assert!(b.size().width == 30.0);
        assert!(b.size().height == 40.0);
        assert!(b.size().depth == 50.0);
    }

    #[test]
    fn test_translate() {
        let size = size3(15.0, 15.0, 200.0);
        let mut center = (size / 2.0).to_vector().to_point();
        let b = Box3D::from_size(size);
        assert!(b.center() == center);
        let translation = vec3(10.0, 2.5, 9.5);
        let b = b.translate(translation);
        center += translation;
        assert!(b.center() == center);
        assert!(b.max.x == 25.0);
        assert!(b.max.y == 17.5);
        assert!(b.max.z == 209.5);
        assert!(b.min.x == 10.0);
        assert!(b.min.y == 2.5);
        assert!(b.min.z == 9.5);
    }

    #[test]
    fn test_union() {
        let b1 = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(0.0, 20.0, 20.0)]);
        let b2 = Box3D::from_points(&[point3(0.0, 20.0, 20.0), point3(20.0, -20.0, -20.0)]);
        let b = b1.union(&b2);
        assert!(b.max.x == 20.0);
        assert!(b.max.y == 20.0);
        assert!(b.max.z == 20.0);
        assert!(b.min.x == -20.0);
        assert!(b.min.y == -20.0);
        assert!(b.min.z == -20.0);
        assert!(b.volume() == (40.0 * 40.0 * 40.0));
    }

    #[test]
    fn test_intersects() {
        let b1 = Box3D::from_points(&[point3(-15.0, -20.0, -20.0), point3(10.0, 20.0, 20.0)]);
        let b2 = Box3D::from_points(&[point3(-10.0, 20.0, 20.0), point3(15.0, -20.0, -20.0)]);
        assert!(b1.intersects(&b2));
    }

    #[test]
    fn test_intersection_unchecked() {
        let b1 = Box3D::from_points(&[point3(-15.0, -20.0, -20.0), point3(10.0, 20.0, 20.0)]);
        let b2 = Box3D::from_points(&[point3(-10.0, 20.0, 20.0), point3(15.0, -20.0, -20.0)]);
        let b = b1.intersection_unchecked(&b2);
        assert!(b.max.x == 10.0);
        assert!(b.max.y == 20.0);
        assert!(b.max.z == 20.0);
        assert!(b.min.x == -10.0);
        assert!(b.min.y == -20.0);
        assert!(b.min.z == -20.0);
        assert!(b.volume() == (20.0 * 40.0 * 40.0));
    }

    #[test]
    fn test_intersection() {
        let b1 = Box3D::from_points(&[point3(-15.0, -20.0, -20.0), point3(10.0, 20.0, 20.0)]);
        let b2 = Box3D::from_points(&[point3(-10.0, 20.0, 20.0), point3(15.0, -20.0, -20.0)]);
        assert!(b1.intersection(&b2).is_some());

        let b1 = Box3D::from_points(&[point3(-15.0, -20.0, -20.0), point3(-10.0, 20.0, 20.0)]);
        let b2 = Box3D::from_points(&[point3(10.0, 20.0, 20.0), point3(15.0, -20.0, -20.0)]);
        assert!(b1.intersection(&b2).is_none());
    }

    #[test]
    fn test_scale() {
        let b = Box3D::from_points(&[point3(-10.0, -10.0, -10.0), point3(10.0, 10.0, 10.0)]);
        let b = b.scale(0.5, 0.5, 0.5);
        assert!(b.max.x == 5.0);
        assert!(b.max.y == 5.0);
        assert!(b.max.z == 5.0);
        assert!(b.min.x == -5.0);
        assert!(b.min.y == -5.0);
        assert!(b.min.z == -5.0);
    }

    #[test]
    fn test_zero() {
        let b = Box3D::<f64>::zero();
        assert!(b.max.x == 0.0);
        assert!(b.max.y == 0.0);
        assert!(b.max.z == 0.0);
        assert!(b.min.x == 0.0);
        assert!(b.min.y == 0.0);
        assert!(b.min.z == 0.0);
    }

    #[test]
    fn test_lerp() {
        let b1 = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(-10.0, -10.0, -10.0)]);
        let b2 = Box3D::from_points(&[point3(10.0, 10.0, 10.0), point3(20.0, 20.0, 20.0)]);
        let b = b1.lerp(b2, 0.5);
        assert!(b.center() == Point3D::zero());
        assert!(b.size().width == 10.0);
        assert!(b.size().height == 10.0);
        assert!(b.size().depth == 10.0);
    }

    #[test]
    fn test_contains() {
        let b = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(20.0, 20.0, 20.0)]);
        assert!(b.contains(point3(-15.3, 10.5, 18.4)));
    }

    #[test]
    fn test_contains_box() {
        let b1 = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(20.0, 20.0, 20.0)]);
        let b2 = Box3D::from_points(&[point3(-14.3, -16.5, -19.3), point3(6.7, 17.6, 2.5)]);
        assert!(b1.contains_box(&b2));
    }

    #[test]
    fn test_inflate() {
        let b = Box3D::from_points(&[point3(-20.0, -20.0, -20.0), point3(20.0, 20.0, 20.0)]);
        let b = b.inflate(10.0, 5.0, 2.0);
        assert!(b.size().width == 60.0);
        assert!(b.size().height == 50.0);
        assert!(b.size().depth == 44.0);
        assert!(b.center() == Point3D::zero());
    }

    #[test]
    fn test_is_empty() {
        for i in 0..3 {
            let mut coords_neg = [-20.0, -20.0, -20.0];
            let mut coords_pos = [20.0, 20.0, 20.0];
            coords_neg[i] = 0.0;
            coords_pos[i] = 0.0;
            let b = Box3D::from_points(&[Point3D::from(coords_neg), Point3D::from(coords_pos)]);
            assert!(b.is_empty());
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_nan_empty_or_negative() {
        use std::f32::NAN;
        assert!(Box3D { min: point3(NAN, 2.0, 1.0), max: point3(1.0, 3.0, 5.0) }.is_empty());
        assert!(Box3D { min: point3(0.0, NAN, 1.0), max: point3(1.0, 2.0, 5.0) }.is_empty());
        assert!(Box3D { min: point3(1.0, -2.0, NAN), max: point3(3.0, 2.0, 5.0) }.is_empty());
        assert!(Box3D { min: point3(1.0, -2.0, 1.0), max: point3(NAN, 2.0, 5.0) }.is_empty());
        assert!(Box3D { min: point3(1.0, -2.0, 1.0), max: point3(0.0, NAN, 5.0) }.is_empty());
        assert!(Box3D { min: point3(1.0, -2.0, 1.0), max: point3(0.0, 1.0, NAN) }.is_empty());
    }
}
