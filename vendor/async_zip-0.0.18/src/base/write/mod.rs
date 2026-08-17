// Copyright (c) 2021-2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A module which supports writing ZIP files.
//!
//! # Example
//! ### Whole data (u8 slice)
//! ```no_run
//! # #[cfg(feature = "deflate")]
//! # {
//! # use async_zip::{Compression, ZipEntryBuilder, base::write::ZipFileWriter};
//! # use async_zip::error::ZipError;
//! #
//! # async fn run() -> Result<(), ZipError> {
//! let mut writer = ZipFileWriter::new(Vec::<u8>::new());
//!
//! let data = b"This is an example file.";
//! let opts = ZipEntryBuilder::new(String::from("foo.txt").into(), Compression::Deflate);
//!
//! writer.write_entry_whole(opts, data).await?;
//! writer.close().await?;
//! #   Ok(())
//! # }
//! # }
//! ```
//! ### Stream data (unknown size & data)
//! ```no_run
//! # #[cfg(feature = "deflate")]
//! # {
//! # use async_zip::{Compression, ZipEntryBuilder, base::write::ZipFileWriter};
//! # use std::io::Cursor;
//! # use async_zip::error::ZipError;
//! # use futures_lite::io::AsyncWriteExt;
//! # use tokio_util::compat::TokioAsyncWriteCompatExt;
//! #
//! # async fn run() -> Result<(), ZipError> {
//! let mut writer = ZipFileWriter::new(Vec::<u8>::new());
//!
//! let data = b"This is an example file.";
//! let opts = ZipEntryBuilder::new(String::from("bar.txt").into(), Compression::Deflate);
//!
//! let mut entry_writer = writer.write_entry_stream(opts).await?;
//! entry_writer.write_all(data).await.unwrap();
//!
//! entry_writer.close().await?;
//! writer.close().await?;
//! #   Ok(())
//! # }
//! # }
//! ```

pub(crate) mod compressed_writer;
pub(crate) mod entry_stream;
pub(crate) mod entry_whole;
pub(crate) mod io;

#[cfg(any(
    feature = "deflate",
    feature = "bzip2",
    feature = "zstd",
    feature = "lzma",
    feature = "xz",
    feature = "deflate64"
))]
pub use entry_whole::compress;

pub use entry_stream::EntryStreamWriter;
pub use entry_whole::crc32;

#[cfg(feature = "tokio")]
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::entry::ZipEntry;
use crate::error::Result;
use crate::spec::extra_field::ExtraFieldAsBytes;
use crate::spec::header::{
    CentralDirectoryRecord, EndOfCentralDirectoryHeader, ExtraField, InfoZipUnicodeCommentExtraField,
    InfoZipUnicodePathExtraField, Zip64EndOfCentralDirectoryLocator, Zip64EndOfCentralDirectoryRecord,
};

#[cfg(feature = "tokio")]
use crate::tokio::write::ZipFileWriter as TokioZipFileWriter;

use entry_whole::EntryWholeWriter;
use io::offset::AsyncOffsetWriter;

use crate::spec::consts::{NON_ZIP64_MAX_NUM_FILES, NON_ZIP64_MAX_SIZE};
use futures_lite::io::{AsyncWrite, AsyncWriteExt};

pub(crate) struct CentralDirectoryEntry {
    pub header: CentralDirectoryRecord,
    pub entry: ZipEntry,
}

/// A ZIP file writer which acts over AsyncWrite implementers.
///
/// # Note
/// - [`ZipFileWriter::close()`] must be called before a stream writer goes out of scope.
pub struct ZipFileWriter<W> {
    pub(crate) writer: AsyncOffsetWriter<W>,
    pub(crate) cd_entries: Vec<CentralDirectoryEntry>,
    /// If true, will error if a Zip64 struct must be written.
    force_no_zip64: bool,
    /// Whether to write Zip64 end of directory structs.
    pub(crate) is_zip64: bool,
    comment_opt: Option<String>,
}

impl<W: AsyncWrite + Unpin> ZipFileWriter<W> {
    /// Construct a new ZIP file writer from a mutable reference to a writer.
    pub fn new(writer: W) -> Self {
        Self {
            writer: AsyncOffsetWriter::new(writer),
            cd_entries: Vec::new(),
            comment_opt: None,
            is_zip64: false,
            force_no_zip64: false,
        }
    }

