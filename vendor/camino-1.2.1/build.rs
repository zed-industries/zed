// Copyright (c) The camino Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Adapted from
//! https://github.com/dtolnay/syn/blob/a54fb0098c6679f1312113ae2eec0305c51c7390/build.rs.

use std::{env, process::Command, str};

// The rustc-cfg strings below are *not* public API. Please let us know by
// opening a GitHub issue if your build environment requires some way to enable
// these cfgs other than by executing our build script.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Required by Rust 1.79+.
    println!("cargo:rustc-check-cfg=cfg(doc_cfg)");
    println!("cargo:rustc-check-cfg=cfg(path_buf_deref_mut)");
    println!("cargo:rustc-check-cfg=cfg(try_reserve_2)");
    println!("cargo:rustc-check-cfg=cfg(os_str_bytes)");
    println!("cargo:rustc-check-cfg=cfg(os_string_pathbuf_leak)");
    println!("cargo:rustc-check-cfg=cfg(absolute_path)");

    let compiler = match rustc_version() {
        Some(compiler) => compiler,
        None => return,
    };

    // NOTE:
    // Adding a new cfg gated by Rust version MUST be accompanied by an addition to the matrix in
    // .github/workflows/ci.yml.
    //
    // try_reserve_2 was added in a 1.63 nightly.
    if (compiler.minor >= 63
        && (compiler.channel == ReleaseChannel::Stable || compiler.channel == ReleaseChannel::Beta))
        || compiler.minor >= 64
    {
        println!("cargo:rustc-cfg=try_reserve_2");
    }
    // path_buf_deref_mut was added in a 1.68 nightly.
    if (compiler.minor >= 68
        && (compiler.channel == ReleaseChannel::Stable || compiler.channel == ReleaseChannel::Beta))
        || compiler.minor >= 69
    {
        println!("cargo:rustc-cfg=path_buf_deref_mut");
    }
    // os_str_bytes was added in 1.74.
    if (compiler.minor >= 74 && compiler.channel == ReleaseChannel::Stable) || compiler.minor >= 75
    {
        println!("cargo:rustc-cfg=os_str_bytes");
    }
    // absolute_path was added in 1.79.
    if (compiler.minor >= 79 && compiler.channel == ReleaseChannel::Stable) || compiler.minor >= 80
    {
        println!("cargo:rustc-cfg=absolute_path");
    }
    // os_string_pathbuf_leak was added in 1.89.
    if (compiler.minor >= 89 && compiler.channel == ReleaseChannel::Stable) || compiler.minor >= 90
    {
        println!("cargo:rustc-cfg=os_string_pathbuf_leak");
    }
}

struct Compiler {
    minor: u32,
    channel: ReleaseChannel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReleaseChannel {
    Stable,
    Beta,
    Nightly,
}

fn rustc_version() -> Option<Compiler> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    let minor = pieces.next()?.parse().ok()?;
    let channel = if version.contains("nightly") {
        ReleaseChannel::Nightly
    } else if version.contains("beta") {
        ReleaseChannel::Beta
    } else {
        ReleaseChannel::Stable
    };
    Some(Compiler { minor, channel })
}
