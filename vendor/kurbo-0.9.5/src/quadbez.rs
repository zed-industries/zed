// Copyright 2018 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Quadratic Bézier segments.

use core::ops::{Mul, Range};

use arrayvec::ArrayVec;

use crate::common::solve_cubic;
use crate::MAX_EXTREMA;
use crate::{
    Affine, CubicBez, Line, Nearest, ParamCurve, ParamCurveArclen, ParamCurveArea,
    ParamCurveCurvature, ParamCurveDeriv, ParamCurveExtrema, ParamCurveNearest, PathEl, Point,
    Rect, Shape,
};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// A single quadratic Bézier segment.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub struct QuadBez {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
}

impl QuadBez {
    /// Create a new quadratic Bézier segment.
    #[inline]
    pub fn new<V: Into<Point>>(p0: V, p1: V, p2: V) -> QuadBez {
        QuadBez {
            p0: p0.into(),
            p1: p1.into(),
            p2: p2.into(),
        }
    }

    /// Raise the order by 1.
    ///
    /// Returns a cubic Bézier segment that exactly represents this quadratic.
    #[inline]
    pub fn raise(&self) -> CubicBez {
        CubicBez::new(
            self.p0,
            self.p0 + (2.0 / 3.0) * (self.p1 - self.p0),
            self.p2 + (2.0 / 3.0) * (self.p1 - self.p2),
            self.p2,
        )
    }

    /// Estimate the number of subdivisions for flattening.
    pub(crate) fn estimate_subdiv(&self, sqrt_tol: f64) -> FlattenParams {
        // Determine transformation to $y = x^2$ parabola.
        let d01 = self.p1 - self.p0;
        let d12 = self.p2 - self.p1;
        let dd = d01 - d12;
        let cross = (self.p2 - self.p0).cross(dd);
        let x0 = d01.dot(dd) * cross.recip();
        let x2 = d12.dot(dd) * cross.recip();
        let scale = (cross / (dd.hypot() * (x2 - x0))).abs();

        // Compute number of subdivisions needed.
        let a0 = approx_parabola_integral(x0);
        let a2 = approx_parabola_integral(x2);
        let val = if scale.is_finite() {
            let da = (a2 - a0).abs();
            let sqrt_scale = scale.sqrt();
            if x0.signum() == x2.signum() {
                da * sqrt_scale
            } else {
                // Handle cusp case (segment contains curvature maximum)
                let xmin = sqrt_tol / sqrt_scale;
                sqrt_tol * da / approx_parabola_integral(xmin)
            }
        } else {
            0.0
        };
        let u0 = approx_parabola_inv_integral(a0);
        let u2 = approx_parabola_inv_integral(a2);
        let uscale = (u2 - u0).recip();
        FlattenParams {
            a0,
            a2,
            u0,
            uscale,
            val,
        }
    }

    // Maps a value from 0..1 to 0..1.
    pub(crate) fn determine_subdiv_t(&self, params: &FlattenParams, x: f64) -> f64 {
        let a = params.a0 + (params.a2 - params.a0) * x;
        let u = approx_parabola_inv_integral(a);
        (u - params.u0) * params.uscale
    }

    /// Is this quadratic Bezier curve finite?
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.p0.is_finite() && self.p1.is_finite() && self.p2.is_finite()
    }

    /// Is this quadratic Bezier curve NaN?
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.p0.is_nan() || self.p1.is_nan() || self.p2.is_nan()
    }
}

/// An iterator for quadratic beziers.
pub struct QuadBezIter {
    quad: QuadBez,
    ix: usize,
}

impl Shape for QuadBez {
    type PathElementsIter<'iter> = QuadBezIter;

    #[inline]
    fn path_elements(&self, _tolerance: f64) -> QuadBezIter {
        QuadBezIter { quad: *self, ix: 0 }
    }

    fn area(&self) -> f64 {
        0.0
    }

    #[inline]
    fn perimeter(&self, accuracy: f64) -> f64 {
        self.arclen(accuracy)
    }

    fn winding(&self, _pt: Point) -> i32 {
        0
    }

    #[inline]
    fn bounding_box(&self) -> Rect {
        ParamCurveExtrema::bounding_box(self)
    }
}

impl Iterator for QuadBezIter {
    type Item = PathEl;

    fn next(&mut self) -> Option<PathEl> {
        self.ix += 1;
        match self.ix {
            1 => Some(PathEl::MoveTo(self.quad.p0)),
            2 => Some(PathEl::QuadTo(self.quad.p1, self.quad.p2)),
            _ => None,
        }
    }
}

pub(crate) struct FlattenParams {
    a0: f64,
    a2: f64,
    u0: f64,
    uscale: f64,
    /// The number of subdivisions * 2 * sqrt_tol.
    pub(crate) val: f64,
}

