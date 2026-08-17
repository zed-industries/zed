use std::str;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // NB: duplicating a workaround in the wasmtime-fiber build script.
    custom_cfg("asan", cfg_is("sanitize", "address"));

    let unix = cfg("unix");
    let windows = cfg("windows");
    let miri = cfg("miri");
    let supported_os = unix || windows;

    // Determine if the current host architecture is supported by Cranelift
    // meaning that we might be executing native code.
    let has_host_compiler_backend = match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" | "riscv64" | "s390x" | "aarch64" => true,
        _ => false,
    };

    let has_native_signals = !miri
        && (supported_os || cfg!(feature = "custom-native-signals"))
        && has_host_compiler_backend;
    let has_virtual_memory = supported_os || cfg!(feature = "custom-virtual-memory");

    custom_cfg("has_native_signals", has_native_signals);
    custom_cfg("has_virtual_memory", has_virtual_memory);
    custom_cfg("has_host_compiler_backend", has_host_compiler_backend);

    // If this OS isn't supported and no debug-builtins or if Cranelift doesn't support
    // the host or there's no need to build these helpers.
    #[cfg(feature = "runtime")]
    if has_host_compiler_backend && (supported_os || cfg!(feature = "debug-builtins")) {
        build_c_helpers();
    }

    // Figure out what to do about Pulley.
    //
    // If the target platform does not have any Cranelift support then Pulley
    // will be used by default. That means that the pulley feature is "enabled"
    // here and the default target is pulley. Note that by enabling the feature
    // here it doesn't actually enable the Cargo feature, it just passes a cfg
    // to rustc. That means that conditional dependencies enabled in
    // `Cargo.toml` (or other features) by `pulley` aren't activated, which is
    // why the `pulley` feature of this crate depends on nothing else.
    let default_target_pulley = !has_host_compiler_backend || miri;
    custom_cfg("default_target_pulley", default_target_pulley);
    if default_target_pulley {
        println!("cargo:rustc-cfg=feature=\"pulley\"");
    }
}

fn cfg(key: &str) -> bool {
    std::env::var(&format!("CARGO_CFG_{}", key.to_uppercase())).is_ok()
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

#[cfg(feature = "runtime")]
fn build_c_helpers() {
    use wasmtime_versioned_export_macros::versioned_suffix;

    let mut build = cc::Build::new();
    build.warnings(true);
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    build.define(&format!("CFG_TARGET_OS_{os}"), None);
    build.define(&format!("CFG_TARGET_ARCH_{arch}"), None);
    build.define("VERSIONED_SUFFIX", Some(versioned_suffix!()));
    if std::env::var("CARGO_FEATURE_DEBUG_BUILTINS").is_ok() {
        build.define("FEATURE_DEBUG_BUILTINS", None);
    }

    // On MinGW targets work around a bug in the MinGW compiler described at
    // https://github.com/bytecodealliance/wasmtime/pull/9688#issuecomment-2573367719
    if cfg("windows") && cfg_is("target_env", "gnu") {
        build.define("__USE_MINGW_SETJMP_NON_SEH", None);
    }

    println!("cargo:rerun-if-changed=src/runtime/vm/helpers.c");
    build.file("src/runtime/vm/helpers.c");
    build.compile("wasmtime-helpers");
}
