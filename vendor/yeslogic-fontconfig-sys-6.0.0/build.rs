fn main() {
    println!("cargo:rerun-if-env-changed=RUST_FONTCONFIG_DLOPEN");
    let dlopen = std::env::var_os("RUST_FONTCONFIG_DLOPEN").is_some();
    if dlopen {
        println!("cargo:rustc-cfg=feature=\"dlopen\"");
    }
    if !(dlopen || cfg!(feature = "dlopen")) {
        pkg_config::find_library("fontconfig").unwrap();
    }
}
