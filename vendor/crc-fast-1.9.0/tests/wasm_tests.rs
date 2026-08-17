//! WASM compatibility tests
//!
//! Tests that the library works in WebAssembly. These tests run natively
//! (test framework requires std) but exercise code paths used in WASM.
//!
//! Run tests: cargo test --test wasm_tests

use crc_fast::{checksum, CrcAlgorithm, Digest};

#[cfg(feature = "alloc")]
use crc_fast::{checksum_combine, checksum_with_params, CrcParams};

/// Test basic CRC calculation
#[test]
fn test_wasm_basic_crc32() {
    let data = b"123456789";
    assert_eq!(checksum(CrcAlgorithm::Crc32IsoHdlc, data), 0xcbf43926);
    assert_eq!(checksum(CrcAlgorithm::Crc32Iscsi, data), 0xe3069283);
}

/// Test CRC-64 variants
#[test]
fn test_wasm_crc64() {
    let data = b"123456789";
    assert_eq!(checksum(CrcAlgorithm::Crc64Nvme, data), 0xae8b14860a799888);
    assert_eq!(checksum(CrcAlgorithm::Crc64Xz, data), 0x995dc9bbdf1939fa);
}

/// Test Digest API (incremental hashing)
#[test]
fn test_wasm_digest() {
    let mut digest = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
    digest.update(b"1234");
    digest.update(b"56789");
    assert_eq!(digest.finalize(), 0xcbf43926);
}

/// Test all CRC-32 algorithms
#[test]
fn test_wasm_all_crc32() {
    let data = b"123456789";
    assert_eq!(checksum(CrcAlgorithm::Crc32IsoHdlc, data), 0xcbf43926);
    assert_eq!(checksum(CrcAlgorithm::Crc32Bzip2, data), 0xfc891918);
    assert_eq!(checksum(CrcAlgorithm::Crc32Iscsi, data), 0xe3069283);
    assert_eq!(checksum(CrcAlgorithm::Crc32Mpeg2, data), 0x0376e6e7);
}

/// Test all CRC-64 algorithms
#[test]
fn test_wasm_all_crc64() {
    let data = b"123456789";
    assert_eq!(
        checksum(CrcAlgorithm::Crc64Ecma182, data),
        0x6c40df5f0b497347
    );
    assert_eq!(checksum(CrcAlgorithm::Crc64Nvme, data), 0xae8b14860a799888);
    assert_eq!(checksum(CrcAlgorithm::Crc64Xz, data), 0x995dc9bbdf1939fa);
}

/// Test various buffer sizes
#[test]
#[cfg(feature = "alloc")]
fn test_wasm_various_sizes() {
    extern crate alloc;
    use alloc::vec::Vec;

    let small = b"hello";
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, small);

    let medium = b"The quick brown fox jumps over the lazy dog";
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, medium);

    let large: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, &large);

    let very_large: Vec<u8> = (0..8192).map(|i| (i % 256) as u8).collect();
    let _ = checksum(CrcAlgorithm::Crc64Nvme, &very_large);
}

/// Test empty input
#[test]
fn test_wasm_empty() {
    let empty: &[u8] = &[];
    assert_eq!(checksum(CrcAlgorithm::Crc32IsoHdlc, empty), 0);
}

/// Test incremental hashing
#[test]
fn test_wasm_incremental() {
    let mut digest = Digest::new(CrcAlgorithm::Crc64Nvme);
    for chunk in [b"123", b"456", b"789"].iter() {
        digest.update(*chunk);
    }
    assert_eq!(digest.finalize(), 0xae8b14860a799888);
}

/// Test digest reset
#[test]
fn test_wasm_reset() {
    let mut digest = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
    digest.update(b"123456789");
    let result1 = digest.finalize();
    digest.reset();
    digest.update(b"123456789");
    let result2 = digest.finalize();
    assert_eq!(result1, result2);
}

/// Test custom CRC parameters
#[test]
#[cfg(feature = "alloc")]
fn test_wasm_custom_params() {
    let params = CrcParams::new(
        "CRC-32/CUSTOM",
        32,
        0x04c11db7,
        0xffffffff,
        true,
        0xffffffff,
        0xcbf43926,
    );
    assert_eq!(checksum_with_params(params, b"123456789"), 0xcbf43926);
}

/// Test checksum combining
#[test]
#[cfg(feature = "alloc")]
fn test_wasm_combine() {
    let crc1 = checksum(CrcAlgorithm::Crc32IsoHdlc, b"1234");
    let crc2 = checksum(CrcAlgorithm::Crc32IsoHdlc, b"56789");
    let combined = checksum_combine(CrcAlgorithm::Crc32IsoHdlc, crc1, crc2, 5);
    let expected = checksum(CrcAlgorithm::Crc32IsoHdlc, b"123456789");
    assert_eq!(combined, expected);
}

/// Test reflected vs non-reflected
#[test]
fn test_wasm_reflection() {
    let data = b"123456789";
    assert_eq!(checksum(CrcAlgorithm::Crc32IsoHdlc, data), 0xcbf43926); // reflected
    assert_eq!(checksum(CrcAlgorithm::Crc32Bzip2, data), 0xfc891918); // non-reflected
}

/// Test standard test vectors
#[test]
fn test_wasm_vectors() {
    let vectors = [
        (b"" as &[u8], CrcAlgorithm::Crc32IsoHdlc, 0_u64),
        (b"a", CrcAlgorithm::Crc32IsoHdlc, 0xe8b7be43),
        (b"abc", CrcAlgorithm::Crc32IsoHdlc, 0x352441c2),
        (b"123456789", CrcAlgorithm::Crc32IsoHdlc, 0xcbf43926),
    ];

    for (data, algo, expected) in &vectors {
        assert_eq!(checksum(*algo, data), *expected);
    }
}
