#[test]
pub fn test() {
    let t = trybuild2::TestCases::new();

    t.pass("tests/regex_compiletest/01-not-dangling.rs");
    // The exact error message format changed sometime between 1.70.0 and 1.73.0,
    // so the .stderr file would be incorrect for at least one CI run,
    // so use compile_fail_check_sub instead of compile_fail.
    t.compile_fail_check_sub(
        "tests/regex_compiletest/02-dangling.rs",
        "error[E0505]: cannot move out of `s` because it is borrowed",
    );

    t.pass("tests/regex_compiletest/03-static-value.rs");
    // See above about 02
    t.compile_fail_check_sub(
        "tests/regex_compiletest/04-nonstatic-value.rs",
        "argument requires that `s` is borrowed for `'static`",
    );

    // Don't use check_sub: Check the exact error message to ensure that only the contravariance check fails.
    t.compile_fail("tests/regex_compiletest/05-variance.rs");

    t.pass("tests/regex_compiletest/06-property.rs");
}
