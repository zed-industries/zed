use crate::scalar::Scalar;
use crate::segment::Segment;
use crate::traits::Transformation;
use crate::{point, Box2D, Point, Vector};
use crate::{CubicBezierSegment, Line, LineEquation, LineSegment, Triangle};
use arrayvec::ArrayVec;
use num_traits::NumCast;

use core::mem;
use core::ops::Range;

/// A 2d curve segment defined by three points: the beginning of the segment, a control
/// point and the end of the segment.
///
/// The curve is defined by equation:
/// ```∀ t ∈ [0..1],  P(t) = (1 - t)² * from + 2 * (1 - t) * t * ctrl + t² * to```
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct QuadraticBezierSegment<S> {
    pub from: Point<S>,
    pub ctrl: Point<S>,
    pub to: Point<S>,
}

impl<S: Scalar> QuadraticBezierSegment<S> {
    pub fn cast<NewS: NumCast>(self) -> QuadraticBezierSegment<NewS> {
        QuadraticBezierSegment {
            from: self.from.cast(),
            ctrl: self.ctrl.cast(),
            to: self.to.cast(),
        }
    }

    /// Sample the curve at t (expecting t between 0 and 1).
    pub fn sample(&self, t: S) -> Point<S> {
        let t2 = t * t;
        let one_t = S::ONE - t;
        let one_t2 = one_t * one_t;

        self.from * one_t2 + self.ctrl.to_vector() * S::TWO * one_t * t + self.to.to_vector() * t2
    }

    /// Sample the x coordinate of the curve at t (expecting t between 0 and 1).
    pub fn x(&self, t: S) -> S {
        let t2 = t * t;
        let one_t = S::ONE - t;
        let one_t2 = one_t * one_t;

        self.from.x * one_t2 + self.ctrl.x * S::TWO * one_t * t + self.to.x * t2
    }

    /// Sample the y coordinate of the curve at t (expecting t between 0 and 1).
    pub fn y(&self, t: S) -> S {
        let t2 = t * t;
        let one_t = S::ONE - t;
        let one_t2 = one_t * one_t;

        self.from.y * one_t2 + self.ctrl.y * S::TWO * one_t * t + self.to.y * t2
    }

    #[inline]
    fn derivative_coefficients(&self, t: S) -> (S, S, S) {
        (S::TWO * t - S::TWO, -S::FOUR * t + S::TWO, S::TWO * t)
    }

    /// Sample the curve's derivative at t (expecting t between 0 and 1).
    pub fn derivative(&self, t: S) -> Vector<S> {
        let (c0, c1, c2) = self.derivative_coefficients(t);
        self.from.to_vector() * c0 + self.ctrl.to_vector() * c1 + self.to.to_vector() * c2
    }

    /// Sample the x coordinate of the curve's derivative at t (expecting t between 0 and 1).
    pub fn dx(&self, t: S) -> S {
        let (c0, c1, c2) = self.derivative_coefficients(t);
        self.from.x * c0 + self.ctrl.x * c1 + self.to.x * c2
    }

    /// Sample the y coordinate of the curve's derivative at t (expecting t between 0 and 1).
    pub fn dy(&self, t: S) -> S {
        let (c0, c1, c2) = self.derivative_coefficients(t);
        self.from.y * c0 + self.ctrl.y * c1 + self.to.y * c2
    }

    /// Swap the beginning and the end of the segment.
    pub fn flip(&self) -> Self {
        QuadraticBezierSegment {
            from: self.to,
            ctrl: self.ctrl,
            to: self.from,
        }
    }

    /// Find the advancement of the y-most position in the curve.
    ///
    /// This returns the advancement along the curve, not the actual y position.
    pub fn y_maximum_t(&self) -> S {
        if let Some(t) = self.local_y_extremum_t() {
            let y = self.y(t);
            if y > self.from.y && y > self.to.y {
                return t;
            }
        }

        if self.from.y > self.to.y {
            S::ZERO
        } else {
            S::ONE
        }
    }

    /// Find the advancement of the y-least position in the curve.
    ///
    /// This returns the advancement along the curve, not the actual y position.
    pub fn y_minimum_t(&self) -> S {
        if let Some(t) = self.local_y_extremum_t() {
            let y = self.y(t);
            if y < self.from.y && y < self.to.y {
                return t;
            }
        }

        if self.from.y < self.to.y {
            S::ZERO
        } else {
            S::ONE
        }
    }

    /// Return the y inflection point or None if this curve is y-monotonic.
    pub fn local_y_extremum_t(&self) -> Option<S> {
        let div = self.from.y - S::TWO * self.ctrl.y + self.to.y;
        if div == S::ZERO {
            return None;
        }
        let t = (self.from.y - self.ctrl.y) / div;
        if t > S::ZERO && t < S::ONE {
            return Some(t);
        }

        None
    }

    /// Find the advancement of the x-most position in the curve.
    ///
    /// This returns the advancement along the curve, not the actual x position.
    pub fn x_maximum_t(&self) -> S {
        if let Some(t) = self.local_x_extremum_t() {
            let x = self.x(t);
            if x > self.from.x && x > self.to.x {
                return t;
            }
        }

        if self.from.x > self.to.x {
            S::ZERO
        } else {
            S::ONE
        }
    }

    /// Find the advancement of the x-least position in the curve.
    ///
    /// This returns the advancement along the curve, not the actual x position.
    pub fn x_minimum_t(&self) -> S {
        if let Some(t) = self.local_x_extremum_t() {
            let x = self.x(t);
            if x < self.from.x && x < self.to.x {
                return t;
            }
        }

        if self.from.x < self.to.x {
            S::ZERO
        } else {
            S::ONE
        }
    }

