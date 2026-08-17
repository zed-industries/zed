fn main() {}

#[proptest::property_test]
fn my_test(x: i32) {
    assert_eq!(x, x);
}
