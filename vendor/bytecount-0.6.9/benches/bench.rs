#[macro_use]
extern crate criterion;
extern crate bytecount;
extern crate rand;

use criterion::{Bencher, BenchmarkId, Criterion};
use rand::RngCore;
use std::env;
use std::time::Duration;

use bytecount::{count, naive_count, naive_count_32, naive_num_chars, num_chars};

fn random_bytes(len: usize) -> Vec<u8> {
    let mut result = vec![0; len];
    rand::thread_rng().fill_bytes(&mut result);
    result
}

static COUNTS: &[usize] = &[
    0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 120, 140, 170, 210, 250, 300, 400, 500, 600, 700,
    800, 900, 1_000, 1_200, 1_400, 1_700, 2_100, 2_500, 3_000, 4_000, 5_000, 6_000, 7_000, 8_000,
    9_000, 10_000, 12_000, 14_000, 17_000, 21_000, 25_000, 30_000, 100_000, 1_000_000,
];

fn get_counts() -> Vec<usize> {
    env::var("COUNTS")
        .map(|s| {
            s.split(',')
                .map(|n| str::parse::<usize>(n).unwrap())
                .collect()
        })
        .unwrap_or(COUNTS.to_owned())
}

fn get_config() -> Criterion {
    if env::var("CI").is_ok() {
        Criterion::default()
            .nresamples(5_000)
            .without_plots()
            .measurement_time(Duration::new(2, 0))
            .warm_up_time(Duration::new(1, 0))
    } else {
        Criterion::default()
    }
}

fn bench_counts(criterion: &mut Criterion) {
    fn naive(b: &mut Bencher, s: &usize) {
        let haystack = random_bytes(*s);
        b.iter(|| naive_count(&haystack, 10))
    }
    fn naive_32(b: &mut Bencher, s: &usize) {
        let haystack = random_bytes(*s);
        b.iter(|| naive_count_32(&haystack, 10))
    }
    fn hyper(b: &mut Bencher, s: &usize) {
        let haystack = random_bytes(*s);
        b.iter(|| count(&haystack, 10))
    }
    let counts = get_counts();
    let mut group = criterion.benchmark_group("counts");
    for count in counts {
        group.throughput(criterion::Throughput::Bytes(count as u64));
        group.bench_with_input(BenchmarkId::new("naive", count), &count, naive);
        group.bench_with_input(BenchmarkId::new("naive_32", count), &count, naive_32);
        group.bench_with_input(BenchmarkId::new("hyper", count), &count, hyper);
    }
}

fn bench_num_chars(criterion: &mut Criterion) {
    fn naive(b: &mut Bencher, s: &usize) {
        let haystack = random_bytes(*s);
        b.iter(|| naive_num_chars(&haystack))
    }
    fn hyper(b: &mut Bencher, s: &usize) {
        let haystack = random_bytes(*s);
        b.iter(|| num_chars(&haystack))
    }
    let counts = get_counts();
    let mut group = criterion.benchmark_group("num_chars");
    for count in counts {
        group.throughput(criterion::Throughput::Bytes(count as u64));
        group.bench_with_input(BenchmarkId::new("naive", count), &count, naive);
        group.bench_with_input(BenchmarkId::new("hyper", count), &count, hyper);
    }
}

criterion_group!(name = count_bench; config = get_config(); targets = bench_counts);
criterion_group!(name = num_chars_bench; config = get_config(); targets = bench_num_chars);
criterion_main!(count_bench, num_chars_bench);
