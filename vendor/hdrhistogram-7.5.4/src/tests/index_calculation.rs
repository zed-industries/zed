use super::super::{CreationError, Histogram};
use crate::tests::helpers::histo64;

#[test]
fn unit_magnitude_0_index_calculations() {
    let h = histo64(1_u64, 1_u64 << 32, 3);
    assert_eq!(2048, h.sub_bucket_count);
    assert_eq!(0, h.unit_magnitude);
    // sub_bucket_count = 2^11, so 2^11 << 22 is > the max of 2^32 for 23 buckets total
    assert_eq!(23, h.bucket_count);

    // first half of first bucket
    assert_eq!(0, h.bucket_for(3));
    assert_eq!(3, h.sub_bucket_for(3, 0));

    // second half of first bucket
    assert_eq!(0, h.bucket_for(1024 + 3));
    assert_eq!(1024 + 3, h.sub_bucket_for(1024 + 3, 0));

    // second bucket (top half)
    assert_eq!(1, h.bucket_for(2048 + 3 * 2));
    // counting by 2s, starting at halfway through the bucket
    assert_eq!(1024 + 3, h.sub_bucket_for(2048 + 3 * 2, 1));

    // third bucket (top half)
    assert_eq!(2, h.bucket_for((2048 << 1) + 3 * 4));
    // counting by 4s, starting at halfway through the bucket
    assert_eq!(1024 + 3, h.sub_bucket_for((2048 << 1) + 3 * 4, 2));

    // past last bucket -- not near u64::max_value(), so should still calculate ok.
    assert_eq!(23, h.bucket_for((2048_u64 << 22) + 3 * (1 << 23)));
    assert_eq!(
        1024 + 3,
        h.sub_bucket_for((2048_u64 << 22) + 3 * (1 << 23), 23)
    );
}

#[test]
fn unit_magnitude_4_index_calculations() {
    let h = histo64(1_u64 << 12, 1_u64 << 32, 3);
    assert_eq!(2048, h.sub_bucket_count);
    assert_eq!(12, h.unit_magnitude);
    // sub_bucket_count = 2^11. With unit magnitude shift, it's 2^23. 2^23 << 10 is > the max of
    // 2^32 for 11 buckets total
    assert_eq!(11, h.bucket_count);
    let unit = 1_u64 << 12;

    // below lowest value
    assert_eq!(0, h.bucket_for(3));
    assert_eq!(0, h.sub_bucket_for(3, 0));

    // first half of first bucket
    assert_eq!(0, h.bucket_for(3 * unit));
    assert_eq!(3, h.sub_bucket_for(3 * unit, 0));

    // second half of first bucket
    // sub_bucket_half_count's worth of units, plus 3 more
    assert_eq!(0, h.bucket_for(unit * (1024 + 3)));
    assert_eq!(1024 + 3, h.sub_bucket_for(unit * (1024 + 3), 0));

    // second bucket (top half), bucket scale = unit << 1.
    // Middle of bucket is (sub_bucket_half_count = 2^10) of bucket scale, = unit << 11.
    // Add on 3 of bucket scale.
    assert_eq!(1, h.bucket_for((unit << 11) + 3 * (unit << 1)));
    assert_eq!(
        1024 + 3,
        h.sub_bucket_for((unit << 11) + 3 * (unit << 1), 1)
    );

    // third bucket (top half), bucket scale = unit << 2.
    // Middle of bucket is (sub_bucket_half_count = 2^10) of bucket scale, = unit << 12.
    // Add on 3 of bucket scale.
    assert_eq!(2, h.bucket_for((unit << 12) + 3 * (unit << 2)));
    assert_eq!(
        1024 + 3,
        h.sub_bucket_for((unit << 12) + 3 * (unit << 2), 2)
    );

    // past last bucket -- not near u64::max_value(), so should still calculate ok.
    assert_eq!(11, h.bucket_for((unit << 21) + 3 * (unit << 11)));
    assert_eq!(
        1024 + 3,
        h.sub_bucket_for((unit << 21) + 3 * (unit << 11), 11)
    );
}

