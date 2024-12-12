use serde::Deserialize;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

const SWIFT_PACKAGE_NAME: &str = "LiveKitBridge";

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

const MACOS_TARGET_VERSION: &str = "10.15.7";

fn main() {
    if cfg!(all(
        target_os = "macos",
        not(any(test, feature = "test-support", feature = "no-webrtc")),
    )) {
        let swift_target = get_swift_target();

        build_bridge(&swift_target);
        link_swift_stdlib(&swift_target);
        link_webrtc_framework(&swift_target);

        // Register exported Objective-C selectors, protocols, etc when building example binaries.
        println!("cargo:rustc-link-arg=-Wl,-ObjC");
    }
}

fn build_bridge(swift_target: &SwiftTarget) {
    println!("cargo:rerun-if-env-changed=MACOSX_DEPLOYMENT_TARGET");
    println!("cargo:rerun-if-changed={}/Sources", SWIFT_PACKAGE_NAME);
    println!(
        "cargo:rerun-if-changed={}/Package.swift",
        SWIFT_PACKAGE_NAME
    );
    println!(
        "cargo:rerun-if-changed={}/Package.resolved",
        SWIFT_PACKAGE_NAME
    );

    let swift_package_root = swift_package_root();
    let swift_target_folder = swift_target_folder();
    let swift_cache_folder = swift_cache_folder();
    if !Command::new("swift")
        .arg("build")
        .arg("--disable-automatic-resolution")
        .args(["--configuration", &env::var("PROFILE").unwrap()])
        .args(["--triple", &swift_target.target.triple])
        .args(["--build-path".into(), swift_target_folder])
        .args(["--cache-path".into(), swift_cache_folder])
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
    for path in &swift_target.paths.runtime_library_paths {
        println!("cargo:rustc-link-search=native={}", path);
    }
}

fn link_webrtc_framework(swift_target: &SwiftTarget) {
    let swift_out_dir_path = swift_target.out_dir_path();
    println!("cargo:rustc-link-lib=framework=WebRTC");
    println!(
        "cargo:rustc-link-search=framework={}",
        swift_out_dir_path.display()
    );
    // Find WebRTC.framework as a sibling of the executable when running tests.
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
    // Find WebRTC.framework in parent directory of the executable when running examples.
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/..");

    let source_path = swift_out_dir_path.join("WebRTC.framework");
    let deps_dir_path =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("../../../deps/WebRTC.framework");
    let target_dir_path =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("../../../WebRTC.framework");
    copy_dir(&source_path, &deps_dir_path);
    copy_dir(&source_path, &target_dir_path);
}

fn get_swift_target() -> SwiftTarget {
    let mut arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if arch == "aarch64" {
        arch = "arm64".into();
    }
    let target = format!("{}-apple-macosx{}", arch, MACOS_TARGET_VERSION);

    let swift_target_info_str = Command::new("swift")
        .args(["-target", &target, "-print-target-info"])
        .output()
        .unwrap()
        .stdout;

    serde_json::from_slice(&swift_target_info_str).unwrap()
}

fn swift_package_root() -> PathBuf {
    env::current_dir().unwrap().join(SWIFT_PACKAGE_NAME)
}

fn swift_target_folder() -> PathBuf {
    let target = env::var("TARGET").unwrap();
    env::current_dir()
        .unwrap()
        .join(format!("../../target/{target}/{SWIFT_PACKAGE_NAME}_target"))
}

fn swift_cache_folder() -> PathBuf {
    let target = env::var("TARGET").unwrap();
    env::current_dir()
        .unwrap()
        .join(format!("../../target/{target}/{SWIFT_PACKAGE_NAME}_cache"))
}

fn copy_dir(source: &Path, destination: &Path) {
    assert!(
        Command::new("rm")
            .arg("-rf")
            .arg(destination)
            .status()
            .unwrap()
            .success(),
        "could not remove {:?} before copying",
        destination
    );

    assert!(
        Command::new("cp")
            .arg("-R")
            .args([source, destination])
            .status()
            .unwrap()
            .success(),
        "could not copy {:?} to {:?}",
        source,
        destination
    );
}

impl SwiftTarget {
    fn out_dir_path(&self) -> PathBuf {
        swift_target_folder()
            .join(&self.target.unversioned_triple)
            .join(env::var("PROFILE").unwrap())
    }
}
