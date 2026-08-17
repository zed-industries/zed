// Copyright 2019 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Example of circle

#[cfg(feature = "std")]
fn main() {
    use kurbo::{Circle, Shape};

    let circle = Circle::new((400.0, 400.0), 380.0);
    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<body>");
    println!("<svg height=\"800\" width=\"800\">");
    let black_path = circle.to_path(1e-3).to_svg();
    println!("  <path d=\"{black_path}\" stroke=\"black\" fill=\"none\" />");
    let red_path = circle.to_path(1.0).to_svg();
    println!("  <path d=\"{red_path}\" stroke=\"red\" fill=\"none\" />");
    println!("</svg>");
    println!("</body>");
    println!("</html>");
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the standard library");
}
