// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Simplification of a Bézier path.
//!
//! This module is currently experimental.
//!
//! The methods in this module create a `SimplifyBezPath` object, which can then
//! be fed to [`fit_to_bezpath`] or [`fit_to_bezpath_opt`] depending on the degree
//! of optimization desired.
//!
//! The implementation uses a number of techniques to achieve high performance and
//! accuracy. The parameter (generally written `t`) evenly divides the curve segments
//! in the original, so sampling can be done in constant time. The derivatives are
//! computed analytically, as that is straightforward with Béziers.
//!
//! The areas and moments are computed analytically (using Green's theorem), and
//! the prefix sum is stored. Thus, it is possible to analytically compute the area
//! and moment of any subdivision of the curve, also in constant time, by taking
//! the difference of two stored prefix sum values, then fixing up the subsegments.
//!
//! A current limitation (hoped to be addressed in the future) is that non-regular
//! cubic segments may have tangents computed incorrectly. This can easily happen,
//! for example when setting a control point equal to an endpoint.
//!
//! In addition, this method does not report corners (adjoining segments where the
//! tangents are not continuous). It is not clear whether it's best to handle such
//! cases here, or in client code.
//!
//! [`fit_to_bezpath`]: crate::fit_to_bezpath
//! [`fit_to_bezpath_opt`]: crate::fit_to_bezpath_opt

use alloc::vec::Vec;

use core::ops::Range;

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

use crate::{
    fit_to_bezpath, fit_to_bezpath_opt, BezPath, CubicBez, CurveFitSample, Line, ParamCurve,
    ParamCurveDeriv, ParamCurveFit, PathEl, PathSeg, Point, QuadBez, Vec2,
};

/// A Bézier path which has been prepared for simplification.
///
/// See the [module-level documentation] for a bit more discussion of the approach,
/// and how this struct is to be used.
///
/// [module-level documentation]: crate::simplify
pub struct SimplifyBezPath(Vec<SimplifyCubic>);

struct SimplifyCubic {
    c: CubicBez,
    // The inclusive prefix sum of the moment integrals
    moments: (f64, f64, f64),
}

/// Options for simplifying paths.
pub struct SimplifyOptions {
    /// The tangent of the minimum angle below which the path is considered smooth.
    angle_thresh: f64,
    opt_level: SimplifyOptLevel,
}

/// Optimization level for simplification.
pub enum SimplifyOptLevel {
    /// Subdivide; faster but not as optimized results.
    Subdivide,
    /// Optimize subdivision points.
    Optimize,
}

impl Default for SimplifyOptions {
    fn default() -> Self {
        let opt_level = SimplifyOptLevel::Subdivide;
        SimplifyOptions {
            angle_thresh: 1e-3,
            opt_level,
        }
    }
}

#[doc(hidden)]
/// Compute moment integrals.
///
/// This is exposed for testing purposes but is an internal detail. We can
/// add to the public, documented interface if there is a use case.
pub fn moment_integrals(c: CubicBez) -> (f64, f64, f64) {
    let (x0, y0) = (c.p0.x, c.p0.y);
    let (x1, y1) = (c.p1.x - x0, c.p1.y - y0);
    let (x2, y2) = (c.p2.x - x0, c.p2.y - y0);
    let (x3, y3) = (c.p3.x - x0, c.p3.y - y0);

    let r0 = 3. * x1;
    let r1 = 3. * y1;
    let r2 = x2 * y3;
    let r3 = x3 * y2;
    let r4 = x3 * y3;
    let r5 = 27. * y1;
    let r6 = x1 * x2;
    let r7 = 27. * y2;
    let r8 = 45. * r2;
    let r9 = 18. * x3;
    let r10 = x1 * y1;
    let r11 = 30. * x1;
    let r12 = 45. * x3;
    let r13 = x2 * y1;
    let r14 = 45. * r3;
    let r15 = x1.powi(2);
    let r16 = 18. * y3;
    let r17 = x2.powi(2);
    let r18 = 45. * y3;
    let r19 = x3.powi(2);
    let r20 = 30. * y1;
    let r21 = y2.powi(2);
    let r22 = y3.powi(2);
    let r23 = y1.powi(2);
    let a = -r0 * y2 - r0 * y3 + r1 * x2 + r1 * x3 - 6. * r2 + 6. * r3 + 10. * r4;

    // Scale and add chord
    let lift = x3 * y0;
    let area = a * 0.05 + lift;
    let x = r10 * r9 - r11 * r4 + r12 * r13 + r14 * x2 - r15 * r16 - r15 * r7 - r17 * r18
        + r17 * r5
        + r19 * r20
        + 105. * r19 * y2
        + 280. * r19 * y3
        - 105. * r2 * x3
        + r5 * r6
        - r6 * r7
        - r8 * x1;
    let y = -r10 * r16 - r10 * r7 - r11 * r22 + r12 * r21 + r13 * r7 + r14 * y1 - r18 * x1 * y2
        + r20 * r4
        - 27. * r21 * x1
        - 105. * r22 * x2
        + 140. * r22 * x3
        + r23 * r9
        + 27. * r23 * x2
        + 105. * r3 * y3
        - r8 * y2;

    let mx = x * (1. / 840.) + x0 * area + 0.5 * x3 * lift;
    let my = y * (1. / 420.) + y0 * a * 0.1 + y0 * lift;

    (area, mx, my)
}

