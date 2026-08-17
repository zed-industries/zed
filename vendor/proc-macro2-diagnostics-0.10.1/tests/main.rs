#[test]
fn main() {
    let t = trybuild::TestCases::new();

    #[cfg(not(nightly_diagnostics))]
    t.compile_fail("tests/stable/*.rs");

    #[cfg(nightly_diagnostics)]
    t.compile_fail("tests/nightly/*.rs");
}
