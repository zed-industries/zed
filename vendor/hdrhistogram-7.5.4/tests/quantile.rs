// rug doesn't support Windows
#![cfg(unix)]

use hdrhistogram::{Counter, Histogram};

use ieee754::Ieee754;
use rand::Rng;
use rug::{Integer, Rational};

#[test]
fn value_at_quantile_internal_count_exceeds_bucket_type() {
    let mut h: Histogram<u8> = Histogram::new(3).unwrap();

    for _ in 0..200 {
        h.record(100).unwrap();
    }

    for _ in 0..200 {
        h.record(100_000).unwrap();
    }

    // we won't get back the original input because of bucketing
    assert_eq!(h.highest_equivalent(100_000), h.value_at_quantile(1.0));
}

#[test]
fn value_at_quantile_2_values() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record(1).unwrap();
    h.record(2).unwrap();

    assert_eq!(1, h.value_at_quantile(0.25));
    assert_eq!(1, h.value_at_quantile(0.5));

    let almost_half = 0.5000000000000001;
    let next = 0.5000000000000002;
    // one ulp apart
    assert_eq!(almost_half, 0.5_f64.next());
    assert_eq!(next, almost_half.next());

    assert_eq!(2, h.value_at_quantile(almost_half));
    assert_eq!(2, h.value_at_quantile(next));
}

#[test]
fn value_at_quantile_5_values() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record(1).unwrap();
    h.record(2).unwrap();
    h.record(2).unwrap();
    h.record(2).unwrap();
    h.record(2).unwrap();

    assert_eq!(2, h.value_at_quantile(0.25));
    assert_eq!(2, h.value_at_quantile(0.3));
}

#[test]
fn value_at_quantile_20k() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    for i in 1..20_001 {
        h.record(i).unwrap();
    }

    assert_eq!(20_000, h.len());

    assert!(h.equivalent(19961, h.value_at_quantile(0.99805)));
}

#[test]
fn value_at_quantile_large_numbers() {
    let mut h = Histogram::<u64>::new_with_bounds(20_000_000, 100_000_000, 5).unwrap();
    h.record(100_000_000).unwrap();
    h.record(20_000_000).unwrap();
    h.record(30_000_000).unwrap();

    assert!(h.equivalent(20_000_000, h.value_at_quantile(0.5)));
    assert!(h.equivalent(30_000_000, h.value_at_quantile(0.5)));
    assert!(h.equivalent(100_000_000, h.value_at_quantile(0.8333)));
    assert!(h.equivalent(100_000_000, h.value_at_quantile(0.8334)));
    assert!(h.equivalent(100_000_000, h.value_at_quantile(0.99)));
}

#[test]
fn value_at_quantile_matches_quantile_iter_sequence_values() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    let lengths = vec![1, 5, 10, 50, 100, 500, 1_000, 5_000, 10_000];
    let mut errors: u64 = 0;

    for length in lengths {
        h.reset();

        for i in 1..(length + 1) {
            h.record(i).unwrap();
        }

        assert_eq!(length, h.len());

        let iter = h.iter_quantiles(100);

        for iter_val in iter {
            let calculated_value = h.value_at_quantile(iter_val.quantile());
            let v = iter_val.value_iterated_to();

            // Quantile iteration has problematic floating-point calculations. Calculating the
            // quantile involves something like `index / total_count`, and that's then multiplied
            // by `total_count` again to get the value at the quantile. This tends to produce
            // artifacts, so this test will frequently fail if you expect the actual value to
            // match the calculated value. Instead, we allow it to be one bucket high or low.

            if calculated_value != v
                && calculated_value != prev_value_nonzero_count(&h, v)
                && calculated_value != next_value_nonzero_count(&h, v)
            {
                let q_count_rational = calculate_quantile_count(iter_val.quantile(), length);

                println!(
                    "len {} iter quantile {} q * count fp {} q count rational {} \
                     iter val {} -> {} calc val {} -> {}",
                    length,
                    iter_val.quantile(),
                    iter_val.quantile() * length as f64,
                    q_count_rational,
                    v,
                    h.highest_equivalent(v),
                    calculated_value,
                    h.highest_equivalent(calculated_value)
                );
                errors += 1;
            }
        }
    }

    assert_eq!(0, errors);
}

#[test]
fn value_at_quantile_matches_quantile_iter_random_values() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    let lengths = vec![1, 5, 10, 50, 100, 500, 1_000, 5_000, 10_000];

    let mut rng = rand::thread_rng();
    let mut errors: u64 = 0;

    for length in lengths {
        h.reset();

        for v in RandomMaxIter::new(&mut rng).take(length) {
            h.record(v).unwrap();
        }

        assert_eq!(length as u64, h.len());

        let iter = h.iter_quantiles(100);

        for iter_val in iter {
            let calculated_value = h.value_at_quantile(iter_val.quantile());
            let v = iter_val.value_iterated_to();

            // Quantile iteration has problematic floating-point calculations. Calculating the
            // quantile involves something like `index / total_count`, and that's then multiplied
            // by `total_count` again to get the value at the quantile. This tends to produce
            // artifacts, so this test will frequently fail if you expect the actual value to
            // match the calculated value. Instead, we allow it to be one bucket high or low.

            if calculated_value != v
                && calculated_value != prev_value_nonzero_count(&h, v)
                && calculated_value != next_value_nonzero_count(&h, v)
            {
                let q_count_rational = calculate_quantile_count(iter_val.quantile(), length as u64);

                println!(
                    "len {} iter quantile {} q * count fp {} q count rational {} \
                     iter val {} -> {} calc val {} -> {}",
                    length,
                    iter_val.quantile(),
                    iter_val.quantile() * length as f64,
                    q_count_rational,
                    v,
                    h.highest_equivalent(v),
                    calculated_value,
                    h.highest_equivalent(calculated_value)
                );
                errors += 1;
            }
        }
    }

    assert_eq!(0, errors);
}

