# Using a debugger

> **DISCLAIMER**: This is not documentation for [configuring Zed's debugger](../debugger.md).
> Rather, it is intended to provide information on how to use a debugger while developing Zed itself to both Zed employees and external contributors.

## Using Zed's built-in debugger

While the Zed project is open you can open the `New Process Modal` and select the `Debug` tab. There you can see to debug configurations to debug Zed with, one for GDB and one for LLDB. Select the configuration you want and Zed will build and launch the binary.

Please note, GDB isn't supported on arm Macbooks

## Release build profile considerations

By default, builds using the release profile (release is the profile used for production builds, i.e. nightly, preview, and stable) include limited debug info.

This is done by setting the `profile.(release).debug` field in the root `Cargo.toml` field to `"limited"`.

The official documentation for the `debug` field can be found [here](https://doc.rust-lang.org/cargo/reference/profiles.html#debug).
But the TLDR is that `"limited"` strips type and variable level debug info.

In release builds, this is done to reduce the binary size, as type and variable level debug info is not required, and does not impact the usability of generated stack traces.

However, while the type and variable level debug info is not required for good stack traces, it is very important for a good experience using debuggers,
as without the type and variable level debug info, the debugger has no way to resolve local variables, inspect them, format them using pretty-printers, etc.

Therefore, in order to use a debugger to it's fullest extent when debugging a release build, you must compile a new Zed binary, with full debug info.

The simplest way to do this, is to use the `--config` flag to override the `debug` field in the root `Cargo.toml` file when running `cargo run` or `cargo build` like so:

```sh
cargo run --config 'profile.release.debug="full"'
cargo build --config 'profile.release.debug="full"'
```

> If you wish to avoid passing the `--config` flag on every invocation of `cargo`. You may also change the section in the [root `Cargo.toml`](https://github.com/zed-industries/zed/blob/main/Cargo.toml)
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
> This will ensure all invocations of `cargo run --release` or `cargo build --release` will compile with full debug info.
>
> **WARNING:** Make sure to avoid committing these changes!

## Running Zed with a shell debugger GDB/LLDB

### Background

When installing rust through rustup, (the recommended way to do so when developing Zed, see the documentation for getting started on your platform [here](../development.md))
a few additional scripts are installed and put on your path to assist with debugging binaries compiled with rust.

These are `rust-gdb` and `rust-lldb` respectively.

You can read more information about these scripts and why they are useful [here](https://michaelwoerister.github.io/2015/03/27/rust-xxdb.html) if you are interested.

However, the summary is that they are simple shell scripts that wrap the standard `gdb` and `lldb` commands, injecting the relevant commands and flags to enable additional
rust-specific features such as pretty-printers and type information.

Therefore, in order to use `rust-gdb` or `rust-lldb`, you must have `gdb` or `lldb` installed on your system. If you don't have them installed, you will need to install them in a manner appropriate for your platform.

According to the [previously linked article](https://michaelwoerister.github.io/2015/03/27/rust-xxdb.html), "The minimum supported debugger versions are GDB 7.7 and LLDB 310. However, the general rule is: the newer the better." Therefore, it is recommended to install the latest version of `gdb` or `lldb` if possible.

> **Note**: `rust-gdb` is not installed by default on Windows, as `gdb` support for windows is not very stable. It is recommended to use `lldb` with `rust-lldb` instead on Windows.

If you are unfamiliar with `gdb` or `lldb`, you can learn more about them [here](https://www.gnu.org/software/gdb/) and [here](https://lldb.llvm.org/) respectively.

### Usage with Zed

After following the steps above for including full debug info when compiling Zed,
You can either run `rust-gdb` or `rust-lldb` on the compiled Zed binary after building it with `cargo build`, by running one of the following commands:

```
rust-gdb target/debug/zed
rust-lldb target/debug/zed
```

Alternatively, you can attach to a running instance of Zed (such as an instance of Zed started using `cargo run`) by running one of the following commands:

```
rust-gdb -p <pid>
rust-lldb -p <pid>
```

Where `<pid>` is the process ID of the Zed instance you want to attach to.

To get the process ID of a running Zed instance, you can use your systems process management tools such as `Task Manager` on windows or `Activity Monitor` on macOS.

Alternatively, you can run the `ps aux | grep zed` command on macOS and Linux or `Get-Process | Select-Object Id, ProcessName` in an instance of PowerShell on Windows.

#### Debugging Panics and Crashes

Debuggers can be an excellent tool for debugging the cause of panics and crashes in all programs, including Zed.

By default, when a process that `gdb` or `lldb` is attached to hits an exception such as a panic, the debugger will automatically stop at the point of the panic and allow you to inspect the state of the program.

Most likely, the point at which the debugger stops will be deep in the rust standard library panic or exception handling code, so you will need to navigate up the stack trace to find the actual cause of the panic.

This can be accomplished using the `backtrace` command in combination with the `frame select` command in `lldb`, with similar commands available in `gdb`.

Once the program is stopped, you will not be able to continue execution as you can before an exception is hit. However, you can jump around to different stack frames, and inspect the values of variables and expressions
within each frame, which can be very useful in identifying the root cause of the crash.

You can find additional information on debugging Zed crashes [here](./debugging-crashes.md).
