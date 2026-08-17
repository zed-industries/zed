use criterion::{black_box, criterion_group, criterion_main};

pub fn bench(c: &mut criterion::Criterion) {
    c.bench_function("Standard range", |b| {
        b.iter(|| {
            http_range_header::parse_range_header(black_box("bytes=0-15"))
                .unwrap()
                .validate(black_box(10_000))
        })
    });
    c.bench_function("Multipart range", |b| {
        b.iter(|| {
            http_range_header::parse_range_header(black_box("bytes=0-15, 20-30, 40-60"))
                .unwrap()
                .validate(black_box(10_000))
        })
    });
    c.bench_function("Suffix range", |b| {
        b.iter(|| {
            http_range_header::parse_range_header(black_box("bytes=-500"))
                .unwrap()
                .validate(black_box(10_000))
        })
    });
    c.bench_function("Open range", |b| {
        b.iter(|| {
            http_range_header::parse_range_header(black_box("bytes=0-"))
                .unwrap()
                .validate(black_box(10_000))
        })
    });
}
criterion_group!(benches, bench);
criterion_main!(benches);
