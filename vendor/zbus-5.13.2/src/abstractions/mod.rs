/// This mod contains a bunch of abstractions.
///
/// These abstractions allow us to make use of the appropriate API depending on which features are
/// enabled.
mod executor;
pub use executor::*;
mod async_drop;
pub(crate) mod async_lock;
pub use async_drop::*;
pub(crate) mod timeout;

// Not unix-specific itself but only used on unix.
#[cfg(target_family = "unix")]
pub(crate) mod process;
