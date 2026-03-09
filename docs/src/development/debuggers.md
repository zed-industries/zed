---
title: Using a debugger
description: "Guide to using a debugger for Zed development."
---

# Using a debugger

> This page is not about [configuring Zed's debugger](../debugger.md).
> It covers how to use a debugger while developing Zed itself.

## Using Zed's built-in debugger

While the Zed project is open you can open the `New Process Modal` and select the `Debug` tab. There you can see two debug configurations to debug Zed with, one for GDB and one for LLDB. Select the configuration you want and Zed will build and launch the binary.

GDB is not supported on Apple Silicon Macs.

## Release build profile considerations

By default, builds using the release profile (the profile used for nightly, preview, and stable) include limited debug info.

This is done by setting the `profile.(release).debug` field in the root `Cargo.toml` field to `"limited"`.

The official documentation for the `debug` field is [here](https://doc.rust-lang.org/cargo/reference/profiles.html#debug).
In short, `"limited"` strips type-level and variable-level debug info.

In release builds, this reduces binary size. Type-level and variable-level debug info is not required for useful stack traces.

However, this data matters when you are actively debugging. Without it, debuggers cannot resolve local variables, inspect values, or format output with pretty-printers.

To get the full debugger experience on a release build, compile a Zed binary with full debug info.

The simplest way is to use `--config` to override the `debug` field in the root `Cargo.toml` when running `cargo run` or `cargo build`:

```sh
cargo run --config 'profile.release.debug="full"'
cargo build --config 'profile.release.debug="full"'
```

> If you do not want to pass `--config` on every `cargo` command, you can also change the section in the [root `Cargo.toml`](https://github.com/zed-industries/zed/blob/main/Cargo.toml)
>
> from
>
> ```toml
> [profile.release]
> debug = "limited"
> ```
>
> to
>
> ```toml
> [profile.release]
> debug = "full"
> ```
>
> This makes all invocations of `cargo run --release` or `cargo build --release` compile with full debug info.
>
> **Warning:** Do not commit these changes.

## Running Zed with a shell debugger GDB/LLDB

### Background

When you install Rust through rustup (the recommended setup for Zed development; see your platform guide [here](../development.md)), rustup also installs helper scripts for debugging Rust binaries.

These scripts are `rust-gdb` and `rust-lldb`.

You can read more about these scripts [here](https://michaelwoerister.github.io/2015/03/27/rust-xxdb.html).

They are wrapper scripts around `gdb` and `lldb` that inject commands and flags for Rust-specific features such as pretty-printers and type info.

To use `rust-gdb` or `rust-lldb`, install `gdb` or `lldb` on your system.

The [linked article](https://michaelwoerister.github.io/2015/03/27/rust-xxdb.html) notes that the minimum supported versions are GDB 7.7 and LLDB 310. In practice, newer versions are usually better.

> **Note**: `rust-gdb` is not installed by default on Windows because `gdb` support there is unstable. Use `rust-lldb` instead.

If you are new to these tools, see the `gdb` docs [here](https://www.gnu.org/software/gdb/) and the `lldb` docs [here](https://lldb.llvm.org/).

### Usage with Zed

After enabling full debug info and building with `cargo build`, run `rust-gdb` or `rust-lldb` against the compiled Zed binary:

```
rust-gdb target/debug/zed
rust-lldb target/debug/zed
```

You can also attach to a running Zed process (for example, one started with `cargo run`):

```
rust-gdb -p <pid>
rust-lldb -p <pid>
```

`<pid>` is the process ID of the Zed instance you want to attach to.

To find the PID, use your system's process tools, such as Task Manager on Windows or Activity Monitor on macOS.

You can also run `ps aux | grep zed` on macOS and Linux, or `Get-Process | Select-Object Id, ProcessName` in PowerShell on Windows.

#### Debugging Panics and Crashes

Debuggers are useful for finding the cause of panics and crashes, including in Zed.

By default, when a process attached to `gdb` or `lldb` hits an exception such as a panic, the debugger stops at that point so you can inspect program state.

The initial stop point is often in Rust standard library panic or exception handling code, so you usually need to walk up the stack to find the root cause.

In `lldb`, use `backtrace` with `frame select`. `gdb` provides equivalent commands.

After the program stops on the exception, you usually cannot continue normal execution. You can still move between stack frames and inspect variables and expressions, which is often enough to identify the crash cause.

You can find additional information on debugging Zed crashes [here](./debugging-crashes.md).
