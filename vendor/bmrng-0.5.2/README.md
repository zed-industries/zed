# bmrng 🪃

[![Crates.io](https://img.shields.io/crates/v/bmrng)](https://crates.io/crates/bmrng)
[![Documentation](https://docs.rs/bmrng/badge.svg)](https://docs.rs/bmrng)
![Unit Tests](https://github.com/oguzbilgener/bmrng/workflows/Unit%20Tests/badge.svg)
[![Coverage Status](https://coveralls.io/repos/github/oguzbilgener/bmrng/badge.svg)](https://coveralls.io/github/oguzbilgener/bmrng)
[![Dependency status](https://deps.rs/repo/github/oguzbilgener/bmrng/status.svg)](https://deps.rs/repo/github/oguzbilgener/bmrng/status.svg)

An async MPSC request-response channel for Tokio, where you can send a response to the sender.
Inspired by [crossbeam_requests](https://docs.rs/crate/crossbeam_requests).

### Example


```rust
#[tokio::main]
async fn main() {
    let buffer_size = 100;
    let (tx, mut rx) = bmrng::channel::<i32, i32>(buffer_size);
    tokio::spawn(async move {
        while let Ok((input, mut responder)) = rx.recv().await {
            if let Err(err) = responder.respond(input * input) {
                println!("sender dropped the response channel");
            }
        }
    });
    for i in 1..=10 {
        if let Ok(response) = tx.send_receive(i).await {
            println!("Requested {}, got {}", i, response);
            assert_eq!(response, i * i);
        }
    }
}
```

#### Request Timeout

It is also possible to create a channel with a request timeout:

```rust
use tokio::time::{Duration, sleep};
#[tokio::main]
async fn main() {
    let (tx, mut rx) = bmrng::channel_with_timeout::<i32, i32>(100, Duration::from_millis(100));
    tokio::spawn(async move {
        match rx.recv().await {
            Ok((input, mut responder)) => {
                sleep(Duration::from_millis(200)).await;
                let res = responder.respond(input * input);
                assert_eq!(res.is_ok(), true);
            }
            Err(err) => {
                println!("all request senders dropped");
            }
        }
    });
    let response = tx.send_receive(8).await;
    assert_eq!(response, Err(bmrng::error::RequestError::<i32>::RecvTimeoutError));
}
```

#### Unbounded Channel

There is also an unbounded alternative, `bmrng::unbounded_channel()` with sync `.send()` calls.