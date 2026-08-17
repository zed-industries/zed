// pathfinder/geometry/src/basic/transform2d.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! 2D affine transforms.

use crate::line_segment::LineSegment2F;
use crate::rect::RectF;
use crate::transform3d::Transform4F;
use crate::unit_vector::UnitVector;
use crate::vector::{IntoVector2F, Vector2F, vec2f};
use pathfinder_simd::default::F32x4;
use std::ops::{Mul, MulAssign, Sub};

/// A 2x2 matrix, optimized with SIMD, in column-major order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2x2F(pub F32x4);

impl Default for Matrix2x2F {
    #[inline]
    fn default() -> Matrix2x2F {
        Self::from_scale(1.0)
    }
}

impl Matrix2x2F {
    #[inline]
    pub fn from_scale<S>(scale: S) -> Matrix2x2F where S: IntoVector2F {
        let scale = scale.into_vector_2f();
        Matrix2x2F(F32x4::new(scale.x(), 0.0, 0.0, scale.y()))
    }

    #[inline]
    pub fn from_rotation(theta: f32) -> Matrix2x2F {
        Matrix2x2F::from_rotation_vector(UnitVector::from_angle(theta))
    }

    #[inline]
    pub fn from_rotation_vector(vector: UnitVector) -> Matrix2x2F {
        Matrix2x2F((vector.0).0.to_f32x4().xyyx() * F32x4::new(1.0, 1.0, -1.0, 1.0))
    }

    #[inline]
    pub fn row_major(m00: f32, m01: f32, m10: f32, m11: f32) -> Matrix2x2F {
        Matrix2x2F(F32x4::new(m00, m10, m01, m11))
    }

    #[inline]
    pub fn entrywise_mul(&self, other: &Matrix2x2F) -> Matrix2x2F {
        Matrix2x2F(self.0 * other.0)
    }

    #[inline]
    pub fn adjugate(&self) -> Matrix2x2F {
        Matrix2x2F(self.0.wyzx() * F32x4::new(1.0, -1.0, -1.0, 1.0))
    }

    #[inline]
    pub fn det(&self) -> f32 {
        self.0[0] * self.0[3] - self.0[2] * self.0[1]
    }

    #[inline]
    pub fn inverse(&self) -> Matrix2x2F {
        Matrix2x2F(F32x4::splat(1.0 / self.det()) * self.adjugate().0)
    }

    #[inline]
    pub fn scale(&self, factor: f32) -> Matrix2x2F {
        Matrix2x2F(self.0 * F32x4::splat(factor))
    }

    /// Extracts the scale from this matrix.
    #[inline]
    pub fn extract_scale(&self) -> Vector2F {
        let squared = self.0 * self.0;
        Vector2F((squared.xy() + squared.zw()).sqrt())
    }