/// An approximation to $\int (1 + 4x^2) ^ -0.25 dx$
///
/// This is used for flattening curves.
fn approx_parabola_integral(x: f64) -> f64 {
    const D: f64 = 0.67;
    x / (1.0 - D + (D.powi(4) + 0.25 * x * x).sqrt().sqrt())
}

/// An approximation to the inverse parabola integral.
fn approx_parabola_inv_integral(x: f64) -> f64 {
    const B: f64 = 0.39;
    x * (1.0 - B + (B * B + 0.25 * x * x).sqrt())
}

impl ParamCurve for QuadBez {
    #[inline]
    fn eval(&self, t: f64) -> Point {
        let mt = 1.0 - t;
        (self.p0.to_vec2() * (mt * mt)
            + (self.p1.to_vec2() * (mt * 2.0) + self.p2.to_vec2() * t) * t)
            .to_point()
    }

    fn subsegment(&self, range: Range<f64>) -> QuadBez {
        let (t0, t1) = (range.start, range.end);
        let p0 = self.eval(t0);
        let p2 = self.eval(t1);
        let p1 = p0 + (self.p1 - self.p0).lerp(self.p2 - self.p1, t0) * (t1 - t0);
        QuadBez { p0, p1, p2 }
    }

    /// Subdivide into halves, using de Casteljau.
    #[inline]
    fn subdivide(&self) -> (QuadBez, QuadBez) {
        let pm = self.eval(0.5);
        (
            QuadBez::new(self.p0, self.p0.midpoint(self.p1), pm),
            QuadBez::new(pm, self.p1.midpoint(self.p2), self.p2),
        )
    }

    #[inline]
    fn start(&self) -> Point {
        self.p0
    }

    #[inline]
    fn end(&self) -> Point {
        self.p2
    }
}

impl ParamCurveDeriv for QuadBez {
    type DerivResult = Line;

    #[inline]
    fn deriv(&self) -> Line {
        Line::new(
            (2.0 * (self.p1.to_vec2() - self.p0.to_vec2())).to_point(),
            (2.0 * (self.p2.to_vec2() - self.p1.to_vec2())).to_point(),
        )
    }
}

impl ParamCurveArclen for QuadBez {
    /// Arclength of a quadratic Bézier segment.
    ///
    /// This computation is based on an analytical formula. Since that formula suffers
    /// from numerical instability when the curve is very close to a straight line, we
    /// detect that case and fall back to Legendre-Gauss quadrature.
    ///
    /// Accuracy should be better than 1e-13 over the entire range.
    ///
    /// Adapted from <http://www.malczak.linuxpl.com/blog/quadratic-bezier-curve-length/>
    /// with permission.
    fn arclen(&self, _accuracy: f64) -> f64 {
        let d2 = self.p0.to_vec2() - 2.0 * self.p1.to_vec2() + self.p2.to_vec2();
        let a = d2.hypot2();
        let d1 = self.p1 - self.p0;
        let c = d1.hypot2();
        if a < 5e-4 * c {
            // This case happens for nearly straight Béziers.
            //
            // Calculate arclength using Legendre-Gauss quadrature using formula from Behdad
            // in https://github.com/Pomax/BezierInfo-2/issues/77
            let v0 = (-0.492943519233745 * self.p0.to_vec2()
                + 0.430331482911935 * self.p1.to_vec2()
                + 0.0626120363218102 * self.p2.to_vec2())
            .hypot();
            let v1 = ((self.p2 - self.p0) * 0.4444444444444444).hypot();
            let v2 = (-0.0626120363218102 * self.p0.to_vec2()
                - 0.430331482911935 * self.p1.to_vec2()
                + 0.492943519233745 * self.p2.to_vec2())
            .hypot();
            return v0 + v1 + v2;
        }
        let b = 2.0 * d2.dot(d1);

        let sabc = (a + b + c).sqrt();
        let a2 = a.powf(-0.5);
        let a32 = a2.powi(3);
        let c2 = 2.0 * c.sqrt();
        let ba_c2 = b * a2 + c2;

        let v0 = 0.25 * a2 * a2 * b * (2.0 * sabc - c2) + sabc;
        // TODO: justify and fine-tune this exact constant.
        if ba_c2 < 1e-13 {
            // This case happens for Béziers with a sharp kink.
            v0
        } else {
            v0 + 0.25
                * a32
                * (4.0 * c * a - b * b)
                * (((2.0 * a + b) * a2 + 2.0 * sabc) / ba_c2).ln()
        }
    }
}

