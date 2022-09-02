use serde::Deserialize;
use std::{env, path::PathBuf, process::Command};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwiftTargetInfo {
    pub triple: String,
    pub unversioned_triple: String,
    pub module_triple: String,
    pub swift_runtime_compatibility_version: String,
    #[serde(rename = "librariesRequireRPath")]
    pub libraries_require_rpath: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwiftPaths {
    pub runtime_library_paths: Vec<String>,
    pub runtime_library_import_paths: Vec<String>,
    pub runtime_resource_path: String,
}

#[derive(Debug, Deserialize)]
pub struct SwiftTarget {
    pub target: SwiftTargetInfo,
    pub paths: SwiftPaths,
}

const MACOS_TARGET_VERSION: &str = "12";

fn main() {
    build_bridge();
    link_swift_stdlib();
}

fn build_bridge() {
    let profile = env::var("PROFILE").unwrap();
    let package_name = "LiveKitBridge";
    let package_root = env::current_dir().unwrap().join(package_name);
    if !Command::new("swift")
        .args(&["build", "-c", &profile])
        .current_dir(&package_root)
        .status()
        .unwrap()
        .success()
    {
        panic!(
            "Failed to compile swift package in {}",
            package_root.display()
        );
    }

    let swift_target_info = get_swift_target();
    let swift_out_dir_path = format!(
        "{}/.build/{}/{}",
        package_root.display(),
        swift_target_info.target.unversioned_triple,
        profile
    );

    println!("cargo:rustc-link-search=native={}", swift_out_dir_path);
    println!(
        "cargo:rustc-link-search=framework={}",
        "/Users/nathan/src/zed/crates/live_kit/frameworks"
    );
    println!("cargo:rustc-link-lib=static={}", package_name);
    println!("cargo:rustc-link-lib=framework=WebRTC");
}

fn link_swift_stdlib() {
    let target = get_swift_target();
    if target.target.libraries_require_rpath {
        panic!("Libraries require RPath! Change minimum MacOS value to fix.")
    }

    target.paths.runtime_library_paths.iter().for_each(|path| {
        println!("cargo:rustc-link-search=native={}", path);
    });
}

fn get_swift_target() -> SwiftTarget {
    let mut arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if arch == "aarch64" {
        arch = "arm64".into();
    }
    let target = format!("{}-apple-macosx{}", arch, MACOS_TARGET_VERSION);

    let swift_target_info_str = Command::new("swift")
        .args(&["-target", &target, "-print-target-info"])
        .output()
        .unwrap()
        .stdout;

    serde_json::from_slice(&swift_target_info_str).unwrap()
}
