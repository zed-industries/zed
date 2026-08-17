// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A ZIP reader which acts over a non-seekable source.
//!
//! # API Design
//! As opposed to other readers provided by this crate, it's important that the data of an entry is fully read before
//! the proceeding entry is read. This is as a result of not being able to seek forwards or backwards, so we must end
//! up at the start of the next entry.
//!
//! **We encode this invariant within Rust's type system so that it can be enforced at compile time.**
//!
//! This requires that any transition methods between these encoded types consume the reader and provide a new owned
//! reader back. This is certainly something to keep in mind when working with this reader, but idiomatic code can
//! still be produced nevertheless.
//!
//! # Considerations
//! As the central directory of a ZIP archive is stored at the end of it, a non-seekable reader doesn't have access
//! to it. We have to rely on information provided within the local file header which may not be accurate or complete.
//! This results in:
//! - The inability to read ZIP entries using the combination of a data descriptor and the Stored compression method.
//! - No file comment being available (defaults to an empty string).
//! - No internal or external file attributes being available (defaults to 0).
//! - The extra field data potentially being inconsistent with what's stored in the central directory.
//! - None of the following being available when the entry was written with a data descriptor (defaults to 0):
//!     - CRC
//!     - compressed size
//!     - uncompressed size
//!
//! # Example
//! ```no_run
//! # use futures_lite::io::Cursor;
//! # use async_zip::error::Result;
//! # use async_zip::base::read::stream::ZipFileReader;
//! #
//! # async fn run() -> Result<()> {
//! let mut zip = ZipFileReader::new(Cursor::new([0; 0]));
//!     
//! // Print the name of every file in a ZIP archive.
//! while let Some(entry) = zip.next_with_entry().await? {
//!     println!("File: {}", entry.reader().entry().filename().as_str().unwrap());
//!     zip = entry.skip().await?;
//! }
//! #
//! #     Ok(())
//! # }
//! ```

use crate::base::read::io::entry::ZipEntryReader;
use crate::error::Result;
use crate::error::ZipError;

use crate::spec::consts::DATA_DESCRIPTOR_LENGTH;
use crate::spec::consts::DATA_DESCRIPTOR_SIGNATURE;
use crate::spec::consts::SIGNATURE_LENGTH;
#[cfg(feature = "tokio")]
use crate::tokio::read::stream::Ready as TokioReady;

use futures_lite::io::AsyncBufRead;
use futures_lite::io::AsyncReadExt;

#[cfg(feature = "tokio")]
use tokio_util::compat::TokioAsyncReadCompatExt;

use super::io::entry::WithEntry;
use super::io::entry::WithoutEntry;

/// A type which encodes that [`ZipFileReader`] is ready to open a new entry.
pub struct Ready<R>(R);

/// A type which encodes that [`ZipFileReader`] is currently reading an entry.
pub struct Reading<'a, R, E>(ZipEntryReader<'a, R, E>, bool);

/// A ZIP reader which acts over a non-seekable source.
///
/// See the [module-level docs](.) for more information.
#[derive(Clone)]
pub struct ZipFileReader<S>(S);

impl<'a, R> ZipFileReader<Ready<R>>
where
    R: AsyncBufRead + Unpin + 'a,
{
    /// Constructs a new ZIP reader from a non-seekable source.
    pub fn new(reader: R) -> Self {
        Self(Ready(reader))
    }

    /// Opens the next entry for reading if the central directory hasn’t yet been reached.
    pub async fn next_without_entry(mut self) -> Result<Option<ZipFileReader<Reading<'a, R, WithoutEntry>>>> {
        let entry = match crate::base::read::lfh(&mut self.0 .0).await? {
            Some(entry) => entry,
            None => return Ok(None),
        };

        let length = if entry.data_descriptor { u64::MAX } else { entry.compressed_size };
        let reader = ZipEntryReader::new_with_owned(self.0 .0, entry.compression, length);

        Ok(Some(ZipFileReader(Reading(reader, entry.data_descriptor))))
    }

    /// Opens the next entry for reading if the central directory hasn’t yet been reached.
    pub async fn next_with_entry(mut self) -> Result<Option<ZipFileReader<Reading<'a, R, WithEntry<'a>>>>> {
        let entry = match crate::base::read::lfh(&mut self.0 .0).await? {
            Some(entry) => entry,
            None => return Ok(None),
        };

        let length = if entry.data_descriptor { u64::MAX } else { entry.compressed_size };
        let reader = ZipEntryReader::new_with_owned(self.0 .0, entry.compression, length);
        let data_descriptor = entry.data_descriptor;

        Ok(Some(ZipFileReader(Reading(reader.into_with_entry_owned(entry), data_descriptor))))
    }

    /// Consumes the `ZipFileReader` returning the original `reader`
    pub async fn into_inner(self) -> R {
        self.0 .0
    }
}

#[cfg(feature = "tokio")]
impl<R> ZipFileReader<TokioReady<R>>
where
    R: tokio::io::AsyncBufRead + Unpin,
{
    /// Constructs a new tokio-specific ZIP reader from a non-seekable source.
    pub fn with_tokio(reader: R) -> ZipFileReader<TokioReady<R>> {
        Self(Ready(reader.compat()))
    }
}

impl<'a, R, E> ZipFileReader<Reading<'a, R, E>>
where
    R: AsyncBufRead + Unpin,
{
    /// Returns an immutable reference to the inner entry reader.
    pub fn reader(&self) -> &ZipEntryReader<'a, R, E> {
        &self.0 .0
    }

    /// Returns a mutable reference to the inner entry reader.
    pub fn reader_mut(&mut self) -> &mut ZipEntryReader<'a, R, E> {
        &mut self.0 .0
    }

    /// Converts the reader back into the Ready state if EOF has been reached.
    pub async fn done(mut self) -> Result<ZipFileReader<Ready<R>>> {
        if self.0 .0.read(&mut [0; 1]).await? != 0 {
            return Err(ZipError::EOFNotReached);
        }

        let mut inner = self.0 .0.into_inner();

        // Has data descriptor.
        if self.0 .1 {
            consume_data_descriptor(&mut inner).await?;
        }

        Ok(ZipFileReader(Ready(inner)))
    }

    /// Reads until EOF and converts the reader back into the Ready state.
    pub async fn skip(mut self) -> Result<ZipFileReader<Ready<R>>> {
        while self.0 .0.read(&mut [0; 2048]).await? != 0 {}
        let mut inner = self.0 .0.into_inner();

        // Has data descriptor.
        if self.0 .1 {
            consume_data_descriptor(&mut inner).await?;
        }

        Ok(ZipFileReader(Ready(inner)))
    }
}

async fn consume_data_descriptor<R: AsyncBufRead + Unpin>(reader: &mut R) -> Result<()> {
    let mut descriptor: [u8; DATA_DESCRIPTOR_LENGTH] = [0; DATA_DESCRIPTOR_LENGTH];
    reader.read_exact(&mut descriptor).await?;

    if descriptor[0..SIGNATURE_LENGTH] == DATA_DESCRIPTOR_SIGNATURE.to_le_bytes() {
        let mut tail: [u8; SIGNATURE_LENGTH] = [0; SIGNATURE_LENGTH];
        reader.read_exact(&mut tail).await?;
    }

    Ok(())
}
