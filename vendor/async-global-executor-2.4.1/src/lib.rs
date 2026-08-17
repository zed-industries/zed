//! A global executor built on top of async-executor and async_io
//!
//! The global executor is lazily spawned on first use. It spawns as many threads
//! as the number of cpus by default. You can override this using the
//! `ASYNC_GLOBAL_EXECUTOR_THREADS` environment variable.
//!
//! # Examples
//!
//! ```
//! # use futures_lite::future;
//!
//! // spawn a task on the multi-threaded executor
//! let task1 = async_global_executor::spawn(async {
//!     1 + 2
//! });
//! // spawn a task on the local executor (same thread)
//! let task2 = async_global_executor::spawn_local(async {
//!     3 + 4
//! });
//! let task = future::zip(task1, task2);
//!
//! // run the executor
//! async_global_executor::block_on(async {
//!     assert_eq!(task.await, (3, 7));
//! });
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub use async_executor::Task;
pub use config::GlobalExecutorConfig;
pub use executor::{block_on, spawn, spawn_blocking, spawn_local};
pub use init::{init, init_with_config};
pub use threading::{spawn_more_threads, stop_current_thread, stop_thread};

mod config;
mod executor;
mod init;
mod reactor;
mod threading;

#[cfg(feature = "tokio")]
mod tokio;
#[cfg(feature = "tokio02")]
mod tokio02;
#[cfg(feature = "tokio03")]
mod tokio03;
