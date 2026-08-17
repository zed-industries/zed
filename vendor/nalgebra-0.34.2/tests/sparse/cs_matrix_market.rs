#![cfg_attr(rustfmt, rustfmt_skip)]


use na::io;
use na::DMatrix;

#[test]
fn cs_matrix_market() {
    let file_str = r#"
  %%MatrixMarket matrix coordinate real general
%=================================================================================
%
% This ASCII file represents a sparse MxN matrix with L
% nonzeros in the following Matrix Market format:
%
% +----------------------------------------------+
% |%%MatrixMarket matrix coordinate real general | <--- header line
% |%                                             | <--+
% |% comments                                    |    |-- 0 or more comment lines
% |%                                             | <--+
% |    M  T  L                                   | <--- rows, columns, entries
% |    I1  J1  A(I1, J1)                         | <--+
% |    I2  J2  A(I2, J2)                         |    |
% |    I3  J3  A(I3, J3)                         |    |-- L lines
% |        . . .                                 |    |
% |    IL JL  A(IL, JL)                          | <--+
% +----------------------------------------------+
%
% Indices are 1-based, i.e. A(1,1) is the first element.
%
%=================================================================================
  5  5  8
    1     1   1.000e+00
    2     2   1.050e+01
    3     3   1.500e-02
    1     4   6.000e+00
    4     2   2.505e+02
    4     4  -2.800e+02
    4     5   3.332e+01
    5     5   1.200e+01
"#;

    let cs_mat = io::cs_matrix_from_matrix_market_str(file_str).unwrap();
    let mat: DMatrix<_> = cs_mat.into();
    let expected = DMatrix::from_row_slice(5, 5, &[
        1.0, 0.0,   0.0,   6.0,    0.0,
        0.0, 10.5,  0.0,   0.0,    0.0,
        0.0, 0.0,   0.015, 0.0,    0.0,
        0.0, 250.5, 0.0,   -280.0, 33.32,
        0.0, 0.0,   0.0,   0.0,    12.0,
    ]);

    assert_eq!(mat, expected);
}
