//! Asynchronous basic functionality.
//!
//! Please see the fundamental `async` and `await` keywords and the [async book]
//! for more information on asynchronous programming in Rust.
//!
//! [async book]: https://rust-lang.github.io/async-book/
//!
//! # Examples
//!
//! ```
//! use futures_concurrency::prelude::*;
//! use futures_lite::future::block_on;
//! use std::future;
//!
//! block_on(async {
//!     // Await multiple similarly-typed futures.
//!     let a = future::ready(1);
//!     let b = future::ready(2);
//!     let c = future::ready(3);
//!     assert_eq!([a, b, c].join().await, [1, 2, 3]);
//!
//!     // Await multiple differently-typed futures.
//!     let a = future::ready(1u8);
//!     let b = future::ready("hello");
//!     let c = future::ready(3u16);
//!     assert_eq!((a, b, c).join().await, (1, "hello", 3));
//!
//!     // It even works with vectors of futures, providing an alternative
//!     // to futures-rs' `join_all`.
//!     let a = future::ready(1);
//!     let b = future::ready(2);
//!     let c = future::ready(3);
//!     assert_eq!(vec![a, b, c].join().await, vec![1, 2, 3]);
//! })
//! ```
//!
//! # Concurrency
//!
//! It's common for operations to depend on the output of multiple futures.
//! Instead of awaiting each future in sequence it can be more efficient to
//! await them _concurrently_. Rust provides built-in mechanisms in the library
//! to make this easy and convenient to do.
//!
//! ## Infallible Concurrency
//!
//! When working with futures which don't return `Result` types, we
//! provide two built-in concurrency operations:
//!
//! - `future::Join`: wait for all futures in the set to complete
//! - `future::Race`: wait for the _first_ future in the set to complete
//!
//! Because futures can be considered to be an async sequence of one, see
//! the [async iterator concurrency][crate::stream#concurrency] section for
//! additional async concurrency operations.
//!
//! ## Fallible Concurrency
//!
//! When working with futures which return `Result` types, the meaning of the
//! existing operations changes, and additional `Result`-aware concurrency
//! operations become available:
//!
//! |                             | __Wait for all outputs__ | __Wait for first output__ |
//! | ---                         | ---                      | ---                       |
//! | __Continue on error__       | `future::Join`           | `future::RaceOk`
//! | __Return early on error__   | `future::TryJoin`        | `future::Race`
//!
//! - `future::TryJoin`: wait for all futures in the set to complete _successfully_, or return on the first error.
//! - `future::RaceOk`: wait for the first _successful_ future in the set to
//!   complete, or return an `Err` if *no* futures complete successfully.
//!
#[doc(inline)]
#[cfg(feature = "alloc")]
pub use future_group::FutureGroup;
pub use futures_ext::FutureExt;
pub use join::Join;
pub use race::Race;
pub use race_ok::RaceOk;
pub use try_join::TryJoin;
pub use wait_until::WaitUntil;

/// A growable group of futures which act as a single unit.
#[cfg(feature = "alloc")]
pub mod future_group;

mod futures_ext;
pub(crate) mod join;
pub(crate) mod race;
pub(crate) mod race_ok;
pub(crate) mod try_join;
pub(crate) mod wait_until;