    /// Return the x inflection point or None if this curve is x-monotonic.
    pub fn local_x_extremum_t(&self) -> Option<S> {
        let div = self.from.x - S::TWO * self.ctrl.x + self.to.x;
        if div == S::ZERO {
            return None;
        }
        let t = (self.from.x - self.ctrl.x) / div;
        if t > S::ZERO && t < S::ONE {
            return Some(t);
        }

        None
    }

    /// Return the sub-curve inside a given range of t.
    ///
    /// This is equivalent splitting at the range's end points.
    pub fn split_range(&self, t_range: Range<S>) -> Self {
        let t0 = t_range.start;
        let t1 = t_range.end;

        let from = self.sample(t0);
        let to = self.sample(t1);
        let ctrl = from + (self.ctrl - self.from).lerp(self.to - self.ctrl, t0) * (t1 - t0);

        QuadraticBezierSegment { from, ctrl, to }
    }

    /// Split this curve into two sub-curves.
    pub fn split(&self, t: S) -> (QuadraticBezierSegment<S>, QuadraticBezierSegment<S>) {
        let split_point = self.sample(t);

        (
            QuadraticBezierSegment {
                from: self.from,
                ctrl: self.from.lerp(self.ctrl, t),
                to: split_point,
            },
            QuadraticBezierSegment {
                from: split_point,
                ctrl: self.ctrl.lerp(self.to, t),
                to: self.to,
            },
        )
    }

    /// Return the curve before the split point.
    pub fn before_split(&self, t: S) -> QuadraticBezierSegment<S> {
        QuadraticBezierSegment {
            from: self.from,
            ctrl: self.from.lerp(self.ctrl, t),
            to: self.sample(t),
        }
    }

    /// Return the curve after the split point.
    pub fn after_split(&self, t: S) -> QuadraticBezierSegment<S> {
        QuadraticBezierSegment {
            from: self.sample(t),
            ctrl: self.ctrl.lerp(self.to, t),
            to: self.to,
        }
    }

    /// Elevate this curve to a third order bézier.
    pub fn to_cubic(&self) -> CubicBezierSegment<S> {
        CubicBezierSegment {
            from: self.from,
            ctrl1: (self.from + self.ctrl.to_vector() * S::TWO) / S::THREE,
            ctrl2: (self.to + self.ctrl.to_vector() * S::TWO) / S::THREE,
            to: self.to,
        }
    }

    #[inline]
    pub fn baseline(&self) -> LineSegment<S> {
        LineSegment {
            from: self.from,
            to: self.to,
        }
    }

    /// Returns whether the curve can be approximated with a single point, given
    /// a tolerance threshold.
    pub fn is_a_point(&self, tolerance: S) -> bool {
        let tol2 = tolerance * tolerance;
        (self.from - self.to).square_length() <= tol2
            && (self.from - self.ctrl).square_length() <= tol2
    }

    /// Returns true if the curve can be approximated with a single line segment
    /// given a tolerance threshold.
    pub fn is_linear(&self, tolerance: S) -> bool {
        if self.from == self.to {
            return true;
        }

        let d = self
            .baseline()
            .to_line()
            .square_distance_to_point(self.ctrl);

        d <= (tolerance * tolerance * S::FOUR)
    }

    /// Computes a "fat line" of this segment.
    ///
    /// A fat line is two conservative lines between which the segment
    /// is fully contained.
    pub fn fat_line(&self) -> (LineEquation<S>, LineEquation<S>) {
        let l1 = self.baseline().to_line().equation();
        let d = S::HALF * l1.signed_distance_to_point(&self.ctrl);
        let l2 = l1.offset(d);
        if d >= S::ZERO {
            (l1, l2)
        } else {
            (l2, l1)
        }
    }

    /// Applies the transform to this curve and returns the results.
    #[inline]
    pub fn transformed<T: Transformation<S>>(&self, transform: &T) -> Self {
        QuadraticBezierSegment {
            from: transform.transform_point(self.from),
            ctrl: transform.transform_point(self.ctrl),
            to: transform.transform_point(self.to),
        }
    }

    /// Find the interval of the beginning of the curve that can be approximated with a
    /// line segment.
    pub fn flattening_step(&self, tolerance: S) -> S {
        let v1 = self.ctrl - self.from;
        let v2 = self.to - self.from;

        let v1_cross_v2 = v2.x * v1.y - v2.y * v1.x;
        let h = S::sqrt(v1.x * v1.x + v1.y * v1.y);

        if S::abs(v1_cross_v2 * h) <= S::EPSILON {
            return S::ONE;
        }

        let s2inv = h / v1_cross_v2;

        let t = S::TWO * S::sqrt(tolerance * S::abs(s2inv) / S::THREE);

        if t > S::ONE {
            return S::ONE;
        }

        t
    }

    /// Approximates the curve with sequence of line segments.
    ///
    /// The `tolerance` parameter defines the maximum distance between the curve and
    /// its approximation.
    pub fn for_each_flattened<F>(&self, tolerance: S, callback: &mut F)
    where
        F: FnMut(&LineSegment<S>),
    {
        debug_assert!(tolerance >= S::EPSILON * S::EPSILON);

        let n = flattened_segments_wang(self, tolerance);
        let step = S::ONE / n;
        let mut t = step;

        let curve = self.polynomial_form();

        let n = n.to_u32().unwrap_or(1) - 1;
        let mut from = self.from;
        for _ in 0..n {
            let to = curve.sample(t);
            callback(&LineSegment { from, to });
            from = to;
            t += step;
        }

        callback(&LineSegment { from, to: self.to });
    }

