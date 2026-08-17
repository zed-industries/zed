// Copyright (c) 2021-2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

// Document all features on docs.rs
#![cfg_attr(docsrs, feature(doc_cfg))]

//! An asynchronous ZIP archive reading/writing crate.
//!
//! ## Features
//! - A base implementation atop `futures`'s IO traits.
//! - An extended implementation atop `tokio`'s IO traits.
//! - Support for Stored, Deflate, bzip2, LZMA, zstd, and xz compression methods.
//! - Various different reading approaches (seek, stream, filesystem, in-memory buffer).
//! - Support for writing complete data (u8 slices) or stream writing using data descriptors.
//! - Initial support for ZIP64 reading and writing.
//! - Aims for reasonable [specification](https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md) compliance.
//!
//! ## Installation
//!
//! ```toml
//! [dependencies]
//! async_zip = { version = "0.0.17", features = ["full"] }
//! ```
//!
//! ### Feature Flags
//! - `full` - Enables all below features.
//! - `full-wasm` - Enables all below features that are compatible with WASM.
//! - `chrono` - Enables support for parsing dates via `chrono`.
//! - `tokio` - Enables support for the `tokio` implementation module.
//! - `tokio-fs` - Enables support for the `tokio::fs` reading module.
//! - `deflate` - Enables support for the Deflate compression method.
//! - `bzip2` - Enables support for the bzip2 compression method.
//! - `lzma` - Enables support for the LZMA compression method.
//! - `zstd` - Enables support for the zstd compression method.
//! - `xz` - Enables support for the xz compression method.
//!
//! [Read more.](https://github.com/Majored/rs-async-zip)

pub mod base;
pub mod error;

#[cfg(feature = "tokio")]
pub mod tokio;

pub(crate) mod date;
pub(crate) mod entry;
pub(crate) mod file;
pub(crate) mod spec;
pub(crate) mod string;
pub(crate) mod utils;

#[cfg(test)]
pub(crate) mod tests;

pub use crate::spec::attribute::AttributeCompatibility;
pub use crate::spec::compression::{Compression, DeflateOption};

pub use crate::date::{builder::ZipDateTimeBuilder, ZipDateTime};
pub use crate::entry::{builder::ZipEntryBuilder, StoredZipEntry, ZipEntry};
pub use crate::file::{builder::ZipFileBuilder, ZipFile};

pub use crate::string::{StringEncoding, ZipString};
