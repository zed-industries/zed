use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use edit_prediction_context::{CodeParts, IdentifierParts, OccurrenceSource};
use rand::prelude::*;
use rand::rngs::StdRng;
use util::RandomCharIter;

fn generate_random_text(mut rng: StdRng, text_len: usize) -> String {
    RandomCharIter::new(&mut rng).take(text_len).collect()
}

fn text_similarity_benchmark(c: &mut Criterion) {
    let rng = StdRng::seed_from_u64(42);
    let sizes = [4, 16, 32, 1024];

    let mut group = c.benchmark_group("hashed_identifier_parts");
    for size in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let text = generate_random_text(rng.clone(), *size);
            b.iter(|| IdentifierParts::occurrences_in_str(text.as_str()).collect::<Vec<_>>());
        });
    }
    group.finish();

    let mut group = c.benchmark_group("hashed_code_parts");
    for size in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let text = generate_random_text(rng.clone(), *size);
            b.iter(|| CodeParts::occurrences_in_str(text.as_str()).collect::<Vec<_>>());
        });
    }
    group.finish();
}

criterion_group!(benches, text_similarity_benchmark);
criterion_main!(benches);
