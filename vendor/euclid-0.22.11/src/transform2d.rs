// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::just_underscores_and_digits)]

use super::{Angle, UnknownUnit};
use crate::approxeq::ApproxEq;
use crate::box2d::Box2D;
use crate::num::{One, Zero};
use crate::point::{point2, Point2D};
use crate::rect::Rect;
use crate::transform3d::Transform3D;
use crate::trig::Trig;
use crate::vector::{vec2, Vector2D};
use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "mint")]
use mint;
use num_traits::NumCast;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A 2d transform represented by a column-major 3 by 3 matrix, compressed down to 3 by 2.
///
/// Transforms can be parametrized over the source and destination units, to describe a
/// transformation from a space to another.
/// For example, `Transform2D<f32, WorldSpace, ScreenSpace>::transform_point4d`
/// takes a `Point2D<f32, WorldSpace>` and returns a `Point2D<f32, ScreenSpace>`.
///
/// Transforms expose a set of convenience methods for pre- and post-transformations.
/// Pre-transformations (`pre_*` methods) correspond to adding an operation that is
/// applied before the rest of the transformation, while post-transformations (`then_*`
/// methods) add an operation that is applied after.
///
/// The matrix representation is conceptually equivalent to a 3 by 3 matrix transformation
/// compressed to 3 by 2 with the components that aren't needed to describe the set of 2d
/// transformations we are interested in implicitly defined:
///
/// ```text
///  | m11 m21 m31 |   |x|   |x'|
///  | m12 m22 m32 | x |y| = |y'|
///  |   0   0   1 |   |1|   |1 |
/// ```
///
/// When translating `Transform2D` into general matrix representations, consider that the
/// representation follows the column-major notation with column vectors.
///
/// The translation terms are `m31` and `m32`.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
)]
#[rustfmt::skip]
pub struct Transform2D<T, Src, Dst> {
    pub m11: T, pub m12: T,
    pub m21: T, pub m22: T,
    pub m31: T, pub m32: T,
    #[doc(hidden)]
    pub _unit: PhantomData<(Src, Dst)>,
}

#[cfg(feature = "arbitrary")]
impl<'a, T, Src, Dst> arbitrary::Arbitrary<'a> for Transform2D<T, Src, Dst>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (m11, m12, m21, m22, m31, m32) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(Transform2D {
            m11,
            m12,
            m21,
            m22,
            m31,
            m32,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, Src, Dst> Zeroable for Transform2D<T, Src, Dst> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, Src: 'static, Dst: 'static> Pod for Transform2D<T, Src, Dst> {}

impl<T: Copy, Src, Dst> Copy for Transform2D<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for Transform2D<T, Src, Dst> {
    fn clone(&self) -> Self {
        Transform2D {
            m11: self.m11.clone(),
            m12: self.m12.clone(),
            m21: self.m21.clone(),
            m22: self.m22.clone(),
            m31: self.m31.clone(),
            m32: self.m32.clone(),
            _unit: PhantomData,
        }
    }
}

impl<T, Src, Dst> Eq for Transform2D<T, Src, Dst> where T: Eq {}

impl<T, Src, Dst> PartialEq for Transform2D<T, Src, Dst>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.m11 == other.m11
            && self.m12 == other.m12
            && self.m21 == other.m21
            && self.m22 == other.m22
            && self.m31 == other.m31
            && self.m32 == other.m32
    }
}

impl<T, Src, Dst> Hash for Transform2D<T, Src, Dst>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.m11.hash(h);
        self.m12.hash(h);
        self.m21.hash(h);
        self.m22.hash(h);
        self.m31.hash(h);
        self.m32.hash(h);
    }
}

