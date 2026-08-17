//! Benchmarks for arithmetic operation

extern crate criterion;
extern crate bigdecimal;
extern crate oorandom;
extern crate lazy_static;

use lazy_static::lazy_static;

use std::fs::File;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bigdecimal::BigDecimal;

#[macro_use]
mod common;
use common::*;

criterion_main!(
    arithmetic,
    arithmetic_bulk,
);

criterion_group!(
    name = arithmetic_bulk;
    config = Criterion::default()
                       .measurement_time(Duration::from_secs(7))
                       .sample_size(700);
    targets =
        criterion_benchmark,
        datafile_1f633481a742923ab65855c90157bbf7::addition_bulk,
);

criterion_group!(
    name = arithmetic;
    config = Criterion::default()
                       .sample_size(300);
    targets =
        datafile_9a08ddaa6ce6693cdd7b8a524e088bd0::arithmetic,
        datafile_1f633481a742923ab65855c90157bbf7::addition,
);


fn make_random_pairs(decs: &[BigDecimal], seed: u64) -> Vec<(&BigDecimal, &BigDecimal)> {
    let mut cartesian_pairs = decs
                            .iter()
                            .enumerate()
                            .flat_map(|(i, x)| {
                                decs.iter().skip(i+1).map(move |y| (x, y))
                            }).collect::<Vec<(&BigDecimal, &BigDecimal)>>();

    // random number generator from random seed
    let mut rng = oorandom::Rand32::new(seed);

    for i in (1..cartesian_pairs.len()).rev() {
        let j = rng.rand_u32() as usize % i;
        cartesian_pairs.swap(i, j);
    }

    cartesian_pairs
}

fn bench_addition_pairwise(name: &str, c: &mut Criterion, decs: &[BigDecimal]) {
    let x_iter = decs.iter().step_by(2);
    let y_iter = decs.iter().skip(1).step_by(2);
    let pair_iter = std::iter::zip(x_iter, y_iter);

    c.bench_function(
        name,
        |b| b.iter_batched(
            || {
                pair_iter.clone()
            },
            |pairs| {
                for (x, y) in pairs {
                    black_box(x + y);
                }
            },
            criterion::BatchSize::SmallInput));
}


fn bench_addition_pairs(
    name: &str,
    c: &mut Criterion,
    dec_pairs: &[(&BigDecimal, &BigDecimal)],
) {
    let pair_iter = dec_pairs.iter().cycle();

    let mut iter_count = 0;
    c.bench_function(
        name,
        |b| b.iter_batched(
            || {
                iter_count += 1;
                pair_iter.clone().skip(iter_count).next().unwrap()
            },
            |(x, y)| {
                black_box(*x + *y);
            },
            criterion::BatchSize::SmallInput));
}


fn bench_addition_pairs_bulk(
    name: &str,
    c: &mut Criterion,
    chunk_size: usize,
    dec_pairs: &[(&BigDecimal, &BigDecimal)],
) {
    let pair_iter = dec_pairs.iter();

    let mut iter_count = 0;
    c.bench_function(
        name,
        |b| b.iter_batched(
            || {
                let skip_by = chunk_size * iter_count;
                iter_count += 1;
                pair_iter.clone().skip(skip_by).take(chunk_size)
            },
            |pairs| {
                for (x, y) in pairs {
                    black_box(*x + *y);
                }
            },
            criterion::BatchSize::SmallInput));
}

fn bench_multiplication_pairs(
    name: &str,
    c: &mut Criterion,
    dec_pairs: &[(&BigDecimal, &BigDecimal)],
) {
    let pair_iter = dec_pairs.iter().cycle();

    let mut iter_count = 0;
    c.bench_function(
        name,
        |b| b.iter_batched(
            || {
                iter_count += 1;
                pair_iter.clone().skip(iter_count).next().unwrap()
            },
            |(x, y)| {
                black_box(*x * *y);
            },
            criterion::BatchSize::SmallInput));
}

fn bench_inverse(
    name: &str,
    c: &mut Criterion,
    decs: &[BigDecimal],
) {
    let mut idx = 0;
    c.bench_function(
        name,
        |b| b.iter_batched(
            || {
                idx += 1;
                if idx == decs.len() {
                    idx = 0;
                }
                &decs[idx]
            },
            |x| {
                black_box(x.inverse());
            },
            criterion::BatchSize::SmallInput));
}


mod datafile_1f633481a742923ab65855c90157bbf7 {
    use super::*;


    fn get_bigdecimals() -> Vec<BigDecimal> {
        let file = open_benchmark_data_file!("random-bigdecimals-1f633481a742923ab65855c90157bbf7.txt");
        super::read_bigdecimal_file(file)
    }

    pub fn addition(c: &mut Criterion) {
        let decs = get_bigdecimals();
        let cartesian_pairs = make_random_pairs(&decs, 7238269155957952517_u64);

        bench_addition_pairwise(
            "addition-pairwise-1f633481a742923ab65855c90157bbf7", c, &decs
        );

        bench_addition_pairs(
            "addition-random-1f633481a742923ab65855c90157bbf7", c, &cartesian_pairs
        );
    }

