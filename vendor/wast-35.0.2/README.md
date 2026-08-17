<div align="center">
  <h1><code>wast</code></h1>

<strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <strong>A Rust parser for the <a href="https://webassembly.github.io/spec/core/text/index.html">WebAssembly Text Format (WAT)</a>.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/wast"><img src="https://img.shields.io/crates/v/wast.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wast"><img src="https://img.shields.io/crates/d/wast.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/wast/"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
wast = "22.0"
```

The intent of this crate is to provide utilities, combinators, and built-in
types to parse anything that looks like a WebAssembly s-expression.

* Need to parse a `*.wat` file?
* Need to parse a `*.wast` file?
* Need to run test suite assertions from the official wasm test suite?
* Want to write an extension do the WebAssembly text format?

If you'd like to do any of the above this crate might be right for you! You may
also want to check out the `wat` crate which provides a much more stable
interface if all you'd like to do is convert `*.wat` to `*.wasm`.

## Cargo features

By default this crate enables and exports support necessary to parse `*.wat` and
`*.wast` files, or in other words entire wasm modules. If you're using this
crate, however, to parse simply an s-expression wasm-related format (like
`*.witx` or `*.wit` perhaps) then you can disable the default set of features to
only include a lexer, the parsing framework, and a few basic token-related
parsers.

```toml
[dependencies]
wast = { version = "22.0", default-features = false }
```

# License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
