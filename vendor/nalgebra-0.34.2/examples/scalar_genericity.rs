extern crate nalgebra as na;

use na::{Scalar, Vector3};
use simba::scalar::RealField;

fn print_vector<T: Scalar>(m: &Vector3<T>) {
    println!("{:?}", m)
}

fn print_norm<T: RealField>(v: &Vector3<T>) {
    // NOTE: alternatively, nalgebra already defines `v.norm()`.
    let norm = v.dot(v).sqrt();

    // The RealField bound implies that T is Display so we can
    // use "{}" instead of "{:?}" for the format string.
    println!("{}", norm)
}

fn main() {
    let v1 = Vector3::new(1, 2, 3);
    let v2 = Vector3::new(1.0, 2.0, 3.0);

    print_vector(&v1);
    print_norm(&v2);
}
