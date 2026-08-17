// Runs the bogo test suite, in the form of a rust test.
// Note that bogo requires a golang environment to build
// and run.

#[test]
#[cfg(all(coverage, feature = "quic", feature = "dangerous_configuration"))]
fn run_bogo_tests() {
    use std::process::Command;

    let rc = Command::new("./runme")
        .current_dir("../bogo")
        .spawn()
        .expect("cannot run bogo/runme")
        .wait()
        .expect("cannot wait for bogo");

    assert!(rc.success(), "bogo exited non-zero");
}
