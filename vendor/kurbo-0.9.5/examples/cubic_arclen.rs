// Copyright 2018 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Research testbed for arclengths of cubic Bézier segments.

// Lots of stuff is commented out or was just something to try.
#![allow(unused)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::many_single_char_names)]

use kurbo::common::*;
use kurbo::{
    CubicBez, ParamCurve, ParamCurveArclen, ParamCurveCurvature, ParamCurveDeriv, Point, Vec2,
};

/// Calculate arclength using Gauss-Legendre quadrature using formula from Behdad
/// in https://github.com/Pomax/BezierInfo-2/issues/77
fn gauss_arclen_5(c: CubicBez) -> f64 {
    let v0 = (c.p1 - c.p0).hypot() * 0.15;
    let v1 = (-0.558983582205757 * c.p0.to_vec2()
        + 0.325650248872424 * c.p1.to_vec2()
        + 0.208983582205757 * c.p2.to_vec2()
        + 0.024349751127576 * c.p3.to_vec2())
    .hypot();
    let v2 = (c.p3 - c.p0 + (c.p2 - c.p1)).hypot() * 0.26666666666666666;
    let v3 = (-0.024349751127576 * c.p0.to_vec2()
        - 0.208983582205757 * c.p1.to_vec2()
        - 0.325650248872424 * c.p2.to_vec2()
        + 0.558983582205757 * c.p3.to_vec2())
    .hypot();
    let v4 = (c.p3 - c.p2).hypot() * 0.15;

    v0 + v1 + v2 + v3 + v4
}

fn gauss_arclen_7<C: ParamCurveDeriv>(c: C) -> f64 {
    c.gauss_arclen(GAUSS_LEGENDRE_COEFFS_7)
}

fn est_gauss5_error(c: CubicBez) -> f64 {
    let lc = (c.p3 - c.p0).hypot();
    let lp = (c.p1 - c.p0).hypot() + (c.p2 - c.p1).hypot() + (c.p3 - c.p2).hypot();

    let d2 = c.deriv().deriv();
    let d3 = d2.deriv();
    let lmi = 2.0 / (lp + lc);
    7e-8 * (d3.eval(0.5).to_vec2().hypot() * lmi + 5.0 * d2.eval(0.5).to_vec2().hypot() * lmi)
        .powi(5)
        * lp
}

fn gauss_errnorm_n<C: ParamCurveDeriv>(c: C, coeffs: &[(f64, f64)]) -> f64
where
    C::DerivResult: ParamCurveDeriv,
{
    let d = c.deriv().deriv();
    coeffs
        .iter()
        .map(|(wi, xi)| wi * d.eval(0.5 * (xi + 1.0)).to_vec2().hypot2())
        .sum::<f64>()
}

// Squared L2 norm of the second derivative of the cubic.
fn cubic_errnorm(c: CubicBez) -> f64 {
    let d = c.deriv().deriv();
    let dd = d.end() - d.start();
    d.start().to_vec2().hypot2() + d.start().to_vec2().dot(dd) + dd.hypot2() * (1.0 / 3.0)
}

fn est_gauss7_error(c: CubicBez) -> f64 {
    let lc = (c.p3 - c.p0).hypot();
    let lp = (c.p1 - c.p0).hypot() + (c.p2 - c.p1).hypot() + (c.p3 - c.p2).hypot();

    8e-9 * (2.0 * cubic_errnorm(c) / lc.powi(2)).powi(6) * lp
}

fn gauss_arclen_9<C: ParamCurveDeriv>(c: C) -> f64 {
    c.gauss_arclen(GAUSS_LEGENDRE_COEFFS_9)
}

fn gauss_arclen_11<C: ParamCurveDeriv>(c: C) -> f64 {
    c.gauss_arclen(GAUSS_LEGENDRE_COEFFS_11)
}

fn est_gauss9_error(c: CubicBez) -> f64 {
    let lc = (c.p3 - c.p0).hypot();
    let lp = (c.p1 - c.p0).hypot() + (c.p2 - c.p1).hypot() + (c.p3 - c.p2).hypot();

    1e-10 * (2.0 * cubic_errnorm(c) / lc.powi(2)).powi(8) * lp
}

fn est_gauss11_error(c: CubicBez) -> f64 {
    let lc = (c.p3 - c.p0).hypot();
    let lp = (c.p1 - c.p0).hypot() + (c.p2 - c.p1).hypot() + (c.p3 - c.p2).hypot();

    1e-12 * (2.0 * cubic_errnorm(c) / lc.powi(2)).powi(11) * lp
}

// A new approach based on integrating local error.
fn est_gauss11_error_2(c: CubicBez) -> f64 {
    let d = c.deriv();
    let d2 = d.deriv();
    GAUSS_LEGENDRE_COEFFS_11
        .iter()
        .map(|(wi, xi)| {
            wi * {
                let t = 0.5 * (xi + 1.0);
                let v = d.eval(t).to_vec2().hypot();
                let a2 = d2.eval(t).to_vec2().hypot2();
                a2.powi(3) / v.powi(5)
            }
        })
        .sum::<f64>()
}

