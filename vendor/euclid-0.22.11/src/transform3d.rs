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
use crate::box3d::Box3D;
use crate::homogen::HomogeneousVector;
use crate::num::{One, Zero};
use crate::point::{point2, point3, Point2D, Point3D};
use crate::rect::Rect;
use crate::scale::Scale;
use crate::transform2d::Transform2D;
use crate::trig::Trig;
use crate::vector::{vec2, vec3, Vector2D, Vector3D};

use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "mint")]
use mint;
use num_traits::NumCast;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A 3d transform stored as a column-major 4 by 4 matrix.
///
/// Transforms can be parametrized over the source and destination units, to describe a
/// transformation from a space to another.
/// For example, `Transform3D<f32, WorldSpace, ScreenSpace>::transform_point3d`
/// takes a `Point3D<f32, WorldSpace>` and returns a `Point3D<f32, ScreenSpace>`.
///
/// Transforms expose a set of convenience methods for pre- and post-transformations.
/// Pre-transformations (`pre_*` methods) correspond to adding an operation that is
/// applied before the rest of the transformation, while post-transformations (`then_*`
/// methods) add an operation that is applied after.
///
/// When translating `Transform3D` into general matrix representations, consider that the
/// representation follows the column major notation with column vectors.
///
/// ```text
///  |x'|   | m11 m12 m13 m14 |   |x|
///  |y'|   | m21 m22 m23 m24 |   |y|
///  |z'| = | m31 m32 m33 m34 | x |y|
///  |w |   | m41 m42 m43 m44 |   |1|
/// ```
///
/// The translation terms are `m41`, `m42` and `m43`.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
)]
#[rustfmt::skip]
pub struct Transform3D<T, Src, Dst> {
    pub m11: T, pub m12: T, pub m13: T, pub m14: T,
    pub m21: T, pub m22: T, pub m23: T, pub m24: T,
    pub m31: T, pub m32: T, pub m33: T, pub m34: T,
    pub m41: T, pub m42: T, pub m43: T, pub m44: T,
    #[doc(hidden)]
    pub _unit: PhantomData<(Src, Dst)>,
}

#[cfg(feature = "arbitrary")]
impl<'a, T, Src, Dst> arbitrary::Arbitrary<'a> for Transform3D<T, Src, Dst>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (m11, m12, m13, m14) = arbitrary::Arbitrary::arbitrary(u)?;
        let (m21, m22, m23, m24) = arbitrary::Arbitrary::arbitrary(u)?;
        let (m31, m32, m33, m34) = arbitrary::Arbitrary::arbitrary(u)?;
        let (m41, m42, m43, m44) = arbitrary::Arbitrary::arbitrary(u)?;

        Ok(Transform3D {
            m11,
            m12,
            m13,
            m14,
            m21,
            m22,
            m23,
            m24,
            m31,
            m32,
            m33,
            m34,
            m41,
            m42,
            m43,
            m44,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, Src, Dst> Zeroable for Transform3D<T, Src, Dst> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, Src: 'static, Dst: 'static> Pod for Transform3D<T, Src, Dst> {}

impl<T: Copy, Src, Dst> Copy for Transform3D<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for Transform3D<T, Src, Dst> {
    fn clone(&self) -> Self {
        Transform3D {
            m11: self.m11.clone(),
            m12: self.m12.clone(),
            m13: self.m13.clone(),
            m14: self.m14.clone(),
            m21: self.m21.clone(),
            m22: self.m22.clone(),
            m23: self.m23.clone(),
            m24: self.m24.clone(),
            m31: self.m31.clone(),
            m32: self.m32.clone(),
            m33: self.m33.clone(),
            m34: self.m34.clone(),
            m41: self.m41.clone(),
            m42: self.m42.clone(),
            m43: self.m43.clone(),
            m44: self.m44.clone(),
            _unit: PhantomData,
        }
    }
}

impl<T, Src, Dst> Eq for Transform3D<T, Src, Dst> where T: Eq {}

impl<T, Src, Dst> PartialEq for Transform3D<T, Src, Dst>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.m11 == other.m11
            && self.m12 == other.m12
            && self.m13 == other.m13
            && self.m14 == other.m14
            && self.m21 == other.m21
            && self.m22 == other.m22
            && self.m23 == other.m23
            && self.m24 == other.m24
            && self.m31 == other.m31
            && self.m32 == other.m32
            && self.m33 == other.m33
            && self.m34 == other.m34
            && self.m41 == other.m41
            && self.m42 == other.m42
            && self.m43 == other.m43
            && self.m44 == other.m44
    }
}

impl<T, Src, Dst> Hash for Transform3D<T, Src, Dst>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.m11.hash(h);
        self.m12.hash(h);
        self.m13.hash(h);
        self.m14.hash(h);
        self.m21.hash(h);
        self.m22.hash(h);
        self.m23.hash(h);
        self.m24.hash(h);
        self.m31.hash(h);
        self.m32.hash(h);
        self.m33.hash(h);
        self.m34.hash(h);
        self.m41.hash(h);
        self.m42.hash(h);
        self.m43.hash(h);
        self.m44.hash(h);
    }
}

impl<T, Src, Dst> Transform3D<T, Src, Dst> {
    /// Create a transform specifying all of it's component as a 4 by 4 matrix.
    ///
    /// Components are specified following column-major-column-vector matrix notation.
    /// For example, the translation terms m41, m42, m43 are the 13rd, 14th and 15th parameters.
    ///
    /// ```
    /// use euclid::default::Transform3D;
    /// let tx = 1.0;
    /// let ty = 2.0;
    /// let tz = 3.0;
    /// let translation = Transform3D::new(
    ///   1.0, 0.0, 0.0, 0.0,
    ///   0.0, 1.0, 0.0, 0.0,
    ///   0.0, 0.0, 1.0, 0.0,
    ///   tx,  ty,  tz,  1.0,
    /// );
    /// ```
    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[rustfmt::skip]
    pub const fn new(
        m11: T, m12: T, m13: T, m14: T,
        m21: T, m22: T, m23: T, m24: T,
        m31: T, m32: T, m33: T, m34: T,
        m41: T, m42: T, m43: T, m44: T,
    ) -> Self {
        Transform3D {
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44,
            _unit: PhantomData,
        }
    }

    /// Create a transform representing a 2d transformation from the components
    /// of a 2 by 3 matrix transformation.
    ///
    /// Components follow the column-major-column-vector notation (m41 and m42
    /// representing the translation terms).
    ///
    /// ```text
    /// m11  m12   0   0
    /// m21  m22   0   0
    ///   0    0   1   0
    /// m41  m42   0   1
    /// ```
    #[inline]
    #[rustfmt::skip]
    pub fn new_2d(m11: T, m12: T, m21: T, m22: T, m41: T, m42: T) -> Self
    where
        T: Zero + One,
    {
        let _0 = || T::zero();
        let _1 = || T::one();

        Self::new(
            m11,  m12,  _0(), _0(),
            m21,  m22,  _0(), _0(),
            _0(), _0(), _1(), _0(),
            m41,  m42,  _0(), _1()
       )
    }