    /// Force the ZIP writer to operate in non-ZIP64 mode.
    /// If any files would need ZIP64, an error will be raised.
    pub fn force_no_zip64(mut self) -> Self {
        self.force_no_zip64 = true;
        self
    }

    /// Force the ZIP writer to emit Zip64 structs at the end of the archive.
    /// Zip64 extended fields will only be written if needed.
    pub fn force_zip64(mut self) -> Self {
        self.is_zip64 = true;
        self
    }

    /// Write a new ZIP entry of known size and data.
    pub async fn write_entry_whole<E: Into<ZipEntry>>(&mut self, entry: E, data: &[u8]) -> Result<()> {
        EntryWholeWriter::from_raw(self, entry.into(), data).write().await
    }

    /// Write a new ZIP entry of known size and data, with the data already being compressed.
    ///
    /// The provided entry's compression method, CRC, and uncompressed size must be set. Use with `base::write::compress`
    /// and `base::write::crc32` to precompress.
    pub async fn write_entry_whole_precompressed<E: Into<ZipEntry>>(&mut self, entry: E, data: &[u8]) -> Result<()> {
        EntryWholeWriter::from_precompressed(self, entry.into(), data).write().await
    }

    /// Write an entry of unknown size and data via streaming (ie. using a data descriptor).
    /// The generated Local File Header will be invalid, with no compressed size, uncompressed size,
    /// and a null CRC. This might cause problems with the destination reader.
    pub async fn write_entry_stream<E: Into<ZipEntry>>(&mut self, entry: E) -> Result<EntryStreamWriter<'_, W>> {
        EntryStreamWriter::from_raw(self, entry.into()).await
    }

    /// Write an entry of unknown size and data via streaming (ie. using a data descriptor), with the data already being compressed.
    ///
    /// The provided entry's compression method, CRC, and uncompressed size must be set. Use with `base::write::compress`
    /// and `base::write::crc32` to precompress.
    ///
    /// The generated Local File Header will be invalid, with no compressed size, uncompressed size,
    /// and a null CRC. This might cause problems with the destination reader.
    pub async fn write_entry_stream_precompressed<E: Into<ZipEntry>>(
        &mut self,
        entry: E,
    ) -> Result<EntryStreamWriter<'_, W>> {
        EntryStreamWriter::from_raw_precompressed(self, entry.into()).await
    }

    /// Set the ZIP file comment.
    pub fn comment(&mut self, comment: String) {
        self.comment_opt = Some(comment);
    }

    /// Returns a mutable reference to the inner writer.
    ///
    /// Care should be taken when using this inner writer as doing so may invalidate internal state of this writer.
    pub fn inner_mut(&mut self) -> &mut W {
        self.writer.inner_mut()
    }

    /// Consumes this ZIP writer and completes all closing tasks.
    ///
    /// This includes:
    /// - Writing all central directory headers.
    /// - Writing the end of central directory header.
    /// - Writing the file comment.
    ///
    /// Failure to call this function before going out of scope would result in a corrupted ZIP file.
    pub async fn close(mut self) -> Result<W> {
        let cd_offset = self.writer.offset();

        for entry in &self.cd_entries {
            let filename_basic =
                entry.entry.filename().alternative().unwrap_or_else(|| entry.entry.filename().as_bytes());
            let comment_basic = entry.entry.comment().alternative().unwrap_or_else(|| entry.entry.comment().as_bytes());

            self.writer.write_all(&crate::spec::consts::CDH_SIGNATURE.to_le_bytes()).await?;
            self.writer.write_all(&entry.header.as_slice()).await?;
            self.writer.write_all(filename_basic).await?;
            self.writer.write_all(&entry.entry.extra_fields().as_bytes()).await?;
            self.writer.write_all(comment_basic).await?;
        }

        let central_directory_size = self.writer.offset() - cd_offset;
        let central_directory_size_u32 = if central_directory_size > NON_ZIP64_MAX_SIZE as u64 {
            NON_ZIP64_MAX_SIZE
        } else {
            central_directory_size as u32
        };
        let num_entries_in_directory = self.cd_entries.len() as u64;
        let num_entries_in_directory_u16 = if num_entries_in_directory > NON_ZIP64_MAX_NUM_FILES as u64 {
            NON_ZIP64_MAX_NUM_FILES
        } else {
            num_entries_in_directory as u16
        };
        let cd_offset_u32 = if cd_offset > NON_ZIP64_MAX_SIZE as u64 {
            if self.force_no_zip64 {
                return Err(crate::error::ZipError::Zip64Needed(crate::error::Zip64ErrorCase::LargeFile));
            } else {
                self.is_zip64 = true;
            }
            NON_ZIP64_MAX_SIZE
        } else {
            cd_offset as u32
        };

        // Add the zip64 EOCDR and EOCDL if we are in zip64 mode.
        if self.is_zip64 {
            let eocdr_offset = self.writer.offset();

            let eocdr = Zip64EndOfCentralDirectoryRecord {
                size_of_zip64_end_of_cd_record: 44,
                version_made_by: crate::spec::version::as_made_by(),
                version_needed_to_extract: 46,
                disk_number: 0,
                disk_number_start_of_cd: 0,
                num_entries_in_directory_on_disk: num_entries_in_directory,
                num_entries_in_directory,
                directory_size: central_directory_size,
                offset_of_start_of_directory: cd_offset,
            };
            self.writer.write_all(&crate::spec::consts::ZIP64_EOCDR_SIGNATURE.to_le_bytes()).await?;
            self.writer.write_all(&eocdr.as_bytes()).await?;

            let eocdl = Zip64EndOfCentralDirectoryLocator {
                number_of_disk_with_start_of_zip64_end_of_central_directory: 0,
                relative_offset: eocdr_offset,
                total_number_of_disks: 1,
            };
            self.writer.write_all(&crate::spec::consts::ZIP64_EOCDL_SIGNATURE.to_le_bytes()).await?;
            self.writer.write_all(&eocdl.as_bytes()).await?;
        }

        let header = EndOfCentralDirectoryHeader {
            disk_num: 0,
            start_cent_dir_disk: 0,
            num_of_entries_disk: num_entries_in_directory_u16,
            num_of_entries: num_entries_in_directory_u16,
            size_cent_dir: central_directory_size_u32,
            cent_dir_offset: cd_offset_u32,
            file_comm_length: self.comment_opt.as_ref().map(|v| v.len() as u16).unwrap_or_default(),
        };

        self.writer.write_all(&crate::spec::consts::EOCDR_SIGNATURE.to_le_bytes()).await?;
        self.writer.write_all(&header.as_slice()).await?;
        if let Some(comment) = self.comment_opt {
            self.writer.write_all(comment.as_bytes()).await?;
        }

        Ok(self.writer.into_inner())
    }
}

