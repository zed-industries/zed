use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=System");
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=12.3");
    println!("cargo:rustc-link-arg=-ObjC");

    let bindings = bindgen::Builder::default()
        .header("src/bindings.h")
        .clang_arg("-isysroot/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk")
        .allowlist_function("CMTimeMake")
        .allowlist_type("CMSampleBufferRef")
        .allowlist_var("_dispatch_main_q")
        .allowlist_function("dispatch_async_f")
        .allowlist_function("dispatch_queue_create")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write dispatch bindings");

    cc::Build::new()
        .file("src/dummy.m")
        .flag("-mmacosx-version-min=12.3")
        .compile("dummy");
}
