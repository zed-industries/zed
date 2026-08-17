fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or(String::new());
    let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or(String::new());

    if target_family != "wasm" {
        return;
    }

    if target_arch != "wasm32" {
        panic!("only wasm32 supports cabi-realloc right now");
    }

    println!("cargo:rustc-link-lib=wit_bindgen_cabi_realloc");
    let cwd = std::env::current_dir().unwrap();
    let cwd = cwd.display();
    println!("cargo:rustc-link-search=native={cwd}/src");
}
