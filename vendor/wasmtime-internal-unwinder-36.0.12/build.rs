fn main() {
    // NB: duplicating a workaround in the wasmtime-fiber build script.
    custom_cfg("asan", cfg_is("sanitize", "address"));
}

fn cfg_is(key: &str, val: &str) -> bool {
    std::env::var(&format!("CARGO_CFG_{}", key.to_uppercase()))
        .ok()
        .as_deref()
        == Some(val)
}

fn custom_cfg(key: &str, enabled: bool) {
    println!("cargo:rustc-check-cfg=cfg({key})");
    if enabled {
        println!("cargo:rustc-cfg={key}");
    }
}
