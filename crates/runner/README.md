# Zed's Plugin Runner
This crate contains a fairly generic interface through which plugins may be added to extend the editor. Currently the intention of this plugin runtime is language server definitions.

Anything that implements the `Runtime` trait may be used as a plugin. Plugin interfaces are declared by implementing the `Interface` trait.

Wasm plugins can be run through `wasmtime`. We plan to add wasi support eventually. We also plan to add macros to generate bindings between Rust plugins compiled to Wasm and the host runtime.