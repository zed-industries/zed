// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::entry::ZipEntry;
use crate::spec::{attribute::AttributeCompatibility, header::ExtraField, Compression};
use crate::{date::ZipDateTime, string::ZipString};

/// A builder for [`ZipEntry`].
pub struct ZipEntryBuilder(pub(crate) ZipEntry);

impl From<ZipEntry> for ZipEntryBuilder {
    fn from(entry: ZipEntry) -> Self {
        Self(entry)
    }
}

impl ZipEntryBuilder {
    /// Constructs a new builder which defines the raw underlying data of a ZIP entry.
    ///
    /// A filename and compression method are needed to construct the builder as minimal parameters.
    pub fn new(filename: ZipString, compression: Compression) -> Self {
        Self(ZipEntry::new(filename, compression))
    }

    /// Sets the entry's filename.
    pub fn filename(mut self, filename: ZipString) -> Self {
        self.0.filename = filename;
        self
    }

    /// Sets the entry's compression method.
    pub fn compression(mut self, compression: Compression) -> Self {
        self.0.compression = compression;
        self
    }

    // Sets the entry's CRC32 checksum.
    pub fn crc32<N: Into<u32>>(mut self, crc: N) -> Self {
        self.0.crc32 = crc.into();
        self
    }

    // Sets the entry's compressed size.
    pub fn compressed_size<N: Into<u64>>(mut self, size: N) -> Self {
        self.0.compressed_size = size.into();
        self
    }

    // Sets the entry's uncompressed size.
    pub fn uncompressed_size<N: Into<u64>>(mut self, size: N) -> Self {
        self.0.uncompressed_size = size.into();
        self
    }

    /// Set the deflate compression option.
    ///
    /// If the compression type isn't deflate, this option has no effect.
    #[cfg(any(feature = "deflate", feature = "bzip2", feature = "zstd", feature = "lzma", feature = "xz"))]
    pub fn deflate_option(mut self, option: crate::DeflateOption) -> Self {
        self.0.compression_level = option.into_level();
        self
    }

    /// Sets the entry's attribute host compatibility.
    pub fn attribute_compatibility(mut self, compatibility: AttributeCompatibility) -> Self {
        self.0.attribute_compatibility = compatibility;
        self
    }

    /// Sets the entry's last modification date.
    pub fn last_modification_date(mut self, date: ZipDateTime) -> Self {
        self.0.last_modification_date = date;
        self
    }

    /// Sets the entry's internal file attribute.
    pub fn internal_file_attribute(mut self, attribute: u16) -> Self {
        self.0.internal_file_attribute = attribute;
        self
    }

    /// Sets the entry's external file attribute.
    pub fn external_file_attribute(mut self, attribute: u32) -> Self {
        self.0.external_file_attribute = attribute;
        self
    }

    /// Sets the entry's extra field data.
    pub fn extra_fields(mut self, field: Vec<ExtraField>) -> Self {
        self.0.extra_fields = field;
        self
    }

    /// Sets the entry's file comment.
    pub fn comment(mut self, comment: ZipString) -> Self {
        self.0.comment = comment;
        self
    }

    /// Sets the entry's Unix permissions mode.
    ///
    /// If the attribute host compatibility isn't set to Unix, this will have no effect.
    pub fn unix_permissions(mut self, mode: u16) -> Self {
        if matches!(self.0.attribute_compatibility, AttributeCompatibility::Unix) {
            self.0.external_file_attribute = (self.0.external_file_attribute & 0xFFFF) | (mode as u32) << 16;
        }
        self
    }

    /// Returns a reference to the currently built entry.
    pub fn current(&self) -> &ZipEntry {
        &self.0
    }

    /// Consumes this builder and returns a final [`ZipEntry`].
    ///
    /// This is equivalent to:
    /// ```
    /// # use async_zip::{ZipEntry, ZipEntryBuilder, Compression};
    /// #
    /// # let builder = ZipEntryBuilder::new(String::from("foo.bar").into(), Compression::Stored);
    /// let entry: ZipEntry = builder.into();
    /// ```
    pub fn build(self) -> ZipEntry {
        self.into()
    }
}
