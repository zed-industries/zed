// Copyright 2019 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementation of circle shape.

use core::{
    f64::consts::{FRAC_PI_2, PI},
    iter,
    ops::{Add, Mul, Sub},
};

use crate::{Affine, Arc, ArcAppendIter, Ellipse, PathEl, Point, Rect, Shape, Vec2};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// A circle.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
    /// The center.
    pub center: Point,
    /// The radius.
    pub radius: f64,
}

impl Circle {
    /// A new circle from center and radius.
    #[inline(always)]
    pub fn new(center: impl Into<Point>, radius: f64) -> Circle {
        Circle {
            center: center.into(),
            radius,
        }
    }

    /// Create a [`CircleSegment`] by cutting out parts of this circle.
    #[inline(always)]
    pub fn segment(self, inner_radius: f64, start_angle: f64, sweep_angle: f64) -> CircleSegment {
        CircleSegment {
            center: self.center,
            outer_radius: self.radius,
            inner_radius,
            start_angle,
            sweep_angle,
        }
    }

    /// Is this circle [finite]?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.center.is_finite() && self.radius.is_finite()
    }

    /// Is this circle [NaN]?
    ///
    /// [NaN]: f64::is_nan
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.center.is_nan() || self.radius.is_nan()
    }
}

impl Add<Vec2> for Circle {
    type Output = Circle;

    #[inline]
    fn add(self, v: Vec2) -> Circle {
        Circle {
            center: self.center + v,
            radius: self.radius,
        }
    }
}

impl Sub<Vec2> for Circle {
    type Output = Circle;

    #[inline]
    fn sub(self, v: Vec2) -> Circle {
        Circle {
            center: self.center - v,
            radius: self.radius,
        }
    }
}

impl Mul<Circle> for Affine {
    type Output = Ellipse;
    fn mul(self, other: Circle) -> Self::Output {
        self * Ellipse::from(other)
    }
}

#[doc(hidden)]
pub struct CirclePathIter {
    circle: Circle,
    delta_th: f64,
    arm_len: f64,
    ix: usize,
    n: usize,
}

impl Shape for Circle {
    type PathElementsIter<'iter> = CirclePathIter;

    fn path_elements(&self, tolerance: f64) -> CirclePathIter {
        let scaled_err = self.radius.abs() / tolerance;
        let (n, arm_len) = if scaled_err < 1.0 / 1.9608e-4 {
            // Solution from http://spencermortensen.com/articles/bezier-circle/
            (4, 0.551915024494)
        } else {
            // This is empirically determined to fall within error tolerance.
            let n = (1.1163 * scaled_err).powf(1.0 / 6.0).ceil() as usize;
            // Note: this isn't minimum error, but it is simple and we can easily
            // estimate the error.
            let arm_len = (4.0 / 3.0) * (FRAC_PI_2 / (n as f64)).tan();
            (n, arm_len)
        };
        CirclePathIter {
            circle: *self,
            delta_th: 2.0 * PI / (n as f64),
            arm_len,
            ix: 0,
            n,
        }
    }

    #[inline]
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    #[inline]
    fn perimeter(&self, _accuracy: f64) -> f64 {
        (2.0 * PI * self.radius).abs()
    }

    fn winding(&self, pt: Point) -> i32 {
        if (pt - self.center).hypot2() < self.radius.powi(2) {
            1
        } else {
            0
        }
    }

    #[inline]
    fn bounding_box(&self) -> Rect {
        let r = self.radius.abs();
        let (x, y) = self.center.into();
        Rect::new(x - r, y - r, x + r, y + r)
    }

    #[inline(always)]
    fn as_circle(&self) -> Option<Circle> {
        Some(*self)
    }
}

impl Iterator for CirclePathIter {
    type Item = PathEl;

