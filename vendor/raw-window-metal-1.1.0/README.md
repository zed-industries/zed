
<h1 align="center">raw-window-metal</h1>
<p align="center">
    <a href="https://crates.io/crates/raw-window-metal">
      <img src="https://img.shields.io/crates/v/raw-window-metal?style=flat-square" alt = "crates.io">
    </a>
    <a href="https://docs.rs/raw-window-metal">
      <img src="https://docs.rs/raw-window-metal/badge.svg?style=flat-square" alt="docs">
    </a>
    <a href="https://github.com/rust-windowing/raw-window-metal/actions">
      <img src="https://github.com/rust-windowing/raw-window-metal/workflows/macos/badge.svg?style=flat" alt="ci - macos">
    </a>
    <a href="https://github.com/rust-windowing/raw-window-metal/actions">
      <img src="https://github.com/rust-windowing/raw-window-metal/workflows/ios/badge.svg?style=flat" alt="ci - ios">
    </a>
    <br>
    <a href="LICENSE-MIT">
      <img src="https://img.shields.io/badge/license-MIT-green.svg?style=flat-square" alt="License - MIT">
    </a>
    <a href="LICENSE-APACHE">
      <img src="https://img.shields.io/badge/license-APACHE2-green.svg?style=flat-square" alt="License - Apache2">
    </a>
</p>

Interoperability library for Metal and [`raw-window-handle`](https://github.com/rust-windowing/raw-window-handle) for surface creation.

`CAMetalLayer` is the common entrypoint for graphics APIs (e.g `gfx` or `MoltenVK`), but the handles provided by window libraries may not include such a layer.
This library may extract either this layer or allocate a new one.

```console
cargo add raw-window-metal
```

See [the docs](https://docs.rs/raw-window-metal) for examples and further information.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
