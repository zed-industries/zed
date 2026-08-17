// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

use crc_fast::checksum;
use crc_fast::CrcAlgorithm;
use criterion::*;
use rand::{rng, RngCore};
use std::hint::black_box;
use std::time::Duration;

pub const SIZES: &[(&str, i32)] = &[
    ("1 MiB", 1024 * 1024),
    //("512 KiB", 512 * 1024),
    //("256 KiB", 256 * 1024),
    //("128 KiB", 128 * 1024),
    //("64 KiB", 64 * 1024),
    //("16 KiB", 16 * 1024),
    //("8 KiB", 8 * 1024),
    //("4 KiB", 4 * 1024),
    //("2 KiB", 4 * 1024),
    ("1 KiB", 1024),
    //("768 bytes", 768),
    //("512 bytes", 512),
    //("256 bytes", 256),
    //("255 bytes", 255),
    //("128 bytes", 128),
    //("127 bytes", 127),
    //("64 bytes", 64),
    //("32 bytes", 32),
    //("16 bytes", 16),
    //("1 bytes", 1),
];

// these are the most important algorithms in popular use, with forward/reflected coverage
pub const CRC32_ALGORITHMS: &[CrcAlgorithm] = &[
    // benchmark both CRC-32/ISCSI and CRC-32/ISO-HDLC since they're special flowers with lots of
    // different acceleration targets.
    CrcAlgorithm::Crc32Autosar, // reflected
    CrcAlgorithm::Crc32Iscsi,   // reflected, fusion
    CrcAlgorithm::Crc32IsoHdlc, // reflected, fusion
    CrcAlgorithm::Crc32Bzip2,   // forward
];

// these are the most important algorithms in popular use, with forward/reflected coverage
pub const CRC64_ALGORITHMS: &[CrcAlgorithm] = &[
    CrcAlgorithm::Crc64Ecma182, // forward
    CrcAlgorithm::Crc64Nvme,    // reflected
];

#[inline(always)]
fn random_data(size: i32) -> Vec<u8> {
    let mut rng = rng();
    let mut buf = vec![0u8; size as usize];
    rng.fill_bytes(&mut buf);

    buf
}

fn create_aligned_data(input: &[u8]) -> Vec<u8> {
    // Size of our target alignment structure
    let align_size = std::mem::size_of::<[[u64; 4]; 2]>(); // 64 bytes

    // Create a zero-filled vector with padding to ensure we can find a properly aligned position
    let mut padded = vec![0; input.len() + align_size];

    // Find the first address that satisfies our alignment
    let start_addr = padded.as_ptr() as usize;
    let align_offset = (align_size - (start_addr % align_size)) % align_size;

    // Copy the input into the aligned position
    let aligned_start = &mut padded[align_offset..];
    aligned_start[..input.len()].copy_from_slice(input);

    // Return the exact slice we need
    aligned_start[..input.len()].to_vec()
}

#[inline(always)]
fn bench_crc32(c: &mut Criterion) {
    let mut group = c.benchmark_group("CRC-32");

    println!(
        "Acceleration target: {}",
        crc_fast::get_calculator_target(CrcAlgorithm::Crc32Iscsi)
    );

    for (size_name, size) in SIZES {
        let buf = create_aligned_data(&random_data(*size));

        let (part1, rest) = buf.split_at(buf.len() / 4);
        let (part2, rest) = rest.split_at(rest.len() / 3);
        let (part3, part4) = rest.split_at(rest.len() / 2);

        for algorithm in CRC32_ALGORITHMS {
            let algorithm_name = algorithm.to_string();
            let mut algorithm_name_parts = algorithm_name.split('/');
            let _ = algorithm_name_parts.next();
            let alg_suffix = algorithm_name_parts.next();

            group.throughput(Throughput::Bytes(*size as u64));
            group.sample_size(1000);
            group.measurement_time(Duration::from_secs(30));

            let bench_name = [alg_suffix.unwrap(), "(checksum)"].join(" ");

            group.bench_function(BenchmarkId::new(bench_name, size_name), |b| {
                b.iter(|| black_box(checksum(*algorithm, &buf)))
            });

            let bench_name = [algorithm_name.clone(), "(4-part digest)".parse().unwrap()].join(" ");

            group.bench_function(BenchmarkId::new(bench_name, size_name), |b| {
                b.iter(|| {
                    black_box({
                        let mut digest = crc_fast::Digest::new(*algorithm);
                        digest.update(part1);
                        digest.update(part2);
                        digest.update(part3);
                        digest.update(part4);
                        digest.finalize()
                    })
                })
            });
        }
    }
}

#[inline(always)]
fn bench_crc64(c: &mut Criterion) {
    println!(
        "Acceleration target: {}",
        crc_fast::get_calculator_target(CrcAlgorithm::Crc64Nvme)
    );

    let mut group = c.benchmark_group("CRC-64");

    for (size_name, size) in SIZES {
        let buf = create_aligned_data(&random_data(*size));

        let (part1, rest) = buf.split_at(buf.len() / 4);
        let (part2, rest) = rest.split_at(rest.len() / 3);
        let (part3, part4) = rest.split_at(rest.len() / 2);

        for algorithm in CRC64_ALGORITHMS {
            let algorithm_name = algorithm.to_string();
            let mut algorithm_name_parts = algorithm_name.split('/');
            let _ = algorithm_name_parts.next();
            let alg_suffix = algorithm_name_parts.next();

            group.throughput(Throughput::Bytes(*size as u64));
            group.sample_size(1000);
            group.measurement_time(Duration::from_secs(30));

            let bench_name = [alg_suffix.unwrap(), "(checksum)"].join(" ");

            group.bench_function(BenchmarkId::new(bench_name, size_name), |b| {
                b.iter(|| black_box(checksum(*algorithm, &buf)))
            });

            let bench_name = [algorithm_name.clone(), "(4-part digest)".parse().unwrap()].join(" ");

            group.bench_function(BenchmarkId::new(bench_name, size_name), |b| {
                b.iter(|| {
                    black_box({
                        let mut digest = crc_fast::Digest::new(*algorithm);
                        digest.update(part1);
                        digest.update(part2);
                        digest.update(part3);
                        digest.update(part4);
                        digest.finalize()
                    })
                })
            });
        }
    }
}

criterion_group!(benches, bench_crc32, bench_crc64);

criterion_main!(benches);
