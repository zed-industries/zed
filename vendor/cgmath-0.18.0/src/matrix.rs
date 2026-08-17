// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num_traits::{cast, NumCast};
#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;
use std::iter;
use std::mem;
use std::ops::*;
use std::ptr;

use structure::*;

use angle::Rad;
use approx;
use euler::Euler;
use num::BaseFloat;
use point::{Point2, Point3};
use quaternion::Quaternion;
use transform::{Transform, Transform2, Transform3};
use vector::{Vector2, Vector3, Vector4};

#[cfg(feature = "mint")]
use mint;

/// A 2 x 2, column major matrix
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Matrix2<S> {
    /// The first column of the matrix.
    pub x: Vector2<S>,
    /// The second column of the matrix.
    pub y: Vector2<S>,
}

/// A 3 x 3, column major matrix
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Matrix3<S> {
    /// The first column of the matrix.
    pub x: Vector3<S>,
    /// The second column of the matrix.
    pub y: Vector3<S>,
    /// The third column of the matrix.
    pub z: Vector3<S>,
}

/// A 4 x 4, column major matrix
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Matrix4<S> {
    /// The first column of the matrix.
    pub x: Vector4<S>,
    /// The second column of the matrix.
    pub y: Vector4<S>,
    /// The third column of the matrix.
    pub z: Vector4<S>,
    /// The fourth column of the matrix.
    pub w: Vector4<S>,
}

impl<S> Matrix2<S> {
    /// Create a new matrix, providing values for each index.
    #[inline]
    pub const fn new(c0r0: S, c0r1: S, c1r0: S, c1r1: S) -> Matrix2<S> {
        Matrix2::from_cols(Vector2::new(c0r0, c0r1), Vector2::new(c1r0, c1r1))
    }

    /// Create a new matrix, providing columns.
    #[inline]
    pub const fn from_cols(c0: Vector2<S>, c1: Vector2<S>) -> Matrix2<S> {
        Matrix2 { x: c0, y: c1 }
    }
}

impl<S: BaseFloat> Matrix2<S> {
    /// Create a transformation matrix that will cause `unit_x()` to point at
    /// `dir`. `unit_y()` will be perpendicular to `dir`, and the closest to `up`.
    pub fn look_at(dir: Vector2<S>, up: Vector2<S>) -> Matrix2<S> {
        Matrix2::look_at_stable(dir, up.x * dir.y >= up.y * dir.x)
    }

    /// Crate a transformation that will cause `unit_x()` to point at
    /// `dir`. This is similar to `look_at`, but does not take an `up` vector.
    /// This will not cause `unit_y()` to flip when `dir` crosses over the `up` vector.
    pub fn look_at_stable(dir: Vector2<S>, flip: bool) -> Matrix2<S> {
        let basis1 = dir.normalize();
        let basis2 = if flip {
            Vector2::new(basis1.y, -basis1.x)
        } else {
            Vector2::new(-basis1.y, basis1.x)
        };
        Matrix2::from_cols(basis1, basis2)
    }

    #[inline]
    pub fn from_angle<A: Into<Rad<S>>>(theta: A) -> Matrix2<S> {
        let (s, c) = Rad::sin_cos(theta.into());

        Matrix2::new(c, s, -s, c)
    }

    /// Are all entries in the matrix finite.
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl<S> Matrix3<S> {
    /// Create a new matrix, providing values for each index.
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub const fn new(
        c0r0:S, c0r1:S, c0r2:S,
        c1r0:S, c1r1:S, c1r2:S,
        c2r0:S, c2r1:S, c2r2:S,
    ) -> Matrix3<S> {
        Matrix3::from_cols(
            Vector3::new(c0r0, c0r1, c0r2),
            Vector3::new(c1r0, c1r1, c1r2),
            Vector3::new(c2r0, c2r1, c2r2),
        )
    }

    /// Create a new matrix, providing columns.
    #[inline]
    pub const fn from_cols(c0: Vector3<S>, c1: Vector3<S>, c2: Vector3<S>) -> Matrix3<S> {
        Matrix3 {
            x: c0,
            y: c1,
            z: c2,
        }
    }
}

impl<S: BaseFloat> Matrix3<S> {
    /// Create a homogeneous transformation matrix from a translation vector.
    #[inline]
    pub fn from_translation(v: Vector2<S>) -> Matrix3<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            S::one(), S::zero(), S::zero(),
            S::zero(), S::one(), S::zero(),
            v.x, v.y, S::one(),
        )
    }

    /// Create a homogeneous transformation matrix from a scale value.
    #[inline]
    pub fn from_scale(value: S) -> Matrix3<S> {
        Matrix3::from_nonuniform_scale(value, value)
    }

    /// Create a homogeneous transformation matrix from a set of scale values.
    #[inline]
    pub fn from_nonuniform_scale(x: S, y: S) -> Matrix3<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            x, S::zero(), S::zero(),
            S::zero(), y, S::zero(),
            S::zero(), S::zero(), S::one(),
        )
    }

    /// Create a rotation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    #[deprecated = "Use Matrix3::look_to_lh"]
    pub fn look_at(dir: Vector3<S>, up: Vector3<S>) -> Matrix3<S> {
        Matrix3::look_to_lh(dir, up)
    }

    /// Create a rotation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_to_lh(dir: Vector3<S>, up: Vector3<S>) -> Matrix3<S> {
        let dir = dir.normalize();
        let side = up.cross(dir).normalize();
        let up = dir.cross(side).normalize();

        Matrix3::from_cols(side, up, dir).transpose()
    }

    /// Create a rotation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_to_rh(dir: Vector3<S>, up: Vector3<S>) -> Matrix3<S> {
        Matrix3::look_to_lh(-dir, up)
    }

    /// Create a rotation matrix from a rotation around the `x` axis (pitch).
    pub fn from_angle_x<A: Into<Rad<S>>>(theta: A) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = Rad::sin_cos(theta.into());

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            S::one(), S::zero(), S::zero(),
            S::zero(), c, s,
            S::zero(), -s, c,
        )
    }

    /// Create a rotation matrix from a rotation around the `y` axis (yaw).
    pub fn from_angle_y<A: Into<Rad<S>>>(theta: A) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = Rad::sin_cos(theta.into());

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            c, S::zero(), -s,
            S::zero(), S::one(), S::zero(),
            s, S::zero(), c,
        )
    }

    /// Create a rotation matrix from a rotation around the `z` axis (roll).
    pub fn from_angle_z<A: Into<Rad<S>>>(theta: A) -> Matrix3<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = Rad::sin_cos(theta.into());

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            c, s, S::zero(),
            -s, c, S::zero(),
            S::zero(), S::zero(), S::one(),
        )
    }

    /// Create a rotation matrix from an angle around an arbitrary axis.
    ///
    /// The specified axis **must be normalized**, or it represents an invalid rotation.
    pub fn from_axis_angle<A: Into<Rad<S>>>(axis: Vector3<S>, angle: A) -> Matrix3<S> {
        let (s, c) = Rad::sin_cos(angle.into());
        let _1subc = S::one() - c;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            _1subc * axis.x * axis.x + c,
            _1subc * axis.x * axis.y + s * axis.z,
            _1subc * axis.x * axis.z - s * axis.y,

            _1subc * axis.x * axis.y - s * axis.z,
            _1subc * axis.y * axis.y + c,
            _1subc * axis.y * axis.z + s * axis.x,

            _1subc * axis.x * axis.z + s * axis.y,
            _1subc * axis.y * axis.z - s * axis.x,
            _1subc * axis.z * axis.z + c,
        )
    }

    /// Are all entries in the matrix finite.
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl<S> Matrix4<S> {
    /// Create a new matrix, providing values for each index.
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub const fn new(
        c0r0: S, c0r1: S, c0r2: S, c0r3: S,
        c1r0: S, c1r1: S, c1r2: S, c1r3: S,
        c2r0: S, c2r1: S, c2r2: S, c2r3: S,
        c3r0: S, c3r1: S, c3r2: S, c3r3: S,
    ) -> Matrix4<S>  {
        Matrix4::from_cols(
            Vector4::new(c0r0, c0r1, c0r2, c0r3),
            Vector4::new(c1r0, c1r1, c1r2, c1r3),
            Vector4::new(c2r0, c2r1, c2r2, c2r3),
            Vector4::new(c3r0, c3r1, c3r2, c3r3),
        )
    }

    /// Create a new matrix, providing columns.
    #[inline]
    pub const fn from_cols(
        c0: Vector4<S>,
        c1: Vector4<S>,
        c2: Vector4<S>,
        c3: Vector4<S>,
    ) -> Matrix4<S> {
        Matrix4 {
            x: c0,
            y: c1,
            z: c2,
            w: c3,
        }
    }
}

