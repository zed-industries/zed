#![cfg(feature = "__NOT_PUBLIC__nightly_testing")]

#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile-pass/*.rs");
    t.compile_fail("tests/compile-fail/*.rs");
}
