//! Composable asynchronous iteration.
//!
//! # Examples
//!
//! Merge multiple streams to handle values as soon as they're ready, without
//! ever dropping a single value:
//!
//! ```
//! use futures_concurrency::prelude::*;
//! use futures_lite::stream::{self, StreamExt};
//! use futures_lite::future::block_on;
//!
//! block_on(async {
//!     let a = stream::once(1);
//!     let b = stream::once(2);
//!     let c = stream::once(3);
//!     let s = (a, b, c).merge();
//!
//!     let mut counter = 0;
//!     s.for_each(|n| counter += n).await;
//!     assert_eq!(counter, 6);
//! })
//! ```
//!
//! # Concurrency
//!
//! When working with multiple (async) iterators, the ordering in which
//! iterators are awaited is important. As part of async iterators, Rust
//! provides built-in operations to control the order of execution of sets of
//! iterators:
//!
//! - `merge`: combine multiple iterators into a single iterator, where the new
//!   iterator yields an item as soon as one is available from one of the
//!   underlying iterators.
//! - `zip`: combine multiple iterators into an iterator of pairs. The
//!   underlying iterators will be awaited concurrently.
//! - `chain`: iterate over multiple iterators in sequence. The next iterator in
//!   the sequence won't start until the previous iterator has finished.
//!
//! ## Futures
//!
//! Futures can be thought of as async sequences of single items. Using
//! `stream::once`, futures can be converted into async iterators and then used
//! with any of the iterator concurrency methods. This enables operations such
//! as `stream::Merge` to be used to execute sets of futures concurrently, but
//! obtain the individual future's outputs as soon as they're available.
//!
//! See the [future concurrency][crate::future#concurrency] documentation for
//! more on futures concurrency.
pub use chain::Chain;
pub use into_stream::IntoStream;
pub use merge::Merge;
pub use stream_ext::StreamExt;
#[doc(inline)]
#[cfg(feature = "alloc")]
pub use stream_group::StreamGroup;
pub use wait_until::WaitUntil;
pub use zip::Zip;

/// A growable group of streams which act as a single unit.
#[cfg(feature = "alloc")]
pub mod stream_group;

pub(crate) mod chain;
mod into_stream;
pub(crate) mod merge;
mod stream_ext;
pub(crate) mod wait_until;
pub(crate) mod zip;
