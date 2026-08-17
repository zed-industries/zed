#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate nalgebra as na;

use na::{DMatrix, Dyn, Matrix2x3, Matrix3x2, Const};

fn main() {
    // Matrices can be reshaped in-place without moving or copying values.
    let m1 = Matrix2x3::new(
        1.1, 1.2, 1.3,
        2.1, 2.2, 2.3
    );
    let m2 = Matrix3x2::new(
        1.1, 2.2,
        2.1, 1.3,
        1.2, 2.3
    );

    let m3 = m1.reshape_generic(Const::<3>, Const::<2>);
    assert_eq!(m3, m2);

    // Note that, for statically sized matrices, invalid reshapes will not compile:
    //let m4 = m3.reshape_generic(U3, U3);

    // If dynamically sized matrices are used, the reshaping is checked at run-time.
    let dm1 = DMatrix::from_row_slice(
        4,
        3,
        &[
            1.0, 0.0, 0.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 0.0,
            0.0, 1.0, 0.0
        ],
    );
    let dm2 = DMatrix::from_row_slice(
        6,
        2,
        &[
            1.0, 0.0,
            0.0, 1.0,
            0.0, 0.0,
            0.0, 1.0,
            0.0, 0.0,
            0.0, 0.0,
        ],
    );

    let dm3 = dm1.reshape_generic(Dyn(6), Dyn(2));
    assert_eq!(dm3, dm2);

    // Invalid reshapings of dynamic matrices will panic at run-time.
    //let dm4 = dm3.reshape_generic(Dyn(6), Dyn(6));
}
