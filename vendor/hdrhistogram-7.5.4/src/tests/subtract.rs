use crate::{Counter, Histogram, SubtractionError};
use num_traits::Saturating;
use std::borrow::Borrow;
use std::cmp;

const TEST_VALUE_LEVEL: u64 = 4;

fn assert_min_max_count<T: Counter, B: Borrow<Histogram<T>>>(hist: B) {
    let h = hist.borrow();
    let mut min = None;
    let mut max = None;
    let mut total = 0;
    for i in 0..h.distinct_values() {
        let value = h.value_for(i);
        let count = h.count_at(value);
        if count == T::zero() {
            continue;
        }

        min = Some(cmp::min(min.unwrap_or(u64::max_value()), value));
        max = Some(cmp::max(max.unwrap_or(0), value));
        total = total.saturating_add(count.to_u64().unwrap());
    }

    let min = min.map(|m| h.lowest_equivalent(m)).unwrap_or(0);
    let max = max.map(|m| h.highest_equivalent(m)).unwrap_or(0);

    assert_eq!(min, h.min());
    assert_eq!(max, h.max());
    assert_eq!(total, h.len());
}

#[test]
fn subtract_after_add() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();

    h1 += TEST_VALUE_LEVEL;
    h1 += 1000 * TEST_VALUE_LEVEL;
    h2 += TEST_VALUE_LEVEL;
    h2 += 1000 * TEST_VALUE_LEVEL;

    h1.add(&h2).unwrap();
    assert_eq!(h1.count_at(TEST_VALUE_LEVEL), 2);
    assert_eq!(h1.count_at(1000 * TEST_VALUE_LEVEL), 2);
    assert_eq!(h1.len(), 4);

    h1 += &h2;
    assert_eq!(h1.count_at(TEST_VALUE_LEVEL), 3);
    assert_eq!(h1.count_at(1000 * TEST_VALUE_LEVEL), 3);
    assert_eq!(h1.len(), 6);

    h1.subtract(&h2).unwrap();
    assert_eq!(h1.count_at(TEST_VALUE_LEVEL), 2);
    assert_eq!(h1.count_at(1000 * TEST_VALUE_LEVEL), 2);
    assert_eq!(h1.len(), 4);

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_to_zero_counts() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();

    h1 += TEST_VALUE_LEVEL;
    h1 += 1000 * TEST_VALUE_LEVEL;

    assert_eq!(h1.count_at(TEST_VALUE_LEVEL), 1);
    assert_eq!(h1.count_at(1000 * TEST_VALUE_LEVEL), 1);
    assert_eq!(h1.len(), 2);

    let clone = h1.clone();
    h1.subtract(&clone).unwrap();
    assert_eq!(h1.count_at(TEST_VALUE_LEVEL), 0);
    assert_eq!(h1.count_at(1000 * TEST_VALUE_LEVEL), 0);
    assert_eq!(h1.len(), 0);

    assert_min_max_count(h1);
}

#[test]
fn subtract_to_negative_counts_error() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();

    h1 += TEST_VALUE_LEVEL;
    h1 += 1000 * TEST_VALUE_LEVEL;
    h2.record_n(TEST_VALUE_LEVEL, 2).unwrap();
    h2.record_n(1000 * TEST_VALUE_LEVEL, 2).unwrap();

    assert_eq!(
        SubtractionError::SubtrahendCountExceedsMinuendCount,
        h1.subtract(&h2).unwrap_err()
    );

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_subtrahend_values_outside_minuend_range_error() {
    let max = u64::max_value() / 2;
    let mut h1 = Histogram::<u64>::new_with_max(max, 3).unwrap();

    h1 += TEST_VALUE_LEVEL;
    h1 += 1000 * TEST_VALUE_LEVEL;

    let mut big = Histogram::<u64>::new_with_max(2 * max, 3).unwrap();
    big += TEST_VALUE_LEVEL;
    big += 1000 * TEST_VALUE_LEVEL;
    big += 2 * max;

    assert_eq!(
        SubtractionError::SubtrahendValueExceedsMinuendRange,
        h1.subtract(&big).unwrap_err()
    );

    assert_min_max_count(h1);
    assert_min_max_count(big);
}

#[test]
fn subtract_values_inside_minuend_range_works() {
    let max = u64::max_value() / 2;
    let mut h1 = Histogram::<u64>::new_with_max(max, 3).unwrap();

    h1 += TEST_VALUE_LEVEL;
    h1 += 1000 * TEST_VALUE_LEVEL;

    let mut big = Histogram::<u64>::new_with_max(2 * max, 3).unwrap();
    big += TEST_VALUE_LEVEL;
    big += 1000 * TEST_VALUE_LEVEL;
    big += 2 * max;

    let big2 = big.clone();
    big += &big2;
    big += &big2;

    assert_eq!(big.count_at(TEST_VALUE_LEVEL), 3);
    assert_eq!(big.count_at(1000 * TEST_VALUE_LEVEL), 3);
    assert_eq!(big.count_at(2 * max), 3); // overflow smaller hist...
    assert_eq!(big.len(), 9);

    // Subtracting the smaller histogram from the bigger one should work:
    big -= &h1;
    assert_eq!(big.count_at(TEST_VALUE_LEVEL), 2);
    assert_eq!(big.count_at(1000 * TEST_VALUE_LEVEL), 2);
    assert_eq!(big.count_at(2 * max), 3); // overflow smaller hist...
    assert_eq!(big.len(), 7);

    assert_min_max_count(h1);
    assert_min_max_count(big);
}

