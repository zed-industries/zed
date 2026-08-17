# Cross-compiling from Linux/Windows

When compiling for e.g. iOS, you're always doing cross-compilation, since the host platform (macOS) is different from the target platform (iOS). This means that Apple's tooling generally has pretty good support for cross-compilation.

To cross-compile Rust applications, you need a few things:
1. A cross-compiler.
  - Rust is a cross-compiler by default, though you'll need to install the standard library for [the relevant target](https://doc.rust-lang.org/rustc/platform-support.html) via. `rustup target add $TARGET`, or use [Cargo's unstable `-Zbuild-std`](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std).
2. A cross-linker.
  - [`ld64`](https://github.com/apple-oss-distributions/ld64) is Apple's native linker. It used to be open source, though the new implementation [hasn't been open sourced](https://developer.apple.com/forums/thread/749558).
  - [LLVM's `lld`](https://lld.llvm.org/) is a fairly good alternative, it comes bundled with Rust as `rust-lld`.
3. Linker stubs (`*.tbd` files).
  - Used to figure out which dynamic system library each symbol comes from.
  - These stubs are located inside the SDK.
4. If compiling C code using e.g. `cc-rs`, you will need:
  - A C cross-compiler (Clang is a good choice here).
  - C headers. Also located inside the SDK.

In the end, you should end up with something like the following:
```console
$ rustup target add aarch64-apple-darwin
$ export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=rust-lld
$ export SDKROOT=$(pwd)/MacOSX.sdk
$ export CC=clang CXX=clang++ AR=llvm-ar # If compiling C code
$ cargo build --target aarch64-apple-darwin
```

For more details, see [Rust Cross](https://github.com/japaric/rust-cross), this has a good explanation of how cross-compiling Rust works in general. [OSXCross](https://github.com/tpoechtrager/osxcross) is also a good resource, it outlines how to create a complete compiler toolchain (including `ld64`).

## SDK

The required `MacOSX.sdk`, `iPhoneOS.sdk` etc. are bundled with Xcode. Please make sure to review [the Xcode SLA](https://www.apple.com/legal/sla/docs/xcode.pdf), especially the sections regarding required use on Apple-branded computers.

Once you've done that, you can [download Xcode here](https://developer.apple.com/download/all/?q=xcode) (requires logging in with an Apple ID). If you only need to cross-compile to macOS, downloading just the Command Line Tools for Xcode will suffice.

Once downloaded, you need to extract and find the `MacOSX.sdk` folder, and pass that to `rustc` using the `SDKROOT` environment variable.

It may also be possible to find these SDKs elsewhere on the web by searching for e.g. "MacOSX.sdk", though we will not endorse any such links here, as the legality of such a re-distribution is a grey area.
