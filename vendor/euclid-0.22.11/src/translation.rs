// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::num::*;
use crate::UnknownUnit;
use crate::{point2, point3, vec2, vec3, Box2D, Box3D, Rect, Size2D};
use crate::{Point2D, Point3D, Transform2D, Transform3D, Vector2D, Vector3D};

use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
use num_traits::NumCast;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A 2d transformation from a space to another that can only express translations.
///
/// The main benefit of this type over a [`Vector2D`] is the ability to cast
/// between source and destination spaces.
///
/// Example:
///
/// ```
/// use euclid::{Translation2D, Point2D, point2};
/// struct ParentSpace;
/// struct ChildSpace;
/// type ScrollOffset = Translation2D<i32, ParentSpace, ChildSpace>;
/// type ParentPoint = Point2D<i32, ParentSpace>;
/// type ChildPoint = Point2D<i32, ChildSpace>;
///
/// let scrolling = ScrollOffset::new(0, 100);
/// let p1: ParentPoint = point2(0, 0);
/// let p2: ChildPoint = scrolling.transform_point(p1);
/// ```
///
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "T: serde::Serialize",
        deserialize = "T: serde::Deserialize<'de>"
    ))
)]
pub struct Translation2D<T, Src, Dst> {
    pub x: T,
    pub y: T,
    #[doc(hidden)]
    pub _unit: PhantomData<(Src, Dst)>,
}

#[cfg(feature = "arbitrary")]
impl<'a, T, Src, Dst> arbitrary::Arbitrary<'a> for Translation2D<T, Src, Dst>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (x, y) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(Translation2D {
            x,
            y,
            _unit: PhantomData,
        })
    }
}

impl<T: Copy, Src, Dst> Copy for Translation2D<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for Translation2D<T, Src, Dst> {
    fn clone(&self) -> Self {
        Translation2D {
            x: self.x.clone(),
            y: self.y.clone(),
            _unit: PhantomData,
        }
    }
}

impl<T, Src, Dst> Eq for Translation2D<T, Src, Dst> where T: Eq {}

impl<T, Src, Dst> PartialEq for Translation2D<T, Src, Dst>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T, Src, Dst> Hash for Translation2D<T, Src, Dst>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
    }
}

impl<T, Src, Dst> Translation2D<T, Src, Dst> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Translation2D {
            x,
            y,
            _unit: PhantomData,
        }
    }

    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Translation2D {
            x: v.clone(),
            y: v,
            _unit: PhantomData,
        }
    }

    /// Creates no-op translation (both `x` and `y` is `zero()`).
    #[inline]
    pub fn identity() -> Self
    where
        T: Zero,
    {
        Self::new(T::zero(), T::zero())
    }

    /// Check if translation does nothing (both x and y is `zero()`).
    ///
    /// ```rust
    /// use euclid::default::Translation2D;
    ///
    /// assert_eq!(Translation2D::<f32>::identity().is_identity(), true);
    /// assert_eq!(Translation2D::new(0, 0).is_identity(), true);
    /// assert_eq!(Translation2D::new(1, 0).is_identity(), false);
    /// assert_eq!(Translation2D::new(0, 1).is_identity(), false);
    /// ```
    #[inline]
    pub fn is_identity(&self) -> bool
    where
        T: Zero + PartialEq,
    {
        let _0 = T::zero();
        self.x == _0 && self.y == _0
    }

    /// No-op, just cast the unit.
    #[inline]
    pub fn transform_size(&self, s: Size2D<T, Src>) -> Size2D<T, Dst> {
        Size2D::new(s.width, s.height)
    }
}

impl<T: Copy, Src, Dst> Translation2D<T, Src, Dst> {
    /// Cast into a 2D vector.
    #[inline]
    pub fn to_vector(&self) -> Vector2D<T, Src> {
        vec2(self.x, self.y)
    }

