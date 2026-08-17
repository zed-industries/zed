//! Tests from HistogramTest.java

use rand::{Rng, SeedableRng};

use hdrhistogram::{Counter, Histogram, SubtractionError};
use std::borrow::Borrow;
use std::fmt;

macro_rules! assert_near {
    ($a:expr, $b:expr, $tolerance:expr) => {{
        let a = $a as f64;
        let b = $b as f64;
        let tol = $tolerance as f64;
        assert!(
            (a - b).abs() <= b * tol,
            "assertion failed: `(left ~= right) (left: `{}`, right: `{}`, tolerance: `{:.5}%`)",
            a,
            b,
            100.0 * tol
        );
    }};
}

fn verify_max<T: Counter, B: Borrow<Histogram<T>>>(hist: B) -> bool {
    let hist = hist.borrow();
    if let Some(mx) = hist
        .iter_recorded()
        .map(|v| v.value_iterated_to())
        .map(|v| hist.highest_equivalent(v))
        .last()
    {
        hist.max() == mx
    } else {
        hist.max() == 0
    }
}

const TRACKABLE_MAX: u64 = 3600 * 1000 * 1000;
// Store up to 2 * 10^3 in single-unit precision. Can be 5 at most.
const SIGFIG: u8 = 3;
const TEST_VALUE_LEVEL: u64 = 4;

#[test]
fn construction_arg_ranges() {
    assert!(Histogram::<u64>::new_with_max(1, SIGFIG).is_err());
    assert!(Histogram::<u64>::new_with_max(TRACKABLE_MAX, 6).is_err());
}

#[test]
fn empty_histogram() {
    let h = Histogram::<u64>::new(SIGFIG).unwrap();
    assert_eq!(h.min(), 0);
    assert_eq!(h.max(), 0);
    assert_near!(h.mean(), 0.0, 0.0000000000001);
    assert_near!(h.stdev(), 0.0, 0.0000000000001);
    assert_near!(h.quantile_below(0), 1.0, 0.0000000000001);
    assert!(verify_max(h));
}

#[test]
fn construction_arg_gets() {
    let h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.low(), 1);
    assert_eq!(h.high(), TRACKABLE_MAX);
    assert_eq!(h.sigfig(), SIGFIG);

    let h = Histogram::<u64>::new_with_bounds(1000, TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.low(), 1000);
}

#[test]
fn record() {
    let mut h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    h += TEST_VALUE_LEVEL;
    assert_eq!(h.count_at(TEST_VALUE_LEVEL), 1);
    assert_eq!(h.len(), 1);
    assert!(verify_max(h));
}

#[test]
fn record_past_trackable_max() {
    let mut h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    assert!(h.record(3 * TRACKABLE_MAX).is_err());
}

#[test]
fn saturating_record() {
    let mut h = Histogram::<u64>::new_with_bounds(512, TRACKABLE_MAX, SIGFIG).unwrap();

    h.saturating_record(1); // clamped below
    h.saturating_record(1000 * 1000); // not clamped
    h.saturating_record(3 * TRACKABLE_MAX); // clamped above

    // https://github.com/HdrHistogram/HdrHistogram_rust/pull/74#discussion_r158192909
    assert_eq!(h.count_at(511), 1);
    assert_eq!(h.count_at(1000 * 1000), 1);
    assert_eq!(h.count_at(h.high()), 1);
    assert_eq!(h.len(), 3);
    assert!(verify_max(h));
}