    /// Returns `true` if this transform can be represented with a `Transform2D`.
    ///
    /// See <https://drafts.csswg.org/css-transforms/#2d-transform>
    #[inline]
    pub fn is_2d(&self) -> bool
    where
        T: Zero + One + PartialEq,
    {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        self.m31 == _0
            && self.m32 == _0
            && self.m13 == _0
            && self.m23 == _0
            && self.m43 == _0
            && self.m14 == _0
            && self.m24 == _0
            && self.m34 == _0
            && self.m33 == _1
            && self.m44 == _1
    }
}

impl<T: Copy, Src, Dst> Transform3D<T, Src, Dst> {
    /// Returns an array containing this transform's terms.
    ///
    /// The terms are laid out in the same order as they are
    /// specified in `Transform3D::new`, that is following the
    /// column-major-column-vector matrix notation.
    ///
    /// For example the translation terms are found on the
    /// 13th, 14th and 15th slots of the array.
    #[inline]
    #[rustfmt::skip]
    pub fn to_array(&self) -> [T; 16] {
        [
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44
        ]
    }

    /// Returns an array containing this transform's terms transposed.
    ///
    /// The terms are laid out in transposed order from the same order of
    /// `Transform3D::new` and `Transform3D::to_array`, that is following
    /// the row-major-column-vector matrix notation.
    ///
    /// For example the translation terms are found at indices 3, 7 and 11
    /// of the array.
    #[inline]
    #[rustfmt::skip]
    pub fn to_array_transposed(&self) -> [T; 16] {
        [
            self.m11, self.m21, self.m31, self.m41,
            self.m12, self.m22, self.m32, self.m42,
            self.m13, self.m23, self.m33, self.m43,
            self.m14, self.m24, self.m34, self.m44
        ]
    }

    /// Equivalent to `to_array` with elements packed four at a time
    /// in an array of arrays.
    #[inline]
    #[rustfmt::skip]
    pub fn to_arrays(&self) -> [[T; 4]; 4] {
        [
            [self.m11, self.m12, self.m13, self.m14],
            [self.m21, self.m22, self.m23, self.m24],
            [self.m31, self.m32, self.m33, self.m34],
            [self.m41, self.m42, self.m43, self.m44],
        ]
    }

    /// Equivalent to `to_array_transposed` with elements packed
    /// four at a time in an array of arrays.
    #[inline]
    #[rustfmt::skip]
    pub fn to_arrays_transposed(&self) -> [[T; 4]; 4] {
        [
            [self.m11, self.m21, self.m31, self.m41],
            [self.m12, self.m22, self.m32, self.m42],
            [self.m13, self.m23, self.m33, self.m43],
            [self.m14, self.m24, self.m34, self.m44],
        ]
    }

    /// Create a transform providing its components via an array
    /// of 16 elements instead of as individual parameters.
    ///
    /// The order of the components corresponds to the
    /// column-major-column-vector matrix notation (the same order
    /// as `Transform3D::new`).
    #[inline]
    #[rustfmt::skip]
    pub fn from_array(array: [T; 16]) -> Self {
        Self::new(
            array[0],  array[1],  array[2],  array[3],
            array[4],  array[5],  array[6],  array[7],
            array[8],  array[9],  array[10], array[11],
            array[12], array[13], array[14], array[15],
        )
    }

    /// Equivalent to `from_array` with elements packed four at a time
    /// in an array of arrays.
    ///
    /// The order of the components corresponds to the
    /// column-major-column-vector matrix notation (the same order
    /// as `Transform3D::new`).
    #[inline]
    #[rustfmt::skip]
    pub fn from_arrays(array: [[T; 4]; 4]) -> Self {
        Self::new(
            array[0][0], array[0][1], array[0][2], array[0][3],
            array[1][0], array[1][1], array[1][2], array[1][3],
            array[2][0], array[2][1], array[2][2], array[2][3],
            array[3][0], array[3][1], array[3][2], array[3][3],
        )
    }

