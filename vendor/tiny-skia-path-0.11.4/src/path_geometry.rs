// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! A collection of functions to work with Bezier paths.
//!
//! Mainly for internal use. Do not rely on it!

#![allow(missing_docs)]

use crate::{Point, Transform};

use crate::f32x2_t::f32x2;
use crate::floating_point::FLOAT_PI;
use crate::scalar::{Scalar, SCALAR_NEARLY_ZERO, SCALAR_ROOT_2_OVER_2};

use crate::floating_point::{NormalizedF32, NormalizedF32Exclusive};
use crate::path_builder::PathDirection;

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use crate::NoStdFloat;

// use for : eval(t) == A * t^2 + B * t + C
#[derive(Clone, Copy, Default, Debug)]
pub struct QuadCoeff {
    pub a: f32x2,
    pub b: f32x2,
    pub c: f32x2,
}

impl QuadCoeff {
    pub fn from_points(points: &[Point; 3]) -> Self {
        let c = points[0].to_f32x2();
        let p1 = points[1].to_f32x2();
        let p2 = points[2].to_f32x2();
        let b = times_2(p1 - c);
        let a = p2 - times_2(p1) + c;

        QuadCoeff { a, b, c }
    }

    pub fn eval(&self, t: f32x2) -> f32x2 {
        (self.a * t + self.b) * t + self.c
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct CubicCoeff {
    pub a: f32x2,
    pub b: f32x2,
    pub c: f32x2,
    pub d: f32x2,
}

impl CubicCoeff {
    pub fn from_points(points: &[Point; 4]) -> Self {
        let p0 = points[0].to_f32x2();
        let p1 = points[1].to_f32x2();
        let p2 = points[2].to_f32x2();
        let p3 = points[3].to_f32x2();
        let three = f32x2::splat(3.0);

        CubicCoeff {
            a: p3 + three * (p1 - p2) - p0,
            b: three * (p2 - times_2(p1) + p0),
            c: three * (p1 - p0),
            d: p0,
        }
    }

    pub fn eval(&self, t: f32x2) -> f32x2 {
        ((self.a * t + self.b) * t + self.c) * t + self.d
    }
}

// TODO: to a custom type?
pub fn new_t_values() -> [NormalizedF32Exclusive; 3] {
    [NormalizedF32Exclusive::ANY; 3]
}

pub fn chop_quad_at(src: &[Point], t: NormalizedF32Exclusive, dst: &mut [Point; 5]) {
    let p0 = src[0].to_f32x2();
    let p1 = src[1].to_f32x2();
    let p2 = src[2].to_f32x2();
    let tt = f32x2::splat(t.get());

    let p01 = interp(p0, p1, tt);
    let p12 = interp(p1, p2, tt);

    dst[0] = Point::from_f32x2(p0);
    dst[1] = Point::from_f32x2(p01);
    dst[2] = Point::from_f32x2(interp(p01, p12, tt));
    dst[3] = Point::from_f32x2(p12);
    dst[4] = Point::from_f32x2(p2);
}

// From Numerical Recipes in C.
//
// Q = -1/2 (B + sign(B) sqrt[B*B - 4*A*C])
// x1 = Q / A
// x2 = C / Q
pub fn find_unit_quad_roots(
    a: f32,
    b: f32,
    c: f32,
    roots: &mut [NormalizedF32Exclusive; 3],
) -> usize {
    if a == 0.0 {
        if let Some(r) = valid_unit_divide(-c, b) {
            roots[0] = r;
            return 1;
        } else {
            return 0;
        }
    }

    // use doubles so we don't overflow temporarily trying to compute R
    let mut dr = f64::from(b) * f64::from(b) - 4.0 * f64::from(a) * f64::from(c);
    if dr < 0.0 {
        return 0;
    }
    dr = dr.sqrt();
    let r = dr as f32;
    if !r.is_finite() {
        return 0;
    }

    let q = if b < 0.0 {
        -(b - r) / 2.0
    } else {
        -(b + r) / 2.0
    };

    let mut roots_offset = 0;
    if let Some(r) = valid_unit_divide(q, a) {
        roots[roots_offset] = r;
        roots_offset += 1;
    }

    if let Some(r) = valid_unit_divide(c, q) {
        roots[roots_offset] = r;
        roots_offset += 1;
    }

    if roots_offset == 2 {
        if roots[0].get() > roots[1].get() {
            roots.swap(0, 1);
        } else if roots[0] == roots[1] {
            // nearly-equal?
            roots_offset -= 1; // skip the double root
        }
    }

    roots_offset
}

pub fn chop_cubic_at2(src: &[Point; 4], t: NormalizedF32Exclusive, dst: &mut [Point]) {
    let p0 = src[0].to_f32x2();
    let p1 = src[1].to_f32x2();
    let p2 = src[2].to_f32x2();
    let p3 = src[3].to_f32x2();
    let tt = f32x2::splat(t.get());

    let ab = interp(p0, p1, tt);
    let bc = interp(p1, p2, tt);
    let cd = interp(p2, p3, tt);
    let abc = interp(ab, bc, tt);
    let bcd = interp(bc, cd, tt);
    let abcd = interp(abc, bcd, tt);

    dst[0] = Point::from_f32x2(p0);
    dst[1] = Point::from_f32x2(ab);
    dst[2] = Point::from_f32x2(abc);
    dst[3] = Point::from_f32x2(abcd);
    dst[4] = Point::from_f32x2(bcd);
    dst[5] = Point::from_f32x2(cd);
    dst[6] = Point::from_f32x2(p3);
}

// Quad'(t) = At + B, where
// A = 2(a - 2b + c)
// B = 2(b - a)
// Solve for t, only if it fits between 0 < t < 1
pub(crate) fn find_quad_extrema(a: f32, b: f32, c: f32) -> Option<NormalizedF32Exclusive> {
    // At + B == 0
    // t = -B / A
    valid_unit_divide(a - b, a - b - b + c)
}

pub fn valid_unit_divide(mut numer: f32, mut denom: f32) -> Option<NormalizedF32Exclusive> {
    if numer < 0.0 {
        numer = -numer;
        denom = -denom;
    }

    if denom == 0.0 || numer == 0.0 || numer >= denom {
        return None;
    }

    let r = numer / denom;
    NormalizedF32Exclusive::new(r)
}

fn interp(v0: f32x2, v1: f32x2, t: f32x2) -> f32x2 {
    v0 + (v1 - v0) * t
}

fn times_2(value: f32x2) -> f32x2 {
    value + value
}

// F(t)    = a (1 - t) ^ 2 + 2 b t (1 - t) + c t ^ 2
// F'(t)   = 2 (b - a) + 2 (a - 2b + c) t
// F''(t)  = 2 (a - 2b + c)
//
// A = 2 (b - a)
// B = 2 (a - 2b + c)
//
// Maximum curvature for a quadratic means solving
// Fx' Fx'' + Fy' Fy'' = 0
//
// t = - (Ax Bx + Ay By) / (Bx ^ 2 + By ^ 2)
pub(crate) fn find_quad_max_curvature(src: &[Point; 3]) -> NormalizedF32 {
    let ax = src[1].x - src[0].x;
    let ay = src[1].y - src[0].y;
    let bx = src[0].x - src[1].x - src[1].x + src[2].x;
    let by = src[0].y - src[1].y - src[1].y + src[2].y;

    let mut numer = -(ax * bx + ay * by);
    let mut denom = bx * bx + by * by;
    if denom < 0.0 {
        numer = -numer;
        denom = -denom;
    }

    if numer <= 0.0 {
        return NormalizedF32::ZERO;
    }

    if numer >= denom {
        // Also catches denom=0
        return NormalizedF32::ONE;
    }

    let t = numer / denom;
    NormalizedF32::new(t).unwrap()
}

pub(crate) fn eval_quad_at(src: &[Point; 3], t: NormalizedF32) -> Point {
    Point::from_f32x2(QuadCoeff::from_points(src).eval(f32x2::splat(t.get())))
}

pub(crate) fn eval_quad_tangent_at(src: &[Point; 3], tol: NormalizedF32) -> Point {
    // The derivative equation is 2(b - a +(a - 2b +c)t). This returns a
    // zero tangent vector when t is 0 or 1, and the control point is equal
    // to the end point. In this case, use the quad end points to compute the tangent.
    if (tol == NormalizedF32::ZERO && src[0] == src[1])
        || (tol == NormalizedF32::ONE && src[1] == src[2])
    {
        return src[2] - src[0];
    }

    let p0 = src[0].to_f32x2();
    let p1 = src[1].to_f32x2();
    let p2 = src[2].to_f32x2();

    let b = p1 - p0;
    let a = p2 - p1 - b;
    let t = a * f32x2::splat(tol.get()) + b;

    Point::from_f32x2(t + t)
}

// Looking for F' dot F'' == 0
//
// A = b - a
// B = c - 2b + a
// C = d - 3c + 3b - a
//
// F' = 3Ct^2 + 6Bt + 3A
// F'' = 6Ct + 6B
//
// F' dot F'' -> CCt^3 + 3BCt^2 + (2BB + CA)t + AB
pub fn find_cubic_max_curvature<'a>(
    src: &[Point; 4],
    t_values: &'a mut [NormalizedF32; 3],
) -> &'a [NormalizedF32] {
    let mut coeff_x = formulate_f1_dot_f2(&[src[0].x, src[1].x, src[2].x, src[3].x]);
    let coeff_y = formulate_f1_dot_f2(&[src[0].y, src[1].y, src[2].y, src[3].y]);

