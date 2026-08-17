#![allow(clippy::float_cmp)]
extern crate nalgebra as na;

use na::{Unit, Vector3};

fn length_on_direction_with_unit(v: &Vector3<f32>, dir: &Unit<Vector3<f32>>) -> f32 {
    // No need to normalize `dir`: we know that it is non-zero and normalized.
    v.dot(dir.as_ref())
}

fn length_on_direction_without_unit(v: &Vector3<f32>, dir: &Vector3<f32>) -> f32 {
    // Obligatory normalization of the direction vector (and test, for robustness).
    if let Some(unit_dir) = dir.try_normalize(1.0e-6) {
        v.dot(&unit_dir)
    } else {
        // Normalization failed because the norm was too small.
        panic!("Invalid input direction.")
    }
}

fn main() {
    let v = Vector3::new(1.0, 2.0, 3.0);

    let l1 = length_on_direction_with_unit(&v, &Vector3::y_axis());
    let l2 = length_on_direction_without_unit(&v, &Vector3::y());

    assert_eq!(l1, l2)
}
