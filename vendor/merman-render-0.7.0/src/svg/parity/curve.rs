use crate::model::LayoutPoint;

use super::path_bounds::{CubicBezier, SvgPathBounds, svg_path_bounds_include_cubic};
use super::*;

#[derive(Debug, Clone, Copy)]
struct PathPoint {
    x: f64,
    y: f64,
}

impl PathPoint {
    const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    const fn nan() -> Self {
        Self {
            x: f64::NAN,
            y: f64::NAN,
        }
    }

    fn from_layout(p: &LayoutPoint) -> Self {
        Self { x: p.x, y: p.y }
    }

    const fn swap_xy(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PathCubic {
    c1: PathPoint,
    c2: PathPoint,
    end: PathPoint,
}

impl PathCubic {
    const fn new(c1: PathPoint, c2: PathPoint, end: PathPoint) -> Self {
        Self { c1, c2, end }
    }

    const fn swap_xy(self) -> Self {
        Self {
            c1: self.c1.swap_xy(),
            c2: self.c2.swap_xy(),
            end: self.end.swap_xy(),
        }
    }
}

#[derive(Debug, Default)]
struct BoundsBuilder {
    bounds: Option<SvgPathBounds>,
    cur: Option<PathPoint>,
}

impl BoundsBuilder {
    fn include_point(&mut self, point: PathPoint) {
        if let Some(b) = &mut self.bounds {
            b.min_x = b.min_x.min(point.x);
            b.min_y = b.min_y.min(point.y);
            b.max_x = b.max_x.max(point.x);
            b.max_y = b.max_y.max(point.y);
        } else {
            self.bounds = Some(SvgPathBounds {
                min_x: point.x,
                min_y: point.y,
                max_x: point.x,
                max_y: point.y,
            });
        }
    }

    fn on_pair(&mut self, cmd: char, point: PathPoint) {
        match cmd {
            'M' | 'L' => {
                self.include_point(point);
                self.cur = Some(point);
            }
            _ => {}
        }
    }

    fn on_cubic(&mut self, cubic: PathCubic) {
        let Some(start) = self.cur else {
            // Defensive: cubic without a current point is invalid; fall back to including end point.
            self.include_point(cubic.end);
            self.cur = Some(cubic.end);
            return;
        };

        if self.bounds.is_none() {
            self.bounds = Some(SvgPathBounds {
                min_x: start.x,
                min_y: start.y,
                max_x: start.x,
                max_y: start.y,
            });
        }
        if let Some(b) = &mut self.bounds {
            svg_path_bounds_include_cubic(
                b,
                CubicBezier {
                    x0: start.x,
                    y0: start.y,
                    x1: cubic.c1.x,
                    y1: cubic.c1.y,
                    x2: cubic.c2.x,
                    y2: cubic.c2.y,
                    x3: cubic.end.x,
                    y3: cubic.end.y,
                },
            );
        }
        self.cur = Some(cubic.end);
    }
}

#[inline]
fn emit_cmd_pair_no_bounds(out: &mut String, cmd: char, point: PathPoint) {
    out.push(cmd);
    fmt_path_into(out, point.x);
    out.push(',');
    fmt_path_into(out, point.y);
}

#[inline]
fn emit_cmd_pair_with_bounds(
    out: &mut String,
    bounds: &mut BoundsBuilder,
    cmd: char,
    point: PathPoint,
) {
    out.push(cmd);
    fmt_path_into(out, point.x);
    out.push(',');
    fmt_path_into(out, point.y);
    bounds.on_pair(cmd, point);
}

fn emit_cmd_pair_impl(
    out: &mut String,
    bounds: Option<&mut BoundsBuilder>,
    cmd: char,
    point: PathPoint,
) {
    if let Some(b) = bounds {
        emit_cmd_pair_with_bounds(out, b, cmd, point);
    } else {
        emit_cmd_pair_no_bounds(out, cmd, point);
    }
}

#[inline]
fn emit_cmd_cubic_no_bounds(out: &mut String, cubic: PathCubic) {
    out.push('C');
    fmt_path_into(out, cubic.c1.x);
    out.push(',');
    fmt_path_into(out, cubic.c1.y);
    out.push(',');
    fmt_path_into(out, cubic.c2.x);
    out.push(',');
    fmt_path_into(out, cubic.c2.y);
    out.push(',');
    fmt_path_into(out, cubic.end.x);
    out.push(',');
    fmt_path_into(out, cubic.end.y);
}

#[inline]
fn emit_cmd_cubic_with_bounds(out: &mut String, bounds: &mut BoundsBuilder, cubic: PathCubic) {
    out.push('C');
    fmt_path_into(out, cubic.c1.x);
    out.push(',');
    fmt_path_into(out, cubic.c1.y);
    out.push(',');
    fmt_path_into(out, cubic.c2.x);
    out.push(',');
    fmt_path_into(out, cubic.c2.y);
    out.push(',');
    fmt_path_into(out, cubic.end.x);
    out.push(',');
    fmt_path_into(out, cubic.end.y);
    bounds.on_cubic(cubic);
}

fn emit_cmd_cubic_impl(out: &mut String, bounds: Option<&mut BoundsBuilder>, cubic: PathCubic) {
    if let Some(b) = bounds {
        emit_cmd_cubic_with_bounds(out, b, cubic);
    } else {
        emit_cmd_cubic_no_bounds(out, cubic);
    }
}

pub(super) fn curve_monotone_path_d_and_bounds(
    points: &[LayoutPoint],
    swap_xy: bool,
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_monotone_path_d_impl(points, swap_xy, Some(&mut b));
    (d, b.bounds)
}

fn curve_monotone_path_d_impl(
    points: &[LayoutPoint],
    swap_xy: bool,
    bounds: Option<&mut BoundsBuilder>,
) -> String {
    fn sign(v: f64) -> f64 {
        if v < 0.0 { -1.0 } else { 1.0 }
    }

    fn to_monotone_point(p: &LayoutPoint, swap_xy: bool) -> PathPoint {
        let point = PathPoint::from_layout(p);
        if swap_xy { point.swap_xy() } else { point }
    }

    fn emit_pair_to(
        out: &mut String,
        cmd: char,
        point: PathPoint,
        swap_xy: bool,
        bounds: Option<&mut BoundsBuilder>,
    ) {
        let point = if swap_xy { point.swap_xy() } else { point };
        emit_cmd_pair_impl(out, bounds, cmd, point);
    }

    fn emit_cubic_to(
        out: &mut String,
        cubic: PathCubic,
        swap_xy: bool,
        bounds: Option<&mut BoundsBuilder>,
    ) {
        let cubic = if swap_xy { cubic.swap_xy() } else { cubic };
        emit_cmd_cubic_impl(out, bounds, cubic);
    }

    fn slope3(p0: PathPoint, p1: PathPoint, p2: PathPoint) -> f64 {
        let h0 = p1.x - p0.x;
        let h1 = p2.x - p1.x;
        let denom0 = if h0 != 0.0 {
            h0
        } else if h1 < 0.0 {
            -0.0
        } else {
            0.0
        };
        let denom1 = if h1 != 0.0 {
            h1
        } else if h0 < 0.0 {
            -0.0
        } else {
            0.0
        };
        let s0 = (p1.y - p0.y) / denom0;
        let s1 = (p2.y - p1.y) / denom1;
        let p = (s0 * h1 + s1 * h0) / (h0 + h1);
        let v = (sign(s0) + sign(s1)) * s0.abs().min(s1.abs()).min(0.5 * p.abs());
        if v.is_finite() { v } else { 0.0 }
    }

    fn slope2(p0: PathPoint, p1: PathPoint, t: f64) -> f64 {
        let h = p1.x - p0.x;
        if h != 0.0 {
            (3.0 * (p1.y - p0.y) / h - t) / 2.0
        } else {
            t
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct HermiteSegment {
        start: PathPoint,
        end: PathPoint,
        t0: f64,
        t1: f64,
    }

    fn hermite_segment(
        out: &mut String,
        segment: HermiteSegment,
        swap_xy: bool,
        bounds: Option<&mut BoundsBuilder>,
    ) {
        let HermiteSegment { start, end, t0, t1 } = segment;
        // dx is in the monotone coordinate system; we swap at emit-time if needed.
        let dx = (end.x - start.x) / 3.0;
        emit_cubic_to(
            out,
            PathCubic::new(
                PathPoint::new(start.x + dx, start.y + dx * t0),
                PathPoint::new(end.x - dx, end.y - dx * t1),
                end,
            ),
            swap_xy,
            bounds,
        );
    }

    let mut bounds = bounds;
    let mut out = String::with_capacity(points.len().saturating_mul(64));
    if points.is_empty() {
        return out;
    }

    let mut point_state: u8 = 0;
    let mut p0 = PathPoint::nan();
    let mut p1 = PathPoint::nan();
    let mut t0 = f64::NAN;

    for p in points {
        let point = to_monotone_point(p, swap_xy);

        if point.x == p1.x && point.y == p1.y {
            continue;
        }

        let mut t1 = f64::NAN;
        match point_state {
            0 => {
                point_state = 1;
                emit_pair_to(&mut out, 'M', point, swap_xy, bounds.as_deref_mut());
            }
            1 => {
                point_state = 2;
            }
            2 => {
                point_state = 3;
                t1 = slope3(p0, p1, point);
                let t0_local = slope2(p0, p1, t1);
                hermite_segment(
                    &mut out,
                    HermiteSegment {
                        start: p0,
                        end: p1,
                        t0: t0_local,
                        t1,
                    },
                    swap_xy,
                    bounds.as_deref_mut(),
                );
            }
            _ => {
                t1 = slope3(p0, p1, point);
                hermite_segment(
                    &mut out,
                    HermiteSegment {
                        start: p0,
                        end: p1,
                        t0,
                        t1,
                    },
                    swap_xy,
                    bounds.as_deref_mut(),
                );
            }
        }

        p0 = p1;
        p1 = point;
        t0 = t1;
    }

    match point_state {
        2 => emit_pair_to(&mut out, 'L', p1, swap_xy, bounds.as_deref_mut()),
        3 => {
            let t1 = slope2(p0, p1, t0);
            hermite_segment(
                &mut out,
                HermiteSegment {
                    start: p0,
                    end: p1,
                    t0,
                    t1,
                },
                swap_xy,
                bounds,
            );
        }
        _ => {}
    }

    out
}

// Ported from D3 `curveBasis` (d3-shape v3.x), used by Mermaid ER renderer `@11.12.2`.
pub(super) fn curve_basis_path_d(points: &[LayoutPoint]) -> String {
    curve_basis_path_d_impl(points, None)
}

pub(super) fn curve_basis_path_d_and_bounds(
    points: &[LayoutPoint],
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_basis_path_d_impl(points, Some(&mut b));
    (d, b.bounds)
}

fn curve_basis_path_d_impl(points: &[LayoutPoint], bounds: Option<&mut BoundsBuilder>) -> String {
    let mut out = String::with_capacity(points.len().saturating_mul(64));
    if points.is_empty() {
        return out;
    }

    fn basis_point(
        out: &mut String,
        bounds: Option<&mut BoundsBuilder>,
        previous: PathPoint,
        current: PathPoint,
        next: PathPoint,
    ) {
        emit_cmd_cubic_impl(
            out,
            bounds,
            PathCubic::new(
                PathPoint::new(
                    (2.0 * previous.x + current.x) / 3.0,
                    (2.0 * previous.y + current.y) / 3.0,
                ),
                PathPoint::new(
                    (previous.x + 2.0 * current.x) / 3.0,
                    (previous.y + 2.0 * current.y) / 3.0,
                ),
                PathPoint::new(
                    (previous.x + 4.0 * current.x + next.x) / 6.0,
                    (previous.y + 4.0 * current.y + next.y) / 6.0,
                ),
            ),
        );
    }

    let mut bounds = bounds;
    let mut point_state = 0u8;
    let mut previous = PathPoint::nan();
    let mut current = PathPoint::nan();

    for pt in points {
        let next = PathPoint::from_layout(pt);
        match point_state {
            0 => {
                point_state = 1;
                emit_cmd_pair_impl(&mut out, bounds.as_deref_mut(), 'M', next);
            }
            1 => {
                point_state = 2;
            }
            2 => {
                point_state = 3;
                let line_to = PathPoint::new(
                    (5.0 * previous.x + current.x) / 6.0,
                    (5.0 * previous.y + current.y) / 6.0,
                );
                emit_cmd_pair_impl(&mut out, bounds.as_deref_mut(), 'L', line_to);
                basis_point(&mut out, bounds.as_deref_mut(), previous, current, next);
            }
            _ => {
                basis_point(&mut out, bounds.as_deref_mut(), previous, current, next);
            }
        }
        previous = current;
        current = next;
    }

    match point_state {
        3 => {
            basis_point(&mut out, bounds.as_deref_mut(), previous, current, current);
            emit_cmd_pair_impl(&mut out, bounds, 'L', current);
        }
        2 => {
            emit_cmd_pair_impl(&mut out, bounds, 'L', current);
        }
        _ => {}
    }

    out
}

pub(super) fn curve_linear_path_d(points: &[LayoutPoint]) -> String {
    curve_linear_path_d_impl(points, None)
}

pub(super) fn curve_linear_path_d_and_bounds(
    points: &[LayoutPoint],
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_linear_path_d_impl(points, Some(&mut b));
    (d, b.bounds)
}

fn curve_linear_path_d_impl(points: &[LayoutPoint], bounds: Option<&mut BoundsBuilder>) -> String {
    let mut bounds = bounds;
    let mut out = String::with_capacity(points.len().saturating_mul(32));
    let Some(first) = points.first() else {
        return out;
    };
    emit_cmd_pair_impl(
        &mut out,
        bounds.as_deref_mut(),
        'M',
        PathPoint::from_layout(first),
    );
    for p in points.iter().skip(1) {
        emit_cmd_pair_impl(
            &mut out,
            bounds.as_deref_mut(),
            'L',
            PathPoint::from_layout(p),
        );
    }
    out
}

pub(super) fn curve_rounded_path_d_and_bounds(
    points: &[LayoutPoint],
    radius: f64,
) -> (String, Option<SvgPathBounds>) {
    (curve_rounded_path_d_impl(points, radius), None)
}

fn curve_rounded_path_d_impl(points: &[LayoutPoint], radius: f64) -> String {
    if points.len() < 2 {
        return String::new();
    }

    let mut out = String::with_capacity(points.len().saturating_mul(48));
    let epsilon = 1e-5;

    for (idx, curr) in points.iter().enumerate() {
        if idx == 0 {
            emit_cmd_pair_no_bounds(&mut out, 'M', PathPoint::from_layout(curr));
            continue;
        }
        if idx + 1 == points.len() {
            emit_cmd_pair_no_bounds(&mut out, 'L', PathPoint::from_layout(curr));
            continue;
        }

        let prev = &points[idx - 1];
        let next = &points[idx + 1];

        let dx1 = curr.x - prev.x;
        let dy1 = curr.y - prev.y;
        let dx2 = next.x - curr.x;
        let dy2 = next.y - curr.y;
        let len1 = dx1.hypot(dy1);
        let len2 = dx2.hypot(dy2);
        if len1 < epsilon || len2 < epsilon {
            emit_cmd_pair_no_bounds(&mut out, 'L', PathPoint::from_layout(curr));
            continue;
        }

        let nx1 = dx1 / len1;
        let ny1 = dy1 / len1;
        let nx2 = dx2 / len2;
        let ny2 = dy2 / len2;
        let dot = (nx1 * nx2 + ny1 * ny2).clamp(-1.0, 1.0);
        let angle = dot.acos();
        if angle < epsilon || (std::f64::consts::PI - angle).abs() < epsilon {
            emit_cmd_pair_no_bounds(&mut out, 'L', PathPoint::from_layout(curr));
            continue;
        }

        let cut_len = (radius / (angle / 2.0).sin())
            .min(len1 / 2.0)
            .min(len2 / 2.0);
        let start = PathPoint::new(curr.x - nx1 * cut_len, curr.y - ny1 * cut_len);
        let end = PathPoint::new(curr.x + nx2 * cut_len, curr.y + ny2 * cut_len);

        emit_cmd_pair_no_bounds(&mut out, 'L', start);
        out.push('Q');
        fmt_path_into(&mut out, curr.x);
        out.push(',');
        fmt_path_into(&mut out, curr.y);
        out.push(' ');
        fmt_path_into(&mut out, end.x);
        out.push(',');
        fmt_path_into(&mut out, end.y);
    }

    out
}

// Ported from D3 `curveNatural` (d3-shape v3.x).
//
// This is used by Mermaid flowchart edge-id curve overrides (e.g. `e1@{ curve: natural }`).
pub(super) fn curve_natural_path_d_and_bounds(
    points: &[LayoutPoint],
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_natural_path_d_impl(points, Some(&mut b));
    (d, b.bounds)
}

fn curve_natural_path_d_impl(points: &[LayoutPoint], bounds: Option<&mut BoundsBuilder>) -> String {
    fn compute_control_points(coords: &[f64]) -> (Vec<f64>, Vec<f64>) {
        // `coords` contains the knot coordinates for points[0..=n], where n = segment count.
        let n = coords.len().saturating_sub(1);
        let mut c1 = vec![0.0f64; n];
        let mut c2 = vec![0.0f64; n];
        if n == 0 {
            return (c1, c2);
        }

        // Tridiagonal solve for first control points (Thomas algorithm).
        let mut a = vec![0.0f64; n];
        let mut b = vec![0.0f64; n];
        let mut c = vec![0.0f64; n];
        let mut rhs = vec![0.0f64; n];

        b[0] = 2.0;
        c[0] = 1.0;
        rhs[0] = coords[0] + 2.0 * coords[1];

        for i in 1..n.saturating_sub(1) {
            a[i] = 1.0;
            b[i] = 4.0;
            c[i] = 1.0;
            rhs[i] = 4.0 * coords[i] + 2.0 * coords[i + 1];
        }

        if n > 1 {
            a[n - 1] = 2.0;
            b[n - 1] = 7.0;
            rhs[n - 1] = 8.0 * coords[n - 1] + coords[n];
        } else {
            // Single segment (2 points): not used (caller returns a line), but keep the solver
            // stable anyway.
            b[0] = 2.0;
            rhs[0] = coords[0] + coords[1];
        }

        for i in 1..n {
            let m = a[i] / b[i - 1];
            b[i] -= m * c[i - 1];
            rhs[i] -= m * rhs[i - 1];
        }

        c1[n - 1] = rhs[n - 1] / b[n - 1];
        for i in (0..n.saturating_sub(1)).rev() {
            c1[i] = (rhs[i] - c[i] * c1[i + 1]) / b[i];
        }

        for i in 0..n.saturating_sub(1) {
            c2[i] = 2.0 * coords[i + 1] - c1[i + 1];
        }
        c2[n - 1] = (coords[n] + c1[n - 1]) / 2.0;

        (c1, c2)
    }

    let mut bounds = bounds;
    let mut out = String::with_capacity(points.len().saturating_mul(64));
    let Some(first) = points.first() else {
        return out;
    };
    emit_cmd_pair_impl(
        &mut out,
        bounds.as_deref_mut(),
        'M',
        PathPoint::from_layout(first),
    );
    if points.len() == 1 {
        return out;
    }
    if points.len() == 2 {
        let p1 = &points[1];
        emit_cmd_pair_impl(
            &mut out,
            bounds.as_deref_mut(),
            'L',
            PathPoint::from_layout(p1),
        );
        return out;
    }

    let mut xs: Vec<f64> = Vec::with_capacity(points.len());
    let mut ys: Vec<f64> = Vec::with_capacity(points.len());
    for p in points {
        xs.push(p.x);
        ys.push(p.y);
    }

    let (x1, x2) = compute_control_points(&xs);
    let (y1, y2) = compute_control_points(&ys);

    for i in 0..points.len().saturating_sub(1) {
        let p = &points[i + 1];
        emit_cmd_cubic_impl(
            &mut out,
            bounds.as_deref_mut(),
            PathCubic::new(
                PathPoint::new(x1[i], y1[i]),
                PathPoint::new(x2[i], y2[i]),
                PathPoint::from_layout(p),
            ),
        );
    }

    out
}

// Ported from D3 `curveStepAfter` (d3-shape v3.x).
pub(super) fn curve_step_after_path_d_and_bounds(
    points: &[LayoutPoint],
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_step_after_path_d_impl(points, Some(&mut b));
    (d, b.bounds)
}

fn curve_step_after_path_d_impl(
    points: &[LayoutPoint],
    bounds: Option<&mut BoundsBuilder>,
) -> String {
    let mut bounds = bounds;
    let mut out = String::with_capacity(points.len().saturating_mul(32));
    let Some(first) = points.first() else {
        return out;
    };
    let mut prev_y = first.y;
    emit_cmd_pair_impl(
        &mut out,
        bounds.as_deref_mut(),
        'M',
        PathPoint::from_layout(first),
    );
    for p in points.iter().skip(1) {
        emit_cmd_pair_impl(
            &mut out,
            bounds.as_deref_mut(),
            'L',
            PathPoint::new(p.x, prev_y),
        );
        emit_cmd_pair_impl(
            &mut out,
            bounds.as_deref_mut(),
            'L',
            PathPoint::from_layout(p),
        );
        prev_y = p.y;
    }
    out
}

// Ported from D3 `curveStepBefore` (d3-shape v3.x).
pub(super) fn curve_step_before_path_d_and_bounds(
    points: &[LayoutPoint],
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_step_before_path_d_impl(points, Some(&mut b));
    (d, b.bounds)
}

fn curve_step_before_path_d_impl(
    points: &[LayoutPoint],
    bounds: Option<&mut BoundsBuilder>,
) -> String {
    let mut bounds = bounds;
    let mut out = String::with_capacity(points.len().saturating_mul(32));
    let Some(first) = points.first() else {
        return out;
    };
    let mut prev_x = first.x;
    emit_cmd_pair_impl(
        &mut out,
        bounds.as_deref_mut(),
        'M',
        PathPoint::from_layout(first),
    );
    for p in points.iter().skip(1) {
        emit_cmd_pair_impl(
            &mut out,
            bounds.as_deref_mut(),
            'L',
            PathPoint::new(prev_x, p.y),
        );
        emit_cmd_pair_impl(
            &mut out,
            bounds.as_deref_mut(),
            'L',
            PathPoint::from_layout(p),
        );
        prev_x = p.x;
    }
    out
}

// Ported from D3 `curveStep` (d3-shape v3.x).
pub(super) fn curve_step_path_d_and_bounds(
    points: &[LayoutPoint],
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_step_path_d_impl(points, Some(&mut b));
    (d, b.bounds)
}

fn curve_step_path_d_impl(points: &[LayoutPoint], bounds: Option<&mut BoundsBuilder>) -> String {
    let mut bounds = bounds;
    let mut out = String::with_capacity(points.len().saturating_mul(32));
    let Some(first) = points.first() else {
        return out;
    };
    emit_cmd_pair_impl(
        &mut out,
        bounds.as_deref_mut(),
        'M',
        PathPoint::from_layout(first),
    );
    let mut prev = first;
    for p in points.iter().skip(1) {
        let mid_x = (prev.x + p.x) / 2.0;
        emit_cmd_pair_impl(
            &mut out,
            bounds.as_deref_mut(),
            'L',
            PathPoint::new(mid_x, prev.y),
        );
        emit_cmd_pair_impl(
            &mut out,
            bounds.as_deref_mut(),
            'L',
            PathPoint::new(mid_x, p.y),
        );
        emit_cmd_pair_impl(
            &mut out,
            bounds.as_deref_mut(),
            'L',
            PathPoint::from_layout(p),
        );
        prev = p;
    }
    out
}

// Ported from D3 `curveCardinal` (d3-shape v3.x).
pub(super) fn curve_cardinal_path_d_and_bounds(
    points: &[LayoutPoint],
    tension: f64,
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_cardinal_path_d_impl(points, tension, Some(&mut b));
    (d, b.bounds)
}

fn curve_cardinal_path_d_impl(
    points: &[LayoutPoint],
    tension: f64,
    bounds: Option<&mut BoundsBuilder>,
) -> String {
    let mut bounds = bounds;
    let mut out = String::with_capacity(points.len().saturating_mul(64));
    if points.is_empty() {
        return out;
    }

    let k = (1.0 - tension) / 6.0;

    #[derive(Debug, Clone, Copy)]
    struct CardinalSegment {
        k: f64,
        previous: PathPoint,
        current: PathPoint,
        next: PathPoint,
        target: PathPoint,
    }

    fn cardinal_point(
        out: &mut String,
        segment: CardinalSegment,
        bounds: Option<&mut BoundsBuilder>,
    ) {
        let CardinalSegment {
            k,
            previous,
            current,
            next,
            target,
        } = segment;
        let c1 = PathPoint::new(
            current.x + k * (next.x - previous.x),
            current.y + k * (next.y - previous.y),
        );
        let c2 = PathPoint::new(
            next.x + k * (current.x - target.x),
            next.y + k * (current.y - target.y),
        );
        emit_cmd_cubic_impl(out, bounds, PathCubic::new(c1, c2, next));
    }

    let mut point_state = 0u8;
    let mut p0 = PathPoint::nan();
    let mut p1 = PathPoint::nan();
    let mut p2 = PathPoint::nan();

    for pt in points {
        let point = PathPoint::from_layout(pt);
        match point_state {
            0 => {
                point_state = 1;
                emit_cmd_pair_impl(&mut out, bounds.as_deref_mut(), 'M', point);
            }
            1 => {
                point_state = 2;
                p1 = point;
            }
            2 => {
                point_state = 3;
                cardinal_point(
                    &mut out,
                    CardinalSegment {
                        k,
                        previous: p0,
                        current: p1,
                        next: p2,
                        target: point,
                    },
                    bounds.as_deref_mut(),
                );
            }
            _ => {
                cardinal_point(
                    &mut out,
                    CardinalSegment {
                        k,
                        previous: p0,
                        current: p1,
                        next: p2,
                        target: point,
                    },
                    bounds.as_deref_mut(),
                );
            }
        }

        p0 = p1;
        p1 = p2;
        p2 = point;
    }

    match point_state {
        2 => {
            emit_cmd_pair_impl(&mut out, bounds.as_deref_mut(), 'L', p2);
        }
        3 => {
            cardinal_point(
                &mut out,
                CardinalSegment {
                    k,
                    previous: p0,
                    current: p1,
                    next: p2,
                    target: p1,
                },
                bounds,
            );
        }
        _ => {}
    }

    out
}

// Ported from D3 `curveBumpY` (d3-shape v3.x).
//
// This is used by Mermaid flowchart edge-id curve overrides (e.g. `e1@{ curve: bumpY }`).
pub(super) fn curve_bump_y_path_d_and_bounds(
    points: &[LayoutPoint],
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_bump_y_path_d_impl(points, Some(&mut b));
    (d, b.bounds)
}

fn curve_bump_y_path_d_impl(points: &[LayoutPoint], bounds: Option<&mut BoundsBuilder>) -> String {
    let mut bounds = bounds;
    let mut out = String::new();
    let Some(first) = points.first() else {
        return out;
    };

    let mut previous = PathPoint::from_layout(first);
    emit_cmd_pair_impl(&mut out, bounds.as_deref_mut(), 'M', previous);

    for p in points.iter().skip(1) {
        let point = PathPoint::from_layout(p);
        let y_mid = (previous.y + point.y) / 2.0;
        emit_cmd_cubic_impl(
            &mut out,
            bounds.as_deref_mut(),
            PathCubic::new(
                PathPoint::new(previous.x, y_mid),
                PathPoint::new(point.x, y_mid),
                point,
            ),
        );
        previous = point;
    }

    out
}

// Ported from D3 `curveCatmullRom` (d3-shape v3.x), with the default alpha=0.5.
//
// This is used by Mermaid flowchart edge-id curve overrides (e.g. `e1@{ curve: catmullRom }`).
pub(super) fn curve_catmull_rom_path_d_and_bounds(
    points: &[LayoutPoint],
) -> (String, Option<SvgPathBounds>) {
    let mut b = BoundsBuilder::default();
    let d = curve_catmull_rom_path_d_with_alpha_impl(points, 0.5, Some(&mut b));
    (d, b.bounds)
}

fn curve_catmull_rom_path_d_with_alpha_impl(
    points: &[LayoutPoint],
    alpha: f64,
    bounds: Option<&mut BoundsBuilder>,
) -> String {
    const EPSILON: f64 = 1e-12;

    #[derive(Debug, Clone, Copy)]
    struct CatmullRomState {
        alpha: f64,
        point_state: u8,
        p0: PathPoint,
        p1: PathPoint,
        p2: PathPoint,
        l01_a: f64,
        l12_a: f64,
        l23_a: f64,
        l01_2a: f64,
        l12_2a: f64,
        l23_2a: f64,
    }

    impl CatmullRomState {
        fn new(alpha: f64) -> Self {
            Self {
                alpha,
                point_state: 0,
                p0: PathPoint::nan(),
                p1: PathPoint::nan(),
                p2: PathPoint::nan(),
                l01_a: 0.0,
                l12_a: 0.0,
                l23_a: 0.0,
                l01_2a: 0.0,
                l12_2a: 0.0,
                l23_2a: 0.0,
            }
        }

        fn emit_segment(
            &self,
            out: &mut String,
            target: PathPoint,
            bounds: Option<&mut BoundsBuilder>,
        ) {
            let mut c1 = self.p1;
            let mut c2 = self.p2;

            if self.l01_a > EPSILON {
                let a = 2.0 * self.l01_2a + 3.0 * self.l01_a * self.l12_a + self.l12_2a;
                let n = 3.0 * self.l01_a * (self.l01_a + self.l12_a);
                if n != 0.0 && n.is_finite() {
                    c1.x = (c1.x * a - self.p0.x * self.l12_2a + self.p2.x * self.l01_2a) / n;
                    c1.y = (c1.y * a - self.p0.y * self.l12_2a + self.p2.y * self.l01_2a) / n;
                }
            }

            if self.l23_a > EPSILON {
                let b = 2.0 * self.l23_2a + 3.0 * self.l23_a * self.l12_a + self.l12_2a;
                let m = 3.0 * self.l23_a * (self.l23_a + self.l12_a);
                if m != 0.0 && m.is_finite() {
                    // Note: D3 uses the original (unadjusted) `_x1/_y1` here.
                    c2.x = (c2.x * b + self.p1.x * self.l23_2a - target.x * self.l12_2a) / m;
                    c2.y = (c2.y * b + self.p1.y * self.l23_2a - target.y * self.l12_2a) / m;
                }
            }

            emit_cmd_cubic_impl(out, bounds, PathCubic::new(c1, c2, self.p2));
        }

        fn point(
            &mut self,
            out: &mut String,
            point: PathPoint,
            bounds: Option<&mut BoundsBuilder>,
        ) {
            if self.point_state != 0 {
                let dx = self.p2.x - point.x;
                let dy = self.p2.y - point.y;
                self.l23_2a = (dx * dx + dy * dy).powf(self.alpha);
                self.l23_a = self.l23_2a.sqrt();
            }

            match self.point_state {
                0 => {
                    self.point_state = 1;
                    emit_cmd_pair_impl(out, bounds, 'M', point);
                }
                1 => {
                    self.point_state = 2;
                }
                2 => {
                    self.point_state = 3;
                    self.emit_segment(out, point, bounds);
                }
                _ => {
                    self.emit_segment(out, point, bounds);
                }
            }

            self.l01_a = self.l12_a;
            self.l12_a = self.l23_a;
            self.l01_2a = self.l12_2a;
            self.l12_2a = self.l23_2a;

            self.p0 = self.p1;
            self.p1 = self.p2;
            self.p2 = point;
        }

        fn line_end(&mut self, out: &mut String, bounds: Option<&mut BoundsBuilder>) {
            match self.point_state {
                2 => {
                    emit_cmd_pair_impl(out, bounds, 'L', self.p2);
                }
                3 => {
                    // Mirror D3's `lineEnd` behavior: `this.point(this._x2, this._y2)`.
                    self.l23_a = 0.0;
                    self.l23_2a = 0.0;
                    self.emit_segment(out, self.p2, bounds);
                }
                _ => {}
            }
        }
    }

    let mut bounds = bounds;
    let mut out = String::new();
    if points.is_empty() {
        return out;
    }

    let mut state = CatmullRomState::new(alpha);
    for p in points {
        state.point(&mut out, PathPoint::from_layout(p), bounds.as_deref_mut());
    }
    state.line_end(&mut out, bounds);

    out
}
