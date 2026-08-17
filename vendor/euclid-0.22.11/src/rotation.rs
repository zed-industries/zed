// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::approxeq::ApproxEq;
use crate::trig::Trig;
use crate::{point2, point3, vec3, Angle, Point2D, Point3D, Vector2D, Vector3D};
use crate::{Transform2D, Transform3D, UnknownUnit};

use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::{Add, Mul, Neg, Sub};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
use num_traits::real::Real;
use num_traits::{NumCast, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A transform that can represent rotations in 2d, represented as an angle in radians.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "T: serde::Serialize",
        deserialize = "T: serde::Deserialize<'de>"
    ))
)]
pub struct Rotation2D<T, Src, Dst> {
    /// Angle in radians
    pub angle: T,
    #[doc(hidden)]
    pub _unit: PhantomData<(Src, Dst)>,
}

impl<T: Copy, Src, Dst> Copy for Rotation2D<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for Rotation2D<T, Src, Dst> {
    fn clone(&self) -> Self {
        Rotation2D {
            angle: self.angle.clone(),
            _unit: PhantomData,
        }
    }
}

impl<T, Src, Dst> Eq for Rotation2D<T, Src, Dst> where T: Eq {}

impl<T, Src, Dst> PartialEq for Rotation2D<T, Src, Dst>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.angle == other.angle
    }
}

impl<T, Src, Dst> Hash for Rotation2D<T, Src, Dst>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.angle.hash(h);
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T, Src, Dst> arbitrary::Arbitrary<'a> for Rotation2D<T, Src, Dst>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Rotation2D::new(arbitrary::Arbitrary::arbitrary(u)?))
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, Src, Dst> Zeroable for Rotation2D<T, Src, Dst> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, Src: 'static, Dst: 'static> Pod for Rotation2D<T, Src, Dst> {}

impl<T, Src, Dst> Rotation2D<T, Src, Dst> {
    /// Creates a rotation from an angle in radians.
    #[inline]
    pub fn new(angle: Angle<T>) -> Self {
        Rotation2D {
            angle: angle.radians,
            _unit: PhantomData,
        }
    }

    /// Creates a rotation from an angle in radians.
    pub fn radians(angle: T) -> Self {
        Self::new(Angle::radians(angle))
    }

    /// Creates the identity rotation.
    #[inline]
    pub fn identity() -> Self
    where
        T: Zero,
    {
        Self::radians(T::zero())
    }
}

impl<T: Copy, Src, Dst> Rotation2D<T, Src, Dst> {
    /// Cast the unit, preserving the numeric value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use euclid::Rotation2D;
    /// enum Local {}
    /// enum World {}
    ///
    /// enum Local2 {}
    /// enum World2 {}
    ///
    /// let to_world: Rotation2D<_, Local, World> = Rotation2D::radians(42);
    ///
    /// assert_eq!(to_world.angle, to_world.cast_unit::<Local2, World2>().angle);
    /// ```
    #[inline]
    pub fn cast_unit<Src2, Dst2>(&self) -> Rotation2D<T, Src2, Dst2> {
        Rotation2D {
            angle: self.angle,
            _unit: PhantomData,
        }
    }

    /// Drop the units, preserving only the numeric value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use euclid::Rotation2D;
    /// enum Local {}
    /// enum World {}
    ///
    /// let to_world: Rotation2D<_, Local, World> = Rotation2D::radians(42);
    ///
    /// assert_eq!(to_world.angle, to_world.to_untyped().angle);
    /// ```
    #[inline]
    pub fn to_untyped(&self) -> Rotation2D<T, UnknownUnit, UnknownUnit> {
        self.cast_unit()
    }

    /// Tag a unitless value with units.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use euclid::Rotation2D;
    /// use euclid::UnknownUnit;
    /// enum Local {}
    /// enum World {}
    ///
    /// let rot: Rotation2D<_, UnknownUnit, UnknownUnit> = Rotation2D::radians(42);
    ///
    /// assert_eq!(rot.angle, Rotation2D::<_, Local, World>::from_untyped(&rot).angle);
    /// ```
    #[inline]
    pub fn from_untyped(r: &Rotation2D<T, UnknownUnit, UnknownUnit>) -> Self {
        r.cast_unit()
    }
}

