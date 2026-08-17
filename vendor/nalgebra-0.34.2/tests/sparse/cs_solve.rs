#![cfg_attr(rustfmt, rustfmt_skip)]

use na::{CsMatrix, CsVector, Matrix5, Vector5};


#[test]
fn cs_lower_triangular_solve() {
    let a = Matrix5::new(
        4.0, 1.0,  4.0,  0.0,  9.0,
        5.0, 6.0,  0.0,  8.0,  10.0,
        9.0, 10.0, 11.0, 12.0, 0.0,
        0.0, -8.0, 3.0,  5.0,  9.0,
        0.0, 0.0,  1.0,  0.0,  -10.0
    );
    let b = Vector5::new(1.0, 2.0, 3.0, 4.0, 5.0);

    let cs_a: CsMatrix<_, _, _> = a.into();

    assert_eq!(cs_a.solve_lower_triangular(&b), a.solve_lower_triangular(&b));
}

#[test]
fn cs_tr_lower_triangular_solve() {
    let a = Matrix5::new(
        4.0, 1.0,  4.0,  0.0,  9.0,
        5.0, 6.0,  0.0,  8.0,  10.0,
        9.0, 10.0, 11.0, 12.0, 0.0,
        0.0, -8.0, 3.0,  5.0,  9.0,
        0.0, 0.0,  1.0,  0.0,  -10.0
    );
    let b = Vector5::new(1.0, 2.0, 3.0, 4.0, 5.0);

    let cs_a: CsMatrix<_, _, _> = a.into();

    assert!(cs_a.tr_solve_lower_triangular(&b).is_some());
    assert_eq!(cs_a.tr_solve_lower_triangular(&b), a.tr_solve_lower_triangular(&b));

    // Singular case.
    let a = Matrix5::new(
        4.0, 1.0,  4.0, 0.0,  9.0,
        5.0, 6.0,  0.0, 8.0,  10.0,
        9.0, 10.0, 0.0, 12.0, 0.0,
        0.0, -8.0, 3.0, 5.0,  9.0,
        0.0, 0.0,  1.0, 0.0,  -10.0
    );
    let cs_a: CsMatrix<_, _, _> = a.into();

    assert!(cs_a.tr_solve_lower_triangular(&b).is_none());
}


#[test]
fn cs_lower_triangular_solve_cs() {
    let a = Matrix5::new(
        4.0, 1.0,  4.0,  0.0,  9.0,
        5.0, 6.0,  0.0,  8.0,  10.0,
        9.0, 10.0, 11.0, 12.0, 0.0,
        0.0, -8.0, 3.0,  5.0,  9.0,
        0.0, 0.0,  1.0,  0.0,  -10.0
    );
    let b1 = Vector5::zeros();
    let b2 = Vector5::new(1.0, 2.0, 3.0, 4.0, 5.0);
    let b3 = Vector5::new(1.0, 0.0, 0.0, 4.0, 0.0);
    let b4 = Vector5::new(0.0, 1.0, 0.0, 4.0, 5.0);
    let b5 = Vector5::x();
    let b6 = Vector5::y();
    let b7 = Vector5::z();
    let b8 = Vector5::w();
    let b9 = Vector5::a();

    let cs_a: CsMatrix<_, _, _> = a.into();
    let cs_b1: CsVector<_, _> = Vector5::zeros().into();
    let cs_b2: CsVector<_, _> = Vector5::new(1.0, 2.0, 3.0, 4.0, 5.0).into();
    let cs_b3: CsVector<_, _> = Vector5::new(1.0, 0.0, 0.0, 4.0, 0.0).into();
    let cs_b4: CsVector<_, _> = Vector5::new(0.0, 1.0, 0.0, 4.0, 5.0).into();
    let cs_b5: CsVector<_, _> = Vector5::x().into();
    let cs_b6: CsVector<_, _> = Vector5::y().into();
    let cs_b7: CsVector<_, _> = Vector5::z().into();
    let cs_b8: CsVector<_, _> = Vector5::w().into();
    let cs_b9: CsVector<_, _> = Vector5::a().into();

    assert_eq!(cs_a.solve_lower_triangular_cs(&cs_b1).map(|v| { assert!(v.is_sorted()); v.into() }), a.solve_lower_triangular(&b1));
    assert_eq!(cs_a.solve_lower_triangular_cs(&cs_b5).map(|v| { assert!(v.is_sorted()); v.into() }), a.solve_lower_triangular(&b5));
    assert_eq!(cs_a.solve_lower_triangular_cs(&cs_b6).map(|v| { assert!(v.is_sorted()); v.into() }), a.solve_lower_triangular(&b6));
    assert_eq!(cs_a.solve_lower_triangular_cs(&cs_b7).map(|v| { assert!(v.is_sorted()); v.into() }), a.solve_lower_triangular(&b7));
    assert_eq!(cs_a.solve_lower_triangular_cs(&cs_b8).map(|v| { assert!(v.is_sorted()); v.into() }), a.solve_lower_triangular(&b8));
    assert_eq!(cs_a.solve_lower_triangular_cs(&cs_b9).map(|v| { assert!(v.is_sorted()); v.into() }), a.solve_lower_triangular(&b9));
    assert_eq!(cs_a.solve_lower_triangular_cs(&cs_b2).map(|v| { assert!(v.is_sorted()); v.into() }), a.solve_lower_triangular(&b2));
    assert_eq!(cs_a.solve_lower_triangular_cs(&cs_b3).map(|v| { assert!(v.is_sorted()); v.into() }), a.solve_lower_triangular(&b3));
    assert_eq!(cs_a.solve_lower_triangular_cs(&cs_b4).map(|v| { assert!(v.is_sorted()); v.into() }), a.solve_lower_triangular(&b4));


    // Singular case.
    let a = Matrix5::new(
        4.0, 1.0,  4.0, 0.0,  9.0,
        5.0, 0.0,  0.0, 8.0,  10.0,
        9.0, 10.0, 0.0, 12.0, 0.0,
        0.0, -8.0, 3.0, 5.0,  9.0,
        0.0, 0.0,  1.0, 0.0,  -10.0
    );
    let cs_a: CsMatrix<_, _, _> = a.into();

    assert!(cs_a.solve_lower_triangular_cs(&cs_b2).is_none());
    assert!(cs_a.solve_lower_triangular_cs(&cs_b3).is_none());
    assert!(cs_a.solve_lower_triangular_cs(&cs_b4).is_none());
}