#[cfg(feature = "tokio")]
impl<W> ZipFileWriter<Compat<W>>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    /// Construct a new ZIP file writer from a mutable reference to a writer.
    pub fn with_tokio(writer: W) -> TokioZipFileWriter<W> {
        Self {
            writer: AsyncOffsetWriter::new(writer.compat_write()),
            cd_entries: Vec::new(),
            comment_opt: None,
            is_zip64: false,
            force_no_zip64: false,
        }
    }
}

pub(crate) fn get_or_put_info_zip_unicode_path_extra_field_mut(
    extra_fields: &mut Vec<ExtraField>,
) -> &mut InfoZipUnicodePathExtraField {
    if !extra_fields.iter().any(|field| matches!(field, ExtraField::InfoZipUnicodePath(_))) {
        extra_fields
            .push(ExtraField::InfoZipUnicodePath(InfoZipUnicodePathExtraField::V1 { crc32: 0, unicode: vec![] }));
    }

    for field in extra_fields.iter_mut() {
        if let ExtraField::InfoZipUnicodePath(extra_field) = field {
            return extra_field;
        }
    }

    panic!("InfoZipUnicodePathExtraField not found after insertion")
}

pub(crate) fn get_or_put_info_zip_unicode_comment_extra_field_mut(
    extra_fields: &mut Vec<ExtraField>,
) -> &mut InfoZipUnicodeCommentExtraField {
    if !extra_fields.iter().any(|field| matches!(field, ExtraField::InfoZipUnicodeComment(_))) {
        extra_fields
            .push(ExtraField::InfoZipUnicodeComment(InfoZipUnicodeCommentExtraField::V1 { crc32: 0, unicode: vec![] }));
    }

    for field in extra_fields.iter_mut() {
        if let ExtraField::InfoZipUnicodeComment(extra_field) = field {
            return extra_field;
        }
    }

    panic!("InfoZipUnicodeCommentExtraField not found after insertion")
}
