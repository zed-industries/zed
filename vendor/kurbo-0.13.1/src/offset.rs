// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Computation of offset curves of cubic Béziers.
//!
//! The main algorithm in this module is a new technique designed for robustness
//! and speed. The details are involved; hopefully there will be a paper.

use arrayvec::ArrayVec;

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

use crate::{
    BezPath, CubicBez, ParamCurve, ParamCurveDeriv, PathSeg, Point, QuadBez, Vec2,
    common::{solve_itp, solve_quadratic},
};

/// State used for computing an offset curve of a single cubic.
struct CubicOffset {
    /// The cubic being offset. This has been regularized.
    c: CubicBez,
    /// The derivative of `c`.
    q: QuadBez,
    /// The offset distance (same as the argument).
    d: f64,
    /// `c0 + c1 t + c2 t^2` is the cross product of second and first
    /// derivatives of the underlying cubic, multiplied by the offset.
    /// This is used for computing cusps on the offset curve.
    ///
    /// Note that given a curve `c(t)`, its signed curvature is
    /// `c''(t) x c'(t) / ||c'(t)||^3`. See also [`Self::cusp_sign`].
    c0: f64,
    c1: f64,
    c2: f64,
    /// The tolerance (same as the argument).
    tolerance: f64,
}

// We never let cusp values have an absolute value smaller than
// this. When a cusp is found, determine its sign and use this value.
const CUSP_EPSILON: f64 = 1e-12;

/// Number of points for least-squares fit and error evaluation.
///
/// This value is a tradeoff between accuracy and performance. The main risk in it
/// being to small is under-sampling the error and thus letting excessive error
/// slip through. That said, the "arc drawing" approach is designed to be robust
/// and not generate approximate results with narrow error peaks, even in near-cusp
/// "J" shape curves.
const N_LSE: usize = 8;

/// The proportion of transverse error that is blended in the least-squares logic.
const BLEND: f64 = 1e-3;

/// Maximum recursion depth.
///
/// Recursion is bounded to this depth, so the total number of subdivisions will
/// not exceed two to this power.
///
/// This is primarily a "belt and suspenders" robustness guard. In normal operation,
/// the recursion bound should never be reached, as accuracy improves very quickly
/// on subdivision. For unreasonably large coordinate values or small tolerances, it
/// is possible, and in those cases the result will be out of tolerance.
///
/// Perhaps should be configurable.
const MAX_DEPTH: usize = 8;

/// State local to a subdivision
struct OffsetRec {
    t0: f64,
    t1: f64,
    // unit tangent at t0
    utan0: Vec2,
    // unit tangent at t1
    utan1: Vec2,
    cusp0: f64,
    cusp1: f64,
    /// Recursion depth
    depth: usize,
}

/// Result of error evaluation
struct ErrEval {
    /// Maximum detected error
    err_squared: f64,
    /// Unit normals sampled uniformly across approximation
    unorms: [Vec2; N_LSE],
    /// Difference between point on source curve and normal from approximation.
    err_vecs: [Vec2; N_LSE],
}

/// Result of subdivision
struct SubdivisionPoint {
    /// Source curve t value at subdivision point
    t: f64,
    /// Unit tangent at subdivision point
    utan: Vec2,
}

