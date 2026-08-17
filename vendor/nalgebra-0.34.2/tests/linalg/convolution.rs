use na::{DVector, Vector2, Vector3, Vector4, Vector5};
use std::panic;

//
// Should mimic calculations in Python's scipy library
// >>>from scipy.signal import convolve
//

// >>> convolve([1,2,3,4],[1,2],"same")
// array([ 1,  4,  7, 10])
#[test]
fn convolve_same_check() {
    // Static Tests
    let actual_s = Vector4::new(1.0, 4.0, 7.0, 10.0);
    let expected_s = Vector4::new(1.0, 2.0, 3.0, 4.0).convolve_same(Vector2::new(1.0, 2.0));

    assert!(relative_eq!(actual_s, expected_s, epsilon = 1.0e-7));

    // Dyn Tests
    let actual_d = DVector::from_vec(vec![1.0, 4.0, 7.0, 10.0]);
    let expected_d = DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0])
        .convolve_same(DVector::from_vec(vec![1.0, 2.0]));

    assert!(relative_eq!(actual_d, expected_d, epsilon = 1.0e-7));

    // Panic Tests
    // These really only apply to dynamic sized vectors
    assert!(
        panic::catch_unwind(|| {
            let _ = DVector::from_vec(vec![1.0, 2.0])
                .convolve_same(DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0]));
        })
        .is_err()
    );

    assert!(
        panic::catch_unwind(|| {
            let _ = DVector::<f32>::from_vec(vec![])
                .convolve_same(DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0]));
        })
        .is_err()
    );

    assert!(
        panic::catch_unwind(|| {
            let _ = DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0])
                .convolve_same(DVector::<f32>::from_vec(vec![]));
        })
        .is_err()
    );
}

// >>> convolve([1,2,3,4],[1,2],"full")
// array([ 1, 4,  7, 10, 8])
#[test]
fn convolve_full_check() {
    // Static Tests
    let actual_s = Vector5::new(1.0, 4.0, 7.0, 10.0, 8.0);
    let expected_s = Vector4::new(1.0, 2.0, 3.0, 4.0).convolve_full(Vector2::new(1.0, 2.0));

    assert!(relative_eq!(actual_s, expected_s, epsilon = 1.0e-7));

    // Dyn Tests
    let actual_d = DVector::from_vec(vec![1.0, 4.0, 7.0, 10.0, 8.0]);
    let expected_d = DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0])
        .convolve_full(DVector::from_vec(vec![1.0, 2.0]));

    assert!(relative_eq!(actual_d, expected_d, epsilon = 1.0e-7));

    // Panic Tests
    // These really only apply to dynamic sized vectors
    assert!(
        panic::catch_unwind(|| {
            DVector::from_vec(vec![1.0, 2.0])
                .convolve_full(DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0]));
        })
        .is_err()
    );

    assert!(
        panic::catch_unwind(|| {
            DVector::<f32>::from_vec(vec![])
                .convolve_full(DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0]));
        })
        .is_err()
    );

    assert!(
        panic::catch_unwind(|| {
            DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0])
                .convolve_full(DVector::<f32>::from_vec(vec![]));
        })
        .is_err()
    );
}

// >>> convolve([1, 2, 3, 4],[1, 2],"valid")
// array([4, 7, 10])
#[test]
fn convolve_valid_check() {
    // Static Tests
    let actual_s = Vector3::from_vec(vec![4.0, 7.0, 10.0]);
    let expected_s = Vector4::new(1.0, 2.0, 3.0, 4.0).convolve_valid(Vector2::new(1.0, 2.0));

    assert!(relative_eq!(actual_s, expected_s, epsilon = 1.0e-7));

    // Dyn Tests
    let actual_d = DVector::from_vec(vec![4.0, 7.0, 10.0]);
    let expected_d = DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0])
        .convolve_valid(DVector::from_vec(vec![1.0, 2.0]));

    assert!(relative_eq!(actual_d, expected_d, epsilon = 1.0e-7));

    // Panic Tests
    // These really only apply to dynamic sized vectors
    assert!(
        panic::catch_unwind(|| {
            DVector::from_vec(vec![1.0, 2.0])
                .convolve_valid(DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0]));
        })
        .is_err()
    );

    assert!(
        panic::catch_unwind(|| {
            DVector::<f32>::from_vec(vec![])
                .convolve_valid(DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0]));
        })
        .is_err()
    );

    assert!(
        panic::catch_unwind(|| {
            DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0])
                .convolve_valid(DVector::<f32>::from_vec(vec![]));
        })
        .is_err()
    );
}
