// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A test case for the cubic Bézier path simplifier, generating a very
//! efficient and accurate SVG file to plot a quartic polynomial. This
//! file was used to generate the image for the Wikipedia article on
//! quartic polynomials.

use std::{io::Write, ops::Range};

use kurbo::{BezPath, CurveFitSample, ParamCurveFit, PathEl, Point, Vec2};

struct MyPoly;

fn eval_mypoly(x: f64) -> f64 {
    (x + 4.) * (x + 1.) * (x - 1.) * (x - 3.) / 14. + 0.5
}

fn eval_mypoly_deriv(x: f64) -> f64 {
    (((4. * x + 3.) * x - 26.) * x - 1.) / 14.
}

impl ParamCurveFit for MyPoly {
    fn sample_pt_deriv(&self, t: f64) -> (Point, Vec2) {
        const S: f64 = 336.;
        let x = 37. + t * S;
        let math_x = (x - 220.) / 40.;
        let y = -eval_mypoly(math_x) * 40. + 260.;
        let dx = S;
        let dy = -dx * eval_mypoly_deriv(math_x);
        (Point::new(x, y), Vec2::new(dx, dy))
    }

    fn sample_pt_tangent(&self, t: f64, _: f64) -> CurveFitSample {
        let (p, tangent) = self.sample_pt_deriv(t);
        CurveFitSample { p, tangent }
    }

    fn break_cusp(&self, _: Range<f64>) -> Option<f64> {
        None
    }
}

fn to_svg_economical(path: &BezPath) -> String {
    let mut buffer = Vec::new();
    write_to(path, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

/// Write the SVG representation of this path to the provided buffer.
fn write_to<W: Write>(path: &BezPath, mut writer: W) -> std::io::Result<()> {
    for el in path.elements() {
        match *el {
            PathEl::MoveTo(p) => write!(writer, "M{:.2} {:.2}", p.x, p.y)?,
            PathEl::LineTo(p) => write!(writer, "L{} {}", p.x, p.y)?,
            PathEl::QuadTo(p1, p2) => write!(writer, "Q{} {} {} {}", p1.x, p1.y, p2.x, p2.y)?,
            PathEl::CurveTo(p1, p2, p3) => write!(
                writer,
                "C{:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                p1.x, p1.y, p2.x, p2.y, p3.x, p3.y
            )?,
            PathEl::ClosePath => write!(writer, "Z")?,
        }
    }
    Ok(())
}

fn main() {
    println!("<svg width='800' height='600' xmlns='http://www.w3.org/2000/svg'>");
    let path = kurbo::fit_to_bezpath_opt(&MyPoly, 0.1);
    println!(
        "  <path d='{}' stroke='#008' fill='none' stroke-width='2.0'/>",
        to_svg_economical(&path)
    );
    println!("</svg>");
}
