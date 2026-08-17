fn main() {
    println!("cargo:rerun-if-env-changed=RUST_FONTCONFIG_DLOPEN");
    let dlopen = std::env::var("RUST_FONTCONFIG_DLOPEN").is_ok();
    if dlopen {
        println!("cargo:rustc-cfg=feature=\"source-fontconfig-dlopen\"");
    }
}
