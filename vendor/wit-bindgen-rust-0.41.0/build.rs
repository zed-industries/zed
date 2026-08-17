fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // this build script is currently only here so OUT_DIR is set for testing.
}
