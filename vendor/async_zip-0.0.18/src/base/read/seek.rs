// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A ZIP reader which acts over a seekable source.
//!
//! ### Example
//! ```no_run
//! # use async_zip::base::read::seek::ZipFileReader;
//! # use async_zip::error::Result;
//! # use futures_lite::io::AsyncReadExt;
//! # use tokio::fs::File;
//! # use tokio_util::compat::TokioAsyncReadCompatExt;
//! # use tokio::io::BufReader;
//! #
//! async fn run() -> Result<()> {
//!     let mut data = BufReader::new(File::open("./foo.zip").await?);
//!     let mut reader = ZipFileReader::new(data.compat()).await?;
//!
//!     let mut data = Vec::new();
//!     let mut entry = reader.reader_without_entry(0).await?;
//!     entry.read_to_end(&mut data).await?;
//!
//!     // Use data within current scope.
//!
//!     Ok(())
//! }
//! ```

use crate::base::read::io::entry::ZipEntryReader;
use crate::error::{Result, ZipError};
use crate::file::ZipFile;

#[cfg(feature = "tokio")]
use crate::tokio::read::seek::ZipFileReader as TokioZipFileReader;

use futures_lite::io::{AsyncBufRead, AsyncSeek};

#[cfg(feature = "tokio")]
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

use super::io::entry::{WithEntry, WithoutEntry};

/// A ZIP reader which acts over a seekable source.
#[derive(Clone)]
pub struct ZipFileReader<R> {
    reader: R,
    file: ZipFile,
}

impl<R> ZipFileReader<R>
where
    R: AsyncBufRead + AsyncSeek + Unpin,
{
    /// Constructs a new ZIP reader from a seekable source.
    pub async fn new(mut reader: R) -> Result<ZipFileReader<R>> {
        let file = crate::base::read::file(&mut reader).await?;
        Ok(ZipFileReader::from_raw_parts(reader, file))
    }

    /// Constructs a ZIP reader from a seekable source and ZIP file information derived from that source.
    ///
    /// Providing a [`ZipFile`] that wasn't derived from that source may lead to inaccurate parsing.
    pub fn from_raw_parts(reader: R, file: ZipFile) -> ZipFileReader<R> {
        ZipFileReader { reader, file }
    }

    /// Returns this ZIP file's information.
    pub fn file(&self) -> &ZipFile {
        &self.file
    }

    /// Returns a mutable reference to the inner seekable source.
    ///
    /// Swapping the source (eg. via std::mem operations) may lead to inaccurate parsing.
    pub fn inner_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Returns the inner seekable source by consuming self.
    pub fn into_inner(self) -> R {
        self.reader
    }

    /// Returns a new entry reader if the provided index is valid.
    pub async fn reader_without_entry(&mut self, index: usize) -> Result<ZipEntryReader<'_, R, WithoutEntry>> {
        let stored_entry = self.file.entries.get(index).ok_or(ZipError::EntryIndexOutOfBounds)?;
        stored_entry.seek_to_data_offset(&mut self.reader).await?;

        Ok(ZipEntryReader::new_with_borrow(
            &mut self.reader,
            stored_entry.entry.compression(),
            stored_entry.entry.compressed_size(),
        ))
    }

    /// Returns a new entry reader if the provided index is valid.
    pub async fn reader_with_entry(&mut self, index: usize) -> Result<ZipEntryReader<'_, R, WithEntry<'_>>> {
        let stored_entry = self.file.entries.get(index).ok_or(ZipError::EntryIndexOutOfBounds)?;

        stored_entry.seek_to_data_offset(&mut self.reader).await?;

        let reader = ZipEntryReader::new_with_borrow(
            &mut self.reader,
            stored_entry.entry.compression(),
            stored_entry.entry.compressed_size(),
        );

        Ok(reader.into_with_entry(stored_entry))
    }

    /// Returns a new entry reader if the provided index is valid.
    /// Consumes self
    pub async fn into_entry<'a>(mut self, index: usize) -> Result<ZipEntryReader<'a, R, WithoutEntry>>
    where
        R: 'a,
    {
        let stored_entry = self.file.entries.get(index).ok_or(ZipError::EntryIndexOutOfBounds)?;

        stored_entry.seek_to_data_offset(&mut self.reader).await?;

        Ok(ZipEntryReader::new_with_owned(
            self.reader,
            stored_entry.entry.compression(),
            stored_entry.entry.compressed_size(),
        ))
    }
}

#[cfg(feature = "tokio")]
impl<R> ZipFileReader<Compat<R>>
where
    R: tokio::io::AsyncBufRead + tokio::io::AsyncSeek + Unpin,
{
    /// Constructs a new tokio-specific ZIP reader from a seekable source.
    pub async fn with_tokio(reader: R) -> Result<TokioZipFileReader<R>> {
        let mut reader = reader.compat();
        let file = crate::base::read::file(&mut reader).await?;
        Ok(ZipFileReader::from_raw_parts(reader, file))
    }
}
