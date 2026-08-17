use crate::tests::helpers::histo64;

#[test]
fn init_fields_smallest_possible_array() {
    let h = histo64(1, 2, 0);

    assert_eq!(2, h.highest_trackable_value);
    assert_eq!(1, h.lowest_discernible_value);
    assert_eq!(0, h.significant_value_digits);

    assert_eq!(2, h.sub_bucket_count);
    assert_eq!(1, h.sub_bucket_half_count);
    assert_eq!(2, h.bucket_count);
    // bottom full bucket, one more half bucket
    assert_eq!(3, h.counts.len());
    assert_eq!(0, h.sub_bucket_half_count_magnitude);
    assert_eq!(1, h.sub_bucket_mask);

    assert_eq!(0, h.unit_magnitude);
    assert_eq!(0, h.unit_magnitude_mask);

    assert_eq!(63, h.leading_zero_count_base);
}

#[test]
fn init_fields_max_value_max_precision_largest_possible_array() {
    let h = histo64(1, u64::max_value(), 5);

    assert_eq!(u64::max_value(), h.highest_trackable_value);
    assert_eq!(1, h.lowest_discernible_value);
    assert_eq!(5, h.significant_value_digits);

    // 5 sigdigs = 100,000. sub bucket must hold 200,000. 2^18 = 262,144.
    assert_eq!(1 << 18, h.sub_bucket_count);
    assert_eq!(1 << 17, h.sub_bucket_half_count);
    // 2^46 * 2^18 = 2^64, so 47 buckets.
    assert_eq!(47, h.bucket_count);
    assert_eq!(
        46 * h.sub_bucket_half_count + h.sub_bucket_count,
        h.counts.len() as u32
    );
    assert_eq!(17, h.sub_bucket_half_count_magnitude);
    assert_eq!((1 << 18) - 1, h.sub_bucket_mask);

    assert_eq!(0, h.unit_magnitude);
    assert_eq!(0, h.unit_magnitude_mask);

    assert_eq!(64 - 17 - 1, h.leading_zero_count_base);
}

#[test]
fn init_fields_max_value_medium_precision() {
    let h = histo64(1, u64::max_value(), 3);

    assert_eq!(u64::max_value(), h.highest_trackable_value);
    assert_eq!(1, h.lowest_discernible_value);
    assert_eq!(3, h.significant_value_digits);

    // should hit the case where it detects impending overflow
    // 3 sigdigs = 1,000. sub bucket must hold 2,000. 2^11 = 2048.
    assert_eq!(1 << 11, h.sub_bucket_count);
    assert_eq!(1 << 10, h.sub_bucket_half_count);
    // 2^53 * 2048 == 2^64, so that's 54 buckets.
    assert_eq!(54, h.bucket_count);
    assert_eq!(
        53 * h.sub_bucket_half_count + h.sub_bucket_count,
        h.counts.len() as u32
    );
    assert_eq!(10, h.sub_bucket_half_count_magnitude);
    assert_eq!((1 << 11) - 1, h.sub_bucket_mask);

    assert_eq!(0, h.unit_magnitude);
    assert_eq!(0, h.unit_magnitude_mask);

    assert_eq!(64 - 10 - 1, h.leading_zero_count_base);
}

#[test]
fn init_fields_1_bucket_medium_precision() {
    let h = histo64(1, 2000, 3);

    assert_eq!(2000, h.highest_trackable_value);
    assert_eq!(1, h.lowest_discernible_value);
    assert_eq!(3, h.significant_value_digits);

    // 3 sigdigs = 1,000. sub bucket must hold 2,000. 2^11 = 2048.
    assert_eq!(1 << 11, h.sub_bucket_count);
    assert_eq!(1 << 10, h.sub_bucket_half_count);
    // 2^0 * 2048 == 2^11, so that's 1 bucket.
    assert_eq!(1, h.bucket_count);
    assert_eq!(h.sub_bucket_count, h.counts.len() as u32);
    assert_eq!(10, h.sub_bucket_half_count_magnitude);
    assert_eq!((1 << 11) - 1, h.sub_bucket_mask);

    assert_eq!(0, h.unit_magnitude);
    assert_eq!(0, h.unit_magnitude_mask);

    assert_eq!(64 - 10 - 1, h.leading_zero_count_base);
}