impl<S: BaseFloat> Matrix4<S> {
    /// Create a homogeneous transformation matrix from a translation vector.
    #[inline]
    pub fn from_translation(v: Vector3<S>) -> Matrix4<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            S::one(), S::zero(), S::zero(), S::zero(),
            S::zero(), S::one(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::one(), S::zero(),
            v.x, v.y, v.z, S::one(),
        )
    }

    /// Create a homogeneous transformation matrix from a scale value.
    #[inline]
    pub fn from_scale(value: S) -> Matrix4<S> {
        Matrix4::from_nonuniform_scale(value, value, value)
    }

    /// Create a homogeneous transformation matrix from a set of scale values.
    #[inline]
    pub fn from_nonuniform_scale(x: S, y: S, z: S) -> Matrix4<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            x, S::zero(), S::zero(), S::zero(),
            S::zero(), y, S::zero(), S::zero(),
            S::zero(), S::zero(), z, S::zero(),
            S::zero(), S::zero(), S::zero(), S::one(),
        )
    }

    /// Create a homogeneous transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    #[deprecated = "Use Matrix4::look_to_rh"]
    pub fn look_at_dir(eye: Point3<S>, dir: Vector3<S>, up: Vector3<S>) -> Matrix4<S> {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            s.x.clone(), u.x.clone(), -f.x.clone(), S::zero(),
            s.y.clone(), u.y.clone(), -f.y.clone(), S::zero(),
            s.z.clone(), u.z.clone(), -f.z.clone(), S::zero(),
            -eye.dot(s), -eye.dot(u), eye.dot(f), S::one(),
        )
    }

    /// Create a homogeneous transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_to_rh(eye: Point3<S>, dir: Vector3<S>, up: Vector3<S>) -> Matrix4<S> {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            s.x.clone(), u.x.clone(), -f.x.clone(), S::zero(),
            s.y.clone(), u.y.clone(), -f.y.clone(), S::zero(),
            s.z.clone(), u.z.clone(), -f.z.clone(), S::zero(),
            -eye.dot(s), -eye.dot(u), eye.dot(f), S::one(),
        )
    }

    /// Create a homogeneous transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_to_lh(eye: Point3<S>, dir: Vector3<S>, up: Vector3<S>) -> Matrix4<S> {
        Matrix4::look_to_rh(eye, -dir, up)
    }

    /// Create a homogeneous transformation matrix that will cause a vector to point at
    /// `center`, using `up` for orientation.
    #[deprecated = "Use Matrix4::look_at_rh"]
    pub fn look_at(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix4<S> {
        Matrix4::look_at_rh(eye, center, up)
    }

    /// Create a homogeneous transformation matrix that will cause a vector to point at
    /// `center`, using `up` for orientation.
    pub fn look_at_rh(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix4<S> {
        Matrix4::look_to_rh(eye, center - eye, up)
    }

    /// Create a homogeneous transformation matrix that will cause a vector to point at
    /// `center`, using `up` for orientation.
    pub fn look_at_lh(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix4<S> {
        Matrix4::look_to_lh(eye, center - eye, up)
    }

    /// Create a homogeneous transformation matrix from a rotation around the `x` axis (pitch).
    pub fn from_angle_x<A: Into<Rad<S>>>(theta: A) -> Matrix4<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = Rad::sin_cos(theta.into());

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            S::one(), S::zero(), S::zero(), S::zero(),
            S::zero(), c, s, S::zero(),
            S::zero(), -s, c, S::zero(),
            S::zero(), S::zero(), S::zero(), S::one(),
        )
    }

    /// Create a homogeneous transformation matrix from a rotation around the `y` axis (yaw).
    pub fn from_angle_y<A: Into<Rad<S>>>(theta: A) -> Matrix4<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = Rad::sin_cos(theta.into());

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            c, S::zero(), -s, S::zero(),
            S::zero(), S::one(), S::zero(), S::zero(),
            s, S::zero(), c, S::zero(),
            S::zero(), S::zero(), S::zero(), S::one(),
        )
    }

    /// Create a homogeneous transformation matrix from a rotation around the `z` axis (roll).
    pub fn from_angle_z<A: Into<Rad<S>>>(theta: A) -> Matrix4<S> {
        // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
        let (s, c) = Rad::sin_cos(theta.into());

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            c, s, S::zero(), S::zero(),
            -s, c, S::zero(), S::zero(),
            S::zero(), S::zero(), S::one(), S::zero(),
            S::zero(), S::zero(), S::zero(), S::one(),
        )
    }

    /// Create a homogeneous transformation matrix from an angle around an arbitrary axis.
    ///
    /// The specified axis **must be normalized**, or it represents an invalid rotation.
    pub fn from_axis_angle<A: Into<Rad<S>>>(axis: Vector3<S>, angle: A) -> Matrix4<S> {
        let (s, c) = Rad::sin_cos(angle.into());
        let _1subc = S::one() - c;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            _1subc * axis.x * axis.x + c,
            _1subc * axis.x * axis.y + s * axis.z,
            _1subc * axis.x * axis.z - s * axis.y,
            S::zero(),

            _1subc * axis.x * axis.y - s * axis.z,
            _1subc * axis.y * axis.y + c,
            _1subc * axis.y * axis.z + s * axis.x,
            S::zero(),

            _1subc * axis.x * axis.z + s * axis.y,
            _1subc * axis.y * axis.z - s * axis.x,
            _1subc * axis.z * axis.z + c,
            S::zero(),

            S::zero(), S::zero(), S::zero(), S::one(),
        )
    }

    /// Are all entries in the matrix finite.
    pub fn is_finite(&self) -> bool {
        self.w.is_finite() && self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl<S: BaseFloat> Zero for Matrix2<S> {
    #[inline]
    fn zero() -> Matrix2<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix2::new(
            S::zero(), S::zero(),
            S::zero(), S::zero(),
        )
    }

    #[inline]
    fn is_zero(&self) -> bool {
        ulps_eq!(self, &Self::zero())
    }
}

impl<S: BaseFloat> Zero for Matrix3<S> {
    #[inline]
    fn zero() -> Matrix3<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(),
        )
    }

    #[inline]
    fn is_zero(&self) -> bool {
        ulps_eq!(self, &Self::zero())
    }
}