    #[inline]
    pub fn m11(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn m21(&self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn m12(&self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn m22(&self) -> f32 {
        self.0[3]
    }
}

impl Sub<Matrix2x2F> for Matrix2x2F {
    type Output = Matrix2x2F;
    #[inline]
    fn sub(self, other: Matrix2x2F) -> Matrix2x2F {
        Matrix2x2F(self.0 - other.0)
    }
}

impl Mul<Matrix2x2F> for Matrix2x2F {
    type Output = Matrix2x2F;
    #[inline]
    fn mul(self, other: Matrix2x2F) -> Matrix2x2F {
        Matrix2x2F(self.0.xyxy() * other.0.xxzz() + self.0.zwzw() * other.0.yyww())
    }
}

impl Mul<Vector2F> for Matrix2x2F {
    type Output = Vector2F;
    #[inline]
    fn mul(self, vector: Vector2F) -> Vector2F {
        let halves = self.0 * vector.0.to_f32x4().xxyy();
        Vector2F(halves.xy() + halves.zw())
    }
}

/// An affine transform, optimized with SIMD.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform2F {
    pub matrix: Matrix2x2F,
    pub vector: Vector2F,
}

impl Default for Transform2F {
    #[inline]
    fn default() -> Transform2F {
        Self::from_scale(vec2f(1.0, 1.0))
    }
}

impl Transform2F {
    #[inline]
    pub fn from_scale<S>(scale: S) -> Transform2F where S: IntoVector2F {
        let scale = scale.into_vector_2f();
        Transform2F {
            matrix: Matrix2x2F::from_scale(scale),
            vector: Vector2F::zero(),
        }
    }

    #[inline]
    pub fn from_rotation(theta: f32) -> Transform2F {
        Transform2F {
            matrix: Matrix2x2F::from_rotation(theta),
            vector: Vector2F::zero(),
        }
    }

    #[inline]
    pub fn from_rotation_vector(vector: UnitVector) -> Transform2F {
        Transform2F {
            matrix: Matrix2x2F::from_rotation_vector(vector),
            vector: Vector2F::zero(),
        }
    }

    #[inline]
    pub fn from_translation(vector: Vector2F) -> Transform2F {
        Transform2F { matrix: Matrix2x2F::default(), vector }
    }

    #[inline]
    pub fn from_scale_rotation_translation<S>(scale: S, theta: f32, translation: Vector2F)
                                              -> Transform2F where S: IntoVector2F {
        let scale = scale.into_vector_2f();
        let rotation = Transform2F::from_rotation(theta);
        let translation = Transform2F::from_translation(translation);
        Transform2F::from_scale(scale) * rotation * translation
    }

    #[inline]
    pub fn row_major(m11: f32, m12: f32, m21: f32, m22: f32, m31: f32, m32: f32) -> Transform2F {
        Transform2F {
            matrix: Matrix2x2F::row_major(m11, m12, m21, m22),
            vector: Vector2F::new(m31, m32),
        }
    }

    // TODO(pcwalton): Optimize better with SIMD.
    #[inline]
    pub fn to_3d(&self) -> Transform4F {
        Transform4F::row_major(
            self.matrix.0[0],
            self.matrix.0[1],
            0.0,
            self.vector.x(),
            self.matrix.0[2],
            self.matrix.0[3],
            0.0,
            self.vector.y(),
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    #[inline]
    pub fn is_identity(&self) -> bool {
        *self == Transform2F::default()
    }

    /// Extracts the scale from this matrix.
    #[inline]
    pub fn extract_scale(&self) -> Vector2F {
        self.matrix.extract_scale()
    }

    #[inline]
    pub fn m11(&self) -> f32 {
        self.matrix.m11()
    }
    #[inline]
    pub fn m21(&self) -> f32 {
        self.matrix.m21()
    }
    #[inline]
    pub fn m12(&self) -> f32 {
        self.matrix.m12()
    }
    #[inline]
    pub fn m22(&self) -> f32 {
        self.matrix.m22()
    }
    #[inline]
    pub fn m31(&self) -> f32 {
        self.vector.x()
    }
    #[inline]
    pub fn m32(&self) -> f32 {
        self.vector.y()
    }

    #[inline]
    pub fn translate(&self, vector: Vector2F) -> Transform2F {
        Transform2F::from_translation(vector) * *self
    }

    #[inline]
    pub fn rotate(&self, theta: f32) -> Transform2F {
        Transform2F::from_rotation(theta) * *self
    }

    #[inline]
    pub fn scale<S>(&self, scale: S) -> Transform2F where S: IntoVector2F {
        let scale = scale.into_vector_2f();
        Transform2F::from_scale(scale) * *self
    }

    /// Returns the translation part of this matrix.
    ///
    /// This decomposition assumes that scale, rotation, and translation are applied in that order.
    #[inline]
    pub fn translation(&self) -> Vector2F {
        self.vector
    }

    /// Returns the rotation angle of this matrix.
    ///
    /// This decomposition assumes that scale, rotation, and translation are applied in that order.
    #[inline]
    pub fn rotation(&self) -> f32 {
        f32::atan2(self.m21(), self.m11())
    }

    /// Returns the scale factor of this matrix.
    ///
    /// This decomposition assumes that scale, rotation, and translation are applied in that order.
    #[inline]
    pub fn scale_factor(&self) -> f32 {
        Vector2F(self.matrix.0.zw()).length()
    }

    #[inline]
    pub fn inverse(&self) -> Transform2F {
        let matrix_inv = self.matrix.inverse();
        let vector_inv = -(matrix_inv * self.vector);
        Transform2F { matrix: matrix_inv, vector: vector_inv }
    }
}

impl Mul<Transform2F> for Transform2F {
    type Output = Transform2F;
    #[inline]
    fn mul(self, other: Transform2F) -> Transform2F {
        Transform2F {
            matrix: self.matrix * other.matrix,
            vector: self * other.vector,
        }
    }
}

impl Mul<Vector2F> for Transform2F {
    type Output = Vector2F;
    #[inline]
    fn mul(self, vector: Vector2F) -> Vector2F {
        self.matrix * vector + self.vector
    }
}

impl Mul<LineSegment2F> for Transform2F {
    type Output = LineSegment2F;
    #[inline]
    fn mul(self, line_segment: LineSegment2F) -> LineSegment2F {
        LineSegment2F::new(self * line_segment.from(), self * line_segment.to())
    }
}

impl Mul<RectF> for Transform2F {
    type Output = RectF;
    #[inline]
    fn mul(self, rect: RectF) -> RectF {
        let (upper_left, upper_right) = (self * rect.origin(),     self * rect.upper_right());
        let (lower_left, lower_right) = (self * rect.lower_left(), self * rect.lower_right());
        let min_point = upper_left.min(upper_right).min(lower_left).min(lower_right);
        let max_point = upper_left.max(upper_right).max(lower_left).max(lower_right);
        RectF::from_points(min_point, max_point)
    }
}

impl MulAssign for Transform2F {
    #[inline]
    fn mul_assign(&mut self, other: Transform2F) {
        *self = *self * other
    }
}
