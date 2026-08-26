fn main() {
    if let Ok(bundled) = std::env::var("ZED_BUNDLE") {
        println!("cargo:rustc-env=ZED_BUNDLE={}", bundled);
    }
}
