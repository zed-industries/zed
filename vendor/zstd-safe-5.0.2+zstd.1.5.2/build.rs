fn main() {
    // Force the `std` feature in some cases
    let target_arch =
        std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_arch == "wasm32" || target_os == "hermit" {
        println!("cargo:rustc-cfg=feature=\"std\"");
    }
}