#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn est_max_curvature(c: CubicBez) -> f64 {
    let n = 100;
    let mut max = 0.0;
    for i in 0..=n {
        let t = (i as f64) * (n as f64).recip();
        let k = c.curvature(t).abs();
        if !(k < max) {
            max = k;
        }
    }
    max
}

fn est_min_deriv_norm2(c: CubicBez) -> f64 {
    let d = c.deriv();
    let n = 100;
    let mut min = d.eval(1.0).to_vec2().hypot2();
    for i in 0..n {
        let t = (i as f64) * (n as f64).recip();
        min = min.min(d.eval(t).to_vec2().hypot2())
    }
    min
}

fn est_gauss11_error_3(c: CubicBez) -> f64 {
    let lc = (c.p3 - c.p0).hypot();
    let lp = (c.p1 - c.p0).hypot() + (c.p2 - c.p1).hypot() + (c.p3 - c.p2).hypot();
    let pc_err = (lp - lc) * 0.02;
    let ks = est_max_curvature(c) * lp;
    let est = ks.powi(3) * lp * 8e-9;
    if est < pc_err {
        est
    } else {
        pc_err
    }
}

fn est_gauss9_error_3(c: CubicBez) -> f64 {
    let lc = (c.p3 - c.p0).hypot();
    let lp = (c.p1 - c.p0).hypot() + (c.p2 - c.p1).hypot() + (c.p3 - c.p2).hypot();
    let pc_err = (lp - lc) * 0.02;
    let ks = est_max_curvature(c) * lp;
    let est = ks.powi(3) * lp * 5e-8;
    if est < pc_err {
        est
    } else {
        pc_err
    }
}

// A new approach based on integrating local error; the cost of evaluating the
// error metric is likely to dominate unless the accuracy buys a lot of subdivisions.
fn est_gauss9_error_2(c: CubicBez) -> f64 {
    let d = c.deriv();
    let d2 = d.deriv();
    let p = 10;
    GAUSS_LEGENDRE_COEFFS_9
        .iter()
        .map(|(wi, xi)| {
            wi * {
                let t = 0.5 * (xi + 1.0);
                let v = d.eval(t).to_vec2().hypot();
                let a = d2.eval(t).to_vec2().hypot();
                (1.0e-1 * a / v).tanh().powi(p) * v
            }
        })
        .sum::<f64>()
        * 3.0
}

fn my_arclen(c: CubicBez, accuracy: f64, depth: usize, count: &mut usize) -> f64 {
    if depth == 16 || est_gauss5_error(c) < accuracy {
        *count += 1;
        gauss_arclen_5(c)
    } else {
        let (c0, c1) = c.subdivide();
        my_arclen(c0, accuracy * 0.5, depth + 1, count)
            + my_arclen(c1, accuracy * 0.5, depth + 1, count)
    }
}

fn my_arclen7(c: CubicBez, accuracy: f64, depth: usize, count: &mut usize) -> f64 {
    if depth == 16 || est_gauss7_error(c) < accuracy {
        *count += 1;
        gauss_arclen_7(c)
    } else {
        let (c0, c1) = c.subdivide();
        my_arclen7(c0, accuracy * 0.5, depth + 1, count)
            + my_arclen7(c1, accuracy * 0.5, depth + 1, count)
    }
}

// Should make this generic instead of copy+paste, but we need only one when we're done.
fn my_arclen9(c: CubicBez, accuracy: f64, depth: usize, count: &mut usize) -> f64 {
    if depth == 16 || est_gauss9_error(c) < accuracy {
        *count += 1;
        gauss_arclen_9(c)
    } else {
        let (c0, c1) = c.subdivide();
        my_arclen9(c0, accuracy * 0.5, depth + 1, count)
            + my_arclen9(c1, accuracy * 0.5, depth + 1, count)
    }
}

// This doesn't help; we can't really get a more accurate error bound, so all this
// does is overkill the accuracy.
fn my_arclen11(c: CubicBez, accuracy: f64, depth: usize, count: &mut usize) -> f64 {
    if depth == 16 || est_gauss9_error(c) < accuracy {
        *count += 1;
        gauss_arclen_11(c)
    } else {
        let (c0, c1) = c.subdivide();
        my_arclen11(c0, accuracy * 0.5, depth + 1, count)
            + my_arclen11(c1, accuracy * 0.5, depth + 1, count)
    }
}

fn randpt() -> Point {
    Point::new(rand::random(), rand::random())
}

fn randbez() -> CubicBez {
    CubicBez::new(randpt(), randpt(), randpt(), randpt())
}

fn main() {
    let accuracy = 1e-4;
    for _ in 0..10_000 {
        let c = randbez();
        let t: f64 = rand::random();
        let c = c.subsegment(0.0..t);
        //let accurate_arclen = c.arclen(1e-12);
        let mut count = 0;
        let accurate_arclen = my_arclen9(c, 1e-15, 0, &mut count);

        let est = gauss_arclen_9(c);
        let est_err = est_gauss9_error_3(c);
        let err = (accurate_arclen - est).abs();
        println!("{est_err} {err}");

        /*
        let mut count = 0;
        let est = my_arclen9(c, accuracy, 0, &mut count);
        let err = (accurate_arclen - est).abs();
        println!("{err} {count}");
        */
    }
}