/// Compute an approximate offset curve.
///
/// The parallel curve of `c` offset by `d` is written to the `result` path.
///
/// There is a fair amount of attention to robustness, but this method is not suitable
/// for degenerate cubics with entirely co-linear control points. Those cases should be
/// handled before calling this function, by replacing them with linear segments.
pub fn offset_cubic(c: CubicBez, d: f64, tolerance: f64, result: &mut BezPath) {
    result.truncate(0);
    // A tuning parameter for regularization. A value too large may distort the curve,
    // while a value too small may fail to generate smooth curves. This is a somewhat
    // arbitrary value, and should be revisited.
    const DIM_TUNE: f64 = 0.25;
    // We use regularization to perturb the curve to avoid *interior* zero-derivative
    // cusps. There is robustness logic in place to handle zero derivatives at the
    // endpoints.
    //
    // As a performance note, it might be a good idea to move regularization and
    // tangent determination to the caller, as those computations are the same for both
    // signs of `d`.
    let c_regularized = c.regularize_cusp(tolerance * DIM_TUNE);
    let co = CubicOffset::new(c_regularized, d, tolerance);
    let (tan0, tan1) = PathSeg::Cubic(c).tangents();
    let utan0 = tan0.normalize();
    let utan1 = tan1.normalize();
    let cusp0 = co.endpoint_cusp(co.q.p0, co.c0);
    let cusp1 = co.endpoint_cusp(co.q.p2, co.c0 + co.c1 + co.c2);
    result.move_to(c.p0 + d * utan0.turn_90());
    let rec = OffsetRec::new(0., 1., utan0, utan1, cusp0, cusp1, 0);
    co.offset_rec(&rec, result);
}

impl CubicOffset {
    /// Create a new curve from Bézier segment and offset.
    fn new(c: CubicBez, d: f64, tolerance: f64) -> Self {
        let q = c.deriv();
        let d2 = 2.0 * d;
        let p1xp0 = q.p1.to_vec2().cross(q.p0.to_vec2());
        let p2xp0 = q.p2.to_vec2().cross(q.p0.to_vec2());
        let p2xp1 = q.p2.to_vec2().cross(q.p1.to_vec2());
        CubicOffset {
            c,
            q,
            d,
            c0: d2 * p1xp0,
            c1: d2 * (p2xp0 - 2.0 * p1xp0),
            c2: d2 * (p2xp1 - p2xp0 + p1xp0),
            tolerance,
        }
    }

    /// Compute curvature of the source curve times offset plus 1.
    ///
    /// This quantity is called "cusp" because cusps appear in the offset curve
    /// where this value crosses zero. This is based on the geometric property
    /// that the offset curve has a cusp when the radius of curvature of the
    /// source curve is equal to the offset curve's distance.
    ///
    /// Note: there is a potential division by zero when the derivative vanishes.
    /// We avoid doing so for interior points by regularizing the cubic beforehand.
    /// We avoid doing so for endpoints by calling `endpoint_cusp` instead.
    fn cusp_sign(&self, t: f64) -> f64 {
        let ds2 = self.q.eval(t).to_vec2().hypot2();
        ((self.c2 * t + self.c1) * t + self.c0) / (ds2 * ds2.sqrt()) + 1.0
    }

    /// Compute cusp value of endpoint.
    ///
    /// This is a special case of [`Self::cusp_sign`]. For the start point, `tan` should be
    /// the start point tangent and `y` should be `c0`. For the end point, `tan` should be
    /// the end point tangent and `y` should be `c0 + c1 + c2`.
    ///
    /// This is just evaluating the polynomial at t=0 and t=1.
    ///
    /// See [`Self::cusp_sign`] for a description of what "cusp value" means.
    fn endpoint_cusp(&self, tan: Point, y: f64) -> f64 {
        // Robustness to avoid divide-by-zero when derivatives vanish
        const TAN_DIST_EPSILON: f64 = 1e-12;
        let tan_dist = tan.to_vec2().hypot().max(TAN_DIST_EPSILON);
        let rsqrt = 1.0 / tan_dist;
        y * (rsqrt * rsqrt * rsqrt) + 1.0
    }

