use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=System");
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=12.3");
    println!("cargo:rustc-link-arg=-ObjC");

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
        .allowlist_function("CMTimeMake")
        .allowlist_type("SCStreamOutputType")
        .allowlist_type("SCFrameStatus")
        .allowlist_var("SCStreamFrameInfo.*")
        .allowlist_function("dispatch_queue_create")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write dispatch bindings");

    println!("cargo:rerun-if-changed=src/dummy.m");
    cc::Build::new()
        .file("src/dummy.m")
        .flag("-mmacosx-version-min=12.3")
        .compile("dummy");
}
