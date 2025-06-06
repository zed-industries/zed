use std::process::Command;

const codeorbit_MANIFEST: &str = include_str!("../CodeOrbit/Cargo.toml");

fn main() {
    let codeorbit_cargo_toml: cargo_toml::Manifest =
        toml::from_str(codeorbit_MANIFEST).expect("failed to parse CodeOrbit Cargo.toml");
    println!(
        "cargo:rustc-env=codeorbit_PKG_VERSION={}",
        codeorbit_cargo_toml.package.unwrap().version.unwrap()
    );
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );

    // Populate git sha environment variable if git is available
    println!("cargo:rerun-if-changed=../../.git/logs/HEAD");
    if let Some(output) = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
    {
        let git_sha = String::from_utf8_lossy(&output.stdout);
        let git_sha = git_sha.trim();

        println!("cargo:rustc-env=codeorbit_COMMIT_SHA={git_sha}");
    }
}
