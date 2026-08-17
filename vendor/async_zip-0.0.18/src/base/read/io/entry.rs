// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::base::read::io::{compressed::CompressedReader, hashed::HashedReader, owned::OwnedReader};
use crate::entry::ZipEntry;
use crate::error::{Result, ZipError};
use crate::spec::Compression;

use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::io::{AsyncBufRead, AsyncRead, AsyncReadExt, Take};
use pin_project::pin_project;

/// A type which encodes that [`ZipEntryReader`] has associated entry data.
pub struct WithEntry<'a>(OwnedEntry<'a>);

/// A type which encodes that [`ZipEntryReader`] has no associated entry data.
pub struct WithoutEntry;

/// A ZIP entry reader which may implement decompression.
#[pin_project]
pub struct ZipEntryReader<'a, R, E> {
    #[pin]
    reader: HashedReader<CompressedReader<Take<OwnedReader<'a, R>>>>,
    entry: E,
}

impl<'a, R> ZipEntryReader<'a, R, WithoutEntry>
where
    R: AsyncBufRead + Unpin,
{
    /// Constructs a new entry reader from its required parameters (incl. an owned R).
    pub(crate) fn new_with_owned(reader: R, compression: Compression, size: u64) -> Self {
        let reader = HashedReader::new(CompressedReader::new(OwnedReader::Owned(reader).take(size), compression));
        Self { reader, entry: WithoutEntry }
    }

    /// Constructs a new entry reader from its required parameters (incl. a mutable borrow of an R).
    pub(crate) fn new_with_borrow(reader: &'a mut R, compression: Compression, size: u64) -> Self {
        let reader = HashedReader::new(CompressedReader::new(OwnedReader::Borrow(reader).take(size), compression));
        Self { reader, entry: WithoutEntry }
    }

    pub(crate) fn into_with_entry(self, entry: &'a ZipEntry) -> ZipEntryReader<'a, R, WithEntry<'a>> {
        ZipEntryReader { reader: self.reader, entry: WithEntry(OwnedEntry::Borrow(entry)) }
    }

    pub(crate) fn into_with_entry_owned(self, entry: ZipEntry) -> ZipEntryReader<'a, R, WithEntry<'a>> {
        ZipEntryReader { reader: self.reader, entry: WithEntry(OwnedEntry::Owned(entry)) }
    }
}

impl<'a, R, E> AsyncRead for ZipEntryReader<'a, R, E>
where
    R: AsyncBufRead + Unpin,
{
    fn poll_read(self: Pin<&mut Self>, c: &mut Context<'_>, b: &mut [u8]) -> Poll<std::io::Result<usize>> {
        self.project().reader.poll_read(c, b)
    }
}

impl<'a, R, E> ZipEntryReader<'a, R, E>
where
    R: AsyncBufRead + Unpin,
{
    /// Computes and returns the CRC32 hash of bytes read by this reader so far.
    ///
    /// This hash should only be computed once EOF has been reached.
    pub fn compute_hash(&mut self) -> u32 {
        self.reader.swap_and_compute_hash()
    }

    /// Consumes this reader and returns the inner value.
    pub(crate) fn into_inner(self) -> R {
        self.reader.into_inner().into_inner().into_inner().owned_into_inner()
    }
}

impl<R> ZipEntryReader<'_, R, WithEntry<'_>>
where
    R: AsyncBufRead + Unpin,
{
    /// Returns an immutable reference to the associated entry data.
    pub fn entry(&self) -> &'_ ZipEntry {
        self.entry.0.entry()
    }

    /// Reads all bytes until EOF has been reached, appending them to buf, and verifies the CRC32 values.
    ///
    /// This is a helper function synonymous to [`AsyncReadExt::read_to_end()`].
    pub async fn read_to_end_checked(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let read = self.read_to_end(buf).await?;

        if self.compute_hash() == self.entry.0.entry().crc32() {
            Ok(read)
        } else {
            Err(ZipError::CRC32CheckError)
        }
    }

    /// Reads all bytes until EOF has been reached, placing them into buf, and verifies the CRC32 values.
    ///
    /// This is a helper function synonymous to [`AsyncReadExt::read_to_string()`].
    pub async fn read_to_string_checked(&mut self, buf: &mut String) -> Result<usize> {
        let read = self.read_to_string(buf).await?;

        if self.compute_hash() == self.entry.0.entry().crc32() {
            Ok(read)
        } else {
            Err(ZipError::CRC32CheckError)
        }
    }
}

enum OwnedEntry<'a> {
    Owned(ZipEntry),
    Borrow(&'a ZipEntry),
}

impl<'a> OwnedEntry<'a> {
    pub fn entry(&self) -> &'_ ZipEntry {
        match self {
            OwnedEntry::Owned(entry) => entry,
            OwnedEntry::Borrow(entry) => entry,
        }
    }
}
