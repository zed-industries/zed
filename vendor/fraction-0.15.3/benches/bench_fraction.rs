extern crate criterion;
extern crate fraction;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fraction::generic;
use fraction::{GenericDecimal, GenericFraction};
use std::str::FromStr;

#[allow(clippy::missing_panics_doc)]
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Decimal u128/u16 init", |b| {
        b.iter(|| GenericDecimal::<u128, u16>::from(black_box(15978.649)));
    });

    c.bench_function("Decimal i64/u16 init", |b| {
        b.iter(|| {
            let a = GenericDecimal::<i64, u16>::from(black_box(15978.649));
            let b = GenericDecimal::<i64, u16>::from(black_box(-15978.649));

            (a, b)
        });
    });

    c.bench_function("Convert int like from str", |b| {
        b.iter(|| {
            let a = GenericFraction::<u8>::from_str(black_box("1"));
            let b = GenericFraction::<u8>::from_str(black_box("100"));

            (a, b)
        });
    });

    c.bench_function("Convert float like from str", |b| {
        b.iter(|| {
            let a = GenericFraction::<u8>::from_str(black_box("1.0"));
            let b = GenericFraction::<u8>::from_str(black_box("100.001"));

            (a, b)
        });
    });

    c.bench_function("Convert fraction like from str", |b| {
        b.iter(|| {
            let a = GenericFraction::<u8>::from_str(black_box("1/1"));
            let b = GenericFraction::<u8>::from_str(black_box("255/255"));
            (a, b)
        });
    });

    c.bench_function("generic::read_generic_integer / i8 to u8", |b| {
        b.iter(|| generic::read_generic_integer::<u8, i8>(black_box(14i8)).unwrap());
    });

    c.bench_function("generic::read_generic_integer / u8 to u8", |b| {
        b.iter(|| generic::read_generic_integer::<u8, u8>(black_box(14u8)).unwrap());
    });

    c.bench_function("From couple", |b| {
        b.iter(|| GenericFraction::<u8>::from(black_box((3u8, 4u8))));
    });

    #[cfg(feature = "with-approx")]
    {
        let num2 = GenericFraction::<u8>::from(2);
        let small_num = fraction::BigFraction::new(1_u8, u128::MAX) / u128::MAX;
        let big_num = fraction::BigFraction::new(u128::MAX, 1_u8) * u128::MAX;

        let mut bench_dp = |n: u32| {
            let mut group = c.benchmark_group(format!("Sqrt {n}dp raw"));

            group.bench_function("2", |b| {
                b.iter(|| num2.sqrt_raw(n));
            });

            group.bench_function("Small", |b| {
                b.iter(|| small_num.sqrt_raw(n));
            });

            group.bench_function("Big", |b| {
                b.iter(|| big_num.sqrt_raw(n));
            });

            group.finish();
        };

        bench_dp(10_000);
        bench_dp(1_000);
        bench_dp(100);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
