// pathfinder/geometry/src/basic/rect.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! 2D axis-aligned rectangles, optimized with SIMD.

use crate::vector::{IntoVector2F, Vector2F, Vector2I};
use pathfinder_simd::default::{F32x4, I32x4};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct RectF(pub F32x4);

impl RectF {
    #[inline]
    pub fn new(origin: Vector2F, size: Vector2F) -> RectF {
        RectF(origin.0.concat_xy_xy(origin.0 + size.0))
    }

    #[inline]
    pub fn from_points(origin: Vector2F, lower_right: Vector2F) -> RectF {
        RectF(origin.0.concat_xy_xy(lower_right.0))
    }

    // Accessors

    #[inline]
    pub fn origin(self) -> Vector2F {
        Vector2F(self.0.xy())
    }

    #[inline]
    pub fn size(self) -> Vector2F {
        Vector2F(self.0.zw() - self.0.xy())
    }

    #[inline]
    pub fn origin_x(self) -> f32 {
        self.0.x()
    }

    #[inline]
    pub fn origin_y(self) -> f32 {
        self.0.y()
    }

    #[inline]
    pub fn width(self) -> f32 {
        self.0.z() - self.0.x()
    }

    #[inline]
    pub fn height(self) -> f32 {
        self.0.w() - self.0.y()
    }

    #[inline]
    pub fn upper_right(self) -> Vector2F {
        Vector2F(self.0.zy())
    }

    #[inline]
    pub fn lower_left(self) -> Vector2F {
        Vector2F(self.0.xw())
    }

    #[inline]
    pub fn lower_right(self) -> Vector2F {
        Vector2F(self.0.zw())
    }

    // Mutators

    #[inline]
    pub fn set_origin_x(&mut self, x: f32) {
        self.0.set_x(x)
    }

    #[inline]
    pub fn set_origin_y(&mut self, y: f32) {
        self.0.set_y(y)
    }

    #[inline]
    pub fn contains_point(self, point: Vector2F) -> bool {
        // self.origin <= point && point <= self.lower_right
        let point = point.0.to_f32x4();
        self.0.concat_xy_xy(point).packed_le(point.concat_xy_zw(self.0)).all_true()
    }

    #[inline]
    pub fn contains_rect(self, other: RectF) -> bool {
        // self.origin <= other.origin && other.lower_right <= self.lower_right
        self.0.concat_xy_zw(other.0).packed_le(other.0.concat_xy_zw(self.0)).all_true()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.origin() == self.lower_right()
    }

    #[inline]
    pub fn union_point(self, point: Vector2F) -> RectF {
        RectF::from_points(self.origin().min(point), self.lower_right().max(point))
    }

    #[inline]
    pub fn union_rect(self, other: RectF) -> RectF {
        RectF::from_points(
            self.origin().min(other.origin()),
            self.lower_right().max(other.lower_right()),
        )
    }

    #[inline]
    pub fn intersects(self, other: RectF) -> bool {
        // self.origin < other.lower_right && other.origin < self.lower_right
        self.0.concat_xy_xy(other.0).packed_lt(other.0.concat_zw_zw(self.0)).all_true()
    }

    #[inline]
    pub fn intersection(self, other: RectF) -> Option<RectF> {
        if !self.intersects(other) {
            None
        } else {
            Some(RectF::from_points(
                self.origin().max(other.origin()),
                self.lower_right().min(other.lower_right()),
            ))
        }
    }

