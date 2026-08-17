use crate::cubic_bezier_intersections::cubic_bezier_intersections_t;
use crate::scalar::Scalar;
use crate::segment::Segment;
use crate::traits::Transformation;
use crate::utils::{cubic_polynomial_roots, min_max};
use crate::{point, Box2D, Point, Vector};
use crate::{Line, LineEquation, LineSegment, QuadraticBezierSegment};
use arrayvec::ArrayVec;

use core::cmp::Ordering::{Equal, Greater, Less};
use core::ops::Range;

#[cfg(test)]
use std::vec::Vec;

/// A 2d curve segment defined by four points: the beginning of the segment, two control
/// points and the end of the segment.
///
/// The curve is defined by equation:²
/// ```∀ t ∈ [0..1],  P(t) = (1 - t)³ * from + 3 * (1 - t)² * t * ctrl1 + 3 * t² * (1 - t) * ctrl2 + t³ * to```
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct CubicBezierSegment<S> {
    pub from: Point<S>,
    pub ctrl1: Point<S>,
    pub ctrl2: Point<S>,
    pub to: Point<S>,
}

impl<S: Scalar> CubicBezierSegment<S> {
    /// Sample the curve at t (expecting t between 0 and 1).
    pub fn sample(&self, t: S) -> Point<S> {
        let t2 = t * t;
        let t3 = t2 * t;
        let one_t = S::ONE - t;
        let one_t2 = one_t * one_t;
        let one_t3 = one_t2 * one_t;

        self.from * one_t3
            + self.ctrl1.to_vector() * S::THREE * one_t2 * t
            + self.ctrl2.to_vector() * S::THREE * one_t * t2
            + self.to.to_vector() * t3
    }

    /// Sample the x coordinate of the curve at t (expecting t between 0 and 1).
    pub fn x(&self, t: S) -> S {
        let t2 = t * t;
        let t3 = t2 * t;
        let one_t = S::ONE - t;
        let one_t2 = one_t * one_t;
        let one_t3 = one_t2 * one_t;

        self.from.x * one_t3
            + self.ctrl1.x * S::THREE * one_t2 * t
            + self.ctrl2.x * S::THREE * one_t * t2
            + self.to.x * t3
    }

    /// Sample the y coordinate of the curve at t (expecting t between 0 and 1).
    pub fn y(&self, t: S) -> S {
        let t2 = t * t;
        let t3 = t2 * t;
        let one_t = S::ONE - t;
        let one_t2 = one_t * one_t;
        let one_t3 = one_t2 * one_t;

        self.from.y * one_t3
            + self.ctrl1.y * S::THREE * one_t2 * t
            + self.ctrl2.y * S::THREE * one_t * t2
            + self.to.y * t3
    }

    /// Return the parameter values corresponding to a given x coordinate.
    pub fn solve_t_for_x(&self, x: S) -> ArrayVec<S, 3> {
        let (min, max) = self.fast_bounding_range_x();
        if min > x || max < x {
            return ArrayVec::new();
        }

        self.parameters_for_xy_value(x, self.from.x, self.ctrl1.x, self.ctrl2.x, self.to.x)
    }

    /// Return the parameter values corresponding to a given y coordinate.
    pub fn solve_t_for_y(&self, y: S) -> ArrayVec<S, 3> {
        let (min, max) = self.fast_bounding_range_y();
        if min > y || max < y {
            return ArrayVec::new();
        }

        self.parameters_for_xy_value(y, self.from.y, self.ctrl1.y, self.ctrl2.y, self.to.y)
    }

    fn parameters_for_xy_value(
        &self,
        value: S,
        from: S,
        ctrl1: S,
        ctrl2: S,
        to: S,
    ) -> ArrayVec<S, 3> {
        let mut result = ArrayVec::new();

        let a = -from + S::THREE * ctrl1 - S::THREE * ctrl2 + to;
        let b = S::THREE * from - S::SIX * ctrl1 + S::THREE * ctrl2;
        let c = -S::THREE * from + S::THREE * ctrl1;
        let d = from - value;

        let roots = cubic_polynomial_roots(a, b, c, d);
        for root in roots {
            if root > S::ZERO && root < S::ONE {
                result.push(root);
            }
        }

        result
    }

    #[inline]
    fn derivative_coefficients(&self, t: S) -> (S, S, S, S) {
        let t2 = t * t;
        (
            -S::THREE * t2 + S::SIX * t - S::THREE,
            S::NINE * t2 - S::value(12.0) * t + S::THREE,
            -S::NINE * t2 + S::SIX * t,
            S::THREE * t2,
        )
    }

    /// Sample the curve's derivative at t (expecting t between 0 and 1).
    pub fn derivative(&self, t: S) -> Vector<S> {
        let (c0, c1, c2, c3) = self.derivative_coefficients(t);
        self.from.to_vector() * c0
            + self.ctrl1.to_vector() * c1
            + self.ctrl2.to_vector() * c2
            + self.to.to_vector() * c3
    }

    /// Sample the x coordinate of the curve's derivative at t (expecting t between 0 and 1).
    pub fn dx(&self, t: S) -> S {
        let (c0, c1, c2, c3) = self.derivative_coefficients(t);
        self.from.x * c0 + self.ctrl1.x * c1 + self.ctrl2.x * c2 + self.to.x * c3
    }

    /// Sample the y coordinate of the curve's derivative at t (expecting t between 0 and 1).
    pub fn dy(&self, t: S) -> S {
        let (c0, c1, c2, c3) = self.derivative_coefficients(t);
        self.from.y * c0 + self.ctrl1.y * c1 + self.ctrl2.y * c2 + self.to.y * c3
    }

    /// Return the sub-curve inside a given range of t.
    ///
    /// This is equivalent to splitting at the range's end points.
    pub fn split_range(&self, t_range: Range<S>) -> Self {
        let (t0, t1) = (t_range.start, t_range.end);
        let from = self.sample(t0);
        let to = self.sample(t1);

        let d = QuadraticBezierSegment {
            from: (self.ctrl1 - self.from).to_point(),
            ctrl: (self.ctrl2 - self.ctrl1).to_point(),
            to: (self.to - self.ctrl2).to_point(),
        };

        let dt = t1 - t0;
        let ctrl1 = from + d.sample(t0).to_vector() * dt;
        let ctrl2 = to - d.sample(t1).to_vector() * dt;

        CubicBezierSegment {
            from,
            ctrl1,
            ctrl2,
            to,
        }
    }

    /// Split this curve into two sub-curves.
    pub fn split(&self, t: S) -> (CubicBezierSegment<S>, CubicBezierSegment<S>) {
        let ctrl1a = self.from + (self.ctrl1 - self.from) * t;
        let ctrl2a = self.ctrl1 + (self.ctrl2 - self.ctrl1) * t;
        let ctrl1aa = ctrl1a + (ctrl2a - ctrl1a) * t;
        let ctrl3a = self.ctrl2 + (self.to - self.ctrl2) * t;
        let ctrl2aa = ctrl2a + (ctrl3a - ctrl2a) * t;
        let ctrl1aaa = ctrl1aa + (ctrl2aa - ctrl1aa) * t;

        (
            CubicBezierSegment {
                from: self.from,
                ctrl1: ctrl1a,
                ctrl2: ctrl1aa,
                to: ctrl1aaa,
            },
            CubicBezierSegment {
                from: ctrl1aaa,
                ctrl1: ctrl2aa,
                ctrl2: ctrl3a,
                to: self.to,
            },
        )
    }

    /// Return the curve before the split point.
    pub fn before_split(&self, t: S) -> CubicBezierSegment<S> {
        let ctrl1a = self.from + (self.ctrl1 - self.from) * t;
        let ctrl2a = self.ctrl1 + (self.ctrl2 - self.ctrl1) * t;
        let ctrl1aa = ctrl1a + (ctrl2a - ctrl1a) * t;
        let ctrl3a = self.ctrl2 + (self.to - self.ctrl2) * t;
        let ctrl2aa = ctrl2a + (ctrl3a - ctrl2a) * t;
        let ctrl1aaa = ctrl1aa + (ctrl2aa - ctrl1aa) * t;

        CubicBezierSegment {
            from: self.from,
            ctrl1: ctrl1a,
            ctrl2: ctrl1aa,
            to: ctrl1aaa,
        }
    }

