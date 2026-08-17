fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or(String::new()) == "aarch64" {
        match autocfg::AutoCfg::new() {
            // From 1.61 aarch64 intrinsics and #[target_feature]
            Ok(ac) => if ac.probe_rustc_version(1, 61) {
                println!("cargo:rustc-cfg=has_aarch64_simd");
            }
            Err(err) => println!("cargo:warning={}", err),
        }
    }
}
