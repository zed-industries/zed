#![allow(unused_variables)]

extern crate nalgebra as na;

use na::{Perspective3, Point2, Point3, Unit};
use std::f32::consts;

fn main() {
    let projection = Perspective3::new(800.0 / 600.0, consts::PI / 2.0, 1.0, 1000.0);
    let screen_point = Point2::new(10.0f32, 20.0);

    // Compute two points in clip-space.
    // "ndc" = normalized device coordinates.
    let near_ndc_point = Point3::new(screen_point.x / 800.0, screen_point.y / 600.0, -1.0);
    let far_ndc_point = Point3::new(screen_point.x / 800.0, screen_point.y / 600.0, 1.0);

    // Unproject them to view-space.
    let near_view_point = projection.unproject_point(&near_ndc_point);
    let far_view_point = projection.unproject_point(&far_ndc_point);

    // Compute the view-space line parameters.
    let line_location = near_view_point;
    let line_direction = Unit::new_normalize(far_view_point - near_view_point);
}
