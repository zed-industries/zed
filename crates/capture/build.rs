fn main() {
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=12.3");
    println!("cargo:rustc-link-arg=-ObjC");

    cc::Build::new()
        .file("src/dummy.m")
        .define("MACOSX_DEPLOYMENT_TARGET", "12.3")
        .flag("-ObjC")
        .compile("dummy");
}
