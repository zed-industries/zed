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
    let swift_target = get_swift_target();

    build_bridge(&swift_target);
    link_swift_stdlib(&swift_target);
    link_webrtc_framework(&swift_target);
}

fn build_bridge(swift_target: &SwiftTarget) {
    println!("cargo:rerun-if-changed={}", SWIFT_PACKAGE_NAME);
    let swift_package_root = swift_package_root();
    if !Command::new("swift")
        .args(&["build", "-c", &env::var("PROFILE").unwrap()])
        .current_dir(&swift_package_root)
        .status()
        .unwrap()
        .success()
    {
        panic!(
            "Failed to compile swift package in {}",
            swift_package_root.display()
        );
    }

    println!(
        "cargo:rustc-link-search=native={}",
        swift_target.out_dir_path().display()
    );
    println!("cargo:rustc-link-lib=static={}", SWIFT_PACKAGE_NAME);
}

fn link_swift_stdlib(swift_target: &SwiftTarget) {
    if swift_target.target.libraries_require_rpath {
        panic!("Libraries require RPath! Change minimum MacOS value to fix.")
    }

    swift_target
        .paths
        .runtime_library_paths
        .iter()
        .for_each(|path| {
            println!("cargo:rustc-link-search=native={}", path);
        });
}

fn link_webrtc_framework(swift_target: &SwiftTarget) {
    let swift_out_dir_path = swift_target.out_dir_path();
    println!("cargo:rustc-link-lib=framework=WebRTC");
    println!(
        "cargo:rustc-link-search=framework={}",
        swift_out_dir_path.display()
    );

    let source_path = swift_out_dir_path.join("WebRTC.framework");
    let target_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("../../../WebRTC.framework");
    assert!(
        Command::new("cp")
            .arg("-r")
            .args(&[&source_path, &target_path])
            .status()
            .unwrap()
            .success(),
        "could not copy WebRTC.framework from {:?} to {:?}",
        source_path,
        target_path
    );
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

const SWIFT_PACKAGE_NAME: &'static str = "LiveKitBridge";

fn swift_package_root() -> PathBuf {
    env::current_dir().unwrap().join(SWIFT_PACKAGE_NAME)
}

impl SwiftTarget {
    fn out_dir_path(&self) -> PathBuf {
        swift_package_root()
            .join(".build")
            .join(&self.target.unversioned_triple)
            .join(env::var("PROFILE").unwrap())
    }
}
