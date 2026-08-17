#[macro_use]
extern crate approx;
extern crate nalgebra as na;

use na::{Isometry2, Point2, Vector2};
use std::f32;

fn main() {
    let t = Isometry2::new(Vector2::new(1.0, 1.0), f32::consts::PI);
    let p = Point2::new(1.0, 0.0); // Will be affected by te rotation and the translation.
    let v = Vector2::x(); // Will *not* be affected by the translation.

    assert_relative_eq!(t * p, Point2::new(-1.0 + 1.0, 1.0));
    //                                     ^^^^ │ ^^^^^^^^
    //                                  rotated │ translated

    assert_relative_eq!(t * v, Vector2::new(-1.0, 0.0));
    //                                      ^^^^^
    //                                   rotated only
}
