// Copyright 2018 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Lines.

use core::ops::{Add, Mul, Range, Sub};

use arrayvec::ArrayVec;

use crate::{
    Affine, DEFAULT_ACCURACY, MAX_EXTREMA, Nearest, ParamCurve, ParamCurveArclen, ParamCurveArea,
    ParamCurveCurvature, ParamCurveDeriv, ParamCurveExtrema, ParamCurveNearest, PathEl, Point,
    Rect, Shape, Vec2,
};

/// A single line.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    /// The line's start point.
    pub p0: Point,
    /// The line's end point.
    pub p1: Point,
}

impl Line {
    /// Create a new line.
    #[inline(always)]
    pub fn new(p0: impl Into<Point>, p1: impl Into<Point>) -> Line {
        Line {
            p0: p0.into(),
            p1: p1.into(),
        }
    }

    /// Returns a copy of this `Line` with the end points swapped so that it
    /// points in the opposite direction.
    #[must_use]
    #[inline(always)]
    pub const fn reversed(&self) -> Line {
        Self {
            p0: self.p1,
            p1: self.p0,
        }
    }

    /// The length of the line.
    #[inline]
    pub fn length(self) -> f64 {
        self.arclen(DEFAULT_ACCURACY)
    }

    /// The midpoint of the line.
    ///
    /// This is the same as calling [`Point::midpoint`] with
    /// the endpoints of this line.
    #[must_use]
    #[inline]
    pub fn midpoint(&self) -> Point {
        self.p0.midpoint(self.p1)
    }

    /// Computes the point where two lines, if extended to infinity, would cross.
    pub fn crossing_point(self, other: Line) -> Option<Point> {
        let ab = self.p1 - self.p0;
        let cd = other.p1 - other.p0;
        let pcd = ab.cross(cd);
        if pcd == 0.0 {
            return None;
        }
        let h = ab.cross(self.p0 - other.p0) / pcd;
        Some(other.p0 + cd * h)
    }

    /// Is this line `finite`?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    pub const fn is_finite(self) -> bool {
        self.p0.is_finite() && self.p1.is_finite()
    }

    /// Is this line `NaN`?
    ///
    /// [NaN]: f64::is_nan
    #[inline]
    pub const fn is_nan(self) -> bool {
        self.p0.is_nan() || self.p1.is_nan()
    }
}

impl From<(Point, Point)> for Line {
    #[inline(always)]
    fn from((from, to): (Point, Point)) -> Self {
        Line::new(from, to)
    }
}

impl From<(Point, Vec2)> for Line {
    #[inline(always)]
    fn from((origin, displacement): (Point, Vec2)) -> Self {
        Line::new(origin, origin + displacement)
    }
}

impl ParamCurve for Line {
    #[inline]
    fn eval(&self, t: f64) -> Point {
        self.p0.lerp(self.p1, t)
    }

    #[inline]
    fn subsegment(&self, range: Range<f64>) -> Line {
        Line {
            p0: self.eval(range.start),
            p1: self.eval(range.end),
        }
    }

    #[inline(always)]
    fn start(&self) -> Point {
        self.p0
    }

    #[inline(always)]
    fn end(&self) -> Point {
        self.p1
    }
}

impl ParamCurveDeriv for Line {
    type DerivResult = ConstPoint;

    #[inline]
    fn deriv(&self) -> ConstPoint {
        ConstPoint((self.p1 - self.p0).to_point())
    }
}

impl ParamCurveArclen for Line {
    #[inline]
    fn arclen(&self, _accuracy: f64) -> f64 {
        (self.p1 - self.p0).hypot()
    }

    #[inline]
    fn inv_arclen(&self, arclen: f64, _accuracy: f64) -> f64 {
        arclen / (self.p1 - self.p0).hypot()
    }
}

impl ParamCurveArea for Line {
    #[inline]
    fn signed_area(&self) -> f64 {
        self.p0.to_vec2().cross(self.p1.to_vec2()) * 0.5
    }
}

