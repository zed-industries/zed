extern crate nalgebra as na;

use na::{Matrix4, Point3, Vector3, Vector4};

fn main() {
    let mut m = Matrix4::new_rotation_wrt_point(Vector3::x() * 1.57, Point3::new(1.0, 2.0, 1.0));
    m.append_scaling_mut(2.0);

    let point1 = Point3::new(2.0, 3.0, 4.0);
    let homogeneous_point2 = Vector4::new(2.0, 3.0, 4.0, 1.0);

    // First option: use the dedicated `.transform_point(...)` method.
    let transformed_point1 = m.transform_point(&point1);
    // Second option: use the homogeneous coordinates of the point.
    let transformed_homogeneous_point2 = m * homogeneous_point2;

    // Recover the 3D point from its 4D homogeneous coordinates.
    let transformed_point2 = Point3::from_homogeneous(transformed_homogeneous_point2);

    // Check that transforming the 3D point with the `.transform_point` method is
    // indeed equivalent to multiplying its 4D homogeneous coordinates by the 4x4
    // matrix.
    assert_eq!(transformed_point1, transformed_point2.unwrap());
}
