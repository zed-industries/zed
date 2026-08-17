# AccessKit Unix adapter

This is the Unix adapter for [AccessKit](https://accesskit.dev/). It exposes an AccessKit accessibility tree through the AT-SPI protocol.

## Compatibility with async runtimes

While this crate's API is purely blocking, it internally spawns asynchronous tasks on an executor.

- If you use tokio, make sure to enable the `tokio` feature of this crate.
- If you use another async runtime or if you don't use one at all, the default feature will suit your needs.
