#![cfg_attr(rustfmt, rustfmt_skip)]

use na::{CsMatrix, CsVector, CsCholesky, Cholesky, Matrix5, Vector5};

#[test]
fn cs_cholesky() {
    let mut a = Matrix5::new(
        40.0, 0.0, 0.0, 0.0, 0.0,
        2.0, 60.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 11.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 50.0, 0.0,
        1.0, 0.0, 0.0, 4.0, 10.0
    );
    a.fill_upper_triangle_with_lower_triangle();
    test_cholesky(a);

    let a = Matrix5::from_diagonal(&Vector5::new(40.0, 60.0, 11.0, 50.0, 10.0));
    test_cholesky(a);

    let mut a = Matrix5::new(
        40.0, 0.0, 0.0, 0.0, 0.0,
        2.0, 60.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 11.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 50.0, 0.0,
        0.0, 0.0, 0.0, 4.0, 10.0
    );
    a.fill_upper_triangle_with_lower_triangle();
    test_cholesky(a);

    let mut a = Matrix5::new(
        2.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 2.0, 0.0, 0.0, 0.0,
        1.0, 1.0, 2.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 2.0, 0.0,
        1.0, 1.0, 0.0, 0.0, 2.0
    );
    a.fill_upper_triangle_with_lower_triangle();
    // Test crate::new, left_looking, and up_looking implementations.
    test_cholesky(a);
}

fn test_cholesky(a: Matrix5<f32>) {
    // Test crate::new
    test_cholesky_variant(a, 0);
    // Test up-looking
    test_cholesky_variant(a, 1);
    // Test left-looking
    test_cholesky_variant(a, 2);
}

fn test_cholesky_variant(a: Matrix5<f32>, option: usize) {
    let cs_a: CsMatrix<_, _, _> = a.into();

    let chol_a = Cholesky::new(a).unwrap();
    let mut chol_cs_a;

    match option {
        0 => chol_cs_a = CsCholesky::new(&cs_a),
        1 => {
            chol_cs_a = CsCholesky::new_symbolic(&cs_a);
            chol_cs_a.decompose_up_looking(cs_a.data.values());
        }
        _ => {
            chol_cs_a = CsCholesky::new_symbolic(&cs_a);
            chol_cs_a.decompose_left_looking(cs_a.data.values());
        }
    };

    let l = chol_a.l();
    let cs_l = chol_cs_a.unwrap_l().unwrap();
    assert!(cs_l.is_sorted());

    let cs_l_mat: Matrix5<_> = cs_l.into();
    assert_relative_eq!(l, cs_l_mat);
}
