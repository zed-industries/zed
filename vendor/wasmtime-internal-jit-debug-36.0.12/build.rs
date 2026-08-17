use wasmtime_versioned_export_macros::versioned_suffix;

fn main() {
    if !cfg!(feature = "gdb_jit_int") {
        return;
    }

    let mut build = cc::Build::new();
    build.warnings(true);
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    build.define(&format!("CFG_TARGET_OS_{os}"), None);
    build.define("VERSIONED_SUFFIX", Some(versioned_suffix!()));

    println!("cargo:rerun-if-changed=gdbjit.c");
    build.file("gdbjit.c");
    build.compile("gdbjit-helpers");
}
