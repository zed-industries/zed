// pathfinder/geometry/src/basic/point.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A SIMD-optimized point type.

use pathfinder_simd::default::{F32x2, F32x4, I32x2};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

/// 2D points with 32-bit floating point coordinates.
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2F(pub F32x2);

impl Vector2F {
    #[inline]
    pub fn new(x: f32, y: f32) -> Vector2F {
        Vector2F(F32x2::new(x, y))
    }

    #[inline]
    pub fn splat(value: f32) -> Vector2F {
        Vector2F(F32x2::splat(value))
    }

    #[inline]
    pub fn zero() -> Vector2F {
        Vector2F::default()
    }

    #[inline]
    pub fn to_3d(self) -> Vector3F {
        Vector3F(self.0.to_f32x4().concat_xy_zw(F32x4::new(0.0, 0.0, 0.0, 0.0)))
    }

    #[inline]
    pub fn to_4d(self) -> Vector4F {
        Vector4F(self.0.to_f32x4().concat_xy_zw(F32x4::new(0.0, 0.0, 0.0, 1.0)))
    }

    #[inline]
    pub fn x(self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn y(self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }

    #[inline]
    pub fn min(self, other: Vector2F) -> Vector2F {
        Vector2F(self.0.min(other.0))
    }

    #[inline]
    pub fn max(self, other: Vector2F) -> Vector2F {
        Vector2F(self.0.max(other.0))
    }

    #[inline]
    pub fn clamp(self, min_val: Vector2F, max_val: Vector2F) -> Vector2F {
        self.max(min_val).min(max_val)
    }

    #[inline]
    pub fn det(self, other: Vector2F) -> f32 {
        self.x() * other.y() - self.y() * other.x()
    }

    #[inline]
    pub fn dot(self, other: Vector2F) -> f32 {
        let xy = self.0 * other.0;
        xy.x() + xy.y()
    }

    #[inline]
    pub fn floor(self) -> Vector2F {
        Vector2F(self.0.floor())
    }

    #[inline]
    pub fn ceil(self) -> Vector2F {
        Vector2F(self.0.ceil())
    }

    /// Rounds both coordinates to the nearest integer.
    #[inline]
    pub fn round(self) -> Vector2F {
        Vector2F(self.0.to_i32x2().to_f32x2())
    }

    /// Treats this point as a vector and calculates its squared length.
    #[inline]
    pub fn square_length(self) -> f32 {
        let squared = self.0 * self.0;
        squared[0] + squared[1]
    }

    /// Treats this point as a vector and calculates its length.
    #[inline]
    pub fn length(self) -> f32 {
        f32::sqrt(self.square_length())
    }

    /// Treats this point as a vector and normalizes it.
    #[inline]
    pub fn normalize(self) -> Vector2F {
        self * (1.0 / self.length())
    }

    /// Swaps y and x.
    #[inline]
    pub fn yx(self) -> Vector2F {
        Vector2F(self.0.yx())
    }

    /// Returns the coefficient when the given vector `a` is projected onto this one.
    ///
    /// That is, if this vector is `v` and this function returns `c`, then `proj_v a = cv`. In
    /// other words, this function computes `(a⋅v) / (v⋅v)`.
    #[inline]
    pub fn projection_coefficient(self, a: Vector2F) -> f32 {
        a.dot(self) / self.square_length()
    }

    #[inline]
    pub fn is_zero(self) -> bool {
        self == Vector2F::zero()
    }

    #[inline]
    pub fn lerp(self, other: Vector2F, t: f32) -> Vector2F {
        self + (other - self) * t
    }

    #[inline]
    pub fn to_i32(self) -> Vector2I {
        Vector2I(self.0.to_i32x2())
    }
}

/// A convenience alias for `Vector2F::new()`.
#[inline]
pub fn vec2f(x: f32, y: f32) -> Vector2F {
    Vector2F::new(x, y)
}

impl PartialEq for Vector2F {
    #[inline]
    fn eq(&self, other: &Vector2F) -> bool {
        self.0.packed_eq(other.0).all_true()
    }
}

impl Add<Vector2F> for Vector2F {
    type Output = Vector2F;
    #[inline]
    fn add(self, other: Vector2F) -> Vector2F {
        Vector2F(self.0 + other.0)
    }
}

impl Add<f32> for Vector2F {
    type Output = Vector2F;
    #[inline]
    fn add(self, other: f32) -> Vector2F {
        self + Vector2F::splat(other)
    }
}

impl AddAssign<Vector2F> for Vector2F {
    #[inline]
    fn add_assign(&mut self, other: Vector2F) {
        *self = *self + other
    }
}

impl Sub<Vector2F> for Vector2F {
    type Output = Vector2F;
    #[inline]
    fn sub(self, other: Vector2F) -> Vector2F {
        Vector2F(self.0 - other.0)
    }
}

impl Sub<f32> for Vector2F {
    type Output = Vector2F;
    #[inline]
    fn sub(self, other: f32) -> Vector2F {
        self - Vector2F::splat(other)
    }
}

impl SubAssign<Vector2F> for Vector2F {
    #[inline]
    fn sub_assign(&mut self, other: Vector2F) {
        *self = *self - other
    }
}

impl Mul<Vector2F> for Vector2F {
    type Output = Vector2F;
    #[inline]
    fn mul(self, other: Vector2F) -> Vector2F {
        Vector2F(self.0 * other.0)
    }
}

impl Mul<f32> for Vector2F {
    type Output = Vector2F;
    #[inline]
    fn mul(self, other: f32) -> Vector2F {
        self * Vector2F::splat(other)
    }
}

impl MulAssign<Vector2F> for Vector2F {
    #[inline]
    fn mul_assign(&mut self, other: Vector2F) {
        *self = *self * other
    }
}

impl MulAssign<f32> for Vector2F {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other
    }
}

