# clang-sys

[![Crate](https://img.shields.io/crates/v/clang-sys.svg)](https://crates.io/crates/clang-sys)
[![Documentation](https://docs.rs/clang-sys/badge.svg)](https://docs.rs/clang-sys)
[![CI](https://img.shields.io/github/actions/workflow/status/KyleMayes/clang-sys/ci.yml?branch=master)](https://github.com/KyleMayes/clang-sys/actions?query=workflow%3ACI)
![MSRV](https://img.shields.io/badge/MSRV-1.60.0-blue)

Rust bindings for `libclang`.

If you are interested in a somewhat idiomatic Rust wrapper for these bindings, see [`clang-rs`](https://github.com/KyleMayes/clang-rs).

Released under the Apache License 2.0.

## [Documentation](https://docs.rs/clang-sys)

Note that the documentation on https://docs.rs for this crate assumes usage of the `runtime` Cargo feature as well as the Cargo feature for the latest supported version of `libclang` (e.g., `clang_16_0`), neither of which are enabled by default.

Due to the usage of the `runtime` Cargo feature, this documentation will contain some additional types and functions to manage a dynamically loaded `libclang` instance at runtime.

Due to the usage of the Cargo feature for the latest supported version of `libclang`, this documentation will contain constants and functions that are not available in the oldest supported version of `libclang` (3.5). All of these types and functions have a documentation comment which specifies the minimum `libclang` version required to use the item.

## Supported Versions

To target a version of `libclang`, enable a Cargo features such as one of the following:

* `clang_3_5` - requires `libclang` 3.5 or later
* `clang_3_6` - requires `libclang` 3.6 or later
* etc...
* `clang_17_0` - requires `libclang` 17.0 or later
* `clang_18_0` - requires `libclang` 18.0 or later

If you do not enable one of these features, the API provided by `libclang` 3.5 will be available by default.

**Note:** If you are using Clang 15.0 or later, you should enable the `clang_15_0` feature or a more recent version feature. Clang 15.0 introduced [a breaking change to the `EntityKind` enum](https://github.com/llvm/llvm-project/commit/bb83f8e70bd1d56152f02307adacd718cd67e312#diff-674613a0e47f4e66cc19061e28e3296d39be2d124dceefb68237b30b8e241e7c) which resulted in a mismatch between the values returned by `libclang` and the values for `EntityKind` defined by this crate in previous versions.

## Dependencies

By default, this crate will attempt to link to `libclang` dynamically. In this case, this crate depends on the `libclang` shared library (`libclang.so` on Linux, `libclang.dylib` on macOS, `libclang.dll` on Windows). If you want to link to `libclang` statically instead, enable the `static` Cargo feature. In this case, this crate depends on the LLVM and Clang static libraries. If you don't want to link to `libclang` at compiletime but instead want to load it at runtime, enable the `runtime` Cargo feature.

These libraries can be either be installed as a part of Clang or downloaded [here](http://llvm.org/releases/download.html).

**Note:** The downloads for LLVM and Clang 3.8 and later do not include the `libclang.a` static library. This means you cannot link to any of these versions of `libclang` statically unless you build it from source.

### Versioned Dependencies

This crate supports finding versioned instances of `libclang.so` (e.g.,`libclang-3.9.so`). In the case where there are multiple instances to choose from, this crate will prefer instances with higher versions. For example, the following instances of `libclang.so` are listed in descending order of preference:

1. `libclang-4.0.so`
2. `libclang-4.so`
3. `libclang-3.9.so`
4. `libclang-3.so`
5. `libclang.so`

**Note:** On BSD distributions, versioned instances of `libclang.so` matching the pattern `libclang.so.*` (e.g., `libclang.so.7.0`) are also included.

**Note:** On Linux distributions when the `runtime` features is enabled, versioned instances of `libclang.so` matching the pattern `libclang.so.*` (e.g., `libclang.so.1`) are also included.

## Environment Variables

The following environment variables, if set, are used by this crate to find the required libraries and executables:

* `LLVM_CONFIG_PATH` **(compiletime)** - provides a full path to an `llvm-config` executable (including the executable itself [i.e., `/usr/local/bin/llvm-config-8.0`])
* `LIBCLANG_PATH` **(compiletime)** - provides a path to a directory containing a `libclang` shared library or a full path to a specific `libclang` shared library
* `LIBCLANG_STATIC_PATH` **(compiletime)** - provides a path to a directory containing LLVM and Clang static libraries
* `CLANG_PATH` **(runtime)** - provides a path to a `clang` executable

## Linking

### Dynamic

`libclang` shared libraries will be searched for in the following directories:

* the directory provided by the `LIBCLANG_PATH` environment variable
* the `bin` and `lib` directories in the directory provided by `llvm-config --libdir`
* the directories provided by `LD_LIBRARY_PATH` environment variable
* a list of likely directories for the target platform (e.g., `/usr/local/lib` on Linux)
* **macOS only:** the toolchain directory in the directory provided by `xcode-select --print-path`

On Linux, running an executable that has been dynamically linked to `libclang` may require you to add a path to `libclang.so` to the `LD_LIBRARY_PATH` environment variable. The same is true on OS X, except the `DYLD_LIBRARY_PATH` environment variable is used instead.

On Windows, running an executable that has been dynamically linked to `libclang` requires that `libclang.dll` can be found by the executable at runtime. See [here](https://msdn.microsoft.com/en-us/library/7d83bc18.aspx) for more information.

### Static

The availability of `llvm-config` is not optional for static linking. Ensure that an instance of this executable can be found on your system's path or set the `LLVM_CONFIG_PATH` environment variable. The required LLVM and Clang static libraries will be searched for in the same way as shared libraries are searched for, except the `LIBCLANG_STATIC_PATH` environment variable is used in place of the `LIBCLANG_PATH` environment variable.

**Note:** The `libcpp` Cargo feature can be used to enable linking to `libc++` instead of `libstd++` when linking to `libclang` statically on Linux or Haiku.

#### Static Library Availability

Linking to `libclang` statically on *nix systems requires that the `libclang.a` static library be available.  
This library is usually *not* included in most distributions of LLVM and Clang (e.g., `libclang-dev` on Debian-based systems).  
If you need to link to `libclang` statically then most likely the only consistent way to get your hands on `libclang.a` is to build it yourself.

Here's an example of building the required static libraries and using them with `clang-sys`:

```text
git clone git@github.com:llvm/llvm-project.git
cd llvm-project

cmake -S llvm -B build -G Ninja -DLLVM_ENABLE_PROJECTS=clang -DLIBCLANG_BUILD_STATIC=ON
ninja -C build

cd ..
git clone git@github.com:KyleMayes/clang-sys.git
cd clang-sys

LLVM_CONFIG_PATH=../llvm-project/build/bin/llvm-config cargo test --features static
```

Linking to `libclang` statically requires linking a large number of big static libraries.  
Using [`rust-lld` as a linker](https://blog.rust-lang.org/2024/05/17/enabling-rust-lld-on-linux.html) can greatly reduce linking times.

### Runtime

The `clang_sys::load` function is used to load a `libclang` shared library for use in the thread in which it is called. The `clang_sys::unload` function will unload the `libclang` shared library. `clang_sys::load` searches for a `libclang` shared library in the same way one is searched for when linking to `libclang` dynamically at compiletime.
