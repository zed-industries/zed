// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

pub(crate) mod builder;

use crate::{entry::StoredZipEntry, string::ZipString};
use builder::ZipFileBuilder;

/// An immutable store of data about a ZIP file.
#[derive(Clone)]
pub struct ZipFile {
    pub(crate) entries: Vec<StoredZipEntry>,
    pub(crate) zip64: bool,
    pub(crate) comment: ZipString,
}

impl From<ZipFileBuilder> for ZipFile {
    fn from(builder: ZipFileBuilder) -> Self {
        builder.0
    }
}

impl ZipFile {
    /// Returns a list of this ZIP file's entries.
    pub fn entries(&self) -> &[StoredZipEntry] {
        &self.entries
    }

    /// Returns this ZIP file's trailing comment.
    pub fn comment(&self) -> &ZipString {
        &self.comment
    }

    /// Returns whether or not this ZIP file is zip64
    pub fn zip64(&self) -> bool {
        self.zip64
    }
}
