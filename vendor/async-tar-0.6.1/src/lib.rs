//! A library for reading and writing TAR archives in an async fashion.
//!
//! This library provides utilities necessary to manage [TAR archives][1]
//! abstracted over a reader or writer. Great strides are taken to ensure that
//! an archive is never required to be fully resident in memory, and all objects
//! provide largely a streaming interface to read bytes from.
//!
//! [1]: http://en.wikipedia.org/wiki/Tar_%28computing%29

// More docs about the detailed tar format can also be found here:
// http://www.freebsd.org/cgi/man.cgi?query=tar&sektion=5&manpath=FreeBSD+8-current

// NB: some of the coding patterns and idioms here may seem a little strange.
//     This is currently attempting to expose a super generic interface while
//     also not forcing clients to codegen the entire crate each time they use
//     it. To that end lots of work is done to ensure that concrete
//     implementations are all found in this crate and the generic functions are
//     all just super thin wrappers (e.g. easy to codegen).

#![deny(missing_docs)]
#![deny(clippy::all)]

#[cfg(all(feature = "runtime-async-std", feature = "runtime-tokio"))]
compile_error!(
    "Features `runtime-async-std` and `runtime-tokio` are mutually exclusive. Please enable only one."
);

#[cfg(not(any(feature = "runtime-async-std", feature = "runtime-tokio")))]
compile_error!("Either `runtime-async-std` or `runtime-tokio` feature must be enabled.");

use std::io::Error;

pub use crate::{
    archive::{Archive, ArchiveBuilder, Entries},
    builder::Builder,
    entry::{Entry, Unpacked},
    entry_type::EntryType,
    header::{
        GnuExtSparseHeader, GnuHeader, GnuSparseHeader, Header, HeaderMode, OldHeader, UstarHeader,
    },
    pax::{PaxExtension, PaxExtensions},
};

mod archive;
mod builder;
mod entry;
mod entry_type;
mod error;
mod header;
mod pax;

#[cfg(test)]
#[macro_use]
extern crate static_assertions;

fn other(msg: &str) -> Error {
    Error::other(msg)
}

#[cfg(feature = "runtime-async-std")]
pub(crate) async fn fs_canonicalize(
    path: &async_std::path::Path,
) -> async_std::io::Result<async_std::path::PathBuf> {
    path.canonicalize().await
}
#[cfg(feature = "runtime-tokio")]
pub(crate) async fn fs_canonicalize(
    path: &std::path::Path,
) -> tokio::io::Result<std::path::PathBuf> {
    tokio::fs::canonicalize(path).await
}

#[cfg(feature = "runtime-async-std")]
async fn symlink_metadata(
    p: &async_std::path::Path,
) -> async_std::io::Result<async_std::fs::Metadata> {
    p.symlink_metadata().await
}
#[cfg(feature = "runtime-tokio")]
async fn symlink_metadata(p: &std::path::Path) -> tokio::io::Result<std::fs::Metadata> {
    tokio::fs::symlink_metadata(p).await
}

#[cfg(feature = "runtime-async-std")]
async fn metadata(p: &async_std::path::Path) -> async_std::io::Result<async_std::fs::Metadata> {
    p.metadata().await
}
#[cfg(feature = "runtime-tokio")]
async fn metadata(p: &std::path::Path) -> tokio::io::Result<std::fs::Metadata> {
    tokio::fs::metadata(p).await
}
