//! Performant, portable, structured concurrency operations for async Rust. It
//! works with any runtime, does not erase lifetimes, always handles
//! cancellation, and always returns output to the caller.
//!
//! `futures-concurrency` provides concurrency operations for both groups of futures
//! and streams. Both for bounded and unbounded sets of futures and streams. In both
//! cases performance should be on par with, if not exceed conventional executor
//! implementations.
//!
//! # Examples
//!
//! **Await multiple futures of different types**
//! ```rust
//! use futures_concurrency::prelude::*;
//! use std::future;
//!
//! # futures::executor::block_on(async {
//! let a = future::ready(1u8);
//! let b = future::ready("hello");
//! let c = future::ready(3u16);
//! assert_eq!((a, b, c).join().await, (1, "hello", 3));
//! # });
//! ```
//!
//! **Concurrently process items in a collection**
//!
//! ```rust
//! use futures_concurrency::prelude::*;
//!
//! # futures::executor::block_on(async {
//! let v: Vec<_> = vec!["chashu", "nori"]
//!     .into_co_stream()
//!     .map(|msg| async move { format!("hello {msg}") })
//!     .collect()
//!     .await;
//!
//! assert_eq!(v, &["hello chashu", "hello nori"]);
//! # });
//! ```
//!
//! **Access stack data outside the futures' scope**
//!
//! _Adapted from [`std::thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html)._
//!
//! ```rust
//! use futures_concurrency::prelude::*;
//!
//! # futures::executor::block_on(async {
//! let mut container = vec![1, 2, 3];
//! let mut num = 0;
//!
//! let a = async {
//!     println!("hello from the first future");
//!     dbg!(&container);
//! };
//!
//! let b = async {
//!     println!("hello from the second future");
//!     num += container[0] + container[2];
//! };
//!
//! println!("hello from the main future");
//! let _ = (a, b).join().await;
//! container.push(4);
//! assert_eq!(num, container.len());
//! # });
//! ```
//!
//! # Operations
//!
//! ## Futures
//!
//! For futures which return a regular type `T` only the `join` and `race`
//! operations are available. `join` waits for all futures to complete, while `race`
//! will wait for the first future to complete. However for futures which return a
//! `Try<Output = T>` two additional operations are available. The following table
//! describes the behavior of concurrency operations for fallible futures:
//!
//! |                            | **Wait for all outputs** | **Wait for first output** |
//! | -------------------------- | :----------------------- | :------------------------ |
//! | **Continue on error**      | `Future::join`           | `Future::race_ok`         |
//! | **Short-circuit on error** | `Future::try_join`       | `Future::race`            |
//!
//! The following futures implementations are provided by `futures-concurrency`:
//! - [`FutureGroup`][future::FutureGroup]: A growable group of futures which operate as a single unit.
//! - `tuple`: [`join`][future::Join#impl-Join-for-(A,+B)], [`try_join`][future::TryJoin#impl-TryJoin-for-(A,+B)], [`race`][future::Race#impl-Race-for-(A,+B)], [`race_ok`][future::RaceOk#impl-RaceOk-for-(A,+B)]
//! - `array`: [`join`][future::Join#impl-Join-for-\[Fut;+N\]], [`try_join`][future::TryJoin#impl-TryJoin-for-\[Fut;+N\]], [`race`][future::Race#impl-Race-for-\[Fut;+N\]], [`race_ok`][future::RaceOk#impl-RaceOk-for-\[Fut;+N\]]
//! - `Vec`: [`join`][future::Join#impl-Join-for-Vec<Fut>], [`try_join`][future::TryJoin#impl-TryJoin-for-Vec<Fut>], [`race`][future::Race#impl-Race-for-Vec<Fut>], [`race_ok`][future::RaceOk#impl-RaceOk-for-Vec<Fut>]
//!
//! ## Streams
//!
//! Streams yield outputs one-by-one, which means that deciding to stop iterating is
//! the same for fallible and infallible streams. The operations provided for
//! streams can be categorized based on whether their inputs can be concurrently
//! evaluated, and whether their outputs can be concurrently processed.
//!
//! Specifically in the case of `merge`, it takes `N` streams in, and yields items
//! one-by-one as soon as any are available. This enables the output of individual
//! streams to be concurrently processed by further operations later on.
//!
//! |                                 | __Sequential output processing__ | __Concurrent output processing__ |
//! | ------------------------------- | -------------------------------- | -------------------------------- |
//! | __Sequential input evaluation__ | `Stream::chain`                  | *not yet available* ‡            |
//! | __Concurrent input evaluation__ | `Stream::zip`                    | `Stream::merge`                  |
//!
//!  ‡: _This could be addressed by a hypothetical `Stream::unzip` operation,
//!  however because we aspire for semantic compatibility with `std::iter::Iterator`
//!  in our operations, the path to adding it is currently unclear_.
//!  
//! The following streams implementations are provided by `futures-concurrency`:
//!
//! - [`StreamGroup`][stream::StreamGroup]: A growable group of streams which operate as a single unit.
//! - [`ConcurrentStream`][concurrent_stream::ConcurrentStream]: A trait for asynchronous streams which can concurrently process items.
//! - `tuple`: [`chain`][stream::Chain#impl-Chain-for-(A,+B)], [`merge`][stream::Merge#impl-Merge-for-(A,+B)], [`zip`][stream::Zip#impl-Zip-for-(A,+B)]
//! - `array`: [`chain`][stream::Chain#impl-Chain-for-\[Fut;+N\]], [`merge`][stream::Merge#impl-Merge-for-\[Fut;+N\]], [`zip`][stream::Zip#impl-Zip-for-\[Fut;+N\]]
//! - `Vec`: [`chain`][stream::Chain#impl-Chain-for-Vec<Fut>], [`merge`][stream::Merge#impl-Merge-for-Vec<Fut>], [`zip`][stream::Zip#impl-Zip-for-Vec<Fut>]
//!
//! # Runtime Support
//!
//! `futures-concurrency` does not depend on any runtime executor being present.
//! This enables it to work out of the box with any async runtime, including:
//! `tokio`, `async-std`, `smol`, `glommio`, and `monoio`. It also supports
//! `#[no_std]` environments, allowing it to be used with embedded async
//! runtimes such as `embassy`.
//!
//! # Feature Flags
//!
//! The `std` feature flag is enabled by default. To target `alloc` or `no_std`
//! environments, you can enable the following configuration:
//!
//! ```toml
//! [dependencies]
//! # no_std
//! futures-concurrency = { version = "7.5.0", default-features = false }
//!
//! # alloc
//! futures-concurrency = { version = "7.5.0", default-features = false, features = ["alloc"] }
//! ```
//!
//! # Further Reading
//!
//! `futures-concurrency` has been developed over the span of several years. It is
//! primarily maintained by Yosh Wuyts, a member of the Rust Async WG. You can read
//! more about the development and ideas behind `futures-concurrency` here:
//!
//! - [Futures Concurrency I: Introduction](https://blog.yoshuawuyts.com/futures-concurrency/)
//! - [Futures Concurrency II: A Trait Approach](https://blog.yoshuawuyts.com/futures-concurrency-2/)
//! - [Futures Concurrency III: `select!`](https://blog.yoshuawuyts.com/futures-concurrency-3/)
//! - [Futures Concurrency IV: Join Semantics](https://blog.yoshuawuyts.com/futures-concurrency-4/)