#[test]
fn unit_magnitude_52_sub_bucket_magnitude_11_index_calculations() {
    // maximum unit magnitude for this precision
    let h = histo64(1_u64 << 52, u64::max_value(), 3);
    assert_eq!(2048, h.sub_bucket_count);
    assert_eq!(52, h.unit_magnitude);
    // sub_bucket_count = 2^11. With unit magnitude shift, it's 2^63. 1 more bucket to (almost)
    // reach 2^64.
    assert_eq!(2, h.bucket_count);
    assert_eq!(1, h.leading_zero_count_base);
    let unit = 1_u64 << 52;

    // below lowest value
    assert_eq!(0, h.bucket_for(3));
    assert_eq!(0, h.sub_bucket_for(3, 0));

    // first half of first bucket
    assert_eq!(0, h.bucket_for(3 * unit));
    assert_eq!(3, h.sub_bucket_for(3 * unit, 0));

    // second half of first bucket
    // sub_bucket_half_count's worth of units, plus 3 more
    assert_eq!(0, h.bucket_for(unit * (1024 + 3)));
    assert_eq!(1024 + 3, h.sub_bucket_for(unit * (1024 + 3), 0));

    // end of second half
    assert_eq!(0, h.bucket_for(unit * 1024 + 1023 * unit));
    assert_eq!(1024 + 1023, h.sub_bucket_for(unit * 1024 + 1023 * unit, 0));

    // second bucket (top half), bucket scale = unit << 1.
    // Middle of bucket is (sub_bucket_half_count = 2^10) of bucket scale, = unit << 11.
    // Add on 3 of bucket scale.
    assert_eq!(1, h.bucket_for((unit << 11) + 3 * (unit << 1)));
    assert_eq!(
        1024 + 3,
        h.sub_bucket_for((unit << 11) + 3 * (unit << 1), 1)
    );

    // upper half of second bucket, last slot
    assert_eq!(1, h.bucket_for(u64::max_value()));
    assert_eq!(1024 + 1023, h.sub_bucket_for(u64::max_value(), 1));
}

#[test]
fn unit_magnitude_53_sub_bucket_magnitude_11_throws() {
    assert_eq!(
        CreationError::CannotRepresentSigFigBeyondLow,
        Histogram::<u64>::new_with_bounds(1_u64 << 53, 1_u64 << 63, 3).unwrap_err()
    );
}

#[test]
fn unit_magnitude_55_sub_bucket_magnitude_8_ok() {
    let h = histo64(1_u64 << 55, 1_u64 << 63, 2);
    assert_eq!(256, h.sub_bucket_count);
    assert_eq!(55, h.unit_magnitude);
    // sub_bucket_count = 2^8. With unit magnitude shift, it's 2^63.
    assert_eq!(2, h.bucket_count);

    // below lowest value
    assert_eq!(0, h.bucket_for(3));
    assert_eq!(0, h.sub_bucket_for(3, 0));

    // upper half of second bucket, last slot
    assert_eq!(1, h.bucket_for(u64::max_value()));
    assert_eq!(128 + 127, h.sub_bucket_for(u64::max_value(), 1));
}

#[test]
fn unit_magnitude_62_sub_bucket_magnitude_1_ok() {
    let h = histo64(1_u64 << 62, 1_u64 << 63, 0);
    assert_eq!(2, h.sub_bucket_count);
    assert_eq!(62, h.unit_magnitude);
    // sub_bucket_count = 2^1. With unit magnitude shift, it's 2^63.
    assert_eq!(2, h.bucket_count);

    // below lowest value
    assert_eq!(0, h.bucket_for(3));
    assert_eq!(0, h.sub_bucket_for(3, 0));

    // upper half of second bucket, last slot
    assert_eq!(1, h.bucket_for(u64::max_value()));
    assert_eq!(1, h.sub_bucket_for(u64::max_value(), 1));
}

#[test]
fn bucket_for_smallest_value_in_first_bucket() {
    let h = histo64(1, 100_000, 3);
    assert_eq!(0, h.bucket_for(0))
}

#[test]
fn bucket_for_biggest_value_in_first_bucket() {
    let h = histo64(1, 100_000, 3);
    // sub bucket size 2048, and first bucket uses all 2048 slots
    assert_eq!(0, h.bucket_for(2047))
}

#[test]
fn bucket_for_smallest_value_in_second_bucket() {
    let h = histo64(1, 100_000, 3);
    assert_eq!(1, h.bucket_for(2048))
}

#[test]
fn bucket_for_biggest_value_in_second_bucket() {
    let h = histo64(1, 100_000, 3);
    // second value uses only 1024 slots, but scales by 2
    assert_eq!(1, h.bucket_for(4095))
}

#[test]
fn bucket_for_smallest_value_in_third_bucket() {
    let h = histo64(1, 100_000, 3);
    assert_eq!(2, h.bucket_for(4096))
}

#[test]
fn bucket_for_smallest_value_in_last_bucket() {
    let h = histo64(1, 100_000, 3);

    // 7 buckets total
    assert_eq!(6, h.bucket_for(65536))
}

#[test]
fn bucket_for_value_below_smallest_clamps_to_zero() {
    let h = histo64(1024, 100_000, 3);

    // masking clamps bucket index to 0
    assert_eq!(0, h.bucket_for(0));
    assert_eq!(0, h.bucket_for(1));
    assert_eq!(0, h.bucket_for(1023));
    assert_eq!(0, h.bucket_for(1024))
}

#[test]
fn bucket_for_value_above_biggest_isnt_clamped_at_max_bucket() {
    let h = histo64(1, 100_000, 3);

    assert_eq!(6, h.bucket_for(100_000));
    // 2048 * 2^26 = 137,438,953,472
    assert_eq!(26, h.bucket_for(100_000_000_000));
}

