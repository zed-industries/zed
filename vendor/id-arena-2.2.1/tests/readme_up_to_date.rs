use std::fs;
use std::process::Command;

#[test]
fn cargo_readme_up_to_date() {
    println!("Checking that `cargo readme > README.md` is up to date...");

    let expected = Command::new("cargo")
        .arg("readme")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("should run `cargo readme` OK")
        .stdout;
    let expected = String::from_utf8_lossy(&expected);

    let actual = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))
        .expect("should read README.md OK");

    if actual != expected {
        panic!("Run `cargo readme > README.md` to update README.md");
    }
}