impl<T, Src, Dst> Transform2D<T, Src, Dst> {
    /// Create a transform specifying its components in using the column-major-column-vector
    /// matrix notation.
    ///
    /// For example, the translation terms m31 and m32 are the last two parameters parameters.
    ///
    /// ```
    /// use euclid::default::Transform2D;
    /// let tx = 1.0;
    /// let ty = 2.0;
    /// let translation = Transform2D::new(
    ///   1.0, 0.0,
    ///   0.0, 1.0,
    ///   tx,  ty,
    /// );
    /// ```
    #[rustfmt::skip]
    pub const fn new(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> Self {
        Transform2D {
            m11, m12,
            m21, m22,
            m31, m32,
            _unit: PhantomData,
        }
    }

    /// Returns `true` if this transform is approximately equal to the other one, using
    /// `T`'s default epsilon value.
    ///
    /// The same as [`ApproxEq::approx_eq`] but available without importing trait.
    #[inline]
    pub fn approx_eq(&self, other: &Self) -> bool
    where
        T: ApproxEq<T>,
    {
        <Self as ApproxEq<T>>::approx_eq(&self, &other)
    }

    /// Returns `true` if this transform is approximately equal to the other one, using
    /// a provided epsilon value.
    ///
    /// The same as [`ApproxEq::approx_eq_eps`] but available without importing trait.
    #[inline]
    pub fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool
    where
        T: ApproxEq<T>,
    {
        <Self as ApproxEq<T>>::approx_eq_eps(&self, &other, &eps)
    }
}

impl<T: Copy, Src, Dst> Transform2D<T, Src, Dst> {
    /// Returns an array containing this transform's terms.
    ///
    /// The terms are laid out in the same order as they are
    /// specified in [`Transform2D::new`], that is following the
    /// column-major-column-vector matrix notation.
    ///
    /// For example the translation terms are found in the
    /// last two slots of the array.
    #[inline]
    #[rustfmt::skip]
    pub fn to_array(&self) -> [T; 6] {
        [
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32
        ]
    }

    /// Returns an array containing this transform's terms transposed.
    ///
    /// The terms are laid out in transposed order from the same order of
    /// `Transform3D::new` and `Transform3D::to_array`, that is following
    /// the row-major-column-vector matrix notation.
    ///
    /// For example the translation terms are found at indices 2 and 5
    /// in the array.
    #[inline]
    #[rustfmt::skip]
    pub fn to_array_transposed(&self) -> [T; 6] {
        [
            self.m11, self.m21, self.m31,
            self.m12, self.m22, self.m32
        ]
    }

    /// Equivalent to `to_array` with elements packed two at a time
    /// in an array of arrays.
    #[inline]
    pub fn to_arrays(&self) -> [[T; 2]; 3] {
        [
            [self.m11, self.m12],
            [self.m21, self.m22],
            [self.m31, self.m32],
        ]
    }

    /// Create a transform providing its components via an array
    /// of 6 elements instead of as individual parameters.
    ///
    /// The order of the components corresponds to the
    /// column-major-column-vector matrix notation (the same order
    /// as `Transform2D::new`).
    #[inline]
    #[rustfmt::skip]
    pub fn from_array(array: [T; 6]) -> Self {
        Self::new(
            array[0], array[1],
            array[2], array[3],
            array[4], array[5],
        )
    }