    /// Return the curve after the split point.
    pub fn after_split(&self, t: S) -> CubicBezierSegment<S> {
        let ctrl1a = self.from + (self.ctrl1 - self.from) * t;
        let ctrl2a = self.ctrl1 + (self.ctrl2 - self.ctrl1) * t;
        let ctrl1aa = ctrl1a + (ctrl2a - ctrl1a) * t;
        let ctrl3a = self.ctrl2 + (self.to - self.ctrl2) * t;
        let ctrl2aa = ctrl2a + (ctrl3a - ctrl2a) * t;

        CubicBezierSegment {
            from: ctrl1aa + (ctrl2aa - ctrl1aa) * t,
            ctrl1: ctrl2a + (ctrl3a - ctrl2a) * t,
            ctrl2: ctrl3a,
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

    /// Returns true if the curve can be approximated with a single line segment, given
    /// a tolerance threshold.
    pub fn is_linear(&self, tolerance: S) -> bool {
        // Similar to Line::square_distance_to_point, except we keep
        // the sign of c1 and c2 to compute tighter upper bounds as we
        // do in fat_line_min_max.
        let baseline = self.to - self.from;
        let v1 = self.ctrl1 - self.from;
        let v2 = self.ctrl2 - self.from;
        let c1 = baseline.cross(v1);
        let c2 = baseline.cross(v2);
        // TODO: it would be faster to multiply the threshold with baseline_len2
        // instead of dividing d1 and d2, but it changes the behavior when the
        // baseline length is zero in ways that breaks some of the cubic intersection
        // tests.
        let inv_baseline_len2 = S::ONE / baseline.square_length();
        let d1 = (c1 * c1) * inv_baseline_len2;
        let d2 = (c2 * c2) * inv_baseline_len2;

        let factor = if (c1 * c2) > S::ZERO {
            S::THREE / S::FOUR
        } else {
            S::FOUR / S::NINE
        };

        let f2 = factor * factor;
        let threshold = tolerance * tolerance;

        d1 * f2 <= threshold && d2 * f2 <= threshold
    }

    /// Returns whether the curve can be approximated with a single point, given
    /// a tolerance threshold.
    pub(crate) fn is_a_point(&self, tolerance: S) -> bool {
        let tolerance_squared = tolerance * tolerance;
        // Use <= so that tolerance can be zero.
        (self.from - self.to).square_length() <= tolerance_squared
            && (self.from - self.ctrl1).square_length() <= tolerance_squared
            && (self.to - self.ctrl2).square_length() <= tolerance_squared
    }

    /// Computes the signed distances (min <= 0 and max >= 0) from the baseline of this
    /// curve to its two "fat line" boundary lines.
    ///
    /// A fat line is two conservative lines between which the segment
    /// is fully contained.
    pub(crate) fn fat_line_min_max(&self) -> (S, S) {
        let baseline = self.baseline().to_line().equation();
        let (d1, d2) = min_max(
            baseline.signed_distance_to_point(&self.ctrl1),
            baseline.signed_distance_to_point(&self.ctrl2),
        );

        let factor = if (d1 * d2) > S::ZERO {
            S::THREE / S::FOUR
        } else {
            S::FOUR / S::NINE
        };

        let d_min = factor * S::min(d1, S::ZERO);
        let d_max = factor * S::max(d2, S::ZERO);

        (d_min, d_max)
    }

    /// Computes a "fat line" of this segment.
    ///
    /// A fat line is two conservative lines between which the segment
    /// is fully contained.
    pub fn fat_line(&self) -> (LineEquation<S>, LineEquation<S>) {
        let baseline = self.baseline().to_line().equation();
        let (d1, d2) = self.fat_line_min_max();

        (baseline.offset(d1), baseline.offset(d2))
    }

    /// Applies the transform to this curve and returns the results.
    #[inline]
    pub fn transformed<T: Transformation<S>>(&self, transform: &T) -> Self {
        CubicBezierSegment {
            from: transform.transform_point(self.from),
            ctrl1: transform.transform_point(self.ctrl1),
            ctrl2: transform.transform_point(self.ctrl2),
            to: transform.transform_point(self.to),
        }
    }

    /// Swap the beginning and the end of the segment.
    pub fn flip(&self) -> Self {
        CubicBezierSegment {
            from: self.to,
            ctrl1: self.ctrl2,
            ctrl2: self.ctrl1,
            to: self.from,
        }
    }

    /// Approximate the curve with a single quadratic bézier segment.
    ///
    /// This is terrible as a general approximation but works if the cubic
    /// curve does not have inflection points and is "flat" enough. Typically
    /// usable after subdividing the curve a few times.
    pub fn to_quadratic(&self) -> QuadraticBezierSegment<S> {
        let c1 = (self.ctrl1 * S::THREE - self.from) * S::HALF;
        let c2 = (self.ctrl2 * S::THREE - self.to) * S::HALF;
        QuadraticBezierSegment {
            from: self.from,
            ctrl: ((c1 + c2) * S::HALF).to_point(),
            to: self.to,
        }
    }

    /// Evaluates an upper bound on the maximum distance between the curve
    /// and its quadratic approximation obtained using `to_quadratic`.
    pub fn to_quadratic_error(&self) -> S {
        // See http://caffeineowl.com/graphics/2d/vectorial/cubic2quad01.html
        S::sqrt(S::THREE) / S::value(36.0)
            * ((self.to - self.ctrl2 * S::THREE) + (self.ctrl1 * S::THREE - self.from)).length()
    }

    /// Returns true if the curve can be safely approximated with a single quadratic bézier
    /// segment given the provided tolerance threshold.
    ///
    /// Equivalent to comparing `to_quadratic_error` with the tolerance threshold, avoiding
    /// the cost of two square roots.
    pub fn is_quadratic(&self, tolerance: S) -> bool {
        S::THREE / S::value(1296.0)
            * ((self.to - self.ctrl2 * S::THREE) + (self.ctrl1 * S::THREE - self.from))
                .square_length()
            <= tolerance * tolerance
    }

    /// Computes the number of quadratic bézier segments required to approximate this cubic curve
    /// given a tolerance threshold.
    ///
    /// Derived by Raph Levien from section 10.6 of Sedeberg's CAGD notes
    /// <https://scholarsarchive.byu.edu/cgi/viewcontent.cgi?article=1000&context=facpub#section.10.6>
    /// and the error metric from the caffein owl blog post <http://caffeineowl.com/graphics/2d/vectorial/cubic2quad01.html>
    pub fn num_quadratics(&self, tolerance: S) -> u32 {
        self.num_quadratics_impl(tolerance)
    }

    fn num_quadratics_impl(&self, tolerance: S) -> u32 {
        debug_assert!(tolerance > S::ZERO);

        let x = self.from.x - S::THREE * self.ctrl1.x + S::THREE * self.ctrl2.x - self.to.x;
        let y = self.from.y - S::THREE * self.ctrl1.y + S::THREE * self.ctrl2.y - self.to.y;

        let err = ((x * x + y * y) / (S::value(432.0) * tolerance * tolerance)).to_f32().unwrap_or(1.0);

        // Avoid computing powf(1/6).ceil() using a lookup table that contains
        // i^6 for i in 1..25.
        const MAX_QUADS: usize = 16;
        const LUT: [f32; MAX_QUADS] = [
            1.0, 64.0, 729.0, 4096.0, 15625.0, 46656.0, 117649.0, 262144.0, 531441.0, 1000000.0,
            1771561.0, 2985984.0, 4826809.0, 7529536.0, 11390625.0, 16777216.0,
        ];

        if err <= 16777216.0 {
            #[allow(clippy::needless_range_loop)]
            for i in 0..MAX_QUADS {
                if err <= LUT[i] {
                    return (i + 1) as u32;
                }
            }
        }

        // If the number of quads does not fit in the LUT, fall back to the
        // expensive computation.
        S::from(err).unwrap().powf(S::ONE / S::SIX).ceil().max(S::ONE).to_u32().unwrap_or(1)
    }

    /// Returns the flattened representation of the curve as an iterator, starting *after* the
    /// current point.
    pub fn flattened(&self, tolerance: S) -> Flattened<S> {
        Flattened::new(self, tolerance)
    }

    /// Invokes a callback for each monotonic part of the segment.
    pub fn for_each_monotonic_range<F>(&self, cb: &mut F)
    where
        F: FnMut(Range<S>),
    {
        let mut extrema: ArrayVec<S, 4> = ArrayVec::new();
        self.for_each_local_x_extremum_t(&mut |t| extrema.push(t));
        self.for_each_local_y_extremum_t(&mut |t| extrema.push(t));
        extrema.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let mut t0 = S::ZERO;
        for &t in &extrema {
            if t != t0 {
                cb(t0..t);
                t0 = t;
            }
        }

        cb(t0..S::ONE);
    }

    /// Invokes a callback for each monotonic part of the segment.
    pub fn for_each_monotonic<F>(&self, cb: &mut F)
    where
        F: FnMut(&CubicBezierSegment<S>),
    {
        self.for_each_monotonic_range(&mut |range| {
            let mut sub = self.split_range(range);
            // Due to finite precision the split may actually result in sub-curves
            // that are almost but not-quite monotonic. Make sure they actually are.
            let min_x = sub.from.x.min(sub.to.x);
            let max_x = sub.from.x.max(sub.to.x);
            let min_y = sub.from.y.min(sub.to.y);
            let max_y = sub.from.y.max(sub.to.y);
            sub.ctrl1.x = sub.ctrl1.x.max(min_x).min(max_x);
            sub.ctrl1.y = sub.ctrl1.y.max(min_y).min(max_y);
            sub.ctrl2.x = sub.ctrl2.x.max(min_x).min(max_x);
            sub.ctrl2.y = sub.ctrl2.y.max(min_y).min(max_y);
            cb(&sub);
        });
    }

    /// Invokes a callback for each y-monotonic part of the segment.
    pub fn for_each_y_monotonic_range<F>(&self, cb: &mut F)
    where
        F: FnMut(Range<S>),
    {
        let mut t0 = S::ZERO;
        self.for_each_local_y_extremum_t(&mut |t| {
            cb(t0..t);
            t0 = t;
        });

        cb(t0..S::ONE);
    }

    /// Invokes a callback for each y-monotonic part of the segment.
    pub fn for_each_y_monotonic<F>(&self, cb: &mut F)
    where
        F: FnMut(&CubicBezierSegment<S>),
    {
        self.for_each_y_monotonic_range(&mut |range| {
            let mut sub = self.split_range(range);
            // Due to finite precision the split may actually result in sub-curves
            // that are almost but not-quite monotonic. Make sure they actually are.
            let min_y = sub.from.y.min(sub.to.y);
            let max_y = sub.from.y.max(sub.to.y);
            sub.ctrl1.y = sub.ctrl1.y.max(min_y).min(max_y);
            sub.ctrl2.y = sub.ctrl2.y.max(min_y).min(max_y);
            cb(&sub);
        });
    }

    /// Invokes a callback for each x-monotonic part of the segment.
    pub fn for_each_x_monotonic_range<F>(&self, cb: &mut F)
    where
        F: FnMut(Range<S>),
    {
        let mut t0 = S::ZERO;
        self.for_each_local_x_extremum_t(&mut |t| {
            cb(t0..t);
            t0 = t;
        });

        cb(t0..S::ONE);
    }

    /// Invokes a callback for each x-monotonic part of the segment.
    pub fn for_each_x_monotonic<F>(&self, cb: &mut F)
    where
        F: FnMut(&CubicBezierSegment<S>),
    {
        self.for_each_x_monotonic_range(&mut |range| {
            let mut sub = self.split_range(range);
            // Due to finite precision the split may actually result in sub-curves
            // that are almost but not-quite monotonic. Make sure they actually are.
            let min_x = sub.from.x.min(sub.to.x);
            let max_x = sub.from.x.max(sub.to.x);
            sub.ctrl1.x = sub.ctrl1.x.max(min_x).min(max_x);
            sub.ctrl2.x = sub.ctrl2.x.max(min_x).min(max_x);
            cb(&sub);
        });
    }

    /// Approximates the cubic bézier curve with sequence of quadratic ones,
    /// invoking a callback at each step.
    pub fn for_each_quadratic_bezier<F>(&self, tolerance: S, cb: &mut F)
    where
        F: FnMut(&QuadraticBezierSegment<S>),
    {
        self.for_each_quadratic_bezier_with_t(tolerance, &mut |quad, _range| cb(quad));
    }

    /// Approximates the cubic bézier curve with sequence of quadratic ones,
    /// invoking a callback at each step.
    pub fn for_each_quadratic_bezier_with_t<F>(&self, tolerance: S, cb: &mut F)
    where
        F: FnMut(&QuadraticBezierSegment<S>, Range<S>),
    {
        debug_assert!(tolerance >= S::EPSILON * S::EPSILON);

        let num_quadratics = S::from(self.num_quadratics_impl(tolerance)).unwrap();
        let step = S::ONE / num_quadratics;
        let n = num_quadratics.to_u32().unwrap_or(1);
        let mut t0 = S::ZERO;
        for _ in 0..(n - 1) {
            let t1 = t0 + step;

            let quad = self.split_range(t0..t1).to_quadratic();
            cb(&quad, t0..t1);

            t0 = t1;
        }

        // Do the last step manually to make sure we finish at t = 1.0 exactly.
        let quad = self.split_range(t0..S::ONE).to_quadratic();
        cb(&quad, t0..S::ONE)
    }

    /// Approximates the curve with sequence of line segments.
    ///
    /// The `tolerance` parameter defines the maximum distance between the curve and
    /// its approximation.
    pub fn for_each_flattened<F: FnMut(&LineSegment<S>)>(&self, tolerance: S, callback: &mut F) {
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

    /// Approximates the curve with sequence of line segments.
    ///
    /// The `tolerance` parameter defines the maximum distance between the curve and
    /// its approximation.
    ///
    /// The end of the t parameter range at the final segment is guaranteed to be equal to `1.0`.
    pub fn for_each_flattened_with_t<F: FnMut(&LineSegment<S>, Range<S>)>(
        &self,
        tolerance: S,
        callback: &mut F,
    ) {
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

    /// Compute the length of the segment using a flattened approximation.
    pub fn approximate_length(&self, tolerance: S) -> S {
        let mut length = S::ZERO;

        self.for_each_quadratic_bezier(tolerance, &mut |quad| {
            length += quad.length();
        });

        length
    }

    /// Invokes a callback at each inflection point if any.
    pub fn for_each_inflection_t<F>(&self, cb: &mut F)
    where
        F: FnMut(S),
    {
        // Find inflection points.
        // See www.faculty.idc.ac.il/arik/quality/appendixa.html for an explanation
        // of this approach.
        let pa = self.ctrl1 - self.from;
        let pb = self.ctrl2.to_vector() - (self.ctrl1.to_vector() * S::TWO) + self.from.to_vector();
        let pc = self.to.to_vector() - (self.ctrl2.to_vector() * S::THREE)
            + (self.ctrl1.to_vector() * S::THREE)
            - self.from.to_vector();

        let a = pb.cross(pc);
        let b = pa.cross(pc);
        let c = pa.cross(pb);

        if S::abs(a) < S::EPSILON {
            // Not a quadratic equation.
            if S::abs(b) < S::EPSILON {
                // Instead of a linear acceleration change we have a constant
                // acceleration change. This means the equation has no solution
                // and there are no inflection points, unless the constant is 0.
                // In that case the curve is a straight line, essentially that means
                // the easiest way to deal with is is by saying there's an inflection
                // point at t == 0. The inflection point approximation range found will
                // automatically extend into infinity.
                if S::abs(c) < S::EPSILON {
                    cb(S::ZERO);
                }
            } else {
                let t = -c / b;
                if in_range(t) {
                    cb(t);
                }
            }

            return;
        }

        fn in_range<S: Scalar>(t: S) -> bool {
            t >= S::ZERO && t < S::ONE
        }

        let discriminant = b * b - S::FOUR * a * c;

        if discriminant < S::ZERO {
            return;
        }

        if discriminant < S::EPSILON {
            let t = -b / (S::TWO * a);

            if in_range(t) {
                cb(t);
            }

            return;
        }

        // This code is derived from https://www2.units.it/ipl/students_area/imm2/files/Numerical_Recipes.pdf page 184.
        // Computing the roots this way avoids precision issues when a, c or both are small.
        let discriminant_sqrt = S::sqrt(discriminant);
        let sign_b = if b >= S::ZERO { S::ONE } else { -S::ONE };
        let q = -S::HALF * (b + sign_b * discriminant_sqrt);
        let mut first_inflection = q / a;
        let mut second_inflection = c / q;

        if first_inflection > second_inflection {
            core::mem::swap(&mut first_inflection, &mut second_inflection);
        }

        if in_range(first_inflection) {
            cb(first_inflection);
        }

        if in_range(second_inflection) {
            cb(second_inflection);
        }
    }

    /// Return local x extrema or None if this curve is monotonic.
    ///
    /// This returns the advancements along the curve, not the actual x position.
    pub fn for_each_local_x_extremum_t<F>(&self, cb: &mut F)
    where
        F: FnMut(S),
    {
        Self::for_each_local_extremum(self.from.x, self.ctrl1.x, self.ctrl2.x, self.to.x, cb)
    }

    /// Return local y extrema or None if this curve is monotonic.
    ///
    /// This returns the advancements along the curve, not the actual y position.
    pub fn for_each_local_y_extremum_t<F>(&self, cb: &mut F)
    where
        F: FnMut(S),
    {
        Self::for_each_local_extremum(self.from.y, self.ctrl1.y, self.ctrl2.y, self.to.y, cb)
    }

    fn for_each_local_extremum<F>(p0: S, p1: S, p2: S, p3: S, cb: &mut F)
    where
        F: FnMut(S),
    {
        // See www.faculty.idc.ac.il/arik/quality/appendixa.html for an explanation
        // The derivative of a cubic bezier curve is a curve representing a second degree polynomial function
        // f(x) = a * x² + b * x + c such as :

        let a = S::THREE * (p3 + S::THREE * (p1 - p2) - p0);
        let b = S::SIX * (p2 - S::TWO * p1 + p0);
        let c = S::THREE * (p1 - p0);

        fn in_range<S: Scalar>(t: S) -> bool {
            t > S::ZERO && t < S::ONE
        }

        // If the derivative is a linear function
        if a == S::ZERO {
            if b != S::ZERO {
                let t = -c / b;
                if in_range(t) {
                    cb(t);
                }
            }
            return;
        }

        let discriminant = b * b - S::FOUR * a * c;

        // There is no Real solution for the equation
        if discriminant < S::ZERO {
            return;
        }

        // There is one Real solution for the equation
        if discriminant == S::ZERO {
            let t = -b / (S::TWO * a);
            if in_range(t) {
                cb(t);
            }
            return;
        }

        // There are two Real solutions for the equation
        let discriminant_sqrt = discriminant.sqrt();

        let mut first_extremum = (-b - discriminant_sqrt) / (S::TWO * a);
        let mut second_extremum = (-b + discriminant_sqrt) / (S::TWO * a);
        if first_extremum > second_extremum {
            core::mem::swap(&mut first_extremum, &mut second_extremum);
        }

        if in_range(first_extremum) {
            cb(first_extremum);
        }

        if in_range(second_extremum) {
            cb(second_extremum);
        }
    }

    /// Find the advancement of the y-most position in the curve.
    ///
    /// This returns the advancement along the curve, not the actual y position.
    pub fn y_maximum_t(&self) -> S {
        let mut max_t = S::ZERO;
        let mut max_y = self.from.y;
        if self.to.y > max_y {
            max_t = S::ONE;
            max_y = self.to.y;
        }
        self.for_each_local_y_extremum_t(&mut |t| {
            let y = self.y(t);
            if y > max_y {
                max_t = t;
                max_y = y;
            }
        });

        max_t
    }

    /// Find the advancement of the y-least position in the curve.
    ///
    /// This returns the advancement along the curve, not the actual y position.
    pub fn y_minimum_t(&self) -> S {
        let mut min_t = S::ZERO;
        let mut min_y = self.from.y;
        if self.to.y < min_y {
            min_t = S::ONE;
            min_y = self.to.y;
        }
        self.for_each_local_y_extremum_t(&mut |t| {
            let y = self.y(t);
            if y < min_y {
                min_t = t;
                min_y = y;
            }
        });

        min_t
    }

    /// Find the advancement of the x-most position in the curve.
    ///
    /// This returns the advancement along the curve, not the actual x position.
    pub fn x_maximum_t(&self) -> S {
        let mut max_t = S::ZERO;
        let mut max_x = self.from.x;
        if self.to.x > max_x {
            max_t = S::ONE;
            max_x = self.to.x;
        }
        self.for_each_local_x_extremum_t(&mut |t| {
            let x = self.x(t);
            if x > max_x {
                max_t = t;
                max_x = x;
            }
        });

        max_t
    }

    /// Find the x-least position in the curve.
    pub fn x_minimum_t(&self) -> S {
        let mut min_t = S::ZERO;
        let mut min_x = self.from.x;
        if self.to.x < min_x {
            min_t = S::ONE;
            min_x = self.to.x;
        }
        self.for_each_local_x_extremum_t(&mut |t| {
            let x = self.x(t);
            if x < min_x {
                min_t = t;
                min_x = x;
            }
        });

        min_t
    }

    /// Returns a conservative rectangle the curve is contained in.
    ///
    /// This method is faster than `bounding_box` but more conservative.
    pub fn fast_bounding_box(&self) -> Box2D<S> {
        let (min_x, max_x) = self.fast_bounding_range_x();
        let (min_y, max_y) = self.fast_bounding_range_y();

        Box2D {
            min: point(min_x, min_y),
            max: point(max_x, max_y),
        }
    }

    /// Returns a conservative range of x that contains this curve.
    #[inline]
    pub fn fast_bounding_range_x(&self) -> (S, S) {
        let min_x = self
            .from
            .x
            .min(self.ctrl1.x)
            .min(self.ctrl2.x)
            .min(self.to.x);
        let max_x = self
            .from
            .x
            .max(self.ctrl1.x)
            .max(self.ctrl2.x)
            .max(self.to.x);

        (min_x, max_x)
    }

    /// Returns a conservative range of y that contains this curve.
    #[inline]
    pub fn fast_bounding_range_y(&self) -> (S, S) {
        let min_y = self
            .from
            .y
            .min(self.ctrl1.y)
            .min(self.ctrl2.y)
            .min(self.to.y);
        let max_y = self
            .from
            .y
            .max(self.ctrl1.y)
            .max(self.ctrl2.y)
            .max(self.to.y);

        (min_y, max_y)
    }

    /// Returns a conservative rectangle that contains the curve.
    #[inline]
    pub fn bounding_box(&self) -> Box2D<S> {
        let (min_x, max_x) = self.bounding_range_x();
        let (min_y, max_y) = self.bounding_range_y();

        Box2D {
            min: point(min_x, min_y),
            max: point(max_x, max_y),
        }
    }

    /// Returns the smallest range of x that contains this curve.
    #[inline]
    pub fn bounding_range_x(&self) -> (S, S) {
        let min_x = self.x(self.x_minimum_t());
        let max_x = self.x(self.x_maximum_t());

        (min_x, max_x)
    }

    /// Returns the smallest range of y that contains this curve.
    #[inline]
    pub fn bounding_range_y(&self) -> (S, S) {
        let min_y = self.y(self.y_minimum_t());
        let max_y = self.y(self.y_maximum_t());

        (min_y, max_y)
    }

    /// Returns whether this segment is monotonic on the x axis.
    pub fn is_x_monotonic(&self) -> bool {
        let mut found = false;
        self.for_each_local_x_extremum_t(&mut |_| {
            found = true;
        });
        !found
    }

    /// Returns whether this segment is monotonic on the y axis.
    pub fn is_y_monotonic(&self) -> bool {
        let mut found = false;
        self.for_each_local_y_extremum_t(&mut |_| {
            found = true;
        });
        !found
    }

    /// Returns whether this segment is fully monotonic.
    pub fn is_monotonic(&self) -> bool {
        self.is_x_monotonic() && self.is_y_monotonic()
    }

    /// Computes the intersections (if any) between this segment and another one.
    ///
    /// The result is provided in the form of the `t` parameters of each point along the curves. To
    /// get the intersection points, sample the curves at the corresponding values.
    ///
    /// Returns endpoint intersections where an endpoint intersects the interior of the other curve,
    /// but not endpoint/endpoint intersections.
    ///
    /// Returns no intersections if either curve is a point.
    pub fn cubic_intersections_t(&self, curve: &CubicBezierSegment<S>) -> ArrayVec<(S, S), 9> {
        cubic_bezier_intersections_t(self, curve)
    }

    /// Computes the intersection points (if any) between this segment and another one.
    pub fn cubic_intersections(&self, curve: &CubicBezierSegment<S>) -> ArrayVec<Point<S>, 9> {
        let intersections = self.cubic_intersections_t(curve);

        let mut result_with_repeats = ArrayVec::<_, 9>::new();
        for (t, _) in intersections {
            result_with_repeats.push(self.sample(t));
        }

        // We can have up to nine "repeated" values here (for example: two lines, each of which
        // overlaps itself 3 times, intersecting in their 3-fold overlaps). We make an effort to
        // dedupe the results, but that's hindered by not having predictable control over how far
        // the repeated intersections can be from each other (and then by the fact that true
        // intersections can be arbitrarily close), so the results will never be perfect.

        let pair_cmp = |s: &Point<S>, t: &Point<S>| {
            if s.x < t.x || (s.x == t.x && s.y < t.y) {
                Less
            } else if s.x == t.x && s.y == t.y {
                Equal
            } else {
                Greater
            }
        };
        result_with_repeats.sort_unstable_by(pair_cmp);
        if result_with_repeats.len() <= 1 {
            return result_with_repeats;
        }

        #[inline]
        fn dist_sq<S: Scalar>(p1: &Point<S>, p2: &Point<S>) -> S {
            (p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)
        }

        let epsilon_squared = S::EPSILON * S::EPSILON;
        let mut result = ArrayVec::new();
        let mut reference_intersection = &result_with_repeats[0];
        result.push(*reference_intersection);
        for i in 1..result_with_repeats.len() {
            let intersection = &result_with_repeats[i];
            if dist_sq(reference_intersection, intersection) < epsilon_squared {
                continue;
            } else {
                result.push(*intersection);
                reference_intersection = intersection;
            }
        }

        result
    }

    /// Computes the intersections (if any) between this segment a quadratic bézier segment.
    ///
    /// The result is provided in the form of the `t` parameters of each point along the curves. To
    /// get the intersection points, sample the curves at the corresponding values.
    ///
    /// Returns endpoint intersections where an endpoint intersects the interior of the other curve,
    /// but not endpoint/endpoint intersections.
    ///
    /// Returns no intersections if either curve is a point.
    pub fn quadratic_intersections_t(
        &self,
        curve: &QuadraticBezierSegment<S>,
    ) -> ArrayVec<(S, S), 9> {
        self.cubic_intersections_t(&curve.to_cubic())
    }

    /// Computes the intersection points (if any) between this segment and a quadratic bézier segment.
    pub fn quadratic_intersections(
        &self,
        curve: &QuadraticBezierSegment<S>,
    ) -> ArrayVec<Point<S>, 9> {
        self.cubic_intersections(&curve.to_cubic())
    }

    /// Computes the intersections (if any) between this segment and a line.
    ///
    /// The result is provided in the form of the `t` parameters of each
    /// point along curve. To get the intersection points, sample the curve
    /// at the corresponding values.
    pub fn line_intersections_t(&self, line: &Line<S>) -> ArrayVec<S, 3> {
        if line.vector.square_length() < S::EPSILON {
            return ArrayVec::new();
        }

        let from = self.from.to_vector();
        let ctrl1 = self.ctrl1.to_vector();
        let ctrl2 = self.ctrl2.to_vector();
        let to = self.to.to_vector();

        let p1 = to - from + (ctrl1 - ctrl2) * S::THREE;
        let p2 = from * S::THREE + (ctrl2 - ctrl1 * S::TWO) * S::THREE;
        let p3 = (ctrl1 - from) * S::THREE;
        let p4 = from;

        let c = line.point.y * line.vector.x - line.point.x * line.vector.y;

        let roots = cubic_polynomial_roots(
            line.vector.y * p1.x - line.vector.x * p1.y,
            line.vector.y * p2.x - line.vector.x * p2.y,
            line.vector.y * p3.x - line.vector.x * p3.y,
            line.vector.y * p4.x - line.vector.x * p4.y + c,
        );

        let mut result = ArrayVec::new();

        for root in roots {
            if root >= S::ZERO && root <= S::ONE {
                result.push(root);
            }
        }

        // TODO: sort the intersections?

        result
    }

    /// Computes the intersection points (if any) between this segment and a line.
    pub fn line_intersections(&self, line: &Line<S>) -> ArrayVec<Point<S>, 3> {
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
    pub fn line_segment_intersections_t(&self, segment: &LineSegment<S>) -> ArrayVec<(S, S), 3> {
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

    pub fn line_segment_intersections(&self, segment: &LineSegment<S>) -> ArrayVec<Point<S>, 3> {
        let intersections = self.line_segment_intersections_t(segment);

        let mut result = ArrayVec::new();
        for (t, _) in intersections {
            result.push(self.sample(t));
        }

        result
    }

    fn baseline_projection(&self, t: S) -> S {
        // See https://pomax.github.io/bezierinfo/#abc
        // We are computing the interpolation factor between
        // `from` and `to` to get the position of C.
        let one_t = S::ONE - t;
        let one_t3 = one_t * one_t * one_t;
        let t3 = t * t * t;

        t3 / (t3 + one_t3)
    }

    fn abc_ratio(&self, t: S) -> S {
        // See https://pomax.github.io/bezierinfo/#abc
        let one_t = S::ONE - t;
        let one_t3 = one_t * one_t * one_t;
        let t3 = t * t * t;

        ((t3 + one_t3 - S::ONE) / (t3 + one_t3)).abs()
    }

    // Returns a quadratic bézier curve built by dragging this curve's point at `t`
    // to a new position, without moving the endpoints.
    //
    // The relative effect on control points is chosen to give a similar "feel" to
    // most vector graphics editors: dragging from near the first endpoint will affect
    // the first control point more than the second control point, etc.
    pub fn drag(&self, t: S, new_position: Point<S>) -> Self {
        // A lot of tweaking could go into making the weight feel as natural as possible.
        let min = S::value(0.1);
        let max = S::value(0.9);
        let weight = if t < min {
            S::ZERO
        } else if t > max {
            S::ONE
        } else {
            (t - min) / (max - min)
        };

        self.drag_with_weight(t, new_position, weight)
    }

    // Returns a quadratic bézier curve built by dragging this curve's point at `t`
    // to a new position, without moving the endpoints.
    //
    // The provided weight specifies the relative effect on control points.
    //  - with `weight = 0.5`, `ctrl1` and `ctrl2` are equally affected,
    //  - with `weight = 0.0`, only `ctrl1` is affected,
    //  - with `weight = 1.0`, only `ctrl2` is affected,
    //  - etc.
    pub fn drag_with_weight(&self, t: S, new_position: Point<S>, weight: S) -> Self {
        // See https://pomax.github.io/bezierinfo/#abc
        //
        //   From-----------Ctrl1
        //    |               \ d1     \
        //    C-------P--------A        \  d12
        //    |                 \d2      \
        //    |                  \        \
        //    To-----------------Ctrl2
        //
        // The ABC relation means we can place the new control points however we like
        // as long as the ratios CA/CP, d1/d12 and d2/d12 remain constant.
        //
        // we use the weight to guide our decisions. A weight of 0.5 would be a uniform
        // displacement (d1 and d2 do not change and both control points are moved by the
        // same amount).
        // The approach is to use the weight interpolate the most constrained control point
        // between it's old position and the position it would have with uniform displacement.
        // then we determine the position of the least constrained control point such that
        // the ratios mentioned earlier remain constant.

        let c = self.from.lerp(self.to, self.baseline_projection(t));
        let cacp_ratio = self.abc_ratio(t);

        let old_pos = self.sample(t);
        // Construct A before and after drag using the constance ca/cp ratio
        let old_a = old_pos + (old_pos - c) / cacp_ratio;
        let new_a = new_position + (new_position - c) / cacp_ratio;

        // Sort ctrl1 and ctrl2 such ctrl1 is the least affected (or most constrained).
        let mut ctrl1 = self.ctrl1;
        let mut ctrl2 = self.ctrl2;
        if t < S::HALF {
            core::mem::swap(&mut ctrl1, &mut ctrl2);
        }

        // Move the most constrained control point by a subset of the uniform displacement
        // depending on the weight.
        let uniform_displacement = new_a - old_a;
        let f = if t < S::HALF {
            S::TWO * weight
        } else {
            S::TWO * (S::ONE - weight)
        };
        let mut new_ctrl1 = ctrl1 + uniform_displacement * f;

        // Now that the most constrained control point is placed there is only one position
        // for the least constrained control point that satisfies the constant ratios.
        let d1_pre = (old_a - ctrl1).length();
        let d12_pre = (self.ctrl2 - self.ctrl1).length();

        let mut new_ctrl2 = new_ctrl1 + (new_a - new_ctrl1) * (d12_pre / d1_pre);

        if t < S::HALF {
            core::mem::swap(&mut new_ctrl1, &mut new_ctrl2);
        }

        CubicBezierSegment {
            from: self.from,
            ctrl1: new_ctrl1,
            ctrl2: new_ctrl2,
            to: self.to,
        }
    }

    pub fn to_f32(&self) -> CubicBezierSegment<f32> {
        CubicBezierSegment {
            from: self.from.to_f32(),
            ctrl1: self.ctrl1.to_f32(),
            ctrl2: self.ctrl2.to_f32(),
            to: self.to.to_f32(),
        }
    }

    pub fn to_f64(&self) -> CubicBezierSegment<f64> {
        CubicBezierSegment {
            from: self.from.to_f64(),
            ctrl1: self.ctrl1.to_f64(),
            ctrl2: self.ctrl2.to_f64(),
            to: self.to.to_f64(),
        }
    }

    pub fn polynomial_form(&self) -> CubicBezierPolynomial<S> {
        CubicBezierPolynomial {
            a0: self.from.to_vector(),
            a1: (self.ctrl1 - self.from) * S::THREE,
            a2: self.from * S::THREE - self.ctrl1 * S::SIX + self.ctrl2.to_vector() * S::THREE,
            a3: self.to - self.from + (self.ctrl1 - self.ctrl2) * S::THREE
        }
    }
}

impl<S: Scalar> Segment for CubicBezierSegment<S> {
    impl_segment!(S);

    fn for_each_flattened_with_t(
        &self,
        tolerance: Self::Scalar,
        callback: &mut dyn FnMut(&LineSegment<S>, Range<S>),
    ) {
        self.for_each_flattened_with_t(tolerance, &mut |s, t| callback(s, t));
    }
}

/// The polynomial form of a cubic bézier segment.
///
/// The `sample` implementation uses Horner's method and tends to be faster than
/// `CubicBezierSegment::sample`.
pub struct CubicBezierPolynomial<S> {
    pub a0: Vector<S>,
    pub a1: Vector<S>,
    pub a2: Vector<S>,
    pub a3: Vector<S>,
}

impl<S: Scalar> CubicBezierPolynomial<S> {
    #[inline(always)]
    pub fn sample(&self, t: S) -> Point<S> {
        // Horner's method.
        let mut v = self.a0;
        let mut t2 = t;
        v += self.a1 * t2;
        t2 *= t;
        v += self.a2 * t2;
        t2 *= t;
        v += self.a3 * t2;

        v.to_point()
    }
}

/// Computes the number of line segments required to build a flattened approximation
/// of the curve with segments placed at regular `t` intervals.
fn flattened_segments_wang<S: Scalar>(curve: &CubicBezierSegment<S>, tolerance: S) -> S {
    let from = curve.from.to_vector();
    let ctrl1 = curve.ctrl1.to_vector();
    let ctrl2 = curve.ctrl2.to_vector();
    let to = curve.to.to_vector();
    let v1 = (from - ctrl1 * S::TWO + ctrl2) * S::SIX;
    let v2 = (ctrl1 - ctrl2 * S::TWO + to) * S::SIX;
    let l = v1.dot(v1).max(v2.dot(v2));
    let d = S::ONE / (S::EIGHT * tolerance);
    let err4 = l * d * d;
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

pub struct Flattened<S: Scalar> {
    curve: CubicBezierPolynomial<S>,
    to: Point<S>,
    segments: u32,
    step: S,
    t: S,
}

impl<S: Scalar> Flattened<S> {
    pub(crate) fn new(curve: &CubicBezierSegment<S>, tolerance: S) -> Self {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.segments as usize;
        (n, Some(n))
    }
}

#[cfg(test)]
fn print_arrays(a: &[Point<f32>], b: &[Point<f32>]) {
    std::println!("left:  {a:?}");
    std::println!("right: {b:?}");
}

#[cfg(test)]
fn assert_approx_eq(a: &[Point<f32>], b: &[Point<f32>]) {
    if a.len() != b.len() {
        print_arrays(a, b);
        panic!("Lengths differ ({} != {})", a.len(), b.len());
    }
    for i in 0..a.len() {
        let threshold = 0.029;
        let dx = f32::abs(a[i].x - b[i].x);
        let dy = f32::abs(a[i].y - b[i].y);
        if dx > threshold || dy > threshold {
            print_arrays(a, b);
            std::println!("diff = {dx:?} {dy:?}");
            panic!("The arrays are not equal");
        }
    }
}

#[test]
fn test_iterator_builder_1() {
    let tolerance = 0.01;
    let c1 = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(1.0, 0.0),
        ctrl2: Point::new(1.0, 1.0),
        to: Point::new(0.0, 1.0),
    };
    let iter_points: Vec<Point<f32>> = c1.flattened(tolerance).collect();
    let mut builder_points = Vec::new();
    c1.for_each_flattened(tolerance, &mut |s| {
        builder_points.push(s.to);
    });

    assert!(iter_points.len() > 2);
    assert_approx_eq(&iter_points[..], &builder_points[..]);
}

#[test]
fn test_iterator_builder_2() {
    let tolerance = 0.01;
    let c1 = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(1.0, 0.0),
        ctrl2: Point::new(0.0, 1.0),
        to: Point::new(1.0, 1.0),
    };
    let iter_points: Vec<Point<f32>> = c1.flattened(tolerance).collect();
    let mut builder_points = Vec::new();
    c1.for_each_flattened(tolerance, &mut |s| {
        builder_points.push(s.to);
    });

    assert!(iter_points.len() > 2);
    assert_approx_eq(&iter_points[..], &builder_points[..]);
}

#[test]
fn test_iterator_builder_3() {
    let tolerance = 0.01;
    let c1 = CubicBezierSegment {
        from: Point::new(141.0, 135.0),
        ctrl1: Point::new(141.0, 130.0),
        ctrl2: Point::new(140.0, 130.0),
        to: Point::new(131.0, 130.0),
    };
    let iter_points: Vec<Point<f32>> = c1.flattened(tolerance).collect();
    let mut builder_points = Vec::new();
    c1.for_each_flattened(tolerance, &mut |s| {
        builder_points.push(s.to);
    });

    assert!(iter_points.len() > 2);
    assert_approx_eq(&iter_points[..], &builder_points[..]);
}

#[test]
fn test_issue_19() {
    let tolerance = 0.15;
    let c1 = CubicBezierSegment {
        from: Point::new(11.71726, 9.07143),
        ctrl1: Point::new(1.889879, 13.22917),
        ctrl2: Point::new(18.142855, 19.27679),
        to: Point::new(18.142855, 19.27679),
    };
    let iter_points: Vec<Point<f32>> = c1.flattened(tolerance).collect();
    let mut builder_points = Vec::new();
    c1.for_each_flattened(tolerance, &mut |s| {
        builder_points.push(s.to);
    });

    assert_approx_eq(&iter_points[..], &builder_points[..]);

    assert!(iter_points.len() > 1);
}

#[test]
fn test_issue_194() {
    let segment = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(0.0, 0.0),
        ctrl2: Point::new(50.0, 70.0),
        to: Point::new(100.0, 100.0),
    };

    let mut points = Vec::new();
    segment.for_each_flattened(0.1, &mut |s| {
        points.push(s.to);
    });

    assert!(points.len() > 2);
}

#[test]
fn flatten_with_t() {
    let segment = CubicBezierSegment {
        from: Point::new(0.0f32, 0.0),
        ctrl1: Point::new(0.0, 0.0),
        ctrl2: Point::new(50.0, 70.0),
        to: Point::new(100.0, 100.0),
    };

    for tolerance in &[0.1, 0.01, 0.001, 0.0001] {
        let tolerance = *tolerance;

        let mut a = Vec::new();
        segment.for_each_flattened(tolerance, &mut |s| {
            a.push(*s);
        });

        let mut b = Vec::new();
        let mut ts = Vec::new();
        segment.for_each_flattened_with_t(tolerance, &mut |s, t| {
            b.push(*s);
            ts.push(t);
        });

        assert_eq!(a, b);

        for i in 0..b.len() {
            let sampled = segment.sample(ts[i].start);
            let point = b[i].from;
            let dist = (sampled - point).length();
            assert!(dist <= tolerance);

            let sampled = segment.sample(ts[i].end);
            let point = b[i].to;
            let dist = (sampled - point).length();
            assert!(dist <= tolerance);
        }
    }
}

#[test]
fn test_flatten_end() {
    let segment = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(100.0, 0.0),
        ctrl2: Point::new(100.0, 100.0),
        to: Point::new(100.0, 200.0),
    };

    let mut last = segment.from;
    segment.for_each_flattened(0.0001, &mut |s| {
        last = s.to;
    });

    assert_eq!(last, segment.to);
}

#[test]
fn test_flatten_point() {
    let segment = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(0.0, 0.0),
        ctrl2: Point::new(0.0, 0.0),
        to: Point::new(0.0, 0.0),
    };

    let mut last = segment.from;
    segment.for_each_flattened(0.0001, &mut |s| {
        last = s.to;
    });

    assert_eq!(last, segment.to);
}

#[test]
fn issue_652() {
    use crate::point;

    let curve = CubicBezierSegment {
        from: point(-1061.0, -3327.0),
        ctrl1: point(-1061.0, -3177.0),
        ctrl2: point(-1061.0, -3477.0),
        to: point(-1061.0, -3327.0),
    };

    for _ in curve.flattened(1.0) {}
    for _ in curve.flattened(0.1) {}
    for _ in curve.flattened(0.01) {}

    curve.for_each_flattened(1.0, &mut |_| {});
    curve.for_each_flattened(0.1, &mut |_| {});
    curve.for_each_flattened(0.01, &mut |_| {});
}

#[test]
fn fast_bounding_box_for_cubic_bezier_segment() {
    let a = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(0.5, 1.0),
        ctrl2: Point::new(1.5, -1.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_aabb = Box2D {
        min: point(0.0, -1.0),
        max: point(2.0, 1.0),
    };

    let actual_aabb = a.fast_bounding_box();

    assert_eq!(expected_aabb, actual_aabb)
}

#[test]
fn minimum_bounding_box_for_cubic_bezier_segment() {
    let a = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(0.5, 2.0),
        ctrl2: Point::new(1.5, -2.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_bigger_aabb: Box2D<f32> = Box2D {
        min: point(0.0, -0.6),
        max: point(2.0, 0.6),
    };
    let expected_smaller_aabb: Box2D<f32> = Box2D {
        min: point(0.1, -0.5),
        max: point(2.0, 0.5),
    };

    let actual_minimum_aabb = a.bounding_box();

    assert!(expected_bigger_aabb.contains_box(&actual_minimum_aabb));
    assert!(actual_minimum_aabb.contains_box(&expected_smaller_aabb));
}

#[test]
fn y_maximum_t_for_simple_cubic_segment() {
    let a = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(0.5, 1.0),
        ctrl2: Point::new(1.5, 1.0),
        to: Point::new(2.0, 2.0),
    };

    let expected_y_maximum = 1.0;

    let actual_y_maximum = a.y_maximum_t();

    assert_eq!(expected_y_maximum, actual_y_maximum)
}

#[test]
fn y_minimum_t_for_simple_cubic_segment() {
    let a = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(0.5, 1.0),
        ctrl2: Point::new(1.5, 1.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_y_minimum = 0.0;

    let actual_y_minimum = a.y_minimum_t();

    assert_eq!(expected_y_minimum, actual_y_minimum)
}

#[test]
fn y_extrema_for_simple_cubic_segment() {
    let a = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(1.0, 2.0),
        ctrl2: Point::new(2.0, 2.0),
        to: Point::new(3.0, 0.0),
    };

    let mut n: u32 = 0;
    a.for_each_local_y_extremum_t(&mut |t| {
        assert_eq!(t, 0.5);
        n += 1;
    });
    assert_eq!(n, 1);
}

#[test]
fn x_extrema_for_simple_cubic_segment() {
    let a = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(1.0, 2.0),
        ctrl2: Point::new(1.0, 2.0),
        to: Point::new(0.0, 0.0),
    };

    let mut n: u32 = 0;
    a.for_each_local_x_extremum_t(&mut |t| {
        assert_eq!(t, 0.5);
        n += 1;
    });
    assert_eq!(n, 1);
}

#[test]
fn x_maximum_t_for_simple_cubic_segment() {
    let a = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(0.5, 1.0),
        ctrl2: Point::new(1.5, 1.0),
        to: Point::new(2.0, 0.0),
    };
    let expected_x_maximum = 1.0;

    let actual_x_maximum = a.x_maximum_t();

    assert_eq!(expected_x_maximum, actual_x_maximum)
}

#[test]
fn x_minimum_t_for_simple_cubic_segment() {
    let a = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(0.5, 1.0),
        ctrl2: Point::new(1.5, 1.0),
        to: Point::new(2.0, 0.0),
    };

