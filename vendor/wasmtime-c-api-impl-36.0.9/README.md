# Wasmtime's C/C++ API

## API Documentation

[The API documentation for the Wasmtime C library is hosted
here.](https://bytecodealliance.github.io/wasmtime/c-api/).

## Using in a C/C++ Project

### Using a Pre-Built Static or Dynamic Library

Each release on Wasmtime's [GitHub Releases
page](https://github.com/bytecodealliance/wasmtime/releases) has pre-built
binaries for both static and dynamic libraries for a variety of architectures
and operating systems attached, as well as header files you can include.

### Building Wasmtime's C/C++ API from Source

To use Wasmtime from a C or C++ project, you must have
[CMake](https://cmake.org/) and [a Rust
toolchain](https://www.rust-lang.org/tools/install) installed.

From the root of the Wasmtime repository, run the following commands:

```
$ cmake -S crates/c-api -B target/c-api --install-prefix "$(pwd)/artifacts"
$ cmake --build target/c-api
$ cmake --install target/c-api
```

These commands will produce the following files:

* `artifacts/lib/libwasmtime.{a,lib}`: Static Wasmtime library. Exact extension
  depends on your operating system.

* `artifacts/lib/libwasmtime.{so,dylib,dll}`: Dynamic Wasmtime library. Exact
  extension depends on your operating system.

* `artifacts/include/**.{h,hh}`: Header files for working with Wasmtime.

## Using in a C++ Project

A header only C++ API is also offered as `wasmtime.hh`, which is built on top
of the C API. Its located next to the C headers when you download a pre-built
library, or when building from source. C++17 is required.

## Using in a Rust Project

If you have a Rust crate that contains bindings to a C or C++ library that uses Wasmtime, you can link the Wasmtime C API using Cargo.

1. Add a dependency on the `wasmtime-c-api-impl` crate to your `Cargo.toml`. Note that package name differs from the library name.

```toml
[dependencies]
wasmtime-c-api = { version = "16.0.0", package = "wasmtime-c-api-impl" }
```

2. In your `build.rs` file, when compiling your C/C++ source code, add the C `wasmtime-c-api` headers to the include path:

```rust
fn main() {
    let mut cfg = cc::Build::new();

    // Add to the include path the wasmtime headers and the standard
    // Wasm C API headers.
    cfg
        .include(std::env::var("DEP_WASMTIME_C_API_INCLUDE").unwrap());

    // Compile your C code.
    cfg
        .file("src/your_c_code.c")
        .compile("your_library");
}
```

## Testing

Running tests for the C API can be done using cmake from the root of the repo:

```sh
$ cmake -S examples -B examples/build -DBUILD_SHARED_LIBS=OFF -DBUILD_TESTS=ON
$ cmake --build examples/build --config Debug
$ CTEST_OUTPUT_ON_FAILURE=1 cmake --build examples/build --config Debug --target test
```