    /// Tag a unitless value with units.
    #[inline]
    #[rustfmt::skip]
    pub fn from_untyped(m: &Transform3D<T, UnknownUnit, UnknownUnit>) -> Self {
        Transform3D::new(
            m.m11, m.m12, m.m13, m.m14,
            m.m21, m.m22, m.m23, m.m24,
            m.m31, m.m32, m.m33, m.m34,
            m.m41, m.m42, m.m43, m.m44,
        )
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    #[rustfmt::skip]
    pub fn to_untyped(&self) -> Transform3D<T, UnknownUnit, UnknownUnit> {
        Transform3D::new(
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44,
        )
    }

    /// Returns the same transform with a different source unit.
    #[inline]
    #[rustfmt::skip]
    pub fn with_source<NewSrc>(&self) -> Transform3D<T, NewSrc, Dst> {
        Transform3D::new(
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44,
        )
    }

    /// Returns the same transform with a different destination unit.
    #[inline]
    #[rustfmt::skip]
    pub fn with_destination<NewDst>(&self) -> Transform3D<T, Src, NewDst> {
        Transform3D::new(
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44,
        )
    }

    /// Create a 2D transform picking the relevant terms from this transform.
    ///
    /// This method assumes that self represents a 2d transformation, callers
    /// should check that [`is_2d`] returns `true` beforehand.
    ///
    /// [`is_2d`]: Self::is_2d
    pub fn to_2d(&self) -> Transform2D<T, Src, Dst> {
        Transform2D::new(self.m11, self.m12, self.m21, self.m22, self.m41, self.m42)
    }
}

impl<T, Src, Dst> Transform3D<T, Src, Dst>
where
    T: Zero + One,
{
    /// Creates an identity matrix:
    ///
    /// ```text
    /// 1 0 0 0
    /// 0 1 0 0
    /// 0 0 1 0
    /// 0 0 0 1
    /// ```
    #[inline]
    pub fn identity() -> Self {
        Self::translation(T::zero(), T::zero(), T::zero())
    }

    /// Intentional not public, because it checks for exact equivalence
    /// while most consumers will probably want some sort of approximate
    /// equivalence to deal with floating-point errors.
    #[inline]
    fn is_identity(&self) -> bool
    where
        T: PartialEq,
    {
        *self == Self::identity()
    }

    /// Create a 2d skew transform.
    ///
    /// See <https://drafts.csswg.org/css-transforms/#funcdef-skew>
    #[rustfmt::skip]
    pub fn skew(alpha: Angle<T>, beta: Angle<T>) -> Self
    where
        T: Trig,
    {
        let _0 = || T::zero();
        let _1 = || T::one();
        let (sx, sy) = (beta.radians.tan(), alpha.radians.tan());

        Self::new(
            _1(), sx,   _0(), _0(),
            sy,   _1(), _0(), _0(),
            _0(), _0(), _1(), _0(),
            _0(), _0(), _0(), _1(),
        )
    }

    /// Create a simple perspective transform, projecting to the plane `z = -d`.
    ///
    /// ```text
    /// 1   0   0   0
    /// 0   1   0   0
    /// 0   0   1 -1/d
    /// 0   0   0   1
    /// ```
    ///
    /// See <https://drafts.csswg.org/css-transforms-2/#PerspectiveDefined>.
    pub fn perspective(d: T) -> Self
    where
        T: Neg<Output = T> + Div<Output = T>,
    {
        let _0 = || T::zero();
        let _1 = || T::one();

        Self::new(
            _1(),
            _0(),
            _0(),
            _0(),
            _0(),
            _1(),
            _0(),
            _0(),
            _0(),
            _0(),
            _1(),
            -_1() / d,
            _0(),
            _0(),
            _0(),
            _1(),
        )
    }
}

/// Methods for combining generic transformations
impl<T, Src, Dst> Transform3D<T, Src, Dst>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    /// Returns the multiplication of the two matrices such that mat's transformation
    /// applies after self's transformation.
    ///
    /// Assuming row vectors, this is equivalent to self * mat
    #[must_use]
    #[rustfmt::skip]
    pub fn then<NewDst>(&self, other: &Transform3D<T, Dst, NewDst>) -> Transform3D<T, Src, NewDst> {
        Transform3D::new(
            self.m11 * other.m11  +  self.m12 * other.m21  +  self.m13 * other.m31  +  self.m14 * other.m41,
            self.m11 * other.m12  +  self.m12 * other.m22  +  self.m13 * other.m32  +  self.m14 * other.m42,
            self.m11 * other.m13  +  self.m12 * other.m23  +  self.m13 * other.m33  +  self.m14 * other.m43,
            self.m11 * other.m14  +  self.m12 * other.m24  +  self.m13 * other.m34  +  self.m14 * other.m44,

            self.m21 * other.m11  +  self.m22 * other.m21  +  self.m23 * other.m31  +  self.m24 * other.m41,
            self.m21 * other.m12  +  self.m22 * other.m22  +  self.m23 * other.m32  +  self.m24 * other.m42,
            self.m21 * other.m13  +  self.m22 * other.m23  +  self.m23 * other.m33  +  self.m24 * other.m43,
            self.m21 * other.m14  +  self.m22 * other.m24  +  self.m23 * other.m34  +  self.m24 * other.m44,

            self.m31 * other.m11  +  self.m32 * other.m21  +  self.m33 * other.m31  +  self.m34 * other.m41,
            self.m31 * other.m12  +  self.m32 * other.m22  +  self.m33 * other.m32  +  self.m34 * other.m42,
            self.m31 * other.m13  +  self.m32 * other.m23  +  self.m33 * other.m33  +  self.m34 * other.m43,
            self.m31 * other.m14  +  self.m32 * other.m24  +  self.m33 * other.m34  +  self.m34 * other.m44,

            self.m41 * other.m11  +  self.m42 * other.m21  +  self.m43 * other.m31  +  self.m44 * other.m41,
            self.m41 * other.m12  +  self.m42 * other.m22  +  self.m43 * other.m32  +  self.m44 * other.m42,
            self.m41 * other.m13  +  self.m42 * other.m23  +  self.m43 * other.m33  +  self.m44 * other.m43,
            self.m41 * other.m14  +  self.m42 * other.m24  +  self.m43 * other.m34  +  self.m44 * other.m44,
        )
    }
}

/// Methods for creating and combining translation transformations
impl<T, Src, Dst> Transform3D<T, Src, Dst>
where
    T: Zero + One,
{
    /// Create a 3d translation transform:
    ///
    /// ```text
    /// 1 0 0 0
    /// 0 1 0 0
    /// 0 0 1 0
    /// x y z 1
    /// ```
    #[inline]
    #[rustfmt::skip]
    pub fn translation(x: T, y: T, z: T) -> Self {
        let _0 = || T::zero();
        let _1 = || T::one();

        Self::new(
            _1(), _0(), _0(), _0(),
            _0(), _1(), _0(), _0(),
            _0(), _0(), _1(), _0(),
             x,    y,    z,   _1(),
        )
    }

    /// Returns a transform with a translation applied before self's transformation.
    #[must_use]
    pub fn pre_translate(&self, v: Vector3D<T, Src>) -> Self
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        Transform3D::translation(v.x, v.y, v.z).then(self)
    }

    /// Returns a transform with a translation applied after self's transformation.
    #[must_use]
    pub fn then_translate(&self, v: Vector3D<T, Dst>) -> Self
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        self.then(&Transform3D::translation(v.x, v.y, v.z))
    }
}

/// Methods for creating and combining rotation transformations
impl<T, Src, Dst> Transform3D<T, Src, Dst>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Zero
        + One
        + Trig,
{
    /// Create a 3d rotation transform from an angle / axis.
    /// The supplied axis must be normalized.
    #[rustfmt::skip]
    pub fn rotation(x: T, y: T, z: T, theta: Angle<T>) -> Self {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let _2 = _1 + _1;

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        let half_theta = theta.get() / _2;
        let sc = half_theta.sin() * half_theta.cos();
        let sq = half_theta.sin() * half_theta.sin();

        Transform3D::new(
            _1 - _2 * (yy + zz) * sq,
            _2 * (x * y * sq + z * sc),
            _2 * (x * z * sq - y * sc),
            _0,


            _2 * (x * y * sq - z * sc),
            _1 - _2 * (xx + zz) * sq,
            _2 * (y * z * sq + x * sc),
            _0,

            _2 * (x * z * sq + y * sc),
            _2 * (y * z * sq - x * sc),
            _1 - _2 * (xx + yy) * sq,
            _0,

            _0,
            _0,
            _0,
            _1
        )
    }

    /// Returns a transform with a rotation applied after self's transformation.
    #[must_use]
    pub fn then_rotate(&self, x: T, y: T, z: T, theta: Angle<T>) -> Self {
        self.then(&Transform3D::rotation(x, y, z, theta))
    }

    /// Returns a transform with a rotation applied before self's transformation.
    #[must_use]
    pub fn pre_rotate(&self, x: T, y: T, z: T, theta: Angle<T>) -> Self {
        Transform3D::rotation(x, y, z, theta).then(self)
    }
}

