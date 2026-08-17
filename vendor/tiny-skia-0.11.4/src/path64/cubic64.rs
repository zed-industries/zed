// Copyright 2012 Google Inc.
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use super::point64::{Point64, SearchAxis};
use super::quad64;
use super::Scalar64;

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use tiny_skia_path::NoStdFloat;

pub const POINT_COUNT: usize = 4;
const PI: f64 = 3.141592653589793;

pub struct Cubic64Pair {
    pub points: [Point64; 7],
}

pub struct Cubic64 {
    pub points: [Point64; POINT_COUNT],
}

impl Cubic64 {
    pub fn new(points: [Point64; POINT_COUNT]) -> Self {
        Cubic64 { points }
    }

    pub fn as_f64_slice(&self) -> [f64; POINT_COUNT * 2] {
        [
            self.points[0].x,
            self.points[0].y,
            self.points[1].x,
            self.points[1].y,
            self.points[2].x,
            self.points[2].y,
            self.points[3].x,
            self.points[3].y,
        ]
    }

    pub fn point_at_t(&self, t: f64) -> Point64 {
        if t == 0.0 {
            return self.points[0];
        }

        if t == 1.0 {
            return self.points[3];
        }

        let one_t = 1.0 - t;
        let one_t2 = one_t * one_t;
        let a = one_t2 * one_t;
        let b = 3.0 * one_t2 * t;
        let t2 = t * t;
        let c = 3.0 * one_t * t2;
        let d = t2 * t;
        Point64::from_xy(
            a * self.points[0].x
                + b * self.points[1].x
                + c * self.points[2].x
                + d * self.points[3].x,
            a * self.points[0].y
                + b * self.points[1].y
                + c * self.points[2].y
                + d * self.points[3].y,
        )
    }

    pub fn search_roots(
        &self,
        mut extrema: usize,
        axis_intercept: f64,
        x_axis: SearchAxis,
        extreme_ts: &mut [f64; 6],
        valid_roots: &mut [f64],
    ) -> usize {
        extrema += self.find_inflections(&mut extreme_ts[extrema..]);
        extreme_ts[extrema] = 0.0;
        extrema += 1;
        extreme_ts[extrema] = 1.0;
        debug_assert!(extrema < 6);
        extreme_ts[0..extrema].sort_by(cmp_f64);
        let mut valid_count = 0;
        let mut index = 0;
        while index < extrema {
            let min = extreme_ts[index];
            index += 1;
            let max = extreme_ts[index];
            if min == max {
                continue;
            }

            let new_t = self.binary_search(min, max, axis_intercept, x_axis);
            if new_t >= 0.0 {
                if valid_count >= 3 {
                    return 0;
                }

                valid_roots[valid_count] = new_t;
                valid_count += 1;
            }
        }

        valid_count
    }

    fn find_inflections(&self, t_values: &mut [f64]) -> usize {
        let ax = self.points[1].x - self.points[0].x;
        let ay = self.points[1].y - self.points[0].y;
        let bx = self.points[2].x - 2.0 * self.points[1].x + self.points[0].x;
        let by = self.points[2].y - 2.0 * self.points[1].y + self.points[0].y;
        let cx = self.points[3].x + 3.0 * (self.points[1].x - self.points[2].x) - self.points[0].x;
        let cy = self.points[3].y + 3.0 * (self.points[1].y - self.points[2].y) - self.points[0].y;
        quad64::roots_valid_t(
            bx * cy - by * cx,
            ax * cy - ay * cx,
            ax * by - ay * bx,
            t_values,
        )
    }

    // give up when changing t no longer moves point
    // also, copy point rather than recompute it when it does change
    fn binary_search(&self, min: f64, max: f64, axis_intercept: f64, x_axis: SearchAxis) -> f64 {
        let mut t = (min + max) / 2.0;
        let mut step = (t - min) / 2.0;
        let mut cubic_at_t = self.point_at_t(t);
        let mut calc_pos = cubic_at_t.axis_coord(x_axis);
        let mut calc_dist = calc_pos - axis_intercept;
        loop {
            let prior_t = min.max(t - step);
            let less_pt = self.point_at_t(prior_t);
            if less_pt.x.approximately_equal_half(cubic_at_t.x)
                && less_pt.y.approximately_equal_half(cubic_at_t.y)
            {
                return -1.0; // binary search found no point at this axis intercept
            }

            let less_dist = less_pt.axis_coord(x_axis) - axis_intercept;
            let last_step = step;
            step /= 2.0;
            let ok = if calc_dist > 0.0 {
                calc_dist > less_dist
            } else {
                calc_dist < less_dist
            };
            if ok {
                t = prior_t;
            } else {
                let next_t = t + last_step;
                if next_t > max {
                    return -1.0;
                }

                let more_pt = self.point_at_t(next_t);
                if more_pt.x.approximately_equal_half(cubic_at_t.x)
                    && more_pt.y.approximately_equal_half(cubic_at_t.y)
                {
                    return -1.0; // binary search found no point at this axis intercept
                }

                let more_dist = more_pt.axis_coord(x_axis) - axis_intercept;
                let ok = if calc_dist > 0.0 {
                    calc_dist <= more_dist
                } else {
                    calc_dist >= more_dist
                };
                if ok {
                    continue;
                }

                t = next_t;
            }

            let test_at_t = self.point_at_t(t);
            cubic_at_t = test_at_t;
            calc_pos = cubic_at_t.axis_coord(x_axis);
            calc_dist = calc_pos - axis_intercept;

            if calc_pos.approximately_equal(axis_intercept) {
                break;
            }
        }

        t
    }

