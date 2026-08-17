// Test handling of test functions that include a return value.

fn return_value(x: i32, y: i32) -> TestCaseResult {
    prop_assert!(x == y)
}
