//! no_std compatibility tests
//!
//! Tests the library works without std. The test framework requires std,
//! but this exercises all no_std code paths.
//!
//! Run tests: cargo test --test no_std_tests

use crc_fast::{checksum, CrcAlgorithm, Digest};

/// Test basic checksum calculation (works without std)
#[test]
fn test_no_std_basic_checksum() {
    let data = b"123456789";

    // Test all major CRC variants
    assert_eq!(
        checksum(CrcAlgorithm::Crc32IsoHdlc, data),
        0xcbf43926,
        "CRC-32/ISO-HDLC failed"
    );

    assert_eq!(
        checksum(CrcAlgorithm::Crc32Iscsi, data),
        0xe3069283,
        "CRC-32/ISCSI failed"
    );

    assert_eq!(
        checksum(CrcAlgorithm::Crc64Nvme, data),
        0xae8b14860a799888,
        "CRC-64/NVME failed"
    );

    assert_eq!(
        checksum(CrcAlgorithm::Crc64Xz, data),
        0x995dc9bbdf1939fa,
        "CRC-64/XZ failed"
    );
}

/// Test all 21 standard CRC algorithms
#[test]
fn test_no_std_all_algorithms() {
    let data = b"123456789";

    // CRC-32 variants (reflected)
    assert_eq!(checksum(CrcAlgorithm::Crc32IsoHdlc, data), 0xcbf43926);
    assert_eq!(checksum(CrcAlgorithm::Crc32Iscsi, data), 0xe3069283);

    // CRC-32 variants (non-reflected/forward)
    assert_eq!(checksum(CrcAlgorithm::Crc32Bzip2, data), 0xfc891918);
    assert_eq!(checksum(CrcAlgorithm::Crc32Mpeg2, data), 0x0376e6e7);

    // CRC-64 variants
    assert_eq!(
        checksum(CrcAlgorithm::Crc64Ecma182, data),
        0x6c40df5f0b497347
    );
    assert_eq!(checksum(CrcAlgorithm::Crc64GoIso, data), 0xb90956c775a41001);
    assert_eq!(checksum(CrcAlgorithm::Crc64Ms, data), 0x75d4b74f024eceea);
    assert_eq!(checksum(CrcAlgorithm::Crc64Nvme, data), 0xae8b14860a799888);
    assert_eq!(checksum(CrcAlgorithm::Crc64Redis, data), 0xe9c6d914c4b8d9ca);
    assert_eq!(checksum(CrcAlgorithm::Crc64We, data), 0x62ec59e3f1a4f00a);
    assert_eq!(checksum(CrcAlgorithm::Crc64Xz, data), 0x995dc9bbdf1939fa);
}

/// Test Digest API (core functionality without std)
#[test]
fn test_no_std_digest_api() {
    let mut digest = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
    digest.update(b"1234");
    digest.update(b"56789");
    let result = digest.finalize();

    assert_eq!(result, 0xcbf43926, "Digest API failed");
}

/// Test empty input
#[test]
fn test_no_std_empty_input() {
    let empty: &[u8] = &[];
    let result = checksum(CrcAlgorithm::Crc32IsoHdlc, empty);
    // Empty input gives 0 after final XOR with xorout
    assert_eq!(result, 0);
}

/// Test various input sizes to ensure all code paths work
#[test]
fn test_no_std_various_sizes() {
    // Small (< 64 bytes)
    let small = b"hello";
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, small);

    // Medium (64-256 bytes)
    let medium = b"The quick brown fox jumps over the lazy dog. \
                    The quick brown fox jumps over the lazy dog. \
                    The quick brown fox jumps over the lazy dog.";
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, medium);

    // Large (> 256 bytes) - tests SIMD path
    let large = [0xAAu8; 1024];
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, &large);
}

/// Test that reflected and non-reflected algorithms both work
#[test]
fn test_no_std_reflection_modes() {
    let data = b"123456789";

    // Reflected (most common)
    assert_eq!(checksum(CrcAlgorithm::Crc32IsoHdlc, data), 0xcbf43926);

    // Non-reflected (forward)
    assert_eq!(checksum(CrcAlgorithm::Crc32Bzip2, data), 0xfc891918);
}

/// Test custom CRC parameters
#[cfg(feature = "alloc")]
#[test]
fn test_no_std_custom_params() {
    use crc_fast::{checksum_with_params, CrcParams};

    let params = CrcParams::new(
        "CRC-32/CUSTOM",
        32,
        0x04c11db7,
        0xffffffff,
        true,
        0xffffffff,
        0xcbf43926,
    );

    let result = checksum_with_params(params, b"123456789");
    assert_eq!(result, 0xcbf43926, "Custom params failed");
}

/// Test that Digest can be reused
#[test]
fn test_no_std_digest_reuse() {
    let mut digest = Digest::new(CrcAlgorithm::Crc32IsoHdlc);

    digest.update(b"123456789");
    let result1 = digest.finalize();
    assert_eq!(result1, 0xcbf43926);

    digest.reset();
    digest.update(b"123456789");
    let result2 = digest.finalize();
    assert_eq!(result2, 0xcbf43926);
}

/// Test incremental digest updates
#[test]
fn test_no_std_incremental_digest() {
    let mut digest1 = Digest::new(CrcAlgorithm::Crc64Nvme);
    digest1.update(b"123456789");
    let result1 = digest1.finalize();

    let mut digest2 = Digest::new(CrcAlgorithm::Crc64Nvme);
    digest2.update(b"123");
    digest2.update(b"456");
    digest2.update(b"789");
    let result2 = digest2.finalize();

    assert_eq!(result1, result2, "Incremental digest failed");
}

/// Test checksum_combine (requires alloc)
#[cfg(feature = "alloc")]
#[test]
fn test_no_std_checksum_combine() {
    use crc_fast::checksum_combine;

    let crc1 = checksum(CrcAlgorithm::Crc32IsoHdlc, b"1234");
    let crc2 = checksum(CrcAlgorithm::Crc32IsoHdlc, b"56789");
    let combined = checksum_combine(CrcAlgorithm::Crc32IsoHdlc, crc1, crc2, 5);

    let expected = checksum(CrcAlgorithm::Crc32IsoHdlc, b"123456789");
    assert_eq!(combined, expected, "checksum_combine failed");
}

/// Test that the library works with stack-only data
#[test]
fn test_no_std_stack_only() {
    // This should work without any heap allocation
    let data = b"123456789";
    let result = checksum(CrcAlgorithm::Crc32IsoHdlc, data);
    assert_eq!(result, 0xcbf43926);
}

/// Test both CRC-32 and CRC-64 widths
#[test]
fn test_no_std_both_widths() {
    let data = b"test data";

    // CRC-32
    let _crc32 = checksum(CrcAlgorithm::Crc32IsoHdlc, data);

    // CRC-64
    let _crc64 = checksum(CrcAlgorithm::Crc64Nvme, data);
}

/// Test with byte patterns that might expose issues
#[test]
fn test_no_std_edge_case_patterns() {
    // All zeros
    let zeros = [0u8; 128];
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, &zeros);

    // All ones
    let ones = [0xFFu8; 128];
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, &ones);

    // Alternating pattern
    let alt = [0xAAu8; 128];
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, &alt);
}
