#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

fn main() {
    println!("cargo::rustc-check-cfg=cfg(macos_sdk_26)");

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let output = Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-version"])
            .output()
            .unwrap();

        let sdk_version = String::from_utf8(output.stdout).unwrap();
        let major_version: Option<u32> = sdk_version
            .trim()
            .split('.')
            .next()
            .and_then(|v| v.parse().ok());

        if let Some(major) = major_version
            && major >= 26
        {
            println!("cargo:rustc-cfg=macos_sdk_26");
        }
    }
}