    let expected_x_minimum = 0.0;

    let actual_x_minimum = a.x_minimum_t();

    assert_eq!(expected_x_minimum, actual_x_minimum)
}

#[test]
fn derivatives() {
    let c1 = CubicBezierSegment {
        from: Point::new(1.0, 1.0),
        ctrl1: Point::new(1.0, 2.0),
        ctrl2: Point::new(2.0, 1.0),
        to: Point::new(2.0, 2.0),
    };

    assert_eq!(c1.dx(0.0), 0.0);
    assert_eq!(c1.dx(1.0), 0.0);
    assert_eq!(c1.dy(0.5), 0.0);
}

#[test]
fn fat_line() {
    use crate::point;

    let c1 = CubicBezierSegment {
        from: point(1.0f32, 2.0),
        ctrl1: point(1.0, 3.0),
        ctrl2: point(11.0, 11.0),
        to: point(11.0, 12.0),
    };

    let (l1, l2) = c1.fat_line();

    for i in 0..100 {
        let t = i as f32 / 99.0;
        assert!(l1.signed_distance_to_point(&c1.sample(t)) >= -0.000001);
        assert!(l2.signed_distance_to_point(&c1.sample(t)) <= 0.000001);
    }

    let c2 = CubicBezierSegment {
        from: point(1.0f32, 2.0),
        ctrl1: point(1.0, 3.0),
        ctrl2: point(11.0, 14.0),
        to: point(11.0, 12.0),
    };

    let (l1, l2) = c2.fat_line();

    for i in 0..100 {
        let t = i as f32 / 99.0;
        assert!(l1.signed_distance_to_point(&c2.sample(t)) >= -0.000001);
        assert!(l2.signed_distance_to_point(&c2.sample(t)) <= 0.000001);
    }

    let c3 = CubicBezierSegment {
        from: point(0.0f32, 1.0),
        ctrl1: point(0.5, 0.0),
        ctrl2: point(0.5, 0.0),
        to: point(1.0, 1.0),
    };

    let (l1, l2) = c3.fat_line();

    for i in 0..100 {
        let t = i as f32 / 99.0;
        assert!(l1.signed_distance_to_point(&c3.sample(t)) >= -0.000001);
        assert!(l2.signed_distance_to_point(&c3.sample(t)) <= 0.000001);
    }
}