    pub fn addition_bulk(c: &mut Criterion) {
        let decs = get_bigdecimals();
        let cartesian_pairs = make_random_pairs(&decs, 7238269155957952517_u64);
        bench_addition_pairs_bulk(
            "addition-random-1f633481a742923ab65855c90157bbf7-100", c, 100, &cartesian_pairs
        );
    }
}

mod datafile_9a08ddaa6ce6693cdd7b8a524e088bd0 {
    use super::*;

    const SRC: &'static str  = include_benchmark_data_file!("random-bigdecimals-9a08ddaa6ce6693cdd7b8a524e088bd0.txt");

    lazy_static! {
        static ref BIG_DECIMALS: Vec<BigDecimal> = super::collect_bigdecimals(SRC);
    }

    fn get_bigdecimals<'a>() -> &'a [BigDecimal] {
        BIG_DECIMALS.as_slice()
    }

    pub fn arithmetic(c: &mut Criterion) {
        let decs = get_bigdecimals();
        let cartesian_pairs = make_random_pairs(&decs, 7238269155957952517_u64);

        bench_addition_pairwise(
            "addition-pairwise-9a08ddaa6ce6693cdd7b8a524e088bd0", c, &decs
        );

        bench_addition_pairs(
            "addition-random-9a08ddaa6ce6693cdd7b8a524e088bd0", c, &cartesian_pairs
        );

        bench_multiplication_pairs(
            "multiplication-random-9a08ddaa6ce6693cdd7b8a524e088bd0", c, &cartesian_pairs
        );

        bench_inverse(
            "inverse-9a08ddaa6ce6693cdd7b8a524e088bd0", c, &decs
        );
    }
}


pub fn criterion_benchmark(c: &mut Criterion) {
    let src_a = include_benchmark_data_file!("random-bigdecimals-a329e61834832d89593b29f12510bdc8.txt");
    let src_b = include_benchmark_data_file!("random-bigdecimals-4be58192272b15fc67573b39910837d0.txt");

    let decs_a = collect_bigdecimals(src_a);
    let decs_b = collect_bigdecimals(src_b);
    assert_ne!(decs_a.len(), 0);
    assert_ne!(decs_b.len(), 0);

    let mut decimal_values = Vec::with_capacity(decs_a.len() + decs_b.len());
    for dec in decs_a.iter().chain(decs_b.iter()) {
        decimal_values.push(dec)
    }

    let mut decimal_pairs = Vec::with_capacity(decs_a.len() * decs_b.len());
    for a in decs_a.iter() {
        for b in decs_b.iter() {
            decimal_pairs.push((a, b));
        }
    }

    let mut random_decimal = RandomIterator::new(&decimal_values);
    // let mut random_decimal = decimal_values.iter().cycle();
    let mut random_pairs = RandomIterator::new(&decimal_pairs);
    // let mut random_pairs = decimal_pairs.iter().cycle().map(|d| *d);

    c.bench_function(
        "addition",
        |b| b.iter_batched(
            || {
                random_pairs.next()
            },
            |(a, b)| {
                black_box(a + b);
            },
            criterion::BatchSize::SmallInput));

    c.bench_function(
        "subtraction",
        |b| b.iter_batched(
            || {
                random_pairs.next()
            },
            |(a, b)| {
                black_box(a - b);
            },
            criterion::BatchSize::SmallInput));

    c.bench_function(
        "multiplication",
        |b| b.iter_batched(
            || {
                random_pairs.next()
            },
            |(a, b)| {
                black_box(a * b);
            },
            criterion::BatchSize::SmallInput));

    c.bench_function(
        "division",
        |b| b.iter_batched(
            || {
                random_pairs.next()
            },
            |(a, b)| {
                black_box(a / b);
            },
            criterion::BatchSize::SmallInput));

    c.bench_function(
        "square",
        |b| b.iter_batched(
            || {
                random_decimal.next()
            },
            |a| {
                black_box(a.square());
            },
            criterion::BatchSize::SmallInput));

    c.bench_function(
        "cube",
        |b| b.iter_batched(
            || {
                random_decimal.next()
            },
            |a| {
                black_box(a.cube());
            },
            criterion::BatchSize::SmallInput));

    let mut sqrt_group = c.benchmark_group("sqrt");
    for prec in [10, 100, 1000, 10000] {
        sqrt_group.bench_function(
            format!("sqrt_prec_{prec}"),
            |b| b.iter_batched(
                || {
                    (bigdecimal::Context::default().with_precision(std::num::NonZeroU64::new(prec).unwrap()), random_decimal.next())
                },
                |(ctx, a)| {
                    black_box(a.sqrt_with_context(&ctx));
                },
                criterion::BatchSize::SmallInput));
    }
}
