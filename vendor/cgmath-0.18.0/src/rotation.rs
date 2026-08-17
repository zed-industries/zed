// Copyright 2014 The CGMath Developers. For a full listing of the authors,
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

use std::fmt;
use std::iter;
use std::ops::*;

use structure::*;

use angle::Rad;
use approx;
use euler::Euler;
use matrix::{Matrix2, Matrix3};
use num::BaseFloat;
use point::{Point2, Point3};
use quaternion::Quaternion;
use vector::{Vector2, Vector3};

/// A trait for a generic rotation. A rotation is a transformation that
/// creates a circular motion, and preserves at least one point in the space.
pub trait Rotation: Sized + Copy + One
where
    // FIXME: Ugly type signatures - blocked by rust-lang/rust#24092
    Self: approx::AbsDiffEq<Epsilon = <<Self as Rotation>::Space as EuclideanSpace>::Scalar>,
    Self: approx::RelativeEq<Epsilon = <<Self as Rotation>::Space as EuclideanSpace>::Scalar>,
    Self: approx::UlpsEq<Epsilon = <<Self as Rotation>::Space as EuclideanSpace>::Scalar>,
    <Self::Space as EuclideanSpace>::Scalar: BaseFloat,
    Self: iter::Product<Self>,
{
    type Space: EuclideanSpace;

    /// Create a rotation to a given direction with an 'up' vector.
    fn look_at(
        dir: <Self::Space as EuclideanSpace>::Diff,
        up: <Self::Space as EuclideanSpace>::Diff,
    ) -> Self;

    /// Create a shortest rotation to transform vector 'a' into 'b'.
    /// Both given vectors are assumed to have unit length.
    fn between_vectors(
        a: <Self::Space as EuclideanSpace>::Diff,
        b: <Self::Space as EuclideanSpace>::Diff,
    ) -> Self;

    /// Rotate a vector using this rotation.
    fn rotate_vector(
        &self,
        vec: <Self::Space as EuclideanSpace>::Diff,
    ) -> <Self::Space as EuclideanSpace>::Diff;

    /// Rotate a point using this rotation, by converting it to its
    /// representation as a vector.
    #[inline]
    fn rotate_point(&self, point: Self::Space) -> Self::Space {
        Self::Space::from_vec(self.rotate_vector(point.to_vec()))
    }

    /// Create a new rotation which "un-does" this rotation. That is,
    /// `r * r.invert()` is the identity.
    fn invert(&self) -> Self;
}

/// A two-dimensional rotation.
pub trait Rotation2:
    Rotation<Space = Point2<<Self as Rotation2>::Scalar>>
    + Into<Matrix2<<Self as Rotation2>::Scalar>>
    + Into<Basis2<<Self as Rotation2>::Scalar>>
{
    type Scalar: BaseFloat;

    /// Create a rotation by a given angle. Thus is a redundant case of both
    /// from_axis_angle() and from_euler() for 2D space.
    fn from_angle<A: Into<Rad<Self::Scalar>>>(theta: A) -> Self;
}

/// A three-dimensional rotation.
pub trait Rotation3:
    Rotation<Space = Point3<<Self as Rotation3>::Scalar>>
    + Into<Matrix3<<Self as Rotation3>::Scalar>>
    + Into<Basis3<<Self as Rotation3>::Scalar>>
    + Into<Quaternion<<Self as Rotation3>::Scalar>>
    + From<Euler<Rad<<Self as Rotation3>::Scalar>>>
{
    type Scalar: BaseFloat;

    /// Create a rotation using an angle around a given axis.
    ///
    /// The specified axis **must be normalized**, or it represents an invalid rotation.
    fn from_axis_angle<A: Into<Rad<Self::Scalar>>>(axis: Vector3<Self::Scalar>, angle: A) -> Self;

    /// Create a rotation from an angle around the `x` axis (pitch).
    #[inline]
    fn from_angle_x<A: Into<Rad<Self::Scalar>>>(theta: A) -> Self {
        Rotation3::from_axis_angle(Vector3::unit_x(), theta)
    }

    /// Create a rotation from an angle around the `y` axis (yaw).
    #[inline]
    fn from_angle_y<A: Into<Rad<Self::Scalar>>>(theta: A) -> Self {
        Rotation3::from_axis_angle(Vector3::unit_y(), theta)
    }

    /// Create a rotation from an angle around the `z` axis (roll).
    #[inline]
    fn from_angle_z<A: Into<Rad<Self::Scalar>>>(theta: A) -> Self {
        Rotation3::from_axis_angle(Vector3::unit_z(), theta)
    }
}

