#[macro_use]
extern crate approx;
extern crate nalgebra as na;

use na::{Matrix4, Point3, Vector3};
use std::f32::consts;

fn main() {
    // Create a uniform scaling matrix with scaling factor 2.
    let mut m = Matrix4::new_scaling(2.0);

    assert_eq!(m.transform_vector(&Vector3::x()), Vector3::x() * 2.0);
    assert_eq!(m.transform_vector(&Vector3::y()), Vector3::y() * 2.0);
    assert_eq!(m.transform_vector(&Vector3::z()), Vector3::z() * 2.0);

    // Append a nonuniform scaling in-place.
    m.append_nonuniform_scaling_mut(&Vector3::new(1.0, 2.0, 3.0));

    assert_eq!(m.transform_vector(&Vector3::x()), Vector3::x() * 2.0);
    assert_eq!(m.transform_vector(&Vector3::y()), Vector3::y() * 4.0);
    assert_eq!(m.transform_vector(&Vector3::z()), Vector3::z() * 6.0);

    // Append a translation out-of-place.
    let m2 = m.append_translation(&Vector3::new(42.0, 0.0, 0.0));

    assert_eq!(
        m2.transform_point(&Point3::new(1.0, 1.0, 1.0)),
        Point3::new(42.0 + 2.0, 4.0, 6.0)
    );

    // Create rotation.
    let rot = Matrix4::from_scaled_axis(Vector3::x() * consts::PI);
    let rot_then_m = m * rot; // Right-multiplication is equivalent to prepending `rot` to `m`.
    let m_then_rot = rot * m; // Left-multiplication is equivalent to appending `rot` to `m`.

    let pt = Point3::new(1.0, 2.0, 3.0);

    assert_relative_eq!(
        m.transform_point(&rot.transform_point(&pt)),
        rot_then_m.transform_point(&pt)
    );
    assert_relative_eq!(
        rot.transform_point(&m.transform_point(&pt)),
        m_then_rot.transform_point(&pt)
    );
}