#[test]
fn value_at_quantile_matches_quantile_at_each_value_sequence_values() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    let lengths = vec![1, 5, 10, 50, 100, 500, 1_000, 5_000, 10_000];
    let mut errors: u64 = 0;

    for length in lengths {
        h.reset();

        for i in 1..(length + 1) {
            h.record(i).unwrap();
        }

        assert_eq!(length, h.len());

        for v in 1..(length + 1) {
            let quantile = Rational::from((v as u64, length as u64)).to_f64();
            let calculated_value = h.value_at_quantile(quantile);
            if !h.equivalent(v, calculated_value) {
                println!(
                    "len {} value {} quantile {} q * count fp {} actual {} -> {} calc {} -> {}",
                    length,
                    v,
                    quantile,
                    quantile * length as f64,
                    v,
                    h.highest_equivalent(v),
                    calculated_value,
                    h.highest_equivalent(calculated_value)
                );
                errors += 1;
            }
        }
    }

    assert_eq!(0, errors);
}

#[test]
fn value_at_quantile_matches_quantile_at_each_value_random_values() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut values = Vec::new();

    let lengths = vec![1, 5, 10, 50, 100, 500, 1_000, 5_000, 10_000];

    let mut rng = rand::thread_rng();

    let mut errors: u64 = 0;

    for length in lengths {
        h.reset();
        values.clear();

        for v in RandomMaxIter::new(&mut rng).take(length) {
            h.record(v).unwrap();
            values.push(v);
        }

        values.sort();

        assert_eq!(length as u64, h.len());

        for (index, &v) in values.iter().enumerate() {
            let quantile = Rational::from((index as u64 + 1, length as u64)).to_f64();
            let calculated_value = h.value_at_quantile(quantile);
            if !h.equivalent(v, calculated_value) {
                errors += 1;
                println!(
                    "len {} index {} quantile {} q * count fp {} actual {} -> {} calc {} -> {}",
                    length,
                    index,
                    quantile,
                    quantile * length as f64,
                    v,
                    h.highest_equivalent(v),
                    calculated_value,
                    h.highest_equivalent(calculated_value)
                );
            }
        }
    }

    assert_eq!(0, errors);
}

#[test]
fn value_at_quantile_matches_random_quantile_random_values() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut values = Vec::new();

    let lengths = vec![1, 5, 10, 50, 100, 500, 1_000, 5_000, 10_000];

    let mut rng = rand::thread_rng();

    let mut errors: u64 = 0;

    for length in lengths {
        h.reset();
        values.clear();

        for v in RandomMaxIter::new(&mut rng).take(length) {
            h.record(v).unwrap();
            values.push(v);
        }

        values.sort();

        assert_eq!(length as u64, h.len());

        for _ in 0..1_000 {
            let quantile = rng.gen_range(0_f64..=1_f64);
            let index_at_quantile = Integer::from(
                (Rational::from_f64(quantile).unwrap() * Rational::from(length as u64)).trunc_ref(),
            )
            .to_u64()
            .unwrap() as usize;
            let calculated_value = h.value_at_quantile(quantile);
            let v = values[index_at_quantile];
            if !h.equivalent(v, calculated_value) {
                errors += 1;
                println!(
                    "len {} index {} quantile {} q * count fp {} actual {} -> {} calc {} -> {}",
                    length,
                    index_at_quantile,
                    quantile,
                    quantile * length as f64,
                    v,
                    h.highest_equivalent(v),
                    calculated_value,
                    h.highest_equivalent(calculated_value)
                );
            }
        }
    }

    assert_eq!(0, errors);
}

/// An iterator of random `u64`s where the maximum value for each random number generation is picked
/// from a uniform distribution of the 64 possible bit lengths for a `u64`.
///
/// This helps create somewhat more realistic distributions of numbers. A simple random u64 is very
/// likely to be a HUGE number; this helps scatter some numbers down in the smaller end.
struct RandomMaxIter<'a, R: Rng + 'a> {
    rng: &'a mut R,
}

impl<'a, R: Rng + 'a> RandomMaxIter<'a, R> {
    fn new(rng: &'a mut R) -> RandomMaxIter<R> {
        RandomMaxIter { rng }
    }
}

impl<'a, R: Rng + 'a> Iterator for RandomMaxIter<'a, R> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let bit_length = self.rng.gen_range(0..=64);

        return Some(match bit_length {
            0 => 0,
            64 => u64::max_value(),
            x => self.rng.gen_range(0..1 << x),
        });
    }
}

/// Calculate the count at a quantile with arbitrary precision arithmetic
fn calculate_quantile_count(quantile: f64, count: u64) -> u64 {
    let product = Rational::from_f64(quantile).unwrap() * Rational::from(count);
    Integer::from(product.ceil().trunc_ref()).to_u64().unwrap()
}

fn next_value_nonzero_count<C: Counter>(h: &Histogram<C>, start_value: u64) -> u64 {
    let mut v = h.next_non_equivalent(start_value);

    loop {
        if h.count_at(v) > C::zero() {
            return h.highest_equivalent(v);
        }

        v = h.next_non_equivalent(v);
    }
}

fn prev_value_nonzero_count<C: Counter>(h: &Histogram<C>, start_value: u64) -> u64 {
    let mut v = h.lowest_equivalent(start_value).saturating_sub(1);

    loop {
        if v == 0 {
            return 0;
        }

        if h.count_at(v) > C::zero() {
            return h.highest_equivalent(v);
        }

        v = h.lowest_equivalent(v).saturating_sub(1);
    }
}