    /// Compute a flattened approximation of the curve, invoking a callback at
    /// each step.
    ///
    /// The `tolerance` parameter defines the maximum distance between the curve and
    /// its approximation.
    ///
    /// The end of the t parameter range at the final segment is guaranteed to be equal to `1.0`.
    pub fn for_each_flattened_with_t<F>(&self, tolerance: S, callback: &mut F)
    where
        F: FnMut(&LineSegment<S>, Range<S>),
    {
        debug_assert!(tolerance >= S::EPSILON * S::EPSILON);

        let n = flattened_segments_wang(self, tolerance);
        let step = S::ONE / n;
        let mut t0 = S::ZERO;

        let curve = self.polynomial_form();

        let n = n.to_u32().unwrap_or(1) - 1;
        let mut from = self.from;
        for _ in 0..n {
            let t1 = t0 + step;
            let to = curve.sample(t1);
            callback(&LineSegment { from, to }, t0..t1);
            from = to;
            t0 = t1;
        }

        callback(&LineSegment { from, to: self.to }, t0..S::ONE);
    }

    /// Returns the flattened representation of the curve as an iterator, starting *after* the
    /// current point.
    pub fn flattened(&self, tolerance: S) -> Flattened<S> {
        Flattened::new(self, tolerance)
    }
    /// Returns the flattened representation of the curve as an iterator, starting *after* the
    /// current point.
    pub fn flattened_t(&self, tolerance: S) -> FlattenedT<S> {
        FlattenedT::new(self, tolerance)
    }

    /// Invokes a callback for each monotonic part of the segment.
    pub fn for_each_monotonic_range<F>(&self, cb: &mut F)
    where
        F: FnMut(Range<S>),
    {
        let mut t0 = self.local_x_extremum_t();
        let mut t1 = self.local_y_extremum_t();

        let swap = match (t0, t1) {
            (Some(tx), Some(ty)) => tx > ty,
            _ => false,
        };

        if swap {
            mem::swap(&mut t0, &mut t1);
        }

        let mut start = S::ZERO;

        if let Some(t) = t0 {
            cb(start..t);
            start = t;
        }

        if let Some(t) = t1 {
            // In extreme cases the same point can be an x and y inflection point.
            if t != start {
                cb(start..t);
                start = t
            }
        }

        cb(start..S::ONE);
    }

    /// Invokes a callback for each monotonic part of the segment.
    pub fn for_each_monotonic<F>(&self, cb: &mut F)
    where
        F: FnMut(&QuadraticBezierSegment<S>),
    {
        self.for_each_monotonic_range(&mut |range| {
            let mut sub = self.split_range(range);
            // Due to finite precision the split may actually result in sub-curves
            // that are almost but not-quite monotonic. Make sure they actually are.
            let min_x = sub.from.x.min(sub.to.x);
            let max_x = sub.from.x.max(sub.to.x);
            let min_y = sub.from.y.min(sub.to.y);
            let max_y = sub.from.y.max(sub.to.y);
            sub.ctrl.x = sub.ctrl.x.max(min_x).min(max_x);
            sub.ctrl.y = sub.ctrl.y.max(min_y).min(max_y);
            cb(&sub);
        });
    }

    /// Invokes a callback for each y-monotonic part of the segment.
    pub fn for_each_y_monotonic_range<F>(&self, cb: &mut F)
    where
        F: FnMut(Range<S>),
    {
        match self.local_y_extremum_t() {
            Some(t) => {
                cb(S::ZERO..t);
                cb(t..S::ONE);
            }
            None => {
                cb(S::ZERO..S::ONE);
            }
        }
    }

    /// Invokes a callback for each y-monotonic part of the segment.
    pub fn for_each_y_monotonic<F>(&self, cb: &mut F)
    where
        F: FnMut(&QuadraticBezierSegment<S>),
    {
        match self.local_y_extremum_t() {
            Some(t) => {
                let (a, b) = self.split(t);
                cb(&a);
                cb(&b);
            }
            None => {
                cb(self);
            }
        }
    }

    /// Invokes a callback for each x-monotonic part of the segment.
    pub fn for_each_x_monotonic_range<F>(&self, cb: &mut F)
    where
        F: FnMut(Range<S>),
    {
        match self.local_x_extremum_t() {
            Some(t) => {
                cb(S::ZERO..t);
                cb(t..S::ONE);
            }
            None => {
                cb(S::ZERO..S::ONE);
            }
        }
    }

    /// Invokes a callback for each x-monotonic part of the segment.
    pub fn for_each_x_monotonic<F>(&self, cb: &mut F)
    where
        F: FnMut(&QuadraticBezierSegment<S>),
    {
        match self.local_x_extremum_t() {
            Some(t) => {
                let (mut a, mut b) = self.split(t);
                // Due to finite precision the split may actually result in sub-curves
                // that are almost but not-quite monotonic. Make sure they actually are.
                let a_min = a.from.x.min(a.to.x);
                let a_max = a.from.x.max(a.to.x);
                let b_min = b.from.x.min(b.to.x);
                let b_max = b.from.x.max(b.to.x);
                a.ctrl.x = a.ctrl.x.max(a_min).min(a_max);
                b.ctrl.x = b.ctrl.x.max(b_min).min(b_max);
                cb(&a);
                cb(&b);
            }
            None => {
                cb(self);
            }
        }
    }

    /// Returns a triangle containing this curve segment.
    pub fn bounding_triangle(&self) -> Triangle<S> {
        Triangle {
            a: self.from,
            b: self.ctrl,
            c: self.to,
        }
    }

    /// Returns a conservative rectangle that contains the curve.
    pub fn fast_bounding_box(&self) -> Box2D<S> {
        let (min_x, max_x) = self.fast_bounding_range_x();
        let (min_y, max_y) = self.fast_bounding_range_y();

        Box2D {
            min: point(min_x, min_y),
            max: point(max_x, max_y),
        }
    }

    /// Returns a conservative range of x that contains this curve.
    pub fn fast_bounding_range_x(&self) -> (S, S) {
        let min_x = self.from.x.min(self.ctrl.x).min(self.to.x);
        let max_x = self.from.x.max(self.ctrl.x).max(self.to.x);

        (min_x, max_x)
    }

