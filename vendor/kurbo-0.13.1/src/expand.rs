// Copyright 2026 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{
    Affine, Arc, BezPath, CubicBez, Join, ParamCurve, ParamCurveDeriv, PathEl, PathSeg, Point,
    QuadBez, Shape, Vec2, bezpath::close_subpaths, segments,
};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// A diagonal matrix, suitable for representing anisotropic scaling.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Diagonal2 {
    /// The horizontal expansion factor.
    pub xx: f64,
    /// The vertical expansion factor.
    pub yy: f64,
}

struct ExpandCtx {
    expand: Diagonal2,
    join: Join,
    miter_limit: f64,
    tolerance: f64,
    result: BezPath,
    first_n: Option<Vec2>,
    first_tan: Vec2,
    last_pt: Point,
    last_n: Option<Vec2>,
    last_tan: Vec2,
}

struct CubicCtx {
    q: QuadBez,
    utan0: Vec2,
    utan1: Vec2,
}

struct TwoPointSample {
    a_n: f64,
    b_n: f64,
    c_n: f64,
}

impl Diagonal2 {
    /// Create a diagonal matrix.
    pub const fn new(xx: f64, yy: f64) -> Self {
        Self { xx, yy }
    }

    /// Matrix inverse.
    ///
    /// Will of course produce infinities if a component is zero.
    pub const fn inv(self) -> Self {
        Diagonal2::new(1.0 / self.xx, 1.0 / self.yy)
    }

    /// Absolute value of transform components.
    pub const fn abs(self) -> Self {
        Self::new(self.xx.abs(), self.yy.abs())
    }

    /// Scale a normal vector.
    ///
    /// This is mathematically equivalent to `self * (self * n).normalize()`
    /// for positive transforms, but handles zeros and gets the sign correct
    /// when negative.
    ///
    /// Note that `n` need not be unit length.
    pub fn scale_normal(self, n: Vec2) -> Vec2 {
        let z = self * n;
        let z_hypot2 = z.hypot2();
        if z_hypot2 == 0.0 {
            Vec2::ZERO
        } else {
            let inv_scale = 1.0 / z_hypot2.sqrt();
            self.abs() * z * inv_scale
        }
    }
}

impl core::ops::Neg for Diagonal2 {
    type Output = Self;

    fn neg(self) -> Self {
        Diagonal2::new(-self.xx, -self.yy)
    }
}

impl core::ops::Mul<Vec2> for Diagonal2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.xx * rhs.x, self.yy * rhs.y)
    }
}

impl core::ops::Mul<Point> for Diagonal2 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::new(self.xx * rhs.x, self.yy * rhs.y)
    }
}

// Note: a bunch more ops on `Diagonal2` could be implemented, but for now we'll stick to what we need.

/// Expand a path.
///
/// Expands a filled path by the expansion, which allows separate x and y factors. The
/// path (and the result) is interpreted according to the nonzero winding rule. Both
/// factors should be positive. A negative expansion will shrink the path but is also
/// likely to leave intersection artifacts at corners.
///
/// The direction of the expansion is based on the signed area of the overall path.
/// This should give expected results most of the time, but there are exceptions. For a
/// figure-eight path, one lobe will be expanded and the other shrunk. Similarly if
/// there are two disjoint subpaths with opposite winding.
///
/// The tolerance is mostly for joins and robustness; it is not used to guide
/// subdivision. Rather, each Bézier segment in the input generally results in one
/// cubic Bézier in the output. Thus, it is not expected to work well when the
/// expansion factor is large compared with the radius of curvature on the input.
pub fn expand_path(
    path: impl Shape,
    mut expand: Diagonal2,
    join: Join,
    miter_limit: f64,
    tolerance: f64,
) -> BezPath {
    if segments(close_subpaths(path.path_elements(tolerance))).area() >= 0.0 {
        expand = -expand;
    }
    expand_path_signed(path, expand, join, miter_limit, tolerance)
}