impl ParamCurveNearest for Line {
    #[inline]
    fn nearest(&self, p: Point, _accuracy: f64) -> Nearest {
        let d = self.p1 - self.p0;
        let v = p - self.p0;

        // Calculate projection parameter `t` of the point onto the line segment s(t), with
        // s(t) = (1-t) * p0 + t * p1.
        //
        // Note when the segment has 0 length, this will be positive or negative infinity or NaN;
        // see the clamping below.
        let t = d.dot(v) / d.hypot2();

        // Clamp the parameter to be on the line segment. This clamps negative infinity and NaN to
        // `0.`, and positive infinity to `1.`.
        #[expect(
            clippy::manual_clamp,
            reason = "`clamp` uses slightly more instructions than chained `max` and `min` on x86 and aarch64"
        )]
        let t = { t.max(0.).min(1.) };

        // Calculate ||p - s(t)||^2.
        let distance_sq = (v - t * d).hypot2();

        Nearest { distance_sq, t }
    }
}

impl ParamCurveCurvature for Line {
    #[inline(always)]
    fn curvature(&self, _t: f64) -> f64 {
        0.0
    }
}

impl ParamCurveExtrema for Line {
    #[inline]
    fn extrema(&self) -> ArrayVec<f64, MAX_EXTREMA> {
        ArrayVec::new()
    }
}

/// A trivial "curve" that is just a constant.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstPoint(Point);

impl ConstPoint {
    /// Is this point [finite]?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    pub const fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Is this point [NaN]?
    ///
    /// [NaN]: f64::is_nan
    #[inline]
    pub const fn is_nan(self) -> bool {
        self.0.is_nan()
    }
}

impl ParamCurve for ConstPoint {
    #[inline(always)]
    fn eval(&self, _t: f64) -> Point {
        self.0
    }

    #[inline(always)]
    fn subsegment(&self, _range: Range<f64>) -> ConstPoint {
        *self
    }
}

impl ParamCurveDeriv for ConstPoint {
    type DerivResult = ConstPoint;

    #[inline(always)]
    fn deriv(&self) -> ConstPoint {
        ConstPoint(Point::new(0.0, 0.0))
    }
}

impl ParamCurveArclen for ConstPoint {
    #[inline(always)]
    fn arclen(&self, _accuracy: f64) -> f64 {
        0.0
    }

    #[inline(always)]
    fn inv_arclen(&self, _arclen: f64, _accuracy: f64) -> f64 {
        0.0
    }
}

impl Mul<Line> for Affine {
    type Output = Line;

    #[inline]
    fn mul(self, other: Line) -> Line {
        Line {
            p0: self * other.p0,
            p1: self * other.p1,
        }
    }
}

impl Add<Vec2> for Line {
    type Output = Line;

    #[inline]
    fn add(self, v: Vec2) -> Line {
        Line::new(self.p0 + v, self.p1 + v)
    }
}

impl Sub<Vec2> for Line {
    type Output = Line;

    #[inline]
    fn sub(self, v: Vec2) -> Line {
        Line::new(self.p0 - v, self.p1 - v)
    }
}

/// An iterator yielding the path for a single line.
#[doc(hidden)]
pub struct LinePathIter {
    line: Line,
    ix: usize,
}

impl Shape for Line {
    type PathElementsIter<'iter> = LinePathIter;

    #[inline]
    fn path_elements(&self, _tolerance: f64) -> LinePathIter {
        LinePathIter { line: *self, ix: 0 }
    }

    /// Returning zero here is consistent with the contract (area is
    /// only meaningful for closed shapes), but an argument can be made
    /// that the contract should be tightened to include the Green's
    /// theorem contribution.
    #[inline(always)]
    fn area(&self) -> f64 {
        0.0
    }

