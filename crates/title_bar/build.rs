#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

fn main() {
    println!("cargo::rustc-check-cfg=cfg(macos_sdk_26)");

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let sdk_version = Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-version"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|v| v.trim().to_string());

        if let Some(version) = sdk_version {
            let major_version: Option<u32> = version.split('.').next().and_then(|v| v.parse().ok());

            if let Some(major) = major_version {
                if major >= 26 {
                    println!("cargo:rustc-cfg=macos_sdk_26");
                }
            }
        }
    }
}