#[test]
fn is_linear() {
    let mut angle = 0.0;
    let center = Point::new(1000.0, -700.0);
    for _ in 0..100 {
        for i in 0..10 {
            for j in 0..10 {
                let (sin, cos) = f64::sin_cos(angle);
                let endpoint = Vector::new(cos * 100.0, sin * 100.0);
                let curve = CubicBezierSegment {
                    from: center - endpoint,
                    ctrl1: center + endpoint.lerp(-endpoint, i as f64 / 9.0),
                    ctrl2: center + endpoint.lerp(-endpoint, j as f64 / 9.0),
                    to: center + endpoint,
                };
                assert!(curve.is_linear(1e-10));
            }
        }
        angle += 0.001;
    }
}

#[test]
fn test_monotonic() {
    use crate::point;
    let curve = CubicBezierSegment {
        from: point(1.0, 1.0),
        ctrl1: point(10.0, 2.0),
        ctrl2: point(1.0, 3.0),
        to: point(10.0, 4.0),
    };

    curve.for_each_monotonic_range(&mut |range| {
        let sub_curve = curve.split_range(range);
        assert!(sub_curve.is_monotonic());
    });
}

#[test]
fn test_line_segment_intersections() {
    use crate::point;
    fn assert_approx_eq(a: ArrayVec<(f32, f32), 3>, b: &[(f32, f32)], epsilon: f32) {
        for i in 0..a.len() {
            if f32::abs(a[i].0 - b[i].0) > epsilon || f32::abs(a[i].1 - b[i].1) > epsilon {
                std::println!("{a:?} != {b:?}");
            }
            assert!((a[i].0 - b[i].0).abs() <= epsilon && (a[i].1 - b[i].1).abs() <= epsilon);
        }
        assert_eq!(a.len(), b.len());
    }

    let epsilon = 0.0001;

    // Make sure we find intersections with horizontal and vertical lines.

    let curve1 = CubicBezierSegment {
        from: point(-1.0, -1.0),
        ctrl1: point(0.0, 4.0),
        ctrl2: point(10.0, -4.0),
        to: point(11.0, 1.0),
    };
    let seg1 = LineSegment {
        from: point(0.0, 0.0),
        to: point(10.0, 0.0),
    };
    assert_approx_eq(
        curve1.line_segment_intersections_t(&seg1),
        &[(0.5, 0.5)],
        epsilon,
    );

    let curve2 = CubicBezierSegment {
        from: point(-1.0, 0.0),
        ctrl1: point(0.0, 5.0),
        ctrl2: point(0.0, 5.0),
        to: point(1.0, 0.0),
    };
    let seg2 = LineSegment {
        from: point(0.0, 0.0),
        to: point(0.0, 5.0),
    };
    assert_approx_eq(
        curve2.line_segment_intersections_t(&seg2),
        &[(0.5, 0.75)],
        epsilon,
    );
}