    for i in 0..4 {
        coeff_x[i] += coeff_y[i];
    }

    let len = solve_cubic_poly(&coeff_x, t_values);
    &t_values[0..len]
}

// Looking for F' dot F'' == 0
//
// A = b - a
// B = c - 2b + a
// C = d - 3c + 3b - a
//
// F' = 3Ct^2 + 6Bt + 3A
// F'' = 6Ct + 6B
//
// F' dot F'' -> CCt^3 + 3BCt^2 + (2BB + CA)t + AB
fn formulate_f1_dot_f2(src: &[f32; 4]) -> [f32; 4] {
    let a = src[1] - src[0];
    let b = src[2] - 2.0 * src[1] + src[0];
    let c = src[3] + 3.0 * (src[1] - src[2]) - src[0];

    [c * c, 3.0 * b * c, 2.0 * b * b + c * a, a * b]
}

/// Solve coeff(t) == 0, returning the number of roots that lie withing 0 < t < 1.
/// coeff[0]t^3 + coeff[1]t^2 + coeff[2]t + coeff[3]
///
/// Eliminates repeated roots (so that all t_values are distinct, and are always
/// in increasing order.
fn solve_cubic_poly(coeff: &[f32; 4], t_values: &mut [NormalizedF32; 3]) -> usize {
    if coeff[0].is_nearly_zero() {
        // we're just a quadratic
        let mut tmp_t = new_t_values();
        let count = find_unit_quad_roots(coeff[1], coeff[2], coeff[3], &mut tmp_t);
        for i in 0..count {
            t_values[i] = tmp_t[i].to_normalized();
        }

        return count;
    }

    debug_assert!(coeff[0] != 0.0);

    let inva = coeff[0].invert();
    let a = coeff[1] * inva;
    let b = coeff[2] * inva;
    let c = coeff[3] * inva;

    let q = (a * a - b * 3.0) / 9.0;
    let r = (2.0 * a * a * a - 9.0 * a * b + 27.0 * c) / 54.0;

    let q3 = q * q * q;
    let r2_minus_q3 = r * r - q3;
    let adiv3 = a / 3.0;

    if r2_minus_q3 < 0.0 {
        // we have 3 real roots
        // the divide/root can, due to finite precisions, be slightly outside of -1...1
        let theta = (r / q3.sqrt()).bound(-1.0, 1.0).acos();
        let neg2_root_q = -2.0 * q.sqrt();

        t_values[0] = NormalizedF32::new_clamped(neg2_root_q * (theta / 3.0).cos() - adiv3);
        t_values[1] = NormalizedF32::new_clamped(
            neg2_root_q * ((theta + 2.0 * FLOAT_PI) / 3.0).cos() - adiv3,
        );
        t_values[2] = NormalizedF32::new_clamped(
            neg2_root_q * ((theta - 2.0 * FLOAT_PI) / 3.0).cos() - adiv3,
        );

        // now sort the roots
        sort_array3(t_values);
        collapse_duplicates3(t_values)
    } else {
        // we have 1 real root
        let mut a = r.abs() + r2_minus_q3.sqrt();
        a = scalar_cube_root(a);
        if r > 0.0 {
            a = -a;
        }

        if a != 0.0 {
            a += q / a;
        }

        t_values[0] = NormalizedF32::new_clamped(a - adiv3);
        1
    }
}