    /// Primary entry point for recursive subdivision.
    ///
    /// At a high level, this method determines whether subdivision is necessary. If
    /// so, it determines a subdivision point and then recursively calls itself on
    /// both subdivisions. If not, it computes a single cubic Bézier to approximate
    /// the offset curve, and appends it to `result`.
    fn offset_rec(&self, rec: &OffsetRec, result: &mut BezPath) {
        // First, determine whether the offset curve contains a cusp. If the sign
        // of the cusp value (curvature times offset plus 1) is different at the
        // subdivision endpoints, then there is definitely a cusp inside. Find it and
        // subdivide there.
        //
        // Note that there's a possibility the curve has two (or, potentially, any
        // even number). We don't rigorously check for this case; if the measured
        // error comes in under the tolerance, we simply accept it. Otherwise, in
        // the common case we expect to detect a sign crossing from the new
        // subdivision point.
        if rec.cusp0 * rec.cusp1 < 0.0 {
            let a = rec.t0;
            let b = rec.t1;
            let s = rec.cusp1.signum();
            let f = |t| s * self.cusp_sign(t);
            let k1 = 0.2 / (b - a);
            const ITP_EPS: f64 = 1e-12;
            let t = solve_itp(f, a, b, ITP_EPS, 1, k1, s * rec.cusp0, s * rec.cusp1);
            // TODO(robustness): If we're unlucky, there will be 3 cusps between t0
            // and t1, and the solver will land on the middle one. In that case, the
            // derivative on cusp will be the opposite sign as expected.
            //
            // If we're even more unlucky, there is a second-order cusp, both zero
            // cusp value and zero derivative.
            let utan_t = self.q.eval(t).to_vec2().normalize();
            let cusp_t_minus = CUSP_EPSILON.copysign(rec.cusp0);
            let cusp_t_plus = CUSP_EPSILON.copysign(rec.cusp1);
            self.subdivide(rec, result, t, utan_t, cusp_t_minus, cusp_t_plus);
            return;
        }
        // We determine the first approximation to the offset curve.
        let (mut a, mut b) = self.draw_arc(rec);
        let dt = (rec.t1 - rec.t0) * (1.0 / (N_LSE + 1) as f64);
        // These represent t values on the source curve.
        let mut ts = core::array::from_fn(|i| rec.t0 + (i + 1) as f64 * dt);
        let mut c_approx = self.apply(rec, a, b);
        let err_init = self.eval_err(rec, c_approx, &mut ts);
        let mut err = err_init;
        // Number of least-squares refinement steps. More gives a smaller
        // error, but takes more time.
        const N_REFINE: usize = 2;
        for _ in 0..N_REFINE {
            if err.err_squared <= self.tolerance * self.tolerance {
                break;
            }
            let (a2, b2) = self.refine_least_squares(rec, a, b, &err);
            let c_approx2 = self.apply(rec, a2, b2);
            let err2 = self.eval_err(rec, c_approx2, &mut ts);
            if err2.err_squared >= err.err_squared {
                break;
            }
            err = err2;
            (a, b) = (a2, b2);
            c_approx = c_approx2;
        }
        if rec.depth < MAX_DEPTH && err.err_squared > self.tolerance * self.tolerance {
            let SubdivisionPoint { t, utan } = self.find_subdivision_point(rec);
            // TODO(robustness): if cusp is extremely near zero, then assign epsilon
            // with alternate signs based on derivative of cusp.
            let cusp = self.cusp_sign(t);
            self.subdivide(rec, result, t, utan, cusp, cusp);
        } else {
            result.curve_to(c_approx.p1, c_approx.p2, c_approx.p3);
        }
    }

    /// Recursively subdivide.
    ///
    /// In the case of subdividing at a cusp, the cusp value at the subdivision point
    /// is mathematically zero, but in those cases we treat it as a signed infinitesimal
    /// value representing the values at t minus epsilon and t plus epsilon.
    ///
    /// Note that unit tangents are passed down explicitly. In the general case, they
    /// are equal to the derivative (evaluated at that t value) normalized to unit
    /// length, but in cases where the derivative is near-zero, they are computed more
    /// robustly.
    fn subdivide(
        &self,
        rec: &OffsetRec,
        result: &mut BezPath,
        t: f64,
        utan_t: Vec2,
        cusp_t_minus: f64,
        cusp_t_plus: f64,
    ) {
        let rec0 = OffsetRec::new(
            rec.t0,
            t,
            rec.utan0,
            utan_t,
            rec.cusp0,
            cusp_t_minus,
            rec.depth + 1,
        );
        self.offset_rec(&rec0, result);
        let rec1 = OffsetRec::new(
            t,
            rec.t1,
            utan_t,
            rec.utan1,
            cusp_t_plus,
            rec.cusp1,
            rec.depth + 1,
        );
        self.offset_rec(&rec1, result);
    }

