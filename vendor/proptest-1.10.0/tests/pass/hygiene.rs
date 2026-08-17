
fn main() {}

struct MyTestArgs {
    something_else: String,
}

#[proptest::property_test]
fn my_test(x: i32) {
    assert_eq!(x, x);
}
