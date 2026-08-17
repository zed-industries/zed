#[cfg(not(miri))]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/integration/ui/compile_fail/*.rs");

    if cfg!(any(feature = "std", feature = "alloc")) {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/integration/ui/compile_fail/std_or_alloc/*.rs");
    }

    if cfg!(feature = "experimental-overwritable") {
        t.compile_fail("tests/integration/ui/compile_fail/overwritable/*.rs");
    }
}
