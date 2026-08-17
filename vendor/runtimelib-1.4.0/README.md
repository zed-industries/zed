# runtimelib

`runtimelib` is a Rust library for interacting with Jupyter kernels natively, over ZeroMQ.

## Installation

Runtimelib allows you to pick which async runtime you want to use. If you're using tokio, include the `tokio-runtime` flag. For `async-dispatcher` users (AKA GPUI devs), use `async-dispatcher-runtime`. The async dispatcher runtime is also compatible for smol/async-std users.

### Tokio Users

```toml
[dependencies]
runtimelib = { version = "0.27.0", features = ["tokio-runtime"] }
```

### Async-dispatcher Users

```toml
[dependencies]
runtimelib = { version = "0.27.0", features = ["async-dispatcher-runtime"] }
```

## Key Features

- **Jupyter Kernel Management**: Discover, start, and manage Jupyter kernels.
- **Messaging Protocol**: Implement Jupyter's wire protocol for communication with kernels over ZeroMQ.
- **Flexible Async Runtime**: Support for both Tokio and async-dispatcher runtimes.

## Message Types

For Jupyter message types, traits, and media types, use the [`jupyter-protocol`](https://crates.io/crates/jupyter-protocol) crate directly:

```toml
[dependencies]
jupyter-protocol = "0.3"
```

## Documentation

For more detailed information about the API and its usage, please refer to the [API documentation](https://docs.rs/runtimelib).

## Contributing

We welcome contributions to Runtimelib! If you'd like to contribute, please:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Write tests for your changes
4. Implement your changes
5. Submit a pull request

Please make sure to update tests as appropriate and adhere to the existing coding style.

## License

Runtimelib is distributed under the terms of both the MIT license and the Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
