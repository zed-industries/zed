use std::ops::Range;

use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
use rand::prelude::*;
use rand::rngs::StdRng;
use rope::{Point, Rope};
use sum_tree::Bias;
use util::RandomCharIter;

fn generate_random_text(mut rng: StdRng, text_len: usize) -> String {
    RandomCharIter::new(&mut rng).take(text_len).collect()
}

fn generate_random_rope(rng: StdRng, text_len: usize) -> Rope {
    let text = generate_random_text(rng, text_len);
    let mut rope = Rope::new();
    rope.push(&text);
    rope
}

fn generate_random_rope_ranges(mut rng: StdRng, rope: &Rope) -> Vec<Range<usize>> {
    let range_max_len = 50;
    let num_ranges = rope.len() / range_max_len;

    let mut ranges = Vec::new();
    let mut start = 0;
    for _ in 0..num_ranges {
        let range_start = rope.clip_offset(
            rng.random_range(start..=(start + range_max_len)),
            sum_tree::Bias::Left,
        );
        let range_end = rope.clip_offset(
            rng.random_range(range_start..(range_start + range_max_len)),
            sum_tree::Bias::Right,
        );

        let range = range_start..range_end;
        if !range.is_empty() {
            ranges.push(range);
        }

        start = range_end + 1;
    }

    ranges
}

fn generate_random_rope_points(mut rng: StdRng, rope: &Rope) -> Vec<Point> {
    let num_points = rope.len() / 10;

    let mut points = Vec::new();
    for _ in 0..num_points {
        points.push(rope.offset_to_point(rng.random_range(0..rope.len())));
    }
    points
}

fn rope_benchmarks(c: &mut Criterion) {
    static SEED: u64 = 9999;
    static KB: usize = 1024;

    let rng = StdRng::seed_from_u64(SEED);
    let sizes = [4 * KB, 64 * KB];

    let mut group = c.benchmark_group("push");
    for size in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let text = generate_random_text(rng.clone(), *size);

            b.iter(|| {
                let mut rope = Rope::new();
                for _ in 0..10 {
                    rope.push(&text);
                }
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("append");
    for size in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut random_ropes = Vec::new();
            for _ in 0..5 {
                random_ropes.push(generate_random_rope(rng.clone(), *size));
            }

            b.iter(|| {
                let mut rope_b = Rope::new();
                for rope in &random_ropes {
                    rope_b.append(rope.clone())
                }
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("slice");
    for size in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rope = generate_random_rope(rng.clone(), *size);

            b.iter_batched(
                || generate_random_rope_ranges(rng.clone(), &rope),
                |ranges| {
                    for range in ranges.iter() {
                        rope.slice(range.clone());
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("bytes_in_range");
    for size in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rope = generate_random_rope(rng.clone(), *size);

            b.iter_batched(
                || generate_random_rope_ranges(rng.clone(), &rope),
                |ranges| {
                    for range in ranges.iter() {
                        let bytes = rope.bytes_in_range(range.clone());
                        assert!(bytes.into_iter().count() > 0);
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("chars");
    for size in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rope = generate_random_rope(rng.clone(), *size);

            b.iter_with_large_drop(|| {
                let chars = rope.chars().count();
                assert!(chars > 0);
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("clip_point");
    for size in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rope = generate_random_rope(rng.clone(), *size);

            b.iter_batched(
                || generate_random_rope_points(rng.clone(), &rope),
                |offsets| {
                    for offset in offsets.iter() {
                        black_box(rope.clip_point(*offset, Bias::Left));
                        black_box(rope.clip_point(*offset, Bias::Right));
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    let mut group = c.benchmark_group("point_to_offset");
    for size in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rope = generate_random_rope(rng.clone(), *size);

            b.iter_batched(
                || generate_random_rope_points(rng.clone(), &rope),
                |offsets| {
                    for offset in offsets.iter() {
                        black_box(rope.point_to_offset(*offset));
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, rope_benchmarks);
criterion_main!(benches);
