use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rustc-link-search=/Users/as-cii/Library/Developer/Xcode/DerivedData/LiveKitObjC-ftgpxknhsgkrocbhhgjkyyvkgkbj/Build/Products/Debug");
    println!("cargo:rustc-link-lib=static=LiveKitObjC");
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
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
