# Welcome to `async-std`

`async-std`, along with its [supporting libraries][organization], is a library making your life in async programming easier. It provides fundamental implementations for downstream libraries and applications alike. The name reflects the approach of this library: it is as closely modeled to the Rust main standard library as possible, replacing all components by async counterparts.

`async-std` provides an interface to all important primitives: filesystem operations, network operations and concurrency basics like timers. It also exposes a `task` in a model similar to the `thread` module found in the Rust standard lib.  But it does not only include I/O primitives, but also `async/await` compatible versions of primitives like `Mutex`.

[organization]: https://github.com/async-rs