#[test]
fn sub_bucket_for_zero_value_in_first_bucket() {
    let h = histo64(1, 100_000, 3);
    // below min distinguishable value, but still gets bucketed into 0
    let value = 0;
    assert_eq!(0, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_smallest_distinguishable_value_in_first_bucket() {
    let h = histo64(1, 100_000, 3);
    let value = 1;
    assert_eq!(1, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_zero_value_in_first_bucket_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);
    let value = 0;
    assert_eq!(2, h.unit_magnitude);
    assert_eq!(0, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_smaller_than_distinguishable_value_in_first_bucket_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);
    let value = 3;
    assert_eq!(2, h.unit_magnitude);
    assert_eq!(0, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_smallest_distinguishable_value_in_first_bucket_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);
    let value = 4;
    assert_eq!(2, h.unit_magnitude);
    assert_eq!(1, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_largest_value_in_first_bucket_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);
    let value = 2048 * 4 - 1;
    assert_eq!(2, h.unit_magnitude);
    assert_eq!(2047, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_smallest_value_in_second_bucket_unit_magnitude_2() {
    let h = histo64(4, 100_000, 3);
    let value = 2048 * 4;
    assert_eq!(2, h.unit_magnitude);
    assert_eq!(1024, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_largest_value_in_first_bucket() {
    let h = histo64(1, 100_000, 3);
    let value = 2047;
    assert_eq!(2047, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_smallest_value_in_second_bucket() {
    let h = histo64(1, 100_000, 3);
    let value = 2048;

    // at midpoint of bucket, which is the first position actually used in second bucket
    assert_eq!(1024, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_biggest_value_in_second_bucket() {
    let h = histo64(1, 100_000, 3);
    let value = 4095;

    // at endpoint of bucket, which is the last position actually used in second bucket
    assert_eq!(2047, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_smallest_value_in_third_bucket() {
    let h = histo64(1, 100_000, 3);
    let value = 4096;

    assert_eq!(1024, h.sub_bucket_for(value, h.bucket_for(value)))
}

#[test]
fn sub_bucket_for_value_below_smallest_clamps_to_zero() {
    let h = histo64(1024, 100_000, 3);

    assert_eq!(0, h.sub_bucket_for(0, 0));
    assert_eq!(0, h.sub_bucket_for(1, 0));
    assert_eq!(0, h.sub_bucket_for(1023, 0));
    assert_eq!(1, h.sub_bucket_for(1024, 0))
}

#[test]
fn sub_bucket_for_value_above_biggest_still_works() {
    let h = histo64(1, 1024 * 1024, 3);

    // normal case:
    // in bucket index 6, scales by 2^6 = 64, start is at 65536.
    // 100_000 - 65536 = 34_464. 34464 / 64 = 538.5. +1024 = 1562
    assert_eq!(1562, h.sub_bucket_for(100_000, h.bucket_for(100_000)));

    // still in sub bucket count but nonsensical
    // In bucket 26, effective start is 1024 * 2^26 = 68,719,476,736.
    // 100b - start = 31,280,523,264. That / 2^26 = 466.1.
    assert_eq!(
        466 + 1024,
        h.sub_bucket_for(100_000_000_000, h.bucket_for(100_000_000_000))
    );
}

#[test]
fn index_for_first_bucket_first_entry() {
    let h = histo64(1, 100_000, 3);
    assert_eq!(0, h.index_for(0).unwrap());
}

#[test]
fn index_for_first_bucket_first_distinguishable_entry() {
    let h = histo64(1, 100_000, 3);
    assert_eq!(1, h.index_for(1).unwrap());
}

#[test]
fn index_for_first_bucket_last_entry() {
    let h = histo64(1, 100_000, 3);
    assert_eq!(2047, h.index_for(2047).unwrap());
}

#[test]
fn index_for_second_bucket_last_entry() {
    let h = histo64(1, 100_000, 3);
    assert_eq!(2048 + 1023, h.index_for(2048 + 2047).unwrap());
}

#[test]
fn index_for_second_bucket_last_entry_indistinguishable() {
    let h = histo64(1, 100_000, 3);
    assert_eq!(2048 + 1023, h.index_for(2048 + 2046).unwrap());
}

#[test]
fn index_for_second_bucket_first_entry() {
    let h = histo64(1, 100_000, 3);
    assert_eq!(2048, h.index_for(2048).unwrap());
}

#[test]
fn index_for_below_smallest() {
    let h = histo64(1024, 100_000, 3);

    assert_eq!(0, h.index_for(512).unwrap());
}

#[test]
fn index_for_way_past_largest_value_exceeds_length() {
    let h = histo64(1, 100_000, 3);

    // 7 * 1024 + 1 more 1024
    assert_eq!(8 * 1024, h.counts.len());

    // 2^39 = 1024 * 2^29, so this should be the start of the 30th bucket.
    // Start index is (bucket index + 1) * 1024.
    assert_eq!(1024 * (30 + 1), h.index_for(1 << 40).unwrap());
}
