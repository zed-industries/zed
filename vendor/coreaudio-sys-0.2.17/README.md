# coreaudio-sys [![Actions Status](https://github.com/rustaudio/coreaudio-sys/workflows/coreaudio-sys/badge.svg)](https://github.com/rustaudio/coreaudio-sys/actions) [![Crates.io](https://img.shields.io/crates/v/coreaudio-sys.svg)](https://crates.io/crates/coreaudio-sys) [![Crates.io](https://img.shields.io/crates/l/coreaudio-sys.svg)](https://github.com/RustAudio/coreaudio-sys/blob/master/LICENSE) [![docs.rs](https://docs.rs/coreaudio-sys/badge.svg)](https://docs.rs/coreaudio-sys/)

Raw bindings to Apple's Core Audio API for macos and iOS generated using [rust-bindgen](https://github.com/rust-lang-nursery/rust-bindgen). [coreaudio-rs](https://github.com/RustAudio/coreaudio-rs) is an attempt at offering a higher level API around this crate.

## Cross Compiling

[Rust Cross](https://github.com/japaric/rust-cross) has a good explanation of how cross-compiling Rust works in general. While the author of Rust Cross advises against it, it is perfectly possible to cross-compile Rust for MacOS on Linux. [OSXCross](https://github.com/tpoechtrager/osxcross) can be used to create a compiler toolchain that can compile for MacOS on Linux.

### Environment Variables

When cross-compiling for MacOS on Linux there are two environment variables that are used to configure how `coreaudio-sys` finds the required headers and libraries. The following examples assume that you have OSXCross installed at `/build/osxcross`.

#### `COREAUDIO_SDK_PATH`

This tell `coreaudio-sys` where to find the MacOS SDK:

```bash
export COREAUDIO_SDK_PATH=/build/osxcross/target/SDK/MacOSX10.11.sdk
```