impl<T, Src, Dst> Rotation2D<T, Src, Dst>
where
    T: Copy,
{
    /// Returns self.angle as a strongly typed `Angle<T>`.
    pub fn get_angle(&self) -> Angle<T> {
        Angle::radians(self.angle)
    }
}

impl<T: Real, Src, Dst> Rotation2D<T, Src, Dst> {
    /// Creates a 3d rotation (around the z axis) from this 2d rotation.
    #[inline]
    pub fn to_3d(&self) -> Rotation3D<T, Src, Dst> {
        Rotation3D::around_z(self.get_angle())
    }

    /// Returns the inverse of this rotation.
    #[inline]
    pub fn inverse(&self) -> Rotation2D<T, Dst, Src> {
        Rotation2D::radians(-self.angle)
    }

    /// Returns a rotation representing the other rotation followed by this rotation.
    #[inline]
    pub fn then<NewSrc>(&self, other: &Rotation2D<T, NewSrc, Src>) -> Rotation2D<T, NewSrc, Dst> {
        Rotation2D::radians(self.angle + other.angle)
    }

    /// Returns the given 2d point transformed by this rotation.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_point(&self, point: Point2D<T, Src>) -> Point2D<T, Dst> {
        let (sin, cos) = Real::sin_cos(self.angle);
        point2(point.x * cos - point.y * sin, point.y * cos + point.x * sin)
    }

    /// Returns the given 2d vector transformed by this rotation.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_vector(&self, vector: Vector2D<T, Src>) -> Vector2D<T, Dst> {
        self.transform_point(vector.to_point()).to_vector()
    }
}

impl<T, Src, Dst> Rotation2D<T, Src, Dst>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Zero + Trig,
{
    /// Returns the matrix representation of this rotation.
    #[inline]
    pub fn to_transform(&self) -> Transform2D<T, Src, Dst> {
        Transform2D::rotation(self.get_angle())
    }
}

impl<T: fmt::Debug, Src, Dst> fmt::Debug for Rotation2D<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rotation({:?} rad)", self.angle)
    }
}

impl<T, Src, Dst> ApproxEq<T> for Rotation2D<T, Src, Dst>
where
    T: Copy + Neg<Output = T> + ApproxEq<T>,
{
    fn approx_epsilon() -> T {
        T::approx_epsilon()
    }

    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        self.angle.approx_eq_eps(&other.angle, eps)
    }
}

/// A transform that can represent rotations in 3d, represented as a quaternion.
///
/// Most methods expect the quaternion to be normalized.
/// When in doubt, use [`unit_quaternion`] instead of [`quaternion`] to create
/// a rotation as the former will ensure that its result is normalized.
///
/// Some people use the `x, y, z, w` (or `w, x, y, z`) notations. The equivalence is
/// as follows: `x -> i`, `y -> j`, `z -> k`, `w -> r`.
/// The memory layout of this type corresponds to the `x, y, z, w` notation
///
/// [`quaternion`]: Self::quaternion
/// [`unit_quaternion`]: Self::unit_quaternion
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "T: serde::Serialize",
        deserialize = "T: serde::Deserialize<'de>"
    ))
)]
pub struct Rotation3D<T, Src, Dst> {
    /// Component multiplied by the imaginary number `i`.
    pub i: T,
    /// Component multiplied by the imaginary number `j`.
    pub j: T,
    /// Component multiplied by the imaginary number `k`.
    pub k: T,
    /// The real part.
    pub r: T,
    #[doc(hidden)]
    pub _unit: PhantomData<(Src, Dst)>,
}

impl<T: Copy, Src, Dst> Copy for Rotation3D<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for Rotation3D<T, Src, Dst> {
    fn clone(&self) -> Self {
        Rotation3D {
            i: self.i.clone(),
            j: self.j.clone(),
            k: self.k.clone(),
            r: self.r.clone(),
            _unit: PhantomData,
        }
    }
}

impl<T, Src, Dst> Eq for Rotation3D<T, Src, Dst> where T: Eq {}

impl<T, Src, Dst> PartialEq for Rotation3D<T, Src, Dst>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.k == other.k && self.r == other.r
    }
}

impl<T, Src, Dst> Hash for Rotation3D<T, Src, Dst>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.i.hash(h);
        self.j.hash(h);
        self.k.hash(h);
        self.r.hash(h);
    }
}