#[test]
fn record_in_interval() {
    let mut h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    h.record_correct(TEST_VALUE_LEVEL, TEST_VALUE_LEVEL / 4)
        .unwrap();
    let mut r = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    r += TEST_VALUE_LEVEL;

    // The data will include corrected samples:
    assert_eq!(h.count_at(TEST_VALUE_LEVEL / 4), 1);
    assert_eq!(h.count_at((TEST_VALUE_LEVEL * 2) / 4), 1);
    assert_eq!(h.count_at((TEST_VALUE_LEVEL * 3) / 4), 1);
    assert_eq!(h.count_at((TEST_VALUE_LEVEL * 4) / 4), 1);
    assert_eq!(h.len(), 4);
    // But the raw data will not:
    assert_eq!(r.count_at(TEST_VALUE_LEVEL / 4), 0);
    assert_eq!(r.count_at((TEST_VALUE_LEVEL * 2) / 4), 0);
    assert_eq!(r.count_at((TEST_VALUE_LEVEL * 3) / 4), 0);
    assert_eq!(r.count_at((TEST_VALUE_LEVEL * 4) / 4), 1);
    assert_eq!(r.len(), 1);

    assert!(verify_max(h));
}

#[test]
fn reset() {
    let mut h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    h += TEST_VALUE_LEVEL;
    h.reset();

    assert_eq!(h.count_at(TEST_VALUE_LEVEL), 0);
    assert_eq!(h.len(), 0);
    assert!(verify_max(h));
}

#[test]
fn add() {
    let mut h1 = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();

    h1 += TEST_VALUE_LEVEL;
    h1 += 1000 * TEST_VALUE_LEVEL;
    h2 += TEST_VALUE_LEVEL;
    h2 += 1000 * TEST_VALUE_LEVEL;
    h1 += &h2;

    assert_eq!(h1.count_at(TEST_VALUE_LEVEL), 2);
    assert_eq!(h1.count_at(1000 * TEST_VALUE_LEVEL), 2);
    assert_eq!(h1.len(), 4);

    let mut big = Histogram::<u64>::new_with_max(2 * TRACKABLE_MAX, SIGFIG).unwrap();
    big += TEST_VALUE_LEVEL;
    big += 1000 * TEST_VALUE_LEVEL;
    big += 2 * TRACKABLE_MAX;

    // Adding the smaller histogram to the bigger one should work:
    big += &h1;
    assert_eq!(big.count_at(TEST_VALUE_LEVEL), 3);
    assert_eq!(big.count_at(1000 * TEST_VALUE_LEVEL), 3);
    assert_eq!(big.count_at(2 * TRACKABLE_MAX), 1); // overflow smaller hist...
    assert_eq!(big.len(), 7);

    // But trying to add a larger histogram into a smaller one should throw an AIOOB:
    assert!(h1.add(&big).is_err());

    assert!(verify_max(h1));
    assert!(verify_max(h2));
    assert!(verify_max(big));
}

#[test]
fn equivalent_range() {
    let h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.equivalent_range(1), 1);
    assert_eq!(h.equivalent_range(2500), 2);
    assert_eq!(h.equivalent_range(8191), 4);
    assert_eq!(h.equivalent_range(8192), 8);
    assert_eq!(h.equivalent_range(10_000), 8);
}

#[test]
fn scaled_equivalent_range() {
    let h = Histogram::<u64>::new_with_bounds(1024, TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.equivalent_range(1024), 1024);
    assert_eq!(h.equivalent_range(2500 * 1024), 2 * 1024);
    assert_eq!(h.equivalent_range(8191 * 1024), 4 * 1024);
    assert_eq!(h.equivalent_range(8192 * 1024), 8 * 1024);
    assert_eq!(h.equivalent_range(10_000 * 1024), 8 * 1024);
}

#[test]
fn lowest_equivalent() {
    let h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.lowest_equivalent(10_007), 10_000);
    assert_eq!(h.lowest_equivalent(10_009), 10_008);
}

#[test]
fn scaled_lowest_equivalent() {
    let h = Histogram::<u64>::new_with_bounds(1024, TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.lowest_equivalent(10_007 * 1024), 10_000 * 1024);
    assert_eq!(h.lowest_equivalent(10_009 * 1024), 10_008 * 1024);
}

