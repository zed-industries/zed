#![allow(unused_variables)]

extern crate nalgebra as na;

use na::{Isometry3, Perspective3, Point3, Vector3};
use std::f32::consts;

fn main() {
    // Our object is translated along the x axis.
    let model = Isometry3::new(Vector3::x(), na::zero());

    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye = Point3::new(0.0, 0.0, 1.0);
    let target = Point3::new(1.0, 0.0, 0.0);
    let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

    // A perspective projection.
    let projection = Perspective3::new(16.0 / 9.0, consts::PI / 2.0, 1.0, 1000.0);

    // The combination of the model with the view is still an isometry.
    let model_view = view * model;

    // Convert everything to a `Matrix4` so that they can be combined.
    let mat_model_view = model_view.to_homogeneous();

    // Combine everything.
    let model_view_projection = projection.as_matrix() * mat_model_view;
}
