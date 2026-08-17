fn some_other_function() {
    // ANCHOR: unused-anchor-that-should-be-stripped
    assert!($TEST_STATUS);
    // ANCHOR_END: unused-anchor-that-should-be-stripped
}

// ANCHOR: rustdoc-include-anchor
fn main() {
    some_other_function();
}
// ANCHOR_END: rustdoc-include-anchor