/// Note: the quaternions produced by this implementation are not normalized
/// (nor even necessarily finite). That is, this is not appropriate to use to
/// choose an actual “arbitrary rotation”, at least not without postprocessing.
#[cfg(feature = "arbitrary")]
impl<'a, T, Src, Dst> arbitrary::Arbitrary<'a> for Rotation3D<T, Src, Dst>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (i, j, k, r) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(Rotation3D::quaternion(i, j, k, r))
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, Src, Dst> Zeroable for Rotation3D<T, Src, Dst> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, Src: 'static, Dst: 'static> Pod for Rotation3D<T, Src, Dst> {}

impl<T, Src, Dst> Rotation3D<T, Src, Dst> {
    /// Creates a rotation around from a quaternion representation.
    ///
    /// The parameters are a, b, c and r compose the quaternion `a*i + b*j + c*k + r`
    /// where `a`, `b` and `c` describe the vector part and the last parameter `r` is
    /// the real part.
    ///
    /// The resulting quaternion is not necessarily normalized. See [`unit_quaternion`].
    ///
    /// [`unit_quaternion`]: Self::unit_quaternion
    #[inline]
    pub fn quaternion(a: T, b: T, c: T, r: T) -> Self {
        Rotation3D {
            i: a,
            j: b,
            k: c,
            r,
            _unit: PhantomData,
        }
    }

    /// Creates the identity rotation.
    #[inline]
    pub fn identity() -> Self
    where
        T: Zero + One,
    {
        Self::quaternion(T::zero(), T::zero(), T::zero(), T::one())
    }
}

impl<T, Src, Dst> Rotation3D<T, Src, Dst>
where
    T: Copy,
{
    /// Returns the vector part (i, j, k) of this quaternion.
    #[inline]
    pub fn vector_part(&self) -> Vector3D<T, UnknownUnit> {
        vec3(self.i, self.j, self.k)
    }

    /// Cast the unit, preserving the numeric value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use euclid::Rotation3D;
    /// enum Local {}
    /// enum World {}
    ///
    /// enum Local2 {}
    /// enum World2 {}
    ///
    /// let to_world: Rotation3D<_, Local, World> = Rotation3D::quaternion(1, 2, 3, 4);
    ///
    /// assert_eq!(to_world.i, to_world.cast_unit::<Local2, World2>().i);
    /// assert_eq!(to_world.j, to_world.cast_unit::<Local2, World2>().j);
    /// assert_eq!(to_world.k, to_world.cast_unit::<Local2, World2>().k);
    /// assert_eq!(to_world.r, to_world.cast_unit::<Local2, World2>().r);
    /// ```
    #[inline]
    pub fn cast_unit<Src2, Dst2>(&self) -> Rotation3D<T, Src2, Dst2> {
        Rotation3D {
            i: self.i,
            j: self.j,
            k: self.k,
            r: self.r,
            _unit: PhantomData,
        }
    }

    /// Drop the units, preserving only the numeric value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use euclid::Rotation3D;
    /// enum Local {}
    /// enum World {}
    ///
    /// let to_world: Rotation3D<_, Local, World> = Rotation3D::quaternion(1, 2, 3, 4);
    ///
    /// assert_eq!(to_world.i, to_world.to_untyped().i);
    /// assert_eq!(to_world.j, to_world.to_untyped().j);
    /// assert_eq!(to_world.k, to_world.to_untyped().k);
    /// assert_eq!(to_world.r, to_world.to_untyped().r);
    /// ```
    #[inline]
    pub fn to_untyped(&self) -> Rotation3D<T, UnknownUnit, UnknownUnit> {
        self.cast_unit()
    }

    /// Tag a unitless value with units.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use euclid::Rotation3D;
    /// use euclid::UnknownUnit;
    /// enum Local {}
    /// enum World {}
    ///
    /// let rot: Rotation3D<_, UnknownUnit, UnknownUnit> = Rotation3D::quaternion(1, 2, 3, 4);
    ///
    /// assert_eq!(rot.i, Rotation3D::<_, Local, World>::from_untyped(&rot).i);
    /// assert_eq!(rot.j, Rotation3D::<_, Local, World>::from_untyped(&rot).j);
    /// assert_eq!(rot.k, Rotation3D::<_, Local, World>::from_untyped(&rot).k);
    /// assert_eq!(rot.r, Rotation3D::<_, Local, World>::from_untyped(&rot).r);
    /// ```
    #[inline]
    pub fn from_untyped(r: &Rotation3D<T, UnknownUnit, UnknownUnit>) -> Self {
        r.cast_unit()
    }
}

