//! See how `rust-decimal` performs compared to native floating numbers.

use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, BenchmarkId, Criterion,
};
use rust_decimal::Decimal;

const DECIMAL_2_01: Decimal = Decimal::from_parts(201, 0, 0, false, 2);
const DECIMAL_13_7: Decimal = Decimal::from_parts(137, 0, 0, false, 1);
const F64_2_01: f64 = 2.01;
const F64_13_7: f64 = 13.7;

macro_rules! add_benchmark_group {
    ($criterion:expr, $f:ident, $op:tt) => {
        fn $f<M, const N: usize>(group: &mut BenchmarkGroup<'_, M>)
        where
            M: Measurement,
        {
            group.bench_with_input(BenchmarkId::new("f64 (diff)", N), &N, |ben, _| {
                ben.iter(|| black_box(F64_2_01 $op F64_13_7))
            });

            group.bench_with_input(BenchmarkId::new("f64 (equal)", N), &N, |ben, _| {
                ben.iter(|| black_box(F64_2_01 $op F64_2_01))
            });

            group.bench_with_input(BenchmarkId::new("rust-decimal (diff)", N), &N, |ben, _| {
                ben.iter(|| black_box(DECIMAL_2_01 $op DECIMAL_13_7))
            });

            group.bench_with_input(BenchmarkId::new("rust-decimal (equal)", N), &N, |ben, _| {
                ben.iter(|| black_box(DECIMAL_2_01 $op DECIMAL_2_01))
            });
        }

        let mut group = $criterion.benchmark_group(stringify!($f));
        $f::<_, 100>(&mut group);
        group.finish();
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    add_benchmark_group!(c, addition, +);
    add_benchmark_group!(c, division, /);
    add_benchmark_group!(c, multiplication, *);
    add_benchmark_group!(c, subtraction, -);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
