// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::error::{Result, ZipError};

#[cfg(any(feature = "deflate", feature = "bzip2", feature = "zstd", feature = "lzma", feature = "xz"))]
use async_compression::Level;

/// A compression method supported by this crate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    Stored,
    #[cfg(feature = "deflate")]
    Deflate,
    #[cfg(feature = "deflate64")]
    Deflate64,
    #[cfg(feature = "bzip2")]
    Bz,
    #[cfg(feature = "lzma")]
    Lzma,
    #[cfg(feature = "zstd")]
    Zstd,
    #[cfg(feature = "xz")]
    Xz,
}

impl TryFrom<u16> for Compression {
    type Error = ZipError;

    // Convert a u16 stored with little endianness into a supported compression method.
    // https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#445
    fn try_from(value: u16) -> Result<Self> {
        match value {
            0 => Ok(Compression::Stored),
            #[cfg(feature = "deflate")]
            8 => Ok(Compression::Deflate),
            #[cfg(feature = "deflate64")]
            9 => Ok(Compression::Deflate64),
            #[cfg(feature = "bzip2")]
            12 => Ok(Compression::Bz),
            #[cfg(feature = "lzma")]
            14 => Ok(Compression::Lzma),
            #[cfg(feature = "zstd")]
            93 => Ok(Compression::Zstd),
            #[cfg(feature = "xz")]
            95 => Ok(Compression::Xz),
            _ => Err(ZipError::CompressionNotSupported(value)),
        }
    }
}

impl From<&Compression> for u16 {
    // Convert a supported compression method into its relevant u16 stored with little endianness.
    // https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#445
    fn from(compression: &Compression) -> u16 {
        match compression {
            Compression::Stored => 0,
            #[cfg(feature = "deflate")]
            Compression::Deflate => 8,
            #[cfg(feature = "deflate64")]
            Compression::Deflate64 => 9,
            #[cfg(feature = "bzip2")]
            Compression::Bz => 12,
            #[cfg(feature = "lzma")]
            Compression::Lzma => 14,
            #[cfg(feature = "zstd")]
            Compression::Zstd => 93,
            #[cfg(feature = "xz")]
            Compression::Xz => 95,
        }
    }
}

impl From<Compression> for u16 {
    fn from(compression: Compression) -> u16 {
        (&compression).into()
    }
}

/// Level of compression data should be compressed with for deflate.
#[derive(Debug, Clone, Copy)]
pub enum DeflateOption {
    // Normal (-en) compression option was used.
    Normal,

    // Maximum (-exx/-ex) compression option was used.
    Maximum,

    // Fast (-ef) compression option was used.
    Fast,

    // Super Fast (-es) compression option was used.
    Super,

    /// Other implementation defined level.
    Other(i32),
}

#[cfg(any(feature = "deflate", feature = "bzip2", feature = "zstd", feature = "lzma", feature = "xz"))]
impl DeflateOption {
    pub(crate) fn into_level(self) -> Level {
        // FIXME: There's no clear documentation on what these specific levels defined in the ZIP specification relate
        // to. We want to be compatible with any other library, and not specific to `async_compression`'s levels.
        if let Self::Other(l) = self {
            Level::Precise(l)
        } else {
            Level::Default
        }
    }
}
