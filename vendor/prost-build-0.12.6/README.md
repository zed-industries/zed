[![Documentation](https://docs.rs/prost-build/badge.svg)](https://docs.rs/prost-build/)
[![Crate](https://img.shields.io/crates/v/prost-build.svg)](https://crates.io/crates/prost-build)

# `prost-build`

`prost-build` makes it easy to generate Rust code from `.proto` files as part of
a Cargo build. See the crate [documentation](https://docs.rs/prost-build/) for examples
of how to integrate `prost-build` into a Cargo project.

## `protoc`

`prost-build` uses `protoc` to parse the proto files. There are two ways to make `protoc`
available for `prost-build`:

* Include `protoc` in your `PATH`. This can be done by following the [`protoc` install instructions].
* Pass the `PROTOC=<my/path/to/protoc>` environment variable with the path to
  `protoc`.

[`protoc` install instructions]: https://github.com/protocolbuffers/protobuf#protocol-compiler-installation

## License

`prost-build` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](../LICENSE) for details.

Copyright 2017 Dan Burkert