impl SimplifyBezPath {
    /// Set up a new Bézier path for simplification.
    ///
    /// Currently this is not dealing with discontinuities at all, but it
    /// could be extended to do so.
    pub fn new(path: impl IntoIterator<Item = PathEl>) -> Self {
        let (mut a, mut x, mut y) = (0.0, 0.0, 0.0);
        let els = crate::segments(path)
            .map(|seg| {
                let c = seg.to_cubic();
                let (ai, xi, yi) = moment_integrals(c);
                a += ai;
                x += xi;
                y += yi;
                SimplifyCubic {
                    c,
                    moments: (a, x, y),
                }
            })
            .collect();
        SimplifyBezPath(els)
    }

    /// Resolve a `t` value to a cubic.
    ///
    /// Also return the resulting `t` value for the selected cubic.
    fn scale(&self, t: f64) -> (usize, f64) {
        let t_scale = t * self.0.len() as f64;
        let t_floor = t_scale.floor();
        (t_floor as usize, t_scale - t_floor)
    }

    fn moment_integrals(&self, i: usize, range: Range<f64>) -> (f64, f64, f64) {
        if range.end == range.start {
            (0.0, 0.0, 0.0)
        } else {
            moment_integrals(self.0[i].c.subsegment(range))
        }
    }
}

impl ParamCurveFit for SimplifyBezPath {
    fn sample_pt_deriv(&self, t: f64) -> (Point, Vec2) {
        let (mut i, mut t0) = self.scale(t);
        let n = self.0.len();
        if i == n {
            i -= 1;
            t0 = 1.0;
        }
        let c = self.0[i].c;
        (c.eval(t0), c.deriv().eval(t0).to_vec2() * n as f64)
    }

    fn sample_pt_tangent(&self, t: f64, _: f64) -> CurveFitSample {
        let (mut i, mut t0) = self.scale(t);
        if i == self.0.len() {
            i -= 1;
            t0 = 1.0;
        }
        let c = self.0[i].c;
        let p = c.eval(t0);
        let tangent = c.deriv().eval(t0).to_vec2();
        CurveFitSample { p, tangent }
    }

    // We could use the default implementation, but provide our own, mostly
    // because it is possible to efficiently provide an analytically accurate
    // answer.
    fn moment_integrals(&self, range: Range<f64>) -> (f64, f64, f64) {
        let (i0, t0) = self.scale(range.start);
        let (i1, t1) = self.scale(range.end);
        if i0 == i1 {
            self.moment_integrals(i0, t0..t1)
        } else {
            let (a0, x0, y0) = self.moment_integrals(i0, t0..1.0);
            let (a1, x1, y1) = self.moment_integrals(i1, 0.0..t1);
            let (mut a, mut x, mut y) = (a0 + a1, x0 + x1, y0 + y1);
            if i1 > i0 + 1 {
                let (a2, x2, y2) = self.0[i0].moments;
                let (a3, x3, y3) = self.0[i1 - 1].moments;
                a += a3 - a2;
                x += x3 - x2;
                y += y3 - y2;
            }
            (a, x, y)
        }
    }

