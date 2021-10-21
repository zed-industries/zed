use std::{
    env,
    fs::{self, Permissions},
    os::unix::prelude::PermissionsExt,
    process::Command,
};

fn main() {
    let target = env::var("TARGET").unwrap();
    let rust_analyzer_filename = format!("rust-analyzer-{}", target);
    let rust_analyzer_url = format!(
        "https://github.com/rust-analyzer/rust-analyzer/releases/download/2021-10-18/{}.gz",
        rust_analyzer_filename
    );
    println!(
        "cargo:rustc-env=RUST_ANALYZER_FILENAME={}",
        rust_analyzer_filename
    );

    let target_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rust_analyzer_target_path = format!("{}/{}", target_dir, rust_analyzer_filename);
    assert!(
        Command::new("/bin/sh")
            .arg("-c")
            .arg(format!(
                "curl -L {} | gunzip > {}",
                rust_analyzer_url, rust_analyzer_target_path
            ))
            .status()
            .unwrap()
            .success(),
        "failed to download rust-analyzer"
    );
    fs::set_permissions(rust_analyzer_target_path, Permissions::from_mode(0x755))
        .expect("failed to make rust-analyzer executable");
}