impl<T, Src, Dst> Rotation3D<T, Src, Dst>
where
    T: Real,
{
    /// Creates a rotation around from a quaternion representation and normalizes it.
    ///
    /// The parameters are a, b, c and r compose the quaternion `a*i + b*j + c*k + r`
    /// before normalization, where `a`, `b` and `c` describe the vector part and the
    /// last parameter `r` is the real part.
    #[inline]
    pub fn unit_quaternion(i: T, j: T, k: T, r: T) -> Self {
        Self::quaternion(i, j, k, r).normalize()
    }

    /// Creates a rotation around a given axis.
    pub fn around_axis(axis: Vector3D<T, Src>, angle: Angle<T>) -> Self {
        let axis = axis.normalize();
        let two = T::one() + T::one();
        let (sin, cos) = Angle::sin_cos(angle / two);
        Self::quaternion(axis.x * sin, axis.y * sin, axis.z * sin, cos)
    }

    /// Creates a rotation around the x axis.
    pub fn around_x(angle: Angle<T>) -> Self {
        let zero = Zero::zero();
        let two = T::one() + T::one();
        let (sin, cos) = Angle::sin_cos(angle / two);
        Self::quaternion(sin, zero, zero, cos)
    }

    /// Creates a rotation around the y axis.
    pub fn around_y(angle: Angle<T>) -> Self {
        let zero = Zero::zero();
        let two = T::one() + T::one();
        let (sin, cos) = Angle::sin_cos(angle / two);
        Self::quaternion(zero, sin, zero, cos)
    }

    /// Creates a rotation around the z axis.
    pub fn around_z(angle: Angle<T>) -> Self {
        let zero = Zero::zero();
        let two = T::one() + T::one();
        let (sin, cos) = Angle::sin_cos(angle / two);
        Self::quaternion(zero, zero, sin, cos)
    }

    /// Creates a rotation from Euler angles.
    ///
    /// The rotations are applied in roll then pitch then yaw order.
    ///
    ///  - Roll (also called bank) is a rotation around the x axis.
    ///  - Pitch (also called bearing) is a rotation around the y axis.
    ///  - Yaw (also called heading) is a rotation around the z axis.
    pub fn euler(roll: Angle<T>, pitch: Angle<T>, yaw: Angle<T>) -> Self {
        let half = T::one() / (T::one() + T::one());

        let (sy, cy) = Real::sin_cos(half * yaw.get());
        let (sp, cp) = Real::sin_cos(half * pitch.get());
        let (sr, cr) = Real::sin_cos(half * roll.get());

        Self::quaternion(
            cy * sr * cp - sy * cr * sp,
            cy * cr * sp + sy * sr * cp,
            sy * cr * cp - cy * sr * sp,
            cy * cr * cp + sy * sr * sp,
        )
    }

    /// Returns the inverse of this rotation.
    #[inline]
    pub fn inverse(&self) -> Rotation3D<T, Dst, Src> {
        Rotation3D::quaternion(-self.i, -self.j, -self.k, self.r)
    }

    /// Computes the norm of this quaternion.
    #[inline]
    pub fn norm(&self) -> T {
        self.square_norm().sqrt()
    }

    /// Computes the squared norm of this quaternion.
    #[inline]
    pub fn square_norm(&self) -> T {
        self.i * self.i + self.j * self.j + self.k * self.k + self.r * self.r
    }

    /// Returns a [unit quaternion] from this one.
    ///
    /// [unit quaternion]: https://en.wikipedia.org/wiki/Quaternion#Unit_quaternion
    #[inline]
    pub fn normalize(&self) -> Self {
        self.mul(T::one() / self.norm())
    }

    /// Returns `true` if [norm] of this quaternion is (approximately) one.
    ///
    /// [norm]: Self::norm
    #[inline]
    pub fn is_normalized(&self) -> bool
    where
        T: ApproxEq<T>,
    {
        let eps = NumCast::from(1.0e-5).unwrap();
        self.square_norm().approx_eq_eps(&T::one(), &eps)
    }

    /// Spherical linear interpolation between this rotation and another rotation.
    ///
    /// `t` is expected to be between zero and one.
    pub fn slerp(&self, other: &Self, t: T) -> Self
    where
        T: ApproxEq<T>,
    {
        debug_assert!(self.is_normalized());
        debug_assert!(other.is_normalized());

        let r1 = *self;
        let mut r2 = *other;

        let mut dot = r1.i * r2.i + r1.j * r2.j + r1.k * r2.k + r1.r * r2.r;

        let one = T::one();

        if dot.approx_eq(&T::one()) {
            // If the inputs are too close, linearly interpolate to avoid precision issues.
            return r1.lerp(&r2, t);
        }

        // If the dot product is negative, the quaternions
        // have opposite handed-ness and slerp won't take
        // the shorter path. Fix by reversing one quaternion.
        if dot < T::zero() {
            r2 = r2.mul(-T::one());
            dot = -dot;
        }

        // For robustness, stay within the domain of acos.
        dot = Real::min(dot, one);

        // Angle between r1 and the result.
        let theta = Real::acos(dot) * t;

        // r1 and r3 form an orthonormal basis.
        let r3 = r2.sub(r1.mul(dot)).normalize();
        let (sin, cos) = Real::sin_cos(theta);
        r1.mul(cos).add(r3.mul(sin))
    }

    /// Basic Linear interpolation between this rotation and another rotation.
    #[inline]
    pub fn lerp(&self, other: &Self, t: T) -> Self {
        let one_t = T::one() - t;
        self.mul(one_t).add(other.mul(t)).normalize()
    }

    /// Returns the given 3d point transformed by this rotation.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    pub fn transform_point3d(&self, point: Point3D<T, Src>) -> Point3D<T, Dst>
    where
        T: ApproxEq<T>,
    {
        debug_assert!(self.is_normalized());

        let two = T::one() + T::one();
        let cross = self.vector_part().cross(point.to_vector().to_untyped()) * two;

        point3(
            point.x + self.r * cross.x + self.j * cross.z - self.k * cross.y,
            point.y + self.r * cross.y + self.k * cross.x - self.i * cross.z,
            point.z + self.r * cross.z + self.i * cross.y - self.j * cross.x,
        )
    }

    /// Returns the given 2d point transformed by this rotation then projected on the xy plane.
    ///
    /// The input point must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_point2d(&self, point: Point2D<T, Src>) -> Point2D<T, Dst>
    where
        T: ApproxEq<T>,
    {
        self.transform_point3d(point.to_3d()).xy()
    }

    /// Returns the given 3d vector transformed by this rotation.
    ///
    /// The input vector must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_vector3d(&self, vector: Vector3D<T, Src>) -> Vector3D<T, Dst>
    where
        T: ApproxEq<T>,
    {
        self.transform_point3d(vector.to_point()).to_vector()
    }

    /// Returns the given 2d vector transformed by this rotation then projected on the xy plane.
    ///
    /// The input vector must be use the unit Src, and the returned point has the unit Dst.
    #[inline]
    pub fn transform_vector2d(&self, vector: Vector2D<T, Src>) -> Vector2D<T, Dst>
    where
        T: ApproxEq<T>,
    {
        self.transform_vector3d(vector.to_3d()).xy()
    }

    /// Returns the matrix representation of this rotation.
    #[inline]
    #[rustfmt::skip]
    pub fn to_transform(&self) -> Transform3D<T, Src, Dst>
    where
        T: ApproxEq<T>,
    {
        debug_assert!(self.is_normalized());

        let i2 = self.i + self.i;
        let j2 = self.j + self.j;
        let k2 = self.k + self.k;
        let ii = self.i * i2;
        let ij = self.i * j2;
        let ik = self.i * k2;
        let jj = self.j * j2;
        let jk = self.j * k2;
        let kk = self.k * k2;
        let ri = self.r * i2;
        let rj = self.r * j2;
        let rk = self.r * k2;

        let one = T::one();
        let zero = T::zero();

        let m11 = one - (jj + kk);
        let m12 = ij + rk;
        let m13 = ik - rj;

        let m21 = ij - rk;
        let m22 = one - (ii + kk);
        let m23 = jk + ri;

        let m31 = ik + rj;
        let m32 = jk - ri;
        let m33 = one - (ii + jj);

        Transform3D::new(
            m11, m12, m13, zero,
            m21, m22, m23, zero,
            m31, m32, m33, zero,
            zero, zero, zero, one,
        )
    }

    /// Returns a rotation representing this rotation followed by the other rotation.
    #[inline]
    pub fn then<NewDst>(&self, other: &Rotation3D<T, Dst, NewDst>) -> Rotation3D<T, Src, NewDst>
    where
        T: ApproxEq<T>,
    {
        debug_assert!(self.is_normalized());
        Rotation3D::quaternion(
            other.i * self.r + other.r * self.i + other.j * self.k - other.k * self.j,
            other.j * self.r + other.r * self.j + other.k * self.i - other.i * self.k,
            other.k * self.r + other.r * self.k + other.i * self.j - other.j * self.i,
            other.r * self.r - other.i * self.i - other.j * self.j - other.k * self.k,
        )
    }

    // add, sub and mul are used internally for intermediate computation but aren't public
    // because they don't carry real semantic meanings (I think?).

    #[inline]
    fn add(&self, other: Self) -> Self {
        Self::quaternion(
            self.i + other.i,
            self.j + other.j,
            self.k + other.k,
            self.r + other.r,
        )
    }

    #[inline]
    fn sub(&self, other: Self) -> Self {
        Self::quaternion(
            self.i - other.i,
            self.j - other.j,
            self.k - other.k,
            self.r - other.r,
        )
    }

    #[inline]
    fn mul(&self, factor: T) -> Self {
        Self::quaternion(
            self.i * factor,
            self.j * factor,
            self.k * factor,
            self.r * factor,
        )
    }
}

