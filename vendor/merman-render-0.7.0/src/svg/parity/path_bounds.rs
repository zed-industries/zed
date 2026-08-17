// Shared SVG path bounds helper extracted from parity.rs.
//
// Keep behavior identical to Mermaid@11.12.2 DOM parity baselines.

#[derive(Debug, Clone, Copy)]
pub(super) struct SvgPathBounds {
    pub(super) min_x: f64,
    pub(super) min_y: f64,
    pub(super) max_x: f64,
    pub(super) max_y: f64,
}

impl SvgPathBounds {
    fn include_point(&mut self, x: f64, y: f64) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct CubicBezier {
    pub(super) x0: f64,
    pub(super) y0: f64,
    pub(super) x1: f64,
    pub(super) y1: f64,
    pub(super) x2: f64,
    pub(super) y2: f64,
    pub(super) x3: f64,
    pub(super) y3: f64,
}

fn cubic_eval(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let a = -p0 + 3.0 * p1 - 3.0 * p2 + p3;
    let b = 3.0 * p0 - 6.0 * p1 + 3.0 * p2;
    let c = -3.0 * p0 + 3.0 * p1;
    ((a * t + b) * t + c) * t + p0
}

pub(super) fn svg_path_bounds_include_cubic(b: &mut SvgPathBounds, c: CubicBezier) {
    b.include_point(c.x0, c.y0);
    b.include_point(c.x3, c.y3);

    fn include_extrema(b: &mut SvgPathBounds, primary: [f64; 4], secondary: [f64; 4], is_x: bool) {
        let [p0, p1, p2, p3] = primary;
        let [fixed0, fixed1, fixed2, fixed3] = secondary;
        let a = -p0 + 3.0 * p1 - 3.0 * p2 + p3;
        let bb = 3.0 * p0 - 6.0 * p1 + 3.0 * p2;
        let c = -3.0 * p0 + 3.0 * p1;
        let qa = 3.0 * a;
        let qb = 2.0 * bb;
        let qc = c;

        const EPS: f64 = 1e-12;
        let mut roots: [f64; 2] = [f64::NAN, f64::NAN];
        let mut root_count = 0usize;
        if qa.abs() <= EPS {
            if qb.abs() > EPS {
                let t = -qc / qb;
                roots[0] = t;
                root_count = 1;
            }
        } else {
            let disc = qb * qb - 4.0 * qa * qc;
            let tol = 1e-12 * (qb * qb + (4.0 * qa * qc).abs() + 1.0);
            if disc >= -tol {
                let s = disc.max(0.0).sqrt();
                roots[0] = (-qb + s) / (2.0 * qa);
                roots[1] = (-qb - s) / (2.0 * qa);
                root_count = 2;
            }
        }

        for &t in roots.iter().take(root_count) {
            if t <= 0.0 || t >= 1.0 {
                continue;
            }
            let v = cubic_eval(p0, p1, p2, p3, t);
            let other = cubic_eval(fixed0, fixed1, fixed2, fixed3, t);
            if is_x {
                b.include_point(v, other);
            } else {
                b.include_point(other, v);
            }
        }
    }

    include_extrema(b, [c.x0, c.x1, c.x2, c.x3], [c.y0, c.y1, c.y2, c.y3], true);
    include_extrema(b, [c.y0, c.y1, c.y2, c.y3], [c.x0, c.x1, c.x2, c.x3], false);
}

pub(super) fn svg_path_bounds_from_d(d: &str) -> Option<SvgPathBounds> {
    use std::f64::consts::PI;

    fn skip_sep(bytes: &[u8], i: &mut usize) {
        while *i < bytes.len() {
            match bytes[*i] {
                b' ' | b'\n' | b'\r' | b'\t' | b',' => *i += 1,
                _ => break,
            }
        }
    }

    fn parse_number(d: &str, bytes: &[u8], i: &mut usize) -> Option<f64> {
        skip_sep(bytes, i);
        if *i >= bytes.len() {
            return None;
        }
        let start = *i;
        if matches!(bytes[*i], b'+' | b'-') {
            *i += 1;
        }
        while *i < bytes.len() && bytes[*i].is_ascii_digit() {
            *i += 1;
        }
        if *i < bytes.len() && bytes[*i] == b'.' {
            *i += 1;
            while *i < bytes.len() && bytes[*i].is_ascii_digit() {
                *i += 1;
            }
        }
        if *i < bytes.len() && matches!(bytes[*i], b'e' | b'E') {
            *i += 1;
            if *i < bytes.len() && matches!(bytes[*i], b'+' | b'-') {
                *i += 1;
            }
            while *i < bytes.len() && bytes[*i].is_ascii_digit() {
                *i += 1;
            }
        }
        d[start..*i].parse::<f64>().ok()
    }

    fn try_parse_number(d: &str, bytes: &[u8], i: &mut usize) -> Option<f64> {
        let save = *i;
        let v = parse_number(d, bytes, i);
        if v.is_none() {
            *i = save;
        }
        v
    }

    fn parse_pair(d: &str, bytes: &[u8], i: &mut usize) -> Option<(f64, f64)> {
        let x = parse_number(d, bytes, i)?;
        let y = parse_number(d, bytes, i)?;
        Some((x, y))
    }

    fn try_parse_pair(d: &str, bytes: &[u8], i: &mut usize) -> Option<(f64, f64)> {
        let save = *i;
        let v = parse_pair(d, bytes, i);
        if v.is_none() {
            *i = save;
        }
        v
    }

    fn quadratic_include_bounds(
        b: &mut SvgPathBounds,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) {
        // Convert quadratic to cubic for extrema math:
        // https://pomax.github.io/bezierinfo/#circles_cubic
        let cx1 = x0 + (2.0 / 3.0) * (x1 - x0);
        let cy1 = y0 + (2.0 / 3.0) * (y1 - y0);
        let cx2 = x2 + (2.0 / 3.0) * (x1 - x2);
        let cy2 = y2 + (2.0 / 3.0) * (y1 - y2);
        svg_path_bounds_include_cubic(
            b,
            CubicBezier {
                x0,
                y0,
                x1: cx1,
                y1: cy1,
                x2: cx2,
                y2: cy2,
                x3: x2,
                y3: y2,
            },
        );
    }

    fn normalize_angle(mut a: f64) -> f64 {
        let two_pi = 2.0 * PI;
        a %= two_pi;
        if a < 0.0 {
            a += two_pi;
        }
        a
    }

    fn angle_between(theta: f64, start: f64, delta: f64) -> bool {
        let two_pi = 2.0 * PI;
        let eps = 1e-9;
        let t = normalize_angle(theta - start);
        if delta >= 0.0 {
            t <= delta + eps
        } else {
            t >= two_pi + delta - eps
        }
    }

    fn vec_angle(ux: f64, uy: f64, vx: f64, vy: f64) -> f64 {
        let dot = ux * vx + uy * vy;
        let det = ux * vy - uy * vx;
        det.atan2(dot)
    }

    #[derive(Debug, Clone, Copy)]
    struct ArcBoundsInput {
        x0: f64,
        y0: f64,
        rx0: f64,
        ry0: f64,
        x_axis_rotation_deg: f64,
        large_arc: bool,
        sweep: bool,
        x1: f64,
        y1: f64,
    }

    fn arc_include_bounds(b: &mut SvgPathBounds, arc: ArcBoundsInput) {
        let ArcBoundsInput {
            x0,
            y0,
            rx0,
            ry0,
            x_axis_rotation_deg,
            large_arc,
            sweep,
            x1,
            y1,
        } = arc;
        // Per SVG 1.1 endpoint-to-center arc conversion.
        // See: https://www.w3.org/TR/SVG/implnote.html#ArcImplementationNotes
        if rx0.abs() < 1e-12 || ry0.abs() < 1e-12 {
            b.include_point(x0, y0);
            b.include_point(x1, y1);
            return;
        }

        let phi = x_axis_rotation_deg.to_radians();
        let (cos_phi, sin_phi) = (phi.cos(), phi.sin());
        let mut rx = rx0.abs();
        let mut ry = ry0.abs();

        let dx2 = (x0 - x1) / 2.0;
        let dy2 = (y0 - y1) / 2.0;

        let x1p = cos_phi * dx2 + sin_phi * dy2;
        let y1p = -sin_phi * dx2 + cos_phi * dy2;

        let x1p2 = x1p * x1p;
        let y1p2 = y1p * y1p;

        // Ensure radii are large enough.
        let lam = x1p2 / (rx * rx) + y1p2 / (ry * ry);
        if lam > 1.0 {
            let s = lam.sqrt();
            rx *= s;
            ry *= s;
        }

        let rx2 = rx * rx;
        let ry2 = ry * ry;

        let num = (rx2 * ry2) - (rx2 * y1p2) - (ry2 * x1p2);
        let den = (rx2 * y1p2) + (ry2 * x1p2);
        if den.abs() < 1e-24 {
            b.include_point(x0, y0);
            b.include_point(x1, y1);
            return;
        }
        let mut sq = num / den;
        if sq < 0.0 {
            sq = 0.0;
        }
        let sign = if large_arc == sweep { -1.0 } else { 1.0 };
        let coef = sign * sq.sqrt();

        let cxp = coef * (rx * y1p) / ry;
        let cyp = coef * (-ry * x1p) / rx;

        let cx = cos_phi * cxp - sin_phi * cyp + (x0 + x1) / 2.0;
        let cy = sin_phi * cxp + cos_phi * cyp + (y0 + y1) / 2.0;

        let ux = (x1p - cxp) / rx;
        let uy = (y1p - cyp) / ry;
        let vx = (-x1p - cxp) / rx;
        let vy = (-y1p - cyp) / ry;

        let theta1 = vec_angle(1.0, 0.0, ux, uy);
        let mut delta = vec_angle(ux, uy, vx, vy);

        if !sweep && delta > 0.0 {
            delta -= 2.0 * PI;
        } else if sweep && delta < 0.0 {
            delta += 2.0 * PI;
        }

        let start = theta1;
        let end = theta1 + delta;

        let arc_point = |theta: f64| -> (f64, f64) {
            let (ct, st) = (theta.cos(), theta.sin());
            let x = cx + rx * ct * cos_phi - ry * st * sin_phi;
            let y = cy + rx * ct * sin_phi + ry * st * cos_phi;
            (x, y)
        };

        b.include_point(x0, y0);
        b.include_point(x1, y1);
        let (sx, sy) = arc_point(start);
        let (ex, ey) = arc_point(end);
        b.include_point(sx, sy);
        b.include_point(ex, ey);

        // Extrema angles for rotated ellipse. See derivative analysis in many references.
        // x extrema: tan(theta) = -(ry*sin(phi)) / (rx*cos(phi))
        // y extrema: tan(theta) =  (ry*cos(phi)) / (rx*sin(phi))
        let tx_base = (-ry * sin_phi).atan2(rx * cos_phi);
        for k in 0..2 {
            let t = tx_base + (k as f64) * PI;
            if angle_between(t, start, delta) {
                let (x, y) = arc_point(t);
                b.include_point(x, y);
            }
        }

        let ty_base = (ry * cos_phi).atan2(rx * sin_phi);
        for k in 0..2 {
            let t = ty_base + (k as f64) * PI;
            if angle_between(t, start, delta) {
                let (x, y) = arc_point(t);
                b.include_point(x, y);
            }
        }
    }

    let bytes = d.as_bytes();
    let mut i = 0usize;
    let mut cmd: u8 = 0;
    let mut cx = 0.0f64;
    let mut cy = 0.0f64;
    let (mut sx, mut sy) = (0.0f64, 0.0f64);
    let mut last_cubic_ctrl: Option<(f64, f64)> = None;
    let mut last_quad_ctrl: Option<(f64, f64)> = None;
    let mut prev_cmd: u8 = 0;
    let mut b: Option<SvgPathBounds> = None;

    while i < bytes.len() {
        skip_sep(bytes, &mut i);
        if i >= bytes.len() {
            break;
        }
        let ch = bytes[i];
        if ch.is_ascii_alphabetic() {
            cmd = ch;
            i += 1;
        } else if cmd == 0 {
            return None;
        }

        let is_rel = cmd.is_ascii_lowercase();
        let ucmd = cmd.to_ascii_uppercase();

        fn ensure_bounds(b: &mut Option<SvgPathBounds>, x: f64, y: f64) -> &mut SvgPathBounds {
            b.get_or_insert(SvgPathBounds {
                min_x: x,
                min_y: y,
                max_x: x,
                max_y: y,
            })
        }

        match ucmd {
            b'M' => {
                // First pair is move-to; subsequent pairs are implicit line-to.
                let (mut x, mut y) = parse_pair(d, bytes, &mut i)?;
                if is_rel {
                    x += cx;
                    y += cy;
                }
                cx = x;
                cy = y;
                sx = x;
                sy = y;
                ensure_bounds(&mut b, cx, cy).include_point(cx, cy);
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                prev_cmd = ucmd;

                loop {
                    let Some((mut nx, mut ny)) = try_parse_pair(d, bytes, &mut i) else {
                        break;
                    };
                    if is_rel {
                        nx += cx;
                        ny += cy;
                    }
                    cx = nx;
                    cy = ny;
                    ensure_bounds(&mut b, cx, cy).include_point(cx, cy);
                    prev_cmd = b'L';
                }
            }
            b'Z' => {
                // Close path: line to subpath start.
                let cur = ensure_bounds(&mut b, cx, cy);
                cur.include_point(cx, cy);
                cur.include_point(sx, sy);
                cx = sx;
                cy = sy;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                prev_cmd = ucmd;
            }
            b'L' => {
                let (mut x, mut y) = parse_pair(d, bytes, &mut i)?;
                if is_rel {
                    x += cx;
                    y += cy;
                }
                cx = x;
                cy = y;
                ensure_bounds(&mut b, cx, cy).include_point(cx, cy);
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                prev_cmd = ucmd;

                loop {
                    let Some((mut nx, mut ny)) = try_parse_pair(d, bytes, &mut i) else {
                        break;
                    };
                    if is_rel {
                        nx += cx;
                        ny += cy;
                    }
                    cx = nx;
                    cy = ny;
                    ensure_bounds(&mut b, cx, cy).include_point(cx, cy);
                    prev_cmd = ucmd;
                }
            }
            b'H' => {
                let mut x = parse_number(d, bytes, &mut i)?;
                if is_rel {
                    x += cx;
                }
                cx = x;
                ensure_bounds(&mut b, cx, cy).include_point(cx, cy);
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                prev_cmd = ucmd;

                loop {
                    let Some(mut nx) = try_parse_number(d, bytes, &mut i) else {
                        break;
                    };
                    if is_rel {
                        nx += cx;
                    }
                    cx = nx;
                    ensure_bounds(&mut b, cx, cy).include_point(cx, cy);
                    prev_cmd = ucmd;
                }
            }
            b'V' => {
                let mut y = parse_number(d, bytes, &mut i)?;
                if is_rel {
                    y += cy;
                }
                cy = y;
                ensure_bounds(&mut b, cx, cy).include_point(cx, cy);
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                prev_cmd = ucmd;

                loop {
                    let Some(mut ny) = try_parse_number(d, bytes, &mut i) else {
                        break;
                    };
                    if is_rel {
                        ny += cy;
                    }
                    cy = ny;
                    ensure_bounds(&mut b, cx, cy).include_point(cx, cy);
                    prev_cmd = ucmd;
                }
            }
            b'C' => {
                let mut x1 = parse_number(d, bytes, &mut i)?;
                let mut y1 = parse_number(d, bytes, &mut i)?;
                let mut x2 = parse_number(d, bytes, &mut i)?;
                let mut y2 = parse_number(d, bytes, &mut i)?;
                let mut x3 = parse_number(d, bytes, &mut i)?;
                let mut y3 = parse_number(d, bytes, &mut i)?;
                if is_rel {
                    x1 += cx;
                    y1 += cy;
                    x2 += cx;
                    y2 += cy;
                    x3 += cx;
                    y3 += cy;
                }
                let cur = ensure_bounds(&mut b, cx, cy);
                svg_path_bounds_include_cubic(
                    cur,
                    CubicBezier {
                        x0: cx,
                        y0: cy,
                        x1,
                        y1,
                        x2,
                        y2,
                        x3,
                        y3,
                    },
                );
                cx = x3;
                cy = y3;
                last_cubic_ctrl = Some((x2, y2));
                last_quad_ctrl = None;
                prev_cmd = ucmd;

                loop {
                    let save = i;
                    let Some(mut nx1) = try_parse_number(d, bytes, &mut i) else {
                        break;
                    };
                    let Some(mut ny1) = try_parse_number(d, bytes, &mut i) else {
                        i = save;
                        break;
                    };
                    let Some(mut nx2) = try_parse_number(d, bytes, &mut i) else {
                        i = save;
                        break;
                    };
                    let Some(mut ny2) = try_parse_number(d, bytes, &mut i) else {
                        i = save;
                        break;
                    };
                    let Some(mut nx3) = try_parse_number(d, bytes, &mut i) else {
                        i = save;
                        break;
                    };
                    let Some(mut ny3) = try_parse_number(d, bytes, &mut i) else {
                        i = save;
                        break;
                    };
                    if is_rel {
                        nx1 += cx;
                        ny1 += cy;
                        nx2 += cx;
                        ny2 += cy;
                        nx3 += cx;
                        ny3 += cy;
                    }
                    let cur = ensure_bounds(&mut b, cx, cy);
                    svg_path_bounds_include_cubic(
                        cur,
                        CubicBezier {
                            x0: cx,
                            y0: cy,
                            x1: nx1,
                            y1: ny1,
                            x2: nx2,
                            y2: ny2,
                            x3: nx3,
                            y3: ny3,
                        },
                    );
                    cx = nx3;
                    cy = ny3;
                    last_cubic_ctrl = Some((nx2, ny2));
                    last_quad_ctrl = None;
                    prev_cmd = ucmd;
                }
            }
            b'S' => {
                let (x1, y1) = if matches!(prev_cmd, b'C' | b'S') {
                    if let Some((px, py)) = last_cubic_ctrl {
                        (2.0 * cx - px, 2.0 * cy - py)
                    } else {
                        (cx, cy)
                    }
                } else {
                    (cx, cy)
                };

                let mut x2 = parse_number(d, bytes, &mut i)?;
                let mut y2 = parse_number(d, bytes, &mut i)?;
                let mut x3 = parse_number(d, bytes, &mut i)?;
                let mut y3 = parse_number(d, bytes, &mut i)?;
                if is_rel {
                    x2 += cx;
                    y2 += cy;
                    x3 += cx;
                    y3 += cy;
                }
                let cur = ensure_bounds(&mut b, cx, cy);
                svg_path_bounds_include_cubic(
                    cur,
                    CubicBezier {
                        x0: cx,
                        y0: cy,
                        x1,
                        y1,
                        x2,
                        y2,
                        x3,
                        y3,
                    },
                );
                cx = x3;
                cy = y3;
                last_cubic_ctrl = Some((x2, y2));
                last_quad_ctrl = None;
                prev_cmd = ucmd;
            }
            b'Q' => {
                let mut x1 = parse_number(d, bytes, &mut i)?;
                let mut y1 = parse_number(d, bytes, &mut i)?;
                let mut x2 = parse_number(d, bytes, &mut i)?;
                let mut y2 = parse_number(d, bytes, &mut i)?;
                if is_rel {
                    x1 += cx;
                    y1 += cy;
                    x2 += cx;
                    y2 += cy;
                }
                let cur = ensure_bounds(&mut b, cx, cy);
                quadratic_include_bounds(cur, cx, cy, x1, y1, x2, y2);
                cx = x2;
                cy = y2;
                last_quad_ctrl = Some((x1, y1));
                last_cubic_ctrl = None;
                prev_cmd = ucmd;
            }
            b'T' => {
                let (x1, y1) = if matches!(prev_cmd, b'Q' | b'T') {
                    if let Some((qx, qy)) = last_quad_ctrl {
                        (2.0 * cx - qx, 2.0 * cy - qy)
                    } else {
                        (cx, cy)
                    }
                } else {
                    (cx, cy)
                };
                let (mut x2, mut y2) = parse_pair(d, bytes, &mut i)?;
                if is_rel {
                    x2 += cx;
                    y2 += cy;
                }
                let cur = ensure_bounds(&mut b, cx, cy);
                quadratic_include_bounds(cur, cx, cy, x1, y1, x2, y2);
                cx = x2;
                cy = y2;
                last_quad_ctrl = Some((x1, y1));
                last_cubic_ctrl = None;
                prev_cmd = ucmd;
            }
            b'A' => {
                let rx = parse_number(d, bytes, &mut i)?;
                let ry = parse_number(d, bytes, &mut i)?;
                let rot = parse_number(d, bytes, &mut i)?;
                let laf = parse_number(d, bytes, &mut i)?;
                let sf = parse_number(d, bytes, &mut i)?;
                let (mut x1, mut y1) = parse_pair(d, bytes, &mut i)?;
                if is_rel {
                    x1 += cx;
                    y1 += cy;
                }
                let large_arc = laf.abs() > 0.5;
                let sweep = sf.abs() > 0.5;
                if let Some(cur) = b.as_mut() {
                    arc_include_bounds(
                        cur,
                        ArcBoundsInput {
                            x0: cx,
                            y0: cy,
                            rx0: rx,
                            ry0: ry,
                            x_axis_rotation_deg: rot,
                            large_arc,
                            sweep,
                            x1,
                            y1,
                        },
                    );
                } else {
                    let mut cur = SvgPathBounds {
                        min_x: cx,
                        min_y: cy,
                        max_x: cx,
                        max_y: cy,
                    };
                    arc_include_bounds(
                        &mut cur,
                        ArcBoundsInput {
                            x0: cx,
                            y0: cy,
                            rx0: rx,
                            ry0: ry,
                            x_axis_rotation_deg: rot,
                            large_arc,
                            sweep,
                            x1,
                            y1,
                        },
                    );
                    b = Some(cur);
                }
                cx = x1;
                cy = y1;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                prev_cmd = ucmd;
            }
            _ => return None,
        }
    }

    b
}