#[test]
fn test_parameters_for_value() {
    use crate::point;
    fn assert_approx_eq(a: ArrayVec<f32, 3>, b: &[f32], epsilon: f32) {
        for i in 0..a.len() {
            if f32::abs(a[i] - b[i]) > epsilon {
                std::println!("{a:?} != {b:?}");
            }
            assert!((a[i] - b[i]).abs() <= epsilon);
        }
        assert_eq!(a.len(), b.len());
    }

    {
        let curve = CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(0.0, 8.0),
            ctrl2: point(10.0, 8.0),
            to: point(10.0, 0.0),
        };

        let epsilon = 1e-4;
        assert_approx_eq(curve.solve_t_for_x(5.0), &[0.5], epsilon);
        assert_approx_eq(curve.solve_t_for_y(6.0), &[0.5], epsilon);
    }
    {
        let curve = CubicBezierSegment {
            from: point(0.0, 10.0),
            ctrl1: point(0.0, 10.0),
            ctrl2: point(10.0, 10.0),
            to: point(10.0, 10.0),
        };

        assert_approx_eq(curve.solve_t_for_y(10.0), &[], 0.0);
    }
}

#[test]
fn test_cubic_intersection_deduping() {
    use crate::point;

    let epsilon = 0.0001;

    // Two "line segments" with 3-fold overlaps, intersecting in their overlaps for a total of nine
    // parameter intersections.
    let line1 = CubicBezierSegment {
        from: point(-1_000_000.0, 0.0),
        ctrl1: point(2_000_000.0, 3_000_000.0),
        ctrl2: point(-2_000_000.0, -1_000_000.0),
        to: point(1_000_000.0, 2_000_000.0),
    };
    let line2 = CubicBezierSegment {
        from: point(-1_000_000.0, 2_000_000.0),
        ctrl1: point(2_000_000.0, -1_000_000.0),
        ctrl2: point(-2_000_000.0, 3_000_000.0),
        to: point(1_000_000.0, 0.0),
    };
    let intersections = line1.cubic_intersections(&line2);
    // (If you increase the coordinates above to 10s of millions, you get two returned intersection
    // points; i.e. the test fails.)
    assert_eq!(intersections.len(), 1);
    assert!(f64::abs(intersections[0].x) < epsilon);
    assert!(f64::abs(intersections[0].y - 1_000_000.0) < epsilon);

    // Two self-intersecting curves that intersect in their self-intersections, for a total of four
    // parameter intersections.
    let curve1 = CubicBezierSegment {
        from: point(-10.0, -13.636363636363636),
        ctrl1: point(15.0, 11.363636363636363),
        ctrl2: point(-15.0, 11.363636363636363),
        to: point(10.0, -13.636363636363636),
    };
    let curve2 = CubicBezierSegment {
        from: point(13.636363636363636, -10.0),
        ctrl1: point(-11.363636363636363, 15.0),
        ctrl2: point(-11.363636363636363, -15.0),
        to: point(13.636363636363636, 10.0),
    };
    let intersections = curve1.cubic_intersections(&curve2);
    assert_eq!(intersections.len(), 1);
    assert!(f64::abs(intersections[0].x) < epsilon);
    assert!(f64::abs(intersections[0].y) < epsilon);
}

