# tokio-tungstenite

Asynchronous WebSockets for Tokio stack.

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/tokio-tungstenite.svg?maxAge=2592000)](https://crates.io/crates/tokio-tungstenite)
[![Build Status](https://travis-ci.org/snapview/tokio-tungstenite.svg?branch=master)](https://travis-ci.org/snapview/tokio-tungstenite)

[Documentation](https://docs.rs/tokio-tungstenite)

## Usage

Add this in your `Cargo.toml`:

```toml
[dependencies]
tokio-tungstenite = "*"
```

Take a look at the `examples/` directory for client and server examples. You may also want to get familiar with
[Tokio](https://github.com/tokio-rs/tokio) if you don't have any experience with it.

## What is tokio-tungstenite?

This crate is based on [`tungstenite-rs`](https://github.com/snapview/tungstenite-rs) Rust WebSocket library and provides `Tokio` bindings and wrappers for it, so you
can use it with non-blocking/asynchronous `TcpStream`s from and couple it together with other crates from `Tokio` stack.

## Features

As with [`tungstenite-rs`](https://github.com/snapview/tungstenite-rs) TLS is supported on all platforms using [`native-tls`](https://github.com/sfackler/rust-native-tls) or [`rustls`](https://github.com/ctz/rustls) through feature flags: `native-tls`, `rustls-tls-native-roots` or `rustls-tls-webpki-roots` feature flags. Neither is enabled by default. See the `Cargo.toml` for more information. If you require support for secure WebSockets (`wss://`) enable one of them.

Note, that if you're using `rustls` features with `tokio-tungstenite` [version `0.23.0`](https://github.com/snapview/tokio-tungstenite/blob/master/CHANGELOG.md#0230) or higher,
you [might observe a panic](https://github.com/snapview/tokio-tungstenite/issues/336) that is easy to fix (see the issue). Check [the discussion](https://github.com/snapview/tokio-tungstenite/issues/353#issuecomment-2455100010)
for more details.

## Is it performant?

In essence, `tokio-tungstenite` is a wrapper for `tungstenite`, so the performance is capped by the performance of `tungstenite`. `tungstenite`
has a decent performance (it has been used in production for real-time communication software, video conferencing, etc), but it's definitely
not the fastest WebSocket library in the world at the moment of writing this note.

We are aware of changes that both `tungstenite` and `tokio-tungstenite` require in order to make it on-par with slightly more performant libraries like `fastwebsockets`. In the course of past years we have merged several performance improvements submitted by the awesome community of Rust users who helped to improve the library! The more recent versions of `tokio-tungstenite` (`> 0.26.2`) are more performant and should be more on-par with `fastwebsockets`.

For a quick summary of the pending performance problems/improvements, see [the comment](https://github.com/snapview/tungstenite-rs/issues/352#issuecomment-1537488614).
