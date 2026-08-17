extern crate nalgebra as na;

use na::{Isometry2, Similarity2, Vector2};
use std::f32::consts;

fn main() {
    // Isometry -> Similarity conversion always succeeds.
    let iso = Isometry2::new(Vector2::new(1.0f32, 2.0), na::zero());
    let _: Similarity2<f32> = na::convert(iso);

    // Similarity -> Isometry conversion fails if the scaling factor is not 1.0.
    let sim_without_scaling = Similarity2::new(Vector2::new(1.0f32, 2.0), consts::PI, 1.0);
    let sim_with_scaling = Similarity2::new(Vector2::new(1.0f32, 2.0), consts::PI, 2.0);

    let iso_success: Option<Isometry2<f32>> = na::try_convert(sim_without_scaling);
    let iso_fail: Option<Isometry2<f32>> = na::try_convert(sim_with_scaling);

    assert!(iso_success.is_some());
    assert!(iso_fail.is_none());

    // Similarity -> Isometry conversion can be forced at your own risks.
    let iso_forced: Isometry2<f32> = na::convert_unchecked(sim_with_scaling);
    assert_eq!(iso_success.unwrap(), iso_forced);
}