/// A two-dimensional rotation matrix.
///
/// The matrix is guaranteed to be orthogonal, so some operations can be
/// implemented more efficiently than the implementations for `math::Matrix2`. To
/// enforce orthogonality at the type level the operations have been restricted
/// to a subset of those implemented on `Matrix2`.
///
/// ## Example
///
/// Suppose we want to rotate a vector that lies in the x-y plane by some
/// angle. We can accomplish this quite easily with a two-dimensional rotation
/// matrix:
///
/// ```no_run
/// use cgmath::Rad;
/// use cgmath::Vector2;
/// use cgmath::{Matrix, Matrix2};
/// use cgmath::{Rotation, Rotation2, Basis2};
/// use cgmath::UlpsEq;
/// use std::f64;
///
/// // For simplicity, we will rotate the unit x vector to the unit y vector --
/// // so the angle is 90 degrees, or π/2.
/// let unit_x: Vector2<f64> = Vector2::unit_x();
/// let rot: Basis2<f64> = Rotation2::from_angle(Rad(0.5f64 * f64::consts::PI));
///
/// // Rotate the vector using the two-dimensional rotation matrix:
/// let unit_y = rot.rotate_vector(unit_x);
///
/// // Since sin(π/2) may not be exactly zero due to rounding errors, we can
/// // use approx's assert_ulps_eq!() feature to show that it is close enough.
/// // assert_ulps_eq!(&unit_y, &Vector2::unit_y()); // TODO: Figure out how to use this
///
/// // This is exactly equivalent to using the raw matrix itself:
/// let unit_y2: Matrix2<_> = rot.into();
/// let unit_y2 = unit_y2 * unit_x;
/// assert_eq!(unit_y2, unit_y);
///
/// // Note that we can also concatenate rotations:
/// let rot_half: Basis2<f64> = Rotation2::from_angle(Rad(0.25f64 * f64::consts::PI));
/// let unit_y3 = (rot_half * rot_half).rotate_vector(unit_x);
/// // assert_ulps_eq!(&unit_y3, &unit_y2); // TODO: Figure out how to use this
/// ```
#[derive(PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Basis2<S> {
    mat: Matrix2<S>,
}

impl<S: BaseFloat> Basis2<S> {
    pub fn look_at_stable(dir: Vector2<S>, flip: bool) -> Basis2<S> {
        Basis2 {
            mat: Matrix2::look_at_stable(dir, flip),
        }
    }
}

impl<S: BaseFloat> AsRef<Matrix2<S>> for Basis2<S> {
    #[inline]
    fn as_ref(&self) -> &Matrix2<S> {
        &self.mat
    }
}

impl<S: BaseFloat> From<Basis2<S>> for Matrix2<S> {
    #[inline]
    fn from(b: Basis2<S>) -> Matrix2<S> {
        b.mat
    }
}

impl<S: BaseFloat> iter::Product<Basis2<S>> for Basis2<S> {
    #[inline]
    fn product<I: Iterator<Item = Basis2<S>>>(iter: I) -> Basis2<S> {
        iter.fold(Basis2::one(), Mul::mul)
    }
}

impl<'a, S: 'a + BaseFloat> iter::Product<&'a Basis2<S>> for Basis2<S> {
    #[inline]
    fn product<I: Iterator<Item = &'a Basis2<S>>>(iter: I) -> Basis2<S> {
        iter.fold(Basis2::one(), Mul::mul)
    }
}