#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs)]
#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod collections;
mod utils;

#[doc(hidden)]
pub use utils::private;

/// The futures concurrency prelude.
pub mod prelude {
    pub use super::future::FutureExt as _;
    pub use super::stream::StreamExt as _;

    pub use super::future::Join as _;
    pub use super::future::Race as _;
    pub use super::future::RaceOk as _;
    pub use super::future::TryJoin as _;
    pub use super::stream::Chain as _;
    pub use super::stream::IntoStream as _;
    pub use super::stream::Merge as _;
    pub use super::stream::Zip as _;

    #[cfg(feature = "alloc")]
    pub use super::concurrent_stream::{
        ConcurrentStream, FromConcurrentStream, IntoConcurrentStream,
    };
}

#[cfg(feature = "alloc")]
pub mod concurrent_stream;

#[cfg(feature = "alloc")]
pub use collections::vec;

pub mod future;
pub mod stream;

/// Helper functions and types for fixed-length arrays.
pub mod array {
    pub use crate::future::join::array::Join;
    pub use crate::future::race::array::Race;
    pub use crate::future::race_ok::array::{AggregateError, RaceOk};
    pub use crate::future::try_join::array::TryJoin;
    pub use crate::stream::chain::array::Chain;
    pub use crate::stream::merge::array::Merge;
    pub use crate::stream::zip::array::Zip;
}
