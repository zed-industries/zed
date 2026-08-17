// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use async_zip::{Compression, ZipEntryBuilder, ZipString};
use futures_lite::AsyncWriteExt;

mod common;

#[cfg(feature = "zstd")]
#[tokio::test]
async fn zip_zstd_in_out() {
    let zip_data = common::compress_to_mem(Compression::Zstd).await;
    common::check_decompress_mem(zip_data).await
}

#[cfg(feature = "deflate")]
#[tokio::test]
async fn zip_decompress_in_out() {
    let zip_data = common::compress_to_mem(Compression::Deflate).await;
    common::check_decompress_mem(zip_data).await
}

#[tokio::test]
async fn zip_store_in_out() {
    let zip_data = common::compress_to_mem(Compression::Stored).await;
    common::check_decompress_mem(zip_data).await
}

#[tokio::test]
async fn zip_utf8_extra_in_out_stream() {
    let mut zip_bytes = Vec::with_capacity(10_000);

    {
        // writing
        let content = "Test".as_bytes();
        let mut writer = async_zip::base::write::ZipFileWriter::new(&mut zip_bytes);
        let filename =
            ZipString::new_with_alternative("\u{4E2D}\u{6587}.txt".to_string(), b"\xD6\xD0\xCe\xC4.txt".to_vec());
        let opts = ZipEntryBuilder::new(filename, Compression::Stored);

        let mut entry_writer = writer.write_entry_stream(opts).await.unwrap();
        entry_writer.write_all(content).await.unwrap();
        entry_writer.close().await.unwrap();

        writer.close().await.unwrap();
    }

    {
        // reading
        let zip = async_zip::base::read::mem::ZipFileReader::new(zip_bytes).await.unwrap();
        let zip_entries: Vec<_> = zip.file().entries().to_vec();
        assert_eq!(zip_entries.len(), 1);
        assert_eq!(zip_entries[0].filename().as_str().unwrap(), "\u{4E2D}\u{6587}.txt");
        assert_eq!(zip_entries[0].filename().alternative(), Some(b"\xD6\xD0\xCe\xC4.txt".as_ref()));
    }
}

#[tokio::test]
async fn zip_utf8_extra_in_out_whole() {
    let mut zip_bytes = Vec::with_capacity(10_000);

    {
        // writing
        let content = "Test".as_bytes();
        let mut writer = async_zip::base::write::ZipFileWriter::new(&mut zip_bytes);
        let filename =
            ZipString::new_with_alternative("\u{4E2D}\u{6587}.txt".to_string(), b"\xD6\xD0\xCe\xC4.txt".to_vec());
        let opts = ZipEntryBuilder::new(filename, Compression::Stored);
        writer.write_entry_whole(opts, content).await.unwrap();
        writer.close().await.unwrap();
    }

    {
        // reading
        let zip = async_zip::base::read::mem::ZipFileReader::new(zip_bytes).await.unwrap();
        let zip_entries: Vec<_> = zip.file().entries().to_vec();
        assert_eq!(zip_entries.len(), 1);
        assert_eq!(zip_entries[0].filename().as_str().unwrap(), "\u{4E2D}\u{6587}.txt");
        assert_eq!(zip_entries[0].filename().alternative(), Some(b"\xD6\xD0\xCe\xC4.txt".as_ref()));
    }
}