impl Div<Vector2F> for Vector2F {
    type Output = Vector2F;
    #[inline]
    fn div(self, other: Vector2F) -> Vector2F {
        Vector2F(self.0 / other.0)
    }
}

impl Div<f32> for Vector2F {
    type Output = Vector2F;
    #[inline]
    fn div(self, other: f32) -> Vector2F {
        self / Vector2F::splat(other)
    }
}

impl Neg for Vector2F {
    type Output = Vector2F;
    #[inline]
    fn neg(self) -> Vector2F {
        Vector2F(-self.0)
    }
}

/// Either a scalar or a `Vector2F`.
/// 
/// Scalars will be automatically splatted (i.e. `x` becomes `vec2f(x, x)`).
/// 
/// Be judicious with the use of this trait. Only use it if it will aid readability without the
/// potential to introduce bugs.
pub trait IntoVector2F {
    fn into_vector_2f(self) -> Vector2F;
}

impl IntoVector2F for Vector2F {
    #[inline]
    fn into_vector_2f(self) -> Vector2F {
        self
    }
}

impl IntoVector2F for f32 {
    #[inline]
    fn into_vector_2f(self) -> Vector2F {
        Vector2F::splat(self)
    }
}

/// 2D points with 32-bit signed integer coordinates.
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2I(pub I32x2);

impl Vector2I {
    #[inline]
    pub fn new(x: i32, y: i32) -> Vector2I {
        Vector2I(I32x2::new(x, y))
    }

    #[inline]
    pub fn splat(value: i32) -> Vector2I {
        Vector2I(I32x2::splat(value))
    }

    #[inline]
    pub fn zero() -> Vector2I {
        Vector2I::default()
    }

    #[inline]
    pub fn x(self) -> i32 {
        self.0[0]
    }

    #[inline]
    pub fn y(self) -> i32 {
        self.0[1]
    }

    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.0[0] = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.0[1] = y;
    }

    #[inline]
    pub fn min(self, other: Vector2I) -> Vector2I {
        Vector2I(self.0.min(other.0))
    }

    #[inline]
    pub fn max(self, other: Vector2I) -> Vector2I {
        Vector2I(self.0.max(other.0))
    }

    #[inline]
    pub fn to_f32(self) -> Vector2F {
        Vector2F(self.0.to_f32x2())
    }
}

