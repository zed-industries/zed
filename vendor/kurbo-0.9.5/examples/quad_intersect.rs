// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Test case that demonstrates quadratic/quadratic intersection using
//! new quartic solver.

use arrayvec::ArrayVec;
use kurbo::{common::solve_quartic, ParamCurve, Point, QuadBez, Shape};
use rand::{thread_rng, Rng};

fn rand_point() -> Point {
    let mut rng = thread_rng();
    Point::new(rng.gen_range(0.0..500.0), rng.gen_range(0.0..500.0))
}

fn rand_quad() -> QuadBez {
    QuadBez::new(rand_point(), rand_point(), rand_point())
}

struct ImplicitQuad {
    x2: f64,
    xy: f64,
    y2: f64,
    x: f64,
    y: f64,
    c: f64,
}

impl ImplicitQuad {
    fn from_quad(q: QuadBez) -> Self {
        let Point { x: ax, y: ay } = q.p0;
        let Point { x: bx, y: by } = q.p1;
        let Point { x: cx, y: cy } = q.p2;
        let (u0, u1, u2) = (by - cy, cx - bx, bx * cy - by * cx);
        let (v0, v1, v2) = (cy - ay, ax - cx, cx * ay - cy * ax);
        let (w0, w1, w2) = (ay - by, bx - ax, ax * by - ay * bx);
        ImplicitQuad {
            x2: 4. * u0 * w0 - v0 * v0,
            xy: 4. * (u0 * w1 + u1 * w0) - 2. * v0 * v1,
            y2: 4. * u1 * w1 - v1 * v1,
            x: 4. * (u0 * w2 + u2 * w0) - 2. * v0 * v2,
            y: 4. * (u1 * w2 + u2 * w1) - 2. * v1 * v2,
            c: 4. * u2 * w2 - v2 * v2,
        }
    }

    fn eval(&self, x: f64, y: f64) -> f64 {
        self.x2 * x * x + self.xy * x * y + self.y2 * y * y + self.x * x + self.y * y + self.c
    }
}

fn intersect_quads(q0: QuadBez, q1: QuadBez) -> ArrayVec<f64, 4> {
    let iq = ImplicitQuad::from_quad(q0);
    let c = q1.p0.to_vec2();
    let b = (q1.p1 - q1.p0) * 2.0;
    let a = c - q1.p1.to_vec2() * 2.0 + q1.p2.to_vec2();
    let c0 = iq.eval(c.x, c.y);
    let c1 = iq.x * b.x
        + iq.y * b.y
        + 2. * iq.x2 * (b.x * c.x)
        + 2. * iq.y2 * (b.y * c.y)
        + iq.xy * (b.x * c.y + b.y * c.x);
    let c2 = iq.x * a.x
        + iq.y * a.y
        + iq.x2 * (2. * a.x * c.x + b.x * b.x)
        + iq.xy * (a.x * c.y + b.x * b.y + a.y * c.x)
        + iq.y2 * (2. * a.y * c.y + b.y * b.y);
    let c3 = iq.x2 * 2. * a.x * b.x + iq.xy * (a.x * b.y + b.x * a.y) + iq.y2 * 2. * a.y * b.y;
    let c4 = iq.x2 * a.x * a.x + iq.xy * a.x * a.y + iq.y2 * a.y * a.y;
    let ts = solve_quartic(c0, c1, c2, c3, c4);
    println!("ts: {:?}", ts);
    ts
}

fn main() {
    let q0 = rand_quad();
    let q1 = rand_quad();
    println!(
        "  <path d='{}' stroke='#000' fill='none' />",
        q0.to_path(1e-9).to_svg()
    );
    println!(
        "  <path d='{}' stroke='#008' fill='none' />",
        q1.to_path(1e-9).to_svg()
    );
    for t in intersect_quads(q0, q1) {
        if (0.0..=1.0).contains(&t) {
            let p = q1.eval(t);
            println!("  <circle cx='{}' cy='{}' r='3' />", p.x, p.y);
        }
    }
}
