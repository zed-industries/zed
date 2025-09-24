# Convex

The official Rust client for [Convex](https://convex.dev/).

![GitHub](https://img.shields.io/github/license/get-convex/convex-rs)

Convex is the backend application platform with everything you need to build
your product.

This Rust client can write and read data from a Convex backend with queries,
mutations, and actions. Get up and running at
[docs.convex.dev](https://docs.convex.dev/introduction/).

[Join us on Discord](https://www.convex.dev/community) to share what you're
working on or get your questions answered.

# Installation

Add the following to your `Cargo.toml` file

```toml
[dependencies]
convex = "*"
```

# Example

```rust
let mut client = ConvexClient::new(DEPLOYMENT_URL).await?;
let mut subscription = client.subscribe("getCounter", vec![]).await?;
while let Some(new_val) = subscription.next().await {
    println!("Counter updated to {new_val:?}");
}
```

# Documentation

Check out the full convex documentation at
[docs.convex.dev](https://docs.convex.dev/introduction/) The rust API docs are
available on [docs.rs](https://docs.rs/convex/latest/convex/)

# MSRV

The Convex rust client works on stable rust 1.71.1 and higher. It also works on
nightly.

# Debug Logging

The Convex Rust Client uses the
[tracing](https://docs.rs/tracing/latest/tracing/) crate for logging. One common
way of initializing is via `tracing_subscriber`. Then, you can see debug logging
by running your program with `RUST_LOG=convex=debug`.

```rust
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();
```

By default, this will emit all logs, including internal logs from the client.
Logs from your Convex backend will show up under the `convex_logs` target at
Level=DEBUG. If you want to isolate just those logs, please refer to the
[tracing_subscriber documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/index.html#filtering-with-layers).