    fn break_cusp(&self, _: Range<f64>) -> Option<f64> {
        None
    }
}

#[derive(Default)]
struct SimplifyState {
    queue: BezPath,
    result: BezPath,
    needs_moveto: bool,
}

impl SimplifyState {
    fn add_seg(&mut self, seg: PathSeg) {
        if self.queue.is_empty() {
            self.queue.move_to(seg.start());
        }
        match seg {
            PathSeg::Quad(q) => self.queue.quad_to(q.p1, q.p2),
            PathSeg::Cubic(c) => self.queue.curve_to(c.p1, c.p2, c.p3),
            _ => unreachable!(),
        }
    }

    fn flush(&mut self, accuracy: f64, options: &SimplifyOptions) {
        if self.queue.is_empty() {
            return;
        }
        // TODO: if queue is one segment, just output that
        let s = SimplifyBezPath::new(&self.queue);
        let b = match options.opt_level {
            SimplifyOptLevel::Subdivide => fit_to_bezpath(&s, accuracy),
            SimplifyOptLevel::Optimize => fit_to_bezpath_opt(&s, accuracy),
        };
        self.result
            .extend(b.iter().skip(!self.needs_moveto as usize));
        self.needs_moveto = false;
        self.queue.truncate(0);
    }
}

/// Simplify a Bézier path.
///
/// This function simplifies an arbitrary Bézier path; it is designed to handle
/// multiple subpaths and also corners.
pub fn simplify_bezpath(
    path: impl IntoIterator<Item = PathEl>,
    accuracy: f64,
    options: &SimplifyOptions,
) -> BezPath {
    let mut last_pt = None;
    let mut last_seg: Option<PathSeg> = None;
    let mut state = SimplifyState::default();
    for el in path {
        let mut this_seg = None;
        match el {
            PathEl::MoveTo(p) => {
                state.flush(accuracy, options);
                state.needs_moveto = true;
                last_pt = Some(p);
            }
            PathEl::LineTo(p) => {
                let last = last_pt.unwrap();
                if last == p {
                    continue;
                }
                state.flush(accuracy, options);
                this_seg = Some(PathSeg::Line(Line::new(last, p)));
            }
            PathEl::QuadTo(p1, p2) => {
                let last = last_pt.unwrap();
                if last == p1 && last == p2 {
                    continue;
                }
                this_seg = Some(PathSeg::Quad(QuadBez::new(last, p1, p2)));
            }
            PathEl::CurveTo(p1, p2, p3) => {
                let last = last_pt.unwrap();
                if last == p1 && last == p2 && last == p3 {
                    continue;
                }
                this_seg = Some(PathSeg::Cubic(CubicBez::new(last, p1, p2, p3)));
            }
            PathEl::ClosePath => {
                state.flush(accuracy, options);
                state.result.close_path();
                state.needs_moveto = true;
                last_seg = None;
                continue;
            }
        }
        if let Some(seg) = this_seg {
            if let Some(last) = last_seg {
                let last_tan = last.tangents().1;
                let this_tan = seg.tangents().0;
                if last_tan.cross(this_tan).abs()
                    > last_tan.dot(this_tan).abs() * options.angle_thresh
                {
                    state.flush(accuracy, options);
                }
            }
            last_pt = Some(seg.end());
            state.add_seg(seg);
        }
        last_seg = this_seg;
    }
    state.flush(accuracy, options);
    state.result
}

impl SimplifyOptions {
    /// Set optimization level.
    pub fn opt_level(mut self, level: SimplifyOptLevel) -> Self {
        self.opt_level = level;
        self
    }

    /// Set angle threshold.
    ///
    /// The tangent of the angle below which joins are considered smooth and
    /// not corners. The default is approximately 1 milliradian.
    pub fn angle_thresh(mut self, thresh: f64) -> Self {
        self.angle_thresh = thresh;
        self
    }
}