impl<T: fmt::Debug, Src, Dst> fmt::Debug for Rotation3D<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Quat({:?}*i + {:?}*j + {:?}*k + {:?})",
            self.i, self.j, self.k, self.r
        )
    }
}

impl<T, Src, Dst> ApproxEq<T> for Rotation3D<T, Src, Dst>
where
    T: Copy + Neg<Output = T> + ApproxEq<T>,
{
    fn approx_epsilon() -> T {
        T::approx_epsilon()
    }

    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        (self.i.approx_eq_eps(&other.i, eps)
            && self.j.approx_eq_eps(&other.j, eps)
            && self.k.approx_eq_eps(&other.k, eps)
            && self.r.approx_eq_eps(&other.r, eps))
            || (self.i.approx_eq_eps(&-other.i, eps)
                && self.j.approx_eq_eps(&-other.j, eps)
                && self.k.approx_eq_eps(&-other.k, eps)
                && self.r.approx_eq_eps(&-other.r, eps))
    }
}

#[test]
fn simple_rotation_2d() {
    use crate::default::Rotation2D;
    use core::f32::consts::{FRAC_PI_2, PI};

    let ri = Rotation2D::identity();
    let r90 = Rotation2D::radians(FRAC_PI_2);
    let rm90 = Rotation2D::radians(-FRAC_PI_2);
    let r180 = Rotation2D::radians(PI);

    assert!(ri
        .transform_point(point2(1.0, 2.0))
        .approx_eq(&point2(1.0, 2.0)));
    assert!(r90
        .transform_point(point2(1.0, 2.0))
        .approx_eq(&point2(-2.0, 1.0)));
    assert!(rm90
        .transform_point(point2(1.0, 2.0))
        .approx_eq(&point2(2.0, -1.0)));
    assert!(r180
        .transform_point(point2(1.0, 2.0))
        .approx_eq(&point2(-1.0, -2.0)));

    assert!(r90
        .inverse()
        .inverse()
        .transform_point(point2(1.0, 2.0))
        .approx_eq(&r90.transform_point(point2(1.0, 2.0))));
}

