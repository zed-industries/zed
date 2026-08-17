#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[test]
fn expand() {
    macrotest::expand("tests/expand/*.rs");
}