impl<S: BaseFloat> Zero for Matrix4<S> {
    #[inline]
    fn zero() -> Matrix4<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            S::zero(), S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(), S::zero(),
        )
    }

    #[inline]
    fn is_zero(&self) -> bool {
        ulps_eq!(self, &Self::zero())
    }
}

impl<S: BaseFloat> One for Matrix2<S> {
    #[inline]
    fn one() -> Matrix2<S> {
        Matrix2::from_value(S::one())
    }
}

impl<S: BaseFloat> One for Matrix3<S> {
    #[inline]
    fn one() -> Matrix3<S> {
        Matrix3::from_value(S::one())
    }
}

impl<S: BaseFloat> One for Matrix4<S> {
    #[inline]
    fn one() -> Matrix4<S> {
        Matrix4::from_value(S::one())
    }
}

impl<S: BaseFloat> VectorSpace for Matrix2<S> {
    type Scalar = S;
}

impl<S: BaseFloat> VectorSpace for Matrix3<S> {
    type Scalar = S;
}

impl<S: BaseFloat> VectorSpace for Matrix4<S> {
    type Scalar = S;
}

impl<S: BaseFloat> Matrix for Matrix2<S> {
    type Column = Vector2<S>;
    type Row = Vector2<S>;
    type Transpose = Matrix2<S>;

    #[inline]
    fn row(&self, r: usize) -> Vector2<S> {
        Vector2::new(self[0][r], self[1][r])
    }

    #[inline]
    fn swap_rows(&mut self, a: usize, b: usize) {
        self[0].swap_elements(a, b);
        self[1].swap_elements(a, b);
    }

    #[inline]
    fn swap_columns(&mut self, a: usize, b: usize) {
        unsafe { ptr::swap(&mut self[a], &mut self[b]) };
    }

    #[inline]
    fn swap_elements(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(&mut self[ac][ar], &mut self[bc][br]) };
    }

    fn transpose(&self) -> Matrix2<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix2::new(
            self[0][0], self[1][0],
            self[0][1], self[1][1],
        )
    }
}

impl<S: BaseFloat> SquareMatrix for Matrix2<S> {
    type ColumnRow = Vector2<S>;

    #[inline]
    fn from_value(value: S) -> Matrix2<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix2::new(
            value, S::zero(),
            S::zero(), value,
        )
    }

    #[inline]
    fn from_diagonal(value: Vector2<S>) -> Matrix2<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix2::new(
            value.x, S::zero(),
            S::zero(), value.y,
        )
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_elements((0, 1), (1, 0));
    }

    #[inline]
    fn determinant(&self) -> S {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    #[inline]
    fn diagonal(&self) -> Vector2<S> {
        Vector2::new(self[0][0], self[1][1])
    }

    #[inline]
    fn invert(&self) -> Option<Matrix2<S>> {
        let det = self.determinant();
        if det == S::zero() {
            None
        } else {
            #[cfg_attr(rustfmt, rustfmt_skip)]
            Some(Matrix2::new(
                self[1][1] / det, -self[0][1] / det,
                -self[1][0] / det, self[0][0] / det,
            ))
        }
    }

    #[inline]
    fn is_diagonal(&self) -> bool {
        ulps_eq!(self[0][1], &S::zero()) && ulps_eq!(self[1][0], &S::zero())
    }

    #[inline]
    fn is_symmetric(&self) -> bool {
        ulps_eq!(self[0][1], &self[1][0]) && ulps_eq!(self[1][0], &self[0][1])
    }
}

impl<S: BaseFloat> Matrix for Matrix3<S> {
    type Column = Vector3<S>;
    type Row = Vector3<S>;
    type Transpose = Matrix3<S>;

    #[inline]
    fn row(&self, r: usize) -> Vector3<S> {
        Vector3::new(self[0][r], self[1][r], self[2][r])
    }

    #[inline]
    fn swap_rows(&mut self, a: usize, b: usize) {
        self[0].swap_elements(a, b);
        self[1].swap_elements(a, b);
        self[2].swap_elements(a, b);
    }

    #[inline]
    fn swap_columns(&mut self, a: usize, b: usize) {
        unsafe { ptr::swap(&mut self[a], &mut self[b]) };
    }

    #[inline]
    fn swap_elements(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(&mut self[ac][ar], &mut self[bc][br]) };
    }

    fn transpose(&self) -> Matrix3<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            self[0][0], self[1][0], self[2][0],
            self[0][1], self[1][1], self[2][1],
            self[0][2], self[1][2], self[2][2],
        )
    }
}

impl<S: BaseFloat> SquareMatrix for Matrix3<S> {
    type ColumnRow = Vector3<S>;

    #[inline]
    fn from_value(value: S) -> Matrix3<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            value, S::zero(), S::zero(),
            S::zero(), value, S::zero(),
            S::zero(), S::zero(), value,
        )
    }

    #[inline]
    fn from_diagonal(value: Vector3<S>) -> Matrix3<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            value.x, S::zero(), S::zero(),
            S::zero(), value.y, S::zero(),
            S::zero(), S::zero(), value.z,
        )
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_elements((0, 1), (1, 0));
        self.swap_elements((0, 2), (2, 0));
        self.swap_elements((1, 2), (2, 1));
    }

    fn determinant(&self) -> S {
        self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2])
            - self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2])
            + self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
    }

    #[inline]
    fn diagonal(&self) -> Vector3<S> {
        Vector3::new(self[0][0], self[1][1], self[2][2])
    }

    fn invert(&self) -> Option<Matrix3<S>> {
        let det = self.determinant();
        if det == S::zero() {
            None
        } else {
            Some(
                Matrix3::from_cols(
                    self[1].cross(self[2]) / det,
                    self[2].cross(self[0]) / det,
                    self[0].cross(self[1]) / det,
                )
                .transpose(),
            )
        }
    }

    fn is_diagonal(&self) -> bool {
        ulps_eq!(self[0][1], &S::zero())
            && ulps_eq!(self[0][2], &S::zero())
            && ulps_eq!(self[1][0], &S::zero())
            && ulps_eq!(self[1][2], &S::zero())
            && ulps_eq!(self[2][0], &S::zero())
            && ulps_eq!(self[2][1], &S::zero())
    }

    fn is_symmetric(&self) -> bool {
        ulps_eq!(self[0][1], &self[1][0])
            && ulps_eq!(self[0][2], &self[2][0])
            && ulps_eq!(self[1][0], &self[0][1])
            && ulps_eq!(self[1][2], &self[2][1])
            && ulps_eq!(self[2][0], &self[0][2])
            && ulps_eq!(self[2][1], &self[1][2])
    }
}

impl<S: BaseFloat> Matrix for Matrix4<S> {
    type Column = Vector4<S>;
    type Row = Vector4<S>;
    type Transpose = Matrix4<S>;

    #[inline]
    fn row(&self, r: usize) -> Vector4<S> {
        Vector4::new(self[0][r], self[1][r], self[2][r], self[3][r])
    }

    #[inline]
    fn swap_rows(&mut self, a: usize, b: usize) {
        self[0].swap_elements(a, b);
        self[1].swap_elements(a, b);
        self[2].swap_elements(a, b);
        self[3].swap_elements(a, b);
    }

    #[inline]
    fn swap_columns(&mut self, a: usize, b: usize) {
        unsafe { ptr::swap(&mut self[a], &mut self[b]) };
    }