fn sort_array3(array: &mut [NormalizedF32; 3]) {
    if array[0] > array[1] {
        array.swap(0, 1);
    }

    if array[1] > array[2] {
        array.swap(1, 2);
    }

    if array[0] > array[1] {
        array.swap(0, 1);
    }
}

fn collapse_duplicates3(array: &mut [NormalizedF32; 3]) -> usize {
    let mut len = 3;

    if array[1] == array[2] {
        len = 2;
    }

    if array[0] == array[1] {
        len = 1;
    }

    len
}

fn scalar_cube_root(x: f32) -> f32 {
    x.powf(0.3333333)
}

// This is SkEvalCubicAt split into three functions.
pub(crate) fn eval_cubic_pos_at(src: &[Point; 4], t: NormalizedF32) -> Point {
    Point::from_f32x2(CubicCoeff::from_points(src).eval(f32x2::splat(t.get())))
}

// This is SkEvalCubicAt split into three functions.
pub(crate) fn eval_cubic_tangent_at(src: &[Point; 4], t: NormalizedF32) -> Point {
    // The derivative equation returns a zero tangent vector when t is 0 or 1, and the
    // adjacent control point is equal to the end point. In this case, use the
    // next control point or the end points to compute the tangent.
    if (t.get() == 0.0 && src[0] == src[1]) || (t.get() == 1.0 && src[2] == src[3]) {
        let mut tangent = if t.get() == 0.0 {
            src[2] - src[0]
        } else {
            src[3] - src[1]
        };

        if tangent.x == 0.0 && tangent.y == 0.0 {
            tangent = src[3] - src[0];
        }

        tangent
    } else {
        eval_cubic_derivative(src, t)
    }
}

