# async-pipe-rs

[![crates.io](https://img.shields.io/crates/v/async-pipe.svg)](https://crates.io/crates/async-pipe)
[![Documentation](https://docs.rs/async-pipe/badge.svg)](https://docs.rs/async-pipe)
[![MIT](https://img.shields.io/crates/l/async-pipe.svg)](./LICENSE)

Creates an asynchronous piped reader and writer pair using `tokio.rs` or
`futures`

[Docs](https://docs.rs/async-pipe)

## Usage

First add this to your Cargo.toml:

```toml
[dependencies]
async-pipe = "0.1"
```

An example:
```rust
use async_pipe;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let (mut w, mut r) = async_pipe::pipe();

    tokio::spawn(async move {
        w.write_all(b"hello world").await.unwrap();
    });

    let mut v = Vec::new();
    r.read_to_end(&mut v).await.unwrap();
    println!("Received: {:?}", String::from_utf8(v));
}
```

## Contributing

Your PRs and stars are always welcome.
