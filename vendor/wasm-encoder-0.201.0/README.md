<div align="center">
  <h1><code>wasm-encoder</code></h1>

<strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <strong>A WebAssembly encoder for Rust.</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/wasm-encoder"><img src="https://img.shields.io/crates/v/wasm-encoder.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wasm-encoder"><img src="https://img.shields.io/crates/d/wasm-encoder.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/wasm-encoder/"><img src="https://img.shields.io/static/v1?label=docs&message=wasm-encoder&color=blue&style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

## Usage

Add `wasm-encoder` to your `Cargo.toml`

```sh
$ cargo add wasm-encoder
```

And then you can encode WebAssembly binaries via:

```rust
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction,
    Module, TypeSection, ValType,
};

let mut module = Module::new();

// Encode the type section.
let mut types = TypeSection::new();
let params = vec![ValType::I32, ValType::I32];
let results = vec![ValType::I32];
types.function(params, results);
module.section(&types);

// Encode the function section.
let mut functions = FunctionSection::new();
let type_index = 0;
functions.function(type_index);
module.section(&functions);

// Encode the export section.
let mut exports = ExportSection::new();
exports.export("f", ExportKind::Func, 0);
module.section(&exports);

// Encode the code section.
let mut codes = CodeSection::new();
let locals = vec![];
let mut f = Function::new(locals);
f.instruction(&Instruction::LocalGet(0));
f.instruction(&Instruction::LocalGet(1));
f.instruction(&Instruction::I32Add);
f.instruction(&Instruction::End);
codes.function(&f);
module.section(&codes);

// Extract the encoded Wasm bytes for this module.
let wasm_bytes = module.finish();

// We generated a valid Wasm module!
assert!(wasmparser::validate(&wasm_bytes).is_ok());
```

# License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
