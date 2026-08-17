use nalgebra::{dmatrix, dvector, matrix, point, vector};

#[test]
fn sanity_test() {
    // The macros are already tested in `nalgebra-macros`. Here we just test that they compile fine.

    let _ = matrix![1, 2, 3; 4, 5, 6];
    let _ = dmatrix![1, 2, 3; 4, 5, 6];
    let _ = point![1, 2, 3, 4, 5, 6];
    let _ = vector![1, 2, 3, 4, 5, 6];
    let _ = dvector![1, 2, 3, 4, 5, 6];
}
