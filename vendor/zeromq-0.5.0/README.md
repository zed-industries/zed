# zmq.rs - A native Rust implementation of ZeroMQ

**DISCLAIMER: This codebase does not implement all of ZeroMQ's feature set.**

ZeroMQ is a high-performance asynchronous messaging library that provides many popular messaging patterns for many transport types. They look and feel like Berkeley style sockets, but are fault tolerant and easier to use. This project aims to provide a native rust alternative to the [reference implementation](https://github.com/zeromq/libzmq), and leverage Rust's async ecosystem.

## Current status

Basic ZMTP implementation is working and tested against the reference implementation.

### Supported transport types:

* TCP
* IPC (unix only)

### Supported socket patterns:

* Request/Response (REQ, REP, DEALER, ROUTER)
* Publish/Subscribe (PUB, SUB, XPUB)
* Pipeline (PUSH, PULL)

## Usage

See the [examples](examples) for some ways to get up and running quickly. You can also generate the documentation by doing `cargo doc --open` on the source code.

### Choosing your async runtime

The project currently supports [`tokio`](tokio.rs), [`async-std`](async.rs), and [`async-dispatcher`](https://github.com/zed-industries/async-dispatcher) controllable via feature flags. `tokio` is used by default. If you want to use `async-std`, you would disable the default features, and then select the `async-std-runtime` feature. For example in your `Cargo.toml`, you might specify the dependency as follows:

```toml
zeromq = { version = "*", default-features = false, features = ["async-std-runtime", "all-transport"] }
```

See the section about feature flags for more info.

### Feature Flags

Feature flags provide a way to customize the functionality provided by this library. Refer to [the cargo guide](https://doc.rust-lang.org/cargo/reference/features.html) for more info.

Features:
- (default) `tokio-runtime`: Use `tokio` as your async runtime.
- `async-std-runtime`: Use `async-std` as your async runtime.
- `async-dispatcher-runtime`: Use `async-dispatcher` as your async runtime.
- (default) `all-transport`: Enable all the `*-transport` flags
- `ipc-transport`: Enable IPC as a transport mechanism
- `tcp-transport`: Enable TCP as a transport mechanism

## Contributing

Contributions are welcome! See our issue tracker for a list of the things we need help with.

## Questions

You can ask quesions in our Discord channel - https://discord.gg/pFXSqWtjQT

## Useful links

* https://rfc.zeromq.org/
* https://rfc.zeromq.org/spec:23/ZMTP/
* https://rfc.zeromq.org/spec:28/REQREP/
* https://rfc.zeromq.org/spec/29/
