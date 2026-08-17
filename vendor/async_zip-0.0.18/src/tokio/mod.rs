// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A set of [`tokio`]-specific type aliases and features.
//!
//! # Usage
//! With the `tokio` feature enabled, types from the [`base`] implementation will implement additional constructors
//! for use with [`tokio`]. These constructors internally implement conversion between the required async IO traits.
//! They are defined as:
//! - [`base::read::seek::ZipFileReader::with_tokio()`]
//! - [`base::read::stream::ZipFileReader::with_tokio()`]
//! - [`base::write::ZipFileWriter::with_tokio()`]
//!
//! As a result of Rust's type inference, we are able to reuse the [`base`] implementation's types with considerable
//! ease. There only exists one caveat with their use; the types returned by these constructors contain a wrapping
//! compatibility type provided by an external crate. These compatibility types cannot be named unless you also pull in
//! the [`tokio_util`] dependency manually. This is why we've provided type aliases within this module so that they can
//! be named without needing to pull in a separate dependency.

#[cfg(doc)]
use crate::base;
#[cfg(doc)]
use tokio;
#[cfg(doc)]
use tokio_util;

pub mod read;

pub mod write {
    //! A module which supports writing ZIP files.

    #[cfg(doc)]
    use crate::base;
    use tokio_util::compat::Compat;

    /// A [`tokio`]-specific type alias for [`base::write::ZipFileWriter`];
    pub type ZipFileWriter<W> = crate::base::write::ZipFileWriter<Compat<W>>;

    /// A [`tokio`]-specific type alias for [`base::write::EntryStreamWriter`];
    pub type EntryStreamWriter<'a, W> = crate::base::write::EntryStreamWriter<'a, Compat<W>>;
}
