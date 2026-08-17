```built``` provides a crate with information from the time it was built.

[![Crates.io Version](https://img.shields.io/crates/v/built.svg)](https://crates.io/crates/built)
[![Docs](https://docs.rs/built/badge.svg)](https://docs.rs/built)
[![Clippy, Format & Test](https://github.com/lukaslueg/built/actions/workflows/check.yml/badge.svg)](https://github.com/lukaslueg/built/actions/workflows/check.yml)
[![Downloads](https://img.shields.io/crates/d/built)](https://crates.io/crates/built)

`built` is used as a build-time dependency to collect various information
about the build-environment, serialize this information into Rust-code and
provide that to the crate. The information collected by `built` include:

* Various metadata like version, authors, homepage etc. as set by `Cargo.toml`
* The tag or commit id if the crate was being compiled from within a Git repository.
* The values of `CARGO_CFG_*` build script environment variables, like `CARGO_CFG_TARGET_OS` and `CARGO_CFG_TARGET_ARCH`.
* The features the crate was compiled with.
* The various dependencies, dependencies of dependencies and their versions Cargo ultimately chose to compile.
* The presence of a CI-platform like `Github Actions`, `Travis CI` and `AppVeyor`.
* The compiler and it's version; the documentation-generator and it's version.

See [the example](https://github.com/lukaslueg/built/tree/master/example_project) or the [docs](https://docs.rs/built) for more information.

### MSRV

`built-0.8` itself currently requires Rust 1.81 with all features enabled.

---

```rust,ignore
// In build.rs

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information")
}
```

```rust,ignore
// In lib.rs or main.rs

// Include the generated-file as a separate module
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

println!(
    "This is version {}, built for {} by {}.",
    built_info::PKG_VERSION,
    built_info::TARGET,
    built_info::RUSTC_VERSION
);

match built_info::CI_PLATFORM {
    None => print!("It seems I've not been built on a continuous integration platform,"),
    Some(ci) => print!("I've been built on CI-platform {},", ci),
}

if built::util::detect_ci().is_some() {
    println!(" but I'm currently executing on one!");
} else {
    println!(" and I'm currently not executing on one!");
}

//....
```

> This is version 0.1.0, built for x86_64-unknown-linux-gnu by rustc 1.77.2 (25ef9e3d8 2024-04-09).
>
> I was built from git `0.7.2-3-g4014a2e`, commit 4014a2eb4e8575bec9a1f17ae7b92d8956ecd1fd, short_commit 4014a2e; the working directory was "clean". The branch was `refs/heads/master`.
>
> I was built for a x86_64-CPU, which is a little-endian architecture. I was compiled to run on linux (a unix-breed) and my runtime should be gnu.
>
> It seems I've not been built on a continuous integration platform, and I'm currently not executing on one!
>
> I was built with profile "debug", features "" on 2024-04-14 14:28:31 +00:00 (38 seconds ago) using `built 0.7.2` (with help from `android-tzdata 0.1.1, android_system_properties 0.1.5, autocfg 1.2.0, bitflags 2.5.0, bumpalo 3.16.0, cargo-lock 9.0.0, cc 1.0.94, cfg-if 1.0.0, chrono 0.4.37, core-foundation-sys 0.8.6, equivalent 1.0.1, fixedbitset 0.4.2, form_urlencoded 1.2.1, git2 0.18.3, hashbrown 0.14.3, iana-time-zone 0.1.60, iana-time-zone-haiku 0.1.2, idna 0.5.0, indexmap 2.2.6, jobserver 0.1.30, js-sys 0.3.69, libc 0.2.153, libgit2-sys 0.16.2+1.7.2, libz-sys 1.1.16, log 0.4.21, memchr 2.7.2, num-traits 0.2.18, once_cell 1.19.0, percent-encoding 2.3.1, petgraph 0.6.4, pkg-config 0.3.30, proc-macro2 1.0.79, quote 1.0.36, semver 1.0.22, serde 1.0.197, serde_derive 1.0.197, serde_spanned 0.6.5, syn 2.0.58, tinyvec 1.6.0, tinyvec_macros 0.1.1, toml 0.7.8, toml_datetime 0.6.5, toml_edit 0.19.15, unicode-bidi 0.3.15, unicode-ident 1.0.12, unicode-normalization 0.1.23, url 2.5.0, vcpkg 0.2.15, wasm-bindgen 0.2.92, wasm-bindgen-backend 0.2.92, wasm-bindgen-macro 0.2.92, wasm-bindgen-macro-support 0.2.92, wasm-bindgen-shared 0.2.92, windows-core 0.52.0, windows-targets 0.52.5, windows_aarch64_gnullvm 0.52.5, windows_aarch64_msvc 0.52.5, windows_i686_gnu 0.52.5, windows_i686_gnullvm 0.52.5, windows_i686_msvc 0.52.5, windows_x86_64_gnu 0.52.5, windows_x86_64_gnullvm 0.52.5, windows_x86_64_msvc 0.52.5, winnow 0.5.40`).