impl ParamCurveArea for QuadBez {
    #[inline]
    fn signed_area(&self) -> f64 {
        (self.p0.x * (2.0 * self.p1.y + self.p2.y) + 2.0 * self.p1.x * (self.p2.y - self.p0.y)
            - self.p2.x * (self.p0.y + 2.0 * self.p1.y))
            * (1.0 / 6.0)
    }
}

impl ParamCurveNearest for QuadBez {
    /// Find the nearest point, using analytical algorithm based on cubic root finding.
    fn nearest(&self, p: Point, _accuracy: f64) -> Nearest {
        fn eval_t(p: Point, t_best: &mut f64, r_best: &mut Option<f64>, t: f64, p0: Point) {
            let r = (p0 - p).hypot2();
            if r_best.map(|r_best| r < r_best).unwrap_or(true) {
                *r_best = Some(r);
                *t_best = t;
            }
        }
        fn try_t(
            q: &QuadBez,
            p: Point,
            t_best: &mut f64,
            r_best: &mut Option<f64>,
            t: f64,
        ) -> bool {
            if !(0.0..=1.0).contains(&t) {
                return true;
            }
            eval_t(p, t_best, r_best, t, q.eval(t));
            false
        }
        let d0 = self.p1 - self.p0;
        let d1 = self.p0.to_vec2() + self.p2.to_vec2() - 2.0 * self.p1.to_vec2();
        let d = self.p0 - p;
        let c0 = d.dot(d0);
        let c1 = 2.0 * d0.hypot2() + d.dot(d1);
        let c2 = 3.0 * d1.dot(d0);
        let c3 = d1.hypot2();
        let roots = solve_cubic(c0, c1, c2, c3);
        let mut r_best = None;
        let mut t_best = 0.0;
        let mut need_ends = false;
        for &t in &roots {
            need_ends |= try_t(self, p, &mut t_best, &mut r_best, t);
        }
        if need_ends {
            eval_t(p, &mut t_best, &mut r_best, 0.0, self.p0);
            eval_t(p, &mut t_best, &mut r_best, 1.0, self.p2);
        }

        Nearest {
            t: t_best,
            distance_sq: r_best.unwrap(),
        }
    }
}

impl ParamCurveCurvature for QuadBez {}

impl ParamCurveExtrema for QuadBez {
    fn extrema(&self) -> ArrayVec<f64, MAX_EXTREMA> {
        let mut result = ArrayVec::new();
        let d0 = self.p1 - self.p0;
        let d1 = self.p2 - self.p1;
        let dd = d1 - d0;
        if dd.x != 0.0 {
            let t = -d0.x / dd.x;
            if t > 0.0 && t < 1.0 {
                result.push(t);
            }
        }
        if dd.y != 0.0 {
            let t = -d0.y / dd.y;
            if t > 0.0 && t < 1.0 {
                result.push(t);
                if result.len() == 2 && result[0] > t {
                    result.swap(0, 1);
                }
            }
        }
        result
    }
}

impl Mul<QuadBez> for Affine {
    type Output = QuadBez;