    #[inline]
    fn swap_elements(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (ac, ar) = a;
        let (bc, br) = b;
        unsafe { ptr::swap(&mut self[ac][ar], &mut self[bc][br]) };
    }

    fn transpose(&self) -> Matrix4<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        )
    }
}

impl<S: BaseFloat> SquareMatrix for Matrix4<S> {
    type ColumnRow = Vector4<S>;

    #[inline]
    fn from_value(value: S) -> Matrix4<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            value, S::zero(), S::zero(), S::zero(),
            S::zero(), value, S::zero(), S::zero(),
            S::zero(), S::zero(), value, S::zero(),
            S::zero(), S::zero(), S::zero(), value,
        )
    }

    #[inline]
    fn from_diagonal(value: Vector4<S>) -> Matrix4<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            value.x, S::zero(), S::zero(), S::zero(),
            S::zero(), value.y, S::zero(), S::zero(),
            S::zero(), S::zero(), value.z, S::zero(),
            S::zero(), S::zero(), S::zero(), value.w,
        )
    }

    fn transpose_self(&mut self) {
        self.swap_elements((0, 1), (1, 0));
        self.swap_elements((0, 2), (2, 0));
        self.swap_elements((0, 3), (3, 0));
        self.swap_elements((1, 2), (2, 1));
        self.swap_elements((1, 3), (3, 1));
        self.swap_elements((2, 3), (3, 2));
    }

    fn determinant(&self) -> S {
        let tmp = unsafe { det_sub_proc_unsafe(self, 1, 2, 3) };
        tmp.dot(Vector4::new(self[0][0], self[1][0], self[2][0], self[3][0]))
    }

    #[inline]
    fn diagonal(&self) -> Vector4<S> {
        Vector4::new(self[0][0], self[1][1], self[2][2], self[3][3])
    }

    // The new implementation results in negative optimization when used
    // without SIMD. so we opt them in with configuration.
    // A better option would be using specialization. But currently somewhat
    // specialization is too buggy, and it won't apply here. I'm getting
    // weird error msgs. Help wanted.
    #[cfg(not(feature = "simd"))]
    fn invert(&self) -> Option<Matrix4<S>> {
        let det = self.determinant();
        if det == S::zero() {
            None
        } else {
            let inv_det = S::one() / det;
            let t = self.transpose();
            let cf = |i, j| {
                let mat = match i {
                    0 => {
                        Matrix3::from_cols(t.y.truncate_n(j), t.z.truncate_n(j), t.w.truncate_n(j))
                    }
                    1 => {
                        Matrix3::from_cols(t.x.truncate_n(j), t.z.truncate_n(j), t.w.truncate_n(j))
                    }
                    2 => {
                        Matrix3::from_cols(t.x.truncate_n(j), t.y.truncate_n(j), t.w.truncate_n(j))
                    }
                    3 => {
                        Matrix3::from_cols(t.x.truncate_n(j), t.y.truncate_n(j), t.z.truncate_n(j))
                    }
                    _ => panic!("out of range"),
                };
                let sign = if (i + j) & 1 == 1 {
                    -S::one()
                } else {
                    S::one()
                };
                mat.determinant() * sign * inv_det
            };

            #[cfg_attr(rustfmt, rustfmt_skip)]
            Some(Matrix4::new(
                cf(0, 0), cf(0, 1), cf(0, 2), cf(0, 3),
                cf(1, 0), cf(1, 1), cf(1, 2), cf(1, 3),
                cf(2, 0), cf(2, 1), cf(2, 2), cf(2, 3),
                cf(3, 0), cf(3, 1), cf(3, 2), cf(3, 3),
            ))
        }
    }
    #[cfg(feature = "simd")]
    fn invert(&self) -> Option<Matrix4<S>> {
        let tmp0 = unsafe { det_sub_proc_unsafe(self, 1, 2, 3) };
        let det = tmp0.dot(Vector4::new(self[0][0], self[1][0], self[2][0], self[3][0]));

        if det == S::zero() {
            None
        } else {
            let inv_det = S::one() / det;
            let tmp0 = tmp0 * inv_det;
            let tmp1 = unsafe { det_sub_proc_unsafe(self, 0, 3, 2) * inv_det };
            let tmp2 = unsafe { det_sub_proc_unsafe(self, 0, 1, 3) * inv_det };
            let tmp3 = unsafe { det_sub_proc_unsafe(self, 0, 2, 1) * inv_det };
            Some(Matrix4::from_cols(tmp0, tmp1, tmp2, tmp3))
        }
    }

    fn is_diagonal(&self) -> bool {
        ulps_eq!(self[0][1], &S::zero())
            && ulps_eq!(self[0][2], &S::zero())
            && ulps_eq!(self[0][3], &S::zero())
            && ulps_eq!(self[1][0], &S::zero())
            && ulps_eq!(self[1][2], &S::zero())
            && ulps_eq!(self[1][3], &S::zero())
            && ulps_eq!(self[2][0], &S::zero())
            && ulps_eq!(self[2][1], &S::zero())
            && ulps_eq!(self[2][3], &S::zero())
            && ulps_eq!(self[3][0], &S::zero())
            && ulps_eq!(self[3][1], &S::zero())
            && ulps_eq!(self[3][2], &S::zero())
    }

    fn is_symmetric(&self) -> bool {
        ulps_eq!(self[0][1], &self[1][0])
            && ulps_eq!(self[0][2], &self[2][0])
            && ulps_eq!(self[0][3], &self[3][0])
            && ulps_eq!(self[1][0], &self[0][1])
            && ulps_eq!(self[1][2], &self[2][1])
            && ulps_eq!(self[1][3], &self[3][1])
            && ulps_eq!(self[2][0], &self[0][2])
            && ulps_eq!(self[2][1], &self[1][2])
            && ulps_eq!(self[2][3], &self[3][2])
            && ulps_eq!(self[3][0], &self[0][3])
            && ulps_eq!(self[3][1], &self[1][3])
            && ulps_eq!(self[3][2], &self[2][3])
    }
}

impl<S: BaseFloat> approx::AbsDiffEq for Matrix2<S> {
    type Epsilon = S::Epsilon;

    #[inline]
    fn default_epsilon() -> S::Epsilon {
        cast(1.0e-6f64).unwrap()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: S::Epsilon) -> bool {
        Vector2::abs_diff_eq(&self[0], &other[0], epsilon)
            && Vector2::abs_diff_eq(&self[1], &other[1], epsilon)
    }
}

impl<S: BaseFloat> approx::RelativeEq for Matrix2<S> {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        Vector2::relative_eq(&self[0], &other[0], epsilon, max_relative)
            && Vector2::relative_eq(&self[1], &other[1], epsilon, max_relative)
    }
}

impl<S: BaseFloat> approx::UlpsEq for Matrix2<S> {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Vector2::ulps_eq(&self[0], &other[0], epsilon, max_ulps)
            && Vector2::ulps_eq(&self[1], &other[1], epsilon, max_ulps)
    }
}

impl<S: BaseFloat> approx::AbsDiffEq for Matrix3<S> {
    type Epsilon = S::Epsilon;

    #[inline]
    fn default_epsilon() -> S::Epsilon {
        cast(1.0e-6f64).unwrap()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: S::Epsilon) -> bool {
        Vector3::abs_diff_eq(&self[0], &other[0], epsilon)
            && Vector3::abs_diff_eq(&self[1], &other[1], epsilon)
            && Vector3::abs_diff_eq(&self[2], &other[2], epsilon)
    }
}