#[test]
fn highest_equivalent() {
    let h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.highest_equivalent(8180), 8183);
    assert_eq!(h.highest_equivalent(8191), 8191);
    assert_eq!(h.highest_equivalent(8193), 8199);
    assert_eq!(h.highest_equivalent(9995), 9999);
    assert_eq!(h.highest_equivalent(10_007), 10_007);
    assert_eq!(h.highest_equivalent(10_008), 10_015);
}

#[test]
fn scaled_highest_equivalent() {
    let h = Histogram::<u64>::new_with_bounds(1024, TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.highest_equivalent(8180 * 1024), 8183 * 1024 + 1023);
    assert_eq!(h.highest_equivalent(8191 * 1024), 8191 * 1024 + 1023);
    assert_eq!(h.highest_equivalent(8193 * 1024), 8199 * 1024 + 1023);
    assert_eq!(h.highest_equivalent(9995 * 1024), 9999 * 1024 + 1023);
    assert_eq!(h.highest_equivalent(10_007 * 1024), 10_007 * 1024 + 1023);
    assert_eq!(h.highest_equivalent(10_008 * 1024), 10_015 * 1024 + 1023);
}

#[test]
fn median_equivalent() {
    let h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.median_equivalent(4), 4);
    assert_eq!(h.median_equivalent(5), 5);
    assert_eq!(h.median_equivalent(4000), 4001);
    assert_eq!(h.median_equivalent(8000), 8002);
    assert_eq!(h.median_equivalent(10_007), 10_004);
}

#[test]
fn median_equivalent_doesnt_panic_at_extremes() {
    let h = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let _ = h.median_equivalent(u64::max_value());
    let _ = h.median_equivalent(u64::max_value() - 1);
    let _ = h.median_equivalent(0);
    let _ = h.median_equivalent(1);
}

#[test]
fn scaled_median_equivalent() {
    let h = Histogram::<u64>::new_with_bounds(1024, TRACKABLE_MAX, SIGFIG).unwrap();
    assert_eq!(h.median_equivalent(1024 * 4), 1024 * 4 + 512);
    assert_eq!(h.median_equivalent(1024 * 5), 1024 * 5 + 512);
    assert_eq!(h.median_equivalent(1024 * 4000), 1024 * 4001);
    assert_eq!(h.median_equivalent(1024 * 8000), 1024 * 8002);
    assert_eq!(h.median_equivalent(1024 * 10_007), 1024 * 10_004);
}

fn are_equal<T, B1, B2>(actual: B1, expected: B2)
where
    T: Counter + fmt::Debug,
    B1: Borrow<Histogram<T>>,
    B2: Borrow<Histogram<T>>,
{
    let actual = actual.borrow();
    let expected = expected.borrow();

    assert_eq!(actual, expected);
    assert_eq!(
        actual.count_at(TEST_VALUE_LEVEL),
        expected.count_at(TEST_VALUE_LEVEL)
    );
    assert_eq!(
        actual.count_at(10 * TEST_VALUE_LEVEL),
        expected.count_at(10 * TEST_VALUE_LEVEL)
    );
    assert_eq!(actual.len(), expected.len());
    assert!(verify_max(expected));
    assert!(verify_max(actual));
}

#[test]
fn clone() {
    let mut h = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    h += TEST_VALUE_LEVEL;
    h += 10 * TEST_VALUE_LEVEL;

    let max = h.high();
    h.record_correct(max - 1, 31_000).unwrap();

    are_equal(h.clone(), h);
}

#[test]
fn scaled_clone() {
    let mut h = Histogram::<u64>::new_with_bounds(1000, TRACKABLE_MAX, SIGFIG).unwrap();
    h += TEST_VALUE_LEVEL;
    h += 10 * TEST_VALUE_LEVEL;

    let max = h.high();
    h.record_correct(max - 1, 31_000).unwrap();

    are_equal(h.clone(), h);
}

