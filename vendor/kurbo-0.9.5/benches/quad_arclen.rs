// Copyright 2018 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Benchmarks of quadratic arclength approaches.

// TODO: organize so there's less cut'n'paste from arclen_accuracy example.

#![cfg(nightly)]
#![feature(test)]
extern crate test;
use test::Bencher;

use kurbo::{ParamCurve, ParamCurveArclen, ParamCurveDeriv, QuadBez};

// Based on http://www.malczak.linuxpl.com/blog/quadratic-bezier-curve-length/
fn quad_arclen_analytical(q: QuadBez) -> f64 {
    let d2 = q.p0.to_vec2() - 2.0 * q.p1.to_vec2() + q.p2.to_vec2();
    let a = d2.hypot2();
    let d1 = q.p1 - q.p0;
    let b = 2.0 * d2.dot(d1);
    let c = d1.hypot2();

    let sabc = (a + b + c).sqrt();
    let a2 = a.powf(-0.5);
    let a32 = a2.powi(3);
    let c2 = 2.0 * c.sqrt();
    let ba = b * a2;

    sabc + 0.25
        * (a2 * a2 * b * (2.0 * sabc - c2)
            + a32 * (4.0 * c * a - b * b) * (((2.0 * a + b) * a2 + 2.0 * sabc) / (ba + c2)).ln())
}

/// Calculate arclength using Gauss-Legendre quadrature using formula from Behdad
/// in https://github.com/Pomax/BezierInfo-2/issues/77
fn gauss_arclen_3(q: QuadBez) -> f64 {
    let v0 = (-0.492943519233745 * q.p0.to_vec2()
        + 0.430331482911935 * q.p1.to_vec2()
        + 0.0626120363218102 * q.p2.to_vec2())
    .hypot();
    let v1 = ((q.p2 - q.p0) * 0.4444444444444444).hypot();
    let v2 = (-0.0626120363218102 * q.p0.to_vec2() - 0.430331482911935 * q.p1.to_vec2()
        + 0.492943519233745 * q.p2.to_vec2())
    .hypot();
    v0 + v1 + v2
}

fn awesome_quad_arclen3(q: QuadBez, accuracy: f64, depth: usize) -> f64 {
    let pm = q.p0.midpoint(q.p2);
    let d1 = q.p1 - pm;
    let d = q.p2 - q.p0;
    let dhypot2 = d.hypot2();
    let x = 2.0 * d.dot(d1) / dhypot2;
    let y = 2.0 * d.cross(d1) / dhypot2;
    let lc = (q.p2 - q.p0).hypot();
    let lp = (q.p1 - q.p0).hypot() + (q.p2 - q.p1).hypot();
    let est_err = 0.06 * (lp - lc) * (x * x + y * y).powf(2.0);
    if est_err < accuracy || depth == 16 {
        gauss_arclen_3(q)
    } else {
        let (q0, q1) = q.subdivide();
        awesome_quad_arclen3(q0, accuracy * 0.5, depth + 1)
            + awesome_quad_arclen3(q1, accuracy * 0.5, depth + 1)
    }
}

/// Another implementation, using weights from
/// https://pomax.github.io/bezierinfo/legendre-gauss.html
fn gauss_arclen_n(q: QuadBez, coeffs: &[(f64, f64)]) -> f64 {
    let d = q.deriv();
    coeffs
        .iter()
        .map(|(wi, xi)| wi * d.eval(0.5 * (xi + 1.0)).to_vec2().hypot())
        .sum::<f64>()
        * 0.5
}

fn gauss_arclen_7(q: QuadBez) -> f64 {
    gauss_arclen_n(
        q,
        &[
            (0.4179591836734694, 0.0000000000000000),
            (0.3818300505051189, 0.4058451513773972),
            (0.3818300505051189, -0.4058451513773972),
            (0.2797053914892766, -0.7415311855993945),
            (0.2797053914892766, 0.7415311855993945),
            (0.1294849661688697, -0.9491079123427585),
            (0.1294849661688697, 0.9491079123427585),
        ],
    )
}

