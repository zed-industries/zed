// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A concurrent ZIP reader which acts over an owned vector of bytes.
//!
//! Concurrency is achieved as a result of:
//! - Wrapping the provided vector of bytes within an [`Arc`] to allow shared ownership.
//! - Wrapping this [`Arc`] around a [`Cursor`] when reading (as the [`Arc`] can deref and coerce into a `&[u8]`).
//!
//! ### Usage
//! Unlike the [`seek`] module, we no longer hold a mutable reference to any inner reader which in turn, allows the
//! construction of concurrent [`ZipEntryReader`]s. Though, note that each individual [`ZipEntryReader`] cannot be sent
//! between thread boundaries due to the masked lifetime requirement. Therefore, the overarching [`ZipFileReader`]
//! should be cloned and moved into those contexts when needed.
//!
//! ### Concurrent Example
//! ```no_run
//! # use async_zip::base::read::mem::ZipFileReader;
//! # use async_zip::error::Result;
//! # use futures_lite::io::AsyncReadExt;
//! #
//! async fn run() -> Result<()> {
//!     let reader = ZipFileReader::new(Vec::new()).await?;
//!     let result = tokio::join!(read(&reader, 0), read(&reader, 1));
//!
//!     let data_0 = result.0?;
//!     let data_1 = result.1?;
//!
//!     // Use data within current scope.
//!
//!     Ok(())
//! }
//!
//! async fn read(reader: &ZipFileReader, index: usize) -> Result<Vec<u8>> {
//!     let mut entry = reader.reader_without_entry(index).await?;
//!     let mut data = Vec::new();
//!     entry.read_to_end(&mut data).await?;
//!     Ok(data)
//! }
//! ```
//!
//! ### Parallel Example
//! ```no_run
//! # use async_zip::base::read::mem::ZipFileReader;
//! # use async_zip::error::Result;
//! # use futures_lite::io::AsyncReadExt;
//! #
//! async fn run() -> Result<()> {
//!     let reader = ZipFileReader::new(Vec::new()).await?;
//!     
//!     let handle_0 = tokio::spawn(read(reader.clone(), 0));
//!     let handle_1 = tokio::spawn(read(reader.clone(), 1));
//!
//!     let data_0 = handle_0.await.expect("thread panicked")?;
//!     let data_1 = handle_1.await.expect("thread panicked")?;
//!
//!     // Use data within current scope.
//!
//!     Ok(())
//! }
//!
//! async fn read(reader: ZipFileReader, index: usize) -> Result<Vec<u8>> {
//!     let mut entry = reader.reader_without_entry(index).await?;
//!     let mut data = Vec::new();
//!     entry.read_to_end(&mut data).await?;
//!     Ok(data)
//! }
//! ```

#[cfg(doc)]
use crate::base::read::seek;

use crate::base::read::io::entry::ZipEntryReader;
use crate::error::{Result, ZipError};
use crate::file::ZipFile;

use std::sync::Arc;

use futures_lite::io::Cursor;

use super::io::entry::{WithEntry, WithoutEntry};

struct Inner {
    data: Vec<u8>,
    file: ZipFile,
}

// A concurrent ZIP reader which acts over an owned vector of bytes.
#[derive(Clone)]
pub struct ZipFileReader {
    inner: Arc<Inner>,
}

impl ZipFileReader {
    /// Constructs a new ZIP reader from an owned vector of bytes.
    pub async fn new(data: Vec<u8>) -> Result<ZipFileReader> {
        let file = crate::base::read::file(Cursor::new(&data)).await?;
        Ok(ZipFileReader::from_raw_parts(data, file))
    }

    /// Constructs a ZIP reader from an owned vector of bytes and ZIP file information derived from those bytes.
    ///
    /// Providing a [`ZipFile`] that wasn't derived from those bytes may lead to inaccurate parsing.
    pub fn from_raw_parts(data: Vec<u8>, file: ZipFile) -> ZipFileReader {
        ZipFileReader { inner: Arc::new(Inner { data, file }) }
    }

    /// Returns this ZIP file's information.
    pub fn file(&self) -> &ZipFile {
        &self.inner.file
    }

    /// Returns the raw bytes provided to the reader during construction.
    pub fn data(&self) -> &[u8] {
        &self.inner.data
    }

    /// Returns a new entry reader if the provided index is valid.
    pub async fn reader_without_entry(&self, index: usize) -> Result<ZipEntryReader<Cursor<&[u8]>, WithoutEntry>> {
        let stored_entry = self.inner.file.entries.get(index).ok_or(ZipError::EntryIndexOutOfBounds)?;
        let mut cursor = Cursor::new(&self.inner.data[..]);

        stored_entry.seek_to_data_offset(&mut cursor).await?;

        Ok(ZipEntryReader::new_with_owned(
            cursor,
            stored_entry.entry.compression(),
            stored_entry.entry.compressed_size(),
        ))
    }

    /// Returns a new entry reader if the provided index is valid.
    pub async fn reader_with_entry(&self, index: usize) -> Result<ZipEntryReader<Cursor<&[u8]>, WithEntry<'_>>> {
        let stored_entry = self.inner.file.entries.get(index).ok_or(ZipError::EntryIndexOutOfBounds)?;
        let mut cursor = Cursor::new(&self.inner.data[..]);

        stored_entry.seek_to_data_offset(&mut cursor).await?;

        let reader = ZipEntryReader::new_with_owned(
            cursor,
            stored_entry.entry.compression(),
            stored_entry.entry.compressed_size(),
        );

        Ok(reader.into_with_entry(stored_entry))
    }
}