    /// Convert from (a, b) parameter space to the approximate cubic Bézier.
    ///
    /// The offset approximation can be considered `B(t) + d * D(t)`, where `D(t)`
    /// is roughly a unit vector in the direction of the unit normal of the source
    /// curve. (The word "roughly" is appropriate because transverse error may
    /// cancel out normal error, resulting in a lower error than either alone).
    /// The endpoints of `D(t)` must be the unit normals of the source curve, and
    /// the endpoint tangents of `D(t)` must tangent to the endpoint tangents of
    /// the source curve, to ensure G1 continuity.
    ///
    /// The (a, b) parameters refer to the magnitude of the vector from the endpoint
    /// to the corresponding control point in `D(t)`, the direction being determined
    /// by the unit tangent.
    ///
    /// When the candidate solution would lead to negative distance from the
    /// endpoint to the control point, that distance is clamped to zero. Otherwise
    /// such solutions should be considered invalid, and have the unpleasant
    /// property of sometimes passing error tolerance checks.
    fn apply(&self, rec: &OffsetRec, a: f64, b: f64) -> CubicBez {
        // wondering if p0 and p3 should be in rec
        // Scale factor from derivatives to displacements
        let s = (1. / 3.) * (rec.t1 - rec.t0);
        let p0 = self.c.eval(rec.t0) + self.d * rec.utan0.turn_90();
        let l0 = s * self.q.eval(rec.t0).to_vec2().length() + a * self.d;
        let mut p1 = p0;
        if l0 * rec.cusp0 > 0.0 {
            p1 += l0 * rec.utan0;
        }
        let p3 = self.c.eval(rec.t1) + self.d * rec.utan1.turn_90();
        let mut p2 = p3;
        let l1 = s * self.q.eval(rec.t1).to_vec2().length() - b * self.d;
        if l1 * rec.cusp1 > 0.0 {
            p2 -= l1 * rec.utan1;
        }
        CubicBez::new(p0, p1, p2, p3)
    }

    /// Compute arc approximation.
    ///
    /// This is called "arc drawing" because if we just look at the delta
    /// vector, it describes an arc from the initial unit normal to the final unit
    /// normal, with "as smooth as possible" parametrization. This approximation
    /// is not necessarily great, but is very robust, and in particular, accuracy
    /// does not degrade for J shaped near-cusp source curves or when the offset
    /// distance is large (with respect to the source curve arc length).
    ///
    /// It is a pretty good approximation overall and has very clean O(n^4) scaling.
    /// Its worst performance is on curves with a large cubic component, where it
    /// undershoots. The theory is that the least squares refinement improves those
    /// cases.
    fn draw_arc(&self, rec: &OffsetRec) -> (f64, f64) {
        // possible optimization: this can probably be done with vectors
        // rather than arctangent
        let th = rec.utan1.cross(rec.utan0).atan2(rec.utan1.dot(rec.utan0));
        let a = (2. / 3.) / (1.0 + (0.5 * th).cos()) * 2.0 * (0.5 * th).sin();
        let b = -a;
        (a, b)
    }

