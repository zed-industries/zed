<h1 align="center">
  glow
</h1>
<div align="center">
  GL on Whatever: a set of bindings to run GL anywhere (Open GL, OpenGL ES, and WebGL) and avoid target-specific code.
</div>
<br />
<div align="center">
  <a href="https://crates.io/crates/glow"><img src="https://img.shields.io/crates/v/glow.svg?label=glow" alt="crates.io"></a>
  <a href="https://docs.rs/glow"><img src="https://docs.rs/glow/badge.svg" alt="docs.rs"></a>
  <a href="https://github.com/grovesNL/glow/actions"><img src="https://github.com/grovesNL/glow/actions/workflows/ci.yml/badge.svg?branch=main" alt="Build Status" /></a>
  <img src="https://img.shields.io/badge/min%20rust-1.73-green.svg" alt="Minimum Rust Version">
</div>

## Build commands

```sh
# native
cargo build

# web-sys
cargo build --target wasm32-unknown-unknown
```

## License

This project is licensed under any one of [Apache License, Version
2.0](LICENSE-APACHE), [zlib License](LICENSE-ZLIB), or [MIT
license](LICENSE-MIT), at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache 2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