    /// Returns a conservative range of y that contains this curve.
    pub fn fast_bounding_range_y(&self) -> (S, S) {
        let min_y = self.from.y.min(self.ctrl.y).min(self.to.y);
        let max_y = self.from.y.max(self.ctrl.y).max(self.to.y);

        (min_y, max_y)
    }

    /// Returns the smallest rectangle the curve is contained in
    pub fn bounding_box(&self) -> Box2D<S> {
        let (min_x, max_x) = self.bounding_range_x();
        let (min_y, max_y) = self.bounding_range_y();

        Box2D {
            min: point(min_x, min_y),
            max: point(max_x, max_y),
        }
    }

    /// Returns the smallest range of x that contains this curve.
    pub fn bounding_range_x(&self) -> (S, S) {
        let min_x = self.x(self.x_minimum_t());
        let max_x = self.x(self.x_maximum_t());

        (min_x, max_x)
    }

    /// Returns the smallest range of y that contains this curve.
    pub fn bounding_range_y(&self) -> (S, S) {
        let min_y = self.y(self.y_minimum_t());
        let max_y = self.y(self.y_maximum_t());

        (min_y, max_y)
    }

    /// Returns whether this segment is monotonic on the x axis.
    pub fn is_x_monotonic(&self) -> bool {
        self.local_x_extremum_t().is_none()
    }

    /// Returns whether this segment is monotonic on the y axis.
    pub fn is_y_monotonic(&self) -> bool {
        self.local_y_extremum_t().is_none()
    }

    /// Returns whether this segment is fully monotonic.
    pub fn is_monotonic(&self) -> bool {
        self.is_x_monotonic() && self.is_y_monotonic()
    }

    /// Computes the intersections (if any) between this segment a line.
    ///
    /// The result is provided in the form of the `t` parameters of each
    /// point along curve. To get the intersection points, sample the curve
    /// at the corresponding values.
    pub fn line_intersections_t(&self, line: &Line<S>) -> ArrayVec<S, 2> {
        // take the quadratic bezier formulation and inject it in
        // the line equation ax + by + c = 0.
        let eqn = line.equation();
        let i = eqn.a() * self.from.x + eqn.b() * self.from.y;
        let j = eqn.a() * self.ctrl.x + eqn.b() * self.ctrl.y;
        let k = eqn.a() * self.to.x + eqn.b() * self.to.y;
        // Solve "(i - 2j + k)t² + (2j - 2i)t + (i + c) = 0"
        let a = i - j - j + k;
        let b = j + j - i - i;
        let c = i + eqn.c();

        let mut result = ArrayVec::new();

        if a == S::ZERO {
            // Linear equation bt + c = 0.
            let t = c / b;
            if t >= S::ZERO && t <= S::ONE {
                result.push(t);
                return result;
            }
        }

        let delta = b * b - S::FOUR * a * c;
        if delta >= S::ZERO {
            // To avoid potential float precision issues when b is close to
            // sqrt_delta, we exploit the fact that given the roots t1 and t2,
            // t2 = c / (a * t1) and t1 = c / (a * t2).
            let sqrt_delta = S::sqrt(delta);
            let s_sqrt_delta = -b.signum() * sqrt_delta;
            let mut t1 = (-b + s_sqrt_delta) / (S::TWO * a);
            let mut t2 = c / (a * t1);

            if t1 > t2 {
                mem::swap(&mut t1, &mut t2);
            }

            if t1 >= S::ZERO && t1 <= S::ONE {
                result.push(t1);
            }

            if t2 >= S::ZERO && t2 <= S::ONE && t1 != t2 {
                result.push(t2);
            }
        }

        result
    }

    /// Computes the intersection points (if any) between this segment a line.
    pub fn line_intersections(&self, line: &Line<S>) -> ArrayVec<Point<S>, 2> {
        let intersections = self.line_intersections_t(line);

        let mut result = ArrayVec::new();
        for t in intersections {
            result.push(self.sample(t));
        }

        result
    }

    /// Computes the intersections (if any) between this segment and a line segment.
    ///
    /// The result is provided in the form of the `t` parameters of each
    /// point along curve and segment. To get the intersection points, sample
    /// the segments at the corresponding values.
    pub fn line_segment_intersections_t(&self, segment: &LineSegment<S>) -> ArrayVec<(S, S), 2> {
        if !self
            .fast_bounding_box()
            .inflate(S::EPSILON, S::EPSILON)
            .intersects(&segment.bounding_box().inflate(S::EPSILON, S::EPSILON))
        {
            return ArrayVec::new();
        }

        let intersections = self.line_intersections_t(&segment.to_line());

        let mut result = ArrayVec::new();
        if intersections.is_empty() {
            return result;
        }

        let seg_is_mostly_vertical =
            S::abs(segment.from.y - segment.to.y) >= S::abs(segment.from.x - segment.to.x);
        let (seg_long_axis_min, seg_long_axis_max) = if seg_is_mostly_vertical {
            segment.bounding_range_y()
        } else {
            segment.bounding_range_x()
        };

        for t in intersections {
            let intersection_xy = if seg_is_mostly_vertical {
                self.y(t)
            } else {
                self.x(t)
            };
            if intersection_xy >= seg_long_axis_min && intersection_xy <= seg_long_axis_max {
                let t2 = (self.sample(t) - segment.from).length() / segment.length();
                // Don't take intersections that are on endpoints of both curves at the same time.
                if (t != S::ZERO && t != S::ONE) || (t2 != S::ZERO && t2 != S::ONE) {
                    result.push((t, t2));
                }
            }
        }

        result
    }

    #[inline]
    pub fn from(&self) -> Point<S> {
        self.from
    }

    #[inline]
    pub fn to(&self) -> Point<S> {
        self.to
    }

