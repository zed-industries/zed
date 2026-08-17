# ZeroMQ Examples

## How to run the examples?
Simply do the usual command:
```
cargo run --example <example_name>
```
This will use the default asynchronous runtime (currently [`tokio`](tokio.rs)). If you want to run them using [`async-std`](async.rs) instead of `tokio`, do the following:
```
cargo run --example <example_name> --no-default-features --features async-std-runtime
```

## What is `async_helpers`?
`zeromq` works on both [`tokio`](tokio.rs) and [`async-std`](async.rs), so we have added some helper code to get things working with either async runtime. In your code you often won't need to care about this, in which case just use `tokio::main` or `async_std::main` as normal instead of `async_helpers::main`.
