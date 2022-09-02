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

pub fn link_swift_libs() {
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

    let swift_target_info: SwiftTarget = serde_json::from_slice(&swift_target_info_str).unwrap();
    if swift_target_info.target.libraries_require_rpath {
        panic!("Libraries require RPath! Change minimum MacOS value to fix.")
    }

    swift_target_info
        .paths
        .runtime_library_paths
        .iter()
        .for_each(|path| {
            println!("cargo:rustc-link-search=native={}", path);
        });
}

fn main() {
    link_swift_libs();
    println!("cargo:rerun-if-changed=/Users/as-cii/Library/Developer/Xcode/DerivedData/LiveKitObjC-ftgpxknhsgkrocbhhgjkyyvkgkbj/Build/Products/Debug/libLiveKitObjC.a");
    println!("cargo:rustc-link-search=native=/Users/as-cii/Library/Developer/Xcode/DerivedData/LiveKitObjC-ftgpxknhsgkrocbhhgjkyyvkgkbj/Build/Products/libs");
    println!("cargo:rustc-link-search=framework=/Users/as-cii/Library/Developer/Xcode/DerivedData/LiveKitObjC-ftgpxknhsgkrocbhhgjkyyvkgkbj/Build/Products/frameworks");
    println!("cargo:rustc-link-lib=static=LiveKitObjC");
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=WebRTC");
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=12.3");

    let sdk_path = String::from_utf8(
        Command::new("xcrun")
            .args(&["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let sdk_path = sdk_path.trim_end();

    println!("cargo:rerun-if-changed=src/bindings.h");
    let bindings = bindgen::Builder::default()
        .header("src/bindings.h")
        .clang_arg(format!("-isysroot{}", sdk_path))
        .clang_arg("-xobjective-c")
        .allowlist_function("dispatch_queue_create")
        .allowlist_type("SCStreamOutputType")
        .allowlist_type("SCFrameStatus")
        .allowlist_var("SCStreamFrameInfo.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write dispatch bindings");
}
