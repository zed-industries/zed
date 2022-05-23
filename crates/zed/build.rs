use std::process::Command;

fn main() {
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");

    let output = Command::new("npm")
        .current_dir("../../styles")
        .args(["ci"])
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
        .args(["run", "build-themes"])
        .output()
        .expect("failed to run npm");
    if !output.status.success() {
        panic!(
            "build-themes script failed {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:rerun-if-changed=../../styles");
}