#[test]
fn set_to() {
    let mut h1 = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    h1 += TEST_VALUE_LEVEL;
    h1 += 10 * TEST_VALUE_LEVEL;

    let max = h1.high();
    h1.record_correct(max - 1, 31_000).unwrap();

    h2.set_to(&h1).unwrap();
    are_equal(&h1, &h2);

    h1 += 20 * TEST_VALUE_LEVEL;

    h2.set_to(&h1).unwrap();
    are_equal(&h1, &h2);
}

#[test]
fn scaled_set_to() {
    let mut h1 = Histogram::<u64>::new_with_bounds(1000, TRACKABLE_MAX, SIGFIG).unwrap();
    let mut h2 = Histogram::<u64>::new_with_bounds(1000, TRACKABLE_MAX, SIGFIG).unwrap();
    h1 += TEST_VALUE_LEVEL;
    h1 += 10 * TEST_VALUE_LEVEL;

    let max = h1.high();
    h1.record_correct(max - 1, 31_000).unwrap();

    h2.set_to(&h1).unwrap();
    are_equal(&h1, &h2);

    h1 += 20 * TEST_VALUE_LEVEL;

    h2.set_to(&h1).unwrap();
    are_equal(&h1, &h2);
}

#[test]
fn random_write_full_value_range_precision_5_no_panic() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 5).unwrap();

    let mut rng = rand::rngs::SmallRng::from_entropy();

    for _ in 0..1_000_000 {
        let mut r: u64 = rng.gen();
        if r == 0 {
            r = 1;
        }

        h.record(r).unwrap();
    }
}

#[test]
fn random_write_full_value_range_precision_0_no_panic() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 0).unwrap();

    let mut rng = rand::rngs::SmallRng::from_entropy();

    for _ in 0..1_000_000 {
        let mut r: u64 = rng.gen();
        if r == 0 {
            r = 1;
        }

        h.record(r).unwrap();
    }
}

#[test]
fn random_write_middle_of_value_range_precision_3_no_panic() {
    let low = 1_000;
    let high = 1_000_000_000;
    let mut h = Histogram::<u64>::new_with_bounds(low, high, 3).unwrap();

    let mut rng = rand::rngs::SmallRng::from_entropy();

    for _ in 0..1_000_000 {
        h.record(rng.gen_range(low..=high)).unwrap();
    }
}

#[test]
fn value_count_overflow_from_record_saturates_u16() {
    let mut h = Histogram::<u16>::new_with_max(TRACKABLE_MAX, 2).unwrap();

    h.record_n(3, u16::max_value() - 1).unwrap();
    h.record_n(3, u16::max_value() - 1).unwrap();

    // individual count has saturated
    assert_eq!(u16::max_value(), h.count_at(3));
    // total is a u64 though
    assert_eq!(u64::from(u16::max_value() - 1) * 2, h.len());
}