fn eval_cubic_derivative(src: &[Point; 4], t: NormalizedF32) -> Point {
    let p0 = src[0].to_f32x2();
    let p1 = src[1].to_f32x2();
    let p2 = src[2].to_f32x2();
    let p3 = src[3].to_f32x2();

    let coeff = QuadCoeff {
        a: p3 + f32x2::splat(3.0) * (p1 - p2) - p0,
        b: times_2(p2 - times_2(p1) + p0),
        c: p1 - p0,
    };

    Point::from_f32x2(coeff.eval(f32x2::splat(t.get())))
}

// Cubic'(t) = At^2 + Bt + C, where
// A = 3(-a + 3(b - c) + d)
// B = 6(a - 2b + c)
// C = 3(b - a)
// Solve for t, keeping only those that fit between 0 < t < 1
pub(crate) fn find_cubic_extrema(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    t_values: &mut [NormalizedF32Exclusive; 3],
) -> usize {
    // we divide A,B,C by 3 to simplify
    let aa = d - a + 3.0 * (b - c);
    let bb = 2.0 * (a - b - b + c);
    let cc = b - a;

    find_unit_quad_roots(aa, bb, cc, t_values)
}

// http://www.faculty.idc.ac.il/arik/quality/appendixA.html
//
// Inflection means that curvature is zero.
// Curvature is [F' x F''] / [F'^3]
// So we solve F'x X F''y - F'y X F''y == 0
// After some canceling of the cubic term, we get
// A = b - a
// B = c - 2b + a
// C = d - 3c + 3b - a
// (BxCy - ByCx)t^2 + (AxCy - AyCx)t + AxBy - AyBx == 0
pub(crate) fn find_cubic_inflections<'a>(
    src: &[Point; 4],
    t_values: &'a mut [NormalizedF32Exclusive; 3],
) -> &'a [NormalizedF32Exclusive] {
    let ax = src[1].x - src[0].x;
    let ay = src[1].y - src[0].y;
    let bx = src[2].x - 2.0 * src[1].x + src[0].x;
    let by = src[2].y - 2.0 * src[1].y + src[0].y;
    let cx = src[3].x + 3.0 * (src[1].x - src[2].x) - src[0].x;
    let cy = src[3].y + 3.0 * (src[1].y - src[2].y) - src[0].y;

    let len = find_unit_quad_roots(
        bx * cy - by * cx,
        ax * cy - ay * cx,
        ax * by - ay * bx,
        t_values,
    );

    &t_values[0..len]
}

