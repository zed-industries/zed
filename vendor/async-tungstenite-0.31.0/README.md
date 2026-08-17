# async-tungstenite

Asynchronous WebSockets for [async-std](https://async.rs),
[tokio](https://tokio.rs), [gio](https://gtk-rs.org) and any `std`
`Future`s runtime.

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/async-tungstenite.svg?maxAge=2592000)](https://crates.io/crates/async-tungstenite)
[![Build Status](https://github.com/sdroege/async-tungstenite/workflows/CI/badge.svg)](https://github.com/sdroege/async-tungstenite/actions?query=workflow%3ACI)

[Documentation](https://docs.rs/async-tungstenite)

## Usage

Add this in your `Cargo.toml`:

```toml
[dependencies]
async-tungstenite = "*"
```

Take a look at the `examples/` directory for client and server examples. You
may also want to get familiar with [async-std](https://async.rs/) or
[tokio](https://tokio.rs) if you don't have any experience with it.

## What is async-tungstenite?

This crate is based on [tungstenite](https://crates.io/crates/tungstenite)
Rust WebSocket library and provides async bindings and wrappers for it, so you
can use it with non-blocking/asynchronous `TcpStream`s from and couple it
together with other crates from the async stack. In addition, optional
integration with various other crates can be enabled via feature flags

 * `async-tls`: Enables the `async_tls` module, which provides integration
   with the [async-tls](https://crates.io/crates/async-tls) TLS stack and can
   be used independent of any async runtime.
 * `async-std-runtime`: Enables the `async_std` module, which provides
   integration with the [async-std](https://async.rs) runtime.
 * `async-native-tls`: Enables the additional functions in the `async_std`
   module to implement TLS via
   [async-native-tls](https://crates.io/crates/async-native-tls).
 * `tokio-runtime`: Enables the `tokio` module, which provides integration
   with the [tokio](https://tokio.rs) runtime.
 * `tokio-native-tls`: Enables the additional functions in the `tokio` module to
   implement TLS via [tokio-native-tls](https://crates.io/crates/tokio-native-tls).
 * `tokio-rustls-native-certs`: Enables the additional functions in the `tokio` 
   module to implement TLS via [tokio-rustls](https://crates.io/crates/tokio-rustls)
   and uses native system certificates found with
   [rustls-native-certs](https://github.com/rustls/rustls-native-certs).
 * `tokio-rustls-webpki-roots`: Enables the additional functions in the `tokio` 
   module to implement TLS via [tokio-rustls](https://crates.io/crates/tokio-rustls)
   and uses the certificates [webpki-roots](https://github.com/rustls/webpki-roots)
   provides.
 * `gio-runtime`: Enables the `gio` module, which provides integration with
   the [gio](https://gtk-rs.org) runtime.

## Messages vs Streaming

WebSocket provides a message-oriented protocol, and this crate primarily
supports sending and receiving data in messages; protocols built on WebSocket
are allowed to make message boundaries semantically significant. However, some
users of WebSocket may want to treat the socket as a continuous stream of
bytes. If you know the other end does not place significance on message
boundaries, and you want to process a stream of bytes without regard to those
boundaries, you can use the `ByteReader` and `ByteWriter` types provided by
this crate.

## Is it performant?

In essence, `async-tungstenite` is a wrapper for `tungstenite`, so the performance is capped by the performance of `tungstenite`. `tungstenite`
has a decent performance (it has been used in production for real-time communication software, video conferencing, etc), but it's definitely
not the fastest WebSocket library in the world at the moment of writing this note.

If performance is of a paramount importance for you (especially if you send **large messages**), then you might want to check other libraries
that have been designed to be performant or you could file a PR against `tungstenite` to improve the performance!

We are aware of changes that both `tungstenite` and `async-tungstenite` need in order to fill the gap of ~30% performance difference between `tungstenite`
and more performant libraries like `fastwebsockets`, but we have not worked on that yet as it was not required for the use case that original authors designed
the library for. In the course of past years we have merged several performance improvements submitted by the awesome community of Rust users who helped to improve
the library! For a quick summary of the pending performance problems/improvements, see [the comment](https://github.com/snapview/tungstenite-rs/issues/352#issuecomment-1537488614).

## tokio-tungstenite

Originally this crate was created as a fork of
[tokio-tungstenite](https://github.com/snapview/tokio-tungstenite) and ported
to the traits of the [`futures`](https://crates.io/crates/futures) crate.
Integration into async-std, tokio and gio was added on top of that.

