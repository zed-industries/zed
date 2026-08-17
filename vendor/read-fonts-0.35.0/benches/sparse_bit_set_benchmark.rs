mod bench_helper;

use bench_helper::random_set;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use read_fonts::collections::int_set::{sparse_bit_set::to_sparse_bit_set_with_bf, IntSet};

struct SparseSetTest {
    set_size: u32,
    density: u32,
    branch_factor: u32,
}

impl SparseSetTest {
    fn max_value(&self) -> u32 {
        self.density * self.set_size
    }
}

impl std::fmt::Display for SparseSetTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "bf{}_{}/{}",
            self.branch_factor, self.set_size, self.density
        )
    }
}

fn set_parameters() -> Vec<SparseSetTest> {
    let mut inputs: Vec<SparseSetTest> = Vec::<SparseSetTest>::new();
    for branch_factor in [2, 4, 8, 32] {
        for density in [2, 8] {
            for set_size in [1024, 4096, 32_768] {
                inputs.push(SparseSetTest {
                    set_size,
                    density,
                    branch_factor,
                })
            }
        }
    }
    inputs
}

pub fn decode_benchmark(c: &mut Criterion) {
    let inputs = set_parameters();

    for input in inputs {
        let set = random_set(input.set_size, input.max_value());
        let encoded = match input.branch_factor {
            2 => to_sparse_bit_set_with_bf::<2>(&set),
            4 => to_sparse_bit_set_with_bf::<4>(&set),
            8 => to_sparse_bit_set_with_bf::<8>(&set),
            32 => to_sparse_bit_set_with_bf::<32>(&set),
            _ => panic!("not implemented."),
        };
        c.bench_with_input(
            BenchmarkId::new("BM_SparseSetDecode", &input),
            &input,
            |b, _: &SparseSetTest| {
                b.iter(|| black_box(IntSet::<u32>::from_sparse_bit_set(&encoded)));
            },
        );
    }
}

criterion_group!(benches, decode_benchmark);
criterion_main!(benches);
