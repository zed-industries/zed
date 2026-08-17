use convert_case::{Case, Casing};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from-to", |b| {
        b.iter(|| {
            let words = vec![
                "iGetUp",
                "and-nothing-gets-me-down",
                "YOUGotItTough",
                "i'veSeen_the_toughest_around",
            ];
            for word in words {
                for to_case in Case::all_cases() {
                    for from_case in Case::all_cases() {
                        word.from_case(*from_case).to_case(*to_case);
                    }
                    word.to_case(*to_case);
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