    /// Cast into an array with x and y.
    #[inline]
    pub fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    /// Cast into a tuple with x and y.
    #[inline]
    pub fn to_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(&self) -> Translation2D<T, UnknownUnit, UnknownUnit> {
        Translation2D {
            x: self.x,
            y: self.y,
            _unit: PhantomData,
        }
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(t: &Translation2D<T, UnknownUnit, UnknownUnit>) -> Self {
        Translation2D {
            x: t.x,
            y: t.y,
            _unit: PhantomData,
        }
    }

    /// Returns the matrix representation of this translation.
    #[inline]
    pub fn to_transform(&self) -> Transform2D<T, Src, Dst>
    where
        T: Zero + One,
    {
        (*self).into()
    }

    /// Translate a point and cast its unit.
    #[inline]
    pub fn transform_point(&self, p: Point2D<T, Src>) -> Point2D<T::Output, Dst>
    where
        T: Add,
    {
        point2(p.x + self.x, p.y + self.y)
    }

    /// Translate a rectangle and cast its unit.
    #[inline]
    pub fn transform_rect(&self, r: &Rect<T, Src>) -> Rect<T::Output, Dst>
    where
        T: Add<Output = T>,
    {
        Rect {
            origin: self.transform_point(r.origin),
            size: self.transform_size(r.size),
        }
    }

    /// Translate a 2D box and cast its unit.
    #[inline]
    pub fn transform_box(&self, r: &Box2D<T, Src>) -> Box2D<T::Output, Dst>
    where
        T: Add,
    {
        Box2D {
            min: self.transform_point(r.min),
            max: self.transform_point(r.max),
        }
    }

    /// Return the inverse transformation.
    #[inline]
    pub fn inverse(&self) -> Translation2D<T::Output, Dst, Src>
    where
        T: Neg,
    {
        Translation2D::new(-self.x, -self.y)
    }
}

impl<T: NumCast + Copy, Src, Dst> Translation2D<T, Src, Dst> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn cast<NewT: NumCast>(self) -> Translation2D<NewT, Src, Dst> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    pub fn try_cast<NewT: NumCast>(self) -> Option<Translation2D<NewT, Src, Dst>> {
        match (NumCast::from(self.x), NumCast::from(self.y)) {
            (Some(x), Some(y)) => Some(Translation2D::new(x, y)),
            _ => None,
        }
    }

    // Convenience functions for common casts.

    /// Cast into an `f32` vector.
    #[inline]
    pub fn to_f32(self) -> Translation2D<f32, Src, Dst> {
        self.cast()
    }

    /// Cast into an `f64` vector.
    #[inline]
    pub fn to_f64(self) -> Translation2D<f64, Src, Dst> {
        self.cast()
    }

    /// Cast into an `usize` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_usize(self) -> Translation2D<usize, Src, Dst> {
        self.cast()
    }

    /// Cast into an `u32` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_u32(self) -> Translation2D<u32, Src, Dst> {
        self.cast()
    }

    /// Cast into an i32 vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i32(self) -> Translation2D<i32, Src, Dst> {
        self.cast()
    }

    /// Cast into an i64 vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i64(self) -> Translation2D<i64, Src, Dst> {
        self.cast()
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, Src, Dst> Zeroable for Translation2D<T, Src, Dst> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, Src: 'static, Dst: 'static> Pod for Translation2D<T, Src, Dst> {}

impl<T: Add, Src, Dst1, Dst2> Add<Translation2D<T, Dst1, Dst2>> for Translation2D<T, Src, Dst1> {
    type Output = Translation2D<T::Output, Src, Dst2>;