impl<S: BaseFloat> Rotation for Basis2<S> {
    type Space = Point2<S>;

    #[inline]
    fn look_at(dir: Vector2<S>, up: Vector2<S>) -> Basis2<S> {
        Basis2 {
            mat: Matrix2::look_at(dir, up),
        }
    }

    #[inline]
    fn between_vectors(a: Vector2<S>, b: Vector2<S>) -> Basis2<S> {
        Rotation2::from_angle(Rad::acos(a.dot(b)))
    }

    #[inline]
    fn rotate_vector(&self, vec: Vector2<S>) -> Vector2<S> {
        self.mat * vec
    }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert(&self) -> Basis2<S> {
        Basis2 {
            mat: self.mat.invert().unwrap(),
        }
    }
}

impl<S: BaseFloat> One for Basis2<S> {
    #[inline]
    fn one() -> Basis2<S> {
        Basis2 {
            mat: Matrix2::one(),
        }
    }
}

impl_operator!(<S: BaseFloat> Mul<Basis2<S> > for Basis2<S> {
    fn mul(lhs, rhs) -> Basis2<S> { Basis2 { mat: lhs.mat * rhs.mat  } }
});

impl<S: BaseFloat> approx::AbsDiffEq for Basis2<S> {
    type Epsilon = S::Epsilon;

    #[inline]
    fn default_epsilon() -> S::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: S::Epsilon) -> bool {
        Matrix2::abs_diff_eq(&self.mat, &other.mat, epsilon)
    }
}

impl<S: BaseFloat> approx::RelativeEq for Basis2<S> {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        Matrix2::relative_eq(&self.mat, &other.mat, epsilon, max_relative)
    }
}

impl<S: BaseFloat> approx::UlpsEq for Basis2<S> {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Matrix2::ulps_eq(&self.mat, &other.mat, epsilon, max_ulps)
    }
}

impl<S: BaseFloat> Rotation2 for Basis2<S> {
    type Scalar = S;

    fn from_angle<A: Into<Rad<S>>>(theta: A) -> Basis2<S> {
        Basis2 {
            mat: Matrix2::from_angle(theta),
        }
    }
}

impl<S: fmt::Debug> fmt::Debug for Basis2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Basis2 ")?;
        <[[S; 2]; 2] as fmt::Debug>::fmt(self.mat.as_ref(), f)
    }
}

/// A three-dimensional rotation matrix.
///
/// The matrix is guaranteed to be orthogonal, so some operations, specifically
/// inversion, can be implemented more efficiently than the implementations for
/// `math::Matrix3`. To ensure orthogonality is maintained, the operations have
/// been restricted to a subset of those implemented on `Matrix3`.
#[derive(PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Basis3<S> {
    mat: Matrix3<S>,
}

impl<S: BaseFloat> Basis3<S> {
    /// Create a new rotation matrix from a quaternion.
    #[inline]
    pub fn from_quaternion(quaternion: &Quaternion<S>) -> Basis3<S> {
        Basis3 {
            mat: quaternion.clone().into(),
        }
    }
}

impl<S> AsRef<Matrix3<S>> for Basis3<S> {
    #[inline]
    fn as_ref(&self) -> &Matrix3<S> {
        &self.mat
    }
}

impl<S: BaseFloat> From<Basis3<S>> for Matrix3<S> {
    #[inline]
    fn from(b: Basis3<S>) -> Matrix3<S> {
        b.mat
    }
}

impl<S: BaseFloat> From<Basis3<S>> for Quaternion<S> {
    #[inline]
    fn from(b: Basis3<S>) -> Quaternion<S> {
        b.mat.into()
    }
}

impl<S: BaseFloat> iter::Product<Basis3<S>> for Basis3<S> {
    #[inline]
    fn product<I: Iterator<Item = Basis3<S>>>(iter: I) -> Basis3<S> {
        iter.fold(Basis3::one(), Mul::mul)
    }
}

