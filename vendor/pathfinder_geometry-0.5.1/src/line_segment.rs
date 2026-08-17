// pathfinder/geometry/src/basic/line_segment.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Line segment types, optimized with SIMD.

use crate::transform2d::Matrix2x2F;
use crate::vector::{Vector2F, vec2f};
use crate::util;
use pathfinder_simd::default::F32x4;
use std::ops::{Add, Mul, MulAssign, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LineSegment2F(pub F32x4);

impl LineSegment2F {
    #[inline]
    pub fn new(from: Vector2F, to: Vector2F) -> LineSegment2F {
        LineSegment2F(from.0.concat_xy_xy(to.0))
    }

    #[inline]
    pub fn from(self) -> Vector2F {
        Vector2F(self.0.xy())
    }

    #[inline]
    pub fn to(self) -> Vector2F {
        Vector2F(self.0.zw())
    }

    #[inline]
    pub fn set_from(&mut self, point: Vector2F) {
        self.0 = point.0.to_f32x4().concat_xy_zw(self.0)
    }

    #[inline]
    pub fn set_to(&mut self, point: Vector2F) {
        self.0 = self.0.concat_xy_xy(point.0.to_f32x4())
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    pub fn from_x(self) -> f32 {
        self.0[0]
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    pub fn from_y(self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn to_x(self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn to_y(self) -> f32 {
        self.0[3]
    }

    #[inline]
    pub fn set_from_x(&mut self, x: f32) {
        self.0[0] = x
    }

    #[inline]
    pub fn set_from_y(&mut self, y: f32) {
        self.0[1] = y
    }

    #[inline]
    pub fn set_to_x(&mut self, x: f32) {
        self.0[2] = x
    }

    #[inline]
    pub fn set_to_y(&mut self, y: f32) {
        self.0[3] = y
    }

    #[inline]
    pub fn split(self, t: f32) -> (LineSegment2F, LineSegment2F) {
        debug_assert!(t >= 0.0 && t <= 1.0);
        let (from_from, to_to) = (self.0.xyxy(), self.0.zwzw());
        let d_d = to_to - from_from;
        let mid_mid = from_from + d_d * F32x4::splat(t);
        (
            LineSegment2F(from_from.concat_xy_xy(mid_mid)),
            LineSegment2F(mid_mid.concat_xy_xy(to_to)),
        )
    }

    // Returns the left segment first, followed by the right segment.
    #[inline]
    pub fn split_at_x(self, x: f32) -> (LineSegment2F, LineSegment2F) {
        let (min_part, max_part) = self.split(self.solve_t_for_x(x));
        if min_part.from_x() < max_part.from_x() {
            (min_part, max_part)
        } else {
            (max_part, min_part)
        }
    }

    // Returns the upper segment first, followed by the lower segment.
    #[inline]
    pub fn split_at_y(self, y: f32) -> (LineSegment2F, LineSegment2F) {
        let (min_part, max_part) = self.split(self.solve_t_for_y(y));

        // Make sure we compare `from_y` and `to_y` to properly handle the case in which one of the
        // two segments is zero-length.
        if min_part.from_y() < max_part.to_y() {
            (min_part, max_part)
        } else {
            (max_part, min_part)
        }
    }

    #[inline]
    pub fn solve_t_for_x(self, x: f32) -> f32 {
        (x - self.from_x()) / (self.to_x() - self.from_x())
    }

    #[inline]
    pub fn solve_t_for_y(self, y: f32) -> f32 {
        (y - self.from_y()) / (self.to_y() - self.from_y())
    }

    #[inline]
    pub fn solve_x_for_y(self, y: f32) -> f32 {
        util::lerp(self.from_x(), self.to_x(), self.solve_t_for_y(y))
    }

    #[inline]
    pub fn solve_y_for_x(self, x: f32) -> f32 {
        util::lerp(self.from_y(), self.to_y(), self.solve_t_for_x(x))
    }

    #[inline]
    pub fn reversed(self) -> LineSegment2F {
        LineSegment2F(self.0.zwxy())
    }

    #[inline]
    pub fn upper_point(self) -> Vector2F {
        if self.from_y() < self.to_y() {
            self.from()
        } else {
            self.to()
        }
    }

    #[inline]
    pub fn min_x(self) -> f32 {
        f32::min(self.from_x(), self.to_x())
    }

    #[inline]
    pub fn max_x(self) -> f32 {
        f32::max(self.from_x(), self.to_x())
    }

    #[inline]
    pub fn min_y(self) -> f32 {
        f32::min(self.from_y(), self.to_y())
    }

    #[inline]
    pub fn max_y(self) -> f32 {
        f32::max(self.from_y(), self.to_y())
    }

    #[inline]
    pub fn y_winding(self) -> i32 {
        if self.from_y() < self.to_y() {
            1
        } else {
            -1
        }
    }

    // Reverses if necessary so that the from point is above the to point. Calling this method
    // again will undo the transformation.
    #[inline]
    pub fn orient(self, y_winding: i32) -> LineSegment2F {
        if y_winding >= 0 {
            self
        } else {
            self.reversed()
        }
    }

    // TODO(pcwalton): Optimize with SIMD.
    #[inline]
    pub fn square_length(self) -> f32 {
        let (dx, dy) = (self.to_x() - self.from_x(), self.to_y() - self.from_y());
        dx * dx + dy * dy
    }

    #[inline]
    pub fn vector(self) -> Vector2F {
        self.to() - self.from()
    }

    // http://www.cs.swan.ac.uk/~cssimon/line_intersection.html
    pub fn intersection_t(self, other: LineSegment2F) -> Option<f32> {
        let p0p1 = self.vector();
        let matrix = Matrix2x2F(other.vector().0.concat_xy_xy((-p0p1).0));
        if f32::abs(matrix.det()) < EPSILON {
            return None;
        }
        return Some((matrix.inverse() * (self.from() - other.from())).y());

        const EPSILON: f32 = 0.0001;
    }

    #[inline]
    pub fn sample(self, t: f32) -> Vector2F {
        self.from() + self.vector() * t
    }

    #[inline]
    pub fn midpoint(self) -> Vector2F {
        self.sample(0.5)
    }

    #[inline]
    pub fn offset(self, distance: f32) -> LineSegment2F {
        if self.is_zero_length() {
            self
        } else {
            self + self.vector().yx().normalize() * vec2f(-distance, distance)
        }
    }

    #[inline]
    pub fn is_zero_length(self) -> bool {
        self.vector().is_zero()
    }
}

impl Add<Vector2F> for LineSegment2F {
    type Output = LineSegment2F;
    #[inline]
    fn add(self, point: Vector2F) -> LineSegment2F {
        LineSegment2F(self.0 + point.0.to_f32x4().xyxy())
    }
}

impl Sub<Vector2F> for LineSegment2F {
    type Output = LineSegment2F;
    #[inline]
    fn sub(self, point: Vector2F) -> LineSegment2F {
        LineSegment2F(self.0 - point.0.to_f32x4().xyxy())
    }
}

impl Mul<Vector2F> for LineSegment2F {
    type Output = LineSegment2F;
    #[inline]
    fn mul(self, factors: Vector2F) -> LineSegment2F {
        LineSegment2F(self.0 * factors.0.to_f32x4().xyxy())
    }
}

impl Mul<f32> for LineSegment2F {
    type Output = LineSegment2F;
    #[inline]
    fn mul(self, factor: f32) -> LineSegment2F {
        LineSegment2F(self.0 * F32x4::splat(factor))
    }
}

impl MulAssign<Vector2F> for LineSegment2F {
    #[inline]
    fn mul_assign(&mut self, factors: Vector2F) {
        *self = *self * factors
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct LineSegmentU4 {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct LineSegmentU8 {
    pub from_x: u8,
    pub from_y: u8,
    pub to_x: u8,
    pub to_y: u8,
}
