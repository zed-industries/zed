# `wit-component`

`wit-component` is a crate for creating and interacting with WebAssembly
components based on the [component model proposal][model].

## CLI usage

The `wit-component` crate is available through the `wasm-tools` CLI suite under
two subcommands:

```
# Create a component from the input core wasm module
$ wasm-tools component new core.wasm -o component.wasm

# Extract a `*.wit` interface from a component
$ wasm-tools component wit component.wasm
```

## Features

* Creates WebAssembly [component binaries][model] from input core WebAssembly
  modules. Input modules communicate with the [canonical ABI] to imported and
  exported interfaces described with [`*.wit` files][wit]. The wit interface is
  either embedded directly in the core wasm binary.

* Supports "adapters" which can be used to bridge legacy core WebAssembly
  imported functions into [component model][model] functions. Adapters are
  themselves core wasm binaries which will be embedded into the final component.
  An adapter's exports can be imported by the main core wasm binary and the
  adapter can then call component model imports.

* A [`*.wit`][wit] interface can be extracted from an existing component to
  see the interface that it exports and intends to import.

[model]: https://github.com/webassembly/component-model
[canonical ABI]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md

## Usage

Note that this crate is intended to be a low-level detail of tooling for
components. Developers will not necessarily interact with this tooling
day-to-day, instead using wrappers such as
[`cargo-component`](https://github.com/bytecodealliance/cargo-component) which
will automatically execute `wit-component` to produce component binaries.

First `wit-component` supports the wasm-based encoding of a WIT package:

```sh
$ cat demo.wit
package my:demo;

interface host {
  hello: func();
}

world demo {
  import host;
}

$ wasm-tools component wit demo.wit -o demo.wasm --wasm

# The output `demo.wasm` is a valid component binary
$ wasm-tools validate --features component-model demo.wasm
$ wasm-tools print demo.wasm

# The `*.wit` file can be recovered from the `demo.wasm` as well
$ wasm-tools component wit demo.wasm
```

Toolchain authors can use `wit-component` to embed this component types section
into a core wasm binary. For a small demo here a raw `*.wat` wasm text file will
be used where the `demo.wit` argument is specified manually, however.

```sh
$ cat demo.core.wat
(module
  (import "my:demo/host" "hello" (func))
)

$ wasm-tools component embed demo.wit --world demo demo.core.wat -o demo.wasm

# See that there's a new `component-type` custom section
$ wasm-tools objdump demo.wasm

# Convert the core wasm into a component now
$ wasm-tools component new demo.wasm -o demo.component.wasm

# Like before the output `demo.wasm` is a valid component binary
$ wasm-tools validate --features component-model demo.component.wasm
$ wasm-tools print demo.component.wasm

# Additionally like before the `*.wit` interface can still be extracted
$ wasm-tools component wit demo.component.wasm
```

Here the `demo.component.wasm` can now be shipped to a component runtime or
embedded into hosts.