    #[inline]
    fn perimeter(&self, _accuracy: f64) -> f64 {
        (self.p1 - self.p0).hypot()
    }

    /// Same consideration as `area`.
    #[inline(always)]
    fn winding(&self, _pt: Point) -> i32 {
        0
    }

    #[inline(always)]
    fn bounding_box(&self) -> Rect {
        Rect::from_points(self.p0, self.p1)
    }

    #[inline(always)]
    fn as_line(&self) -> Option<Line> {
        Some(*self)
    }
}

impl Iterator for LinePathIter {
    type Item = PathEl;

    fn next(&mut self) -> Option<PathEl> {
        self.ix += 1;
        match self.ix {
            1 => Some(PathEl::MoveTo(self.line.p0)),
            2 => Some(PathEl::LineTo(self.line.p1)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Line, ParamCurveArclen, Point};

    #[test]
    fn line_reversed() {
        let l = Line::new((0.0, 0.0), (1.0, 1.0));
        let f = l.reversed();

        assert_eq!(l.p0, f.p1);
        assert_eq!(l.p1, f.p0);

        // Reversing it again should result in the original line
        assert_eq!(l, f.reversed());
    }

    #[test]
    fn line_arclen() {
        let l = Line::new((0.0, 0.0), (1.0, 1.0));
        let true_len = 2.0f64.sqrt();
        let epsilon = 1e-9;
        assert!(l.arclen(epsilon) - true_len < epsilon);

        let t = l.inv_arclen(true_len / 3.0, epsilon);
        assert!((t - 1.0 / 3.0).abs() < epsilon);
    }

    #[test]
    fn line_midpoint() {
        let l = Line::new((0.0, 0.0), (2.0, 4.0));
        assert_eq!(l.midpoint(), Point::new(1.0, 2.0));
    }

    #[test]
    fn line_is_finite() {
        assert!(
            (Line {
                p0: Point { x: 0., y: 0. },
                p1: Point { x: 1., y: 1. }
            })
            .is_finite()
        );

        assert!(
            !(Line {
                p0: Point { x: 0., y: 0. },
                p1: Point {
                    x: f64::INFINITY,
                    y: 1.
                }
            })
            .is_finite()
        );

        assert!(
            !(Line {
                p0: Point { x: 0., y: 0. },
                p1: Point {
                    x: 0.,
                    y: f64::INFINITY
                }
            })
            .is_finite()
        );
    }

    #[test]
    fn line_nearest() {
        use crate::{ParamCurve, ParamCurveNearest};

        const EPSILON: f64 = 1e-9;

        let line = Line::new((-4., 0.), (2., 1.));

        // Projects onto the line segment end point.
        let point = Point::new(4., 0.);
        let nearest = line.nearest(point, 0.);
        assert_eq!(nearest.t, 1.);
        assert!((nearest.distance_sq - line.p1.distance_squared(point)).abs() < EPSILON);

        // Projects onto the line segment start point.
        let point = Point::new(0., -50.);
        let nearest = line.nearest(point, 0.);
        assert_eq!(nearest.t, 0.);
        assert!((nearest.distance_sq - line.p0.distance_squared(point)).abs() < EPSILON);

        // Projects onto the line segment proper (not just onto one of its extrema).
        let point = Point::new(-1., 0.5);
        let nearest = line.nearest(point, 0.);
        assert!(nearest.t > 0. && nearest.t < 1.);
        // Ensure evaluating and calculating distance manually has the same result.
        assert!(
            (line.eval(nearest.t).distance_squared(point) - nearest.distance_sq).abs() < EPSILON
        );

        // Test minimality while avoiding reimplementing projection in this test by checking that
        // moving to a slightly different point on the segment increases the distance.
        assert!(line.eval(nearest.t * 0.95).distance_squared(point) > nearest.distance_sq);
        assert!(line.eval(nearest.t * 1.05).distance_squared(point) > nearest.distance_sq);
    }
}
