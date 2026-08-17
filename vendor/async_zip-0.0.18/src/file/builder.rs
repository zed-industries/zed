// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::{file::ZipFile, string::ZipString};

/// A builder for [`ZipFile`].
pub struct ZipFileBuilder(pub(crate) ZipFile);

impl From<ZipFile> for ZipFileBuilder {
    fn from(file: ZipFile) -> Self {
        Self(file)
    }
}

impl Default for ZipFileBuilder {
    fn default() -> Self {
        ZipFileBuilder(ZipFile { entries: Vec::new(), zip64: false, comment: String::new().into() })
    }
}

impl ZipFileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the file's comment.
    pub fn comment(mut self, comment: ZipString) -> Self {
        self.0.comment = comment;
        self
    }

    /// Consumes this builder and returns a final [`ZipFile`].
    ///
    /// This is equivalent to:
    /// ```
    /// # use async_zip::{ZipFile, ZipFileBuilder};
    /// #
    /// # let builder = ZipFileBuilder::new();
    /// let file: ZipFile = builder.into();
    /// ```
    pub fn build(self) -> ZipFile {
        self.into()
    }
}
