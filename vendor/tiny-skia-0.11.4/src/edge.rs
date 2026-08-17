// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::Point;

use crate::fixed_point::{fdot16, fdot6, FDot16, FDot6};
use crate::math::left_shift;

/// We store 1<<shift in a (signed) byte, so its maximum value is 1<<6 == 64.
///
/// Note that this limits the number of lines we use to approximate a curve.
/// If we need to increase this, we need to store curve_count in something
/// larger than i8.
const MAX_COEFF_SHIFT: i32 = 6;

#[derive(Clone, Debug)]
pub enum Edge {
    Line(LineEdge),
    Quadratic(QuadraticEdge),
    Cubic(CubicEdge),
}

impl Edge {
    pub fn as_line(&self) -> &LineEdge {
        match self {
            Edge::Line(line) => line,
            Edge::Quadratic(quad) => &quad.line,
            Edge::Cubic(cubic) => &cubic.line,
        }
    }

    pub fn as_line_mut(&mut self) -> &mut LineEdge {
        match self {
            Edge::Line(line) => line,
            Edge::Quadratic(quad) => &mut quad.line,
            Edge::Cubic(cubic) => &mut cubic.line,
        }
    }
}

impl core::ops::Deref for Edge {
    type Target = LineEdge;

    fn deref(&self) -> &Self::Target {
        self.as_line()
    }
}

impl core::ops::DerefMut for Edge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_line_mut()
    }
}

#[derive(Clone, Default, Debug)]
pub struct LineEdge {
    // Imitate a linked list.
    pub prev: Option<u32>,
    pub next: Option<u32>,

    pub x: FDot16,
    pub dx: FDot16,
    pub first_y: i32,
    pub last_y: i32,
    pub winding: i8, // 1 or -1
}

impl LineEdge {
    pub fn new(p0: Point, p1: Point, shift: i32) -> Option<Self> {
        let scale = (1 << (shift + 6)) as f32;
        let mut x0 = (p0.x * scale) as i32;
        let mut y0 = (p0.y * scale) as i32;
        let mut x1 = (p1.x * scale) as i32;
        let mut y1 = (p1.y * scale) as i32;

        let mut winding = 1;

        if y0 > y1 {
            core::mem::swap(&mut x0, &mut x1);
            core::mem::swap(&mut y0, &mut y1);
            winding = -1;
        }

        let top = fdot6::round(y0);
        let bottom = fdot6::round(y1);

        // are we a zero-height line?
        if top == bottom {
            return None;
        }

        let slope = fdot6::div(x1 - x0, y1 - y0);
        let dy = compute_dy(top, y0);

        Some(LineEdge {
            next: None,
            prev: None,
            x: fdot6::to_fdot16(x0 + fdot16::mul(slope, dy)),
            dx: slope,
            first_y: top,
            last_y: bottom - 1,
            winding,
        })
    }

    pub fn is_vertical(&self) -> bool {
        self.dx == 0
    }

    fn update(&mut self, mut x0: FDot16, mut y0: FDot16, mut x1: FDot16, mut y1: FDot16) -> bool {
        debug_assert!(self.winding == 1 || self.winding == -1);

        y0 >>= 10;
        y1 >>= 10;

        debug_assert!(y0 <= y1);

        let top = fdot6::round(y0);
        let bottom = fdot6::round(y1);

        // are we a zero-height line?
        if top == bottom {
            return false;
        }

        x0 >>= 10;
        x1 >>= 10;

        let slope = fdot6::div(x1 - x0, y1 - y0);
        let dy = compute_dy(top, y0);

        self.x = fdot6::to_fdot16(x0 + fdot16::mul(slope, dy));
        self.dx = slope;
        self.first_y = top;
        self.last_y = bottom - 1;

        true
    }
}

#[derive(Clone, Debug)]
pub struct QuadraticEdge {
    pub line: LineEdge,
    pub curve_count: i8,
    curve_shift: u8, // applied to all dx/ddx/dddx
    qx: FDot16,
    qy: FDot16,
    qdx: FDot16,
    qdy: FDot16,
    qddx: FDot16,
    qddy: FDot16,
    q_last_x: FDot16,
    q_last_y: FDot16,
}

impl QuadraticEdge {
    pub fn new(points: &[Point], shift: i32) -> Option<Self> {
        let mut quad = Self::new2(points, shift)?;
        if quad.update() {
            Some(quad)
        } else {
            None
        }
    }