    /// Computes the intersection points (if any) between this segment a line segment.
    pub fn line_segment_intersections(&self, segment: &LineSegment<S>) -> ArrayVec<Point<S>, 2> {
        let intersections = self.line_segment_intersections_t(segment);

        let mut result = ArrayVec::new();
        for (t, _) in intersections {
            result.push(self.sample(t));
        }

        result
    }

    /// Analytic solution to finding the closest point on the curve to `pos`.
    pub fn closest_point(&self, pos: Point<S>) -> S {
        // We are looking for the points in the curve where the line passing through pos
        // and these points are perpendicular to the curve.
        let a = self.from - pos;
        let b = self.ctrl - self.from;
        let c = self.from + self.to.to_vector() - self.ctrl * S::TWO;

        // Polynomial coefficients
        let c0 = c.dot(c);
        let c1 = b.dot(c) * S::THREE;
        let c2 = b.dot(b) * S::TWO + a.dot(c);
        let c3 = a.dot(b);

        let roots = crate::utils::cubic_polynomial_roots(c0, c1, c2, c3);

        let mut sq_dist = a.square_length();
        let mut t = S::ZERO;
        let to_dist = (self.to - pos).square_length();
        if to_dist < sq_dist {
            sq_dist = to_dist;
            t = S::ONE
        }
        for root in roots {
            if root >= S::ZERO && root <= S::ONE {
                let p = self.sample(root);
                let d = (pos - p).square_length();
                if d < sq_dist {
                    sq_dist = d;
                    t = root;
                }
            }
        }

        t
    }

    /// Returns the shortest distance between this segment and a point.
    pub fn distance_to_point(&self, pos: Point<S>) -> S {
        (self.sample(self.closest_point(pos)) - pos).length()
    }

    /// Returns the shortest squared distance between this segment and a point.
    ///
    /// May be useful to avoid the cost of a square root when comparing against a distance
    /// that can be squared instead.
    pub fn square_distance_to_point(&self, pos: Point<S>) -> S {
        (self.sample(self.closest_point(pos)) - pos).square_length()
    }

    // Returns a quadratic bézier curve built by dragging this curve's point at `t`
    // to a new position without moving the endpoints.
    pub fn drag(&self, t: S, new_position: Point<S>) -> Self {
        let t2 = t * t;
        let one_t = S::ONE - t;
        let one_t2 = one_t * one_t;

        let u = t2 / (t2 + one_t2);
        let c = self.from.lerp(self.to, u);

        let inv_r = S::abs((t2 + one_t2) / (t2 + one_t2 - S::ONE));

        QuadraticBezierSegment {
            from: self.from,
            ctrl: new_position + (new_position - c) * inv_r,
            to: self.to,
        }
    }

    /// Computes the length of this segment.
    ///
    /// Implements Raph Levien's analytical approach described in
    /// https://raphlinus.github.io/curves/2018/12/28/bezier-arclength.html
    pub fn length(&self) -> S {
        // This is ported from kurbo's implementation.
        // https://github.com/linebender/kurbo/blob/d0b956b47f219ba2303b4e2f2d904ea7b946e783/src/quadbez.rs#L239
        let d2 = self.from - self.ctrl * S::TWO + self.to.to_vector();
        let d1 = self.ctrl - self.from;
        let a = d2.square_length();
        let c = d1.square_length();
        if a < S::value(1e-4) * c {
            // The segment is almost straight.
            //
            // Legendre-Gauss quadrature using formula from Behdad
            // in https://github.com/Pomax/BezierInfo-2/issues/77
            let v0 = (self.from.to_vector() * S::value(-0.492943519233745)
                + self.ctrl.to_vector() * S::value(0.430331482911935)
                + self.to.to_vector() * S::value(0.0626120363218102))
            .length();
            let v1 = ((self.to - self.from) * S::value(0.4444444444444444)).length();
            let v2 = (self.from.to_vector() * S::value(-0.0626120363218102)
                + self.ctrl.to_vector() * S::value(-0.430331482911935)
                + self.to.to_vector() * S::value(0.492943519233745))
            .length();
            return v0 + v1 + v2;
        }

        let b = S::TWO * d2.dot(d1);

        let sqr_abc = (a + b + c).sqrt();
        let a2 = a.powf(-S::HALF);
        let a32 = a2.powi(3);
        let c2 = S::TWO * c.sqrt();
        let ba_c2 = b * a2 + c2;

        let v0 = S::HALF * S::HALF * a2 * a2 * b * (S::TWO * sqr_abc - c2) + sqr_abc;

        if ba_c2 < S::EPSILON {
            // The curve has a sharp turns.
            v0
        } else {
            v0 + S::HALF
                * S::HALF
                * a32
                * (S::FOUR * c * a - b * b)
                * (((S::TWO * a + b) * a2 + S::TWO * sqr_abc) / ba_c2).ln()
        }
    }

    // This is to conform to the `impl_segment!` macro
    fn approximate_length(&self, _tolerance: S) -> S {
        self.length()
    }

    pub fn to_f32(&self) -> QuadraticBezierSegment<f32> {
        QuadraticBezierSegment {
            from: self.from.to_f32(),
            ctrl: self.ctrl.to_f32(),
            to: self.to.to_f32(),
        }
    }

    pub fn to_f64(&self) -> QuadraticBezierSegment<f64> {
        QuadraticBezierSegment {
            from: self.from.to_f64(),
            ctrl: self.ctrl.to_f64(),
            to: self.to.to_f64(),
        }
    }

    #[inline]
    pub fn polynomial_form(&self) -> QuadraticBezierPolynomial<S> {
        let from = self.from.to_vector();
        let ctrl = self.ctrl.to_vector();
        let to = self.to.to_vector();
        QuadraticBezierPolynomial {
            a0: from,
            a1: (ctrl - from) * S::TWO,
            a2: from + to - ctrl * S::TWO,
        }
    }
}

