# wiggle

Wiggle is a code generator for the host side of a `witx` interface. It is
invoked as a Rust procedural macro.

Wiggle is not specialized to any particular WebAssembly runtime. It is usable
in at least Wasmtime and Lucet.

## Learning more

Read the docs on [docs.rs](https://docs.rs/wiggle/).

There are child crates for [integrating with Wasmtime](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wiggle) (this crate), and [Lucet](https://github.com/bytecodealliance/lucet/tree/main/lucet-wiggle).

The [wasi-common crate](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-common) is implemented using Wiggle and the [wasmtime-wasi
crate](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi) integrates wasi-common with the Wasmtime engine.

Andrew Brown wrote a great [blog post](https://bytecodealliance.org/articles/implementing-wasi-nn-in-wasmtime) on using Wiggle with Wasmtime.