/// Expand a path when the sign is known.
///
/// Applies the expansion based on the path orientation, so that expansion happens
/// with positive `expand` values on subpaths with negative area, or vice versa. This
/// is backwards from the intuitive sign convention, but results from the choice of
/// convention for the offset primitives.
pub fn expand_path_signed(
    path: impl Shape,
    expand: Diagonal2,
    join: Join,
    miter_limit: f64,
    tolerance: f64,
) -> BezPath {
    let result = BezPath::new();
    let mut ctx = ExpandCtx {
        expand,
        join,
        miter_limit,
        tolerance,
        result,
        first_n: None,
        first_tan: Vec2::default(),
        last_pt: Point::default(),
        last_n: None,
        last_tan: Vec2::default(),
    };
    let mut first_pt = Point::default();
    for el in path.path_elements(tolerance) {
        match el {
            PathEl::MoveTo(point) => {
                ctx.do_close_path(first_pt);
                first_pt = point;
                ctx.last_pt = point;
            }
            PathEl::LineTo(p1) => ctx.do_line(p1),
            PathEl::QuadTo(p1, p2) => ctx.do_quad(p1, p2),
            PathEl::CurveTo(p1, p2, p3) => ctx.do_cubic(p1, p2, p3),
            PathEl::ClosePath => ctx.do_close_path(first_pt),
        }
    }
    // Treat all subpaths as closed; close if left open in input.
    ctx.do_close_path(first_pt);
    ctx.result
}

impl ExpandCtx {
    /// Helper function to determine if a distance is within tolerance.
    fn in_tolerance(&self, v: Vec2) -> bool {
        v.hypot2() < self.tolerance * self.tolerance
    }

    /// Process a line segment. Includes initial join.
    fn do_line(&mut self, p1: Point) {
        if p1 == self.last_pt {
            return;
        }
        let tan = p1 - self.last_pt;
        let n = self.expand.scale_normal(tan.turn_90());
        self.do_join(n, tan, true);
        let out_p1 = p1 + n;
        self.result.line_to(out_p1);
        self.last_n = Some(n);
        self.last_tan = tan;
        self.last_pt = p1;
    }

    /// Process a quadratic Bézier segment. Includes initial join.
    fn do_quad(&mut self, p1: Point, p2: Point) {
        let q0 = p1 - self.last_pt;
        let q1 = p2 - p1;
        if self.in_tolerance(q0) || self.in_tolerance(q1) {
            self.do_line(p2);
            return;
        }
        let einv = self.expand.inv();
        let utan0 = (einv * q0).normalize();
        let utan1 = (einv * q1).normalize();
        let det = utan1.cross(utan0);
        if det.abs() < 1e-10 {
            self.do_line(p2);
            return;
        }
        let utanm = (einv * (p2 - self.last_pt)).normalize();
        let n0 = self.expand.abs() * utan0.turn_90();
        let n1 = self.expand.abs() * utan1.turn_90();
        let mid_chord = self.last_pt.midpoint(p2);
        let m = mid_chord.midpoint(p1) + self.expand.abs() * utanm.turn_90();
        let out_p0 = self.last_pt + n0;
        let out_p3 = p2 + n1;
        let rhs = einv * (m - out_p0.midpoint(out_p3));
        let idet = (8. / 3.) / det;
        let a = (utan1.cross(rhs) * idet).max(0.0);
        let b = (utan0.cross(rhs) * idet).max(0.0);
        self.do_join(n0, q0, false);
        let out_p1 = out_p0 + self.expand * (a * utan0);
        let out_p2 = out_p3 - self.expand * (b * utan1);
        self.result.curve_to(out_p1, out_p2, out_p3);
        self.last_n = Some(n1);
        self.last_tan = q1;
        self.last_pt = p2;
    }

    /// Process a cubic segment. Includes initial join.
    fn do_cubic(&mut self, p1: Point, p2: Point, p3: Point) {
        let einv = self.expand.inv();
        let c = CubicBez::new(einv * self.last_pt, einv * p1, einv * p2, einv * p3);
        let (tan0, tan1) = PathSeg::Cubic(c).tangents();
        let utan0 = tan0.normalize();
        let utan1 = tan1.normalize();
        let q = c.deriv();
        let p1xp0 = q.p1.to_vec2().cross(q.p0.to_vec2());
        let p2xp1 = q.p2.to_vec2().cross(q.p1.to_vec2());
        let cx = CubicCtx { q, utan0, utan1 };
        // First we try one-point, but only if there isn't one inflection point.
        let mut soln = if p1xp0 * p2xp1 >= 0.0 {
            try_one_point(&cx)
        } else {
            None
        };
        if soln.is_none() {
            // We try two-point linear if we don't have a one-point solution. A
            // more sophisticated approach would be to evaluate error and pick a
            // minimum, but that would be more complexity and take time. This is
            // very likely good enough for the purpose.
            soln = two_point_linear(&cx);
        }
        if let Some((a, b)) = soln {
            let n0 = self.expand.abs() * utan0.turn_90();
            let n1 = self.expand.abs() * utan1.turn_90();
            self.do_join(n0, self.expand * utan0, false);
            let out_p3 = p3 + n1;
            // TODO: clamp to correct direction
            let out_p1 = p1 + n0 + self.expand.abs() * (a * utan0);
            let out_p2 = p2 + n1 + self.expand.abs() * (b * utan1);
            self.result.curve_to(out_p1, out_p2, out_p3);
            self.last_n = Some(n1);
            self.last_tan = self.expand * utan1;
            self.last_pt = p3;
        } else {
            self.do_line(p3);
        }
    }

