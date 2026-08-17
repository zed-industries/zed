<div align="center">
  <h1><code>wasmprinter</code></h1>

<strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <strong>A Rust parser for printing a WebAssembly binary in the <a href="https://webassembly.github.io/spec/core/text/index.html">WebAssembly Text Format (WAT)</a>.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/wasmprinter"><img src="https://img.shields.io/crates/v/wasmprinter.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wasmprinter"><img src="https://img.shields.io/crates/d/wasmprinter.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/wasmprinter/"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

## Usage

Add `wasmprinter` to your `Cargo.toml`

```sh
$ cargo add wasmprinter
```

You can then convert wasm binaries to strings like so:

```rust
fn main() -> Result<()> {
    let foo_wat = wasmprinter::print_file("path/to/foo.wasm")?;

    let binary = /* ... */;
    let wat = wasmprinter::print_bytes(&binary)?;

    // ...
}
```

## License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
