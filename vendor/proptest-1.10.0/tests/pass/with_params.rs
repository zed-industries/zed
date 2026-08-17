fn main() {}

#[proptest::property_test(
    config = proptest::test_runner::Config {
        cases: 10,
        ..Default::default()
    }
)]
fn no_trailing_comma(x: i32) {
    assert_eq!(x, x);
}

#[proptest::property_test(
    config = proptest::test_runner::Config {
        cases: 10,
        ..Default::default()
    }
)]
fn trailing_comma(x: i32,) {
    assert_eq!(x, x);
}