    pub fn chop_at(&self, t: f64) -> Cubic64Pair {
        let mut dst = [Point64::zero(); 7];
        if t == 0.5 {
            dst[0] = self.points[0];
            dst[1].x = (self.points[0].x + self.points[1].x) / 2.0;
            dst[1].y = (self.points[0].y + self.points[1].y) / 2.0;
            dst[2].x = (self.points[0].x + 2.0 * self.points[1].x + self.points[2].x) / 4.0;
            dst[2].y = (self.points[0].y + 2.0 * self.points[1].y + self.points[2].y) / 4.0;
            dst[3].x =
                (self.points[0].x + 3.0 * (self.points[1].x + self.points[2].x) + self.points[3].x)
                    / 8.0;
            dst[3].y =
                (self.points[0].y + 3.0 * (self.points[1].y + self.points[2].y) + self.points[3].y)
                    / 8.0;
            dst[4].x = (self.points[1].x + 2.0 * self.points[2].x + self.points[3].x) / 4.0;
            dst[4].y = (self.points[1].y + 2.0 * self.points[2].y + self.points[3].y) / 4.0;
            dst[5].x = (self.points[2].x + self.points[3].x) / 2.0;
            dst[5].y = (self.points[2].y + self.points[3].y) / 2.0;
            dst[6] = self.points[3];

            Cubic64Pair { points: dst }
        } else {
            interp_cubic_coords_x(&self.points, t, &mut dst);
            interp_cubic_coords_y(&self.points, t, &mut dst);
            Cubic64Pair { points: dst }
        }
    }
}

pub fn coefficients(src: &[f64]) -> (f64, f64, f64, f64) {
    let mut a = src[6]; // d
    let mut b = src[4] * 3.0; // 3*c
    let mut c = src[2] * 3.0; // 3*b
    let d = src[0]; // a
    a -= d - c + b; // A =   -a + 3*b - 3*c + d
    b += 3.0 * d - 2.0 * c; // B =  3*a - 6*b + 3*c
    c -= 3.0 * d; // C = -3*a + 3*b

    (a, b, c, d)
}

// from SkGeometry.cpp (and Numeric Solutions, 5.6)
pub fn roots_valid_t(a: f64, b: f64, c: f64, d: f64, t: &mut [f64; 3]) -> usize {
    let mut s = [0.0; 3];
    let real_roots = roots_real(a, b, c, d, &mut s);
    let mut found_roots = quad64::push_valid_ts(&s, real_roots, t);
    'outer: for index in 0..real_roots {
        let t_value = s[index];
        if !t_value.approximately_one_or_less() && t_value.between(1.0, 1.00005) {
            for idx2 in 0..found_roots {
                if t[idx2].approximately_equal(1.0) {
                    continue 'outer;
                }
            }

            debug_assert!(found_roots < 3);
            t[found_roots] = 1.0;
            found_roots += 1;
        } else if !t_value.approximately_zero_or_more() && t_value.between(-0.00005, 0.0) {
            for idx2 in 0..found_roots {
                if t[idx2].approximately_equal(0.0) {
                    continue 'outer;
                }
            }

            debug_assert!(found_roots < 3);
            t[found_roots] = 0.0;
            found_roots += 1;
        }
    }

    found_roots
}

