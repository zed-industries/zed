use std::env;
use wasmtime_versioned_export_macros::versioned_suffix;

fn main() {
    let mut build = cc::Build::new();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // NB: Technically `cfg(sanitize = "address")` is not stable and requires a
    // `#![feature]` but sort of abuse the fact that cfgs are "leaked" through
    // into Cargo ungated via `--print cfg`. Translate that to `cfg(asan)` for
    // us to write down in the code.
    println!("cargo:rustc-check-cfg=cfg(asan)");
    match env::var("CARGO_CFG_SANITIZE") {
        Ok(s) if s == "address" => {
            println!("cargo:rustc-cfg=asan");
        }
        _ => {}
    }

    if os == "windows" {
        println!("cargo:rerun-if-changed=src/windows.c");
        build.file("src/windows.c");
        build.define("VERSIONED_SUFFIX", Some(versioned_suffix!()));
    } else if arch == "s390x" {
        println!("cargo:rerun-if-changed=src/stackswitch/s390x.S");
        build.file("src/stackswitch/s390x.S");
        build.define("VERSIONED_SUFFIX", Some(versioned_suffix!()));
    } else {
        // assume that this is included via inline assembly in the crate itself,
        // and the crate will otherwise have a `compile_error!` for unsupported
        // platforms.
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }
    build.define(&format!("CFG_TARGET_OS_{os}"), None);
    build.define(&format!("CFG_TARGET_ARCH_{arch}"), None);
    build.compile("wasmtime-fiber");
}
