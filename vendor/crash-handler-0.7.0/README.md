<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 MD036 -->

<div align="center">

# `🔥 crash-handler`

**Runs user-specified code when a crash occurs**

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/crash-handler.svg)](https://crates.io/crates/crash-handler)
[![Docs](https://docs.rs/crash-handler/badge.svg)](https://docs.rs/crash-handler)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/crash-handling/status.svg)](https://deps.rs/repo/github/EmbarkStudios/crash-handling)
[![Build status](https://github.com/EmbarkStudios/crash-handling/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/crash-handling/actions)

</div>

## Linux/Android

On Linux this is done by handling [signals](https://man7.org/linux/man-pages/man7/signal.7.html), namely the following.

One important detail of the Linux signal handling is that this crate hooks [`pthread_create`](https://man7.org/linux/man-pages/man3/pthread_create.3.html) so that an [alternate signal stack](https://man7.org/linux/man-pages/man2/sigaltstack.2.html) is always installed on every thread. [`std::thread::Thread`] already does this, however hooking `pthread_create` allows us to ensure this occurs for threads created from eg. C/C++ code as well. An alternate stack is necessary to reliably handle a [`SIGSEGV`](#sigsegv) caused by a [stack overflow](https://en.wikipedia.org/wiki/Stack_buffer_overflow), as signals are otherwise handled on the same stack that raised the signal.

### `SIGABRT`

Signal sent to a process to tell it to abort, i.e. to terminate. The signal is usually initiated by the process itself when it calls `std::process::abort` or `libc::abort`, but it can be sent to the process from outside itself like any other signal.

### `SIGBUS`

Signal sent to a process when it causes a [bus error](https://en.wikipedia.org/wiki/Bus_error).

### `SIGFPE`

Signal sent to a process when it executes an erroneous arithmetic operation. Though it stands for **f**loating **p**oint **e**xception this signal covers integer operations as well.

### `SIGILL`

Signal sent to a process when it attempts to execute an **illegal**, malformed, unknown, or privileged, instruction.

### `SIGSEGV`

Signal sent to a process when it makes an invalid virtual memory reference, a [segmentation fault](https://en.wikipedia.org/wiki/Segmentation_fault). This covers infamous `null` pointer access, out of bounds access, use after free, stack overflows, etc.

### `SIGTRAP`

Signal sent to a process when a trap is raised, eg. a breakpoint or debug assertion.

## Windows

On Windows we catch [exceptions](https://docs.microsoft.com/en-us/windows/win32/debug/structured-exception-handling), which cover a wide range of crash reasons, as well as [invalid parameters](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/set-invalid-parameter-handler-set-thread-local-invalid-parameter-handler?view=msvc-170) and [purecall](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/get-purecall-handler-set-purecall-handler?view=msvc-170)

## `MacOS`

On Macos we use [exception ports](https://flylib.com/books/en/3.126.1.109/1/). Exception ports are the first layer that exceptions are filtered, from a thread level, to a process (task) level, and finally to a host level.

If no user ports have been registered, the default Macos implementation is to convert the Mach exception into an equivalent Unix signal and deliver it to any registered signal handlers before performing the default action for the exception/signal (ie process termination). This means that if you use this crate in conjunction with signal handling on `MacOS`, **you will not get the results you expect** as the exception port used by this crate will take precedence over the signal handler. See [this issue](https://github.com/bytecodealliance/wasmtime/issues/2456) for a concrete example.

Note that there is one exception to the above, which is that `SIGABRT` is handled by a signal handler, as there is no equivalent Mach exception for it.

### `EXC_BAD_ACCESS`

Covers similar crashes as [`SIGSEGV`](#sigsegv) and [`SIGBUS`](#sigbus)

### `EXC_BAD_INSTRUCTION`

Covers similar crashes as [`SIGILL`](#sigill)

### `EXC_ARITHMETIC`

Covers similar crashes as [`SIGFPE`](#sigfpe)

### `EXC_BREAKPOINT`

Covers similar crashes as [`SIGTRAP`](#sigtrap)

## Contribution

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](../CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](../CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](../CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in an Embark Studios project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to Embark or any other licensee/user of the contribution.
