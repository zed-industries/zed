// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! A type-checked scaling factor between units.

use crate::num::One;

use crate::approxord::{max, min};
use crate::{Box2D, Box3D, Point2D, Point3D, Rect, Size2D, Vector2D};

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
use num_traits::NumCast;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A scaling factor between two different units of measurement.
///
/// This is effectively a type-safe float, intended to be used in combination with other types like
/// `length::Length` to enforce conversion between systems of measurement at compile time.
///
/// `Src` and `Dst` represent the units before and after multiplying a value by a `Scale`. They
/// may be types without values, such as empty enums.  For example:
///
/// ```rust
/// use euclid::Scale;
/// use euclid::Length;
/// enum Mm {};
/// enum Inch {};
///
/// let mm_per_inch: Scale<f32, Inch, Mm> = Scale::new(25.4);
///
/// let one_foot: Length<f32, Inch> = Length::new(12.0);
/// let one_foot_in_mm: Length<f32, Mm> = one_foot * mm_per_inch;
/// ```
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "T: serde::Serialize",
        deserialize = "T: serde::Deserialize<'de>"
    ))
)]
pub struct Scale<T, Src, Dst>(pub T, #[doc(hidden)] pub PhantomData<(Src, Dst)>);

impl<T, Src, Dst> Scale<T, Src, Dst> {
    #[inline]
    pub const fn new(x: T) -> Self {
        Scale(x, PhantomData)
    }

    /// Creates an identity scale (1.0).
    #[inline]
    pub fn identity() -> Self
    where
        T: One,
    {
        Scale::new(T::one())
    }

    /// Returns the given point transformed by this scale.
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::{Scale, point2};
    /// enum Mm {};
    /// enum Cm {};
    ///
    /// let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);
    ///
    /// assert_eq!(to_mm.transform_point(point2(42, -42)), point2(420, -420));
    /// ```
    #[inline]
    pub fn transform_point(self, point: Point2D<T, Src>) -> Point2D<T::Output, Dst>
    where
        T: Copy + Mul,
    {
        Point2D::new(point.x * self.0, point.y * self.0)
    }

    /// Returns the given point transformed by this scale.
    #[inline]
    pub fn transform_point3d(self, point: Point3D<T, Src>) -> Point3D<T::Output, Dst>
    where
        T: Copy + Mul,
    {
        Point3D::new(point.x * self.0, point.y * self.0, point.z * self.0)
    }

    /// Returns the given vector transformed by this scale.
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::{Scale, vec2};
    /// enum Mm {};
    /// enum Cm {};
    ///
    /// let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);
    ///
    /// assert_eq!(to_mm.transform_vector(vec2(42, -42)), vec2(420, -420));
    /// ```
    #[inline]
    pub fn transform_vector(self, vec: Vector2D<T, Src>) -> Vector2D<T::Output, Dst>
    where
        T: Copy + Mul,
    {
        Vector2D::new(vec.x * self.0, vec.y * self.0)
    }

    /// Returns the given size transformed by this scale.
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::{Scale, size2};
    /// enum Mm {};
    /// enum Cm {};
    ///
    /// let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);
    ///
    /// assert_eq!(to_mm.transform_size(size2(42, -42)), size2(420, -420));
    /// ```
    #[inline]
    pub fn transform_size(self, size: Size2D<T, Src>) -> Size2D<T::Output, Dst>
    where
        T: Copy + Mul,
    {
        Size2D::new(size.width * self.0, size.height * self.0)
    }

    /// Returns the given rect transformed by this scale.
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::{Scale, rect};
    /// enum Mm {};
    /// enum Cm {};
    ///
    /// let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);
    ///
    /// assert_eq!(to_mm.transform_rect(&rect(1, 2, 42, -42)), rect(10, 20, 420, -420));
    /// ```
    #[inline]
    pub fn transform_rect(self, rect: &Rect<T, Src>) -> Rect<T::Output, Dst>
    where
        T: Copy + Mul,
    {
        Rect::new(
            self.transform_point(rect.origin),
            self.transform_size(rect.size),
        )
    }

    /// Returns the given box transformed by this scale.
    #[inline]
    pub fn transform_box2d(self, b: &Box2D<T, Src>) -> Box2D<T::Output, Dst>
    where
        T: Copy + Mul,
    {
        Box2D {
            min: self.transform_point(b.min),
            max: self.transform_point(b.max),
        }
    }

    /// Returns the given box transformed by this scale.
    #[inline]
    pub fn transform_box3d(self, b: &Box3D<T, Src>) -> Box3D<T::Output, Dst>
    where
        T: Copy + Mul,
    {
        Box3D {
            min: self.transform_point3d(b.min),
            max: self.transform_point3d(b.max),
        }
    }

    /// Returns `true` if this scale has no effect.
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::Scale;
    /// use euclid::num::One;
    /// enum Mm {};
    /// enum Cm {};
    ///
    /// let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);
    /// let mm_per_mm: Scale<f32, Mm, Mm> = Scale::new(1.0);
    ///
    /// assert_eq!(cm_per_mm.is_identity(), false);
    /// assert_eq!(mm_per_mm.is_identity(), true);
    /// assert_eq!(mm_per_mm, Scale::one());
    /// ```
    #[inline]
    pub fn is_identity(self) -> bool
    where
        T: PartialEq + One,
    {
        self.0 == T::one()
    }

    /// Returns the underlying scalar scale factor.
    #[inline]
    pub fn get(self) -> T {
        self.0
    }

    /// The inverse Scale (1.0 / self).
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::Scale;
    /// enum Mm {};
    /// enum Cm {};
    ///
    /// let cm_per_mm: Scale<f32, Cm, Mm> = Scale::new(0.1);
    ///
    /// assert_eq!(cm_per_mm.inverse(), Scale::new(10.0));
    /// ```
    pub fn inverse(self) -> Scale<T::Output, Dst, Src>
    where
        T: One + Div,
    {
        let one: T = One::one();
        Scale::new(one / self.0)
    }
}

impl<T: PartialOrd, Src, Dst> Scale<T, Src, Dst> {
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self::new(min(self.0, other.0))
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self::new(max(self.0, other.0))
    }

    /// Returns the point each component of which clamped by corresponding
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
}

