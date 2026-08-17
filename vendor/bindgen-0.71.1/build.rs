use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut dst =
        File::create(Path::new(&out_dir).join("host-target.txt")).unwrap();
    dst.write_all(env::var("TARGET").unwrap().as_bytes())
        .unwrap();

    // On behalf of clang_sys, rebuild ourselves if important configuration
    // variables change, to ensure that bindings get rebuilt if the
    // underlying libclang changes.
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");
    println!("cargo:rerun-if-env-changed=LIBCLANG_STATIC_PATH");
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    println!(
        "cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{}",
        env::var("TARGET").unwrap()
    );
    println!(
        "cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{}",
        env::var("TARGET").unwrap().replace('-', "_")
    );
}