#[test]
fn init_fields_max_value_0_precision_most_buckets() {
    let h = histo64(1, u64::max_value(), 0);

    assert_eq!(u64::max_value(), h.highest_trackable_value);
    assert_eq!(1, h.lowest_discernible_value);
    assert_eq!(0, h.significant_value_digits);

    // sub bucket must hold 2.
    assert_eq!(2, h.sub_bucket_count);
    assert_eq!(1, h.sub_bucket_half_count);
    // 2^63 * 2 = 2^64, so 64 buckets.
    assert_eq!(64, h.bucket_count);
    // 63 half buckets, one full bucket
    assert_eq!(63 + 2, h.counts.len() as u32);
    assert_eq!(0, h.sub_bucket_half_count_magnitude);
    assert_eq!(1, h.sub_bucket_mask);

    assert_eq!(0, h.unit_magnitude);
    assert_eq!(0, h.unit_magnitude_mask);

    assert_eq!(64 - 1, h.leading_zero_count_base);
}

#[test]
fn init_fields_max_value_0_precision_increased_min_value() {
    let h = histo64(1000, u64::max_value(), 0);

    assert_eq!(u64::max_value(), h.highest_trackable_value);
    assert_eq!(1000, h.lowest_discernible_value);
    assert_eq!(0, h.significant_value_digits);

    // sub bucket must hold 2 * 10^0 = 2.
    assert_eq!(2, h.sub_bucket_count);
    assert_eq!(1, h.sub_bucket_half_count);
    // 2 << (unit magnitude = 9) = 2^10
    // 2^54 * 2^10 = 2^64, so 55 buckets.
    assert_eq!(55, h.bucket_count);
    // 54 half buckets, one full bucket
    assert_eq!(54 + 2, h.counts.len() as u32);
    assert_eq!(0, h.sub_bucket_half_count_magnitude);
    assert_eq!(1 << 9, h.sub_bucket_mask);

    assert_eq!(9, h.unit_magnitude);
    // bottom 9 bits
    assert_eq!((1 << 9) - 1, h.unit_magnitude_mask);

    assert_eq!(64 - 9 - 1, h.leading_zero_count_base);
}

#[test]
fn init_fields_max_value_max_precision_increased_min_value() {
    let h = histo64(1000, u64::max_value(), 5);

    assert_eq!(u64::max_value(), h.highest_trackable_value);
    assert_eq!(1000, h.lowest_discernible_value);
    assert_eq!(5, h.significant_value_digits);

    // sub bucket must hold 2 * 10^5 = 200,000, so 2^18
    assert_eq!(1 << 18, h.sub_bucket_count);
    assert_eq!(1 << 17, h.sub_bucket_half_count);
    // 2^18 << (unit magnitude = 9) = 2^27
    // 2^37 * 2^27 = 2^64, so 38 buckets.
    assert_eq!(38, h.bucket_count);
    // 37 half buckets, one full bucket
    assert_eq!(
        37 * h.sub_bucket_half_count + h.sub_bucket_count,
        h.counts.len() as u32
    );
    assert_eq!(17, h.sub_bucket_half_count_magnitude);
    assert_eq!(((1 << 18) - 1) << 9, h.sub_bucket_mask);

    assert_eq!(9, h.unit_magnitude);
    // bottom 9 bits
    assert_eq!((1 << 9) - 1, h.unit_magnitude_mask);

    assert_eq!(64 - 9 - 17 - 1, h.leading_zero_count_base);
}