// Return location (in t) of cubic cusp, if there is one.
// Note that classify cubic code does not reliably return all cusp'd cubics, so
// it is not called here.
pub(crate) fn find_cubic_cusp(src: &[Point; 4]) -> Option<NormalizedF32Exclusive> {
    // When the adjacent control point matches the end point, it behaves as if
    // the cubic has a cusp: there's a point of max curvature where the derivative
    // goes to zero. Ideally, this would be where t is zero or one, but math
    // error makes not so. It is not uncommon to create cubics this way; skip them.
    if src[0] == src[1] {
        return None;
    }

    if src[2] == src[3] {
        return None;
    }

    // Cubics only have a cusp if the line segments formed by the control and end points cross.
    // Detect crossing if line ends are on opposite sides of plane formed by the other line.
    if on_same_side(src, 0, 2) || on_same_side(src, 2, 0) {
        return None;
    }

    // Cubics may have multiple points of maximum curvature, although at most only
    // one is a cusp.
    let mut t_values = [NormalizedF32::ZERO; 3];
    let max_curvature = find_cubic_max_curvature(src, &mut t_values);
    for test_t in max_curvature {
        if 0.0 >= test_t.get() || test_t.get() >= 1.0 {
            // no need to consider max curvature on the end
            continue;
        }

        // A cusp is at the max curvature, and also has a derivative close to zero.
        // Choose the 'close to zero' meaning by comparing the derivative length
        // with the overall cubic size.
        let d_pt = eval_cubic_derivative(src, *test_t);
        let d_pt_magnitude = d_pt.length_sqd();
        let precision = calc_cubic_precision(src);
        if d_pt_magnitude < precision {
            // All three max curvature t values may be close to the cusp;
            // return the first one.
            return Some(NormalizedF32Exclusive::new_bounded(test_t.get()));
        }
    }

    None
}

// Returns true if both points src[testIndex], src[testIndex+1] are in the same half plane defined
// by the line segment src[lineIndex], src[lineIndex+1].
fn on_same_side(src: &[Point; 4], test_index: usize, line_index: usize) -> bool {
    let origin = src[line_index];
    let line = src[line_index + 1] - origin;
    let mut crosses = [0.0, 0.0];
    for index in 0..2 {
        let test_line = src[test_index + index] - origin;
        crosses[index] = line.cross(test_line);
    }

    crosses[0] * crosses[1] >= 0.0
}

