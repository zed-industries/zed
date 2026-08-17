`page_size_rs` is a Rust crate that provides an easy, fast, cross-platform way to retrieve the memory page size of the current system. It supports any POSIX-compliant system, Windows, and WebAssembly.

[Documentation](https://docs.rs/page_size)

[![Linux Status](https://travis-ci.org/Elzair/page_size_rs.svg?branch=master)](https://travis-ci.org/Elzair/page_size_rs)
[![Build status](https://ci.appveyor.com/api/projects/status/yudj2sx460ywyygn/branch/master?svg=true)](https://ci.appveyor.com/project/Elzair/page_size_rs)

# Introduction

Modern hardware and software tend to load data into RAM (and transfer data from RAM to disk) in discrete chunk called pages. This crate provides a helper method to retrieve the size in bytes of these pages. Since the page size *should not* change during execution, `page_size_rs` will cache the result after it has been called once. 

To make this crate useful for writing memory allocators, it does not require (but can use) the Rust standard library.

Since Windows addresses sometimes have to correspond with an allocation granularity that does not always match the size of the page, I have included a method to retrieve that as well.

# Example

```rust
extern crate page_size;

fn main() {
    println!("{}", page_size::get());
}
```

# Platforms

`page_size_rs` should Work on Windows, any POSIX compatible system (Linux, Mac OSX, etc.), and WebAssembly.

`page_size_rs` is continuously tested on:
  * `x86_64-unknown-linux-gnu` (Linux)
  * `i686-unknown-linux-gnu`
  * `x86_64-unknown-linux-musl` (Linux w/ [MUSL](https://www.musl-libc.org/))
  * `i686-unknown-linux-musl`
  * `x86_64-apple-darwin` (Mac OSX)
  * `i686-apple-darwin`
  * `x86_64-pc-windows-msvc` (Windows)
  * `i686-pc-windows-msvc`
  * `x86_64-pc-windows-gnu`
  * `i686-pc-windows-gnu`

`page_size_rs` is continuously cross-compiled for:
  * `arm-unknown-linux-gnueabihf`
  * `aarch64-unknown-linux-gnu`
  * `mips-unknown-linux-gnu`
  * `aarch64-unknown-linux-musl`
  * `i686-linux-android`
  * `x86_64-linux-android`
  * `arm-linux-androideabi`
  * `aarch64-linux-android`
  * `i386-apple-ios`
  * `x86_64-apple-ios`
  * `i686-unknown-freebsd`
  * `x86_64-unknown-freebsd`
  * `x86_64-unknown-netbsd`
  * `asmjs-unknown-emscripten`
  * `wasm32-wasi`
  * `wasm32-unknown-unknown`
