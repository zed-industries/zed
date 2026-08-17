use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use v_frame::frame::Frame;
use v_frame::pixel::{CastFromPrimitive, ChromaSampling};
use v_frame::plane::Plane;

fn frame(c: &mut Criterion) {
    c.bench_function("frame new_with_padding padding=0", |b| {
        b.iter(|| {
            Frame::<u8>::new_with_padding(
                black_box(640),
                black_box(480),
                black_box(ChromaSampling::Cs420),
                black_box(0),
            )
        })
    });

    c.bench_function("frame new_with_padding padding!=0", |b| {
        b.iter(|| {
            Frame::<u8>::new_with_padding(
                black_box(640),
                black_box(480),
                black_box(ChromaSampling::Cs420),
                black_box(24),
            )
        })
    });
}

fn plane(c: &mut Criterion) {
    c.bench_function("plane new padding=0", |b| {
        b.iter(|| {
            Plane::<u8>::new(
                black_box(640),
                black_box(480),
                black_box(1),
                black_box(1),
                black_box(0),
                black_box(0),
            )
        })
    });

    c.bench_function("plane new padding!=0", |b| {
        b.iter(|| {
            Plane::<u8>::new(
                black_box(640),
                black_box(480),
                black_box(1),
                black_box(1),
                black_box(24),
                black_box(24),
            )
        })
    });

    let p8b = Plane::<u8>::new(
        black_box(640),
        black_box(480),
        black_box(1),
        black_box(1),
        black_box(0),
        black_box(0),
    );

    // We need to use PerIteration because we modify the Plane every iteration
    let batch_size = criterion::BatchSize::PerIteration;

    c.bench_function("plane clone", |b| b.iter(|| p8b.clone()));

    c.bench_function("plane pad", |b| {
        b.iter_batched_ref(
            || p8b.clone(),
            |p| p.pad(black_box(680), black_box(520)),
            batch_size,
        )
    });

    let data_8b: Vec<u8> = vec![2; 640 * 480];
    c.bench_function("plane copy_from_raw_u8 8-bit", |b| {
        b.iter_batched_ref(
            || p8b.clone(),
            |p| p.copy_from_raw_u8(black_box(&data_8b), black_box(640), black_box(1)),
            batch_size,
        )
    });

    let p10b = Plane::<u16>::new(
        black_box(640),
        black_box(480),
        black_box(1),
        black_box(1),
        black_box(0),
        black_box(0),
    );
    let data_10b: Vec<u8> = vec![2; 640 * 480 * 2];
    c.bench_function("plane copy_from_raw_u8 10-bit", |b| {
        b.iter_batched_ref(
            || p10b.clone(),
            |p| p.copy_from_raw_u8(black_box(&data_10b), black_box(640), black_box(2)),
            batch_size,
        )
    });

    c.bench_function("plane downsampled", |b| {
        b.iter(|| p8b.downsampled(black_box(320), black_box(240)))
    });

    c.bench_function("plane downscale", |b| b.iter(|| p8b.downscale::<2>()));

    // This may seem silly to benchmark, but there is some math in the iterator
    // that has been known to hinder compiler optimizations
    c.bench_function("plane rows_iter", |b| {
        b.iter(|| {
            p8b.rows_iter()
                .map(|r| r.iter().map(|v| u8::cast_from(*v) as u64).sum::<u64>())
                .collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, frame, plane);
criterion_main!(benches);
