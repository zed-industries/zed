#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");
    }

    #[cfg(windows)]
    {
        println!("cargo:rerun-if-env-changed=RELEASE_CHANNEL");
        println!("cargo:rerun-if-env-changed=GITHUB_RUN_NUMBER");

        windows_resources::compile(false).expect("failed to compile Windows resources");
    }
}