// TODO(breaking change): This should not have been a public struct.
// It's not useful to users of the crate but removing it is a breaking change so it
// stays empty for now.
pub struct FlatteningParameters<S> {
    _marker: std::marker::PhantomData<S>
}

impl<S: Scalar> FlatteningParameters<S> {
    pub fn new(_curve: &QuadraticBezierSegment<S>, _tolerance: S) -> Self {
        Self { _marker: std::marker::PhantomData }
    }
}

/// The polynomial form of a quadratic bézier segment.
///
/// The `sample` implementation uses Horner's method and tends to be faster than
/// `QuadraticBezierSegment::sample`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QuadraticBezierPolynomial<S> {
    pub a0: Vector<S>,
    pub a1: Vector<S>,
    pub a2: Vector<S>,
}

impl<S: Scalar> QuadraticBezierPolynomial<S> {
    #[inline(always)]
    pub fn sample(&self, t: S) -> Point<S> {
        // Horner's method.
        let mut v = self.a0;
        let mut t2 = t;
        v += self.a1 * t2;
        t2 *= t;
        v += self.a2 * t2;

        v.to_point()
    }
}

/// Computes the number of line segments required to build a flattened approximation
/// of the curve with segments placed at regular `t` intervals.
fn flattened_segments_wang<S: Scalar>(curve: &QuadraticBezierSegment<S>, tolerance: S) -> S {
    let from = curve.from.to_vector();
    let ctrl = curve.ctrl.to_vector();
    let to = curve.to.to_vector();
    let l = (from - ctrl * S::TWO + to) * S::TWO;
    let d = S::ONE / (S::EIGHT * tolerance);
    let err4 = l.dot(l) * d * d;
    let err4_f32 = err4.to_f32().unwrap_or(1.0);

    // Avoid two square roots using a lookup table that contains
    // i^4 for  i in 1..25.
    const N: usize = 24;
    const LUT: [f32; N] = [
        1.0, 16.0, 81.0, 256.0, 625.0, 1296.0, 2401.0, 4096.0, 6561.0,
        10000.0, 14641.0, 20736.0, 28561.0, 38416.0, 50625.0, 65536.0,
        83521.0, 104976.0, 130321.0, 160000.0, 194481.0, 234256.0,
        279841.0, 331776.0
    ];

    // If the value we are looking for is within the LUT, take the fast path
    if err4_f32 <= 331776.0 {
        #[allow(clippy::needless_range_loop)]
        for i in 0..N {
            if err4_f32 <= LUT[i] {
                return S::from(i + 1).unwrap_or(S::ONE);
            }
        }
    }

    // Otherwise fall back to computing via two square roots.
    err4.sqrt().sqrt().max(S::ONE)
}

/// A flattening iterator for quadratic bézier segments.
///
/// Yields points at each iteration.
pub struct Flattened<S> {
    curve: QuadraticBezierPolynomial<S>,
    to: Point<S>,
    segments: u32,
    step: S,
    t: S,
}

impl<S: Scalar> Flattened<S> {
    #[inline]
    pub(crate) fn new(curve: &QuadraticBezierSegment<S>, tolerance: S) -> Self {
        debug_assert!(tolerance >= S::EPSILON * S::EPSILON);

        let n = flattened_segments_wang(curve, tolerance);

        let step = S::ONE / n;

        Flattened {
            curve: curve.polynomial_form(),
            to: curve.to,
            segments: n.to_u32().unwrap_or(1),
            step,
            t: step
        }
    }
}

impl<S: Scalar> Iterator for Flattened<S> {
    type Item = Point<S>;

    #[inline]
    fn next(&mut self) -> Option<Point<S>> {
        if self.segments == 0 {
            return None;
        }

        self.segments -= 1;
        if self.segments == 0 {
            return Some(self.to)
        }

        let p = self.curve.sample(self.t);
        self.t += self.step;

        Some(p)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.segments as usize;
        (n, Some(n))
    }
}

// TODO(breaking change): is this useful? Either remove
// it or add the equivalent for cubic curves.
/// A flattening iterator for quadratic bézier segments.
///
/// Yields the curve parameter at each iteration.
pub struct FlattenedT<S> {
    segments: u32,
    step: S,
    t: S,
}

impl<S: Scalar> FlattenedT<S> {
    #[inline]
    pub(crate) fn new(curve: &QuadraticBezierSegment<S>, tolerance: S) -> Self {
        debug_assert!(tolerance >= S::EPSILON * S::EPSILON);

        let n = flattened_segments_wang(curve, tolerance);

        let step = S::ONE / n;

        FlattenedT {
            segments: n.to_u32().unwrap_or(1),
            step,
            t: step
        }
    }
}

impl<S: Scalar> Iterator for FlattenedT<S> {
    type Item = S;

    #[inline]
    fn next(&mut self) -> Option<S> {
        if self.segments == 0 {
            return None;
        }

        self.segments -= 1;
        if self.segments == 0 {
            return Some(S::ONE)
        }

        self.t += self.step;

        Some(self.t)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.segments as usize;
        (n, Some(n))
    }
}

impl<S: Scalar> Segment for QuadraticBezierSegment<S> {
    impl_segment!(S);

    fn for_each_flattened_with_t(
        &self,
        tolerance: Self::Scalar,
        callback: &mut dyn FnMut(&LineSegment<S>, Range<S>),
    ) {
        self.for_each_flattened_with_t(tolerance, &mut |s, t| callback(s, t));
    }
}