    fn next(&mut self) -> Option<PathEl> {
        let a = self.arm_len;
        let r = self.circle.radius;
        let (x, y) = self.circle.center.into();
        let ix = self.ix;
        self.ix += 1;
        if ix == 0 {
            Some(PathEl::MoveTo(Point::new(x + r, y)))
        } else if ix <= self.n {
            let th1 = self.delta_th * (ix as f64);
            let th0 = th1 - self.delta_th;
            let (s0, c0) = th0.sin_cos();
            let (s1, c1) = if ix == self.n {
                (0.0, 1.0)
            } else {
                th1.sin_cos()
            };
            Some(PathEl::CurveTo(
                Point::new(x + r * (c0 - a * s0), y + r * (s0 + a * c0)),
                Point::new(x + r * (c1 + a * s1), y + r * (s1 - a * c1)),
                Point::new(x + r * c1, y + r * s1),
            ))
        } else if ix == self.n + 1 {
            Some(PathEl::ClosePath)
        } else {
            None
        }
    }
}

/// A segment of a circle.
///
/// If `inner_radius > 0`, then the shape will be a doughnut segment.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CircleSegment {
    /// The center.
    pub center: Point,
    /// The outer radius.
    pub outer_radius: f64,
    /// The inner radius.
    pub inner_radius: f64,
    /// The angle to start drawing the segment (in radians).
    pub start_angle: f64,
    /// The arc length of the segment (in radians).
    pub sweep_angle: f64,
}

impl CircleSegment {
    /// Create a `CircleSegment` out of its constituent parts.
    #[inline(always)]
    pub fn new(
        center: impl Into<Point>,
        outer_radius: f64,
        inner_radius: f64,
        start_angle: f64,
        sweep_angle: f64,
    ) -> Self {
        CircleSegment {
            center: center.into(),
            outer_radius,
            inner_radius,
            start_angle,
            sweep_angle,
        }
    }

    /// Return an arc representing the outer radius.
    #[must_use]
    #[inline(always)]
    pub fn outer_arc(&self) -> Arc {
        Arc {
            center: self.center,
            radii: Vec2::new(self.outer_radius, self.outer_radius),
            start_angle: self.start_angle,
            sweep_angle: self.sweep_angle,
            x_rotation: 0.0,
        }
    }

    /// Return an arc representing the inner radius.
    ///
    /// This is [reversed] from the outer arc, so that it is in the
    /// same direction as the arc that would be drawn (as the path
    /// elements for this circle segment produce a closed path).
    ///
    /// [reversed]: Arc::reversed
    #[must_use]
    #[inline]
    pub fn inner_arc(&self) -> Arc {
        Arc {
            center: self.center,
            radii: Vec2::new(self.inner_radius, self.inner_radius),
            start_angle: self.start_angle + self.sweep_angle,
            sweep_angle: -self.sweep_angle,
            x_rotation: 0.0,
        }
    }

    /// Is this circle segment [finite]?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    pub const fn is_finite(&self) -> bool {
        self.center.is_finite()
            && self.outer_radius.is_finite()
            && self.inner_radius.is_finite()
            && self.start_angle.is_finite()
            && self.sweep_angle.is_finite()
    }

    /// Is this circle segment [NaN]?
    ///
    /// [NaN]: f64::is_nan
    #[inline]
    pub const fn is_nan(&self) -> bool {
        self.center.is_nan()
            || self.outer_radius.is_nan()
            || self.inner_radius.is_nan()
            || self.start_angle.is_nan()
            || self.sweep_angle.is_nan()
    }
}

impl Add<Vec2> for CircleSegment {
    type Output = CircleSegment;

    #[inline]
    fn add(self, v: Vec2) -> Self {
        Self {
            center: self.center + v,
            ..self
        }
    }
}

impl Sub<Vec2> for CircleSegment {
    type Output = CircleSegment;

    #[inline]
    fn sub(self, v: Vec2) -> Self {
        Self {
            center: self.center - v,
            ..self
        }
    }
}

