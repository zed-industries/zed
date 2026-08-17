//! Working with the Web [Streams API](https://developer.mozilla.org/en-US/docs/Web/API/Streams_API)
//! in Rust.
//!
//! This crate provides wrappers around [`ReadableStream`], [`WritableStream`] and [`TransformStream`].
//! It also supports converting from and into [`Stream`]s and [`Sink`]s from the [futures] crate.
//!
//! [`Stream`]: https://docs.rs/futures/0.3.30/futures/stream/trait.Stream.html
//! [`Sink`]: https://docs.rs/futures/0.3.30/futures/sink/trait.Sink.html
//! [futures]: https://docs.rs/futures/0.3.30/futures/index.html

pub use readable::ReadableStream;
pub use transform::TransformStream;
pub use writable::WritableStream;

pub(crate) mod queuing_strategy;
pub mod readable;
pub mod transform;
pub(crate) mod util;
pub mod writable;
