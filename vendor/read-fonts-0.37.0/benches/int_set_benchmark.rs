mod bench_helper;
use bench_helper::{random_set, random_u32_set};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use read_fonts::collections::{IntSet, U32Set};

struct SetTest {
    set_size: u32,
    density: u32,
}

impl SetTest {
    fn max_value(&self) -> u32 {
        self.density * self.set_size
    }
}

impl std::fmt::Display for SetTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.set_size, self.density)
    }
}

fn set_parameters() -> Vec<SetTest> {
    let mut inputs: Vec<SetTest> = Vec::<SetTest>::new();
    for density in [2, 8, 64, 512] {
        for set_size in [1024, 4096, 32_768, 65_536] {
            inputs.push(SetTest { set_size, density })
        }
    }
    inputs
}

pub fn insert_benchmark(c: &mut Criterion) {
    const NUM_INSERTS: u32 = 1000;
    let inputs = set_parameters();

    for input in inputs {
        c.bench_with_input(
            BenchmarkId::new("BM_SetInsert_1000", &input),
            &input,
            |b, p: &SetTest| {
                let set = random_set(p.set_size, p.max_value());
                b.iter_batched(
                    || set.clone(),
                    |mut s| {
                        for i in 0..NUM_INSERTS {
                            let v: u32 = i.wrapping_mul(2_654_435_761) % p.max_value();
                            s.insert(v);
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

pub fn ordered_extend_benchmark(c: &mut Criterion) {
    let num_inserts = 1000u32;
    let inputs = set_parameters();

    for input in inputs {
        c.bench_with_input(
            BenchmarkId::new("BM_SetOrderedInsert_1000", &input),
            &input,
            |b, p: &SetTest| {
                let set = random_set(p.set_size, p.max_value());
                b.iter_batched(
                    || set.clone(),
                    |mut s| {
                        s.extend(0..num_inserts);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

pub fn lookup_random_benchmark(c: &mut Criterion) {
    let inputs = set_parameters();

    for input in inputs {
        let set = random_set(input.set_size, input.max_value());
        let mut needle = input.max_value() / 2;
        c.bench_with_input(
            BenchmarkId::new("BM_SetLookup/random", &input),
            &set,
            |b, s: &IntSet<u32>| {
                b.iter(|| {
                    needle += 12345;
                    s.contains(needle % input.max_value())
                })
            },
        );
    }
}

pub fn lookup_ordered_benchmark(c: &mut Criterion) {
    let inputs = set_parameters();

    for input in inputs {
        let set = random_set(input.set_size, input.max_value());
        let mut needle = input.max_value() / 2;
        c.bench_with_input(
            BenchmarkId::new("BM_SetLookup/ordered", &input),
            &set,
            |b, s: &IntSet<u32>| {
                b.iter(|| {
                    needle += 3;
                    s.contains(needle % input.max_value())
                })
            },
        );
    }
}

pub fn lookup_ordered_u32_benchmark(c: &mut Criterion) {
    let inputs = set_parameters();

    for input in inputs {
        let set = random_u32_set(input.set_size, input.max_value());
        let mut needle = input.max_value() / 2;
        c.bench_with_input(
            BenchmarkId::new("BM_SetLookup/ordered_u32", &input),
            &set,
            |b, s: &U32Set| {
                b.iter(|| {
                    needle += 3;
                    s.contains(needle % input.max_value())
                })
            },
        );
    }
}

pub fn iteration_benchmark(c: &mut Criterion) {
    let inputs = set_parameters();

    for input in inputs {
        let set = random_set(input.set_size, input.max_value());
        c.bench_with_input(
            BenchmarkId::new("BM_SetIteration", &input),
            &set,
            |b, s: &IntSet<u32>| {
                b.iter(|| {
                    for v in s.iter() {
                        black_box(v);
                    }
                })
            },
        );
    }
}

criterion_group!(
    benches,
    insert_benchmark,
    ordered_extend_benchmark,
    lookup_random_benchmark,
    lookup_ordered_benchmark,
    lookup_ordered_u32_benchmark,
    iteration_benchmark,
);
criterion_main!(benches);