/// Methods for creating and combining scale transformations
impl<T, Src, Dst> Transform3D<T, Src, Dst>
where
    T: Zero + One,
{
    /// Create a 3d scale transform:
    ///
    /// ```text
    /// x 0 0 0
    /// 0 y 0 0
    /// 0 0 z 0
    /// 0 0 0 1
    /// ```
    #[inline]
    #[rustfmt::skip]
    pub fn scale(x: T, y: T, z: T) -> Self {
        let _0 = || T::zero();
        let _1 = || T::one();

        Self::new(
             x,   _0(), _0(), _0(),
            _0(),  y,   _0(), _0(),
            _0(), _0(),  z,   _0(),
            _0(), _0(), _0(), _1(),
        )
    }

    /// Returns a transform with a scale applied before self's transformation.
    #[must_use]
    #[rustfmt::skip]
    pub fn pre_scale(&self, x: T, y: T, z: T) -> Self
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        Transform3D::new(
            self.m11 * x, self.m12 * x, self.m13 * x, self.m14 * x,
            self.m21 * y, self.m22 * y, self.m23 * y, self.m24 * y,
            self.m31 * z, self.m32 * z, self.m33 * z, self.m34 * z,
            self.m41    , self.m42,     self.m43,     self.m44
        )
    }

    /// Returns a transform with a scale applied after self's transformation.
    #[must_use]
    pub fn then_scale(&self, x: T, y: T, z: T) -> Self
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        self.then(&Transform3D::scale(x, y, z))
    }
}

/// Methods for apply transformations to objects
impl<T, Src, Dst> Transform3D<T, Src, Dst>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    /// Returns the homogeneous vector corresponding to the transformed 2d point.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    #[rustfmt::skip]
    pub fn transform_point2d_homogeneous(
        &self, p: Point2D<T, Src>
    ) -> HomogeneousVector<T, Dst> {
        let x = p.x * self.m11 + p.y * self.m21 + self.m41;
        let y = p.x * self.m12 + p.y * self.m22 + self.m42;
        let z = p.x * self.m13 + p.y * self.m23 + self.m43;
        let w = p.x * self.m14 + p.y * self.m24 + self.m44;

        HomogeneousVector::new(x, y, z, w)
    }

    /// Returns the given 2d point transformed by this transform, if the transform makes sense,
    /// or `None` otherwise.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_point2d(&self, p: Point2D<T, Src>) -> Option<Point2D<T, Dst>>
    where
        T: Div<Output = T> + Zero + PartialOrd,
    {
        //Note: could use `transform_point2d_homogeneous()` but it would waste the calculus of `z`
        let w = p.x * self.m14 + p.y * self.m24 + self.m44;
        if w > T::zero() {
            let x = p.x * self.m11 + p.y * self.m21 + self.m41;
            let y = p.x * self.m12 + p.y * self.m22 + self.m42;

            Some(Point2D::new(x / w, y / w))
        } else {
            None
        }
    }

    /// Returns the given 2d vector transformed by this matrix.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_vector2d(&self, v: Vector2D<T, Src>) -> Vector2D<T, Dst> {
        vec2(
            v.x * self.m11 + v.y * self.m21,
            v.x * self.m12 + v.y * self.m22,
        )
    }

    /// Returns the homogeneous vector corresponding to the transformed 3d point.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_point3d_homogeneous(&self, p: Point3D<T, Src>) -> HomogeneousVector<T, Dst> {
        let x = p.x * self.m11 + p.y * self.m21 + p.z * self.m31 + self.m41;
        let y = p.x * self.m12 + p.y * self.m22 + p.z * self.m32 + self.m42;
        let z = p.x * self.m13 + p.y * self.m23 + p.z * self.m33 + self.m43;
        let w = p.x * self.m14 + p.y * self.m24 + p.z * self.m34 + self.m44;

        HomogeneousVector::new(x, y, z, w)
    }

    /// Returns the given 3d point transformed by this transform, if the transform makes sense,
    /// or `None` otherwise.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_point3d(&self, p: Point3D<T, Src>) -> Option<Point3D<T, Dst>>
    where
        T: Div<Output = T> + Zero + PartialOrd,
    {
        self.transform_point3d_homogeneous(p).to_point3d()
    }

    /// Returns the given 3d vector transformed by this matrix.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_vector3d(&self, v: Vector3D<T, Src>) -> Vector3D<T, Dst> {
        vec3(
            v.x * self.m11 + v.y * self.m21 + v.z * self.m31,
            v.x * self.m12 + v.y * self.m22 + v.z * self.m32,
            v.x * self.m13 + v.y * self.m23 + v.z * self.m33,
        )
    }

    /// Returns a rectangle that encompasses the result of transforming the given rectangle by this
    /// transform, if the transform makes sense for it, or `None` otherwise.
    pub fn outer_transformed_rect(&self, rect: &Rect<T, Src>) -> Option<Rect<T, Dst>>
    where
        T: Sub<Output = T> + Div<Output = T> + Zero + PartialOrd,
    {
        let min = rect.min();
        let max = rect.max();
        Some(Rect::from_points(&[
            self.transform_point2d(min)?,
            self.transform_point2d(max)?,
            self.transform_point2d(point2(max.x, min.y))?,
            self.transform_point2d(point2(min.x, max.y))?,
        ]))
    }

    /// Returns a 2d box that encompasses the result of transforming the given box by this
    /// transform, if the transform makes sense for it, or `None` otherwise.
    pub fn outer_transformed_box2d(&self, b: &Box2D<T, Src>) -> Option<Box2D<T, Dst>>
    where
        T: Sub<Output = T> + Div<Output = T> + Zero + PartialOrd,
    {
        Some(Box2D::from_points(&[
            self.transform_point2d(b.min)?,
            self.transform_point2d(b.max)?,
            self.transform_point2d(point2(b.max.x, b.min.y))?,
            self.transform_point2d(point2(b.min.x, b.max.y))?,
        ]))
    }

    /// Returns a 3d box that encompasses the result of transforming the given box by this
    /// transform, if the transform makes sense for it, or `None` otherwise.
    pub fn outer_transformed_box3d(&self, b: &Box3D<T, Src>) -> Option<Box3D<T, Dst>>
    where
        T: Sub<Output = T> + Div<Output = T> + Zero + PartialOrd,
    {
        Some(Box3D::from_points(&[
            self.transform_point3d(point3(b.min.x, b.min.y, b.min.z))?,
            self.transform_point3d(point3(b.min.x, b.min.y, b.max.z))?,
            self.transform_point3d(point3(b.min.x, b.max.y, b.min.z))?,
            self.transform_point3d(point3(b.min.x, b.max.y, b.max.z))?,
            self.transform_point3d(point3(b.max.x, b.min.y, b.min.z))?,
            self.transform_point3d(point3(b.max.x, b.min.y, b.max.z))?,
            self.transform_point3d(point3(b.max.x, b.max.y, b.min.z))?,
            self.transform_point3d(point3(b.max.x, b.max.y, b.max.z))?,
        ]))
    }
}