#[test]
fn simple_rotation_3d_in_2d() {
    use crate::default::Rotation3D;
    use core::f32::consts::{FRAC_PI_2, PI};

    let ri = Rotation3D::identity();
    let r90 = Rotation3D::around_z(Angle::radians(FRAC_PI_2));
    let rm90 = Rotation3D::around_z(Angle::radians(-FRAC_PI_2));
    let r180 = Rotation3D::around_z(Angle::radians(PI));

    assert!(ri
        .transform_point2d(point2(1.0, 2.0))
        .approx_eq(&point2(1.0, 2.0)));
    assert!(r90
        .transform_point2d(point2(1.0, 2.0))
        .approx_eq(&point2(-2.0, 1.0)));
    assert!(rm90
        .transform_point2d(point2(1.0, 2.0))
        .approx_eq(&point2(2.0, -1.0)));
    assert!(r180
        .transform_point2d(point2(1.0, 2.0))
        .approx_eq(&point2(-1.0, -2.0)));

    assert!(r90
        .inverse()
        .inverse()
        .transform_point2d(point2(1.0, 2.0))
        .approx_eq(&r90.transform_point2d(point2(1.0, 2.0))));
}

#[test]
fn pre_post() {
    use crate::default::Rotation3D;
    use core::f32::consts::FRAC_PI_2;

    let r1 = Rotation3D::around_x(Angle::radians(FRAC_PI_2));
    let r2 = Rotation3D::around_y(Angle::radians(FRAC_PI_2));
    let r3 = Rotation3D::around_z(Angle::radians(FRAC_PI_2));

    let t1 = r1.to_transform();
    let t2 = r2.to_transform();
    let t3 = r3.to_transform();

    let p = point3(1.0, 2.0, 3.0);

    // Check that the order of transformations is correct (corresponds to what
    // we do in Transform3D).
    let p1 = r1.then(&r2).then(&r3).transform_point3d(p);
    let p2 = t1.then(&t2).then(&t3).transform_point3d(p);

    assert!(p1.approx_eq(&p2.unwrap()));

    // Check that changing the order indeed matters.
    let p3 = t3.then(&t1).then(&t2).transform_point3d(p);
    assert!(!p1.approx_eq(&p3.unwrap()));
}

