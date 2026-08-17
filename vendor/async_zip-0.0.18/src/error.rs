// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A module which holds relevant error reporting structures/types.

use std::fmt::{Display, Formatter};
use thiserror::Error;

/// A Result type alias over ZipError to minimise repetition.
pub type Result<V> = std::result::Result<V, ZipError>;

#[derive(Debug, PartialEq, Eq)]
pub enum Zip64ErrorCase {
    TooManyFiles,
    LargeFile,
}

impl Display for Zip64ErrorCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyFiles => write!(f, "More than 65536 files in archive"),
            Self::LargeFile => write!(f, "File is larger than 4 GiB"),
        }
    }
}

/// An enum of possible errors and their descriptions.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ZipError {
    #[error("feature not supported: '{0}'")]
    FeatureNotSupported(&'static str),
    #[error("compression not supported: {0}")]
    CompressionNotSupported(u16),
    #[error("host attribute compatibility not supported: {0}")]
    AttributeCompatibilityNotSupported(u16),
    #[error("attempted to read a ZIP64 file whilst on a 32-bit target")]
    TargetZip64NotSupported,
    #[error("attempted to write a ZIP file with force_no_zip64 when ZIP64 is needed: {0}")]
    Zip64Needed(Zip64ErrorCase),
    #[error("end of file has not been reached")]
    EOFNotReached,
    #[error("extra fields exceeded maximum size")]
    ExtraFieldTooLarge,
    #[error("comment exceeded maximum size")]
    CommentTooLarge,
    #[error("filename exceeded maximum size")]
    FileNameTooLarge,
    #[error("attempted to convert non-UTF8 bytes to a string/str")]
    StringNotUtf8,

    #[error("unable to locate the end of central directory record")]
    UnableToLocateEOCDR,
    #[error("extra field size was indicated to be {0} but only {1} bytes remain")]
    InvalidExtraFieldHeader(u16, usize),
    #[error("zip64 extended information field was incomplete")]
    Zip64ExtendedFieldIncomplete,

    #[error("an upstream reader returned an error: {0}")]
    UpstreamReadError(#[from] std::io::Error),
    #[error("a computed CRC32 value did not match the expected value")]
    CRC32CheckError,
    #[error("entry index was out of bounds")]
    EntryIndexOutOfBounds,
    #[error("Encountered an unexpected header (actual: {0:#x}, expected: {1:#x}).")]
    UnexpectedHeaderError(u32, u32),

    #[error("Info-ZIP Unicode Comment Extra Field was incomplete")]
    InfoZipUnicodeCommentFieldIncomplete,
    #[error("Info-ZIP Unicode Path Extra Field was incomplete")]
    InfoZipUnicodePathFieldIncomplete,
}