    /// Evaluate error and also refine t values.
    ///
    /// Returns evaluation of error including error vectors and (squared)
    /// maximum error.
    ///
    /// The vector of t values represents points on the source curve; the logic
    /// here is a Newton step to bring those points closer to the normal ray of
    /// the approximation.
    fn eval_err(&self, rec: &OffsetRec, c_approx: CubicBez, ts: &mut [f64; N_LSE]) -> ErrEval {
        let qa = c_approx.deriv();
        let mut err_squared = 0.0;
        let mut unorms = [Vec2::ZERO; N_LSE];
        let mut err_vecs = [Vec2::ZERO; N_LSE];
        for i in 0..N_LSE {
            let ta = (i + 1) as f64 * (1.0 / (N_LSE + 1) as f64);
            let mut t = ts[i];
            let p = self.c.eval(t);
            // Newton step to refine t value
            let pa = c_approx.eval(ta);
            let tana = qa.eval(ta).to_vec2();
            t += tana.dot(pa - p) / tana.dot(self.q.eval(t).to_vec2());
            t = t.max(rec.t0).min(rec.t1);
            ts[i] = t;
            let cusp = rec.cusp0.signum();
            let unorm = cusp * tana.normalize().turn_90();
            unorms[i] = unorm;
            let p_new = self.c.eval(t) + self.d * unorm;
            let err_vec = pa - p_new;
            err_vecs[i] = err_vec;
            let mut dist_err_squared = err_vec.length_squared();
            if !dist_err_squared.is_finite() {
                // A hack to make sure we reject bad refinements
                dist_err_squared = 1e12;
            }
            // Note: consider also incorporating angle error
            err_squared = dist_err_squared.max(err_squared);
        }
        ErrEval {
            err_squared,
            unorms,
            err_vecs,
        }
    }

    /// Refine an approximation, minimizing least squares error.
    ///
    /// Compute the approximation that minimizes least squares error, based on the given error
    /// evaluation.
    ///
    /// The effectiveness of this refinement varies. Basically, if the curve has a large cubic
    /// component, then the arc drawing will undershoot systematically and this refinement will
    /// reduce error considerably. In other cases, it will eventually converge to a local
    /// minimum, but convergence is slow.
    ///
    /// The `BLEND` parameter controls a tradeoff between robustness and speed of convergence.
    /// In the happy case, convergence is fast and not very sensitive to this parameter. If the
    /// parameter is too small, then in near-parabola cases the determinant will be small and
    /// the result not numerically stable.
    ///
    /// A value of 1.0 for `BLEND` corresponds to essentially the Hoschek method, minimizing
    /// Euclidean distance (which tends to over-anchor on the given t values). A value of 0 would
    /// minimize the dot product of error wrt the normal vector, ignoring the cross product
    /// component.
    ///
    /// A possible future direction would be to tune the parameter adaptively.
    fn refine_least_squares(&self, rec: &OffsetRec, a: f64, b: f64, err: &ErrEval) -> (f64, f64) {
        let mut aa = 0.0;
        let mut ab = 0.0;
        let mut ac = 0.0;
        let mut bb = 0.0;
        let mut bc = 0.0;
        for i in 0..N_LSE {
            let n = err.unorms[i];
            let err_vec = err.err_vecs[i];
            let c_n = err_vec.dot(n);
            let c_t = err_vec.cross(n);
            let a_n = A_WEIGHTS[i] * rec.utan0.dot(n);
            let a_t = A_WEIGHTS[i] * rec.utan0.cross(n);
            let b_n = B_WEIGHTS[i] * rec.utan1.dot(n);
            let b_t = B_WEIGHTS[i] * rec.utan1.cross(n);
            aa += a_n * a_n + BLEND * (a_t * a_t);
            ab += a_n * b_n + BLEND * a_t * b_t;
            ac += a_n * c_n + BLEND * a_t * c_t;
            bb += b_n * b_n + BLEND * (b_t * b_t);
            bc += b_n * c_n + BLEND * b_t * c_t;
        }
        let idet = 1.0 / (self.d * (aa * bb - ab * ab));
        let delta_a = idet * (ac * bb - ab * bc);
        let delta_b = idet * (aa * bc - ac * ab);
        (a - delta_a, b - delta_b)
    }