    fn new2(points: &[Point], mut shift: i32) -> Option<Self> {
        let scale = (1 << (shift + 6)) as f32;
        let mut x0 = (points[0].x * scale) as i32;
        let mut y0 = (points[0].y * scale) as i32;
        let x1 = (points[1].x * scale) as i32;
        let y1 = (points[1].y * scale) as i32;
        let mut x2 = (points[2].x * scale) as i32;
        let mut y2 = (points[2].y * scale) as i32;

        let mut winding = 1;
        if y0 > y2 {
            core::mem::swap(&mut x0, &mut x2);
            core::mem::swap(&mut y0, &mut y2);
            winding = -1;
        }
        debug_assert!(y0 <= y1 && y1 <= y2);

        let top = fdot6::round(y0);
        let bottom = fdot6::round(y2);

        // are we a zero-height quad (line)?
        if top == bottom {
            return None;
        }

        // compute number of steps needed (1 << shift)
        {
            let dx = (left_shift(x1, 1) - x0 - x2) >> 2;
            let dy = (left_shift(y1, 1) - y0 - y2) >> 2;
            // This is a little confusing:
            // before this line, shift is the scale up factor for AA;
            // after this line, shift is the fCurveShift.
            shift = diff_to_shift(dx, dy, shift);
            debug_assert!(shift >= 0);
        }

        // need at least 1 subdivision for our bias trick
        if shift == 0 {
            shift = 1;
        } else if shift > MAX_COEFF_SHIFT {
            shift = MAX_COEFF_SHIFT;
        }

        let curve_count = (1 << shift) as i8;

        // We want to reformulate into polynomial form, to make it clear how we
        // should forward-difference.
        //
        // p0 (1 - t)^2 + p1 t(1 - t) + p2 t^2 ==> At^2 + Bt + C
        //
        // A = p0 - 2p1 + p2
        // B = 2(p1 - p0)
        // C = p0
        //
        // Our caller must have constrained our inputs (p0..p2) to all fit into
        // 16.16. However, as seen above, we sometimes compute values that can be
        // larger (e.g. B = 2*(p1 - p0)). To guard against overflow, we will store
        // A and B at 1/2 of their actual value, and just apply a 2x scale during
        // application in updateQuadratic(). Hence we store (shift - 1) in
        // curve_shift.

        let curve_shift = (shift - 1) as u8;

        let mut a = fdot6_to_fixed_div2(x0 - x1 - x1 + x2); // 1/2 the real value
        let mut b = fdot6::to_fdot16(x1 - x0); // 1/2 the real value

        let qx = fdot6::to_fdot16(x0);
        let qdx = b + (a >> shift); // biased by shift
        let qddx = a >> (shift - 1); // biased by shift

        a = fdot6_to_fixed_div2(y0 - y1 - y1 + y2); // 1/2 the real value
        b = fdot6::to_fdot16(y1 - y0); // 1/2 the real value

        let qy = fdot6::to_fdot16(y0);
        let qdy = b + (a >> shift); // biased by shift
        let qddy = a >> (shift - 1); // biased by shift

        let q_last_x = fdot6::to_fdot16(x2);
        let q_last_y = fdot6::to_fdot16(y2);

        Some(QuadraticEdge {
            line: LineEdge {
                next: None,
                prev: None,
                x: 0,
                dx: 0,
                first_y: 0,
                last_y: 0,
                winding,
            },
            curve_count,
            curve_shift,
            qx,
            qy,
            qdx,
            qdy,
            qddx,
            qddy,
            q_last_x,
            q_last_y,
        })
    }

    pub fn update(&mut self) -> bool {
        let mut success;
        let mut count = self.curve_count;
        let mut oldx = self.qx;
        let mut oldy = self.qy;
        let mut dx = self.qdx;
        let mut dy = self.qdy;
        let mut newx;
        let mut newy;
        let shift = self.curve_shift;

        debug_assert!(count > 0);

        loop {
            count -= 1;
            if count > 0 {
                newx = oldx + (dx >> shift);
                dx += self.qddx;
                newy = oldy + (dy >> shift);
                dy += self.qddy;
            } else {
                // last segment
                newx = self.q_last_x;
                newy = self.q_last_y;
            }
            success = self.line.update(oldx, oldy, newx, newy);
            oldx = newx;
            oldy = newy;

            if count == 0 || success {
                break;
            }
        }

        self.qx = newx;
        self.qy = newy;
        self.qdx = dx;
        self.qdy = dy;
        self.curve_count = count;

        success
    }
}