    fn add(self, other: Translation2D<T, Dst1, Dst2>) -> Self::Output {
        Translation2D::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: AddAssign, Src, Dst> AddAssign<Translation2D<T, Dst, Dst>> for Translation2D<T, Src, Dst> {
    fn add_assign(&mut self, other: Translation2D<T, Dst, Dst>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Sub, Src, Dst1, Dst2> Sub<Translation2D<T, Dst1, Dst2>> for Translation2D<T, Src, Dst2> {
    type Output = Translation2D<T::Output, Src, Dst1>;

    fn sub(self, other: Translation2D<T, Dst1, Dst2>) -> Self::Output {
        Translation2D::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: SubAssign, Src, Dst> SubAssign<Translation2D<T, Dst, Dst>> for Translation2D<T, Src, Dst> {
    fn sub_assign(&mut self, other: Translation2D<T, Dst, Dst>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T, Src, Dst> From<Vector2D<T, Src>> for Translation2D<T, Src, Dst> {
    fn from(v: Vector2D<T, Src>) -> Self {
        Translation2D::new(v.x, v.y)
    }
}

impl<T, Src, Dst> From<Translation2D<T, Src, Dst>> for Vector2D<T, Src> {
    fn from(t: Translation2D<T, Src, Dst>) -> Self {
        vec2(t.x, t.y)
    }
}

impl<T, Src, Dst> From<Translation2D<T, Src, Dst>> for Transform2D<T, Src, Dst>
where
    T: Zero + One,
{
    fn from(t: Translation2D<T, Src, Dst>) -> Self {
        Transform2D::translation(t.x, t.y)
    }
}

impl<T, Src, Dst> Default for Translation2D<T, Src, Dst>
where
    T: Zero,
{
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: fmt::Debug, Src, Dst> fmt::Debug for Translation2D<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Translation({:?},{:?})", self.x, self.y)
    }
}

/// A 3d transformation from a space to another that can only express translations.
///
/// The main benefit of this type over a [`Vector3D`] is the ability to cast
/// between source and destination spaces.
#[repr(C)]
pub struct Translation3D<T, Src, Dst> {
    pub x: T,
    pub y: T,
    pub z: T,
    #[doc(hidden)]
    pub _unit: PhantomData<(Src, Dst)>,
}

#[cfg(feature = "arbitrary")]
impl<'a, T, Src, Dst> arbitrary::Arbitrary<'a> for Translation3D<T, Src, Dst>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (x, y, z) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(Translation3D {
            x,
            y,
            z,
            _unit: PhantomData,
        })
    }
}

impl<T: Copy, Src, Dst> Copy for Translation3D<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for Translation3D<T, Src, Dst> {
    fn clone(&self) -> Self {
        Translation3D {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
            _unit: PhantomData,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T, Src, Dst> serde::Deserialize<'de> for Translation3D<T, Src, Dst>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y, z) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Translation3D {
            x,
            y,
            z,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
impl<T, Src, Dst> serde::Serialize for Translation3D<T, Src, Dst>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.x, &self.y, &self.z).serialize(serializer)
    }
}

impl<T, Src, Dst> Eq for Translation3D<T, Src, Dst> where T: Eq {}

impl<T, Src, Dst> PartialEq for Translation3D<T, Src, Dst>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T, Src, Dst> Hash for Translation3D<T, Src, Dst>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
        self.z.hash(h);
    }
}

impl<T, Src, Dst> Translation3D<T, Src, Dst> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Translation3D {
            x,
            y,
            z,
            _unit: PhantomData,
        }
    }

    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Translation3D {
            x: v.clone(),
            y: v.clone(),
            z: v,
            _unit: PhantomData,
        }
    }

    /// Creates no-op translation (`x`, `y` and `z` is `zero()`).
    #[inline]
    pub fn identity() -> Self
    where
        T: Zero,
    {
        Translation3D::new(T::zero(), T::zero(), T::zero())
    }