impl<T, Src, Dst> Transform3D<T, Src, Dst>
where
    T: Copy
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Neg<Output = T>
        + PartialOrd
        + One
        + Zero,
{
    /// Create an orthogonal projection transform.
    #[rustfmt::skip]
    pub fn ortho(left: T, right: T,
                 bottom: T, top: T,
                 near: T, far: T) -> Self {
        let tx = -((right + left) / (right - left));
        let ty = -((top + bottom) / (top - bottom));
        let tz = -((far + near) / (far - near));

        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let _2 = _1 + _1;
        Transform3D::new(
            _2 / (right - left), _0                 , _0                , _0,
            _0                 , _2 / (top - bottom), _0                , _0,
            _0                 , _0                 , -_2 / (far - near), _0,
            tx                 , ty                 , tz                , _1
        )
    }

    /// Check whether shapes on the XY plane with Z pointing towards the
    /// screen transformed by this matrix would be facing back.
    #[rustfmt::skip]
    pub fn is_backface_visible(&self) -> bool {
        // inverse().m33 < 0;
        let det = self.determinant();
        let m33 = self.m12 * self.m24 * self.m41 - self.m14 * self.m22 * self.m41 +
                  self.m14 * self.m21 * self.m42 - self.m11 * self.m24 * self.m42 -
                  self.m12 * self.m21 * self.m44 + self.m11 * self.m22 * self.m44;
        let _0: T = Zero::zero();
        (m33 * det) < _0
    }

    /// Returns whether it is possible to compute the inverse transform.
    #[inline]
    pub fn is_invertible(&self) -> bool {
        self.determinant() != Zero::zero()
    }

    /// Returns the inverse transform if possible.
    pub fn inverse(&self) -> Option<Transform3D<T, Dst, Src>> {
        let det = self.determinant();

        if det == Zero::zero() {
            return None;
        }

        // todo(gw): this could be made faster by special casing
        // for simpler transform types.
        #[rustfmt::skip]
        let m = Transform3D::new(
            self.m23*self.m34*self.m42 - self.m24*self.m33*self.m42 +
            self.m24*self.m32*self.m43 - self.m22*self.m34*self.m43 -
            self.m23*self.m32*self.m44 + self.m22*self.m33*self.m44,

            self.m14*self.m33*self.m42 - self.m13*self.m34*self.m42 -
            self.m14*self.m32*self.m43 + self.m12*self.m34*self.m43 +
            self.m13*self.m32*self.m44 - self.m12*self.m33*self.m44,

            self.m13*self.m24*self.m42 - self.m14*self.m23*self.m42 +
            self.m14*self.m22*self.m43 - self.m12*self.m24*self.m43 -
            self.m13*self.m22*self.m44 + self.m12*self.m23*self.m44,

            self.m14*self.m23*self.m32 - self.m13*self.m24*self.m32 -
            self.m14*self.m22*self.m33 + self.m12*self.m24*self.m33 +
            self.m13*self.m22*self.m34 - self.m12*self.m23*self.m34,

            self.m24*self.m33*self.m41 - self.m23*self.m34*self.m41 -
            self.m24*self.m31*self.m43 + self.m21*self.m34*self.m43 +
            self.m23*self.m31*self.m44 - self.m21*self.m33*self.m44,

            self.m13*self.m34*self.m41 - self.m14*self.m33*self.m41 +
            self.m14*self.m31*self.m43 - self.m11*self.m34*self.m43 -
            self.m13*self.m31*self.m44 + self.m11*self.m33*self.m44,

            self.m14*self.m23*self.m41 - self.m13*self.m24*self.m41 -
            self.m14*self.m21*self.m43 + self.m11*self.m24*self.m43 +
            self.m13*self.m21*self.m44 - self.m11*self.m23*self.m44,

            self.m13*self.m24*self.m31 - self.m14*self.m23*self.m31 +
            self.m14*self.m21*self.m33 - self.m11*self.m24*self.m33 -
            self.m13*self.m21*self.m34 + self.m11*self.m23*self.m34,

            self.m22*self.m34*self.m41 - self.m24*self.m32*self.m41 +
            self.m24*self.m31*self.m42 - self.m21*self.m34*self.m42 -
            self.m22*self.m31*self.m44 + self.m21*self.m32*self.m44,

            self.m14*self.m32*self.m41 - self.m12*self.m34*self.m41 -
            self.m14*self.m31*self.m42 + self.m11*self.m34*self.m42 +
            self.m12*self.m31*self.m44 - self.m11*self.m32*self.m44,

            self.m12*self.m24*self.m41 - self.m14*self.m22*self.m41 +
            self.m14*self.m21*self.m42 - self.m11*self.m24*self.m42 -
            self.m12*self.m21*self.m44 + self.m11*self.m22*self.m44,

            self.m14*self.m22*self.m31 - self.m12*self.m24*self.m31 -
            self.m14*self.m21*self.m32 + self.m11*self.m24*self.m32 +
            self.m12*self.m21*self.m34 - self.m11*self.m22*self.m34,

            self.m23*self.m32*self.m41 - self.m22*self.m33*self.m41 -
            self.m23*self.m31*self.m42 + self.m21*self.m33*self.m42 +
            self.m22*self.m31*self.m43 - self.m21*self.m32*self.m43,

            self.m12*self.m33*self.m41 - self.m13*self.m32*self.m41 +
            self.m13*self.m31*self.m42 - self.m11*self.m33*self.m42 -
            self.m12*self.m31*self.m43 + self.m11*self.m32*self.m43,

            self.m13*self.m22*self.m41 - self.m12*self.m23*self.m41 -
            self.m13*self.m21*self.m42 + self.m11*self.m23*self.m42 +
            self.m12*self.m21*self.m43 - self.m11*self.m22*self.m43,

            self.m12*self.m23*self.m31 - self.m13*self.m22*self.m31 +
            self.m13*self.m21*self.m32 - self.m11*self.m23*self.m32 -
            self.m12*self.m21*self.m33 + self.m11*self.m22*self.m33
        );

        let _1: T = One::one();
        Some(m.mul_s(_1 / det))
    }

    /// Compute the determinant of the transform.
    #[rustfmt::skip]
    pub fn determinant(&self) -> T {
        self.m14 * self.m23 * self.m32 * self.m41 -
        self.m13 * self.m24 * self.m32 * self.m41 -
        self.m14 * self.m22 * self.m33 * self.m41 +
        self.m12 * self.m24 * self.m33 * self.m41 +
        self.m13 * self.m22 * self.m34 * self.m41 -
        self.m12 * self.m23 * self.m34 * self.m41 -
        self.m14 * self.m23 * self.m31 * self.m42 +
        self.m13 * self.m24 * self.m31 * self.m42 +
        self.m14 * self.m21 * self.m33 * self.m42 -
        self.m11 * self.m24 * self.m33 * self.m42 -
        self.m13 * self.m21 * self.m34 * self.m42 +
        self.m11 * self.m23 * self.m34 * self.m42 +
        self.m14 * self.m22 * self.m31 * self.m43 -
        self.m12 * self.m24 * self.m31 * self.m43 -
        self.m14 * self.m21 * self.m32 * self.m43 +
        self.m11 * self.m24 * self.m32 * self.m43 +
        self.m12 * self.m21 * self.m34 * self.m43 -
        self.m11 * self.m22 * self.m34 * self.m43 -
        self.m13 * self.m22 * self.m31 * self.m44 +
        self.m12 * self.m23 * self.m31 * self.m44 +
        self.m13 * self.m21 * self.m32 * self.m44 -
        self.m11 * self.m23 * self.m32 * self.m44 -
        self.m12 * self.m21 * self.m33 * self.m44 +
        self.m11 * self.m22 * self.m33 * self.m44
    }

    /// Multiplies all of the transform's component by a scalar and returns the result.
    #[must_use]
    #[rustfmt::skip]
    pub fn mul_s(&self, x: T) -> Self {
        Transform3D::new(
            self.m11 * x, self.m12 * x, self.m13 * x, self.m14 * x,
            self.m21 * x, self.m22 * x, self.m23 * x, self.m24 * x,
            self.m31 * x, self.m32 * x, self.m33 * x, self.m34 * x,
            self.m41 * x, self.m42 * x, self.m43 * x, self.m44 * x
        )
    }

    /// Convenience function to create a scale transform from a `Scale`.
    pub fn from_scale(scale: Scale<T, Src, Dst>) -> Self {
        Transform3D::scale(scale.get(), scale.get(), scale.get())
    }
}

