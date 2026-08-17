//! A module for working with processes.
//!
//! This module is mostly concerned with spawning and interacting with child processes, but it also
//! provides abort and exit for terminating the current process.
//!
//! This is an async version of [`std::process`].
//!
//! [`std::process`]: https://doc.rust-lang.org/std/process/index.html

#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[doc(inline)]
pub use async_process::ExitStatus;

#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[doc(inline)]
pub use async_process::Output;

#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[doc(inline)]
pub use async_process::Stdio;

#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[doc(inline)]
pub use async_process::Child;

#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[doc(inline)]
pub use async_process::ChildStderr;

#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[doc(inline)]
pub use async_process::ChildStdin;

#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[doc(inline)]
pub use async_process::ChildStdout;

#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[doc(inline)]
pub use async_process::Command;

// Re-export functions.
pub use std::process::{abort, exit, id};