impl<S: BaseFloat> approx::RelativeEq for Matrix3<S> {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        Vector3::relative_eq(&self[0], &other[0], epsilon, max_relative)
            && Vector3::relative_eq(&self[1], &other[1], epsilon, max_relative)
            && Vector3::relative_eq(&self[2], &other[2], epsilon, max_relative)
    }
}

impl<S: BaseFloat> approx::UlpsEq for Matrix3<S> {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Vector3::ulps_eq(&self[0], &other[0], epsilon, max_ulps)
            && Vector3::ulps_eq(&self[1], &other[1], epsilon, max_ulps)
            && Vector3::ulps_eq(&self[2], &other[2], epsilon, max_ulps)
    }
}

impl<S: BaseFloat> approx::AbsDiffEq for Matrix4<S> {
    type Epsilon = S::Epsilon;

    #[inline]
    fn default_epsilon() -> S::Epsilon {
        cast(1.0e-6f64).unwrap()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: S::Epsilon) -> bool {
        Vector4::abs_diff_eq(&self[0], &other[0], epsilon)
            && Vector4::abs_diff_eq(&self[1], &other[1], epsilon)
            && Vector4::abs_diff_eq(&self[2], &other[2], epsilon)
            && Vector4::abs_diff_eq(&self[3], &other[3], epsilon)
    }
}

impl<S: BaseFloat> approx::RelativeEq for Matrix4<S> {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        Vector4::relative_eq(&self[0], &other[0], epsilon, max_relative)
            && Vector4::relative_eq(&self[1], &other[1], epsilon, max_relative)
            && Vector4::relative_eq(&self[2], &other[2], epsilon, max_relative)
            && Vector4::relative_eq(&self[3], &other[3], epsilon, max_relative)
    }
}

impl<S: BaseFloat> approx::UlpsEq for Matrix4<S> {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Vector4::ulps_eq(&self[0], &other[0], epsilon, max_ulps)
            && Vector4::ulps_eq(&self[1], &other[1], epsilon, max_ulps)
            && Vector4::ulps_eq(&self[2], &other[2], epsilon, max_ulps)
            && Vector4::ulps_eq(&self[3], &other[3], epsilon, max_ulps)
    }
}

impl<S: BaseFloat> Transform<Point2<S>> for Matrix3<S> {
    fn look_at(eye: Point2<S>, center: Point2<S>, up: Vector2<S>) -> Matrix3<S> {
        let dir = center - eye;
        Matrix3::from(Matrix2::look_at(dir, up))
    }

    fn look_at_lh(eye: Point2<S>, center: Point2<S>, up: Vector2<S>) -> Matrix3<S> {
        let dir = center - eye;
        Matrix3::from(Matrix2::look_at(dir, up))
    }

    fn look_at_rh(eye: Point2<S>, center: Point2<S>, up: Vector2<S>) -> Matrix3<S> {
        let dir = eye - center;
        Matrix3::from(Matrix2::look_at(dir, up))
    }

    fn transform_vector(&self, vec: Vector2<S>) -> Vector2<S> {
        (self * vec.extend(S::zero())).truncate()
    }

    fn transform_point(&self, point: Point2<S>) -> Point2<S> {
        Point2::from_vec((self * Point3::new(point.x, point.y, S::one()).to_vec()).truncate())
    }

    fn concat(&self, other: &Matrix3<S>) -> Matrix3<S> {
        self * other
    }

    fn inverse_transform(&self) -> Option<Matrix3<S>> {
        SquareMatrix::invert(self)
    }
}

