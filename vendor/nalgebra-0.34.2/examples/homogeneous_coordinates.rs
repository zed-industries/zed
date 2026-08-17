#[macro_use]
extern crate approx;
extern crate nalgebra as na;

use na::{Isometry2, Point2, Vector2};
use std::f32;

fn use_dedicated_types() {
    let iso = Isometry2::new(Vector2::new(1.0, 1.0), f32::consts::PI);
    let pt = Point2::new(1.0, 0.0);
    let vec = Vector2::x();

    let transformed_pt = iso * pt;
    let transformed_vec = iso * vec;

    assert_relative_eq!(transformed_pt, Point2::new(0.0, 1.0));
    assert_relative_eq!(transformed_vec, Vector2::new(-1.0, 0.0));
}

fn use_homogeneous_coordinates() {
    let iso = Isometry2::new(Vector2::new(1.0, 1.0), f32::consts::PI);
    let pt = Point2::new(1.0, 0.0);
    let vec = Vector2::x();

    // Compute using homogeneous coordinates.
    let hom_iso = iso.to_homogeneous();
    let hom_pt = pt.to_homogeneous();
    let hom_vec = vec.to_homogeneous();

    let hom_transformed_pt = hom_iso * hom_pt;
    let hom_transformed_vec = hom_iso * hom_vec;

    // Convert back to the cartesian coordinates.
    let transformed_pt = Point2::from_homogeneous(hom_transformed_pt).unwrap();
    let transformed_vec = Vector2::from_homogeneous(hom_transformed_vec).unwrap();

    assert_relative_eq!(transformed_pt, Point2::new(0.0, 1.0));
    assert_relative_eq!(transformed_vec, Vector2::new(-1.0, 0.0));
}

fn main() {
    use_dedicated_types();
    use_homogeneous_coordinates();
}
