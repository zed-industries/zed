use std::{env, path::PathBuf, process::Command};

fn main() {
    // Find WebRTC.framework as a sibling of the executable when running outside of an application bundle
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");

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
