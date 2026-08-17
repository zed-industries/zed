#![feature(test)]
extern crate test;

use crc::*;
use test::{black_box, Bencher};

pub const BLUETOOTH: Crc<u8> = Crc::<u8>::new(&CRC_8_BLUETOOTH);
pub const BLUETOOTH_SLICE16: Crc<u8, Table<16>> = Crc::<u8, Table<16>>::new(&CRC_8_BLUETOOTH);
pub const BLUETOOTH_BYTEWISE: Crc<u8, Table<1>> = Crc::<u8, Table<1>>::new(&CRC_8_BLUETOOTH);
pub const BLUETOOTH_NOLOOKUP: Crc<u8, NoTable> = Crc::<u8, NoTable>::new(&CRC_8_BLUETOOTH);
pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
pub const X25_SLICE16: Crc<u16, Table<16>> = Crc::<u16, Table<16>>::new(&CRC_16_IBM_SDLC);
pub const X25_BYTEWISE: Crc<u16, Table<1>> = Crc::<u16, Table<1>>::new(&CRC_16_IBM_SDLC);
pub const X25_NOLOOKUP: Crc<u16, NoTable> = Crc::<u16, NoTable>::new(&CRC_16_IBM_SDLC);
pub const ISCSI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
pub const ISCSI_SLICE16: Crc<u32, Table<16>> = Crc::<u32, Table<16>>::new(&CRC_32_ISCSI);
pub const ISCSI_BYTEWISE: Crc<u32, Table<1>> = Crc::<u32, Table<1>>::new(&CRC_32_ISCSI);
pub const ISCSI_NOLOOKUP: Crc<u32, NoTable> = Crc::<u32, NoTable>::new(&CRC_32_ISCSI);
pub const GSM_40: Crc<u64> = Crc::<u64>::new(&CRC_40_GSM);
pub const ECMA: Crc<u64> = Crc::<u64>::new(&CRC_64_ECMA_182);
pub const ECMA_SLICE16: Crc<u64, Table<16>> = Crc::<u64, Table<16>>::new(&CRC_64_ECMA_182);
pub const ECMA_BYTEWISE: Crc<u64, Table<1>> = Crc::<u64, Table<1>>::new(&CRC_64_ECMA_182);
pub const ECMA_NOLOOKUP: Crc<u64, NoTable> = Crc::<u64, NoTable>::new(&CRC_64_ECMA_182);
pub const DARC: Crc<u128> = Crc::<u128>::new(&CRC_82_DARC);
pub const DARC_SLICE16: Crc<u128, Table<16>> = Crc::<u128, Table<16>>::new(&CRC_82_DARC);
pub const DARC_BYTEWISE: Crc<u128, Table<1>> = Crc::<u128, Table<1>>::new(&CRC_82_DARC);
pub const DARC_NOLOOKUP: Crc<u128, NoTable> = Crc::<u128, NoTable>::new(&CRC_82_DARC);

static KB: usize = 1024;

// Baseline benchmark
fn baseline(data: &[u8]) -> usize {
    data.iter()
        .fold(0usize, |acc, v| acc.wrapping_add(*v as usize))
}

#[bench]
fn bench_baseline(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| baseline(black_box(&bytes)));
}

// CRC-8 benchmarks
#[bench]
fn bench_crc8_nolookup(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| BLUETOOTH_NOLOOKUP.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc8_bytewise(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| BLUETOOTH_BYTEWISE.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc8_slice16(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| BLUETOOTH_SLICE16.checksum(black_box(&bytes)));
}

// CRC-16 benchmarks
#[bench]
fn bench_crc16_nolookup(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| X25_NOLOOKUP.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc16_bytewise(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| X25_BYTEWISE.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc16_slice16(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| X25_SLICE16.checksum(black_box(&bytes)));
}

// CRC-32 benchmarks
#[bench]
fn bench_crc32_nolookup(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| ISCSI_NOLOOKUP.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc32_bytewise(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| ISCSI_BYTEWISE.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc32_slice16(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| ISCSI_SLICE16.checksum(black_box(&bytes)));
}

// CRC-64 benchmarks
#[bench]
fn bench_crc64_nolookup(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| ECMA_NOLOOKUP.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc64_bytewise(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| ECMA_BYTEWISE.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc64_slice16(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| ECMA_SLICE16.checksum(black_box(&bytes)));
}

// CRC-82 benchmarks
#[bench]
fn bench_crc82_nolookup(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| DARC_NOLOOKUP.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc82_bytewise(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| DARC_BYTEWISE.checksum(black_box(&bytes)));
}

#[bench]
fn bench_crc82_slice16(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| DARC_SLICE16.checksum(black_box(&bytes)));
}

// Miscellaneous benchmarks
#[bench]
fn bench_crc40(b: &mut Bencher) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];
    b.bytes = size as u64;
    b.iter(|| GSM_40.checksum(black_box(&bytes)));
}