impl<S: BaseFloat> Transform<Point3<S>> for Matrix3<S> {
    fn look_at(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix3<S> {
        let dir = center - eye;
        Matrix3::look_to_lh(dir, up)
    }

    fn look_at_lh(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix3<S> {
        let dir = center - eye;
        Matrix3::look_to_lh(dir, up)
    }

    fn look_at_rh(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix3<S> {
        let dir = center - eye;
        Matrix3::look_to_rh(dir, up)
    }

    fn transform_vector(&self, vec: Vector3<S>) -> Vector3<S> {
        self * vec
    }

    fn transform_point(&self, point: Point3<S>) -> Point3<S> {
        Point3::from_vec(self * point.to_vec())
    }

    fn concat(&self, other: &Matrix3<S>) -> Matrix3<S> {
        self * other
    }

    fn inverse_transform(&self) -> Option<Matrix3<S>> {
        SquareMatrix::invert(self)
    }
}

impl<S: BaseFloat> Transform<Point3<S>> for Matrix4<S> {

    fn look_at(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix4<S> {
        Matrix4::look_at_rh(eye, center, up)
    }

    fn look_at_lh(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix4<S> {
        Matrix4::look_at_lh(eye, center, up)
    }

    fn look_at_rh(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Matrix4<S> {
        Matrix4::look_at_rh(eye, center, up)
    }

    fn transform_vector(&self, vec: Vector3<S>) -> Vector3<S> {
        (self * vec.extend(S::zero())).truncate()
    }

    fn transform_point(&self, point: Point3<S>) -> Point3<S> {
        Point3::from_homogeneous(self * point.to_homogeneous())
    }

    fn concat(&self, other: &Matrix4<S>) -> Matrix4<S> {
        self * other
    }

    fn inverse_transform(&self) -> Option<Matrix4<S>> {
        SquareMatrix::invert(self)
    }
}

impl<S: BaseFloat> Transform2 for Matrix3<S> {
    type Scalar = S;
}

impl<S: BaseFloat> Transform3 for Matrix3<S> {
    type Scalar = S;
}

impl<S: BaseFloat> Transform3 for Matrix4<S> {
    type Scalar = S;
}

macro_rules! impl_matrix {
    ($MatrixN:ident, $VectorN:ident { $($field:ident : $row_index:expr),+ }) => {
        impl_operator!(<S: BaseFloat> Neg for $MatrixN<S> {
            fn neg(matrix) -> $MatrixN<S> { $MatrixN { $($field: -matrix.$field),+ } }
        });

        impl_operator!(<S: BaseFloat> Mul<S> for $MatrixN<S> {
            fn mul(matrix, scalar) -> $MatrixN<S> { $MatrixN { $($field: matrix.$field * scalar),+ } }
        });
        impl_operator!(<S: BaseFloat> Div<S> for $MatrixN<S> {
            fn div(matrix, scalar) -> $MatrixN<S> { $MatrixN { $($field: matrix.$field / scalar),+ } }
        });
        impl_operator!(<S: BaseFloat> Rem<S> for $MatrixN<S> {
            fn rem(matrix, scalar) -> $MatrixN<S> { $MatrixN { $($field: matrix.$field % scalar),+ } }
        });
        impl_assignment_operator!(<S: BaseFloat> MulAssign<S> for $MatrixN<S> {
            fn mul_assign(&mut self, scalar) { $(self.$field *= scalar);+ }
        });
        impl_assignment_operator!(<S: BaseFloat> DivAssign<S> for $MatrixN<S> {
            fn div_assign(&mut self, scalar) { $(self.$field /= scalar);+ }
        });
        impl_assignment_operator!(<S: BaseFloat> RemAssign<S> for $MatrixN<S> {
            fn rem_assign(&mut self, scalar) { $(self.$field %= scalar);+ }
        });

        impl_operator!(<S: BaseFloat> Add<$MatrixN<S> > for $MatrixN<S> {
            fn add(lhs, rhs) -> $MatrixN<S> { $MatrixN { $($field: lhs.$field + rhs.$field),+ } }
        });
        impl_operator!(<S: BaseFloat> Sub<$MatrixN<S> > for $MatrixN<S> {
            fn sub(lhs, rhs) -> $MatrixN<S> { $MatrixN { $($field: lhs.$field - rhs.$field),+ } }
        });
        impl<S: BaseFloat + AddAssign<S>> AddAssign<$MatrixN<S>> for $MatrixN<S> {
            fn add_assign(&mut self, other: $MatrixN<S>) { $(self.$field += other.$field);+ }
        }
        impl<S: BaseFloat + SubAssign<S>> SubAssign<$MatrixN<S>> for $MatrixN<S> {
            fn sub_assign(&mut self, other: $MatrixN<S>) { $(self.$field -= other.$field);+ }
        }

        impl<S: BaseFloat> iter::Sum<$MatrixN<S>> for $MatrixN<S> {
            #[inline]
            fn sum<I: Iterator<Item=$MatrixN<S>>>(iter: I) -> $MatrixN<S> {
                iter.fold($MatrixN::zero(), Add::add)
            }
        }

        impl<'a, S: 'a + BaseFloat> iter::Sum<&'a $MatrixN<S>> for $MatrixN<S> {
            #[inline]
            fn sum<I: Iterator<Item=&'a $MatrixN<S>>>(iter: I) -> $MatrixN<S> {
                iter.fold($MatrixN::zero(), Add::add)
            }
        }

        impl<S: BaseFloat> iter::Product for $MatrixN<S> {
            #[inline]
            fn product<I: Iterator<Item=$MatrixN<S>>>(iter: I) -> $MatrixN<S> {
                iter.fold($MatrixN::identity(), Mul::mul)
            }
        }

        impl<'a, S: 'a + BaseFloat> iter::Product<&'a $MatrixN<S>> for $MatrixN<S> {
            #[inline]
            fn product<I: Iterator<Item=&'a $MatrixN<S>>>(iter: I) -> $MatrixN<S> {
                iter.fold($MatrixN::identity(), Mul::mul)
            }
        }

        impl_scalar_ops!($MatrixN<usize> { $($field),+ });
        impl_scalar_ops!($MatrixN<u8> { $($field),+ });
        impl_scalar_ops!($MatrixN<u16> { $($field),+ });
        impl_scalar_ops!($MatrixN<u32> { $($field),+ });
        impl_scalar_ops!($MatrixN<u64> { $($field),+ });
        impl_scalar_ops!($MatrixN<isize> { $($field),+ });
        impl_scalar_ops!($MatrixN<i8> { $($field),+ });
        impl_scalar_ops!($MatrixN<i16> { $($field),+ });
        impl_scalar_ops!($MatrixN<i32> { $($field),+ });
        impl_scalar_ops!($MatrixN<i64> { $($field),+ });
        impl_scalar_ops!($MatrixN<f32> { $($field),+ });
        impl_scalar_ops!($MatrixN<f64> { $($field),+ });


        impl<S: NumCast + Copy> $MatrixN<S> {
            /// Component-wise casting to another type
            #[inline]
            pub fn cast<T: NumCast>(&self) -> Option<$MatrixN<T>> {
                $(
                    let $field = match self.$field.cast() {
                        Some(field) => field,
                        None => return None
                    };
                )+
                Some($MatrixN { $($field),+ })
            }
        }
    }
}

macro_rules! impl_scalar_ops {
    ($MatrixN:ident<$S:ident> { $($field:ident),+ }) => {
        impl_operator!(Mul<$MatrixN<$S>> for $S {
            fn mul(scalar, matrix) -> $MatrixN<$S> { $MatrixN { $($field: scalar * matrix.$field),+ } }
        });
        impl_operator!(Div<$MatrixN<$S>> for $S {
            fn div(scalar, matrix) -> $MatrixN<$S> { $MatrixN { $($field: scalar / matrix.$field),+ } }
        });
        impl_operator!(Rem<$MatrixN<$S>> for $S {
            fn rem(scalar, matrix) -> $MatrixN<$S> { $MatrixN { $($field: scalar % matrix.$field),+ } }
        });
    };
}

impl_matrix!(Matrix2, Vector2 { x: 0, y: 1 });
impl_matrix!(Matrix3, Vector3 { x: 0, y: 1, z: 2 });
#[cfg_attr(rustfmt, rustfmt_skip)]
impl_matrix!(Matrix4, Vector4 { x: 0, y: 1, z: 2, w: 3 });

macro_rules! impl_mv_operator {
    ($MatrixN:ident, $VectorN:ident { $($field:ident : $row_index:expr),+ }) => {
        impl_operator!(<S: BaseFloat> Mul<$VectorN<S> > for $MatrixN<S> {
            fn mul(matrix, vector) -> $VectorN<S> {$VectorN::new($(matrix.row($row_index).dot(vector.clone())),+)}
        });
    }
}

impl_mv_operator!(Matrix2, Vector2 { x: 0, y: 1 });
impl_mv_operator!(Matrix3, Vector3 { x: 0, y: 1, z: 2 });
#[cfg(not(feature = "simd"))]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl_mv_operator!(Matrix4, Vector4 { x: 0, y: 1, z: 2, w: 3 });

#[cfg(feature = "simd")]
impl_operator!(<S: BaseFloat> Mul<Vector4<S> > for Matrix4<S> {
    fn mul(matrix, vector) -> Vector4<S> {
        matrix[0] * vector[0] + matrix[1] * vector[1] + matrix[2] * vector[2] + matrix[3] * vector[3]
    }
});

impl_operator!(<S: BaseFloat> Mul<Matrix2<S> > for Matrix2<S> {
    fn mul(lhs, rhs) -> Matrix2<S> {
        Matrix2::new(lhs.row(0).dot(rhs[0]), lhs.row(1).dot(rhs[0]),
                     lhs.row(0).dot(rhs[1]), lhs.row(1).dot(rhs[1]))
    }
});

impl_operator!(<S: BaseFloat> Mul<Matrix3<S> > for Matrix3<S> {
    fn mul(lhs, rhs) -> Matrix3<S> {
        Matrix3::new(lhs.row(0).dot(rhs[0]), lhs.row(1).dot(rhs[0]), lhs.row(2).dot(rhs[0]),
                     lhs.row(0).dot(rhs[1]), lhs.row(1).dot(rhs[1]), lhs.row(2).dot(rhs[1]),
                     lhs.row(0).dot(rhs[2]), lhs.row(1).dot(rhs[2]), lhs.row(2).dot(rhs[2]))
    }
});

// Using self.row(0).dot(other[0]) like the other matrix multiplies
// causes the LLVM to miss identical loads and multiplies. This optimization
// causes the code to be auto vectorized properly increasing the performance
// around ~4 times.
// Update: this should now be a bit more efficient

impl_operator!(<S: BaseFloat> Mul<Matrix4<S> > for Matrix4<S> {
    fn mul(lhs, rhs) -> Matrix4<S> {
        {
            let a = lhs[0];
            let b = lhs[1];
            let c = lhs[2];
            let d = lhs[3];

            #[cfg_attr(rustfmt, rustfmt_skip)]
            Matrix4::from_cols(
                a*rhs[0][0] + b*rhs[0][1] + c*rhs[0][2] + d*rhs[0][3],
                a*rhs[1][0] + b*rhs[1][1] + c*rhs[1][2] + d*rhs[1][3],
                a*rhs[2][0] + b*rhs[2][1] + c*rhs[2][2] + d*rhs[2][3],
                a*rhs[3][0] + b*rhs[3][1] + c*rhs[3][2] + d*rhs[3][3],
            )
        }
    }
});

macro_rules! index_operators {
    ($MatrixN:ident<$S:ident>, $n:expr, $Output:ty, $I:ty) => {
        impl<$S> Index<$I> for $MatrixN<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[[$S; $n]; $n] = self.as_ref();
                From::from(&v[i])
            }
        }

        impl<$S> IndexMut<$I> for $MatrixN<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [[$S; $n]; $n] = self.as_mut();
                From::from(&mut v[i])
            }
        }
    };
}