    /// Do a join.
    ///
    /// The `tan` parameter is a vector tangent to the start of the new segment.
    /// The `n` parameter is the normal vector (turned tangent) scaled by the expansion.
    fn do_join(&mut self, n: Vec2, tan: Vec2, is_line: bool) {
        // TODO: other join types etc
        if let Some(last_n) = self.last_n {
            let p = self.last_pt + n;
            if !self.in_tolerance(n - last_n) {
                if self.join != Join::Bevel {
                    let cross = self.last_tan.cross(tan);
                    if cross * self.expand.xx < 0.0 {
                        match self.join {
                            Join::Bevel => unreachable!(),
                            Join::Miter => {
                                let dot = self.last_tan.dot(tan);
                                let hypot = cross.hypot(dot);
                                if 2.0 * hypot < (hypot + dot) * self.miter_limit * self.miter_limit
                                {
                                    let h = (n - last_n).cross(tan) / self.last_tan.cross(tan);
                                    let miter_pt = self.last_pt + last_n + h * self.last_tan;
                                    // A cheap optimization to reduce line segments with joins to lines
                                    if let Some(PathEl::LineTo(p)) =
                                        self.result.elements_mut().last_mut()
                                    {
                                        *p = miter_pt;
                                    } else {
                                        self.result.line_to(miter_pt);
                                    }
                                    if is_line {
                                        return;
                                    }
                                }
                            }
                            Join::Round => {
                                // Cheaper inverse; everything is normalized so we don't care about uniform scaling.
                                let einv = Diagonal2::new(self.expand.yy, self.expand.xx);
                                let last_tann = einv * self.last_tan;
                                let tann = einv * tan;
                                let crossn = (last_tann).cross(tann);
                                let dotn = (last_tann).dot(tann);
                                let angle = crossn.atan2(dotn).abs();
                                let nt = self.expand * tann.normalize();
                                let a = Affine::new([
                                    n.x,
                                    n.y,
                                    nt.x,
                                    nt.y,
                                    self.last_pt.x,
                                    self.last_pt.y,
                                ]);
                                let arc: Arc =
                                    Arc::new(Point::ORIGIN, (1.0, 1.0), -angle, angle, 0.0);
                                let tolerance = self.tolerance / einv.xx.abs().max(einv.yy.abs());
                                arc.to_cubic_beziers(tolerance, |p1, p2, p3| {
                                    self.result.curve_to(a * p1, a * p2, a * p3);
                                });
                                return;
                            }
                        }
                    }
                }
                // Bevel case
                self.result.line_to(p);
            }
        } else {
            self.result.move_to(self.last_pt + n);
            self.first_n = Some(n);
            self.first_tan = tan;
        }
    }

    /// Close an open subpath if there is one; no-op if already closed.
    fn do_close_path(&mut self, first_pt: Point) {
        if let Some(first_n) = self.first_n.take() {
            // could do this test inside do_line for all lines, but it already checks for 0-length
            if first_pt.distance_squared(self.last_pt) > self.tolerance * self.tolerance {
                self.do_line(first_pt);
            }
            self.do_join(first_n, self.first_tan, true);
            self.result.close_path();
        }
        self.last_n = None;
    }
}

// Note: here is our own copy of sophisticated cubic Bézier offset logic. We have
// the ability to apply anisotropic expansion, but not cusp detection or subdivision,
// and I've also stripped out all the error evaluation. At some point, we want to
// redo stroking, and there may be an opportunity to share code.

