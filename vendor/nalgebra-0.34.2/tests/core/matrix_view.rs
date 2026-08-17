#![allow(non_snake_case)]

use na::{
    DMatrix, DMatrixView, DMatrixViewMut, Matrix2, Matrix2x3, Matrix2x4, Matrix2x6, Matrix3,
    Matrix3x2, Matrix3x4, Matrix4x2, Matrix6x2, MatrixView2, MatrixView2x3, MatrixView2xX,
    MatrixView3, MatrixView3x2, MatrixViewMut2, MatrixViewMut2x3, MatrixViewMut2xX, MatrixViewMut3,
    MatrixViewMut3x2, MatrixViewMutXx3, MatrixViewXx3, RowVector4, Vector3,
};

#[test]
#[rustfmt::skip]
fn nested_fixed_views() {
    let a = Matrix3x4::new(11.0, 12.0, 13.0, 14.0,
                           21.0, 22.0, 23.0, 24.0,
                           31.0, 32.0, 33.0, 34.0);

    let s1 = a.fixed_view::<3, 3>(0, 1);                       // Simple view.
    let s2 = s1.fixed_view::<2, 2>(1, 1);                      // View of view.
    let s3 = s1.fixed_view_with_steps::<2, 2>((0, 0), (1, 1)); // View of view with steps.

    let expected_owned_s1 = Matrix3::new(12.0, 13.0, 14.0,
                                         22.0, 23.0, 24.0,
                                         32.0, 33.0, 34.0);

    let expected_owned_s2 = Matrix2::new(23.0, 24.0,
                                         33.0, 34.0);

    let expected_owned_s3 = Matrix2::new(12.0, 14.0,
                                         32.0, 34.0);

    assert_eq!(expected_owned_s1, s1.clone_owned());
    assert_eq!(expected_owned_s2, s2.clone_owned());
    assert_eq!(expected_owned_s3, s3.clone_owned());
}

#[test]
#[rustfmt::skip]
fn nested_views() {
    let a = Matrix3x4::new(11.0, 12.0, 13.0, 14.0,
                           21.0, 22.0, 23.0, 24.0,
                           31.0, 32.0, 33.0, 34.0);

    let s1 = a.view((0, 1), (3, 3));
    let s2 = s1.view((1, 1), (2, 2));
    let s3 = s1.view_with_steps((0, 0), (2, 2), (1, 1));

    let expected_owned_s1 = DMatrix::from_row_slice(3, 3, &[ 12.0, 13.0, 14.0,
                                                             22.0, 23.0, 24.0,
                                                             32.0, 33.0, 34.0 ]);

    let expected_owned_s2 = DMatrix::from_row_slice(2, 2, &[ 23.0, 24.0,
                                                             33.0, 34.0 ]);

    let expected_owned_s3 = DMatrix::from_row_slice(2, 2, &[ 12.0, 14.0,
                                                             32.0, 34.0 ]);

    assert_eq!(expected_owned_s1, s1.clone_owned());
    assert_eq!(expected_owned_s2, s2.clone_owned());
    assert_eq!(expected_owned_s3, s3.clone_owned());
}

#[test]
#[rustfmt::skip]
fn view_mut() {
    let mut a = Matrix3x4::new(11.0, 12.0, 13.0, 14.0,
                               21.0, 22.0, 23.0, 24.0,
                               31.0, 32.0, 33.0, 34.0);

    {
        // We modify `a` through the mutable view.
        let mut s1 = a.view_with_steps_mut((0, 1), (2, 2), (1, 1));
        s1.fill(0.0);
    }

    let expected_a = Matrix3x4::new(11.0,  0.0, 13.0,  0.0,
                                    21.0, 22.0, 23.0, 24.0,
                                    31.0,  0.0, 33.0,  0.0);

    assert_eq!(expected_a, a);
}

#[test]
#[rustfmt::skip]
fn nested_row_views() {
    let a = Matrix6x2::new(11.0, 12.0,
                           21.0, 22.0,
                           31.0, 32.0,
                           41.0, 42.0,
                           51.0, 52.0,
                           61.0, 62.0);
    let s1 = a.fixed_rows::<4>(1);
    let s2 = s1.fixed_rows_with_step::<2>(1, 1);

    let expected_owned_s1 = Matrix4x2::new(21.0, 22.0,
                                           31.0, 32.0,
                                           41.0, 42.0,
                                           51.0, 52.0);

    let expected_owned_s2 = Matrix2::new(31.0, 32.0,
                                         51.0, 52.0);

    assert_eq!(expected_owned_s1, s1.clone_owned());
    assert_eq!(expected_owned_s2, s2.clone_owned());
}

