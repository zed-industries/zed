# warp

[![crates.io](https://img.shields.io/crates/v/warp.svg)](https://crates.io/crates/warp)
[![Released API docs](https://docs.rs/warp/badge.svg)](https://docs.rs/warp)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![GHA Build Status](https://github.com/seanmonstar/warp/workflows/CI/badge.svg)](https://github.com/seanmonstar/warp/actions?query=workflow%3ACI)
[![Discord chat][discord-badge]][discord-url]

A super-easy, composable, web server framework for warp speeds.

The fundamental building block of `warp` is the `Filter`: they can be combined
and composed to express rich requirements on requests.

Thanks to its `Filter` system, warp provides these out of the box:

* Path routing and parameter extraction
* Header requirements and extraction
* Query string deserialization
* JSON and Form bodies
* Multipart form data
* Static Files and Directories
* Websockets
* Access logging
* Gzip, Deflate, and Brotli compression

Since it builds on top of [hyper](https://hyper.rs), you automatically get:

- HTTP/1
- HTTP/2
- Asynchronous
- One of the fastest HTTP implementations
- Tested and **correct**

## Example

Add warp and Tokio to your dependencies:

```toml
tokio = { version = "1", features = ["full"] }
warp = "0.3"
```

And then get started in your `main.rs`:

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```

For more information you can check the [docs](https://docs.rs/warp) or the [examples](https://github.com/seanmonstar/warp/tree/master/examples).

[discord-badge]: https://img.shields.io/discord/500028886025895936.svg?logo=discord
[discord-url]: https://discord.gg/RFsPjyt