    /// Decide where to subdivide when error is exceeded.
    ///
    /// For curves not containing an inflection point, subdivide at the tangent
    /// bisecting the endpoint tangents. The logic is that for a near cusp in the
    /// source curve, you want the subdivided sections to be approximately
    /// circular arcs with progressively smaller angles.
    ///
    /// When there is an inflection point (or, more specifically, when the curve
    /// crosses its chord), bisecting the angle can lead to very lopsided arc
    /// lengths, so just subdivide by t in that case.
    fn find_subdivision_point(&self, rec: &OffsetRec) -> SubdivisionPoint {
        let t = 0.5 * (rec.t0 + rec.t1);
        let q_t = self.q.eval(t).to_vec2();
        let x0 = rec.utan0.cross(q_t).abs();
        let x1 = rec.utan1.cross(q_t).abs();
        const SUBDIVIDE_THRESH: f64 = 0.1;
        if x0 > SUBDIVIDE_THRESH * x1 && x1 > SUBDIVIDE_THRESH * x0 {
            let utan = q_t.normalize();
            return SubdivisionPoint { t, utan };
        }

        // Note: do we want to track p0 & p3 in rec, to avoid repeated eval?
        let chord = self.c.eval(rec.t1) - self.c.eval(rec.t0);
        if chord.cross(rec.utan0) * chord.cross(rec.utan1) < 0.0 {
            let tan = rec.utan0 + rec.utan1;
            if let Some(subdivision) =
                self.subdivide_for_tangent(rec.utan0, rec.t0, rec.t1, tan, false)
            {
                return subdivision;
            }
        }
        // Curve definitely has an inflection point
        // Try to subdivide based on integral of absolute curvature.

        // Tangents at recursion endpoints and inflection points.
        let mut tangents: ArrayVec<Vec2, 4> = ArrayVec::new();
        let mut ts: ArrayVec<f64, 4> = ArrayVec::new();
        tangents.push(rec.utan0);
        ts.push(rec.t0);
        for t in self.c.inflections() {
            if t > rec.t0 && t < rec.t1 {
                tangents.push(self.q.eval(t).to_vec2());
                ts.push(t);
            }
        }
        tangents.push(rec.utan1);
        ts.push(rec.t1);
        let mut arc_angles: ArrayVec<f64, 3> = ArrayVec::new();
        let mut sum = 0.0;
        for i in 0..tangents.len() - 1 {
            let tan0 = tangents[i];
            let tan1 = tangents[i + 1];
            let th = tan0.cross(tan1).atan2(tan0.dot(tan1));
            sum += th.abs();
            arc_angles.push(th);
        }
        let mut target = sum * 0.5;
        let mut i = 0;
        while arc_angles[i].abs() < target {
            target -= arc_angles[i].abs();
            i += 1;
        }
        let rotation = Vec2::from_angle(target.copysign(arc_angles[i]));
        let base = tangents[i];
        let tan = base.rotate_scale(rotation);
        let utan0 = if i == 0 { rec.utan0 } else { base.normalize() };
        self.subdivide_for_tangent(utan0, ts[i], ts[i + 1], tan, true)
            .unwrap()
    }

    /// Find a subdivision point, given a tangent vector.
    ///
    /// When subdividing by bisecting the angle (or, more generally, subdividing by
    /// the L1 norm of curvature when there are inflection points), we find the
    /// subdivision point by solving for the tangent matching, specifically the
    /// cross-product of the tangent and the curve's derivative being zero. For
    /// internal cusps, subdividing near the cusp is a good thing, but there is
    /// still a robustness concern for vanishing derivative at the endpoints.
    fn subdivide_for_tangent(
        &self,
        utan0: Vec2,
        t0: f64,
        t1: f64,
        tan: Vec2,
        force: bool,
    ) -> Option<SubdivisionPoint> {
        let mut t = 0.0;
        let mut n_soln = 0;
        // set up quadratic equation for matching tangents
        let z0 = tan.cross(self.q.p0.to_vec2());
        let z1 = tan.cross(self.q.p1.to_vec2());
        let z2 = tan.cross(self.q.p2.to_vec2());
        let c0 = z0;
        let c1 = 2.0 * (z1 - z0);
        let c2 = (z2 - z1) - (z1 - z0);
        for root in solve_quadratic(c0, c1, c2) {
            if root >= t0 && root <= t1 {
                t = root;
                n_soln += 1;
            }
        }
        if n_soln != 1 {
            if !force {
                return None;
            }
            // Numerical failure, try to subdivide at cusp; we pick the
            // smaller derivative.
            if self.q.eval(t0).to_vec2().length_squared()
                > self.q.eval(t1).to_vec2().length_squared()
            {
                t = t1;
            } else {
                t = t0;
            }
        }
        let q = self.q.eval(t).to_vec2();
        const UTAN_EPSILON: f64 = 1e-12;
        let utan = if n_soln == 1 && q.length_squared() >= UTAN_EPSILON {
            q.normalize()
        } else if tan.length_squared() >= UTAN_EPSILON {
            // Curve has a zero-derivative cusp but angles well defined
            tan.normalize()
        } else {
            // 180 degree U-turn, arbitrarily pick a direction.
            // If we get to this point, there will probably be a failure.
            utan0.turn_90()
        };
        Some(SubdivisionPoint { t, utan })
    }
}