    /// Equivalent to `from_array` with elements packed two at a time
    /// in an array of arrays.
    ///
    /// The order of the components corresponds to the
    /// column-major-column-vector matrix notation (the same order
    /// as `Transform3D::new`).
    #[inline]
    #[rustfmt::skip]
    pub fn from_arrays(array: [[T; 2]; 3]) -> Self {
        Self::new(
            array[0][0], array[0][1],
            array[1][0], array[1][1],
            array[2][0], array[2][1],
        )
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    #[rustfmt::skip]
    pub fn to_untyped(&self) -> Transform2D<T, UnknownUnit, UnknownUnit> {
        Transform2D::new(
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32
        )
    }

    /// Tag a unitless value with units.
    #[inline]
    #[rustfmt::skip]
    pub fn from_untyped(p: &Transform2D<T, UnknownUnit, UnknownUnit>) -> Self {
        Transform2D::new(
            p.m11, p.m12,
            p.m21, p.m22,
            p.m31, p.m32
        )
    }

    /// Returns the same transform with a different source unit.
    #[inline]
    #[rustfmt::skip]
    pub fn with_source<NewSrc>(&self) -> Transform2D<T, NewSrc, Dst> {
        Transform2D::new(
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32,
        )
    }

    /// Returns the same transform with a different destination unit.
    #[inline]
    #[rustfmt::skip]
    pub fn with_destination<NewDst>(&self) -> Transform2D<T, Src, NewDst> {
        Transform2D::new(
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32,
        )
    }

    /// Create a 3D transform from the current transform
    pub fn to_3d(&self) -> Transform3D<T, Src, Dst>
    where
        T: Zero + One,
    {
        Transform3D::new_2d(self.m11, self.m12, self.m21, self.m22, self.m31, self.m32)
    }
}

impl<T: NumCast + Copy, Src, Dst> Transform2D<T, Src, Dst> {
    /// Cast from one numeric representation to another, preserving the units.
    #[inline]
    pub fn cast<NewT: NumCast>(&self) -> Transform2D<NewT, Src, Dst> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    #[rustfmt::skip]
    pub fn try_cast<NewT: NumCast>(&self) -> Option<Transform2D<NewT, Src, Dst>> {
        match (NumCast::from(self.m11), NumCast::from(self.m12),
               NumCast::from(self.m21), NumCast::from(self.m22),
               NumCast::from(self.m31), NumCast::from(self.m32)) {
            (Some(m11), Some(m12),
             Some(m21), Some(m22),
             Some(m31), Some(m32)) => {
                Some(Transform2D::new(
                    m11, m12,
                    m21, m22,
                    m31, m32
                ))
            },
            _ => None
        }
    }
}

impl<T, Src, Dst> Transform2D<T, Src, Dst>
where
    T: Zero + One,
{
    /// Create an identity matrix:
    ///
    /// ```text
    /// 1 0
    /// 0 1
    /// 0 0
    /// ```
    #[inline]
    pub fn identity() -> Self {
        Self::translation(T::zero(), T::zero())
    }

    /// Intentional not public, because it checks for exact equivalence
    /// while most consumers will probably want some sort of approximate
    /// equivalence to deal with floating-point errors.
    fn is_identity(&self) -> bool
    where
        T: PartialEq,
    {
        *self == Self::identity()
    }
}

/// Methods for combining generic transformations
impl<T, Src, Dst> Transform2D<T, Src, Dst>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    /// Returns the multiplication of the two matrices such that mat's transformation
    /// applies after self's transformation.
    #[must_use]
    #[rustfmt::skip]
    pub fn then<NewDst>(&self, mat: &Transform2D<T, Dst, NewDst>) -> Transform2D<T, Src, NewDst> {
        Transform2D::new(
            self.m11 * mat.m11 + self.m12 * mat.m21,
            self.m11 * mat.m12 + self.m12 * mat.m22,

            self.m21 * mat.m11 + self.m22 * mat.m21,
            self.m21 * mat.m12 + self.m22 * mat.m22,

            self.m31 * mat.m11 + self.m32 * mat.m21 + mat.m31,
            self.m31 * mat.m12 + self.m32 * mat.m22 + mat.m32,
        )
    }
}

/// Methods for creating and combining translation transformations
impl<T, Src, Dst> Transform2D<T, Src, Dst>
where
    T: Zero + One,
{
    /// Create a 2d translation transform:
    ///
    /// ```text
    /// 1 0
    /// 0 1
    /// x y
    /// ```
    #[inline]
    #[rustfmt::skip]
    pub fn translation(x: T, y: T) -> Self {
        let _0 = || T::zero();
        let _1 = || T::one();

        Self::new(
            _1(), _0(),
            _0(), _1(),
             x,    y,
        )
    }

    /// Applies a translation after self's transformation and returns the resulting transform.
    #[inline]
    #[must_use]
    pub fn then_translate(&self, v: Vector2D<T, Dst>) -> Self
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        self.then(&Transform2D::translation(v.x, v.y))
    }

    /// Applies a translation before self's transformation and returns the resulting transform.
    #[inline]
    #[must_use]
    pub fn pre_translate(&self, v: Vector2D<T, Src>) -> Self
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        Transform2D::translation(v.x, v.y).then(self)
    }
}