    #[inline]
    pub fn min_x(self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn min_y(self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn max_x(self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn max_y(self) -> f32 {
        self.0[3]
    }

    #[inline]
    pub fn center(self) -> Vector2F {
        self.origin() + self.size() * 0.5
    }

    /// Rounds all points to the nearest integer.
    #[inline]
    pub fn round(self) -> RectF {
        RectF(self.0.to_i32x4().to_f32x4())
    }

    #[inline]
    pub fn round_out(self) -> RectF {
        RectF::from_points(self.origin().floor(), self.lower_right().ceil())
    }

    #[inline]
    pub fn dilate<A>(self, amount: A) -> RectF where A: IntoVector2F {
        let amount = amount.into_vector_2f();
        RectF::from_points(self.origin() - amount, self.lower_right() + amount)
    }

    #[inline]
    pub fn contract<A>(self, amount: A) -> RectF where A: IntoVector2F {
        let amount = amount.into_vector_2f();
        RectF::from_points(self.origin() + amount, self.lower_right() - amount)
    }

    #[inline]
    pub fn to_i32(&self) -> RectI {
        RectI(self.0.to_i32x4())
    }
}

impl Add<Vector2F> for RectF {
    type Output = RectF;
    #[inline]
    fn add(self, other: Vector2F) -> RectF {
        RectF::new(self.origin() + other, self.size())
    }
}

impl Add<f32> for RectF {
    type Output = RectF;
    #[inline]
    fn add(self, other: f32) -> RectF {
        RectF::new(self.origin() + other, self.size())
    }
}

impl Mul<Vector2F> for RectF {
    type Output = RectF;
    #[inline]
    fn mul(self, factors: Vector2F) -> RectF {
        RectF(self.0 * factors.0.concat_xy_xy(factors.0))
    }
}

impl Mul<f32> for RectF {
    type Output = RectF;
    #[inline]
    fn mul(self, factor: f32) -> RectF {
        RectF(self.0 * F32x4::splat(factor))
    }
}

impl Sub<Vector2F> for RectF {
    type Output = RectF;
    #[inline]
    fn sub(self, other: Vector2F) -> RectF {
        RectF::new(self.origin() - other, self.size())
    }
}

impl Sub<f32> for RectF {
    type Output = RectF;
    #[inline]
    fn sub(self, other: f32) -> RectF {
        RectF::new(self.origin() - other, self.size())
    }
}

/// NB: The origin is inclusive, while the lower right point is exclusive.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct RectI(pub I32x4);

impl RectI {
    #[inline]
    pub fn new(origin: Vector2I, size: Vector2I) -> RectI {
        RectI(origin.0.concat_xy_xy(origin.0 + size.0))
    }

    #[inline]
    pub fn from_points(origin: Vector2I, lower_right: Vector2I) -> RectI {
        RectI(origin.0.concat_xy_xy(lower_right.0))
    }

    // Accessors

    #[inline]
    pub fn origin(&self) -> Vector2I {
        Vector2I(self.0.xy())
    }

    #[inline]
    pub fn size(&self) -> Vector2I {
        Vector2I(self.0.zw() - self.0.xy())
    }

    #[inline]
    pub fn origin_x(self) -> i32 {
        self.0.x()
    }

    #[inline]
    pub fn origin_y(self) -> i32 {
        self.0.y()
    }

    #[inline]
    pub fn width(self) -> i32 {
        self.0.z() - self.0.x()
    }

    #[inline]
    pub fn height(self) -> i32 {
        self.0.w() - self.0.y()
    }

    #[inline]
    pub fn upper_right(&self) -> Vector2I {
        Vector2I(self.0.zy())
    }

    #[inline]
    pub fn lower_left(&self) -> Vector2I {
        Vector2I(self.0.xw())
    }

    #[inline]
    pub fn lower_right(&self) -> Vector2I {
        Vector2I(self.0.zw())
    }

    #[inline]
    pub fn scale(self, factor: i32) -> RectI {
        RectI(self.0 * I32x4::splat(factor))
    }

    #[inline]
    pub fn scale_xy(self, factors: Vector2I) -> RectI {
        RectI(self.0 * factors.0.concat_xy_xy(factors.0))
    }

    #[inline]
    pub fn min_x(self) -> i32 {
        self.0[0]
    }

    #[inline]
    pub fn min_y(self) -> i32 {
        self.0[1]
    }

    #[inline]
    pub fn max_x(self) -> i32 {
        self.0[2]
    }

    #[inline]
    pub fn max_y(self) -> i32 {
        self.0[3]
    }

    #[inline]
    pub fn intersects(self, other: RectI) -> bool {
        // self.origin < other.lower_right && other.origin < self.lower_right
        self.0.concat_xy_xy(other.0).packed_lt(other.0.concat_zw_zw(self.0)).all_true()
    }

    #[inline]
    pub fn intersection(self, other: RectI) -> Option<RectI> {
        if !self.intersects(other) {
            None
        } else {
            Some(RectI::from_points(
                self.origin().max(other.origin()),
                self.lower_right().min(other.lower_right()),
            ))
        }
    }

    #[inline]
    pub fn contains_point(&self, point: Vector2I) -> bool {
        // self.origin <= point && point <= self.lower_right - 1
        let lower_right = self.lower_right() - 1;
        self.origin()
            .0
            .concat_xy_xy(point.0)
            .packed_le(point.0.concat_xy_xy(lower_right.0))
            .all_true()
    }

    #[inline]
    pub fn contract(self, amount: Vector2I) -> RectI {
        RectI::from_points(self.origin() + amount, self.lower_right() - amount)
    }

    #[inline]
    pub fn to_f32(&self) -> RectF {
        RectF(self.0.to_f32x4())
    }
}
