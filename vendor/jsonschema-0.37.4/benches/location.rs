#[cfg(not(target_arch = "wasm32"))]
mod bench {
    pub(crate) use criterion::{criterion_group, BenchmarkId, Criterion};
    pub(crate) use jsonschema::paths::{Location, LocationSegment};
    pub(crate) use std::hint::black_box;

    pub(crate) fn benchmark_into_iterator(c: &mut Criterion) {
        let empty = Location::new();
        let small = vec!["a", "b", "c"]
            .into_iter()
            .map(LocationSegment::from)
            .chain((0..3).map(LocationSegment::from))
            .collect::<Location>();
        let large = (0..500)
            .map(|_| LocationSegment::from("abc"))
            .chain((0..500).map(LocationSegment::from))
            .collect::<Location>();

        for (parameter, input) in [("empty", empty), ("small", small), ("large", large)] {
            c.bench_with_input(
                BenchmarkId::new("IntoIterator", parameter),
                &input,
                |b, i| b.iter_with_large_drop(|| black_box(i).into_iter().collect::<Vec<_>>()),
            );
        }
    }

    pub(crate) fn benchmark_from_iterator(c: &mut Criterion) {
        let empty = vec![];
        let small = vec![
            LocationSegment::from("a"),
            LocationSegment::from("b"),
            LocationSegment::from("c"),
        ];
        let large = (0..1000)
            .map(|_| LocationSegment::from("abc"))
            .collect::<Vec<_>>();

        for (parameter, input) in [("empty", empty), ("small", small), ("large", large)] {
            c.bench_with_input(
                BenchmarkId::new("FromIterator", parameter),
                &input,
                |b, i| {
                    b.iter_batched(
                        || i.clone().into_iter(),
                        |i| black_box(i).collect::<Location>(),
                        criterion::BatchSize::SmallInput,
                    );
                },
            );
        }
    }

    criterion_group!(benches, benchmark_into_iterator, benchmark_from_iterator);
}

#[cfg(not(target_arch = "wasm32"))]
criterion::criterion_main!(bench::benches);

#[cfg(target_arch = "wasm32")]
fn main() {}
