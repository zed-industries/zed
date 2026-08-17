use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn size_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("imagesize");

    let mut paths = Vec::new();
    for file in walkdir::WalkDir::new("tests/images")
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            paths.push(std::fs::canonicalize(file.path()).unwrap());
        }
    }

    group.bench_with_input(
        BenchmarkId::from_parameter(paths.len()),
        &paths,
        |b, paths| {
            b.iter(|| {
                for path in paths {
                    let _ = imagesize::size(black_box(path));
                }
            })
        },
    );

    group.finish();
}

criterion_group!(benches, size_benchmarks);
criterion_main!(benches);
