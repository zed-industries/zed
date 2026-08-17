// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::entry::ZipEntry;
#[cfg(any(
    feature = "deflate",
    feature = "bzip2",
    feature = "zstd",
    feature = "lzma",
    feature = "xz",
    feature = "deflate64"
))]
use crate::spec::Compression;

pub(crate) const SPEC_VERSION_MADE_BY: u16 = 63;

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#443
pub fn as_needed_to_extract(entry: &ZipEntry) -> u16 {
    let mut version = match entry.compression() {
        #[cfg(feature = "deflate")]
        Compression::Deflate => 20,
        #[cfg(feature = "deflate64")]
        Compression::Deflate64 => 21,
        #[cfg(feature = "bzip2")]
        Compression::Bz => 46,
        #[cfg(feature = "lzma")]
        Compression::Lzma => 63,
        _ => 10,
    };

    if let Ok(true) = entry.dir() {
        version = std::cmp::max(version, 20);
    }

    version
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#442
pub fn as_made_by() -> u16 {
    // Default to UNIX mapping for the moment.
    3 << 8 | SPEC_VERSION_MADE_BY
}
