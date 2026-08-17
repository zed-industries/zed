# zstd

[![Build on Linux](https://github.com/gyscos/zstd-rs/actions/workflows/linux.yml/badge.svg)](https://github.com/gyscos/zstd-rs/actions/workflows/linux.yml)
[![Build on Windows](https://github.com/gyscos/zstd-rs/actions/workflows/windows.yml/badge.svg)](https://github.com/gyscos/zstd-rs/actions/workflows/windows.yml)
[![Build on macOS](https://github.com/gyscos/zstd-rs/actions/workflows/macos.yml/badge.svg)](https://github.com/gyscos/zstd-rs/actions/workflows/macos.yml)
[![crates.io](https://img.shields.io/crates/v/zstd.svg)](https://crates.io/crates/zstd)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

This library is a rust binding for the [zstd compression library][zstd].

# [Documentation][doc]

## 1 - Add to `cargo.toml`

### Using [cargo-edit]

```bash
$ cargo add zstd
```

### Manually

```toml
# Cargo.toml

[dependencies]
zstd = "0.11"
```

## 2 - Usage

This library provides `Read` and `Write` wrappers to handle (de)compression,
along with convenience functions to made common tasks easier.

For instance, `stream::copy_encode` and `stream::copy_decode` are easy-to-use
wrappers around `std::io::copy`. Check the [stream] example:

```rust
use std::io;

// This function use the convenient `copy_encode` method
fn compress(level: i32) {
    zstd::stream::copy_encode(io::stdin(), io::stdout(), level).unwrap();
}

// This function does the same thing, directly using an `Encoder`:
fn compress_manually(level: i32) {
    let mut encoder = zstd::stream::Encoder::new(io::stdout(), level).unwrap();
    io::copy(&mut io::stdin(), &mut encoder).unwrap();
    encoder.finish().unwrap();
}

fn decompress() {
    zstd::stream::copy_decode(io::stdin(), io::stdout()).unwrap();
}
```

# Asynchronous support

The [`async-compression`](https://github.com/Nemo157/async-compression/) crate
provides an async-ready integration of various compression algorithms,
including `zstd-rs`.

# Compile it yourself

`zstd` is included as a submodule. To get everything during your clone, use:

```
git clone https://github.com/gyscos/zstd-rs --recursive
```

Or, if you cloned it without the `--recursive` flag,
call this from inside the repository:

```
git submodule update --init
```

Then, running `cargo build` should take care
of building the C library and linking to it.

# Build-time bindgen

This library includes a pre-generated `bindings.rs` file.
You can also generate new bindings at build-time, using the `bindgen` feature:

```
cargo build --features bindgen
```

# TODO

* Benchmarks, optimizations, ...

# Disclaimer

This implementation is largely inspired by bozaro's [lz4-rs].

# License

* The zstd C library is under a dual BSD/GPLv2 license.
* This zstd-rs binding library is under a [MIT](LICENSE) license.

[zstd]: https://github.com/facebook/zstd
[lz4-rs]: https://github.com/bozaro/lz4-rs
[cargo-edit]: https://github.com/killercup/cargo-edit#cargo-add
[doc]: https://docs.rs/zstd
[stream]: examples/stream.rs
[submodule]: https://git-scm.com/book/en/v2/Git-Tools-Submodules