/// Methods for creating and combining rotation transformations
impl<T, Src, Dst> Transform2D<T, Src, Dst>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Zero + Trig,
{
    /// Returns a rotation transform.
    #[inline]
    #[rustfmt::skip]
    pub fn rotation(theta: Angle<T>) -> Self {
        let _0 = Zero::zero();
        let cos = theta.get().cos();
        let sin = theta.get().sin();
        Transform2D::new(
            cos, sin,
            _0 - sin, cos,
            _0, _0
        )
    }

    /// Applies a rotation after self's transformation and returns the resulting transform.
    #[inline]
    #[must_use]
    pub fn then_rotate(&self, theta: Angle<T>) -> Self {
        self.then(&Transform2D::rotation(theta))
    }

    /// Applies a rotation before self's transformation and returns the resulting transform.
    #[inline]
    #[must_use]
    pub fn pre_rotate(&self, theta: Angle<T>) -> Self {
        Transform2D::rotation(theta).then(self)
    }
}

/// Methods for creating and combining scale transformations
impl<T, Src, Dst> Transform2D<T, Src, Dst> {
    /// Create a 2d scale transform:
    ///
    /// ```text
    /// x 0
    /// 0 y
    /// 0 0
    /// ```
    #[inline]
    #[rustfmt::skip]
    pub fn scale(x: T, y: T) -> Self
    where
        T: Zero,
    {
        let _0 = || Zero::zero();

        Self::new(
             x,   _0(),
            _0(),  y,
            _0(), _0(),
        )
    }

    /// Applies a scale after self's transformation and returns the resulting transform.
    #[inline]
    #[must_use]
    pub fn then_scale(&self, x: T, y: T) -> Self
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Zero,
    {
        self.then(&Transform2D::scale(x, y))
    }

    /// Applies a scale before self's transformation and returns the resulting transform.
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn pre_scale(&self, x: T, y: T) -> Self
    where
        T: Copy + Mul<Output = T>,
    {
        Transform2D::new(
            self.m11 * x, self.m12 * x,
            self.m21 * y, self.m22 * y,
            self.m31,     self.m32
        )
    }
}

/// Methods for apply transformations to objects
impl<T, Src, Dst> Transform2D<T, Src, Dst>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    /// Returns the given point transformed by this transform.
    #[inline]
    #[must_use]
    pub fn transform_point(&self, point: Point2D<T, Src>) -> Point2D<T, Dst> {
        Point2D::new(
            point.x * self.m11 + point.y * self.m21 + self.m31,
            point.x * self.m12 + point.y * self.m22 + self.m32,
        )
    }

    /// Returns the given vector transformed by this matrix.
    #[inline]
    #[must_use]
    pub fn transform_vector(&self, vec: Vector2D<T, Src>) -> Vector2D<T, Dst> {
        vec2(
            vec.x * self.m11 + vec.y * self.m21,
            vec.x * self.m12 + vec.y * self.m22,
        )
    }

    /// Returns a rectangle that encompasses the result of transforming the given rectangle by this
    /// transform.
    #[inline]
    #[must_use]
    pub fn outer_transformed_rect(&self, rect: &Rect<T, Src>) -> Rect<T, Dst>
    where
        T: Sub<Output = T> + Zero + PartialOrd,
    {
        let min = rect.min();
        let max = rect.max();
        Rect::from_points(&[
            self.transform_point(min),
            self.transform_point(max),
            self.transform_point(point2(max.x, min.y)),
            self.transform_point(point2(min.x, max.y)),
        ])
    }

    /// Returns a box that encompasses the result of transforming the given box by this
    /// transform.
    #[inline]
    #[must_use]
    pub fn outer_transformed_box(&self, b: &Box2D<T, Src>) -> Box2D<T, Dst>
    where
        T: Sub<Output = T> + Zero + PartialOrd,
    {
        Box2D::from_points(&[
            self.transform_point(b.min),
            self.transform_point(b.max),
            self.transform_point(point2(b.max.x, b.min.y)),
            self.transform_point(point2(b.min.x, b.max.y)),
        ])
    }
}

