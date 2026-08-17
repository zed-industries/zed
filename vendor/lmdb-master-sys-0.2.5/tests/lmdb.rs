use std::env;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_lmdb() {
    let mut lmdb = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    lmdb.push("lmdb");
    lmdb.push("libraries");
    lmdb.push("liblmdb");

    let make_cmd = Command::new("make")
        .current_dir(&lmdb)
        .status()
        .expect("failed to execute process");

    assert!(make_cmd.success());

    let make_test_cmd = Command::new("make")
        .arg("test")
        .current_dir(&lmdb)
        .status()
        .expect("failed to execute process");

    assert!(make_test_cmd.success());
}
