// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A simple example to show an offset curve of a cubic Bézier segment.

use kurbo::{offset::CubicOffset, CubicBez, Shape};

fn main() {
    println!("<svg width='800' height='600' xmlns='http://www.w3.org/2000/svg'>");
    let c = CubicBez::new((100., 100.), (150., 75.), (300., 50.), (400., 200.));
    println!(
        "  <path d='{}' stroke='#000' fill='none' />",
        c.to_path(1e-9).to_svg()
    );
    for i in 1..=80 {
        let co = CubicOffset::new(c, i as f64 * 4.0);
        let path = kurbo::fit_to_bezpath_opt(&co, 1e-3);
        println!(
            "  <path d='{}' stroke='#008' fill='none' stroke-width='0.2'/>",
            path.to_svg()
        );
    }
    println!("</svg>");
}
