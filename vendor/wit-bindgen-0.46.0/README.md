<div align="center">
  <h1><code>wit-bindgen</code></h1>

  <p>
    <strong>Guest Rust language bindings generator for
    <a href="https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md">WIT</a>
    and the
    <a href="https://github.com/WebAssembly/component-model">Component Model</a>
    </strong>
  </p>

<strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <img src="https://img.shields.io/badge/rustc-stable+-green.svg" alt="supported rustc stable" />
    <a href="https://docs.rs/wit-bindgen"><img src="https://docs.rs/wit-bindgen/badge.svg" alt="Documentation Status" /></a>
  </p>
</div>

# About

This crate provides a macro, [`generate!`], to automatically generate Rust
bindings for a [WIT] [world]. For more information about this crate see the
[online documentation] which includes some examples and longer form reference
documentation as well.

This crate is developed as a portion of the [`wit-bindgen` repository] which
also contains a CLI and generators for other languages.

[`generate!`]: https://docs.rs/wit-bindgen/latest/wit_bindgen/macro.generate.html
[WIT]: https://component-model.bytecodealliance.org/design/wit.html
[world]: https://component-model.bytecodealliance.org/design/worlds.html
[online documentation]: https://docs.rs/wit-bindgen
[`wit-bindgen` repository]: https://github.com/bytecodealliance/wit-bindgen

# License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
