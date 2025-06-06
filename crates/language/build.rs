fn main() {
    if let Ok(bundled) = std::env::var("codeorbit_BUNDLE") {
        println!("cargo:rustc-env=codeorbit_BUNDLE={}", bundled);
    }
}
