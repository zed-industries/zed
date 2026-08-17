// SPDX-License-Identifier: Apache-2.0

//! Finds `libclang` static or shared libraries and links to them.
//!
//! # Environment Variables
//!
//! This build script can make use of several environment variables to help it
//! find the required static or shared libraries.
//!
//! * `LLVM_CONFIG_PATH` - provides a path to an `llvm-config` executable
//! * `LIBCLANG_PATH` - provides a path to a directory containing a `libclang`
//!    shared library or a path to a specific `libclang` shared library
//! * `LIBCLANG_STATIC_PATH` - provides a path to a directory containing LLVM
//!    and Clang static libraries

#![allow(unused_attributes)]

use std::path::Path;

#[macro_use]
#[path = "build/macros.rs"]
pub mod macros;

#[path = "build/common.rs"]
pub mod common;
#[path = "build/dynamic.rs"]
pub mod dynamic;
#[path = "build/static.rs"]
pub mod r#static;

/// Copies a file.
#[cfg(feature = "runtime")]
fn copy(source: &str, destination: &Path) {
    use std::fs::File;
    use std::io::{Read, Write};

    let mut string = String::new();
    File::open(source)
        .unwrap()
        .read_to_string(&mut string)
        .unwrap();
    File::create(destination)
        .unwrap()
        .write_all(string.as_bytes())
        .unwrap();
}

/// Copies the code used to find and link to `libclang` shared libraries into
/// the build output directory so that it may be used when linking at runtime.
#[cfg(feature = "runtime")]
fn main() {
    use std::env;

    if cfg!(feature = "static") {
        panic!("`runtime` and `static` features can't be combined");
    }

    let out = env::var("OUT_DIR").unwrap();
    copy("build/macros.rs", &Path::new(&out).join("macros.rs"));
    copy("build/common.rs", &Path::new(&out).join("common.rs"));
    copy("build/dynamic.rs", &Path::new(&out).join("dynamic.rs"));
}

/// Finds and links to the required libraries dynamically or statically.
#[cfg(not(feature = "runtime"))]
fn main() {
    if cfg!(feature = "static") {
        r#static::link();
    } else {
        dynamic::link();
    }

    if let Some(output) = common::run_llvm_config(&["--includedir"]) {
        let directory = Path::new(output.trim_end());
        println!("cargo:include={}", directory.display());
    }
}