    #[inline]
    fn mul(self, other: QuadBez) -> QuadBez {
        QuadBez {
            p0: self * other.p0,
            p1: self * other.p1,
            p2: self * other.p2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Affine, Nearest, ParamCurve, ParamCurveArclen, ParamCurveArea, ParamCurveDeriv,
        ParamCurveExtrema, ParamCurveNearest, Point, QuadBez,
    };

    fn assert_near(p0: Point, p1: Point, epsilon: f64) {
        assert!((p1 - p0).hypot() < epsilon, "{p0:?} != {p1:?}");
    }

    #[test]
    fn quadbez_deriv() {
        let q = QuadBez::new((0.0, 0.0), (0.0, 0.5), (1.0, 1.0));
        let deriv = q.deriv();

        let n = 10;
        for i in 0..=n {
            let t = (i as f64) * (n as f64).recip();
            let delta = 1e-6;
            let p = q.eval(t);
            let p1 = q.eval(t + delta);
            let d_approx = (p1 - p) * delta.recip();
            let d = deriv.eval(t).to_vec2();
            assert!((d - d_approx).hypot() < delta * 2.0);
        }
    }

    #[test]
    fn quadbez_arclen() {
        let q = QuadBez::new((0.0, 0.0), (0.0, 0.5), (1.0, 1.0));
        let true_arclen = 0.5 * 5.0f64.sqrt() + 0.25 * (2.0 + 5.0f64.sqrt()).ln();
        for i in 0..12 {
            let accuracy = 0.1f64.powi(i);
            let est = q.arclen(accuracy);
            let error = est - true_arclen;
            assert!(error.abs() < accuracy, "{est} != {true_arclen}");
        }
    }

    #[test]
    fn quadbez_arclen_pathological() {
        let q = QuadBez::new((-1.0, 0.0), (1.03, 0.0), (1.0, 0.0));
        let true_arclen = 2.0008737864167325; // A rough empirical calculation
        let accuracy = 1e-11;
        let est = q.arclen(accuracy);
        assert!(
            (est - true_arclen).abs() < accuracy,
            "{est} != {true_arclen}"
        );
    }

    #[test]
    fn quadbez_subsegment() {
        let q = QuadBez::new((3.1, 4.1), (5.9, 2.6), (5.3, 5.8));
        let t0 = 0.1;
        let t1 = 0.8;
        let qs = q.subsegment(t0..t1);
        let epsilon = 1e-12;
        let n = 10;
        for i in 0..=n {
            let t = (i as f64) * (n as f64).recip();
            let ts = t0 + t * (t1 - t0);
            assert_near(q.eval(ts), qs.eval(t), epsilon);
        }
    }

    #[test]
    fn quadbez_raise() {
        let q = QuadBez::new((3.1, 4.1), (5.9, 2.6), (5.3, 5.8));
        let c = q.raise();
        let qd = q.deriv();
        let cd = c.deriv();
        let epsilon = 1e-12;
        let n = 10;
        for i in 0..=n {
            let t = (i as f64) * (n as f64).recip();
            assert_near(q.eval(t), c.eval(t), epsilon);
            assert_near(qd.eval(t), cd.eval(t), epsilon);
        }
    }

    #[test]
    fn quadbez_signed_area() {
        // y = 1 - x^2
        let q = QuadBez::new((1.0, 0.0), (0.5, 1.0), (0.0, 1.0));
        let epsilon = 1e-12;
        assert!((q.signed_area() - 2.0 / 3.0).abs() < epsilon);
        assert!(((Affine::rotate(0.5) * q).signed_area() - 2.0 / 3.0).abs() < epsilon);
        assert!(((Affine::translate((0.0, 1.0)) * q).signed_area() - 3.5 / 3.0).abs() < epsilon);
        assert!(((Affine::translate((1.0, 0.0)) * q).signed_area() - 3.5 / 3.0).abs() < epsilon);
    }

    fn verify(result: Nearest, expected: f64) {
        assert!(
            (result.t - expected).abs() < 1e-6,
            "got {result:?} expected {expected}"
        );
    }

    #[test]
    fn quadbez_nearest() {
        // y = x^2
        let q = QuadBez::new((-1.0, 1.0), (0.0, -1.0), (1.0, 1.0));
        verify(q.nearest((0.0, 0.0).into(), 1e-3), 0.5);
        verify(q.nearest((0.0, 0.1).into(), 1e-3), 0.5);
        verify(q.nearest((0.0, -0.1).into(), 1e-3), 0.5);
        verify(q.nearest((0.5, 0.25).into(), 1e-3), 0.75);
        verify(q.nearest((1.0, 1.0).into(), 1e-3), 1.0);
        verify(q.nearest((1.1, 1.1).into(), 1e-3), 1.0);
        verify(q.nearest((-1.1, 1.1).into(), 1e-3), 0.0);
        let a = Affine::rotate(0.5);
        verify((a * q).nearest(a * Point::new(0.5, 0.25), 1e-3), 0.75);
    }

    // This test exposes a degenerate case in the solver used internally
    // by the "nearest" calculation - the cubic term is zero.
    #[test]
    fn quadbez_nearest_low_order() {
        let q = QuadBez::new((-1.0, 0.0), (0.0, 0.0), (1.0, 0.0));

        verify(q.nearest((0.0, 0.0).into(), 1e-3), 0.5);
        verify(q.nearest((0.0, 1.0).into(), 1e-3), 0.5);
    }

    #[test]
    fn quadbez_extrema() {
        // y = x^2
        let q = QuadBez::new((-1.0, 1.0), (0.0, -1.0), (1.0, 1.0));
        let extrema = q.extrema();
        assert_eq!(extrema.len(), 1);
        assert!((extrema[0] - 0.5).abs() < 1e-6);

        let q = QuadBez::new((0.0, 0.5), (1.0, 1.0), (0.5, 0.0));
        let extrema = q.extrema();
        assert_eq!(extrema.len(), 2);
        assert!((extrema[0] - 1.0 / 3.0).abs() < 1e-6);
        assert!((extrema[1] - 2.0 / 3.0).abs() < 1e-6);

        // Reverse direction
        let q = QuadBez::new((0.5, 0.0), (1.0, 1.0), (0.0, 0.5));
        let extrema = q.extrema();
        assert_eq!(extrema.len(), 2);
        assert!((extrema[0] - 1.0 / 3.0).abs() < 1e-6);
        assert!((extrema[1] - 2.0 / 3.0).abs() < 1e-6);
    }
}
