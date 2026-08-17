#![cfg(not(windows))]

use tokio::io::AsyncWriteExt as _;

#[tokio::test]
async fn trained_zstd_decode_no_dict() {
    let compressed = include_bytes!("./artifacts/lib.rs.zst");

    let mut decoder = async_compression::tokio::write::ZstdDecoder::new(Vec::new());
    decoder.write_all(compressed).await.unwrap_err();
}

#[tokio::test]
async fn trained_zstd_decode_with_dict() {
    let source = include_bytes!("./artifacts/lib.rs");
    let dict = include_bytes!("./artifacts/dictionary-rust");
    let compressed = include_bytes!("./artifacts/lib.rs.zst");

    let mut decoder =
        async_compression::tokio::write::ZstdDecoder::with_dict(Vec::new(), dict).unwrap();
    decoder.write_all(compressed).await.unwrap();
    decoder.shutdown().await.unwrap();

    assert_eq!(decoder.into_inner(), source);
}

#[tokio::test]
async fn trained_zstd_decode_with_wrong_dict() {
    let dict = include_bytes!("./artifacts/dictionary-rust-other");
    let compressed = include_bytes!("./artifacts/lib.rs.zst");

    let mut decoder =
        async_compression::tokio::write::ZstdDecoder::with_dict(Vec::new(), dict).unwrap();
    decoder.write_all(compressed).await.unwrap_err();
}