// Returns a constant proportional to the dimensions of the cubic.
// Constant found through experimentation -- maybe there's a better way....
fn calc_cubic_precision(src: &[Point; 4]) -> f32 {
    (src[1].distance_to_sqd(src[0])
        + src[2].distance_to_sqd(src[1])
        + src[3].distance_to_sqd(src[2]))
        * 1e-8
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Conic {
    pub points: [Point; 3],
    pub weight: f32,
}

impl Conic {
    pub fn new(pt0: Point, pt1: Point, pt2: Point, weight: f32) -> Self {
        Conic {
            points: [pt0, pt1, pt2],
            weight,
        }
    }

    pub fn from_points(points: &[Point], weight: f32) -> Self {
        Conic {
            points: [points[0], points[1], points[2]],
            weight,
        }
    }

    fn compute_quad_pow2(&self, tolerance: f32) -> Option<u8> {
        if tolerance < 0.0 || !tolerance.is_finite() {
            return None;
        }

        if !self.points[0].is_finite() || !self.points[1].is_finite() || !self.points[2].is_finite()
        {
            return None;
        }

        // Limit the number of suggested quads to approximate a conic
        const MAX_CONIC_TO_QUAD_POW2: usize = 4;

        // "High order approximation of conic sections by quadratic splines"
        // by Michael Floater, 1993
        let a = self.weight - 1.0;
        let k = a / (4.0 * (2.0 + a));
        let x = k * (self.points[0].x - 2.0 * self.points[1].x + self.points[2].x);
        let y = k * (self.points[0].y - 2.0 * self.points[1].y + self.points[2].y);

        let mut error = (x * x + y * y).sqrt();
        let mut pow2 = 0;
        for _ in 0..MAX_CONIC_TO_QUAD_POW2 {
            if error <= tolerance {
                break;
            }

            error *= 0.25;
            pow2 += 1;
        }

        // Unlike Skia, we always expect `pow2` to be at least 1.
        // Otherwise it produces ugly results.
        Some(pow2.max(1))
    }

    // Chop this conic into N quads, stored continuously in pts[], where
    // N = 1 << pow2. The amount of storage needed is (1 + 2 * N)
    pub fn chop_into_quads_pow2(&self, pow2: u8, points: &mut [Point]) -> u8 {
        debug_assert!(pow2 < 5);

        points[0] = self.points[0];
        subdivide(self, &mut points[1..], pow2);

        let quad_count = 1 << pow2;
        let pt_count = 2 * quad_count + 1;
        if points.iter().take(pt_count).any(|n| !n.is_finite()) {
            // if we generated a non-finite, pin ourselves to the middle of the hull,
            // as our first and last are already on the first/last pts of the hull.
            for p in points.iter_mut().take(pt_count - 1).skip(1) {
                *p = self.points[1];
            }
        }

        1 << pow2
    }

    fn chop(&self) -> (Conic, Conic) {
        let scale = f32x2::splat((1.0 + self.weight).invert());
        let new_w = subdivide_weight_value(self.weight);

        let p0 = self.points[0].to_f32x2();
        let p1 = self.points[1].to_f32x2();
        let p2 = self.points[2].to_f32x2();
        let ww = f32x2::splat(self.weight);

        let wp1 = ww * p1;
        let m = (p0 + times_2(wp1) + p2) * scale * f32x2::splat(0.5);
        let mut m_pt = Point::from_f32x2(m);
        if !m_pt.is_finite() {
            let w_d = self.weight as f64;
            let w_2 = w_d * 2.0;
            let scale_half = 1.0 / (1.0 + w_d) * 0.5;
            m_pt.x = ((self.points[0].x as f64
                + w_2 * self.points[1].x as f64
                + self.points[2].x as f64)
                * scale_half) as f32;

            m_pt.y = ((self.points[0].y as f64
                + w_2 * self.points[1].y as f64
                + self.points[2].y as f64)
                * scale_half) as f32;
        }

        (
            Conic {
                points: [self.points[0], Point::from_f32x2((p0 + wp1) * scale), m_pt],
                weight: new_w,
            },
            Conic {
                points: [m_pt, Point::from_f32x2((wp1 + p2) * scale), self.points[2]],
                weight: new_w,
            },
        )
    }

    pub fn build_unit_arc(
        u_start: Point,
        u_stop: Point,
        dir: PathDirection,
        user_transform: Transform,
        dst: &mut [Conic; 5],
    ) -> Option<&[Conic]> {
        // rotate by x,y so that u_start is (1.0)
        let x = u_start.dot(u_stop);
        let mut y = u_start.cross(u_stop);

        let abs_y = y.abs();

        // check for (effectively) coincident vectors
        // this can happen if our angle is nearly 0 or nearly 180 (y == 0)
        // ... we use the dot-prod to distinguish between 0 and 180 (x > 0)
        if abs_y <= SCALAR_NEARLY_ZERO
            && x > 0.0
            && ((y >= 0.0 && dir == PathDirection::CW) || (y <= 0.0 && dir == PathDirection::CCW))
        {
            return None;
        }

        if dir == PathDirection::CCW {
            y = -y;
        }

        // We decide to use 1-conic per quadrant of a circle. What quadrant does [xy] lie in?
        //      0 == [0  .. 90)
        //      1 == [90 ..180)
        //      2 == [180..270)
        //      3 == [270..360)
        //
        let mut quadrant = 0;
        if y == 0.0 {
            quadrant = 2; // 180
            debug_assert!((x + 1.0) <= SCALAR_NEARLY_ZERO);
        } else if x == 0.0 {
            debug_assert!(abs_y - 1.0 <= SCALAR_NEARLY_ZERO);
            quadrant = if y > 0.0 { 1 } else { 3 }; // 90 / 270
        } else {
            if y < 0.0 {
                quadrant += 2;
            }

            if (x < 0.0) != (y < 0.0) {
                quadrant += 1;
            }
        }

        let quadrant_points = [
            Point::from_xy(1.0, 0.0),
            Point::from_xy(1.0, 1.0),
            Point::from_xy(0.0, 1.0),
            Point::from_xy(-1.0, 1.0),
            Point::from_xy(-1.0, 0.0),
            Point::from_xy(-1.0, -1.0),
            Point::from_xy(0.0, -1.0),
            Point::from_xy(1.0, -1.0),
        ];

        const QUADRANT_WEIGHT: f32 = SCALAR_ROOT_2_OVER_2;

        let mut conic_count = quadrant;
        for i in 0..conic_count {
            dst[i] = Conic::from_points(&quadrant_points[i * 2..], QUADRANT_WEIGHT);
        }

        // Now compute any remaing (sub-90-degree) arc for the last conic
        let final_pt = Point::from_xy(x, y);
        let last_q = quadrant_points[quadrant * 2]; // will already be a unit-vector
        let dot = last_q.dot(final_pt);
        debug_assert!(0.0 <= dot && dot <= 1.0 + SCALAR_NEARLY_ZERO);

        if dot < 1.0 {
            let mut off_curve = Point::from_xy(last_q.x + x, last_q.y + y);
            // compute the bisector vector, and then rescale to be the off-curve point.
            // we compute its length from cos(theta/2) = length / 1, using half-angle identity we get
            // length = sqrt(2 / (1 + cos(theta)). We already have cos() when to computed the dot.
            // This is nice, since our computed weight is cos(theta/2) as well!
            let cos_theta_over_2 = ((1.0 + dot) / 2.0).sqrt();
            off_curve.set_length(cos_theta_over_2.invert());
            if !last_q.almost_equal(off_curve) {
                dst[conic_count] = Conic::new(last_q, off_curve, final_pt, cos_theta_over_2);
                conic_count += 1;
            }
        }

        // now handle counter-clockwise and the initial unitStart rotation
        let mut transform = Transform::from_sin_cos(u_start.y, u_start.x);
        if dir == PathDirection::CCW {
            transform = transform.pre_scale(1.0, -1.0);
        }

        transform = transform.post_concat(user_transform);

        for conic in dst.iter_mut().take(conic_count) {
            transform.map_points(&mut conic.points);
        }

        if conic_count == 0 {
            None
        } else {
            Some(&dst[0..conic_count])
        }
    }
}

fn subdivide_weight_value(w: f32) -> f32 {
    (0.5 + w * 0.5).sqrt()
}

fn subdivide<'a>(src: &Conic, mut points: &'a mut [Point], mut level: u8) -> &'a mut [Point] {
    if level == 0 {
        points[0] = src.points[1];
        points[1] = src.points[2];
        &mut points[2..]
    } else {
        let mut dst = src.chop();

        let start_y = src.points[0].y;
        let end_y = src.points[2].y;
        if between(start_y, src.points[1].y, end_y) {
            // If the input is monotonic and the output is not, the scan converter hangs.
            // Ensure that the chopped conics maintain their y-order.
            let mid_y = dst.0.points[2].y;
            if !between(start_y, mid_y, end_y) {
                // If the computed midpoint is outside the ends, move it to the closer one.
                let closer_y = if (mid_y - start_y).abs() < (mid_y - end_y).abs() {
                    start_y
                } else {
                    end_y
                };
                dst.0.points[2].y = closer_y;
                dst.1.points[0].y = closer_y;
            }

            if !between(start_y, dst.0.points[1].y, dst.0.points[2].y) {
                // If the 1st control is not between the start and end, put it at the start.
                // This also reduces the quad to a line.
                dst.0.points[1].y = start_y;
            }

            if !between(dst.1.points[0].y, dst.1.points[1].y, end_y) {
                // If the 2nd control is not between the start and end, put it at the end.
                // This also reduces the quad to a line.
                dst.1.points[1].y = end_y;
            }

            // Verify that all five points are in order.
            debug_assert!(between(start_y, dst.0.points[1].y, dst.0.points[2].y));
            debug_assert!(between(
                dst.0.points[1].y,
                dst.0.points[2].y,
                dst.1.points[1].y
            ));
            debug_assert!(between(dst.0.points[2].y, dst.1.points[1].y, end_y));
        }

        level -= 1;
        points = subdivide(&dst.0, points, level);
        subdivide(&dst.1, points, level)
    }
}