impl<T, Src, Dst> Transform3D<T, Src, Dst>
where
    T: Copy + Mul<Output = T> + Div<Output = T> + Zero + One + PartialEq,
{
    /// Returns a projection of this transform in 2d space.
    pub fn project_to_2d(&self) -> Self {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());

        let mut result = self.clone();

        result.m31 = _0;
        result.m32 = _0;
        result.m13 = _0;
        result.m23 = _0;
        result.m33 = _1;
        result.m43 = _0;
        result.m34 = _0;

        // Try to normalize perspective when possible to convert to a 2d matrix.
        // Some matrices, such as those derived from perspective transforms, can
        // modify m44 from 1, while leaving the rest of the fourth column
        // (m14, m24) at 0. In this case, after resetting the third row and
        // third column above, the value of m44 functions only to scale the
        // coordinate transform divide by W. The matrix can be converted to
        // a true 2D matrix by normalizing out the scaling effect of m44 on
        // the remaining components ahead of time.
        if self.m14 == _0 && self.m24 == _0 && self.m44 != _0 && self.m44 != _1 {
            let scale = _1 / self.m44;
            result.m11 = result.m11 * scale;
            result.m12 = result.m12 * scale;
            result.m21 = result.m21 * scale;
            result.m22 = result.m22 * scale;
            result.m41 = result.m41 * scale;
            result.m42 = result.m42 * scale;
            result.m44 = _1;
        }

        result
    }
}

impl<T: NumCast + Copy, Src, Dst> Transform3D<T, Src, Dst> {
    /// Cast from one numeric representation to another, preserving the units.
    #[inline]
    pub fn cast<NewT: NumCast>(&self) -> Transform3D<NewT, Src, Dst> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    #[rustfmt::skip]
    pub fn try_cast<NewT: NumCast>(&self) -> Option<Transform3D<NewT, Src, Dst>> {
        match (NumCast::from(self.m11), NumCast::from(self.m12),
               NumCast::from(self.m13), NumCast::from(self.m14),
               NumCast::from(self.m21), NumCast::from(self.m22),
               NumCast::from(self.m23), NumCast::from(self.m24),
               NumCast::from(self.m31), NumCast::from(self.m32),
               NumCast::from(self.m33), NumCast::from(self.m34),
               NumCast::from(self.m41), NumCast::from(self.m42),
               NumCast::from(self.m43), NumCast::from(self.m44)) {
            (Some(m11), Some(m12), Some(m13), Some(m14),
             Some(m21), Some(m22), Some(m23), Some(m24),
             Some(m31), Some(m32), Some(m33), Some(m34),
             Some(m41), Some(m42), Some(m43), Some(m44)) => {
                Some(Transform3D::new(m11, m12, m13, m14,
                                      m21, m22, m23, m24,
                                      m31, m32, m33, m34,
                                      m41, m42, m43, m44))
            },
            _ => None
        }
    }
}

impl<T: ApproxEq<T>, Src, Dst> Transform3D<T, Src, Dst> {
    /// Returns `true` if this transform is approximately equal to the other one, using
    /// `T`'s default epsilon value.
    ///
    /// The same as [`ApproxEq::approx_eq`] but available without importing trait.
    #[inline]
    pub fn approx_eq(&self, other: &Self) -> bool {
        <Self as ApproxEq<T>>::approx_eq(&self, &other)
    }

    /// Returns `true` if this transform is approximately equal to the other one, using
    /// a provided epsilon value.
    ///
    /// The same as [`ApproxEq::approx_eq_eps`] but available without importing trait.
    #[inline]
    pub fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        <Self as ApproxEq<T>>::approx_eq_eps(&self, &other, &eps)
    }
}

impl<T: ApproxEq<T>, Src, Dst> ApproxEq<T> for Transform3D<T, Src, Dst> {
    #[inline]
    fn approx_epsilon() -> T {
        T::approx_epsilon()
    }

    #[rustfmt::skip]
    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        self.m11.approx_eq_eps(&other.m11, eps) && self.m12.approx_eq_eps(&other.m12, eps) &&
        self.m13.approx_eq_eps(&other.m13, eps) && self.m14.approx_eq_eps(&other.m14, eps) &&
        self.m21.approx_eq_eps(&other.m21, eps) && self.m22.approx_eq_eps(&other.m22, eps) &&
        self.m23.approx_eq_eps(&other.m23, eps) && self.m24.approx_eq_eps(&other.m24, eps) &&
        self.m31.approx_eq_eps(&other.m31, eps) && self.m32.approx_eq_eps(&other.m32, eps) &&
        self.m33.approx_eq_eps(&other.m33, eps) && self.m34.approx_eq_eps(&other.m34, eps) &&
        self.m41.approx_eq_eps(&other.m41, eps) && self.m42.approx_eq_eps(&other.m42, eps) &&
        self.m43.approx_eq_eps(&other.m43, eps) && self.m44.approx_eq_eps(&other.m44, eps)
    }
}

impl<T, Src, Dst> Default for Transform3D<T, Src, Dst>
where
    T: Zero + One,
{
    /// Returns the [identity transform](Self::identity).
    fn default() -> Self {
        Self::identity()
    }
}

