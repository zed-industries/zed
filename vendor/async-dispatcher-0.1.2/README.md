# Async Dispatcher

[![crates.io version](https://img.shields.io/crates/v/async-dispatcher.svg?style=flat-square)](https://crates.io/crates/async-dispatcher)
[![docs](https://docs.rs/async-dispatcher/badge.svg)](https://docs.rs/async-dispatcher)
![CI](https://github.com/zed-industries/async-dispatcher/actions/workflows/ci.yml/badge.svg)

This crate allows async libraries to spawn tasks and set timers without being tied to a particular async runtime.

The core of this need comes from wanting to be able to use the native OS scheduler, as written about in [Zed Decoded: Async Rust](https://zed.dev/blog/zed-decoded-async-rust).

Libraries can `spawn` in a generic way:

```rust
use async_dispatcher::{spawn, sleep};

pub async my_library_function() {
    let task = spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("in a spawned task!");
    });

    // ...
}
```

Applications using those libraries can control how that work is dispatched by implementing the `Dispatcher` trait:

```rust
use async_dispatcher::{set_dispatcher, Dispatcher, Runnable};

struct MyAppDispatcher;

impl Dispatcher for MyAppDispatcher {
    fn dispatch(&self, runnable: Runnable) {
        // ...
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        // ...
    }
}

fn main() {
    set_dispatcher(MyAppDispatcher);

    async_dispatcher::block_on(async move {
        my_library_function().await;
    });
}
```