#[derive(Clone, Debug)]
pub struct CubicEdge {
    pub line: LineEdge,
    pub curve_count: i8,
    curve_shift: u8, // applied to all dx/ddx/dddx except for dshift exception
    dshift: u8,      // applied to cdx and cdy
    cx: FDot16,
    cy: FDot16,
    cdx: FDot16,
    cdy: FDot16,
    cddx: FDot16,
    cddy: FDot16,
    cdddx: FDot16,
    cdddy: FDot16,
    c_last_x: FDot16,
    c_last_y: FDot16,
}

impl CubicEdge {
    pub fn new(points: &[Point], shift: i32) -> Option<Self> {
        let mut cubic = Self::new2(points, shift, true)?;
        if cubic.update() {
            Some(cubic)
        } else {
            None
        }
    }

    fn new2(points: &[Point], mut shift: i32, sort_y: bool) -> Option<Self> {
        let scale = (1 << (shift + 6)) as f32;
        let mut x0 = (points[0].x * scale) as i32;
        let mut y0 = (points[0].y * scale) as i32;
        let mut x1 = (points[1].x * scale) as i32;
        let mut y1 = (points[1].y * scale) as i32;
        let mut x2 = (points[2].x * scale) as i32;
        let mut y2 = (points[2].y * scale) as i32;
        let mut x3 = (points[3].x * scale) as i32;
        let mut y3 = (points[3].y * scale) as i32;

        let mut winding = 1;
        if sort_y && y0 > y3 {
            core::mem::swap(&mut x0, &mut x3);
            core::mem::swap(&mut x1, &mut x2);
            core::mem::swap(&mut y0, &mut y3);
            core::mem::swap(&mut y1, &mut y2);
            winding = -1;
        }

        let top = fdot6::round(y0);
        let bot = fdot6::round(y3);

        // are we a zero-height cubic (line)?
        if sort_y && top == bot {
            return None;
        }

        // compute number of steps needed (1 << shift)
        {
            // Can't use (center of curve - center of baseline), since center-of-curve
            // need not be the max delta from the baseline (it could even be coincident)
            // so we try just looking at the two off-curve points
            let dx = cubic_delta_from_line(x0, x1, x2, x3);
            let dy = cubic_delta_from_line(y0, y1, y2, y3);
            // add 1 (by observation)
            shift = diff_to_shift(dx, dy, 2) + 1;
        }
        // need at least 1 subdivision for our bias trick
        debug_assert!(shift > 0);
        if shift > MAX_COEFF_SHIFT {
            shift = MAX_COEFF_SHIFT;
        }

        // Since our in coming data is initially shifted down by 10 (or 8 in
        // antialias). That means the most we can shift up is 8. However, we
        // compute coefficients with a 3*, so the safest upshift is really 6
        let mut up_shift = 6; // largest safe value
        let mut down_shift = shift + up_shift - 10;
        if down_shift < 0 {
            down_shift = 0;
            up_shift = 10 - shift;
        }

        let curve_count = left_shift(-1, shift) as i8;
        let curve_shift = shift as u8;
        let dshift = down_shift as u8;

        let mut b = fdot6_up_shift(3 * (x1 - x0), up_shift);
        let mut c = fdot6_up_shift(3 * (x0 - x1 - x1 + x2), up_shift);
        let mut d = fdot6_up_shift(x3 + 3 * (x1 - x2) - x0, up_shift);

        let cx = fdot6::to_fdot16(x0);
        let cdx = b + (c >> shift) + (d >> (2 * shift)); // biased by shift
        let cddx = 2 * c + ((3 * d) >> (shift - 1)); // biased by 2*shift
        let cdddx = (3 * d) >> (shift - 1); // biased by 2*shift

        b = fdot6_up_shift(3 * (y1 - y0), up_shift);
        c = fdot6_up_shift(3 * (y0 - y1 - y1 + y2), up_shift);
        d = fdot6_up_shift(y3 + 3 * (y1 - y2) - y0, up_shift);

        let cy = fdot6::to_fdot16(y0);
        let cdy = b + (c >> shift) + (d >> (2 * shift)); // biased by shift
        let cddy = 2 * c + ((3 * d) >> (shift - 1)); // biased by 2*shift
        let cdddy = (3 * d) >> (shift - 1); // biased by 2*shift

        let c_last_x = fdot6::to_fdot16(x3);
        let c_last_y = fdot6::to_fdot16(y3);

        Some(CubicEdge {
            line: LineEdge {
                next: None,
                prev: None,
                x: 0,
                dx: 0,
                first_y: 0,
                last_y: 0,
                winding,
            },
            curve_count,
            curve_shift,
            dshift,
            cx,
            cy,
            cdx,
            cdy,
            cddx,
            cddy,
            cdddx,
            cdddy,
            c_last_x,
            c_last_y,
        })
    }

