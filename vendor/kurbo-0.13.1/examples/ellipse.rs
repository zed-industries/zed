// Copyright 2020 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Example of ellipse

#[cfg(feature = "std")]
fn main() {
    use kurbo::{Ellipse, Shape};
    use std::f64::consts::PI;

    let ellipse = Ellipse::new((400.0, 400.0), (200.0, 100.0), 0.25 * PI);
    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<body>");
    println!("<svg height=\"800\" width=\"800\" style=\"background-color: #999\">");
    let black_path = ellipse.to_path(1e-3).to_svg();
    println!("  <path d=\"{black_path}\" stroke=\"black\" fill=\"none\" />");
    let red_path = ellipse.to_path(1.0).to_svg();
    println!("  <path d=\"{red_path}\" stroke=\"red\" fill=\"none\" />");
    println!("</svg>");
    println!("</body>");
    println!("</html>");
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the standard library");
}