impl<T, Src, Dst> fmt::Debug for Transform3D<T, Src, Dst>
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
impl<T, Src, Dst> From<mint::RowMatrix4<T>> for Transform3D<T, Src, Dst> {
    #[rustfmt::skip]
    fn from(m: mint::RowMatrix4<T>) -> Self {
        Transform3D {
            m11: m.x.x, m12: m.x.y, m13: m.x.z, m14: m.x.w,
            m21: m.y.x, m22: m.y.y, m23: m.y.z, m24: m.y.w,
            m31: m.z.x, m32: m.z.y, m33: m.z.z, m34: m.z.w,
            m41: m.w.x, m42: m.w.y, m43: m.w.z, m44: m.w.w,
            _unit: PhantomData,
        }
    }
}
#[cfg(feature = "mint")]
impl<T, Src, Dst> From<Transform3D<T, Src, Dst>> for mint::RowMatrix4<T> {
    #[rustfmt::skip]
    fn from(t: Transform3D<T, Src, Dst>) -> Self {
        mint::RowMatrix4 {
            x: mint::Vector4 { x: t.m11, y: t.m12, z: t.m13, w: t.m14 },
            y: mint::Vector4 { x: t.m21, y: t.m22, z: t.m23, w: t.m24 },
            z: mint::Vector4 { x: t.m31, y: t.m32, z: t.m33, w: t.m34 },
            w: mint::Vector4 { x: t.m41, y: t.m42, z: t.m43, w: t.m44 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approxeq::ApproxEq;
    use crate::default;
    use crate::{point2, point3};

    use core::f32::consts::{FRAC_PI_2, PI};

    type Mf32 = default::Transform3D<f32>;

    // For convenience.
    fn rad(v: f32) -> Angle<f32> {
        Angle::radians(v)
    }

    #[test]
    pub fn test_translation() {
        let t1 = Mf32::translation(1.0, 2.0, 3.0);
        let t2 = Mf32::identity().pre_translate(vec3(1.0, 2.0, 3.0));
        let t3 = Mf32::identity().then_translate(vec3(1.0, 2.0, 3.0));
        assert_eq!(t1, t2);
        assert_eq!(t1, t3);

        assert_eq!(
            t1.transform_point3d(point3(1.0, 1.0, 1.0)),
            Some(point3(2.0, 3.0, 4.0))
        );
        assert_eq!(
            t1.transform_point2d(point2(1.0, 1.0)),
            Some(point2(2.0, 3.0))
        );

        assert_eq!(t1.then(&t1), Mf32::translation(2.0, 4.0, 6.0));

        assert!(!t1.is_2d());
        assert_eq!(
            Mf32::translation(1.0, 2.0, 3.0).to_2d(),
            Transform2D::translation(1.0, 2.0)
        );
    }

    #[test]
    pub fn test_rotation() {
        let r1 = Mf32::rotation(0.0, 0.0, 1.0, rad(FRAC_PI_2));
        let r2 = Mf32::identity().pre_rotate(0.0, 0.0, 1.0, rad(FRAC_PI_2));
        let r3 = Mf32::identity().then_rotate(0.0, 0.0, 1.0, rad(FRAC_PI_2));
        assert_eq!(r1, r2);
        assert_eq!(r1, r3);

        assert!(r1
            .transform_point3d(point3(1.0, 2.0, 3.0))
            .unwrap()
            .approx_eq(&point3(-2.0, 1.0, 3.0)));
        assert!(r1
            .transform_point2d(point2(1.0, 2.0))
            .unwrap()
            .approx_eq(&point2(-2.0, 1.0)));

        assert!(r1
            .then(&r1)
            .approx_eq(&Mf32::rotation(0.0, 0.0, 1.0, rad(FRAC_PI_2 * 2.0))));

        assert!(r1.is_2d());
        assert!(r1.to_2d().approx_eq(&Transform2D::rotation(rad(FRAC_PI_2))));
    }

    #[test]
    pub fn test_scale() {
        let s1 = Mf32::scale(2.0, 3.0, 4.0);
        let s2 = Mf32::identity().pre_scale(2.0, 3.0, 4.0);
        let s3 = Mf32::identity().then_scale(2.0, 3.0, 4.0);
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);

        assert!(s1
            .transform_point3d(point3(2.0, 2.0, 2.0))
            .unwrap()
            .approx_eq(&point3(4.0, 6.0, 8.0)));
        assert!(s1
            .transform_point2d(point2(2.0, 2.0))
            .unwrap()
            .approx_eq(&point2(4.0, 6.0)));

        assert_eq!(s1.then(&s1), Mf32::scale(4.0, 9.0, 16.0));

        assert!(!s1.is_2d());
        assert_eq!(
            Mf32::scale(2.0, 3.0, 0.0).to_2d(),
            Transform2D::scale(2.0, 3.0)
        );
    }

    #[test]
    pub fn test_pre_then_scale() {
        let m = Mf32::rotation(0.0, 0.0, 1.0, rad(FRAC_PI_2)).then_translate(vec3(6.0, 7.0, 8.0));
        let s = Mf32::scale(2.0, 3.0, 4.0);
        assert_eq!(m.then(&s), m.then_scale(2.0, 3.0, 4.0));
    }

    #[test]
    #[rustfmt::skip]
    pub fn test_ortho() {
        let (left, right, bottom, top) = (0.0f32, 1.0f32, 0.1f32, 1.0f32);
        let (near, far) = (-1.0f32, 1.0f32);
        let result = Mf32::ortho(left, right, bottom, top, near, far);
        let expected = Mf32::new(
             2.0,  0.0,         0.0, 0.0,
             0.0,  2.22222222,  0.0, 0.0,
             0.0,  0.0,        -1.0, 0.0,
            -1.0, -1.22222222, -0.0, 1.0
        );
        assert!(result.approx_eq(&expected));
    }

    #[test]
    pub fn test_is_2d() {
        assert!(Mf32::identity().is_2d());
        assert!(Mf32::rotation(0.0, 0.0, 1.0, rad(0.7854)).is_2d());
        assert!(!Mf32::rotation(0.0, 1.0, 0.0, rad(0.7854)).is_2d());
    }

    #[test]
    #[rustfmt::skip]
    pub fn test_new_2d() {
        let m1 = Mf32::new_2d(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let m2 = Mf32::new(
            1.0, 2.0, 0.0, 0.0,
            3.0, 4.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            5.0, 6.0, 0.0, 1.0
        );
        assert_eq!(m1, m2);
    }

    #[test]
    pub fn test_inverse_simple() {
        let m1 = Mf32::identity();
        let m2 = m1.inverse().unwrap();
        assert!(m1.approx_eq(&m2));
    }

    #[test]
    pub fn test_inverse_scale() {
        let m1 = Mf32::scale(1.5, 0.3, 2.1);
        let m2 = m1.inverse().unwrap();
        assert!(m1.then(&m2).approx_eq(&Mf32::identity()));
        assert!(m2.then(&m1).approx_eq(&Mf32::identity()));
    }

    #[test]
    pub fn test_inverse_translate() {
        let m1 = Mf32::translation(-132.0, 0.3, 493.0);
        let m2 = m1.inverse().unwrap();
        assert!(m1.then(&m2).approx_eq(&Mf32::identity()));
        assert!(m2.then(&m1).approx_eq(&Mf32::identity()));
    }

    #[test]
    pub fn test_inverse_rotate() {
        let m1 = Mf32::rotation(0.0, 1.0, 0.0, rad(1.57));
        let m2 = m1.inverse().unwrap();
        assert!(m1.then(&m2).approx_eq(&Mf32::identity()));
        assert!(m2.then(&m1).approx_eq(&Mf32::identity()));
    }

    #[test]
    pub fn test_inverse_transform_point_2d() {
        let m1 = Mf32::translation(100.0, 200.0, 0.0);
        let m2 = m1.inverse().unwrap();
        assert!(m1.then(&m2).approx_eq(&Mf32::identity()));
        assert!(m2.then(&m1).approx_eq(&Mf32::identity()));

        let p1 = point2(1000.0, 2000.0);
        let p2 = m1.transform_point2d(p1);
        assert_eq!(p2, Some(point2(1100.0, 2200.0)));

        let p3 = m2.transform_point2d(p2.unwrap());
        assert_eq!(p3, Some(p1));
    }

    #[test]
    fn test_inverse_none() {
        assert!(Mf32::scale(2.0, 0.0, 2.0).inverse().is_none());
        assert!(Mf32::scale(2.0, 2.0, 2.0).inverse().is_some());
    }

    #[test]
    pub fn test_pre_post() {
        let m1 = default::Transform3D::identity()
            .then_scale(1.0, 2.0, 3.0)
            .then_translate(vec3(1.0, 2.0, 3.0));
        let m2 = default::Transform3D::identity()
            .pre_translate(vec3(1.0, 2.0, 3.0))
            .pre_scale(1.0, 2.0, 3.0);
        assert!(m1.approx_eq(&m2));

        let r = Mf32::rotation(0.0, 0.0, 1.0, rad(FRAC_PI_2));
        let t = Mf32::translation(2.0, 3.0, 0.0);

        let a = point3(1.0, 1.0, 1.0);

        assert!(r
            .then(&t)
            .transform_point3d(a)
            .unwrap()
            .approx_eq(&point3(1.0, 4.0, 1.0)));
        assert!(t
            .then(&r)
            .transform_point3d(a)
            .unwrap()
            .approx_eq(&point3(-4.0, 3.0, 1.0)));
        assert!(t.then(&r).transform_point3d(a).unwrap().approx_eq(
            &r.transform_point3d(t.transform_point3d(a).unwrap())
                .unwrap()
        ));
    }

    #[test]
    fn test_size_of() {
        use core::mem::size_of;
        assert_eq!(
            size_of::<default::Transform3D<f32>>(),
            16 * size_of::<f32>()
        );
        assert_eq!(
            size_of::<default::Transform3D<f64>>(),
            16 * size_of::<f64>()
        );
    }

    #[test]
    #[rustfmt::skip]
    pub fn test_transform_associativity() {
        let m1 = Mf32::new(3.0, 2.0, 1.5, 1.0,
                           0.0, 4.5, -1.0, -4.0,
                           0.0, 3.5, 2.5, 40.0,
                           0.0, 3.0, 0.0, 1.0);
        let m2 = Mf32::new(1.0, -1.0, 3.0, 0.0,
                           -1.0, 0.5, 0.0, 2.0,
                           1.5, -2.0, 6.0, 0.0,
                           -2.5, 6.0, 1.0, 1.0);

        let p = point3(1.0, 3.0, 5.0);
        let p1 = m1.then(&m2).transform_point3d(p).unwrap();
        let p2 = m2.transform_point3d(m1.transform_point3d(p).unwrap()).unwrap();
        assert!(p1.approx_eq(&p2));
    }

    #[test]
    pub fn test_is_identity() {
        let m1 = default::Transform3D::identity();
        assert!(m1.is_identity());
        let m2 = m1.then_translate(vec3(0.1, 0.0, 0.0));
        assert!(!m2.is_identity());
    }

    #[test]
    pub fn test_transform_vector() {
        // Translation does not apply to vectors.
        let m = Mf32::translation(1.0, 2.0, 3.0);
        let v1 = vec3(10.0, -10.0, 3.0);
        assert_eq!(v1, m.transform_vector3d(v1));
        // While it does apply to points.
        assert_ne!(Some(v1.to_point()), m.transform_point3d(v1.to_point()));

        // same thing with 2d vectors/points
        let v2 = vec2(10.0, -5.0);
        assert_eq!(v2, m.transform_vector2d(v2));
        assert_ne!(Some(v2.to_point()), m.transform_point2d(v2.to_point()));
    }

    #[test]
    pub fn test_is_backface_visible() {
        // backface is not visible for rotate-x 0 degree.
        let r1 = Mf32::rotation(1.0, 0.0, 0.0, rad(0.0));
        assert!(!r1.is_backface_visible());
        // backface is not visible for rotate-x 45 degree.
        let r1 = Mf32::rotation(1.0, 0.0, 0.0, rad(PI * 0.25));
        assert!(!r1.is_backface_visible());
        // backface is visible for rotate-x 180 degree.
        let r1 = Mf32::rotation(1.0, 0.0, 0.0, rad(PI));
        assert!(r1.is_backface_visible());
        // backface is visible for rotate-x 225 degree.
        let r1 = Mf32::rotation(1.0, 0.0, 0.0, rad(PI * 1.25));
        assert!(r1.is_backface_visible());
        // backface is not visible for non-inverseable matrix
        let r1 = Mf32::scale(2.0, 0.0, 2.0);
        assert!(!r1.is_backface_visible());
    }

    #[test]
    pub fn test_homogeneous() {
        #[rustfmt::skip]
        let m = Mf32::new(
            1.0, 2.0, 0.5, 5.0,
            3.0, 4.0, 0.25, 6.0,
            0.5, -1.0, 1.0, -1.0,
            -1.0, 1.0, -1.0, 2.0,
        );
        assert_eq!(
            m.transform_point2d_homogeneous(point2(1.0, 2.0)),
            HomogeneousVector::new(6.0, 11.0, 0.0, 19.0),
        );
        assert_eq!(
            m.transform_point3d_homogeneous(point3(1.0, 2.0, 4.0)),
            HomogeneousVector::new(8.0, 7.0, 4.0, 15.0),
        );
    }

    #[test]
    pub fn test_perspective_division() {
        let p = point2(1.0, 2.0);
        let mut m = Mf32::identity();
        assert!(m.transform_point2d(p).is_some());
        m.m44 = 0.0;
        assert_eq!(None, m.transform_point2d(p));
        m.m44 = 1.0;
        m.m24 = -1.0;
        assert_eq!(None, m.transform_point2d(p));
    }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let m1 = Mf32::rotation(0.0, 0.0, 1.0, rad(FRAC_PI_2));
        let mm: mint::RowMatrix4<_> = m1.into();
        let m2 = Mf32::from(mm);

        assert_eq!(m1, m2);
    }
}
