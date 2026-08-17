//! General purpose helpers for async runtime cross-compatibility

pub mod task;

#[cfg(feature = "tokio-runtime")]
extern crate tokio;
#[cfg(feature = "tokio-runtime")]
pub use tokio::{main, test};

#[cfg(feature = "async-std-runtime")]
extern crate async_std;
#[cfg(feature = "async-std-runtime")]
pub use async_std::{main, test};

#[cfg(all(
    feature = "async-dispatcher-runtime",
    feature = "async-dispatcher-macros"
))]
pub use async_dispatcher::{main, test};
