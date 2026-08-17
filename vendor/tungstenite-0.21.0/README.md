# Tungstenite

Lightweight stream-based WebSocket implementation for [Rust](https://www.rust-lang.org/).

```rust
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

/// A WebSocket echo server
fn main () {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}
```

Take a look at the examples section to see how to write a simple client/server.

**NOTE:** `tungstenite-rs` is more like a barebone to build reliable modern networking applications
using WebSockets. If you're looking for a modern production-ready "batteries included" WebSocket
library that allows you to efficiently use non-blocking sockets and do "full-duplex" communication,
take a look at [`tokio-tungstenite`](https://github.com/snapview/tokio-tungstenite).

[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/tungstenite.svg?maxAge=2592000)](https://crates.io/crates/tungstenite)
[![Build Status](https://github.com/snapview/tungstenite-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/snapview/tungstenite-rs/actions)

[Documentation](https://docs.rs/tungstenite)

Introduction
------------
This library provides an implementation of WebSockets,
[RFC6455](https://tools.ietf.org/html/rfc6455). It allows for both synchronous (like TcpStream)
and asynchronous usage and is easy to integrate into any third-party event loops including
[MIO](https://github.com/tokio-rs/mio). The API design abstracts away all the internals of the
WebSocket protocol but still makes them accessible for those who wants full control over the
network.

Why Tungstenite?
----------------

It's formerly WS2, the 2nd implementation of WS. WS2 is the chemical formula of
tungsten disulfide, the tungstenite mineral.

Features
--------

Tungstenite provides a complete implementation of the WebSocket specification.
TLS is supported on all platforms using `native-tls` or `rustls`. The following
features are available:

* `native-tls`
* `native-tls-vendored`
* `rustls-tls-native-roots`
* `rustls-tls-webpki-roots`

Choose the one that is appropriate for your needs.

By default **no TLS feature is activated**, so make sure you use one of the TLS features,
otherwise you won't be able to communicate with the TLS endpoints.

There is no support for permessage-deflate at the moment, but the PRs are welcome :wink:

Testing
-------

Tungstenite is thoroughly tested and passes the [Autobahn Test Suite](https://github.com/crossbario/autobahn-testsuite) for
WebSockets. It is also covered by internal unit tests as well as possible.

Contributing
------------

Please report bugs and make feature requests [here](https://github.com/snapview/tungstenite-rs/issues).