#[test]
fn cubic_line_intersection_on_endpoint() {
    let l1 = LineSegment {
        from: Point::new(0.0, -100.0),
        to: Point::new(0.0, 100.0),
    };

    let cubic = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(20.0, 20.0),
        ctrl2: Point::new(20.0, 40.0),
        to: Point::new(0.0, 60.0),
    };

    let intersections = cubic.line_segment_intersections_t(&l1);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0], (1.0, 0.8));
    assert_eq!(intersections[1], (0.0, 0.5));

    let l2 = LineSegment {
        from: Point::new(0.0, 0.0),
        to: Point::new(0.0, 60.0),
    };

    let intersections = cubic.line_segment_intersections_t(&l2);

    assert!(intersections.is_empty());

    let c1 = CubicBezierSegment {
        from: Point::new(0.0, 0.0),
        ctrl1: Point::new(20.0, 0.0),
        ctrl2: Point::new(20.0, 20.0),
        to: Point::new(0.0, 60.0),
    };

    let c2 = CubicBezierSegment {
        from: Point::new(0.0, 60.0),
        ctrl1: Point::new(-40.0, 4.0),
        ctrl2: Point::new(-20.0, 20.0),
        to: Point::new(0.0, 00.0),
    };

    let intersections = c1.cubic_intersections_t(&c2);

    assert!(intersections.is_empty());
}

