// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::base::read::io::compressed::CompressedReader;
use crate::spec::Compression;

compressed_test_helper!(stored_test, Compression::Stored, "foo bar", "foo bar");

#[cfg(feature = "deflate")]
compressed_test_helper!(deflate_test, Compression::Deflate, "foo bar", include_bytes!("deflate.data"));

#[cfg(feature = "bzip2")]
compressed_test_helper!(bz_test, Compression::Bz, "foo bar", include_bytes!("bzip2.data"));

#[cfg(feature = "lzma")]
compressed_test_helper!(lzma_test, Compression::Lzma, "foo bar", include_bytes!("lzma.data"));

#[cfg(feature = "zstd")]
compressed_test_helper!(zstd_test, Compression::Zstd, "foo bar", include_bytes!("zstd.data"));

#[cfg(feature = "xz")]
compressed_test_helper!(xz_test, Compression::Xz, "foo bar", include_bytes!("xz.data"));

/// A helper macro for generating a CompressedReader test using a specific compression method.
macro_rules! compressed_test_helper {
    ($name:ident, $type:expr, $data_raw:expr, $data:expr) => {
        #[cfg(test)]
        #[tokio::test]
        async fn $name() {
            use futures_lite::io::{AsyncReadExt, Cursor};

            let data = $data;
            let data_raw = $data_raw;

            let cursor = Cursor::new(data);
            let mut reader = CompressedReader::new(cursor, $type);

            let mut read_data = String::new();
            reader.read_to_string(&mut read_data).await.expect("read into CompressedReader failed");

            assert_eq!(read_data, data_raw);
        }
    };
}

use compressed_test_helper;