#[test]
#[rustfmt::skip]
fn row_view_mut() {
    let mut a = Matrix6x2::new(11.0, 12.0,
                               21.0, 22.0,
                               31.0, 32.0,
                               41.0, 42.0,
                               51.0, 52.0,
                               61.0, 62.0);
    {
        // We modify `a` through the mutable view.
        let mut s1 = a.rows_with_step_mut(1, 3, 1);
        s1.fill(0.0);
    }

    let expected_a = Matrix6x2::new(11.0, 12.0,
                                     0.0,  0.0,
                                    31.0, 32.0,
                                     0.0,  0.0,
                                    51.0, 52.0,
                                     0.0,  0.0);

    assert_eq!(expected_a, a);
}

#[test]
#[rustfmt::skip]
fn nested_col_views() {
    let a = Matrix2x6::new(11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
                           21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
    let s1 = a.fixed_columns::<4>(1);
    let s2 = s1.fixed_columns_with_step::<2>(1, 1);

    let expected_owned_s1 = Matrix2x4::new(12.0, 13.0, 14.0, 15.0,
                                           22.0, 23.0, 24.0, 25.0);

    let expected_owned_s2 = Matrix2::new(13.0, 15.0,
                                         23.0, 25.0);

    assert_eq!(expected_owned_s1, s1.clone_owned());
    assert_eq!(expected_owned_s2, s2.clone_owned());
}

#[test]
#[rustfmt::skip]
fn col_view_mut() {
    let mut a = Matrix2x6::new(11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
                               21.0, 22.0, 23.0, 24.0, 25.0, 26.0);

    {
        // We modify `a` through the mutable view.
        let mut s1 = a.columns_with_step_mut(1, 3, 1);
        s1.fill(0.0);
    }

    let expected_a = Matrix2x6::new(11.0, 0.0, 13.0, 0.0, 15.0, 0.0,
                                    21.0, 0.0, 23.0, 0.0, 25.0, 0.0);

    assert_eq!(expected_a, a.clone_owned());
}

#[test]
#[rustfmt::skip]
fn rows_range_pair() {
    let a = Matrix3x4::new(11.0, 12.0, 13.0, 14.0,
                           21.0, 22.0, 23.0, 24.0,
                           31.0, 32.0, 33.0, 34.0);

    let (l, r) = a.rows_range_pair(.. 3, 3 ..);
    assert!(r.len() == 0 && l.eq(&a));

    let (l, r) = a.rows_range_pair(0, 1 ..);

    let expected_l = RowVector4::new(11.0, 12.0, 13.0, 14.0);
    let expected_r = Matrix2x4::new(21.0, 22.0, 23.0, 24.0,
                                    31.0, 32.0, 33.0, 34.0);
    assert!(l.eq(&expected_l) && r.eq(&expected_r));
}

#[test]
#[rustfmt::skip]
fn columns_range_pair() {
    let a = Matrix3x4::new(11.0, 12.0, 13.0, 14.0,
                           21.0, 22.0, 23.0, 24.0,
                           31.0, 32.0, 33.0, 34.0);

    let (l, r) = a.columns_range_pair(.. 4, 4 ..);
    assert!(r.len() == 0 && l.eq(&a));

    let (l, r) = a.columns_range_pair(0, 1 ..);

    let expected_l = Vector3::new(11.0, 21.0, 31.0);
    let expected_r = Matrix3::new(12.0, 13.0, 14.0,
                                  22.0, 23.0, 24.0,
                                  32.0, 33.0, 34.0);
    assert!(l.eq(&expected_l) && r.eq(&expected_r));
}

#[test]
#[rustfmt::skip]
fn new_from_slice() {
    let data = [ 1.0, 2.0,  3.0,  4.0,
                 5.0, 6.0,  7.0,  8.0,
                 9.0, 10.0, 11.0, 12.0 ];

    let expected2   = Matrix2::from_column_slice(&data);
    let expected3   = Matrix3::from_column_slice(&data);
    let expected2x3 = Matrix2x3::from_column_slice(&data);
    let expected3x2 = Matrix3x2::from_column_slice(&data);

    {
        let m2   = MatrixView2::from_slice(&data);
        let m3   = MatrixView3::from_slice(&data);
        let m2x3 = MatrixView2x3::from_slice(&data);
        let m3x2 = MatrixView3x2::from_slice(&data);
        let m2xX = MatrixView2xX::from_slice(&data, 3);
        let mXx3 = MatrixViewXx3::from_slice(&data, 2);
        let mXxX = DMatrixView::from_slice(&data, 2, 3);

        assert!(m2.eq(&expected2));
        assert!(m3.eq(&expected3));
        assert!(m2x3.eq(&expected2x3));
        assert!(m3x2.eq(&expected3x2));
        assert!(m2xX.eq(&expected2x3));
        assert!(mXx3.eq(&expected2x3));
        assert!(mXxX.eq(&expected2x3));
    }
}

#[test]
#[rustfmt::skip]
fn new_from_slice_mut() {
    let data = [ 1.0, 2.0,  3.0,  4.0,
                 5.0, 6.0,  7.0,  8.0,
                 9.0, 10.0, 11.0, 12.0 ];
    let expected2 = [ 0.0, 0.0,  0.0,  0.0,
                      5.0, 6.0,  7.0,  8.0,
                      9.0, 10.0, 11.0, 12.0 ];
    let expected3 = [ 0.0, 0.0,  0.0,  0.0,
                      0.0, 0.0,  0.0,  0.0,
                      0.0, 10.0, 11.0, 12.0 ];
    let expected2x3 = [ 0.0, 0.0,  0.0,  0.0,
                        0.0, 0.0,  7.0,  8.0,
                        9.0, 10.0, 11.0, 12.0 ];
    let expected3x2 = [ 0.0, 0.0,  0.0,  0.0,
                        0.0, 0.0,  7.0,  8.0,
                        9.0, 10.0, 11.0, 12.0 ];

    let mut data_mut = data.clone();
    MatrixViewMut2::from_slice(&mut data_mut).fill(0.0);
    assert!(data_mut == expected2);

    let mut data_mut = data.clone();
    MatrixViewMut3::from_slice(&mut data_mut).fill(0.0);
    assert!(data_mut == expected3);

    let mut data_mut = data.clone();
    MatrixViewMut2x3::from_slice(&mut data_mut).fill(0.0);
    assert!(data_mut == expected2x3);

    let mut data_mut = data.clone();
    MatrixViewMut3x2::from_slice(&mut data_mut).fill(0.0);
    assert!(data_mut == expected3x2);

    let mut data_mut = data.clone();
    MatrixViewMut2xX::from_slice(&mut data_mut, 3).fill(0.0);
    assert!(data_mut == expected2x3);

    let mut data_mut = data.clone();
    MatrixViewMutXx3::from_slice(&mut data_mut, 2).fill(0.0);
    assert!(data_mut == expected2x3);

    let mut data_mut = data.clone();
    DMatrixViewMut::from_slice(&mut data_mut, 2, 3).fill(0.0);
    assert!(data_mut == expected2x3);
}

#[test]
#[should_panic]
fn row_out_of_bounds() {
    let a = Matrix3x4::<f32>::zeros();
    a.row(3);
}

#[test]
#[should_panic]
fn rows_out_of_bounds() {
    let a = Matrix3x4::<f32>::zeros();
    a.rows(1, 3);
}

#[test]
#[should_panic]
fn rows_with_step_out_of_bounds() {
    let a = Matrix3x4::<f32>::zeros();
    a.rows_with_step(1, 2, 1);
}

#[test]
#[should_panic]
fn column_out_of_bounds() {
    let a = Matrix3x4::<f32>::zeros();
    a.column(4);
}

#[test]
#[should_panic]
fn columns_out_of_bounds() {
    let a = Matrix3x4::<f32>::zeros();
    a.columns(2, 3);
}

#[test]
#[should_panic]
fn columns_with_step_out_of_bounds() {
    let a = Matrix3x4::<f32>::zeros();
    a.columns_with_step(2, 2, 1);
}

#[test]
#[should_panic]
fn view_out_of_bounds() {
    let a = Matrix3x4::<f32>::zeros();
    a.view((1, 2), (3, 1));
}

#[test]
#[should_panic]
fn view_with_steps_out_of_bounds() {
    let a = Matrix3x4::<f32>::zeros();
    a.view_with_steps((1, 2), (2, 2), (0, 1));
}
