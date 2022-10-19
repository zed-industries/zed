use std::process::Command;

fn main() {
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");

    if let Ok(api_key) = std::env::var("ZED_AMPLITUDE_API_KEY") {
        println!("cargo:rustc-env=ZED_AMPLITUDE_API_KEY={api_key}");
    }

    let output = Command::new("npm")
        .current_dir("../../styles")
        .args(["install", "--no-save"])
        .output()
        .expect("failed to run npm");
    if !output.status.success() {
        panic!(
            "failed to install theme dependencies {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = Command::new("npm")
        .current_dir("../../styles")
        .args(["run", "build"])
        .output()
        .expect("failed to run npm");
    if !output.status.success() {
        panic!(
            "build script failed {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:rerun-if-changed=../../styles/src");
}
