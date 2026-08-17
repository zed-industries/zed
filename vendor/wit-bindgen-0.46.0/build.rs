use std::env;
use std::path::PathBuf;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or(String::new());
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or(String::new());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    if target_family != "wasm" {
        return;
    }

    if target_arch != "wasm32" {
        panic!("only wasm32 supports cabi-realloc right now");
    }

    let mut src = env::current_dir().unwrap();
    src.push("src");
    src.push("rt");
    src.push("libwit_bindgen_cabi.a");

    let dst_name = format!(
        "wit_bindgen_cabi{}",
        env!("CARGO_PKG_VERSION").replace(".", "_")
    );
    let dst = out_dir.join(format!("lib{dst_name}.a"));

    std::fs::copy(&src, &dst).unwrap();

    println!("cargo:rustc-link-lib=static={dst_name}");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
}
