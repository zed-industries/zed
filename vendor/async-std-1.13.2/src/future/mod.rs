//! Asynchronous values.
//!
//! ## Base Futures Concurrency
//!
//! Often it's desirable to await multiple futures as if it was a single
//! future. The `join` family of operations converts multiple futures into a
//! single future that returns all of their outputs. The `race` family of
//! operations converts multiple future into a single future that returns the
//! first output.
//!
//! For operating on futures the following functions can be used:
//!
//! | Name               | Return signature | When does it return?     |
//! | ---                | ---              | ---                      |
//! | [`Future::join`]   | `(T1, T2)`       | Wait for all to complete
//! | [`Future::race`]   | `T`              | Return on first value
//!
//! ## Fallible Futures Concurrency
//!
//! For operating on futures that return `Result` additional `try_` variants of
//! the functions mentioned before can be used. These functions are aware of `Result`,
//! and will behave slightly differently from their base variants.
//!
//! In the case of `try_join`, if any of the futures returns `Err` all
//! futures are dropped and an error is returned. This is referred to as
//! "short-circuiting".
//!
//! In the case of `try_race`, instead of returning the first future that
//! completes it returns the first future that _successfully_ completes. This
//! means `try_race` will keep going until any one of the futures returns
//! `Ok`, or _all_ futures have returned `Err`.
//!
//! However sometimes it can be useful to use the base variants of the functions
//! even on futures that return `Result`. Here is an overview of operations that
//! work on `Result`, and their respective semantics:
//!
//! | Name                   | Return signature               | When does it return? |
//! | ---                    | ---                            | ---                  |
//! | [`Future::join`]       | `(Result<T, E>, Result<T, E>)` | Wait for all to complete
//! | [`Future::try_join`]   | `Result<(T1, T2), E>`          | Return on first `Err`, wait for all to complete
//! | [`Future::race`]       | `Result<T, E>`                 | Return on first value
//! | [`Future::try_race`]   | `Result<T, E>`                 | Return on first `Ok`, reject on last Err
//!
//! [`Future::join`]: trait.Future.html#method.join
//! [`Future::try_join`]: trait.Future.html#method.try_join
//! [`Future::race`]: trait.Future.html#method.race
//! [`Future::try_race`]: trait.Future.html#method.try_race

cfg_alloc! {
    pub use future::Future;
    pub(crate) mod future;
}

cfg_std! {
    pub use pending::pending;
    pub use poll_fn::poll_fn;
    pub use ready::ready;

    mod pending;
    mod poll_fn;
    mod ready;
}

#[cfg(any(feature = "unstable", feature = "default"))]
pub use timeout::{timeout, TimeoutError};
#[cfg(any(feature = "unstable", feature = "default"))]
mod timeout;

cfg_unstable! {
    pub use into_future::IntoFuture;
    pub(crate) use maybe_done::MaybeDone;
    mod into_future;
    mod maybe_done;
}
