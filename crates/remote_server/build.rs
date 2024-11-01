use std::process::Command;

const ZED_MANIFEST: &str = include_str!("../zed/Cargo.toml");

fn main() {
    let zed_cargo_toml: cargo_toml::Manifest =
        toml::from_str(ZED_MANIFEST).expect("failed to parse zed Cargo.toml");
    println!(
        "cargo:rustc-env=ZED_PKG_VERSION={}",
        zed_cargo_toml.package.unwrap().version.unwrap()
    );

    // If we're building this for nightly, we want to set the ZED_COMMIT_SHA
    if let Some(release_channel) = std::env::var("ZED_RELEASE_CHANNEL").ok() {
        if release_channel.as_str() == "nightly" {
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

                println!("cargo:rustc-env=ZED_COMMIT_SHA={git_sha}");
            }
        }
    }
}
