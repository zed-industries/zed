![Tracing — Structured, application-level diagnostics][splash]

[splash]: https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/splash.svg

# tracing-serde

An adapter for serializing [`tracing`] types using [`serde`].

[![Documentation][docs-badge]][docs-url]
[![Documentation (master)][docs-master-badge]][docs-master-url]

[docs-badge]: https://docs.rs/tracing-serde/badge.svg
[docs-url]: https://docs.rs/tracing-serde
[docs-master-badge]: https://img.shields.io/badge/docs-master-blue
[docs-master-url]: https://tracing-rs.netlify.com/tracing_serde

## Overview

[`tracing`] is a framework for instrumenting Rust programs to collect
scoped, structured, and async-aware diagnostics.`tracing-serde` enables
serializing `tracing` types using [`serde`].

Traditional logging is based on human-readable text messages.
`tracing` gives us machine-readable structured diagnostic
information. This lets us interact with diagnostic data
programmatically. With `tracing-serde`, you can implement a
`Subscriber` to serialize your `tracing` types and make use of the
existing ecosystem of `serde` serializers to talk with distributed
tracing systems.

Serializing diagnostic information allows us to do more with our logged
values. For instance, when working with logging data in JSON gives us
pretty-print when we're debugging in development and you can emit JSON
and tracing data to monitor your services in production.

The `tracing` crate provides the APIs necessary for instrumenting
libraries and applications to emit trace data.

*Compiler support: [requires `rustc` 1.63+][msrv]*

[msrv]: #supported-rust-versions

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
tracing = "0.1"
tracing-serde = "0.2"
```

Next, add this to your crate:

```rust
use tracing_serde::AsSerde;
```

Please read the [`tracing` documentation](https://docs.rs/tracing/latest/tracing/index.html)
for more information on how to create trace data.

This crate provides the `as_serde` function, via the `AsSerde` trait,
which enables serializing the `Attributes`, `Event`, `Id`, `Metadata`,
and `Record` `tracing` values.

For the full example, please see the [examples](../examples) folder.

Implement a `Subscriber` to format the serialization of `tracing`
types how you'd like.

```rust
pub struct JsonSubscriber {
    next_id: AtomicUsize, // you need to assign span IDs, so you need a counter
}

impl Subscriber for JsonSubscriber {

    fn new_span(&self, attrs: &Attributes) -> Id {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = Id::from_u64(id as u64);
        let json = json!({
        "new_span": {
            "attributes": attrs.as_serde(),
            "id": id.as_serde(),
        }});
        println!("{}", json);
        id
    }
    // ...
}
```

After you implement your `Subscriber`, you can use your `tracing`
subscriber (`JsonSubscriber` in the above example) to record serialized
trace data.

## Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported
version is 1.63. The current Tracing version is not guaranteed to build on Rust
versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current stable
compiler version is 1.69, the minimum supported version will not be increased
past 1.66, three minor versions prior. Increasing the minimum supported compiler
version is not considered a semver breaking change as long as doing so complies
with this policy.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tokio by you, shall be licensed as MIT, without any additional
terms or conditions.

[`tracing`]: https://crates.io/crates/tracing
[`serde`]: https://crates.io/crates/serde