    pub fn update(&mut self) -> bool {
        let mut success;
        let mut count = self.curve_count;
        let mut oldx = self.cx;
        let mut oldy = self.cy;
        let mut newx;
        let mut newy;
        let ddshift = self.curve_shift;
        let dshift = self.dshift;

        debug_assert!(count < 0);

        loop {
            count += 1;
            if count < 0 {
                newx = oldx + (self.cdx >> dshift);
                self.cdx += self.cddx >> ddshift;
                self.cddx += self.cdddx;

                newy = oldy + (self.cdy >> dshift);
                self.cdy += self.cddy >> ddshift;
                self.cddy += self.cdddy;
            } else {
                // last segment
                newx = self.c_last_x;
                newy = self.c_last_y;
            }

            // we want to say debug_assert(oldy <= newy), but our finite fixedpoint
            // doesn't always achieve that, so we have to explicitly pin it here.
            if newy < oldy {
                newy = oldy;
            }

            success = self.line.update(oldx, oldy, newx, newy);
            oldx = newx;
            oldy = newy;

            if count == 0 || success {
                break;
            }
        }

        self.cx = newx;
        self.cy = newy;
        self.curve_count = count;

        success
    }
}

// This correctly favors the lower-pixel when y0 is on a 1/2 pixel boundary
fn compute_dy(top: FDot6, y0: FDot6) -> FDot6 {
    left_shift(top, 6) + 32 - y0
}

fn diff_to_shift(dx: FDot6, dy: FDot6, shift_aa: i32) -> i32 {
    // cheap calc of distance from center of p0-p2 to the center of the curve
    let mut dist = cheap_distance(dx, dy);

    // shift down dist (it is currently in dot6)
    // down by 3 should give us 1/8 pixel accuracy (assuming our dist is accurate...)
    // this is chosen by heuristic: make it as big as possible (to minimize segments)
    // ... but small enough so that our curves still look smooth
    // When shift > 0, we're using AA and everything is scaled up so we can
    // lower the accuracy.
    dist = (dist + (1 << 4)) >> (3 + shift_aa);

    // each subdivision (shift value) cuts this dist (error) by 1/4
    (32 - dist.leading_zeros() as i32) >> 1
}

fn cheap_distance(mut dx: FDot6, mut dy: FDot6) -> FDot6 {
    dx = dx.abs();
    dy = dy.abs();
    // return max + min/2
    if dx > dy {
        dx + (dy >> 1)
    } else {
        dy + (dx >> 1)
    }
}

// In LineEdge::new, QuadraticEdge::new, CubicEdge::new, the first thing we do is to convert
// the points into FDot6. This is modulated by the shift parameter, which
// will either be 0, or something like 2 for antialiasing.
//
// In the float case, we want to turn the float into .6 by saying pt * 64,
// or pt * 256 for antialiasing. This is implemented as 1 << (shift + 6).
//
// In the fixed case, we want to turn the fixed into .6 by saying pt >> 10,
// or pt >> 8 for antialiasing. This is implemented as pt >> (10 - shift).
fn fdot6_to_fixed_div2(value: FDot6) -> FDot16 {
    // we want to return SkFDot6ToFixed(value >> 1), but we don't want to throw
    // away data in value, so just perform a modify up-shift
    left_shift(value, 16 - 6 - 1)
}

fn fdot6_up_shift(x: FDot6, up_shift: i32) -> i32 {
    debug_assert!((left_shift(x, up_shift) >> up_shift) == x);
    left_shift(x, up_shift)
}

// f(1/3) = (8a + 12b + 6c + d) / 27
// f(2/3) = (a + 6b + 12c + 8d) / 27
//
// f(1/3)-b = (8a - 15b + 6c + d) / 27
// f(2/3)-c = (a + 6b - 15c + 8d) / 27
//
// use 16/512 to approximate 1/27
fn cubic_delta_from_line(a: FDot6, b: FDot6, c: FDot6, d: FDot6) -> FDot6 {
    // since our parameters may be negative, we don't use <<
    let one_third = ((a * 8 - b * 15 + 6 * c + d) * 19) >> 9;
    let two_third = ((a + 6 * b - c * 15 + d * 8) * 19) >> 9;

    one_third.abs().max(two_third.abs())
}