#[test]
fn subtract_values_strictly_inside_minuend_range_yields_same_min_max_no_restat() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();

    h1 += 1;
    h1 += 10;
    h1 += 100;
    h1 += 1000;

    h2 += 10;
    h2 += 100;

    // will not require a restat
    h1.subtract(&h2).unwrap();

    assert_eq!(1, h1.min());
    assert_eq!(1000, h1.max());
    assert_eq!(2, h1.len());

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_values_at_extent_of_minuend_zero_count_range_recalculates_min_max() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();

    h1 += 1;
    h1 += 10;
    h1 += 100;
    h1 += 1000;

    h2 += 1;
    h2 += 1000;

    // will trigger a restat because min/max values are having counts subtracted
    h1.subtract(&h2).unwrap();

    assert_eq!(10, h1.min());
    assert_eq!(100, h1.max());

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_values_at_extent_of_minuend_nonzero_count_range_recalculates_same_min_max() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();

    h1.record_n(1, 2).unwrap();
    h1.record_n(10, 2).unwrap();
    h1.record_n(100, 2).unwrap();
    h1.record_n(1000, 2).unwrap();

    h2 += 1;
    h2 += 1000;

    // will trigger a restat because min/max values are having counts subtracted
    h1.subtract(&h2).unwrap();

    assert_eq!(1, h1.min());
    assert_eq!(1000, h1.max());

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_values_within_bucket_precision_of_of_minuend_min_recalculates_min_max() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 5).unwrap();

    // sub bucket size is 2 above 2048 with 3 sigfits
    h1.record(3000).unwrap();
    h1.record(3100).unwrap();
    h1.record(3200).unwrap();
    h1.record(3300).unwrap();

    // h2 has 5 sigfits, so bucket size is 1 still
    h2 += 3001;

    // will trigger a restat because min/max values are having counts subtracted
    h1.subtract(&h2).unwrap();

    assert_eq!(3100, h1.min());
    assert_eq!(3301, h1.max());

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_values_at_minuend_min_recalculates_min_max() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 5).unwrap();

    // sub bucket size is 2 above 2048 with 3 sigfits
    h1.record(3000).unwrap();
    h1.record(3100).unwrap();
    h1.record(3200).unwrap();
    h1.record(3300).unwrap();

    // h2 has 5 sigfits, so bucket size is 1 still
    h2 += 3000;

    // will trigger a restat because min/max values are having counts subtracted
    h1.subtract(&h2).unwrap();

    assert_eq!(3100, h1.min());
    assert_eq!(3301, h1.max());

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_values_within_bucket_precision_of_of_minuend_max_recalculates_min_max() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 5).unwrap();

    // sub bucket size is 2 above 2048 with 3 sigfits
    h1.record(3000).unwrap();
    h1.record(3100).unwrap();
    h1.record(3200).unwrap();
    h1.record(3300).unwrap();

    // h2 has 5 sigfits, so bucket size is 1 still
    h2 += 3301;

    // will trigger a restat because min/max values are having counts subtracted
    h1.subtract(&h2).unwrap();

    assert_eq!(3000, h1.min());
    assert_eq!(3201, h1.max());

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_values_at_minuend_max_recalculates_min_max() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 5).unwrap();

    // sub bucket size is 2 above 2048 with 3 sigfits
    h1.record(3000).unwrap();
    h1.record(3100).unwrap();
    h1.record(3200).unwrap();
    h1.record(3300).unwrap();

    // h2 has 5 sigfits, so bucket size is 1 still
    h2 += 3300;

    // will trigger a restat because min/max values are having counts subtracted
    h1.subtract(&h2).unwrap();

    assert_eq!(3000, h1.min());
    assert_eq!(3201, h1.max());

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_values_minuend_saturated_total_recalculates_saturated() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();

    h1.record_n(1, u64::max_value()).unwrap();
    h1.record_n(10, u64::max_value()).unwrap();
    h1.record_n(100, u64::max_value()).unwrap();
    h1.record_n(1000, u64::max_value()).unwrap();

    h2.record(10).unwrap();
    h2.record(100).unwrap();

    // will trigger a restat - total count is saturated
    h1.subtract(&h2).unwrap();

    // min, max haven't changed
    assert_eq!(1, h1.min());
    assert_eq!(1000, h1.max());
    // still saturated
    assert_eq!(u64::max_value(), h1.len());

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}

#[test]
fn subtract_values_minuend_saturated_total_recalculates_not_saturated() {
    let mut h1 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();
    let mut h2 = Histogram::<u64>::new_with_max(u64::max_value(), 3).unwrap();

    // 3 of these is just under u64::max_value()
    let chunk = (u64::max_value() / 16) * 5;

    h1.record_n(1, chunk).unwrap();
    h1.record_n(10, chunk).unwrap();
    h1.record_n(100, chunk).unwrap();
    h1.record_n(1000, chunk).unwrap();

    h2.record_n(10, chunk).unwrap();

    // will trigger a restat - total count is saturated
    h1.subtract(&h2).unwrap();

    // min, max haven't changed
    assert_eq!(1, h1.min());
    assert_eq!(1000, h1.max());
    // not saturated
    assert_eq!(u64::max_value() / 16 * 15, h1.len());

    assert_min_max_count(h1);
    assert_min_max_count(h2);
}
