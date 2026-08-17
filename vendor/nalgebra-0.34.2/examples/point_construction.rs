extern crate nalgebra as na;

use na::{Point3, Vector3, Vector4};

fn main() {
    // Build using components directly.
    let p0 = Point3::new(2.0, 3.0, 4.0);

    // Build from a coordinates vector.
    let coords = Vector3::new(2.0, 3.0, 4.0);
    let p1 = Point3::from(coords);

    // Build by translating the origin.
    let translation = Vector3::new(2.0, 3.0, 4.0);
    let p2 = Point3::origin() + translation;

    // Build from homogeneous coordinates. The last component of the
    // vector will be removed and all other components divided by 10.0.
    let homogeneous_coords = Vector4::new(20.0, 30.0, 40.0, 10.0);
    let p3 = Point3::from_homogeneous(homogeneous_coords);

    assert_eq!(p0, p1);
    assert_eq!(p0, p2);
    assert_eq!(p0, p3.unwrap());
}
