fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");

        println!("cargo:rerun-if-env-changed=ZED_BUNDLE");
    }
}