/// A convenience alias for `Vector2I::new()`.
#[inline]
pub fn vec2i(x: i32, y: i32) -> Vector2I {
    Vector2I::new(x, y)
}

impl Add<Vector2I> for Vector2I {
    type Output = Vector2I;
    #[inline]
    fn add(self, other: Vector2I) -> Vector2I {
        Vector2I(self.0 + other.0)
    }
}

impl Add<i32> for Vector2I {
    type Output = Vector2I;
    #[inline]
    fn add(self, other: i32) -> Vector2I {
        self + Vector2I::splat(other)
    }
}

impl AddAssign<Vector2I> for Vector2I {
    #[inline]
    fn add_assign(&mut self, other: Vector2I) {
        self.0 += other.0
    }
}

impl Neg for Vector2I {
    type Output = Vector2I;
    #[inline]
    fn neg(self) -> Vector2I {
        Vector2I(-self.0)
    }
}

impl Sub<Vector2I> for Vector2I {
    type Output = Vector2I;
    #[inline]
    fn sub(self, other: Vector2I) -> Vector2I {
        Vector2I(self.0 - other.0)
    }
}

impl Sub<i32> for Vector2I {
    type Output = Vector2I;
    #[inline]
    fn sub(self, other: i32) -> Vector2I {
        self - Vector2I::splat(other)
    }
}

impl Mul<Vector2I> for Vector2I {
    type Output = Vector2I;
    #[inline]
    fn mul(self, other: Vector2I) -> Vector2I {
        Vector2I(self.0 * other.0)
    }
}

impl Mul<i32> for Vector2I {
    type Output = Vector2I;
    #[inline]
    fn mul(self, other: i32) -> Vector2I {
        self * Vector2I::splat(other)
    }
}

impl PartialEq for Vector2I {
    #[inline]
    fn eq(&self, other: &Vector2I) -> bool {
        self.0.packed_eq(other.0).all_true()
    }
}

impl Eq for Vector2I {}

impl Hash for Vector2I {
    #[inline]
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.x().hash(state);
        self.y().hash(state);
    }
}

/// 3D points.
///
/// The w value in the SIMD vector is always 0.0.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3F(pub F32x4);

impl Vector3F {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3F {
        Vector3F(F32x4::new(x, y, z, 0.0))
    }

    #[inline]
    pub fn splat(x: f32) -> Vector3F {
        let mut vector = F32x4::splat(x);
        vector.set_w(0.0);
        Vector3F(vector)
    }

    /// Truncates this vector to 2D.
    #[inline]
    pub fn to_2d(self) -> Vector2F {
        Vector2F(self.0.xy())
    }

    /// Converts this vector to an equivalent 3D homogeneous one with a w component of 1.0.
    #[inline]
    pub fn to_4d(self) -> Vector4F {
        let mut vector = self.0;
        vector.set_w(1.0);
        Vector4F(vector)
    }

    #[inline]
    pub fn cross(self, other: Vector3F) -> Vector3F {
        Vector3F(self.0.yzxw() * other.0.zxyw() - self.0.zxyw() * other.0.yzxw())
    }

    #[inline]
    pub fn square_length(self) -> f32 {
        let squared = self.0 * self.0;
        squared[0] + squared[1] + squared[2]
    }

    #[inline]
    pub fn length(self) -> f32 {
        f32::sqrt(self.square_length())
    }

    #[inline]
    pub fn normalize(self) -> Vector3F {
        Vector3F(self.0 * F32x4::splat(1.0 / self.length()))
    }

