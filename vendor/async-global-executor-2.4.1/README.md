# async-global-executor

[![API Docs](https://docs.rs/async-global-executor/badge.svg)](https://docs.rs/async-global-executor)
[![Build status](https://github.com/Keruspe/async-global-executor/workflows/Build%20and%20test/badge.svg)](https://github.com/Keruspe/async-global-executor/actions)
[![Downloads](https://img.shields.io/crates/d/async-global-executor.svg)](https://crates.io/crates/async-global-executor)

A global executor built on top of async-executor and async-io

# Features

* `async-io`: if enabled, `async-global-executor` will use `async_io::block_on` instead of
  `futures_lite::future::block_on` internally. this is preferred if your application also uses `async-io`.
* `blocking`: enable the use of the `blocking` crate through `async_global_executor::spawn_blocking`.
* `tokio`: if enabled, `async-global-executor` will ensure that all tasks that you will spawn run in the context of a
  tokio 1.0 runtime, spawning a new one if required.
* `tokio03`: if enabled, `async-global-executor` will ensure that all tasks that you will spawn run in the context of a
  tokio 0.3 runtime, spawning a new one if required.
* `tokio02`: if enabled, `async-global-executor` will ensure that all tasks that you will spawn run in the context of a
  tokio 0.2 runtime, spawning a new one if required.

# Examples

```
# use futures_lite::future;

// spawn a task on the multi-threaded executor
let task1 = async_global_executor::spawn(async {
    1 + 2
});
// spawn a task on the local executor (same thread)
let task2 = async_global_executor::spawn_local(async {
    3 + 4
});
let task = future::zip(task1, task2);

// run the executor
async_global_executor::block_on(async {
    assert_eq!(task.await, (3, 7));
});
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
