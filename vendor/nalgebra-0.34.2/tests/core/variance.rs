use nalgebra::DVector;

#[test]
fn test_variance_catastrophic_cancellation() {
    let long_repeating_vector = DVector::repeat(10_000, 100000000.0);
    assert_eq!(long_repeating_vector.variance(), 0.0);

    let short_vec = DVector::from_vec(vec![1., 2., 3.]);
    assert_eq!(short_vec.variance(), 2.0 / 3.0);

    let short_vec =
        DVector::<f64>::from_vec(vec![1.0e8 + 4.0, 1.0e8 + 7.0, 1.0e8 + 13.0, 1.0e8 + 16.0]);
    assert_eq!(short_vec.variance(), 22.5);

    let short_vec =
        DVector::<f64>::from_vec(vec![1.0e9 + 4.0, 1.0e9 + 7.0, 1.0e9 + 13.0, 1.0e9 + 16.0]);
    assert_eq!(short_vec.variance(), 22.5);
}
