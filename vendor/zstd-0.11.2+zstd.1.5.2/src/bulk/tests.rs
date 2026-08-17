use super::{compress, decompress};

const TEXT: &str = include_str!("../../assets/example.txt");

#[test]
fn test_direct() {
    // Can we include_str!("assets/example.txt")?
    // It's excluded from the packaging step, so maybe not.
    crate::test_cycle_unwrap(
        TEXT.as_bytes(),
        |data| compress(data, 1),
        |data| decompress(data, TEXT.len()),
    );
}

#[test]
fn test_stream_compat() {
    // We can bulk-compress and stream-decode
    crate::test_cycle_unwrap(
        TEXT.as_bytes(),
        |data| compress(data, 1),
        |data| crate::decode_all(data),
    );

    // We can stream-encode and bulk-decompress
    crate::test_cycle_unwrap(
        TEXT.as_bytes(),
        |data| crate::encode_all(data, 1),
        |data| decompress(data, TEXT.len()),
    );
}

#[test]
fn has_content_size() {
    let compressed = compress(TEXT.as_bytes(), 1).unwrap();

    // Bulk functions by default include the content size.
    assert_eq!(
        zstd_safe::get_frame_content_size(&compressed),
        TEXT.len() as u64
    );
}