index_operators!(Matrix2<S>, 2, Vector2<S>, usize);
index_operators!(Matrix3<S>, 3, Vector3<S>, usize);
index_operators!(Matrix4<S>, 4, Vector4<S>, usize);
// index_operators!(Matrix2<S>, 2, [Vector2<S>], Range<usize>);
// index_operators!(Matrix3<S>, 3, [Vector3<S>], Range<usize>);
// index_operators!(Matrix4<S>, 4, [Vector4<S>], Range<usize>);
// index_operators!(Matrix2<S>, 2, [Vector2<S>], RangeTo<usize>);
// index_operators!(Matrix3<S>, 3, [Vector3<S>], RangeTo<usize>);
// index_operators!(Matrix4<S>, 4, [Vector4<S>], RangeTo<usize>);
// index_operators!(Matrix2<S>, 2, [Vector2<S>], RangeFrom<usize>);
// index_operators!(Matrix3<S>, 3, [Vector3<S>], RangeFrom<usize>);
// index_operators!(Matrix4<S>, 4, [Vector4<S>], RangeFrom<usize>);
// index_operators!(Matrix2<S>, 2, [Vector2<S>], RangeFull);
// index_operators!(Matrix3<S>, 3, [Vector3<S>], RangeFull);
// index_operators!(Matrix4<S>, 4, [Vector4<S>], RangeFull);

impl<A> From<Euler<A>> for Matrix3<A::Unitless>
where
    A: Angle + Into<Rad<<A as Angle>::Unitless>>,
{
    fn from(src: Euler<A>) -> Matrix3<A::Unitless> {
        // Page A-2: http://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770024290.pdf
        let (sx, cx) = Rad::sin_cos(src.x.into());
        let (sy, cy) = Rad::sin_cos(src.y.into());
        let (sz, cz) = Rad::sin_cos(src.z.into());

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            cy * cz, cx * sz + sx * sy * cz, sx * sz - cx * sy * cz,
            -cy * sz, cx * cz - sx * sy * sz, sx * cz + cx * sy * sz,
            sy, -sx * cy, cx * cy,
        )
    }
}

impl<A> From<Euler<A>> for Matrix4<A::Unitless>
where
    A: Angle + Into<Rad<<A as Angle>::Unitless>>,
{
    fn from(src: Euler<A>) -> Matrix4<A::Unitless> {
        // Page A-2: http://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770024290.pdf
        let (sx, cx) = Rad::sin_cos(src.x.into());
        let (sy, cy) = Rad::sin_cos(src.y.into());
        let (sz, cz) = Rad::sin_cos(src.z.into());

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            cy * cz, cx * sz + sx * sy * cz, sx * sz - cx * sy * cz, A::Unitless::zero(),
            -cy * sz, cx * cz - sx * sy * sz, sx * cz + cx * sy * sz, A::Unitless::zero(),
            sy, -sx * cy, cx * cy, A::Unitless::zero(),
            A::Unitless::zero(), A::Unitless::zero(), A::Unitless::zero(), A::Unitless::one(),
        )
    }
}

macro_rules! fixed_array_conversions {
    ($MatrixN:ident <$S:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {
        impl<$S> Into<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn into(self) -> [[$S; $n]; $n] {
                match self { $MatrixN { $($field),+ } => [$($field.into()),+] }
            }
        }

        impl<$S> AsRef<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn as_ref(&self) -> &[[$S; $n]; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [[$S; $n]; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S: Copy> From<[[$S; $n]; $n]> for $MatrixN<$S> {
            #[inline]
            fn from(m: [[$S; $n]; $n]) -> $MatrixN<$S> {
                // We need to use a copy here because we can't pattern match on arrays yet
                $MatrixN { $($field: From::from(m[$index])),+ }
            }
        }

        impl<'a, $S> From<&'a [[$S; $n]; $n]> for &'a $MatrixN<$S> {
            #[inline]
            fn from(m: &'a [[$S; $n]; $n]) -> &'a $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }

        impl<'a, $S> From<&'a mut [[$S; $n]; $n]> for &'a mut $MatrixN<$S> {
            #[inline]
            fn from(m: &'a mut [[$S; $n]; $n]) -> &'a mut $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }

        // impl<$S> Into<[$S; ($n * $n)]> for $MatrixN<$S> {
        //     #[inline]
        //     fn into(self) -> [[$S; $n]; $n] {
        //         // TODO: Not sure how to implement this...
        //         unimplemented!()
        //     }
        // }

        impl<$S> AsRef<[$S; ($n * $n)]> for $MatrixN<$S> {
            #[inline]
            fn as_ref(&self) -> &[$S; ($n * $n)] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[$S; ($n * $n)]> for $MatrixN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$S; ($n * $n)] {
                unsafe { mem::transmute(self) }
            }
        }

        // impl<$S> From<[$S; ($n * $n)]> for $MatrixN<$S> {
        //     #[inline]
        //     fn from(m: [$S; ($n * $n)]) -> $MatrixN<$S> {
        //         // TODO: Not sure how to implement this...
        //         unimplemented!()
        //     }
        // }

        impl<'a, $S> From<&'a [$S; ($n * $n)]> for &'a $MatrixN<$S> {
            #[inline]
            fn from(m: &'a [$S; ($n * $n)]) -> &'a $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }

        impl<'a, $S> From<&'a mut [$S; ($n * $n)]> for &'a mut $MatrixN<$S> {
            #[inline]
            fn from(m: &'a mut [$S; ($n * $n)]) -> &'a mut $MatrixN<$S> {
                unsafe { mem::transmute(m) }
            }
        }
    }
}

fixed_array_conversions!(Matrix2<S> { x:0, y:1 }, 2);
fixed_array_conversions!(Matrix3<S> { x:0, y:1, z:2 }, 3);
fixed_array_conversions!(Matrix4<S> { x:0, y:1, z:2, w:3 }, 4);

#[cfg(feature = "mint")]
macro_rules! mint_conversions {
    ($MatrixN:ident { $($field:ident),+ }, $MintN:ident) => {
        impl<S: Clone> Into<mint::$MintN<S>> for $MatrixN<S> {
            #[inline]
            fn into(self) -> mint::$MintN<S> {
                mint::$MintN { $($field: self.$field.into()),+ }
            }
        }

        impl<S> From<mint::$MintN<S>> for $MatrixN<S> {
            #[inline]
            fn from(m: mint::$MintN<S>) -> Self {
                $MatrixN { $($field: m.$field.into()),+ }
            }
        }

    }
}

#[cfg(feature = "mint")]
mint_conversions!(Matrix2 { x, y }, ColumnMatrix2);
#[cfg(feature = "mint")]
mint_conversions!(Matrix3 { x, y, z }, ColumnMatrix3);
#[cfg(feature = "mint")]
mint_conversions!(Matrix4 { x, y, z, w }, ColumnMatrix4);

impl<S: BaseFloat> From<Matrix2<S>> for Matrix3<S> {
    /// Clone the elements of a 2-dimensional matrix into the top-left corner
    /// of a 3-dimensional identity matrix.
    fn from(m: Matrix2<S>) -> Matrix3<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            m[0][0], m[0][1], S::zero(),
            m[1][0], m[1][1], S::zero(),
            S::zero(), S::zero(), S::one(),
        )
    }
}