#[test]
fn value_count_overflow_from_record_saturates_u64() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record_n(1, u64::max_value() - 1).unwrap();
    h.record_n(1, u64::max_value() - 1).unwrap();

    assert_eq!(u64::max_value(), h.count_at(1));
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn value_count_overflow_from_record_autoresize_doesnt_panic_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, 10_000, 3).unwrap();
    h.auto(true);

    h.record_n(1, u64::max_value() - 1).unwrap();
    h.record_n(1, u64::max_value() - 1).unwrap();

    // forces resize
    h.record_n(1_000_000_000, u64::max_value() - 1).unwrap();
    h.record_n(1_000_000_000, u64::max_value() - 1).unwrap();

    assert_eq!(u64::max_value(), h.count_at(1));
    assert_eq!(u64::max_value(), h.count_at(1_000_000_000));
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn value_count_overflow_from_add_same_dimensions_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record_n(1, u64::max_value() - 1).unwrap();
    h2.record_n(1, u64::max_value() - 1).unwrap();

    h.add(h2).unwrap();
    assert_eq!(u64::max_value(), h.count_at(1));
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn value_count_overflow_from_add_different_precision_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    // different precision
    let mut h2 = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 4).unwrap();

    h.record_n(1, u64::max_value() - 1).unwrap();
    h2.record_n(1, u64::max_value() - 1).unwrap();

    h.add(h2).unwrap();
    assert_eq!(u64::max_value(), h.count_at(1));
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn value_count_overflow_from_add_with_resize_to_same_dimensions_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, 10_000, 3).unwrap();
    h.auto(true);
    let mut h2 = Histogram::<u64>::new_with_bounds(1, 10_000_000_000, 3).unwrap();

    h.record_n(1, u64::max_value() - 1).unwrap();
    h2.record_n(1, u64::max_value() - 1).unwrap();
    // recording at value == h2 max should trigger h to resize to the same dimensions when added
    h2.record_n(10_000_000_000, u64::max_value() - 1).unwrap();

    h.add(h2).unwrap();
    assert_eq!(u64::max_value(), h.count_at(1));
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn total_count_overflow_from_record_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record_n(1, u64::max_value() - 1).unwrap();
    h.record_n(10, u64::max_value() - 1).unwrap();

    assert_eq!(u64::max_value() - 1, h.count_at(1));
    assert_eq!(u64::max_value() - 1, h.count_at(10));
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn total_count_overflow_from_add_same_dimensions_saturates_calculating_other_addend_total() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record_n(1, u64::max_value() - 10).unwrap();
    h2.record_n(10, u64::max_value() - 1).unwrap();
    h2.record_n(20, 10).unwrap();

    // just h2's total would overflow

    h.add(h2).unwrap();
    assert_eq!(u64::max_value() - 10, h.count_at(1));
    assert_eq!(10, h.count_at(20));

    // if accumulating total count for h2 had overflowed, we would see max_value - 1000 + 9 here
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn total_count_overflow_from_add_same_dimensions_saturates_when_added_to_orig_total_count() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record_n(1, u64::max_value() - 10).unwrap();
    h2.record_n(10, 9).unwrap();
    h2.record_n(20, 9).unwrap();

    // h2's total wouldn't overflow, but it would when added to h1

    h.add(h2).unwrap();
    assert_eq!(u64::max_value() - 10, h.count_at(1));
    assert_eq!(9, h.count_at(20));
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn total_count_overflow_from_add_different_precision_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    // different precision
    let mut h2 = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 4).unwrap();

    h.record_n(1, u64::max_value() - 1).unwrap();

    h2.record_n(20, u64::max_value() - 1).unwrap();

    h.add(h2).unwrap();
    assert_eq!(u64::max_value() - 1, h.count_at(1));
    assert_eq!(u64::max_value() - 1, h.count_at(20));
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn total_count_overflow_from_add_with_resize_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, 10_000, 3).unwrap();
    h.auto(true);
    let mut h2 = Histogram::<u64>::new_with_bounds(1, 10_000_000_000, 3).unwrap();

    h.record_n(1, u64::max_value() - 1).unwrap();
    h2.record_n(1, u64::max_value() - 1).unwrap();
    h2.record_n(10_000_000_000, u64::max_value() - 1).unwrap();

    h.add(h2).unwrap();
    assert_eq!(u64::max_value(), h.count_at(1));
    assert_eq!(u64::max_value(), h.len());
}

#[test]
fn subtract_underflow_guarded_by_per_value_count_check() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record_n(1, 1).unwrap();
    h2.record_n(1, 100).unwrap();

    assert_eq!(
        SubtractionError::SubtrahendCountExceedsMinuendCount,
        h.subtract(h2).unwrap_err()
    );
}

#[test]
fn recorded_only_zeros() {
    let mut h = Histogram::<u64>::new(1).unwrap();
    h += 0;
    assert_eq!(h.iter_recorded().count(), 1);
}
