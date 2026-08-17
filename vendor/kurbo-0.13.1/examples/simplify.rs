// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Test case showing simplification of a Bézier path. In this case, the
//! source path is a simple mathematical function, but it should work for
//! more general cases as well.

use kurbo::{BezPath, Point};

fn plot_fn(f: &dyn Fn(f64) -> f64, d: &dyn Fn(f64) -> f64, xa: f64, xb: f64, n: usize) -> BezPath {
    let width = 800.;
    let dx = (xb - xa) / n as f64;
    let xs = width / (xb - xa);
    let ys = 250.;
    let y_origin = 300.;
    let plot = |x, y| Point::new((x - xa) * xs, y_origin - y * ys);
    let mut x0 = xa;
    let mut y0 = f(xa);
    let mut d0 = d(xa);
    let mut path = BezPath::new();
    path.move_to(plot(x0, y0));
    for i in 0..n {
        let x3 = xa + dx * (i + 1) as f64;
        let y3 = f(x3);
        let d3 = d(x3);
        let x1 = x0 + (1. / 3.) * dx;
        let x2 = x3 - (1. / 3.) * dx;
        let y1 = y0 + d0 * (1. / 3.) * dx;
        let y2 = y3 - d3 * (1. / 3.) * dx;
        path.curve_to(plot(x1, y1), plot(x2, y2), plot(x3, y3));
        x0 = x3;
        y0 = y3;
        d0 = d3;
    }
    path
}

fn main() {
    println!("<svg width='800' height='600' xmlns='http://www.w3.org/2000/svg'>");
    let path = plot_fn(&|x| x.sin(), &|x| x.cos(), -8., 8., 20);
    println!(
        "  <path d='{}' stroke='#8c8' fill='none' stroke-width='5'/>",
        path.to_svg()
    );
    let simpl = kurbo::simplify::SimplifyBezPath::new(path);
    let simplified_path = kurbo::fit_to_bezpath_opt(&simpl, 0.1);
    println!(
        "  <path d='{}' stroke='#000' fill='none' stroke-width='1'/>",
        simplified_path.to_svg()
    );
    println!("</svg>");
}