impl OffsetRec {
    #[allow(clippy::too_many_arguments)]
    fn new(
        t0: f64,
        t1: f64,
        utan0: Vec2,
        utan1: Vec2,
        cusp0: f64,
        cusp1: f64,
        depth: usize,
    ) -> Self {
        OffsetRec {
            t0,
            t1,
            utan0,
            utan1,
            cusp0,
            cusp1,
            depth,
        }
    }
}

/// Compute Bézier weights for evenly subdivided t values.
const fn mk_a_weights(rev: bool) -> [f64; N_LSE] {
    let mut result = [0.0; N_LSE];
    let mut i = 0;
    while i < N_LSE {
        let t = (i + 1) as f64 / (N_LSE + 1) as f64;
        let mt = 1. - t;
        let ix = if rev { N_LSE - 1 - i } else { i };
        result[ix] = 3.0 * mt * t * mt;
        i += 1;
    }
    result
}

const A_WEIGHTS: [f64; N_LSE] = mk_a_weights(false);
const B_WEIGHTS: [f64; N_LSE] = mk_a_weights(true);

#[cfg(test)]
mod tests {
    use super::offset_cubic;
    use crate::{BezPath, CubicBez, PathEl};

    // This test tries combinations of parameters that have caused problems in the past.
    #[test]
    fn pathological_curves() {
        let curve = CubicBez {
            p0: (-1236.3746269978635, 152.17981429574826).into(),
            p1: (-1175.18662093517, 108.04721798590596).into(),
            p2: (-1152.142883879584, 105.76260301083356).into(),
            p3: (-1151.842639804639, 105.73040758939104).into(),
        };
        let offset = 3603.7267536453924;
        let accuracy = 0.1;

        let mut result = BezPath::new();
        offset_cubic(curve, offset, accuracy, &mut result);
        assert!(matches!(result.iter().next(), Some(PathEl::MoveTo(_))));

        let mut result = BezPath::new();
        offset_cubic(curve, offset, accuracy, &mut result);
        assert!(matches!(result.iter().next(), Some(PathEl::MoveTo(_))));
    }

    /// Cubic offset that used to trigger infinite recursion.
    #[test]
    fn infinite_recursion() {
        const TOLERANCE: f64 = 0.1;
        const OFFSET: f64 = -0.5;
        let c = CubicBez::new(
            (1096.2962962962963, 593.90243902439033),
            (1043.6213991769548, 593.90243902439033),
            (1030.4526748971193, 593.90243902439033),
            (1056.7901234567901, 593.90243902439033),
        );

        let mut result = BezPath::new();
        offset_cubic(c, OFFSET, TOLERANCE, &mut result);
    }

    #[test]
    fn test_cubic_offset_simple_line() {
        let cubic = CubicBez::new((0., 0.), (10., 0.), (20., 0.), (30., 0.));
        let mut result = BezPath::new();
        offset_cubic(cubic, 5., 1e-6, &mut result);
    }
}