impl<'a, S: 'a + BaseFloat> iter::Product<&'a Basis3<S>> for Basis3<S> {
    #[inline]
    fn product<I: Iterator<Item = &'a Basis3<S>>>(iter: I) -> Basis3<S> {
        iter.fold(Basis3::one(), Mul::mul)
    }
}

impl<S: BaseFloat> Rotation for Basis3<S> {
    type Space = Point3<S>;

    #[inline]
    fn look_at(dir: Vector3<S>, up: Vector3<S>) -> Basis3<S> {
        Basis3 {
            mat: Matrix3::look_to_lh(dir, up),
        }
    }

    #[inline]
    fn between_vectors(a: Vector3<S>, b: Vector3<S>) -> Basis3<S> {
        let q: Quaternion<S> = Rotation::between_vectors(a, b);
        q.into()
    }

    #[inline]
    fn rotate_vector(&self, vec: Vector3<S>) -> Vector3<S> {
        self.mat * vec
    }

    // TODO: we know the matrix is orthogonal, so this could be re-written
    // to be faster
    #[inline]
    fn invert(&self) -> Basis3<S> {
        Basis3 {
            mat: self.mat.invert().unwrap(),
        }
    }
}

impl<S: BaseFloat> One for Basis3<S> {
    #[inline]
    fn one() -> Basis3<S> {
        Basis3 {
            mat: Matrix3::one(),
        }
    }
}

impl_operator!(<S: BaseFloat> Mul<Basis3<S> > for Basis3<S> {
    fn mul(lhs, rhs) -> Basis3<S> { Basis3 { mat: lhs.mat * rhs.mat  } }
});

impl<S: BaseFloat> approx::AbsDiffEq for Basis3<S> {
    type Epsilon = S::Epsilon;

    #[inline]
    fn default_epsilon() -> S::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: S::Epsilon) -> bool {
        Matrix3::abs_diff_eq(&self.mat, &other.mat, epsilon)
    }
}

impl<S: BaseFloat> approx::RelativeEq for Basis3<S> {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        Matrix3::relative_eq(&self.mat, &other.mat, epsilon, max_relative)
    }
}

impl<S: BaseFloat> approx::UlpsEq for Basis3<S> {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Matrix3::ulps_eq(&self.mat, &other.mat, epsilon, max_ulps)
    }
}

impl<S: BaseFloat> Rotation3 for Basis3<S> {
    type Scalar = S;

    fn from_axis_angle<A: Into<Rad<S>>>(axis: Vector3<S>, angle: A) -> Basis3<S> {
        Basis3 {
            mat: Matrix3::from_axis_angle(axis, angle),
        }
    }

    fn from_angle_x<A: Into<Rad<S>>>(theta: A) -> Basis3<S> {
        Basis3 {
            mat: Matrix3::from_angle_x(theta),
        }
    }

    fn from_angle_y<A: Into<Rad<S>>>(theta: A) -> Basis3<S> {
        Basis3 {
            mat: Matrix3::from_angle_y(theta),
        }
    }

    fn from_angle_z<A: Into<Rad<S>>>(theta: A) -> Basis3<S> {
        Basis3 {
            mat: Matrix3::from_angle_z(theta),
        }
    }
}

impl<A: Angle> From<Euler<A>> for Basis3<A::Unitless>
where
    A: Into<Rad<<A as Angle>::Unitless>>,
{
    /// Create a three-dimensional rotation matrix from a set of euler angles.
    fn from(src: Euler<A>) -> Basis3<A::Unitless> {
        Basis3 {
            mat: Matrix3::from(src),
        }
    }
}

impl<S: fmt::Debug> fmt::Debug for Basis3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Basis3 ")?;
        <[[S; 3]; 3] as fmt::Debug>::fmt(self.mat.as_ref(), f)
    }
}
