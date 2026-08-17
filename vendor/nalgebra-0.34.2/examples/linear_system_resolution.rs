#![cfg_attr(rustfmt, rustfmt_skip)]
#[macro_use]
extern crate approx; // for assert_relative_eq
extern crate nalgebra as na;
use na::{Matrix4, Matrix4x3, Vector4};

fn main() {
    let a = Matrix4::new(
        1.0, 1.0,  2.0, -5.0,
        2.0, 5.0, -1.0, -9.0,
        2.0, 1.0, -1.0,  3.0,
        1.0, 3.0,  2.0,  7.0,
    );
    let mut b = Vector4::new(3.0, -3.0, -11.0, -5.0);
    let decomp = a.lu();
    let x = decomp.solve(&b).expect("Linear resolution failed.");
    assert_relative_eq!(a * x, b);

    /*
     * It is possible to perform the resolution in-place.
     * This is particularly useful to avoid allocations when
     * `b` is a `DVector` or a `DMatrix`.
     */
    assert!(decomp.solve_mut(&mut b), "Linear resolution failed.");
    assert_relative_eq!(x, b);

    /*
     * It is possible to solve multiple systems
     * simultaneously by using a matrix for `b`.
     */
    let b = Matrix4x3::new(
         3.0,  2.0,  0.0,
        -3.0,  0.0,  0.0,
        -11.0, 5.0, -3.0,
        -5.0,  10.0, 4.0,
    );
    let x = decomp.solve(&b).expect("Linear resolution failed.");
    assert_relative_eq!(a * x, b);
}
