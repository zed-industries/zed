//! **The feature-rich, portable async channel library.**
//!
//! # Why use Postage?
//! - Includes a **rich set of channels.**
//!   - [barrier](./barrier/index.html), a oneshot channel that transmits when the sender half is dropped.
//!   - [broadcast](./broadcast/index.html), a lossless multi-producer, multi-consumer broadcast channel with backpressure (no lagging!).
//!   - [dispatch](./dispatch/index.html), a multi-producer, multi-consumer queue.
//!   - [mpsc](./mpsc/index.html), a multi-producer, single-consumer channel.
//!   - [oneshot](./oneshot/index.html), a oneshot transfer channel.
//!   - [watch](./watch/index.html), a state distribution channel with a value that can be borrowed.
//! - Works with **any executor.**
//!   - Currently regressions are written for `tokio` and `async-std`.
//!   - With the `futures-traits` feature, channels implement the futures `Sink/Stream` traits.
//! - **Throughly tested.**  
//!   - Channels have full unit test coverage, and integration test coverage with multiple async executors.
//! - Comes with **built-in [Sink](./sink/trait.Sink.html) and [Stream](./stream/trait.Stream.html) combinators.**
//!   - Sinks can be chained, and filtered.
//!   - Streams can be chained, filtered, mapped, and merged.
//!   - With the `logging` feature, Sinks and streams can log their values.  This is really helpful when debugging applications.
//!
//! See [the readme](https://github.com/austinjones/postage-rs#benchmarks) for benchmarks.
//!
//! ## Cargo features:
//! - `blocking (default)` - enables [Sink::blocking_send](./sink/trait.Sink.html#method.blocking_send) and [Stream::blocking_recv](./stream/trait.Stream.html#method.blocking_recv)
//! - `debug` - enables _extremely verbose_ internal log statements.
//! - `futures-traits` - enables `futures::Sink` and `futures::Stream` implementations for the postage channels.  Compatible with `v0.3`.
//! - `logging (default)` - enables the enables [Sink::log(Level)](./sink/trait.Sink.html#method.log) and [Stream::log(Level)](./stream/trait.Stream.html#method.log) combinators.

mod channels;
mod context;
mod logging;
pub mod prelude;
pub mod sink;
pub mod stream;
mod sync;

#[cfg(feature = "futures-traits")]
mod futures;

pub use channels::barrier;
pub use channels::broadcast;
pub use channels::dispatch;
pub use channels::mpsc;
pub use channels::oneshot;
pub use channels::watch;

pub use context::Context;

#[cfg(test)]
mod test;