#[test]
fn to_transform3d() {
    use crate::default::Rotation3D;

    use core::f32::consts::{FRAC_PI_2, PI};
    let rotations = [
        Rotation3D::identity(),
        Rotation3D::around_x(Angle::radians(FRAC_PI_2)),
        Rotation3D::around_x(Angle::radians(-FRAC_PI_2)),
        Rotation3D::around_x(Angle::radians(PI)),
        Rotation3D::around_y(Angle::radians(FRAC_PI_2)),
        Rotation3D::around_y(Angle::radians(-FRAC_PI_2)),
        Rotation3D::around_y(Angle::radians(PI)),
        Rotation3D::around_z(Angle::radians(FRAC_PI_2)),
        Rotation3D::around_z(Angle::radians(-FRAC_PI_2)),
        Rotation3D::around_z(Angle::radians(PI)),
    ];

    let points = [
        point3(0.0, 0.0, 0.0),
        point3(1.0, 2.0, 3.0),
        point3(-5.0, 3.0, -1.0),
        point3(-0.5, -1.0, 1.5),
    ];

    for rotation in &rotations {
        for &point in &points {
            let p1 = rotation.transform_point3d(point);
            let p2 = rotation.to_transform().transform_point3d(point);
            assert!(p1.approx_eq(&p2.unwrap()));
        }
    }
}

#[test]
fn slerp() {
    use crate::default::Rotation3D;

    let q1 = Rotation3D::quaternion(1.0, 0.0, 0.0, 0.0);
    let q2 = Rotation3D::quaternion(0.0, 1.0, 0.0, 0.0);
    let q3 = Rotation3D::quaternion(0.0, 0.0, -1.0, 0.0);

    // The values below can be obtained with a python program:
    // import numpy
    // import quaternion
    // q1 = numpy.quaternion(1, 0, 0, 0)
    // q2 = numpy.quaternion(0, 1, 0, 0)
    // quaternion.slerp_evaluate(q1, q2, 0.2)

    assert!(q1.slerp(&q2, 0.0).approx_eq(&q1));
    assert!(q1.slerp(&q2, 0.2).approx_eq(&Rotation3D::quaternion(
        0.951056516295154,
        0.309016994374947,
        0.0,
        0.0
    )));
    assert!(q1.slerp(&q2, 0.4).approx_eq(&Rotation3D::quaternion(
        0.809016994374947,
        0.587785252292473,
        0.0,
        0.0
    )));
    assert!(q1.slerp(&q2, 0.6).approx_eq(&Rotation3D::quaternion(
        0.587785252292473,
        0.809016994374947,
        0.0,
        0.0
    )));
    assert!(q1.slerp(&q2, 0.8).approx_eq(&Rotation3D::quaternion(
        0.309016994374947,
        0.951056516295154,
        0.0,
        0.0
    )));
    assert!(q1.slerp(&q2, 1.0).approx_eq(&q2));

    assert!(q1.slerp(&q3, 0.0).approx_eq(&q1));
    assert!(q1.slerp(&q3, 0.2).approx_eq(&Rotation3D::quaternion(
        0.951056516295154,
        0.0,
        -0.309016994374947,
        0.0
    )));
    assert!(q1.slerp(&q3, 0.4).approx_eq(&Rotation3D::quaternion(
        0.809016994374947,
        0.0,
        -0.587785252292473,
        0.0
    )));
    assert!(q1.slerp(&q3, 0.6).approx_eq(&Rotation3D::quaternion(
        0.587785252292473,
        0.0,
        -0.809016994374947,
        0.0
    )));
    assert!(q1.slerp(&q3, 0.8).approx_eq(&Rotation3D::quaternion(
        0.309016994374947,
        0.0,
        -0.951056516295154,
        0.0
    )));
    assert!(q1.slerp(&q3, 1.0).approx_eq(&q3));
}