impl<T, Src, Dst> Transform2D<T, Src, Dst>
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialEq + Zero + One,
{
    /// Computes and returns the determinant of this transform.
    pub fn determinant(&self) -> T {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    /// Returns whether it is possible to compute the inverse transform.
    #[inline]
    pub fn is_invertible(&self) -> bool {
        self.determinant() != Zero::zero()
    }

    /// Returns the inverse transform if possible.
    #[must_use]
    pub fn inverse(&self) -> Option<Transform2D<T, Dst, Src>> {
        let det = self.determinant();

        let _0: T = Zero::zero();
        let _1: T = One::one();

        if det == _0 {
            return None;
        }

        let inv_det = _1 / det;
        Some(Transform2D::new(
            inv_det * self.m22,
            inv_det * (_0 - self.m12),
            inv_det * (_0 - self.m21),
            inv_det * self.m11,
            inv_det * (self.m21 * self.m32 - self.m22 * self.m31),
            inv_det * (self.m31 * self.m12 - self.m11 * self.m32),
        ))
    }
}

impl<T, Src, Dst> Default for Transform2D<T, Src, Dst>
where
    T: Zero + One,
{
    /// Returns the [identity transform](Transform2D::identity).
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: ApproxEq<T>, Src, Dst> ApproxEq<T> for Transform2D<T, Src, Dst> {
    #[inline]
    fn approx_epsilon() -> T {
        T::approx_epsilon()
    }

    /// Returns `true` if this transform is approximately equal to the other one, using
    /// a provided epsilon value.
    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        self.m11.approx_eq_eps(&other.m11, eps)
            && self.m12.approx_eq_eps(&other.m12, eps)
            && self.m21.approx_eq_eps(&other.m21, eps)
            && self.m22.approx_eq_eps(&other.m22, eps)
            && self.m31.approx_eq_eps(&other.m31, eps)
            && self.m32.approx_eq_eps(&other.m32, eps)
    }
}

impl<T, Src, Dst> fmt::Debug for Transform2D<T, Src, Dst>
where
    T: Copy + fmt::Debug + PartialEq + One + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_identity() {
            write!(f, "[I]")
        } else {
            self.to_array().fmt(f)
        }
    }
}