    /// Check if translation does nothing (`x`, `y` and `z` is `zero()`).
    ///
    /// ```rust
    /// use euclid::default::Translation3D;
    ///
    /// assert_eq!(Translation3D::<f32>::identity().is_identity(), true);
    /// assert_eq!(Translation3D::new(0, 0, 0).is_identity(), true);
    /// assert_eq!(Translation3D::new(1, 0, 0).is_identity(), false);
    /// assert_eq!(Translation3D::new(0, 1, 0).is_identity(), false);
    /// assert_eq!(Translation3D::new(0, 0, 1).is_identity(), false);
    /// ```
    #[inline]
    pub fn is_identity(&self) -> bool
    where
        T: Zero + PartialEq,
    {
        let _0 = T::zero();
        self.x == _0 && self.y == _0 && self.z == _0
    }

    /// No-op, just cast the unit.
    #[inline]
    pub fn transform_size(self, s: Size2D<T, Src>) -> Size2D<T, Dst> {
        Size2D::new(s.width, s.height)
    }
}

impl<T: Copy, Src, Dst> Translation3D<T, Src, Dst> {
    /// Cast into a 3D vector.
    #[inline]
    pub fn to_vector(&self) -> Vector3D<T, Src> {
        vec3(self.x, self.y, self.z)
    }

    /// Cast into an array with x, y and z.
    #[inline]
    pub fn to_array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    /// Cast into a tuple with x, y and z.
    #[inline]
    pub fn to_tuple(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(&self) -> Translation3D<T, UnknownUnit, UnknownUnit> {
        Translation3D {
            x: self.x,
            y: self.y,
            z: self.z,
            _unit: PhantomData,
        }
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(t: &Translation3D<T, UnknownUnit, UnknownUnit>) -> Self {
        Translation3D {
            x: t.x,
            y: t.y,
            z: t.z,
            _unit: PhantomData,
        }
    }

    /// Returns the matrix representation of this translation.
    #[inline]
    pub fn to_transform(&self) -> Transform3D<T, Src, Dst>
    where
        T: Zero + One,
    {
        (*self).into()
    }

    /// Translate a point and cast its unit.
    #[inline]
    pub fn transform_point3d(&self, p: &Point3D<T, Src>) -> Point3D<T::Output, Dst>
    where
        T: Add,
    {
        point3(p.x + self.x, p.y + self.y, p.z + self.z)
    }

    /// Translate a point and cast its unit.
    #[inline]
    pub fn transform_point2d(&self, p: &Point2D<T, Src>) -> Point2D<T::Output, Dst>
    where
        T: Add,
    {
        point2(p.x + self.x, p.y + self.y)
    }

    /// Translate a 2D box and cast its unit.
    #[inline]
    pub fn transform_box2d(&self, b: &Box2D<T, Src>) -> Box2D<T::Output, Dst>
    where
        T: Add,
    {
        Box2D {
            min: self.transform_point2d(&b.min),
            max: self.transform_point2d(&b.max),
        }
    }

    /// Translate a 3D box and cast its unit.
    #[inline]
    pub fn transform_box3d(&self, b: &Box3D<T, Src>) -> Box3D<T::Output, Dst>
    where
        T: Add,
    {
        Box3D {
            min: self.transform_point3d(&b.min),
            max: self.transform_point3d(&b.max),
        }
    }

    /// Translate a rectangle and cast its unit.
    #[inline]
    pub fn transform_rect(&self, r: &Rect<T, Src>) -> Rect<T, Dst>
    where
        T: Add<Output = T>,
    {
        Rect {
            origin: self.transform_point2d(&r.origin),
            size: self.transform_size(r.size),
        }
    }

    /// Return the inverse transformation.
    #[inline]
    pub fn inverse(&self) -> Translation3D<T::Output, Dst, Src>
    where
        T: Neg,
    {
        Translation3D::new(-self.x, -self.y, -self.z)
    }
}

impl<T: NumCast + Copy, Src, Dst> Translation3D<T, Src, Dst> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn cast<NewT: NumCast>(self) -> Translation3D<NewT, Src, Dst> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    pub fn try_cast<NewT: NumCast>(self) -> Option<Translation3D<NewT, Src, Dst>> {
        match (
            NumCast::from(self.x),
            NumCast::from(self.y),
            NumCast::from(self.z),
        ) {
            (Some(x), Some(y), Some(z)) => Some(Translation3D::new(x, y, z)),
            _ => None,
        }
    }

    // Convenience functions for common casts.

    /// Cast into an `f32` vector.
    #[inline]
    pub fn to_f32(self) -> Translation3D<f32, Src, Dst> {
        self.cast()
    }

    /// Cast into an `f64` vector.
    #[inline]
    pub fn to_f64(self) -> Translation3D<f64, Src, Dst> {
        self.cast()
    }

    /// Cast into an `usize` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_usize(self) -> Translation3D<usize, Src, Dst> {
        self.cast()
    }