#[test]
fn around_axis() {
    use crate::default::Rotation3D;
    use core::f32::consts::{FRAC_PI_2, PI};

    // Two sort of trivial cases:
    let r1 = Rotation3D::around_axis(vec3(1.0, 1.0, 0.0), Angle::radians(PI));
    let r2 = Rotation3D::around_axis(vec3(1.0, 1.0, 0.0), Angle::radians(FRAC_PI_2));
    assert!(r1
        .transform_point3d(point3(1.0, 2.0, 0.0))
        .approx_eq(&point3(2.0, 1.0, 0.0)));
    assert!(r2
        .transform_point3d(point3(1.0, 0.0, 0.0))
        .approx_eq(&point3(0.5, 0.5, -0.5.sqrt())));

    // A more arbitrary test (made up with numpy):
    let r3 = Rotation3D::around_axis(vec3(0.5, 1.0, 2.0), Angle::radians(2.291288));
    assert!(r3
        .transform_point3d(point3(1.0, 0.0, 0.0))
        .approx_eq(&point3(-0.58071821, 0.81401868, -0.01182979)));
}

#[test]
fn from_euler() {
    use crate::default::Rotation3D;
    use core::f32::consts::FRAC_PI_2;

    // First test simple separate yaw pitch and roll rotations, because it is easy to come
    // up with the corresponding quaternion.
    // Since several quaternions can represent the same transformation we compare the result
    // of transforming a point rather than the values of each quaternions.
    let p = point3(1.0, 2.0, 3.0);

    let angle = Angle::radians(FRAC_PI_2);
    let zero = Angle::radians(0.0);

    // roll
    let roll_re = Rotation3D::euler(angle, zero, zero);
    let roll_rq = Rotation3D::around_x(angle);
    let roll_pe = roll_re.transform_point3d(p);
    let roll_pq = roll_rq.transform_point3d(p);

    // pitch
    let pitch_re = Rotation3D::euler(zero, angle, zero);
    let pitch_rq = Rotation3D::around_y(angle);
    let pitch_pe = pitch_re.transform_point3d(p);
    let pitch_pq = pitch_rq.transform_point3d(p);

    // yaw
    let yaw_re = Rotation3D::euler(zero, zero, angle);
    let yaw_rq = Rotation3D::around_z(angle);
    let yaw_pe = yaw_re.transform_point3d(p);
    let yaw_pq = yaw_rq.transform_point3d(p);

    assert!(roll_pe.approx_eq(&roll_pq));
    assert!(pitch_pe.approx_eq(&pitch_pq));
    assert!(yaw_pe.approx_eq(&yaw_pq));

    // Now check that the yaw pitch and roll transformations when combined are applied in
    // the proper order: roll -> pitch -> yaw.
    let ypr_e = Rotation3D::euler(angle, angle, angle);
    let ypr_q = roll_rq.then(&pitch_rq).then(&yaw_rq);
    let ypr_pe = ypr_e.transform_point3d(p);
    let ypr_pq = ypr_q.transform_point3d(p);

    assert!(ypr_pe.approx_eq(&ypr_pq));
}