#[test]
fn init_fields_10m_max_1k_min_middle_precision() {
    let h = histo64(1000, 10_000_000, 3);

    assert_eq!(10_000_000, h.highest_trackable_value);
    assert_eq!(1000, h.lowest_discernible_value);
    assert_eq!(3, h.significant_value_digits);

    // sub bucket must hold 2 * 10^3 = 2,000, so 2^11
    assert_eq!(1 << 11, h.sub_bucket_count);
    assert_eq!(1 << 10, h.sub_bucket_half_count);
    // 2^11 << (unit magnitude = 9) = 2^20
    // 2^24 is 16M.
    // 2^4 * 2^20 = 2^24, so 5 buckets.
    assert_eq!(5, h.bucket_count);
    // 4 half buckets, one full bucket
    assert_eq!(
        4 * h.sub_bucket_half_count + h.sub_bucket_count,
        h.counts.len() as u32
    );
    assert_eq!(10, h.sub_bucket_half_count_magnitude);
    assert_eq!(((1 << 11) - 1) << 9, h.sub_bucket_mask);

    assert_eq!(9, h.unit_magnitude);
    // bottom 9 bits
    assert_eq!((1 << 9) - 1, h.unit_magnitude_mask);

    assert_eq!(64 - 9 - 10 - 1, h.leading_zero_count_base);
}

#[test]
fn init_fields_max_value_max_unit_magnitude_0_precision() {
    let h = histo64(u64::max_value() / 4, u64::max_value(), 0);

    assert_eq!(u64::max_value(), h.highest_trackable_value);
    assert_eq!(u64::max_value() / 4, h.lowest_discernible_value);
    assert_eq!(0, h.significant_value_digits);

    // sub bucket must hold 2 * 10^0
    assert_eq!(2, h.sub_bucket_count);
    assert_eq!(1, h.sub_bucket_half_count);
    // 2^1 << (unit magnitude = 62) = 2^63
    // 2^1 * 2^63 = 2^64, so 2 buckets.
    assert_eq!(2, h.bucket_count);
    // 1 half buckets, one full bucket
    assert_eq!(
        h.sub_bucket_half_count + h.sub_bucket_count,
        h.counts.len() as u32
    );
    assert_eq!(0, h.sub_bucket_half_count_magnitude);
    assert_eq!((2 - 1) << 62, h.sub_bucket_mask);
    // didn't shift off too much
    assert_eq!(1, h.sub_bucket_mask.count_ones());

    assert_eq!(62, h.unit_magnitude);
    // bottom 62 bits
    assert_eq!((1 << 62) - 1, h.unit_magnitude_mask);

    assert_eq!(64 - 62 - 1, h.leading_zero_count_base);
}

#[test]
fn init_fields_max_value_max_unit_magnitude_max_precision() {
    let h = histo64(1 << 45, u64::max_value(), 5);

    assert_eq!(u64::max_value(), h.highest_trackable_value);
    assert_eq!(1 << 45, h.lowest_discernible_value);
    assert_eq!(5, h.significant_value_digits);

    // sub bucket must hold 2 * 10^5, so 2^18
    assert_eq!(1 << 18, h.sub_bucket_count);
    assert_eq!(1 << 17, h.sub_bucket_half_count);
    // 2^18 << (unit magnitude = 45) = 2^63
    // 2^1 * 2^63 = 2^64, so 2 buckets.
    assert_eq!(2, h.bucket_count);
    // 1 half buckets, one full bucket
    assert_eq!(
        h.sub_bucket_half_count + h.sub_bucket_count,
        h.counts.len() as u32
    );
    assert_eq!(17, h.sub_bucket_half_count_magnitude);
    assert_eq!(((1 << 18) - 1) << 45, h.sub_bucket_mask);
    // didn't shift off too much
    assert_eq!(18, h.sub_bucket_mask.count_ones());

    assert_eq!(45, h.unit_magnitude);
    // bottom 45 bits
    assert_eq!((1 << 45) - 1, h.unit_magnitude_mask);

    assert_eq!(64 - 62 - 1, h.leading_zero_count_base);
}