#[test]
fn bounding_box_for_monotonic_quadratic_bezier_segment() {
    let a = QuadraticBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl: Point::new(0.0, 0.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_aabb = Box2D {
        min: point(0.0, 0.0),
        max: point(2.0, 0.0),
    };

    let actual_aabb = a.bounding_box();

    assert_eq!(expected_aabb, actual_aabb)
}

#[test]
fn fast_bounding_box_for_quadratic_bezier_segment() {
    let a = QuadraticBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl: Point::new(1.0, 1.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_aabb = Box2D {
        min: point(0.0, 0.0),
        max: point(2.0, 1.0),
    };

    let actual_aabb = a.fast_bounding_box();

    assert_eq!(expected_aabb, actual_aabb)
}

#[test]
fn minimum_bounding_box_for_quadratic_bezier_segment() {
    let a = QuadraticBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl: Point::new(1.0, 1.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_aabb = Box2D {
        min: point(0.0, 0.0),
        max: point(2.0, 0.5),
    };

    let actual_aabb = a.bounding_box();

    assert_eq!(expected_aabb, actual_aabb)
}

#[test]
fn y_maximum_t_for_simple_segment() {
    let a = QuadraticBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl: Point::new(1.0, 1.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_y_maximum = 0.5;

    let actual_y_maximum = a.y_maximum_t();

    assert_eq!(expected_y_maximum, actual_y_maximum)
}

#[test]
fn local_y_extremum_for_simple_segment() {
    let a = QuadraticBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl: Point::new(1.0, 1.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_y_inflection = 0.5;

    match a.local_y_extremum_t() {
        Some(actual_y_inflection) => assert_eq!(expected_y_inflection, actual_y_inflection),
        None => panic!(),
    }
}

#[test]
fn y_minimum_t_for_simple_segment() {
    let a = QuadraticBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl: Point::new(1.0, -1.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_y_minimum = 0.5;

    let actual_y_minimum = a.y_minimum_t();

    assert_eq!(expected_y_minimum, actual_y_minimum)
}

#[test]
fn x_maximum_t_for_simple_segment() {
    let a = QuadraticBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl: Point::new(1.0, 1.0),
        to: Point::new(0.0, 2.0),
    };

    let expected_x_maximum = 0.5;

    let actual_x_maximum = a.x_maximum_t();

    assert_eq!(expected_x_maximum, actual_x_maximum)
}

#[test]
fn local_x_extremum_for_simple_segment() {
    let a = QuadraticBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl: Point::new(1.0, 1.0),
        to: Point::new(0.0, 2.0),
    };

    let expected_x_inflection = 0.5;

    match a.local_x_extremum_t() {
        Some(actual_x_inflection) => assert_eq!(expected_x_inflection, actual_x_inflection),
        None => panic!(),
    }
}

#[test]
fn x_minimum_t_for_simple_segment() {
    let a = QuadraticBezierSegment {
        from: Point::new(2.0, 0.0),
        ctrl: Point::new(1.0, 1.0),
        to: Point::new(2.0, 2.0),
    };

    let expected_x_minimum = 0.5;

    let actual_x_minimum = a.x_minimum_t();

    assert_eq!(expected_x_minimum, actual_x_minimum)
}

#[test]
fn length_straight_line() {
    // Sanity check: aligned points so both these curves are straight lines
    // that go form (0.0, 0.0) to (2.0, 0.0).

    let len = QuadraticBezierSegment {
        from: Point::new(0.0f64, 0.0),
        ctrl: Point::new(1.0, 0.0),
        to: Point::new(2.0, 0.0),
    }
    .length();
    assert!((len - 2.0).abs() < 0.000001);

    let len = CubicBezierSegment {
        from: Point::new(0.0f64, 0.0),
        ctrl1: Point::new(1.0, 0.0),
        ctrl2: Point::new(1.0, 0.0),
        to: Point::new(2.0, 0.0),
    }
    .approximate_length(0.0001);
    assert!((len - 2.0).abs() < 0.000001);
}

#[test]
fn derivatives() {
    let c1 = QuadraticBezierSegment {
        from: Point::new(1.0, 1.0),
        ctrl: Point::new(2.0, 1.0),
        to: Point::new(2.0, 2.0),
    };

    assert_eq!(c1.dy(0.0), 0.0);
    assert_eq!(c1.dx(1.0), 0.0);
    assert_eq!(c1.dy(0.5), c1.dx(0.5));
}

#[test]
fn fat_line() {
    use crate::point;

    let c1 = QuadraticBezierSegment {
        from: point(1.0f32, 2.0),
        ctrl: point(1.0, 3.0),
        to: point(11.0, 12.0),
    };

    let (l1, l2) = c1.fat_line();

    for i in 0..100 {
        let t = i as f32 / 99.0;
        assert!(l1.signed_distance_to_point(&c1.sample(t)) >= -0.000001);
        assert!(l2.signed_distance_to_point(&c1.sample(t)) <= 0.000001);
    }
}

#[test]
fn is_linear() {
    let mut angle = 0.0;
    let center = Point::new(1000.0, -700.0);
    for _ in 0..100 {
        for i in 0..10 {
            let (sin, cos) = f64::sin_cos(angle);
            let endpoint = Vector::new(cos * 100.0, sin * 100.0);
            let curve = QuadraticBezierSegment {
                from: center - endpoint,
                ctrl: center + endpoint.lerp(-endpoint, i as f64 / 9.0),
                to: center + endpoint,
            };

            assert!(curve.is_linear(1e-10));
        }
        angle += 0.001;
    }
}

#[test]
fn test_flattening() {
    use crate::point;

    let c1 = QuadraticBezierSegment {
        from: point(0.0, 0.0),
        ctrl: point(5.0, 0.0),
        to: point(5.0, 5.0),
    };

    let c2 = QuadraticBezierSegment {
        from: point(0.0, 0.0),
        ctrl: point(50.0, 0.0),
        to: point(50.0, 50.0),
    };

    let c3 = QuadraticBezierSegment {
        from: point(0.0, 0.0),
        ctrl: point(100.0, 100.0),
        to: point(5.0, 0.0),
    };

    fn check_tolerance(curve: &QuadraticBezierSegment<f64>, tolerance: f64) {
        let mut c = curve.clone();
        loop {
            let t = c.flattening_step(tolerance);
            if t >= 1.0 {
                break;
            }
            let (before, after) = c.split(t);
            let mid_point = before.sample(0.5);
            let distance = before
                .baseline()
                .to_line()
                .equation()
                .distance_to_point(&mid_point);
            assert!(distance <= tolerance);
            c = after;
        }
    }

    check_tolerance(&c1, 1.0);
    check_tolerance(&c1, 0.1);
    check_tolerance(&c1, 0.01);
    check_tolerance(&c1, 0.001);
    check_tolerance(&c1, 0.0001);

    check_tolerance(&c2, 1.0);
    check_tolerance(&c2, 0.1);
    check_tolerance(&c2, 0.01);
    check_tolerance(&c2, 0.001);
    check_tolerance(&c2, 0.0001);

    check_tolerance(&c3, 1.0);
    check_tolerance(&c3, 0.1);
    check_tolerance(&c3, 0.01);
    check_tolerance(&c3, 0.001);
    check_tolerance(&c3, 0.0001);
}

#[test]
fn test_flattening_empty_curve() {
    use crate::point;

    let curve = QuadraticBezierSegment {
        from: point(0.0, 0.0),
        ctrl: point(0.0, 0.0),
        to: point(0.0, 0.0),
    };

    let mut iter = FlattenedT::new(&curve, 0.1);

    assert_eq!(iter.next(), Some(1.0));
    assert_eq!(iter.next(), None);

    let mut count: u32 = 0;
    curve.for_each_flattened(0.1, &mut |_| count += 1);
    assert_eq!(count, 1);
}

#[test]
fn test_flattening_straight_line() {
    use crate::point;

    let curve = QuadraticBezierSegment {
        from: point(0.0, 0.0),
        ctrl: point(10.0, 0.0),
        to: point(20.0, 0.0),
    };

    let mut iter = FlattenedT::new(&curve, 0.1);

    assert_eq!(iter.next(), Some(1.0));
    assert!(iter.next().is_none());

    let mut count: u32 = 0;
    curve.for_each_flattened(0.1, &mut |_| count += 1);
    assert_eq!(count, 1);
}

#[test]
fn issue_678() {
    let points = [
        [-7768.80859375f32, -35563.80859375],
        [-38463.125, -10941.41796875],
        [-21846.12890625, -13518.1953125],
        [-11727.439453125, -22080.33203125],
    ];

    let quadratic = QuadraticBezierSegment {
        from: Point::new(points[0][0], points[0][1]),
        ctrl: Point::new(points[1][0], points[1][1]),
        to: Point::new(points[2][0], points[2][1]),
    };

    let line = Line {
        point: Point::new(points[3][0], points[3][1]),
        vector: Vector::new(-0.5, -0.5).normalize(),
    };

    let intersections = quadratic.line_intersections(&line);
    std::println!("{intersections:?}");

    assert_eq!(intersections.len(), 1);
}

#[test]
fn line_intersections_t() {
    let curve = QuadraticBezierSegment {
        from: point(0.0f64, 0.0),
        ctrl: point(100.0, 0.0),
        to: point(100.0, 500.0),
    };
    let cubic = curve.to_cubic();

    let line = Line {
        point: point(0.0, -50.0),
        vector: crate::vector(100.0, 500.0),
    };

    let mut i1 = curve.line_intersections_t(&line);
    let mut i2 = curve.to_cubic().line_intersections_t(&line);

    use std::cmp::Ordering::{Equal, Greater, Less};
    i1.sort_by(|a, b| {
        if a == b {
            Equal
        } else if a > b {
            Greater
        } else {
            Less
        }
    });
    i2.sort_by(|a, b| {
        if a == b {
            Equal
        } else if a > b {
            Greater
        } else {
            Less
        }
    });

    for (t1, t2) in i1.iter().zip(i2.iter()) {
        use euclid::approxeq::ApproxEq;
        let p1 = curve.sample(*t1);
        let p2 = cubic.sample(*t2);
        assert!(p1.approx_eq(&p2), "{:?} == {:?}", p1, p2);
    }
    assert_eq!(i2.len(), 2);
    assert_eq!(i1.len(), 2);
}

#[test]
fn drag() {
    let curve = QuadraticBezierSegment {
        from: point(0.0f32, 0.0),
        ctrl: point(100.0, 0.0),
        to: point(100.0, 100.0),
    };

    for t in [0.5, 0.25, 0.1, 0.4, 0.7] {
        let target = point(0.0, 10.0);

        let dragged = curve.drag(t, target);

        use euclid::approxeq::ApproxEq;
        let p1 = dragged.sample(t);
        assert!(
            p1.approx_eq_eps(&target, &point(0.001, 0.001)),
            "{:?} == {:?}",
            p1,
            target
        );
    }
}

#[test]
fn arc_length() {
    let curves = [
        QuadraticBezierSegment {
            from: point(0.0f64, 0.0),
            ctrl: point(100.0, 0.0),
            to: point(0.0, 100.0),
        },
        QuadraticBezierSegment {
            from: point(0.0, 0.0),
            ctrl: point(100.0, 0.0),
            to: point(200.0, 0.0),
        },
        QuadraticBezierSegment {
            from: point(100.0, 0.0),
            ctrl: point(0.0, 0.0),
            to: point(50.0, 1.0),
        },
    ];

    for (idx, curve) in curves.iter().enumerate() {
        let length = curve.length();
        let mut accum = 0.0;
        curve.for_each_flattened(0.00000001, &mut |line| {
            accum += line.length();
        });

        assert!(
            (length - accum).abs() < 0.00001,
            "curve {:?}, {:?} == {:?}",
            idx,
            length,
            accum
        );
    }
}