impl<T: NumCast, Src, Dst> Scale<T, Src, Dst> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// # Panics
    ///
    /// If the source value cannot be represented by the target type `NewT`, then
    /// method panics. Use `try_cast` if that must be case.
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::Scale;
    /// enum Mm {};
    /// enum Cm {};
    ///
    /// let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);
    ///
    /// assert_eq!(to_mm.cast::<f32>(), Scale::new(10.0));
    /// ```
    /// That conversion will panic, because `i32` not enough to store such big numbers:
    /// ```rust,should_panic
    /// use euclid::Scale;
    /// enum Mm {};// millimeter = 10^-2 meters
    /// enum Em {};// exameter   = 10^18 meters
    ///
    /// // Panics
    /// let to_em: Scale<i32, Mm, Em> = Scale::new(10e20).cast();
    /// ```
    #[inline]
    pub fn cast<NewT: NumCast>(self) -> Scale<NewT, Src, Dst> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    /// If the source value cannot be represented by the target type `NewT`, then `None`
    /// is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use euclid::Scale;
    /// enum Mm {};
    /// enum Cm {};
    /// enum Em {};// Exameter = 10^18 meters
    ///
    /// let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);
    /// let to_em: Scale<f32, Mm, Em> = Scale::new(10e20);
    ///
    /// assert_eq!(to_mm.try_cast::<f32>(), Some(Scale::new(10.0)));
    /// // Integer to small to store that number
    /// assert_eq!(to_em.try_cast::<i32>(), None);
    /// ```
    pub fn try_cast<NewT: NumCast>(self) -> Option<Scale<NewT, Src, Dst>> {
        NumCast::from(self.0).map(Scale::new)
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T, Src, Dst> arbitrary::Arbitrary<'a> for Scale<T, Src, Dst>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Scale::new(arbitrary::Arbitrary::arbitrary(u)?))
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, Src, Dst> Zeroable for Scale<T, Src, Dst> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, Src: 'static, Dst: 'static> Pod for Scale<T, Src, Dst> {}

// scale0 * scale1
// (A,B) * (B,C) = (A,C)
impl<T: Mul, A, B, C> Mul<Scale<T, B, C>> for Scale<T, A, B> {
    type Output = Scale<T::Output, A, C>;

    #[inline]
    fn mul(self, other: Scale<T, B, C>) -> Self::Output {
        Scale::new(self.0 * other.0)
    }
}

// scale0 + scale1
impl<T: Add, Src, Dst> Add for Scale<T, Src, Dst> {
    type Output = Scale<T::Output, Src, Dst>;

    #[inline]
    fn add(self, other: Scale<T, Src, Dst>) -> Self::Output {
        Scale::new(self.0 + other.0)
    }
}

// scale0 - scale1
impl<T: Sub, Src, Dst> Sub for Scale<T, Src, Dst> {
    type Output = Scale<T::Output, Src, Dst>;

    #[inline]
    fn sub(self, other: Scale<T, Src, Dst>) -> Self::Output {
        Scale::new(self.0 - other.0)
    }
}

// FIXME: Switch to `derive(PartialEq, Clone)` after this Rust issue is fixed:
// https://github.com/rust-lang/rust/issues/26925

impl<T: PartialEq, Src, Dst> PartialEq for Scale<T, Src, Dst> {
    fn eq(&self, other: &Scale<T, Src, Dst>) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq, Src, Dst> Eq for Scale<T, Src, Dst> {}

impl<T: PartialOrd, Src, Dst> PartialOrd for Scale<T, Src, Dst> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord, Src, Dst> Ord for Scale<T, Src, Dst> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Clone, Src, Dst> Clone for Scale<T, Src, Dst> {
    fn clone(&self) -> Scale<T, Src, Dst> {
        Scale::new(self.0.clone())
    }
}

impl<T: Copy, Src, Dst> Copy for Scale<T, Src, Dst> {}

impl<T: fmt::Debug, Src, Dst> fmt::Debug for Scale<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Default, Src, Dst> Default for Scale<T, Src, Dst> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Hash, Src, Dst> Hash for Scale<T, Src, Dst> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T: One, Src, Dst> One for Scale<T, Src, Dst> {
    #[inline]
    fn one() -> Self {
        Scale::new(T::one())
    }
}

#[cfg(test)]
mod tests {
    use super::Scale;

    enum Inch {}
    enum Cm {}
    enum Mm {}

    #[test]
    fn test_scale() {
        let mm_per_inch: Scale<f32, Inch, Mm> = Scale::new(25.4);
        let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

        let mm_per_cm: Scale<f32, Cm, Mm> = cm_per_mm.inverse();
        assert_eq!(mm_per_cm.get(), 10.0);

        let one: Scale<f32, Mm, Mm> = cm_per_mm * mm_per_cm;
        assert_eq!(one.get(), 1.0);

        let one: Scale<f32, Cm, Cm> = mm_per_cm * cm_per_mm;
        assert_eq!(one.get(), 1.0);

        let cm_per_inch: Scale<f32, Inch, Cm> = mm_per_inch * cm_per_mm;
        //  mm     cm     cm
        // ---- x ---- = ----
        // inch    mm    inch
        assert_eq!(cm_per_inch, Scale::new(2.54));

        let a: Scale<isize, Inch, Inch> = Scale::new(2);
        let b: Scale<isize, Inch, Inch> = Scale::new(3);
        assert_ne!(a, b);
        assert_eq!(a, a.clone());
        assert_eq!(a.clone() + b.clone(), Scale::new(5));
        assert_eq!(a - b, Scale::new(-1));

        // Clamp
        assert_eq!(Scale::identity().clamp(a, b), a);
        assert_eq!(Scale::new(5).clamp(a, b), b);
        let a = Scale::<f32, Inch, Inch>::new(2.0);
        let b = Scale::<f32, Inch, Inch>::new(3.0);
        let c = Scale::<f32, Inch, Inch>::new(2.5);
        assert_eq!(c.clamp(a, b), c);
    }
}
