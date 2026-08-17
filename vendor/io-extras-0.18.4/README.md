<div align="center">
  <h1><code>io-extras</code></h1>

  <p>
    <strong>File/socket handle/descriptor utilities</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/io-extras/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/io-extras/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/io-extras"><img src="https://img.shields.io/crates/v/io-extras.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/io-extras"><img src="https://docs.rs/io-extras/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

This crate provides a few miscellaneous utilities related to I/O:

 - `HandleOrSocket` types and traits for Windows, which abstract over Windows
   `*Handle*` and their corresponding Windows `*Socket*` types and traits.

 - `Grip` types and traits, which abstract over the aforementioned Windows
   `HandleOrSocket` types and traits and their corresponding non-Windows `Fd`
   types and traits.

 - `RawReadable` and `RawWritable`, which adapt a raw `Fd`/`Handle` to
   implement the `Read` and `Write` traits, respectively.

 - `ReadWrite` traits, and supporting types, which provide abstractions over
   types with one or two I/O resources, for reading and for writing.

## Minimum Supported Rust Version (MSRV)

This crate currently works on Rust 1.63, when default features are enabled.
Some of the optional features have stricter requirements.