fn gauss_arclen_24(q: QuadBez) -> f64 {
    gauss_arclen_n(
        q,
        &[
            (0.1279381953467522, -0.0640568928626056),
            (0.1279381953467522, 0.0640568928626056),
            (0.1258374563468283, -0.1911188674736163),
            (0.1258374563468283, 0.1911188674736163),
            (0.1216704729278034, -0.3150426796961634),
            (0.1216704729278034, 0.3150426796961634),
            (0.1155056680537256, -0.4337935076260451),
            (0.1155056680537256, 0.4337935076260451),
            (0.1074442701159656, -0.5454214713888396),
            (0.1074442701159656, 0.5454214713888396),
            (0.0976186521041139, -0.6480936519369755),
            (0.0976186521041139, 0.6480936519369755),
            (0.0861901615319533, -0.7401241915785544),
            (0.0861901615319533, 0.7401241915785544),
            (0.0733464814110803, -0.8200019859739029),
            (0.0733464814110803, 0.8200019859739029),
            (0.0592985849154368, -0.8864155270044011),
            (0.0592985849154368, 0.8864155270044011),
            (0.0442774388174198, -0.9382745520027328),
            (0.0442774388174198, 0.9382745520027328),
            (0.0285313886289337, -0.9747285559713095),
            (0.0285313886289337, 0.9747285559713095),
            (0.0123412297999872, -0.9951872199970213),
            (0.0123412297999872, 0.9951872199970213),
        ],
    )
}

fn awesome_quad_arclen7(q: QuadBez, accuracy: f64, depth: usize) -> f64 {
    let pm = q.p0.midpoint(q.p2);
    let d1 = q.p1 - pm;
    let d = q.p2 - q.p0;
    let dhypot2 = d.hypot2();
    let x = 2.0 * d.dot(d1) / dhypot2;
    let y = 2.0 * d.cross(d1) / dhypot2;
    let lc = dhypot2.sqrt();
    let lp = (q.p1 - q.p0).hypot() + (q.p2 - q.p1).hypot();
    let est_err = 2.5e-2 * (lp - lc) * (x * x + y * y).powf(8.0).tanh();
    if est_err < accuracy || depth == 16 {
        gauss_arclen_7(q)
    } else {
        let (q0, q1) = q.subdivide();
        awesome_quad_arclen7(q0, accuracy * 0.5, depth + 1)
            + awesome_quad_arclen7(q1, accuracy * 0.5, depth + 1)
    }
}

#[bench]
fn bench_quad_arclen_analytical(b: &mut Bencher) {
    // Analytical solution is not sensitive to exact shape.
    let q = QuadBez::new((0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
    b.iter(|| quad_arclen_analytical(test::black_box(q)))
}

const ACCURACY: f64 = 1e-6;

#[bench]
fn bench_quad_arclen(b: &mut Bencher) {
    // This is a pretty easy case.
    let q = QuadBez::new((0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
    b.iter(|| (test::black_box(q).arclen(ACCURACY)))
}

#[bench]
fn bench_quad_arclen_gauss3(b: &mut Bencher) {
    let q = QuadBez::new((0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
    b.iter(|| awesome_quad_arclen3(test::black_box(q), ACCURACY, 0))
}

#[bench]
fn bench_quad_arclen_gauss7(b: &mut Bencher) {
    let q = QuadBez::new((0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
    b.iter(|| awesome_quad_arclen7(test::black_box(q), ACCURACY, 0))
}

#[bench]
fn bench_quad_arclen_gauss3_one(b: &mut Bencher) {
    let q = QuadBez::new((0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
    b.iter(|| gauss_arclen_3(test::black_box(q)))
}

#[bench]
fn bench_quad_arclen_gauss7_one(b: &mut Bencher) {
    let q = QuadBez::new((0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
    b.iter(|| gauss_arclen_7(test::black_box(q)))
}

#[bench]
fn bench_quad_arclen_gauss24_one(b: &mut Bencher) {
    let q = QuadBez::new((0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
    b.iter(|| gauss_arclen_24(test::black_box(q)))
}
