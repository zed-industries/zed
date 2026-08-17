#![feature(test)]

extern crate test;

use hdrhistogram::*;
use rand::SeedableRng;
use test::Bencher;

use self::rand_varint::*;

#[path = "../src/serialization/rand_varint.rs"]
mod rand_varint;

#[bench]
fn record_precalc_random_values_with_1_count_u64(b: &mut Bencher) {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut indices = Vec::<u64>::new();
    let mut rng = rand::rngs::SmallRng::from_entropy();

    // same value approach as record_precalc_random_values_with_max_count_u64 so that they are
    // comparable

    for v in RandomVarintEncodedLengthIter::new(&mut rng).take(1_000_000) {
        indices.push(v);
    }

    b.iter(|| {
        for i in indices.iter() {
            // u64 counts, won't overflow
            h.record(*i).unwrap()
        }
    })
}

#[bench]
fn record_precalc_random_values_with_max_count_u64(b: &mut Bencher) {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut indices = Vec::<u64>::new();
    let mut rng = rand::rngs::SmallRng::from_entropy();

    // store values in an array and re-use so we can be sure to hit the overflow case

    for v in RandomVarintEncodedLengthIter::new(&mut rng).take(1_000_000) {
        indices.push(v);
        h.record_n(v, u64::max_value()).unwrap();
    }

    b.iter(|| {
        for i in indices.iter() {
            // all values are already at u64
            h.record(*i).unwrap()
        }
    })
}

#[bench]
fn record_correct_precalc_random_values_with_1_count_u64(b: &mut Bencher) {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut indices = Vec::<u64>::new();
    let mut rng = rand::rngs::SmallRng::from_entropy();

    for v in RandomVarintEncodedLengthIter::new(&mut rng).take(10_000) {
        indices.push(v);
    }

    b.iter(|| {
        for i in indices.iter() {
            // using a fairly big number so it doesn't have to fill in TOO many intervening counts
            h.record_correct(*i, 1 << 54).unwrap()
        }
    })
}

#[bench]
fn record_random_values_with_1_count_u64(b: &mut Bencher) {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut rng = rand::rngs::SmallRng::from_entropy();

    // This should be *slower* than the benchmarks above where we pre-calculate the values
    // outside of the hot loop. If it isn't, then those measurements are likely spurious.

    b.iter(|| {
        for v in RandomVarintEncodedLengthIter::new(&mut rng).take(1_000_000) {
            h.record(v).unwrap()
        }
    })
}

#[bench]
fn add_precalc_random_value_1_count_same_dimensions_u64(b: &mut Bencher) {
    do_add_benchmark(b, 1, || {
        Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap()
    })
}

#[bench]
fn add_precalc_random_value_max_count_same_dimensions_u64(b: &mut Bencher) {
    do_add_benchmark(b, u64::max_value(), || {
        Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap()
    })
}

#[bench]
fn add_precalc_random_value_1_count_different_precision_u64(b: &mut Bencher) {
    do_add_benchmark(b, 1, || {
        Histogram::<u64>::new_with_bounds(1, u64::max_value(), 2).unwrap()
    })
}

#[bench]
fn add_precalc_random_value_max_count_different_precision_u64(b: &mut Bencher) {
    do_add_benchmark(b, u64::max_value(), || {
        Histogram::<u64>::new_with_bounds(1, u64::max_value(), 2).unwrap()
    })
}

#[bench]
fn subtract_precalc_random_value_1_count_same_dimensions_u64(b: &mut Bencher) {
    do_subtract_benchmark(b, 1, || {
        Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap()
    })
}

// can't do subtraction with max count because it will error after the first iteration because
// subtrahend count exceeds minuend. Similarly, when subtracting a different precision, the same
// issue happens because the smallest equivalent value in the lower precision can map to a different
// bucket in higher precision so we cannot easily pre-populate.

fn do_subtract_benchmark<F: Fn() -> Histogram<u64>>(
    b: &mut Bencher,
    count_at_each_addend_value: u64,
    addend_factory: F,
) {
    let mut accum = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut subtrahends = Vec::new();
    let mut rng = rand::rngs::SmallRng::from_entropy();

    for _ in 0..1000 {
        let mut h = addend_factory();

        for v in RandomVarintEncodedLengthIter::new(&mut rng).take(1_000) {
            h.record_n(v, count_at_each_addend_value).unwrap();
            // ensure there's a count to subtract from
            accum.record_n(v, count_at_each_addend_value).unwrap();
        }

        subtrahends.push(h);
    }

    b.iter(|| {
        let mut clone = accum.clone();
        for h in subtrahends.iter() {
            clone.subtract(h).unwrap();
        }
    })
}

fn do_add_benchmark<F: Fn() -> Histogram<u64>>(
    b: &mut Bencher,
    count_at_each_addend_value: u64,
    addend_factory: F,
) {
    let mut accum = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut addends = Vec::new();
    let mut rng = rand::rngs::SmallRng::from_entropy();

    for _ in 0..1000 {
        let mut h = addend_factory();

        for v in RandomVarintEncodedLengthIter::new(&mut rng).take(1_000) {
            h.record_n(v, count_at_each_addend_value).unwrap();
        }

        addends.push(h);
    }

    b.iter(|| {
        for h in addends.iter() {
            accum.add(h).unwrap();
        }
    })
}
