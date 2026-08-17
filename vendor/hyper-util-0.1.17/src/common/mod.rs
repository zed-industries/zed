#![allow(missing_docs)]

pub(crate) mod exec;
#[cfg(feature = "client-legacy")]
mod lazy;
#[cfg(feature = "server")]
// #[cfg(feature = "server-auto")]
pub(crate) mod rewind;
#[cfg(feature = "client-legacy")]
mod sync;
pub(crate) mod timer;

#[cfg(feature = "client-legacy")]
pub(crate) use exec::Exec;

#[cfg(feature = "client-legacy")]
pub(crate) use lazy::{lazy, Started as Lazy};
#[cfg(feature = "client-legacy")]
pub(crate) use sync::SyncWrapper;

pub(crate) mod future;
