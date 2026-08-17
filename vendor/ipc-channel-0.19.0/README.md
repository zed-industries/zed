# ipc-channel

📚 [Documentation](https://docs.rs/ipc-channel) 📚

## Overview

`ipc-channel` is an implementation of the Rust channel API (a form of communicating sequential processes, CSP) over the native OS abstractions. Under the hood, this API uses Mach ports on the Mac and file descriptor passing over Unix sockets on Linux. The `serde` library is used to serialize values for transport over the wire.

As much as possible, `ipc-channel` has been designed to be a drop-in replacement for Rust channels. The mapping from the Rust channel APIs to `ipc-channel` APIs is as follows:

* `channel()` → `ipc::channel().unwrap()`
* `Sender<T>` → `ipc::IpcSender<T>` (requires `T: Serialize`)
* `Receiver<T>` → `ipc::IpcReceiver<T>` (requires `T: Deserialize`)

Note that both `IpcSender<T>` and `IpcReceiver<T>` implement `Serialize` and `Deserialize`, so you can send IPC channels over IPC channels freely, just as you can with Rust channels.

The easiest way to make your types implement `Serialize` and `Deserialize` is to use the `serde_macros` crate from crates.io as a plugin and then annotate the types you want to send with `#[derive(Deserialize, Serialize])`. In many cases, that's all you need to do—the compiler generates all the tedious boilerplate code needed to save and restore instances of your types.

In order to bootstrap an IPC connection across processes, you create an instance of the `IpcOneShotServer` type, register a global name, pass that name into the client process (perhaps with an environment variable or command line flag), and connect to the server in the client. See `cross_process_embedded_senders()` in `test.rs` for an example of how to do this using Unix `fork()` to spawn the process.

## Major missing features

* Servers only accept one client at a time. This is fine if you simply want to use this API to split your application up into a fixed number of mutually untrusting processes, but it's not suitable for implementing a system service. An API for multiple clients may be added later if demand exists for it.
