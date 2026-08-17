//! Adapters to/from other IO traits.
//!
//! To interoperate with other IO trait ecosystems, wrap a type in one of these
//! adapters.
//!
//! There's no separate adapters for Read/ReadBuf/Write traits. Instead, a single
//! adapter implements the right traits based on what the inner type implements.
//! This allows adapting a `Read+Write`, for example.

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod std_io;
#[cfg(feature = "std")]
pub use std_io::*;

#[cfg(feature = "futures")]
#[cfg_attr(docsrs, doc(cfg(feature = "futures")))]
mod futures_io;
#[cfg(feature = "futures")]
pub use futures_io::*;

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
mod tokio;
#[cfg(feature = "tokio")]
pub use crate::adapters::tokio::*;

#[cfg(feature = "std")]
fn to_io_error<T: core::fmt::Debug>(err: T) -> std::io::Error {
    let kind = std::io::ErrorKind::Other;
    std::io::Error::new(kind, format!("{:?}", err))
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl crate::Error for std::io::Error {
    fn kind(&self) -> crate::ErrorKind {
        crate::ErrorKind::Other
    }
}
