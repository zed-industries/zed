<div align="center">
  <h1><code>wasmtime</code></h1>

  <p>
    <strong>A standalone runtime for
    <a href="https://webassembly.org/">WebAssembly</a></strong>
  </p>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>
</div>

## About

This crate is the Rust embedding API for the [Wasmtime] project: a
cross-platform engine for running WebAssembly programs. Notable features of
Wasmtime are:

* **Fast**. Wasmtime is built on the optimizing [Cranelift] code generator to
  quickly generate high-quality machine code either at runtime or
  ahead-of-time. Wasmtime's runtime is also optimized for cases such as
  efficient instantiation, low-overhead transitions between the embedder and
  wasm, and scalability of concurrent instances.

* **[Secure]**. Wasmtime's development is strongly focused on the correctness of
  its implementation with 24/7 fuzzing donated by [Google's OSS Fuzz],
  leveraging Rust's API and runtime safety guarantees, careful design of
  features and APIs through an [RFC process], a [security policy] in place
  for when things go wrong, and a [release policy] for patching older versions
  as well. We follow best practices for defense-in-depth and known
  protections and mitigations for issues like Spectre. Finally, we're working
  to push the state-of-the-art by collaborating with academic
  researchers to formally verify critical parts of Wasmtime and Cranelift.

* **[Configurable]**. Wastime supports a rich set of APIs and build time
  configuration to provide many options such as further means of restricting
  WebAssembly beyond its basic guarantees such as its CPU and Memory
  consumption. Wasmtime also runs in tiny environments all the way up to massive
  servers with many concurrent instances.

* **[WASI]**. Wasmtime supports a rich set of APIs for interacting with the host
  environment through the [WASI standard](https://wasi.dev).

* **[Standards Compliant]**. Wasmtime passes the [official WebAssembly test
  suite](https://github.com/WebAssembly/testsuite), implements the [official C
  API of wasm](https://github.com/WebAssembly/wasm-c-api), and implements
  [future proposals to WebAssembly](https://github.com/WebAssembly/proposals) as
  well. Wasmtime developers are intimately engaged with the WebAssembly
  standards process all along the way too.

[Wasmtime]: https://github.com/bytecodealliance/wasmtime
[Cranelift]: https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/README.md
[Google's OSS Fuzz]: https://google.github.io/oss-fuzz/
[security policy]: https://bytecodealliance.org/security
[RFC process]: https://github.com/bytecodealliance/rfcs
[release policy]: https://docs.wasmtime.dev/stability-release.html
[Secure]: https://docs.wasmtime.dev/security.html
[Configurable]: https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html
[WASI]: https://docs.rs/wasmtime-wasi/latest/wasmtime_wasi/
[Standards Compliant]: https://docs.wasmtime.dev/stability-wasm-proposals.html

## Example

An example of using the Wasmtime embedding API for running a small WebAssembly
module might look like:

```rust
use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    let wat = r#"
        (module
            (import "host" "host_func" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
    let module = Module::new(&engine, wat)?;

    // Create a `Linker` which will be later used to instantiate this module.
    // Host functionality is defined by name within the `Linker`.
    let mut linker = Linker::new(&engine);
    linker.func_wrap("host", "host_func", |caller: Caller<'_, u32>, param: i32| {
        println!("Got {} from WebAssembly", param);
        println!("my host state is: {}", caller.data());
    })?;

    // All wasm objects operate within the context of a "store". Each
    // `Store` has a type parameter to store host-specific data, which in
    // this case we're using `4` for.
    let mut store = Store::new(&engine, 4);
    let instance = linker.instantiate(&mut store, &module)?;
    let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;

    // And finally we can call the wasm!
    hello.call(&mut store, ())?;

    Ok(())
}
```

More examples and information can be found in the `wasmtime` crate's [online
documentation](https://docs.rs/wasmtime) as well.

## Documentation

[📚 Read the Wasmtime guide here! 📚][guide]

The [wasmtime guide][guide] is the best starting point to learn about what
Wasmtime can do for you or help answer your questions about Wasmtime. If you're
curious in contributing to Wasmtime, [it can also help you do
that][contributing]!

For information on supported versions of Wasmtime see [the corresponding online book
documentation](https://docs.wasmtime.dev/stability-release.html#current-versions).

[contributing]: https://bytecodealliance.github.io/wasmtime/contributing.html
[guide]: https://bytecodealliance.github.io/wasmtime
