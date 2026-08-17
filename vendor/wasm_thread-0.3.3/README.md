# wasm_thread

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/chemicstry/wasm_thread)
[![Cargo](https://img.shields.io/crates/v/wasm_thread.svg)](https://crates.io/crates/wasm_thread)
[![Documentation](https://docs.rs/wasm_thread/badge.svg)](https://docs.rs/wasm_thread)

An `std::thread` replacement for wasm32 target.

This crate tries to closely replicate `std::thread` API. Namely, it doesn't require you to bundle worker scripts and resolves wasm-bindgen shim URL automatically.

Note that some API is still missing and may be even impossible to implement given wasm limitations.

## Using as a library

- Add `wasm_thread` to your `Cargo.toml`.
- This project supports `wasm-pack` targets `web` and `no-modules`. `es_modules` feature is enabled by default, if building for `no-modules`, use `default-features = false` when specifying dependency.
- Replace `use std::thread` with `use wasm_thread as thread`. Note that some API might be missing.
- Build normally using `wasm-pack` or adapt [build_wasm.sh](build_wasm.sh) to your project.

## Notes on wasm limitations

- In order for multiple wasm instances to share the same memory, `SharedArrayBuffer` is required. This means that the COOP and COEP security headers for the webpage will need to be set (see [Mozilla's documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer)). These may be enabled by adjusting webserver settings or using a [service worker](https://github.com/gzuidhof/coi-serviceworker).
- Any blocking API (`thread.join()`, `futures::block_on()`, etc) on the main thread will freeze the browser for as long as lock is maintained. This also freezes any proxied functions, which means that worker spawning, network fetches and other similar asynchronous APIs will block also and can cause a deadlock. To avoid this, either run your `main()` in a worker thread or use async futures.
- Atomic locks (`i32.atomic.wait` to be specific) will panic on the main thread. This means that `mutex.lock()` will likely crash. Solution is the same as above.
- Web workers are normally spawned by providing a script URL, however, to avoid bundling scripts this library uses URL encoded blob [web_worker.js](src/wasm32/js/web_worker.js) to avoid HTTP fetch. `wasm_bindgen` generated `.js` shim script is still needed and a [hack](src/wasm32/js/script_path.js) is used to obtain its URL. If this for some reason does not work in your setup, please report an issue or use `Builder::wasm_bindgen_shim_url()` to specify explicit URL.
- For additional information on wasm threading look at [this](https://rustwasm.github.io/2018/10/24/multithreading-rust-and-wasm.html) blogpost or [raytrace-parallel](https://rustwasm.github.io/wasm-bindgen/examples/raytrace.html) example.

## Alternatives

For a higher-level threading solution, see [wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon), which allows one to utilize a fixed-size threadpool in web browsers. 

## Running examples

### Simple

#### Native

- Just `cargo run --example simple`

#### wasm-bindgen

- Install nightly toolchain and dependencies:
```bash
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo install wasm-bindgen-cli
```
- Build with `./build_wasm.sh` (bash) or `./build_wasm.ps1` (PowerShell). This custom build step is required because prebuilt standard library does not have support for atomics yet. Read more about this [here](https://rustwasm.github.io/2018/10/24/multithreading-rust-and-wasm.html).
- Serve `examples` directory over HTTP with cross-origin isolation enabled and open `simple.html` in the browser. Inspect console output. You can use `cargo install sfz` as a basic HTTP server and serve with `sfz examples --coi`.

#### wasm-pack

- Install `wasm-pack`:
```bash
cargo install wasm-pack
```
- Build with `./examples-wasm-pack/web-build.sh` for an example targeting `web`, and `./examples-wasm-pack/web-build-no-module.sh` for an example targeting `no-modules`.
- Serve `./examples-wasm-pack/module` or `./examples-wasm-pack/no-module`, respectively, over HTTP and open `simple.html` in browser. Inspect console output.

### Example output

Native:
```
hi number 1 from the spawned thread ThreadId(2)!
hi number 1 from the main thread ThreadId(1)!
hi number 1 from the spawned thread ThreadId(3)!
hi number 2 from the main thread ThreadId(1)!
hi number 2 from the spawned thread ThreadId(2)!
hi number 2 from the spawned thread ThreadId(3)!
```

Wasm:
```
hi number 1 from the main thread ThreadId(1)!
hi number 2 from the main thread ThreadId(1)!
hi number 1 from the spawned thread ThreadId(2)!
hi number 1 from the spawned thread ThreadId(3)!
hi number 2 from the spawned thread ThreadId(2)!
hi number 2 from the spawned thread ThreadId(3)!
```

As you can see wasm threads are only spawned after `main()` returns, because browser event loop cannot continue while main thread is blocked.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