#[cfg(feature = "mint")]
impl<T, Src, Dst> From<mint::RowMatrix3x2<T>> for Transform2D<T, Src, Dst> {
    #[rustfmt::skip]
    fn from(m: mint::RowMatrix3x2<T>) -> Self {
        Transform2D {
            m11: m.x.x, m12: m.x.y,
            m21: m.y.x, m22: m.y.y,
            m31: m.z.x, m32: m.z.y,
            _unit: PhantomData,
        }
    }
}
#[cfg(feature = "mint")]
impl<T, Src, Dst> From<Transform2D<T, Src, Dst>> for mint::RowMatrix3x2<T> {
    fn from(t: Transform2D<T, Src, Dst>) -> Self {
        mint::RowMatrix3x2 {
            x: mint::Vector2 { x: t.m11, y: t.m12 },
            y: mint::Vector2 { x: t.m21, y: t.m22 },
            z: mint::Vector2 { x: t.m31, y: t.m32 },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::approxeq::ApproxEq;
    use crate::default;
    #[cfg(feature = "mint")]
    use mint;

    use core::f32::consts::FRAC_PI_2;

    type Mat = default::Transform2D<f32>;

    fn rad(v: f32) -> Angle<f32> {
        Angle::radians(v)
    }

    #[test]
    pub fn test_translation() {
        let t1 = Mat::translation(1.0, 2.0);
        let t2 = Mat::identity().pre_translate(vec2(1.0, 2.0));
        let t3 = Mat::identity().then_translate(vec2(1.0, 2.0));
        assert_eq!(t1, t2);
        assert_eq!(t1, t3);

        assert_eq!(
            t1.transform_point(Point2D::new(1.0, 1.0)),
            Point2D::new(2.0, 3.0)
        );

        assert_eq!(t1.then(&t1), Mat::translation(2.0, 4.0));
    }

    #[test]
    pub fn test_rotation() {
        let r1 = Mat::rotation(rad(FRAC_PI_2));
        let r2 = Mat::identity().pre_rotate(rad(FRAC_PI_2));
        let r3 = Mat::identity().then_rotate(rad(FRAC_PI_2));
        assert_eq!(r1, r2);
        assert_eq!(r1, r3);

        assert!(r1
            .transform_point(Point2D::new(1.0, 2.0))
            .approx_eq(&Point2D::new(-2.0, 1.0)));

        assert!(r1.then(&r1).approx_eq(&Mat::rotation(rad(FRAC_PI_2 * 2.0))));
    }

    #[test]
    pub fn test_scale() {
        let s1 = Mat::scale(2.0, 3.0);
        let s2 = Mat::identity().pre_scale(2.0, 3.0);
        let s3 = Mat::identity().then_scale(2.0, 3.0);
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);

        assert!(s1
            .transform_point(Point2D::new(2.0, 2.0))
            .approx_eq(&Point2D::new(4.0, 6.0)));
    }

    #[test]
    pub fn test_pre_then_scale() {
        let m = Mat::rotation(rad(FRAC_PI_2)).then_translate(vec2(6.0, 7.0));
        let s = Mat::scale(2.0, 3.0);
        assert_eq!(m.then(&s), m.then_scale(2.0, 3.0));
    }

    #[test]
    pub fn test_inverse_simple() {
        let m1 = Mat::identity();
        let m2 = m1.inverse().unwrap();
        assert!(m1.approx_eq(&m2));
    }

    #[test]
    pub fn test_inverse_scale() {
        let m1 = Mat::scale(1.5, 0.3);
        let m2 = m1.inverse().unwrap();
        assert!(m1.then(&m2).approx_eq(&Mat::identity()));
        assert!(m2.then(&m1).approx_eq(&Mat::identity()));
    }

    #[test]
    pub fn test_inverse_translate() {
        let m1 = Mat::translation(-132.0, 0.3);
        let m2 = m1.inverse().unwrap();
        assert!(m1.then(&m2).approx_eq(&Mat::identity()));
        assert!(m2.then(&m1).approx_eq(&Mat::identity()));
    }

    #[test]
    fn test_inverse_none() {
        assert!(Mat::scale(2.0, 0.0).inverse().is_none());
        assert!(Mat::scale(2.0, 2.0).inverse().is_some());
    }

    #[test]
    pub fn test_pre_post() {
        let m1 = default::Transform2D::identity()
            .then_scale(1.0, 2.0)
            .then_translate(vec2(1.0, 2.0));
        let m2 = default::Transform2D::identity()
            .pre_translate(vec2(1.0, 2.0))
            .pre_scale(1.0, 2.0);
        assert!(m1.approx_eq(&m2));

        let r = Mat::rotation(rad(FRAC_PI_2));
        let t = Mat::translation(2.0, 3.0);

        let a = Point2D::new(1.0, 1.0);

        assert!(r
            .then(&t)
            .transform_point(a)
            .approx_eq(&Point2D::new(1.0, 4.0)));
        assert!(t
            .then(&r)
            .transform_point(a)
            .approx_eq(&Point2D::new(-4.0, 3.0)));
        assert!(t
            .then(&r)
            .transform_point(a)
            .approx_eq(&r.transform_point(t.transform_point(a))));
    }

    #[test]
    fn test_size_of() {
        use core::mem::size_of;
        assert_eq!(size_of::<default::Transform2D<f32>>(), 6 * size_of::<f32>());
        assert_eq!(size_of::<default::Transform2D<f64>>(), 6 * size_of::<f64>());
    }

    #[test]
    pub fn test_is_identity() {
        let m1 = default::Transform2D::identity();
        assert!(m1.is_identity());
        let m2 = m1.then_translate(vec2(0.1, 0.0));
        assert!(!m2.is_identity());
    }

    #[test]
    pub fn test_transform_vector() {
        // Translation does not apply to vectors.
        let m1 = Mat::translation(1.0, 1.0);
        let v1 = vec2(10.0, -10.0);
        assert_eq!(v1, m1.transform_vector(v1));
    }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let m1 = Mat::rotation(rad(FRAC_PI_2));
        let mm: mint::RowMatrix3x2<_> = m1.into();
        let m2 = Mat::from(mm);

        assert_eq!(m1, m2);
    }
}