impl<S: BaseFloat> From<Matrix2<S>> for Matrix4<S> {
    /// Clone the elements of a 2-dimensional matrix into the top-left corner
    /// of a 4-dimensional identity matrix.
    fn from(m: Matrix2<S>) -> Matrix4<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            m[0][0], m[0][1], S::zero(), S::zero(),
            m[1][0], m[1][1], S::zero(), S::zero(),
            S::zero(), S::zero(), S::one(), S::zero(),
            S::zero(), S::zero(), S::zero(), S::one(),
        )
    }
}

impl<S: BaseFloat> From<Matrix3<S>> for Matrix4<S> {
    /// Clone the elements of a 3-dimensional matrix into the top-left corner
    /// of a 4-dimensional identity matrix.
    fn from(m: Matrix3<S>) -> Matrix4<S> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            m[0][0], m[0][1], m[0][2], S::zero(),
            m[1][0], m[1][1], m[1][2], S::zero(),
            m[2][0], m[2][1], m[2][2], S::zero(),
            S::zero(), S::zero(), S::zero(), S::one(),
        )
    }
}

impl<S: BaseFloat> From<Matrix3<S>> for Quaternion<S> {
    /// Convert the matrix to a quaternion
    fn from(mat: Matrix3<S>) -> Quaternion<S> {
        // http://www.cs.ucr.edu/~vbz/resources/quatut.pdf
        let trace = mat.trace();
        let half: S = cast(0.5f64).unwrap();

        if trace >= S::zero() {
            let s = (S::one() + trace).sqrt();
            let w = half * s;
            let s = half / s;
            let x = (mat[1][2] - mat[2][1]) * s;
            let y = (mat[2][0] - mat[0][2]) * s;
            let z = (mat[0][1] - mat[1][0]) * s;
            Quaternion::new(w, x, y, z)
        } else if (mat[0][0] > mat[1][1]) && (mat[0][0] > mat[2][2]) {
            let s = ((mat[0][0] - mat[1][1] - mat[2][2]) + S::one()).sqrt();
            let x = half * s;
            let s = half / s;
            let y = (mat[1][0] + mat[0][1]) * s;
            let z = (mat[0][2] + mat[2][0]) * s;
            let w = (mat[1][2] - mat[2][1]) * s;
            Quaternion::new(w, x, y, z)
        } else if mat[1][1] > mat[2][2] {
            let s = ((mat[1][1] - mat[0][0] - mat[2][2]) + S::one()).sqrt();
            let y = half * s;
            let s = half / s;
            let z = (mat[2][1] + mat[1][2]) * s;
            let x = (mat[1][0] + mat[0][1]) * s;
            let w = (mat[2][0] - mat[0][2]) * s;
            Quaternion::new(w, x, y, z)
        } else {
            let s = ((mat[2][2] - mat[0][0] - mat[1][1]) + S::one()).sqrt();
            let z = half * s;
            let s = half / s;
            let x = (mat[0][2] + mat[2][0]) * s;
            let y = (mat[2][1] + mat[1][2]) * s;
            let w = (mat[0][1] - mat[1][0]) * s;
            Quaternion::new(w, x, y, z)
        }
    }
}

impl<S: fmt::Debug> fmt::Debug for Matrix2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix2 ")?;
        <[[S; 2]; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Matrix3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix3 ")?;
        <[[S; 3]; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S: fmt::Debug> fmt::Debug for Matrix4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix4 ")?;
        <[[S; 4]; 4] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

#[cfg(feature = "rand")]
impl<S> Distribution<Matrix2<S>> for Standard
where
    Standard: Distribution<Vector2<S>>,
    S: BaseFloat,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix2<S> {
        Matrix2 {
            x: self.sample(rng),
            y: self.sample(rng),
        }
    }
}

#[cfg(feature = "rand")]
impl<S> Distribution<Matrix3<S>> for Standard
where
    Standard: Distribution<Vector3<S>>,
    S: BaseFloat,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix3<S> {
        Matrix3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
}

#[cfg(feature = "rand")]
impl<S> Distribution<Matrix4<S>> for Standard
where
    Standard: Distribution<Vector4<S>>,
    S: BaseFloat,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix4<S> {
        Matrix4 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
            w: rng.gen(),
        }
    }
}

// Sub procedure for SIMD when dealing with determinant and inversion
#[inline]
unsafe fn det_sub_proc_unsafe<S: BaseFloat>(
    m: &Matrix4<S>,
    x: usize,
    y: usize,
    z: usize,
) -> Vector4<S> {
    let s: &[S; 16] = m.as_ref();
    let a = Vector4::new(
        *s.get_unchecked(4 + x),
        *s.get_unchecked(12 + x),
        *s.get_unchecked(x),
        *s.get_unchecked(8 + x),
    );
    let b = Vector4::new(
        *s.get_unchecked(8 + y),
        *s.get_unchecked(8 + y),
        *s.get_unchecked(4 + y),
        *s.get_unchecked(4 + y),
    );
    let c = Vector4::new(
        *s.get_unchecked(12 + z),
        *s.get_unchecked(z),
        *s.get_unchecked(12 + z),
        *s.get_unchecked(z),
    );

    let d = Vector4::new(
        *s.get_unchecked(8 + x),
        *s.get_unchecked(8 + x),
        *s.get_unchecked(4 + x),
        *s.get_unchecked(4 + x),
    );
    let e = Vector4::new(
        *s.get_unchecked(12 + y),
        *s.get_unchecked(y),
        *s.get_unchecked(12 + y),
        *s.get_unchecked(y),
    );
    let f = Vector4::new(
        *s.get_unchecked(4 + z),
        *s.get_unchecked(12 + z),
        *s.get_unchecked(z),
        *s.get_unchecked(8 + z),
    );

    let g = Vector4::new(
        *s.get_unchecked(12 + x),
        *s.get_unchecked(x),
        *s.get_unchecked(12 + x),
        *s.get_unchecked(x),
    );
    let h = Vector4::new(
        *s.get_unchecked(4 + y),
        *s.get_unchecked(12 + y),
        *s.get_unchecked(y),
        *s.get_unchecked(8 + y),
    );
    let i = Vector4::new(
        *s.get_unchecked(8 + z),
        *s.get_unchecked(8 + z),
        *s.get_unchecked(4 + z),
        *s.get_unchecked(4 + z),
    );
    let mut tmp = a.mul_element_wise(b.mul_element_wise(c));
    tmp += d.mul_element_wise(e.mul_element_wise(f));
    tmp += g.mul_element_wise(h.mul_element_wise(i));
    tmp -= a.mul_element_wise(e.mul_element_wise(i));
    tmp -= d.mul_element_wise(h.mul_element_wise(c));
    tmp -= g.mul_element_wise(b.mul_element_wise(f));
    tmp
}
