use std::process::Stdio;

#[test]
fn no_std() {
    assert!(::std::process::Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../renamed_num_enum/Cargo.toml",
            ),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap()
        .success())
}

#[test]
fn std() {
    assert!(::std::process::Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../renamed_num_enum/Cargo.toml",
            ),
            "--features",
            "std",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap()
        .success())
}