// This was originally developed and tested for pathops: see SkOpTypes.h
// returns true if (a <= b <= c) || (a >= b >= c)
fn between(a: f32, b: f32, c: f32) -> bool {
    (a - b) * (c - b) <= 0.0
}

pub(crate) struct AutoConicToQuads {
    pub points: [Point; 64],
    pub len: u8, // the number of quads
}

impl AutoConicToQuads {
    pub fn compute(pt0: Point, pt1: Point, pt2: Point, weight: f32) -> Option<Self> {
        let conic = Conic::new(pt0, pt1, pt2, weight);
        let pow2 = conic.compute_quad_pow2(0.25)?;
        let mut points = [Point::zero(); 64];
        let len = conic.chop_into_quads_pow2(pow2, &mut points);
        Some(AutoConicToQuads { points, len })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_cubic_at_1() {
        let src = [
            Point::from_xy(30.0, 40.0),
            Point::from_xy(30.0, 40.0),
            Point::from_xy(171.0, 45.0),
            Point::from_xy(180.0, 155.0),
        ];

        assert_eq!(
            eval_cubic_pos_at(&src, NormalizedF32::ZERO),
            Point::from_xy(30.0, 40.0)
        );
        assert_eq!(
            eval_cubic_tangent_at(&src, NormalizedF32::ZERO),
            Point::from_xy(141.0, 5.0)
        );
    }

    #[test]
    fn find_cubic_max_curvature_1() {
        let src = [
            Point::from_xy(20.0, 160.0),
            Point::from_xy(20.0001, 160.0),
            Point::from_xy(160.0, 20.0),
            Point::from_xy(160.0001, 20.0),
        ];

        let mut t_values = [NormalizedF32::ZERO; 3];
        let t_values = find_cubic_max_curvature(&src, &mut t_values);

        assert_eq!(
            &t_values,
            &[
                NormalizedF32::ZERO,
                NormalizedF32::new_clamped(0.5),
                NormalizedF32::ONE,
            ]
        );
    }
}
