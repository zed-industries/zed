# Asynchronous Codec

Utilities for encoding and decoding frames using async/await.

This is a fork of [`futures-codec`](https://github.com/matthunz/futures-codec)
by [Matt Hunzinger](https://github.com/matthunz) borrowing many concepts from
[`tokio-codec`](https://crates.io/crates/tokio-codec).

Contains adapters to go from streams of bytes, `AsyncRead` and `AsyncWrite`,
to framed streams implementing `Sink` and `Stream`. Framed streams are also known as transports.

[![Latest Version](https://img.shields.io/crates/v/asynchronous-codec.svg)](https://crates.io/crates/asynchronous-codec)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/asynchronous-codec)
![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)


### Example

```rust
use asynchronous_codec::{LinesCodec, Framed};

async fn main() {
    // let stream = ...
    let mut framed = Framed::new(stream, LinesCodec {});

    while let Some(line) = framed.try_next().await.unwrap() {
        println!("{:?}", line);
    }
}
```