fn roots_real(a: f64, b: f64, c: f64, d: f64, s: &mut [f64; 3]) -> usize {
    if a.approximately_zero()
        && a.approximately_zero_when_compared_to(b)
        && a.approximately_zero_when_compared_to(c)
        && a.approximately_zero_when_compared_to(d)
    {
        // we're just a quadratic
        return quad64::roots_real(b, c, d, s);
    }

    if d.approximately_zero_when_compared_to(a)
        && d.approximately_zero_when_compared_to(b)
        && d.approximately_zero_when_compared_to(c)
    {
        // 0 is one root
        let mut num = quad64::roots_real(a, b, c, s);
        for i in 0..num {
            if s[i].approximately_zero() {
                return num;
            }
        }

        s[num] = 0.0;
        num += 1;

        return num;
    }

    if (a + b + c + d).approximately_zero() {
        // 1 is one root
        let mut num = quad64::roots_real(a, a + b, -d, s);
        for i in 0..num {
            if s[i].almost_dequal_ulps(1.0) {
                return num;
            }
        }
        s[num] = 1.0;
        num += 1;
        return num;
    }

    let (a, b, c) = {
        let inv_a = 1.0 / a;
        let a = b * inv_a;
        let b = c * inv_a;
        let c = d * inv_a;
        (a, b, c)
    };

    let a2 = a * a;
    let q = (a2 - b * 3.0) / 9.0;
    let r = (2.0 * a2 * a - 9.0 * a * b + 27.0 * c) / 54.0;
    let r2 = r * r;
    let q3 = q * q * q;
    let r2_minus_q3 = r2 - q3;
    let adiv3 = a / 3.0;
    let mut offset = 0;
    if r2_minus_q3 < 0.0 {
        // we have 3 real roots

        // the divide/root can, due to finite precisions, be slightly outside of -1...1
        let theta = (r / q3.sqrt()).bound(-1.0, 1.0).acos();
        let neg2_root_q = -2.0 * q.sqrt();

        let mut rr = neg2_root_q * (theta / 3.0).cos() - adiv3;
        s[offset] = rr;
        offset += 1;

        rr = neg2_root_q * ((theta + 2.0 * PI) / 3.0).cos() - adiv3;
        if !s[0].almost_dequal_ulps(rr) {
            s[offset] = rr;
            offset += 1;
        }

        rr = neg2_root_q * ((theta - 2.0 * PI) / 3.0).cos() - adiv3;
        if !s[0].almost_dequal_ulps(rr) && (offset == 1 || !s[1].almost_dequal_ulps(rr)) {
            s[offset] = rr;
            offset += 1;
        }
    } else {
        // we have 1 real root
        let sqrt_r2_minus_q3 = r2_minus_q3.sqrt();
        let mut a = r.abs() + sqrt_r2_minus_q3;
        a = super::cube_root(a);
        if r > 0.0 {
            a = -a;
        }

        if a != 0.0 {
            a += q / a;
        }

        let mut r2 = a - adiv3;
        s[offset] = r2;
        offset += 1;
        if r2.almost_dequal_ulps(q3) {
            r2 = -a / 2.0 - adiv3;
            if !s[0].almost_dequal_ulps(r2) {
                s[offset] = r2;
                offset += 1;
            }
        }
    }

    offset
}

// Cubic64'(t) = At^2 + Bt + C, where
// A = 3(-a + 3(b - c) + d)
// B = 6(a - 2b + c)
// C = 3(b - a)
// Solve for t, keeping only those that fit between 0 < t < 1
pub fn find_extrema(src: &[f64], t_values: &mut [f64]) -> usize {
    // we divide A,B,C by 3 to simplify
    let a = src[0];
    let b = src[2];
    let c = src[4];
    let d = src[6];
    let a2 = d - a + 3.0 * (b - c);
    let b2 = 2.0 * (a - b - b + c);
    let c2 = b - a;

    quad64::roots_valid_t(a2, b2, c2, t_values)
}

// Skia doesn't seems to care about NaN/inf during sorting, so we don't too.
fn cmp_f64(a: &f64, b: &f64) -> core::cmp::Ordering {
    if a < b {
        core::cmp::Ordering::Less
    } else if a > b {
        core::cmp::Ordering::Greater
    } else {
        core::cmp::Ordering::Equal
    }
}

// classic one t subdivision
fn interp_cubic_coords_x(src: &[Point64; 4], t: f64, dst: &mut [Point64; 7]) {
    use super::interp;

    let ab = interp(src[0].x, src[1].x, t);
    let bc = interp(src[1].x, src[2].x, t);
    let cd = interp(src[2].x, src[3].x, t);
    let abc = interp(ab, bc, t);
    let bcd = interp(bc, cd, t);
    let abcd = interp(abc, bcd, t);

    dst[0].x = src[0].x;
    dst[1].x = ab;
    dst[2].x = abc;
    dst[3].x = abcd;
    dst[4].x = bcd;
    dst[5].x = cd;
    dst[6].x = src[3].x;
}

fn interp_cubic_coords_y(src: &[Point64; 4], t: f64, dst: &mut [Point64; 7]) {
    use super::interp;

    let ab = interp(src[0].y, src[1].y, t);
    let bc = interp(src[1].y, src[2].y, t);
    let cd = interp(src[2].y, src[3].y, t);
    let abc = interp(ab, bc, t);
    let bcd = interp(bc, cd, t);
    let abcd = interp(abc, bcd, t);

    dst[0].y = src[0].y;
    dst[1].y = ab;
    dst[2].y = abc;
    dst[3].y = abcd;
    dst[4].y = bcd;
    dst[5].y = cd;
    dst[6].y = src[3].y;
}
