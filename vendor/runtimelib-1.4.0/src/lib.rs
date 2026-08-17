#![doc = include_str!("../README.md")]

#[deprecated(
    since = "0.24.0",
    note = "This re-export will be removed in a future version. Please use jupyter_protocol::media directly."
)]
#[doc(hidden)]
pub use jupyter_protocol::media;

#[deprecated(
    since = "0.24.0",
    note = "This re-export will be removed in a future version. Please use jupyter_protocol::media directly."
)]
#[doc(hidden)]
pub use jupyter_protocol::media::*;

#[deprecated(
    since = "0.24.0",
    note = "This re-export will be removed in a future version. Please use jupyter_protocol::media directly."
)]
#[doc(hidden)]
pub use jupyter_protocol::ExecutionCount;

pub mod kernelspec;
pub use kernelspec::*;

pub mod dirs;
pub use dirs::*;

mod error;
pub use error::{Result, RuntimeError};

#[cfg(any(feature = "tokio-runtime", feature = "async-dispatcher-runtime"))]
pub mod connection;
#[cfg(any(feature = "tokio-runtime", feature = "async-dispatcher-runtime"))]
pub use connection::*;

#[cfg(feature = "test-kernel")]
pub mod test_kernel;
#[cfg(feature = "test-kernel")]
pub use test_kernel::*;
