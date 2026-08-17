// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use tokio::io::BufReader;
use tokio_util::compat::TokioAsyncReadCompatExt;

mod common;

const ZSTD_ZIP_FILE: &str = "tests/test_inputs/sample_data.zstd.zip";
const DEFLATE_ZIP_FILE: &str = "tests/test_inputs/sample_data.deflate.zip";
const STORE_ZIP_FILE: &str = "tests/test_inputs/sample_data.store.zip";
const UTF8_EXTRA_ZIP_FILE: &str = "tests/test_inputs/sample_data_utf8_extra.zip";

#[cfg(feature = "zstd")]
#[tokio::test]
async fn decompress_zstd_zip_seek() {
    common::check_decompress_seek(ZSTD_ZIP_FILE).await
}

#[cfg(feature = "deflate")]
#[tokio::test]
async fn decompress_deflate_zip_seek() {
    common::check_decompress_seek(DEFLATE_ZIP_FILE).await
}

#[tokio::test]
async fn check_empty_zip_seek() {
    let mut data: Vec<u8> = Vec::new();
    async_zip::base::write::ZipFileWriter::new(futures::io::Cursor::new(&mut data)).close().await.unwrap();
    async_zip::base::read::seek::ZipFileReader::new(futures::io::Cursor::new(&data)).await.unwrap();
}

#[tokio::test]
async fn decompress_store_zip_seek() {
    common::check_decompress_seek(STORE_ZIP_FILE).await
}

#[cfg(feature = "zstd")]
#[tokio::test]
async fn decompress_zstd_zip_mem() {
    let content = tokio::fs::read(ZSTD_ZIP_FILE).await.unwrap();
    common::check_decompress_mem(content).await
}

#[cfg(feature = "deflate")]
#[tokio::test]
async fn decompress_deflate_zip_mem() {
    let content = tokio::fs::read(DEFLATE_ZIP_FILE).await.unwrap();
    common::check_decompress_mem(content).await
}

#[tokio::test]
async fn decompress_store_zip_mem() {
    let content = tokio::fs::read(STORE_ZIP_FILE).await.unwrap();
    common::check_decompress_mem(content).await
}

#[cfg(feature = "zstd")]
#[cfg(feature = "tokio-fs")]
#[tokio::test]
async fn decompress_zstd_zip_fs() {
    common::check_decompress_fs(ZSTD_ZIP_FILE).await
}

#[cfg(feature = "deflate")]
#[cfg(feature = "tokio-fs")]
#[tokio::test]
async fn decompress_deflate_zip_fs() {
    common::check_decompress_fs(DEFLATE_ZIP_FILE).await
}

#[cfg(feature = "tokio-fs")]
#[tokio::test]
async fn decompress_store_zip_fs() {
    common::check_decompress_fs(STORE_ZIP_FILE).await
}

#[tokio::test]
async fn decompress_zip_with_utf8_extra() {
    let file = BufReader::new(tokio::fs::File::open(UTF8_EXTRA_ZIP_FILE).await.unwrap());
    let mut file_compat = file.compat();
    let zip = async_zip::base::read::seek::ZipFileReader::new(&mut file_compat).await.unwrap();
    let zip_entries: Vec<_> = zip.file().entries().to_vec();
    assert_eq!(zip_entries.len(), 1);
    assert_eq!(zip_entries[0].header_size(), 93);
    assert_eq!(zip_entries[0].filename().as_str().unwrap(), "\u{4E2D}\u{6587}.txt");
    assert_eq!(zip_entries[0].filename().alternative(), Some(b"\xD6\xD0\xCe\xC4.txt".as_ref()));
}
