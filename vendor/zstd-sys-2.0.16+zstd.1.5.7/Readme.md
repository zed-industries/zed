# zstd-sys

This is the low-level auto-generated binding to the [zstd] library.
You probably don't want to use this library directly; instead, look at [zstd-rs] or [zstd-safe].

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

Then, running `cargo build` in this directory should
take care of building the C library and linking to it.

# Build-time bindgen

This library includes a pre-generated `bindings.rs` file.
You can also generate new bindings at build-time, using the `bindgen` feature:

```
cargo build --features bindgen
```

[zstd]: https://github.com/facebook/zstd
[zstd-rs]: https://github.com/gyscos/zstd-rs
[zstd-safe]: https://github.com/gyscos/zstd-rs/tree/main/zstd-safe