    #[inline]
    pub fn x(self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn y(self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn z(self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn scale(self, factor: f32) -> Vector3F {
        Vector3F(self.0 * F32x4::splat(factor))
    }
}

impl Add<Vector3F> for Vector3F {
    type Output = Vector3F;
    #[inline]
    fn add(self, other: Vector3F) -> Vector3F {
        Vector3F(self.0 + other.0)
    }
}

impl AddAssign for Vector3F {
    #[inline]
    fn add_assign(&mut self, other: Vector3F) {
        self.0 += other.0
    }
}

impl Neg for Vector3F {
    type Output = Vector3F;
    #[inline]
    fn neg(self) -> Vector3F {
        Vector3F(self.0 * F32x4::new(-1.0, -1.0, -1.0, 0.0))
    }
}

impl Sub<Vector3F> for Vector3F {
    type Output = Vector3F;
    #[inline]
    fn sub(self, other: Vector3F) -> Vector3F {
        Vector3F(self.0 - other.0)
    }
}

/// 3D homogeneous points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector4F(pub F32x4);

impl Vector4F {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4F {
        Vector4F(F32x4::new(x, y, z, w))
    }

    #[inline]
    pub fn splat(value: f32) -> Vector4F {
        Vector4F(F32x4::splat(value))
    }

    #[inline]
    pub fn to_2d(self) -> Vector2F {
        self.to_3d().to_2d()
    }

    /// Performs perspective division to convert this vector to 3D.
    #[inline]
    pub fn to_3d(self) -> Vector3F {
        let mut vector = self.0 * F32x4::splat(1.0 / self.w());
        vector.set_w(0.0);
        Vector3F(vector)
    }

    #[inline]
    pub fn x(self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn y(self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn z(self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn w(self) -> f32 {
        self.0[3]
    }

    #[inline]
    pub fn scale(self, x: f32) -> Vector4F {
        let mut factors = F32x4::splat(x);
        factors[3] = 1.0;
        Vector4F(self.0 * factors)
    }

    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self.0[0] = x
    }

    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self.0[1] = y
    }

    #[inline]
    pub fn set_z(&mut self, z: f32) {
        self.0[2] = z
    }

    #[inline]
    pub fn set_w(&mut self, w: f32) {
        self.0[3] = w
    }

    #[inline]
    pub fn approx_eq(self, other: Vector4F, epsilon: f32) -> bool {
        self.0.approx_eq(other.0, epsilon)
    }

    /// Checks to see whether this *homogeneous* coordinate equals zero.
    ///
    /// Note that since this treats the coordinate as a homogeneous coordinate, the `w` is ignored.
    // TODO(pcwalton): Optimize with SIMD.
    #[inline]
    pub fn is_zero(self) -> bool {
        self.x() == 0.0 && self.y() == 0.0 && self.z() == 0.0
    }

    #[inline]
    pub fn lerp(self, other: Vector4F, t: f32) -> Vector4F {
        Vector4F(self.0 + (other.0 - self.0) * F32x4::splat(t))
    }
}

impl Add<Vector4F> for Vector4F {
    type Output = Vector4F;
    #[inline]
    fn add(self, other: Vector4F) -> Vector4F {
        Vector4F(self.0 + other.0)
    }
}

impl AddAssign for Vector4F {
    #[inline]
    fn add_assign(&mut self, other: Vector4F) {
        self.0 += other.0
    }
}

impl Mul<Vector4F> for Vector4F {
    type Output = Vector4F;
    #[inline]
    fn mul(self, other: Vector4F) -> Vector4F {
        Vector4F(self.0 * other.0)
    }
}

impl Neg for Vector4F {
    type Output = Vector4F;
    /// NB: This does not negate w, because that is rarely what you what for homogeneous
    /// coordinates.
    #[inline]
    fn neg(self) -> Vector4F {
        Vector4F(self.0 * F32x4::new(-1.0, -1.0, -1.0, 1.0))
    }
}

impl Sub<Vector4F> for Vector4F {
    type Output = Vector4F;
    #[inline]
    fn sub(self, other: Vector4F) -> Vector4F {
        Vector4F(self.0 - other.0)
    }
}

impl Default for Vector4F {
    #[inline]
    fn default() -> Vector4F {
        let mut point = F32x4::default();
        point.set_w(1.0);
        Vector4F(point)
    }
}