type CircleSegmentPathIter = iter::Chain<
    iter::Chain<
        iter::Chain<iter::Chain<iter::Once<PathEl>, iter::Once<PathEl>>, ArcAppendIter>,
        iter::Once<PathEl>,
    >,
    ArcAppendIter,
>;

impl Shape for CircleSegment {
    type PathElementsIter<'iter> = CircleSegmentPathIter;

    fn path_elements(&self, tolerance: f64) -> CircleSegmentPathIter {
        iter::once(PathEl::MoveTo(point_on_circle(
            self.center,
            self.inner_radius,
            self.start_angle,
        )))
        // First radius
        .chain(iter::once(PathEl::LineTo(point_on_circle(
            self.center,
            self.outer_radius,
            self.start_angle,
        ))))
        // outer arc
        .chain(self.outer_arc().append_iter(tolerance))
        // second radius
        .chain(iter::once(PathEl::LineTo(point_on_circle(
            self.center,
            self.inner_radius,
            self.start_angle + self.sweep_angle,
        ))))
        // inner arc
        .chain(self.inner_arc().append_iter(tolerance))
    }

    #[inline]
    fn area(&self) -> f64 {
        0.5 * (self.outer_radius.powi(2) - self.inner_radius.powi(2)).abs() * self.sweep_angle
    }

    #[inline]
    fn perimeter(&self, _accuracy: f64) -> f64 {
        2.0 * (self.outer_radius - self.inner_radius).abs()
            + self.sweep_angle * (self.inner_radius + self.outer_radius)
    }

    fn winding(&self, pt: Point) -> i32 {
        let angle = (pt - self.center).atan2();
        if angle < self.start_angle || angle > self.start_angle + self.sweep_angle {
            return 0;
        }
        let dist2 = (pt - self.center).hypot2();
        if (dist2 < self.outer_radius.powi(2) && dist2 > self.inner_radius.powi(2)) ||
            // case where outer_radius < inner_radius
            (dist2 < self.inner_radius.powi(2) && dist2 > self.outer_radius.powi(2))
        {
            1
        } else {
            0
        }
    }

    #[inline]
    fn bounding_box(&self) -> Rect {
        // todo this is currently not tight
        let r = self.inner_radius.max(self.outer_radius);
        let (x, y) = self.center.into();
        Rect::new(x - r, y - r, x + r, y + r)
    }
}

#[inline]
fn point_on_circle(center: Point, radius: f64, angle: f64) -> Point {
    let (angle_sin, angle_cos) = angle.sin_cos();
    center
        + Vec2 {
            x: angle_cos * radius,
            y: angle_sin * radius,
        }
}

#[cfg(test)]
mod tests {
    use crate::{Circle, Point, Shape};
    use std::f64::consts::PI;

    fn assert_approx_eq(x: f64, y: f64) {
        // Note: we might want to be more rigorous in testing the accuracy
        // of the conversion into Béziers. But this seems good enough.
        assert!((x - y).abs() < 1e-7, "{x} != {y}");
    }

    #[test]
    fn area_sign() {
        let center = Point::new(5.0, 5.0);
        let c = Circle::new(center, 5.0);
        assert_approx_eq(c.area(), 25.0 * PI);

        assert_eq!(c.winding(center), 1);

        let p = c.to_path(1e-9);
        assert_approx_eq(c.area(), p.area());
        assert_eq!(c.winding(center), p.winding(center));

        let c_neg_radius = Circle::new(center, -5.0);
        assert_approx_eq(c_neg_radius.area(), 25.0 * PI);

        assert_eq!(c_neg_radius.winding(center), 1);

        let p_neg_radius = c_neg_radius.to_path(1e-9);
        assert_approx_eq(c_neg_radius.area(), p_neg_radius.area());
        assert_eq!(c_neg_radius.winding(center), p_neg_radius.winding(center));
    }
}
