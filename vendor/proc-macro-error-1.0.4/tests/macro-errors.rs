extern crate trybuild;

#[cfg_attr(skip_ui_tests, ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
