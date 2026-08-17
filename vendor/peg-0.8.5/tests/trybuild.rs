fn main() {
    let args: Vec<_> = std::env::args().collect();
    let t = trybuild::TestCases::new();

    t.pass("tests/run-pass/*.rs");

    let expected_rust_ver = env!("CARGO_PKG_RUST_VERSION");
    let run_anyway = args.iter().any(|a| a == "--compile-fail");

    let run_compile_fail = run_anyway || version_check::is_exact_version(expected_rust_ver).unwrap_or(true);
    if run_compile_fail {
        t.compile_fail("tests/compile-fail/*.rs");
    }

    // Trybuild runs the configured tests on drop
    drop(t);

    if !run_compile_fail {
        eprintln!("!!! Skipped compile-fail tests !!!");
        eprintln!("These tests are only checked on rust version {expected_rust_ver} because");
        eprintln!("the error message text may change between compiler versions.");
        eprintln!("");
        eprintln!("Run `cargo +{expected_rust_ver} test` to run these tests.");
        eprintln!("");
    }
}
