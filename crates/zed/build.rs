use std::process::Command;

fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");

        println!("cargo:rerun-if-env-changed=ZED_BUNDLE");
        if std::env::var("ZED_BUNDLE").ok().as_deref() == Some("true") {
            // Find WebRTC.framework in the Frameworks folder when running as part of an application bundle.
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Frameworks");
        } else {
            // Find WebRTC.framework as a sibling of the executable when running outside of an application bundle.
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        }

        // Weakly link ReplayKit to ensure Zed can be used on macOS 10.15+.
        println!("cargo:rustc-link-arg=-Wl,-weak_framework,ReplayKit");

        // Seems to be required to enable Swift concurrency
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

        // Register exported Objective-C selectors, protocols, etc
        println!("cargo:rustc-link-arg=-Wl,-ObjC");
    }

    // Populate git sha environment variable if git is available
    println!("cargo:rerun-if-changed=../../.git/logs/HEAD");
    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        if output.status.success() {
            let git_sha = String::from_utf8_lossy(&output.stdout);
            let git_sha = git_sha.trim();

            println!("cargo:rustc-env=ZED_COMMIT_SHA={git_sha}");

            if let Ok(build_profile) = std::env::var("PROFILE") {
                if build_profile == "release" {
                    // This is currently the best way to make `cargo build ...`'s build script
                    // to print something to stdout without extra verbosity.
                    println!(
                        "cargo:warning=Info: using '{git_sha}' hash for ZED_COMMIT_SHA env var"
                    );
                }
            }
        }
    }
}