/// Try to compute one-point shape control for a cubic.
///
/// Result is (a, b) parameters.
fn try_one_point(cx: &CubicCtx) -> Option<(f64, f64)> {
    // TODO: possibly reduce duplication with quadratic case.
    let tan = cx.q.eval(0.5).to_vec2();
    let tan_hypot2 = tan.hypot2();
    if tan_hypot2 < 1e-12 {
        return None;
    }
    let utan = tan / tan_hypot2.sqrt();
    let z = (utan - 0.5 * (cx.utan0 + cx.utan1)).turn_90();
    let cross = cx.utan0.cross(cx.utan1);
    if cross.abs() < 1e-12 {
        return None;
    }
    let idet = (8. / 3.) / cross;
    let a = z.cross(cx.utan1) * idet;
    let b = cx.utan0.cross(z) * idet;
    //let delta_tan = 0.75 * (b * cx.utan1 - a * cx.utan0) + 1.5 * (cx.utan1 - cx.utan0).turn_90();
    //let angle_err = delta_tan.cross(utan);
    //let err_est = 0.16 * angle_err.abs();
    Some((a, b))
}

fn two_point_linear(cx: &CubicCtx) -> Option<(f64, f64)> {
    const T0: f64 = 0.35;
    let s = [T0, 1.0 - T0].map(|t| {
        let utan = cx.q.eval(t).to_vec2().normalize();
        let n = utan.turn_90();
        let utan0_n = cx.utan0.dot(n);
        let utan1_n = cx.utan1.dot(n);
        let utan0xn = cx.utan0.dot(utan);
        let utan1xn = cx.utan1.dot(utan);
        let mt = 1.0 - t;
        let b0 = mt * mt * mt;
        let b1 = 3.0 * mt * t * mt;
        let b2 = 3.0 * mt * t * t;
        let b3 = t * t * t;
        let a_n = b1 * utan0_n;
        let b_n = b2 * utan1_n;
        let c_n = (b0 + b1) * utan0xn + (b2 + b3) * utan1xn - 1.0;
        TwoPointSample { a_n, b_n, c_n }
    });
    let det = s[0].a_n * s[1].b_n - s[1].a_n * s[0].b_n;
    // Consider both near-zero and NaN determinants to be failures.
    if det.abs().partial_cmp(&1e-12) != Some(core::cmp::Ordering::Greater) {
        return None;
    }
    let idet = -1.0 / det;
    let a = idet * (s[0].c_n * s[1].b_n - s[1].c_n * s[0].b_n);
    let b = idet * (s[0].a_n * s[1].c_n - s[1].a_n * s[0].c_n);
    Some((a, b))
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        Affine, BezPath, Circle, Diagonal2, Join, ParamCurveMoments, PathEl, Point, Rect, Shape,
        Size, Vec2, expand_path,
    };

    #[test]
    fn expand_circle_basic() {
        const CENTER: Point = Point::new(100., 100.);
        const RADIUS: f64 = 10.0;
        let circle = Circle::new(CENTER, RADIUS);
        let expected_area = PI * RADIUS.powi(2);
        let expected_perimeter = 2.0 * PI * RADIUS;
        assert!((circle.area() - expected_area).abs() < 1e-9);
        assert!((circle.perimeter(1e-12) - expected_perimeter).abs() < 1e-9);
        const R_DELTA: f64 = 2.0;
        const EXPAND: Diagonal2 = Diagonal2::new(R_DELTA, R_DELTA);
        let expanded = expand_path(circle, EXPAND, Join::Bevel, 4.0, 1e-3);
        let expected_area_expanded = PI * (RADIUS + R_DELTA).powi(2);
        let expected_perimeter_expanded = 2.0 * PI * (RADIUS + R_DELTA);
        assert!((expanded.area() - expected_area_expanded).abs() < 4e-2);
        assert!((expanded.perimeter(1e-12) - expected_perimeter_expanded).abs() < 3e-3);

        let mirror = Affine::reflect(CENTER, Vec2::new(0.0, 1.0));
        // We have to convert to a path here as the circle type doesn't support negative area.
        let circle_mirror = mirror * circle.to_path(1e-3);
        let expanded_mirror = expand_path(circle_mirror, EXPAND, Join::Bevel, 4.0, 1e-3);
        assert!((-expanded_mirror.area() - expected_area_expanded).abs() < 4e-2);
        assert!((expanded_mirror.perimeter(1e-12) - expected_perimeter_expanded).abs() < 3e-3);
    }

    #[test]
    fn expand_ellipse_anisotropic() {
        const CENTER: Point = Point::new(100., 100.);
        const RADIUS: f64 = 10.0;
        let circle = Circle::new(CENTER, RADIUS);
        const STRETCH: f64 = 2.717;
        let ellipse = Affine::scale_non_uniform(STRETCH, 1.0) * circle;
        let expected_area = PI * RADIUS.powi(2) * STRETCH;
        assert!((ellipse.area() - expected_area).abs() < 1e-9);
        const R_DELTA: f64 = 2.0;
        const EXPAND: Diagonal2 = Diagonal2::new(R_DELTA * STRETCH, R_DELTA);
        let expanded = expand_path(ellipse, EXPAND, Join::Bevel, 4.0, 1e-3);
        let expected_area_expanded = PI * (RADIUS + R_DELTA).powi(2) * STRETCH;
        assert!((expanded.area() - expected_area_expanded).abs() < 4e-2);
    }

    #[test]
    fn expand_rect() {
        const CENTER: Point = Point::new(100., 100.);
        const SIZE: Size = Size::new(30., 20.);
        let rect = Rect::from_center_size(CENTER, SIZE);
        assert!((rect.area() - SIZE.area()).abs() < 1e-9);
        const EXPAND: Diagonal2 = Diagonal2::new(10.0, 10.0);
        let mitered = expand_path(rect, EXPAND, Join::Miter, 4.0, 1e-3);
        let expected_area_mitered =
            (SIZE.width + 2.0 * EXPAND.xx) * (SIZE.height + 2.0 * EXPAND.yy);
        assert!((mitered.area() - expected_area_mitered).abs() < 1e-9);

        let beveled = expand_path(rect, EXPAND, Join::Bevel, 4.0, 1e-3);
        let expected_area_beveled = expected_area_mitered - 2.0 * EXPAND.xx * EXPAND.yy;
        assert!((beveled.area() - expected_area_beveled).abs() < 1e-9);

        let rounded = expand_path(rect, EXPAND, Join::Round, 4.0, 0.1);
        let expected_area_rounded = expected_area_mitered - (4.0 - PI) * EXPAND.xx * EXPAND.yy;
        assert!((rounded.area() - expected_area_rounded).abs() < 1e-1);
    }

    #[test]
    fn expand_rounding_tolerance() {
        const CENTER: Point = Point::new(100., 100.);
        const SIZE: Size = Size::new(30., 20.);
        let rect = Rect::from_center_size(CENTER, SIZE);
        const EXPAND: Diagonal2 = Diagonal2::new(10.0, 10.0);
        let expected_area_mitered =
            (SIZE.width + 2.0 * EXPAND.xx) * (SIZE.height + 2.0 * EXPAND.yy);
        let expected_area_rounded = expected_area_mitered - (4.0 - PI) * EXPAND.xx * EXPAND.yy;
        let rounded = expand_path(rect, EXPAND, Join::Round, 4.0, 1e-3);
        assert!((rounded.area() - expected_area_rounded).abs() < 2e-3);
        // Validate that a lower tolerance increases accuracy.
        let rounded_fine = expand_path(rect, EXPAND, Join::Round, 4.0, 1e-6);
        assert!((rounded_fine.area() - expected_area_rounded).abs() < 3e-5);
    }

    #[test]
    fn expand_glyph_shape() {
        // WARNING: this test is fragile. If there is optimization to simplify inner joins by
        // computing intersections, the measurements change. A properly written test would
        // compare the result after path intersection, or alternatively probe for zero/nonzero
        // winding number for a collection of sample points. But at least it is sensitive to
        // regressions.

        // Roboto "e" glyph
        let path = BezPath::from_svg(
            "M10.359375,-0.359375 Q6.484375,-0.359375 4.0625,2.1875 Q1.640625,4.734375 1.640625,8.984375 \
             L1.640625,9.578125 Q1.640625,12.40625 2.71875,14.625 Q3.796875,16.859375 5.734375,18.109375 \
             Q7.6875,19.375 9.953125,19.375 Q13.65625,19.375 15.703125,16.921875 Q17.765625,14.484375 17.765625,9.9375 \
             L17.765625,8.578125 L4.890625,8.578125 Q4.953125,5.765625 6.53125,4.03125 Q8.109375,2.296875 10.53125,2.296875 \
             Q12.25,2.296875 13.4375,3 Q14.640625,3.703125 15.546875,4.875 L17.53125,3.328125 Q15.140625,-0.359375 10.359375,-0.359375 Z \
             M9.953125,16.703125 Q7.984375,16.703125 6.640625,15.265625 Q5.3125,13.828125 5,11.25 L14.515625,11.25 L14.515625,11.5 \
             Q14.375,13.96875 13.171875,15.328125 Q11.984375,16.703125 9.953125,16.703125 Z",
        )
        .unwrap();
        const EXPAND: Diagonal2 = Diagonal2::new(1.5, 1.0);
        let expanded = expand_path(&path, EXPAND, Join::Round, 4.0, 0.1);
        //println!("{}", expanded.to_svg());
        // The outline has been visually verified, these are measurements taken from known-good.
        const EXPECTED_AREA: f64 = -291.3217958410297;
        const EXPECTED_PERIMETER: f64 = 120.89147879578239;
        const EXPECTED_MOMENT_X: f64 = -2788.568539588466;
        const EXPECTED_MOMENT_Y: f64 = -2801.3818805722685;
        const EXPECTED_MOMENT_XX: f64 = -34280.940819978576;
        const EXPECTED_MOMENT_XY: f64 = -26971.281063345254;
        const EXPECTED_MOMENT_YY: f64 = -36722.298990246876;
        assert!((expanded.area() - EXPECTED_AREA).abs() < 1e-3);
        assert!((expanded.perimeter(1e-9) - EXPECTED_PERIMETER).abs() < 1e-3);
        let moments = expanded.moments();
        assert!((moments.moment_x - EXPECTED_MOMENT_X).abs() < 1e-3);
        assert!((moments.moment_y - EXPECTED_MOMENT_Y).abs() < 1e-3);
        assert!((moments.moment_xx - EXPECTED_MOMENT_XX).abs() < 1e-3);
        assert!((moments.moment_xy - EXPECTED_MOMENT_XY).abs() < 1e-3);
        assert!((moments.moment_yy - EXPECTED_MOMENT_YY).abs() < 1e-3);
    }

    #[test]
    fn assert_expand_subpath_closed() {
        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.line_to((100.0, 0.0));
        path.line_to((100.0, 100.0));
        path.line_to((0.0, 100.0));

        let expanded = expand_path(path, Diagonal2::new(10.0, 10.0), Join::Miter, 4.0, 1e-3);
        assert_eq!(expanded.elements().last(), Some(&PathEl::ClosePath));
    }

    #[test]
    fn expand_degenerate_input() {
        let path = BezPath::from_svg("M0,0 C0,0 0,0 0,0 Z").unwrap();
        let expanded = expand_path(path, Diagonal2::new(10.0, 10.0), Join::Miter, 4.0, 1e-3);
        assert!(expanded.is_empty());
    }

    #[test]
    fn expand_degenerate_ghost() {
        let mut path = BezPath::new();
        path.extend(Rect::new(0.0, 0.0, 10.0, 10.0).path_elements(1e-3));
        let expected = expand_path(&path, Diagonal2::new(1.0, 1.0), Join::Miter, 4.0, 1e-3);

        path.move_to((20.0, 20.0));
        path.close_path();
        let actual = expand_path(&path, Diagonal2::new(1.0, 1.0), Join::Miter, 4.0, 1e-3);

        assert_eq!(actual, expected);
    }

    #[test]
    fn expand_open_subpath_uses_implicit_close_for_area_sign() {
        let mut open = BezPath::new();
        open.move_to((100.0, 100.0));
        open.line_to((110.0, 100.0));
        open.line_to((100.0, 110.0));

        let mut closed = open.clone();
        closed.close_path();

        let expand = Diagonal2::new(1.0, 1.0);
        let open_expanded = expand_path(open, expand, Join::Miter, 4.0, 1e-3);
        let closed_expanded = expand_path(closed, expand, Join::Miter, 4.0, 1e-3);

        assert_eq!(open_expanded, closed_expanded);
    }
}