#[test]
fn test_cubic_to_quadratics() {
    use euclid::approxeq::ApproxEq;

    let quadratic = QuadraticBezierSegment {
        from: point(1.0, 2.0),
        ctrl: point(10.0, 5.0),
        to: point(0.0, 1.0),
    };

    let mut count = 0;
    assert_eq!(quadratic.to_cubic().num_quadratics(0.0001), 1);
    quadratic
        .to_cubic()
        .for_each_quadratic_bezier(0.0001, &mut |c| {
            assert_eq!(count, 0);
            assert!(c.from.approx_eq(&quadratic.from));
            assert!(c.ctrl.approx_eq(&quadratic.ctrl));
            assert!(c.to.approx_eq(&quadratic.to));
            count += 1;
        });

    let cubic = CubicBezierSegment {
        from: point(1.0f32, 1.0),
        ctrl1: point(10.0, 2.0),
        ctrl2: point(1.0, 3.0),
        to: point(10.0, 4.0),
    };

    let mut prev = cubic.from;
    let mut count = 0;
    cubic.for_each_quadratic_bezier(0.01, &mut |c| {
        assert!(c.from.approx_eq(&prev));
        prev = c.to;
        count += 1;
    });
    assert!(prev.approx_eq(&cubic.to));
    assert!(count < 10);
    assert!(count > 4);
}
