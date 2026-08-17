//! This module defines a trait that can be used to plug in different Futures executors into
//! Criterion.rs' async benchmarking support.
//!
//! Implementations are provided for:
//! * Tokio (implemented directly for tokio::Runtime)
//! * Async-std
//! * Smol
//! * The Futures crate
//!
//! Please note that async benchmarks will have a small amount of measurement overhead relative
//! to synchronous benchmarks. It is recommended to use synchronous benchmarks where possible, to
//! improve measurement accuracy.

use std::future::Future;

/// Plugin trait used to allow benchmarking on multiple different async runtimes.
///
/// Smol, Tokio and Async-std are supported out of the box, as is the current-thread runner from the
/// Futures crate; it is recommended to use whichever runtime you use in production.
pub trait AsyncExecutor {
    /// Spawn the given future onto this runtime and block until it's complete, returning the result.
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T;
}

/// Runs futures on the 'futures' crate's built-in current-thread executor
#[cfg(feature = "async_futures")]
pub struct FuturesExecutor;
#[cfg(feature = "async_futures")]
impl AsyncExecutor for FuturesExecutor {
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        futures::executor::block_on(future)
    }
}

/// Runs futures on the 'smol' crate's global executor
#[cfg(feature = "async_smol")]
pub struct SmolExecutor;
#[cfg(feature = "async_smol")]
impl AsyncExecutor for SmolExecutor {
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        smol::block_on(future)
    }
}

#[cfg(feature = "async_tokio")]
impl AsyncExecutor for tokio::runtime::Runtime {
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        self.block_on(future)
    }
}
#[cfg(feature = "async_tokio")]
impl AsyncExecutor for &tokio::runtime::Runtime {
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        (*self).block_on(future)
    }
}

/// Runs futures on the 'async-std' crate's global executor
#[cfg(feature = "async_std")]
pub struct AsyncStdExecutor;
#[cfg(feature = "async_std")]
impl AsyncExecutor for AsyncStdExecutor {
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        async_std::task::block_on(future)
    }
}