    /// Cast into an `u32` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_u32(self) -> Translation3D<u32, Src, Dst> {
        self.cast()
    }

    /// Cast into an i32 vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i32(self) -> Translation3D<i32, Src, Dst> {
        self.cast()
    }

    /// Cast into an i64 vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i64(self) -> Translation3D<i64, Src, Dst> {
        self.cast()
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, Src, Dst> Zeroable for Translation3D<T, Src, Dst> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, Src: 'static, Dst: 'static> Pod for Translation3D<T, Src, Dst> {}

impl<T: Add, Src, Dst1, Dst2> Add<Translation3D<T, Dst1, Dst2>> for Translation3D<T, Src, Dst1> {
    type Output = Translation3D<T::Output, Src, Dst2>;

    fn add(self, other: Translation3D<T, Dst1, Dst2>) -> Self::Output {
        Translation3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: AddAssign, Src, Dst> AddAssign<Translation3D<T, Dst, Dst>> for Translation3D<T, Src, Dst> {
    fn add_assign(&mut self, other: Translation3D<T, Dst, Dst>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Sub, Src, Dst1, Dst2> Sub<Translation3D<T, Dst1, Dst2>> for Translation3D<T, Src, Dst2> {
    type Output = Translation3D<T::Output, Src, Dst1>;

    fn sub(self, other: Translation3D<T, Dst1, Dst2>) -> Self::Output {
        Translation3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: SubAssign, Src, Dst> SubAssign<Translation3D<T, Dst, Dst>> for Translation3D<T, Src, Dst> {
    fn sub_assign(&mut self, other: Translation3D<T, Dst, Dst>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T, Src, Dst> From<Vector3D<T, Src>> for Translation3D<T, Src, Dst> {
    fn from(v: Vector3D<T, Src>) -> Self {
        Translation3D::new(v.x, v.y, v.z)
    }
}

impl<T, Src, Dst> From<Translation3D<T, Src, Dst>> for Vector3D<T, Src> {
    fn from(t: Translation3D<T, Src, Dst>) -> Self {
        vec3(t.x, t.y, t.z)
    }
}

impl<T, Src, Dst> From<Translation3D<T, Src, Dst>> for Transform3D<T, Src, Dst>
where
    T: Zero + One,
{
    fn from(t: Translation3D<T, Src, Dst>) -> Self {
        Transform3D::translation(t.x, t.y, t.z)
    }
}

impl<T, Src, Dst> Default for Translation3D<T, Src, Dst>
where
    T: Zero,
{
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: fmt::Debug, Src, Dst> fmt::Debug for Translation3D<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Translation({:?},{:?},{:?})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod _2d {
    #[test]
    fn simple() {
        use crate::{rect, Rect, Translation2D};

        struct A;
        struct B;

        type Translation = Translation2D<i32, A, B>;
        type SrcRect = Rect<i32, A>;
        type DstRect = Rect<i32, B>;

        let tx = Translation::new(10, -10);
        let r1: SrcRect = rect(10, 20, 30, 40);
        let r2: DstRect = tx.transform_rect(&r1);
        assert_eq!(r2, rect(20, 10, 30, 40));

        let inv_tx = tx.inverse();
        assert_eq!(inv_tx.transform_rect(&r2), r1);

        assert!((tx + inv_tx).is_identity());
    }

    /// Operation tests
    mod ops {
        use crate::default::Translation2D;

        #[test]
        fn test_add() {
            let t1 = Translation2D::new(1.0, 2.0);
            let t2 = Translation2D::new(3.0, 4.0);
            assert_eq!(t1 + t2, Translation2D::new(4.0, 6.0));

            let t1 = Translation2D::new(1.0, 2.0);
            let t2 = Translation2D::new(0.0, 0.0);
            assert_eq!(t1 + t2, Translation2D::new(1.0, 2.0));

            let t1 = Translation2D::new(1.0, 2.0);
            let t2 = Translation2D::new(-3.0, -4.0);
            assert_eq!(t1 + t2, Translation2D::new(-2.0, -2.0));

            let t1 = Translation2D::new(0.0, 0.0);
            let t2 = Translation2D::new(0.0, 0.0);
            assert_eq!(t1 + t2, Translation2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_add_assign() {
            let mut t = Translation2D::new(1.0, 2.0);
            t += Translation2D::new(3.0, 4.0);
            assert_eq!(t, Translation2D::new(4.0, 6.0));

            let mut t = Translation2D::new(1.0, 2.0);
            t += Translation2D::new(0.0, 0.0);
            assert_eq!(t, Translation2D::new(1.0, 2.0));

            let mut t = Translation2D::new(1.0, 2.0);
            t += Translation2D::new(-3.0, -4.0);
            assert_eq!(t, Translation2D::new(-2.0, -2.0));

            let mut t = Translation2D::new(0.0, 0.0);
            t += Translation2D::new(0.0, 0.0);
            assert_eq!(t, Translation2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_sub() {
            let t1 = Translation2D::new(1.0, 2.0);
            let t2 = Translation2D::new(3.0, 4.0);
            assert_eq!(t1 - t2, Translation2D::new(-2.0, -2.0));

            let t1 = Translation2D::new(1.0, 2.0);
            let t2 = Translation2D::new(0.0, 0.0);
            assert_eq!(t1 - t2, Translation2D::new(1.0, 2.0));

            let t1 = Translation2D::new(1.0, 2.0);
            let t2 = Translation2D::new(-3.0, -4.0);
            assert_eq!(t1 - t2, Translation2D::new(4.0, 6.0));

            let t1 = Translation2D::new(0.0, 0.0);
            let t2 = Translation2D::new(0.0, 0.0);
            assert_eq!(t1 - t2, Translation2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_sub_assign() {
            let mut t = Translation2D::new(1.0, 2.0);
            t -= Translation2D::new(3.0, 4.0);
            assert_eq!(t, Translation2D::new(-2.0, -2.0));

            let mut t = Translation2D::new(1.0, 2.0);
            t -= Translation2D::new(0.0, 0.0);
            assert_eq!(t, Translation2D::new(1.0, 2.0));

            let mut t = Translation2D::new(1.0, 2.0);
            t -= Translation2D::new(-3.0, -4.0);
            assert_eq!(t, Translation2D::new(4.0, 6.0));

            let mut t = Translation2D::new(0.0, 0.0);
            t -= Translation2D::new(0.0, 0.0);
            assert_eq!(t, Translation2D::new(0.0, 0.0));
        }
    }
}

#[cfg(test)]
mod _3d {
    #[test]
    fn simple() {
        use crate::{point3, Point3D, Translation3D};

        struct A;
        struct B;

        type Translation = Translation3D<i32, A, B>;
        type SrcPoint = Point3D<i32, A>;
        type DstPoint = Point3D<i32, B>;

        let tx = Translation::new(10, -10, 100);
        let p1: SrcPoint = point3(10, 20, 30);
        let p2: DstPoint = tx.transform_point3d(&p1);
        assert_eq!(p2, point3(20, 10, 130));

        let inv_tx = tx.inverse();
        assert_eq!(inv_tx.transform_point3d(&p2), p1);

        assert!((tx + inv_tx).is_identity());
    }

    /// Operation tests
    mod ops {
        use crate::default::Translation3D;

        #[test]
        pub fn test_add() {
            let t1 = Translation3D::new(1.0, 2.0, 3.0);
            let t2 = Translation3D::new(4.0, 5.0, 6.0);
            assert_eq!(t1 + t2, Translation3D::new(5.0, 7.0, 9.0));

            let t1 = Translation3D::new(1.0, 2.0, 3.0);
            let t2 = Translation3D::new(0.0, 0.0, 0.0);
            assert_eq!(t1 + t2, Translation3D::new(1.0, 2.0, 3.0));

            let t1 = Translation3D::new(1.0, 2.0, 3.0);
            let t2 = Translation3D::new(-4.0, -5.0, -6.0);
            assert_eq!(t1 + t2, Translation3D::new(-3.0, -3.0, -3.0));

            let t1 = Translation3D::new(0.0, 0.0, 0.0);
            let t2 = Translation3D::new(0.0, 0.0, 0.0);
            assert_eq!(t1 + t2, Translation3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_add_assign() {
            let mut t = Translation3D::new(1.0, 2.0, 3.0);
            t += Translation3D::new(4.0, 5.0, 6.0);
            assert_eq!(t, Translation3D::new(5.0, 7.0, 9.0));

            let mut t = Translation3D::new(1.0, 2.0, 3.0);
            t += Translation3D::new(0.0, 0.0, 0.0);
            assert_eq!(t, Translation3D::new(1.0, 2.0, 3.0));

            let mut t = Translation3D::new(1.0, 2.0, 3.0);
            t += Translation3D::new(-4.0, -5.0, -6.0);
            assert_eq!(t, Translation3D::new(-3.0, -3.0, -3.0));

            let mut t = Translation3D::new(0.0, 0.0, 0.0);
            t += Translation3D::new(0.0, 0.0, 0.0);
            assert_eq!(t, Translation3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_sub() {
            let t1 = Translation3D::new(1.0, 2.0, 3.0);
            let t2 = Translation3D::new(4.0, 5.0, 6.0);
            assert_eq!(t1 - t2, Translation3D::new(-3.0, -3.0, -3.0));

            let t1 = Translation3D::new(1.0, 2.0, 3.0);
            let t2 = Translation3D::new(0.0, 0.0, 0.0);
            assert_eq!(t1 - t2, Translation3D::new(1.0, 2.0, 3.0));

            let t1 = Translation3D::new(1.0, 2.0, 3.0);
            let t2 = Translation3D::new(-4.0, -5.0, -6.0);
            assert_eq!(t1 - t2, Translation3D::new(5.0, 7.0, 9.0));

            let t1 = Translation3D::new(0.0, 0.0, 0.0);
            let t2 = Translation3D::new(0.0, 0.0, 0.0);
            assert_eq!(t1 - t2, Translation3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_sub_assign() {
            let mut t = Translation3D::new(1.0, 2.0, 3.0);
            t -= Translation3D::new(4.0, 5.0, 6.0);
            assert_eq!(t, Translation3D::new(-3.0, -3.0, -3.0));

            let mut t = Translation3D::new(1.0, 2.0, 3.0);
            t -= Translation3D::new(0.0, 0.0, 0.0);
            assert_eq!(t, Translation3D::new(1.0, 2.0, 3.0));

            let mut t = Translation3D::new(1.0, 2.0, 3.0);
            t -= Translation3D::new(-4.0, -5.0, -6.0);
            assert_eq!(t, Translation3D::new(5.0, 7.0, 9.0));

            let mut t = Translation3D::new(0.0, 0.0, 0.0);
            t -= Translation3D::new(0.0, 0.0, 0.0);
            assert_eq!(t, Translation3D::new(0.0, 0.0, 0.0));
        }
    }
}
