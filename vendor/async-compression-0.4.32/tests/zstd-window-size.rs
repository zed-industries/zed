#![cfg(not(windows))]

use compression_codecs::zstd::params::DParameter;
use tokio::io::AsyncWriteExt as _;

#[tokio::test]
async fn zstd_decode_large_window_size_default() {
    let compressed = include_bytes!("./artifacts/long-window-size-lib.rs.zst");

    // Default decoder should throw with an error, window size maximum is too low.
    let mut decoder = async_compression::tokio::write::ZstdDecoder::new(Vec::new());
    decoder.write_all(compressed).await.unwrap_err();
}

#[tokio::test]
async fn zstd_decode_large_window_size_explicit_small_window_size() {
    let compressed = include_bytes!("./artifacts/long-window-size-lib.rs.zst");

    // Short window decoder should throw with an error, window size maximum is too low.
    let mut decoder = async_compression::tokio::write::ZstdDecoder::with_params(
        Vec::new(),
        &[DParameter::window_log_max(16)],
    );
    decoder.write_all(compressed).await.unwrap_err();
}

#[tokio::test]
async fn zstd_decode_large_window_size_explicit_large_window_size() {
    let compressed = include_bytes!("./artifacts/long-window-size-lib.rs.zst");
    let source = include_bytes!("./artifacts/lib.rs");

    // Long window decoder should succeed as the window size is large enough to decompress the given input.
    let mut long_window_size_decoder = async_compression::tokio::write::ZstdDecoder::with_params(
        Vec::new(),
        &[DParameter::window_log_max(31)],
    );
    // Long window size decoder should successfully decode the given input data.
    long_window_size_decoder
        .write_all(compressed)
        .await
        .unwrap();
    long_window_size_decoder.shutdown().await.unwrap();

    assert_eq!(long_window_size_decoder.into_inner(), source);
}
