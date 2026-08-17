use na::{CsMatrix, DMatrix, Matrix4x5};

#[test]
fn cs_from_to_matrix() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let m = Matrix4x5::new(
        5.0, 6.0, 0.0, 8.0, 15.0,
        9.0, 10.0, 11.0, 12.0, 0.0,
        0.0, 0.0, 13.0, 0.0, 0.0,
        0.0, 1.0, 4.0, 0.0, 14.0,
    );

    let cs: CsMatrix<_, _, _> = m.into();
    assert!(cs.is_sorted());

    let m2: Matrix4x5<_> = cs.into();
    assert_eq!(m2, m);
}

#[test]
fn cs_matrix_from_triplet() {
    let mut irows = vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 3, 3, 3];
    let mut icols = vec![0, 1, 3, 4, 0, 1, 2, 3, 2, 1, 2, 4];
    let mut vals = vec![
        5.0, 6.0, 8.0, 15.0, 9.0, 10.0, 11.0, 12.0, 13.0, 1.0, 4.0, 14.0,
    ];

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = DMatrix::from_row_slice(4, 5, &[
        5.0, 6.0, 0.0, 8.0, 15.0,
        9.0, 10.0, 11.0, 12.0, 0.0,
        0.0, 0.0, 13.0, 0.0, 0.0,
        0.0, 1.0, 4.0, 0.0, 14.0,
    ]);
    let cs_expected = CsMatrix::from_parts(
        4,
        5,
        vec![0, 2, 5, 8, 10],
        vec![0, 1, 0, 1, 3, 1, 2, 3, 0, 1, 0, 3],
        vec![
            5.0, 9.0, 6.0, 10.0, 1.0, 11.0, 13.0, 4.0, 8.0, 12.0, 15.0, 14.0,
        ],
    );

    let cs_mat = CsMatrix::from_triplet(4, 5, &irows, &icols, &vals);
    assert!(cs_mat.is_sorted());
    assert_eq!(cs_mat, cs_expected);

    let m: DMatrix<_> = cs_mat.into();
    assert_eq!(m, expected);

    /*
     * Try again with some permutations.
     */
    let permutations = [(2, 5), (0, 4), (8, 10), (1, 11)];

    for (i, j) in &permutations {
        irows.swap(*i, *j);
        icols.swap(*i, *j);
        vals.swap(*i, *j);
    }

    let cs_mat = CsMatrix::from_triplet(4, 5, &irows, &icols, &vals);
    assert!(cs_mat.is_sorted());
    assert_eq!(cs_mat, cs_expected);

    let m: DMatrix<_> = cs_mat.into();
    assert_eq!(m, expected);

    /*
     * Try again, duplicating all entries.
     */
    let mut ir = irows.clone();
    let mut ic = icols.clone();
    let mut va = vals.clone();
    irows.append(&mut ir);
    icols.append(&mut ic);
    vals.append(&mut va);

    let cs_mat = CsMatrix::from_triplet(4, 5, &irows, &icols, &vals);
    assert!(cs_mat.is_sorted());
    assert_eq!(cs_mat, cs_expected * 2.0);

    let m: DMatrix<_> = cs_mat.into();
    assert_eq!(m, expected * 2.0);
}
