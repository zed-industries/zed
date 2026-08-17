fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-env=DEBUG_OUTPUT_DIR={out_dir}");
}
