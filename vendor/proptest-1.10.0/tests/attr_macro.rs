/// Regression for https://github.com/proptest-rs/proptest/issues/601
#[cfg(feature = "attr-macro")]
#[proptest::property_test]
fn attr_macro_does_not_clobber_mutability(mut x: i32, (mut y, _z): (i32, i32)) {
    let _suppress_unused_warning = x == y;

    x = 0;
    y = 0;
    assert_eq!(x, y);
}

