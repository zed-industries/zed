use hdrhistogram::Histogram;

#[test]
fn iter_recorded_non_saturated_total_count() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record(1).unwrap();
    h.record(1_000).unwrap();
    h.record(1_000_000).unwrap();

    let expected = vec![1, 1_000, h.highest_equivalent(1_000_000)];
    assert_eq!(
        expected,
        h.iter_recorded()
            .map(|iv| iv.value_iterated_to())
            .collect::<Vec<u64>>()
    );
}

#[test]
fn iter_recorded_saturated_total_count() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record_n(1, u64::max_value()).unwrap();
    h.record_n(1_000, u64::max_value()).unwrap();
    h.record_n(1_000_000, u64::max_value()).unwrap();

    let expected = vec![1, 1_000, h.highest_equivalent(1_000_000)];
    assert_eq!(
        expected,
        h.iter_recorded()
            .map(|iv| iv.value_iterated_to())
            .collect::<Vec<u64>>()
    );
}

#[test]
fn iter_linear_count_since_last_iteration_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record_n(1, u64::max_value()).unwrap();
    h.record_n(4, u64::max_value() - 1).unwrap();
    h.record_n(5, u64::max_value() - 1).unwrap();
    h.record_n(6, 100).unwrap();
    h.record_n(7, 200).unwrap();
    h.record_n(10, 400).unwrap();

    let expected = vec![
        // 0-1 has 1's max value
        (1, u64::max_value()),
        // 2-3 has nothing
        (3, 0),
        // 4-5 has 2x (max - 1), should saturate
        (5, u64::max_value()),
        // 6-7 shouldn't be saturated from 4-5
        (7, 300),
        // 8-9 has nothing
        (9, 0),
        // 10-11 has just 10's count
        (11, 400),
    ];

    // step in 2s to test count accumulation for each step
    assert_eq!(
        expected,
        h.iter_linear(2)
            .map(|iv| (iv.value_iterated_to(), iv.count_since_last_iteration()))
            .collect::<Vec<(u64, u64)>>()
    );
}

#[test]
fn iter_linear_visits_buckets_wider_than_step_size_multiple_times() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record(1).unwrap();
    h.record(2047).unwrap();
    // bucket size 2
    h.record(2048).unwrap();
    h.record(2049).unwrap();
    h.record(4095).unwrap();
    // bucket size 4
    h.record(4096).unwrap();
    h.record(4097).unwrap();
    h.record(4098).unwrap();
    h.record(4099).unwrap();
    // 2nd bucket in size 4
    h.record(4100).unwrap();

    let iter_values = h
        .iter_linear(1)
        .map(|iv| (iv.value_iterated_to(), iv.count_since_last_iteration()))
        .collect::<Vec<(u64, u64)>>();

    // bucket size 1
    assert_eq!((0, 0), iter_values[0]);
    assert_eq!((1, 1), iter_values[1]);
    assert_eq!((2046, 0), iter_values[2046]);
    assert_eq!((2047, 1), iter_values[2047]);
    // bucket size 2
    assert_eq!((2048, 2), iter_values[2048]);
    assert_eq!((2049, 0), iter_values[2049]);
    assert_eq!((2050, 0), iter_values[2050]);
    assert_eq!((2051, 0), iter_values[2051]);
    assert_eq!((4094, 1), iter_values[4094]);
    assert_eq!((4095, 0), iter_values[4095]);
    // bucket size 4
    assert_eq!((4096, 4), iter_values[4096]);
    assert_eq!((4097, 0), iter_values[4097]);
    assert_eq!((4098, 0), iter_values[4098]);
    assert_eq!((4099, 0), iter_values[4099]);
    // also size 4, last bucket
    assert_eq!((4100, 1), iter_values[4100]);
    assert_eq!((4101, 0), iter_values[4101]);
    assert_eq!((4102, 0), iter_values[4102]);
    assert_eq!((4103, 0), iter_values[4103]);

    assert_eq!(4104, iter_values.len());
}

#[test]
fn iter_linear_visits_buckets_once_when_step_size_equals_bucket_size() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    h.record(1).unwrap();
    h.record(2047).unwrap();
    // bucket size 2
    h.record(2048).unwrap();
    h.record(2049).unwrap();
    h.record(4095).unwrap();
    // bucket size 4
    h.record(4096).unwrap();
    h.record(4097).unwrap();
    h.record(4098).unwrap();
    h.record(4099).unwrap();
    // 2nd bucket in size 4
    h.record(4100).unwrap();

    let iter_values = h
        .iter_linear(4)
        .map(|iv| (iv.value_iterated_to(), iv.count_since_last_iteration()))
        .collect::<Vec<(u64, u64)>>();

    // bucket size 1
    assert_eq!((3, 1), iter_values[0]);
    assert_eq!((2047, 1), iter_values[511]);
    // bucket size 2
    assert_eq!((2051, 2), iter_values[512]);
    assert_eq!((4095, 1), iter_values[1023]);
    // bucket size 4
    assert_eq!((4099, 4), iter_values[1024]);
    // also size 4, last bucket
    assert_eq!((4103, 1), iter_values[1025]);

    assert_eq!(1026, iter_values.len());
}

#[test]
fn iter_all_values_all_buckets() {
    let mut h = histo64(1, 8191, 3);

    h.record(1).unwrap();
    h.record(2).unwrap();
    // first in top half
    h.record(1024).unwrap();
    // first in 2nd bucket
    h.record(2048).unwrap();
    // first in 3rd
    h.record(4096).unwrap();
    // smallest value in last sub bucket of third
    h.record(8192 - 4).unwrap();

    let iter_values: Vec<(u64, u64)> = h
        .iter_all()
        .map(|v| (v.value_iterated_to(), v.count_at_value()))
        .collect();

    // 4096 distinct expressible values
    assert_eq!(2048 + 2 * 1024, iter_values.len());

    // value to expected count
    let expected = vec![
        (1, 1),
        (2, 1),
        (1024, 1),
        (2048 + 1, 1),
        (4096 + 3, 1),
        (8192 - 1, 1),
    ];

    let nonzero_count = iter_values
        .iter()
        .filter(|v| v.1 != 0)
        .map(|&v| v)
        .collect::<Vec<(u64, u64)>>();

    assert_eq!(expected, nonzero_count);
}

#[test]
fn iter_all_values_all_buckets_unit_magnitude_2() {
    let mut h = histo64(4, 16384 - 1, 3);

    h.record(4).unwrap();
    // first in top half
    h.record(4096).unwrap();
    // first in second bucket
    h.record(8192).unwrap();
    // smallest value in last sub bucket of second
    h.record(16384 - 8).unwrap();

    let iter_values: Vec<(u64, u64)> = h
        .iter_all()
        .map(|v| (v.value_iterated_to(), v.count_at_value()))
        .collect();

    // magnitude 2 means 2nd bucket is scale of 8 = 2 * 2^2
    assert_eq!(2048 + 1024, iter_values.len());

    // value to expected count
    let expected = vec![(4 + 3, 1), (4096 + 3, 1), (8192 + 7, 1), (16384 - 1, 1)];

    let nonzero_count = iter_values
        .iter()
        .filter(|v| v.1 != 0)
        .map(|&v| v)
        .collect::<Vec<(u64, u64)>>();

    assert_eq!(expected, nonzero_count);
}

#[test]
fn iter_recorded_values_all_buckets() {
    let mut h = histo64(1, 8191, 3);

    h.record(1).unwrap();
    h.record(2).unwrap();
    // first in top half
    h.record(1024).unwrap();
    // first in 2nd bucket
    h.record(2048).unwrap();
    // first in 3rd
    h.record(4096).unwrap();
    // smallest value in last sub bucket of third
    h.record(8192 - 4).unwrap();

    let iter_values: Vec<(u64, u64)> = h
        .iter_recorded()
        .map(|v| (v.value_iterated_to(), v.count_at_value()))
        .collect();

    let expected = vec![
        (1, 1),
        (2, 1),
        (1024, 1),
        (2048 + 1, 1),
        (4096 + 3, 1),
        (8192 - 1, 1),
    ];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_recorded_values_all_buckets_unit_magnitude_2() {
    let mut h = histo64(4, 16384 - 1, 3);

    h.record(4).unwrap();
    // first in top half
    h.record(4096).unwrap();
    // first in second bucket
    h.record(8192).unwrap();
    // smallest value in last sub bucket of second
    h.record(16384 - 8).unwrap();

    let iter_values: Vec<(u64, u64)> = h
        .iter_recorded()
        .map(|v| (v.value_iterated_to(), v.count_at_value()))
        .collect();

    let expected = vec![(4 + 3, 1), (4096 + 3, 1), (8192 + 7, 1), (16384 - 1, 1)];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_logarithmic_bucket_values_min_1_base_2_all_buckets() {
    let h = prepare_histo_for_logarithmic_iterator();

    let iter_values: Vec<(u64, u64, u64)> = h
        .iter_log(1, 2.0)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
            )
        })
        .collect();

    let expected = vec![
        (0, 0, 0),
        (1, 1, 1),
        (3, 1, 0),
        (7, 0, 0),
        (15, 0, 0),
        (31, 3, 1),
        (63, 0, 0),
        (127, 0, 0),
        (255, 0, 0),
        (511, 0, 0),
        (1023, 0, 0),
        (2047, 3, 0),
        (4095, 1, 1),
    ];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_logarithmic_bucket_values_min_4_base_2_all_buckets() {
    let h = prepare_histo_for_logarithmic_iterator();

    let iter_values: Vec<(u64, u64, u64)> = h
        .iter_log(4, 2.0)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
            )
        })
        .collect();

    let expected = vec![
        (3, 2, 0),
        (7, 0, 0),
        (15, 0, 0),
        (31, 3, 1),
        (63, 0, 0),
        (127, 0, 0),
        (255, 0, 0),
        (511, 0, 0),
        (1023, 0, 0),
        (2047, 3, 0),
        (4095, 1, 1),
    ];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_logarithmic_bucket_values_min_1_base_2_all_buckets_unit_magnitude_2() {
    // two buckets
    let mut h = histo64(4, 16383, 3);

    h.record(3).unwrap();
    h.record(4).unwrap();

    // inside [2^(4 + 2), 2^(5 + 2)
    h.record(70).unwrap();
    h.record(80).unwrap();
    h.record(90).unwrap();

    // in 2nd half
    h.record(5000).unwrap();
    h.record(5100).unwrap();
    h.record(5200).unwrap();

    // in last sub bucket of 2nd bucket
    h.record(16384 - 1).unwrap();

    let iter_values: Vec<(u64, u64, u64)> = h
        .iter_log(1, 2.0)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
            )
        })
        .collect();

    // first 3 iterations are just getting up to 3, which is still the '0' sub bucket.
    // All at the same index, so count_at_value stays at 1 for the first 3
    let expected = vec![
        (0, 1, 1),
        (1, 0, 1),
        (3, 0, 1),
        (7, 1, 1),
        (15, 0, 0),
        (31, 0, 0),
        (63, 0, 0),
        (127, 3, 0),
        (255, 0, 0),
        (511, 0, 0),
        (1023, 0, 0),
        (2047, 0, 0),
        (4095, 0, 0),
        (8191, 3, 0),
        (16383, 1, 1),
    ];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_logarithmic_bucket_values_min_1_base_10_all_buckets() {
    let h = prepare_histo_for_logarithmic_iterator();

    let iter_values: Vec<(u64, u64, u64)> = h
        .iter_log(1, 10.0)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
            )
        })
        .collect();

    let expected = vec![(0, 0, 0), (9, 2, 0), (99, 3, 0), (999, 0, 0), (9999, 4, 1)];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_linear_bucket_values_size_8_all_buckets() {
    // two buckets: 32 sub-buckets with scale 1, 16 with scale 2
    let mut h = histo64(1, 63, 1);

    assert_eq!(48, h.distinct_values());

    h.record(3).unwrap();
    h.record(4).unwrap();
    h.record(7).unwrap();

    // top half of first bucket
    h.record(24).unwrap();
    h.record(25).unwrap();

    h.record(61).unwrap();
    // stored in same sub bucket as last value
    h.record(62).unwrap();
    // in last sub bucket of 2nd bucket
    h.record(63).unwrap();

    let iter_values: Vec<(u64, u64, u64)> = h
        .iter_linear(8)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
            )
        })
        .collect();

    let expected = vec![
        (7, 3, 1),
        (15, 0, 0),
        (23, 0, 0),
        (31, 2, 0),
        (39, 0, 0),
        (47, 0, 0),
        (55, 0, 0),
        (63, 3, 2),
    ];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_quantiles_smorgasboard() {
    let mut h = histo64(1, 4095, 3);

    // one of each value up to 2 buckets
    for i in 0..4096 {
        h.record(i).unwrap();
    }

    let iter_values: Vec<(u64, u64, u64, f64, f64)> = h
        .iter_quantiles(2)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
                v.quantile(),
                v.quantile_iterated_to(),
            )
        })
        .collect();

    // penultimate percentile is 100.0 because 99.96% of 4095 is 4093.36, so it falls into last
    // sub bucket, thus gets to 100.0% of count.
    // this does look like it takes 2 steps and then decreases the step size
    let expected = vec![
        (0, 1, 1, 0.000244140625, 0.0),
        (1023, 1023, 1, 0.25, 0.25),
        (2047, 1024, 1, 0.5, 0.5),
        (2559, 512, 2, 0.625, 0.625),
        (3071, 512, 2, 0.75, 0.75),
        (3327, 256, 2, 0.8125, 0.8125),
        (3583, 256, 2, 0.875, 0.875),
        (3711, 128, 2, 0.90625, 0.90625),
        (3839, 128, 2, 0.9375, 0.9375),
        (3903, 64, 2, 0.953125, 0.953125),
        (3967, 64, 2, 0.96875, 0.96875),
        (3999, 32, 2, 0.9765625, 0.9765625),
        (4031, 32, 2, 0.984375, 0.984375),
        (4047, 16, 2, 0.98828125, 0.98828125),
        (4063, 16, 2, 0.9921875, 0.9921875),
        (4071, 8, 2, 0.994140625, 0.994140625),
        (4079, 8, 2, 0.99609375, 0.99609375),
        (4083, 4, 2, 0.9970703125, 0.9970703125),
        (4087, 4, 2, 0.998046875, 0.998046875),
        (4089, 2, 2, 0.99853515625, 0.99853515625),
        (4091, 2, 2, 0.9990234375, 0.9990234375),
        (4093, 2, 2, 0.99951171875, 0.999267578125),
        (4093, 0, 2, 0.99951171875, 0.99951171875),
        (4095, 2, 2, 1.0, 0.9996337890625),
        (4095, 0, 2, 1.0, 1.0),
    ];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_quantiles_iterates_to_end_skips_intermediate_at_final_value() {
    let mut h = histo64(1, 4095, 3);

    h.record_n(1, 1).unwrap();
    h.record_n(1_000, 1_000_000).unwrap();

    let iter_values: Vec<(u64, u64, u64, f64, f64)> = h
        .iter_quantiles(2)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
                v.quantile(),
                v.quantile_iterated_to(),
            )
        })
        .collect();

    // almost every nonzero quantile is in the bucket whose value is at quantile 1.0, so we should
    // iterate into that bucket (at quantile iteration 0.25), then skip to quantile iteration 1.0
    let expected = vec![
        (1, 1, 1, 0.000000999999000001, 0.0),
        (1000, 1000_000, 1000_000, 1.0, 0.25),
        (1000, 0, 1000_000, 1.0, 1.0),
    ];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_quantiles_saturated_count_before_max_value() {
    let mut h = histo64(1, 4095, 3);

    for i in 0..1000 {
        // this will quickly saturate total count as well as count since last iteration
        h.record_n(i, u64::max_value() / 100).unwrap();
    }

    let iter_values: Vec<(u64, u64, u64, f64, f64)> = h
        .iter_quantiles(2)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
                v.quantile(),
                v.quantile_iterated_to(),
            )
        })
        .collect();

    // here we do NOT skip to 1.0 because we haven't detected that we're at the max value (because
    // we aren't!).
    // This will iterate towards quantile 1.0  and will reach a point where a fraction very close to
    // 1.0 + a tiny tiny increment = the same fraction. It should then skip to 1.0.
    let expected = vec![
        (0, 184467440737095516, 184467440737095516, 0.01, 0.0),
        (24, 4427218577690292384, 184467440737095516, 0.25, 0.25),
        (49, 4611686018427387900, 184467440737095516, 0.5, 0.5),
        (62, 2398076729582241708, 184467440737095516, 0.63, 0.625),
        (74, 2213609288845146192, 184467440737095516, 0.75, 0.75),
        (81, 1291272085159668612, 184467440737095516, 0.82, 0.8125),
        (87, 1106804644422573096, 184467440737095516, 0.88, 0.875),
        (90, 553402322211286548, 184467440737095516, 0.91, 0.90625),
        (93, 553402322211286548, 184467440737095516, 0.94, 0.9375),
        (95, 368934881474191032, 184467440737095516, 0.96, 0.953125),
        (96, 184467440737095516, 184467440737095516, 0.97, 0.96875),
        (97, 184467440737095516, 184467440737095516, 0.98, 0.9765625),
        (98, 184467440737095516, 184467440737095516, 0.99, 0.984375),
        (98, 0, 184467440737095516, 0.99, 0.98828125),
        (99, 184467440737095516, 184467440737095516, 1.0, 0.9921875),
        (99, 0, 184467440737095516, 1.0, 0.994140625),
        (99, 0, 184467440737095516, 1.0, 0.99609375),
        (99, 0, 184467440737095516, 1.0, 0.9970703125),
        (99, 0, 184467440737095516, 1.0, 0.998046875),
        (99, 0, 184467440737095516, 1.0, 0.99853515625),
        (99, 0, 184467440737095516, 1.0, 0.9990234375),
        (99, 0, 184467440737095516, 1.0, 0.999267578125),
        (99, 0, 184467440737095516, 1.0, 0.99951171875),
        (99, 0, 184467440737095516, 1.0, 0.9996337890625),
        (99, 0, 184467440737095516, 1.0, 0.999755859375),
        (99, 0, 184467440737095516, 1.0, 0.99981689453125),
        (99, 0, 184467440737095516, 1.0, 0.9998779296875),
        (99, 0, 184467440737095516, 1.0, 0.999908447265625),
        (99, 0, 184467440737095516, 1.0, 0.99993896484375),
        (99, 0, 184467440737095516, 1.0, 0.9999542236328125),
        (99, 0, 184467440737095516, 1.0, 0.999969482421875),
        (99, 0, 184467440737095516, 1.0, 0.9999771118164063),
        (99, 0, 184467440737095516, 1.0, 0.9999847412109375),
        (99, 0, 184467440737095516, 1.0, 0.9999885559082031),
        (99, 0, 184467440737095516, 1.0, 0.9999923706054688),
        (99, 0, 184467440737095516, 1.0, 0.9999942779541016),
        (99, 0, 184467440737095516, 1.0, 0.9999961853027344),
        (99, 0, 184467440737095516, 1.0, 0.9999971389770508),
        (99, 0, 184467440737095516, 1.0, 0.9999980926513672),
        (99, 0, 184467440737095516, 1.0, 0.9999985694885254),
        (99, 0, 184467440737095516, 1.0, 0.9999990463256836),
        (99, 0, 184467440737095516, 1.0, 0.9999992847442627),
        (99, 0, 184467440737095516, 1.0, 0.9999995231628418),
        (99, 0, 184467440737095516, 1.0, 0.9999996423721313),
        (99, 0, 184467440737095516, 1.0, 0.9999997615814209),
        (99, 0, 184467440737095516, 1.0, 0.9999998211860657),
        (99, 0, 184467440737095516, 1.0, 0.9999998807907104),
        (99, 0, 184467440737095516, 1.0, 0.9999999105930328),
        (99, 0, 184467440737095516, 1.0, 0.9999999403953552),
        (99, 0, 184467440737095516, 1.0, 0.9999999552965164),
        (99, 0, 184467440737095516, 1.0, 0.9999999701976776),
        (99, 0, 184467440737095516, 1.0, 0.9999999776482582),
        (99, 0, 184467440737095516, 1.0, 0.9999999850988388),
        (99, 0, 184467440737095516, 1.0, 0.9999999888241291),
        (99, 0, 184467440737095516, 1.0, 0.9999999925494194),
        (99, 0, 184467440737095516, 1.0, 0.9999999944120646),
        (99, 0, 184467440737095516, 1.0, 0.9999999962747097),
        (99, 0, 184467440737095516, 1.0, 0.9999999972060323),
        (99, 0, 184467440737095516, 1.0, 0.9999999981373549),
        (99, 0, 184467440737095516, 1.0, 0.9999999986030161),
        (99, 0, 184467440737095516, 1.0, 0.9999999990686774),
        (99, 0, 184467440737095516, 1.0, 0.9999999993015081),
        (99, 0, 184467440737095516, 1.0, 0.9999999995343387),
        (99, 0, 184467440737095516, 1.0, 0.999999999650754),
        (99, 0, 184467440737095516, 1.0, 0.9999999997671694),
        (99, 0, 184467440737095516, 1.0, 0.999999999825377),
        (99, 0, 184467440737095516, 1.0, 0.9999999998835847),
        (99, 0, 184467440737095516, 1.0, 0.9999999999126885),
        (99, 0, 184467440737095516, 1.0, 0.9999999999417923),
        (99, 0, 184467440737095516, 1.0, 0.9999999999563443),
        (99, 0, 184467440737095516, 1.0, 0.9999999999708962),
        (99, 0, 184467440737095516, 1.0, 0.9999999999781721),
        (99, 0, 184467440737095516, 1.0, 0.9999999999854481),
        (99, 0, 184467440737095516, 1.0, 0.9999999999890861),
        (99, 0, 184467440737095516, 1.0, 0.999999999992724),
        (99, 0, 184467440737095516, 1.0, 0.999999999994543),
        (99, 0, 184467440737095516, 1.0, 0.999999999996362),
        (99, 0, 184467440737095516, 1.0, 0.9999999999972715),
        (99, 0, 184467440737095516, 1.0, 0.999999999998181),
        (99, 0, 184467440737095516, 1.0, 0.9999999999986358),
        (99, 0, 184467440737095516, 1.0, 0.9999999999990905),
        (99, 0, 184467440737095516, 1.0, 0.9999999999993179),
        (99, 0, 184467440737095516, 1.0, 0.9999999999995453),
        (99, 0, 184467440737095516, 1.0, 0.9999999999996589),
        (99, 0, 184467440737095516, 1.0, 0.9999999999997726),
        (99, 0, 184467440737095516, 1.0, 0.9999999999998295),
        (99, 0, 184467440737095516, 1.0, 0.9999999999998863),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999147),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999432),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999574),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999716),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999787),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999858),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999893),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999929),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999947),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999964),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999973),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999982),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999987),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999991),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999993),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999996),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999997),
        (99, 0, 184467440737095516, 1.0, 0.9999999999999998),
        // at 0.99...98, adding the resulting increment = the same 0.999...9998.
        // The increment calculations at this point involve 1 / (1 << 54), and f64
        // has 53 significand bits, so it's not too surprising that this is where
        // things get noticeably imprecise.
        (99, 0, 184467440737095516, 1.0, 1.0),
    ];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_quantiles_iterates_to_quantile_10_as_it_reaches_last_bucket() {
    let mut h = histo64(1, 4095, 3);

    // at 2 ticks per half distance, 0.9999999999999998 is the largest quantile we can get to before
    // 1.0. So, we craft a histogram that has that much before the last bucket.

    let total = 10_000_000_000_000_000_u64;
    let quantile = 0.9999999999999998_f64;
    let first_bucket = (quantile * total as f64) as u64;
    assert_eq!(9999999999999998, first_bucket);
    h.record_n(1, first_bucket).unwrap();
    // and now the leftovers to reach the total
    h.record_n(2, 2).unwrap();

    let iter_values: Vec<(u64, u64, u64, f64, f64)> = h
        .iter_quantiles(2)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
                v.quantile(),
                v.quantile_iterated_to(),
            )
        })
        .collect();

    let expected = vec![
        (1, first_bucket, first_bucket, quantile, 0.0),
        (1, 0, first_bucket, quantile, 0.25),
        (1, 0, first_bucket, quantile, 0.5),
        (1, 0, first_bucket, quantile, 0.625),
        (1, 0, first_bucket, quantile, 0.75),
        (1, 0, first_bucket, quantile, 0.8125),
        (1, 0, first_bucket, quantile, 0.875),
        (1, 0, first_bucket, quantile, 0.90625),
        (1, 0, first_bucket, quantile, 0.9375),
        (1, 0, first_bucket, quantile, 0.953125),
        (1, 0, first_bucket, quantile, 0.96875),
        (1, 0, first_bucket, quantile, 0.9765625),
        (1, 0, first_bucket, quantile, 0.984375),
        (1, 0, first_bucket, quantile, 0.98828125),
        (1, 0, first_bucket, quantile, 0.9921875),
        (1, 0, first_bucket, quantile, 0.994140625),
        (1, 0, first_bucket, quantile, 0.99609375),
        (1, 0, first_bucket, quantile, 0.9970703125),
        (1, 0, first_bucket, quantile, 0.998046875),
        (1, 0, first_bucket, quantile, 0.99853515625),
        (1, 0, first_bucket, quantile, 0.9990234375),
        (1, 0, first_bucket, quantile, 0.999267578125),
        (1, 0, first_bucket, quantile, 0.99951171875),
        (1, 0, first_bucket, quantile, 0.9996337890625),
        (1, 0, first_bucket, quantile, 0.999755859375),
        (1, 0, first_bucket, quantile, 0.99981689453125),
        (1, 0, first_bucket, quantile, 0.9998779296875),
        (1, 0, first_bucket, quantile, 0.999908447265625),
        (1, 0, first_bucket, quantile, 0.99993896484375),
        (1, 0, first_bucket, quantile, 0.9999542236328125),
        (1, 0, first_bucket, quantile, 0.999969482421875),
        (1, 0, first_bucket, quantile, 0.9999771118164063),
        (1, 0, first_bucket, quantile, 0.9999847412109375),
        (1, 0, first_bucket, quantile, 0.9999885559082031),
        (1, 0, first_bucket, quantile, 0.9999923706054688),
        (1, 0, first_bucket, quantile, 0.9999942779541016),
        (1, 0, first_bucket, quantile, 0.9999961853027344),
        (1, 0, first_bucket, quantile, 0.9999971389770508),
        (1, 0, first_bucket, quantile, 0.9999980926513672),
        (1, 0, first_bucket, quantile, 0.9999985694885254),
        (1, 0, first_bucket, quantile, 0.9999990463256836),
        (1, 0, first_bucket, quantile, 0.9999992847442627),
        (1, 0, first_bucket, quantile, 0.9999995231628418),
        (1, 0, first_bucket, quantile, 0.9999996423721313),
        (1, 0, first_bucket, quantile, 0.9999997615814209),
        (1, 0, first_bucket, quantile, 0.9999998211860657),
        (1, 0, first_bucket, quantile, 0.9999998807907104),
        (1, 0, first_bucket, quantile, 0.9999999105930328),
        (1, 0, first_bucket, quantile, 0.9999999403953552),
        (1, 0, first_bucket, quantile, 0.9999999552965164),
        (1, 0, first_bucket, quantile, 0.9999999701976776),
        (1, 0, first_bucket, quantile, 0.9999999776482582),
        (1, 0, first_bucket, quantile, 0.9999999850988388),
        (1, 0, first_bucket, quantile, 0.9999999888241291),
        (1, 0, first_bucket, quantile, 0.9999999925494194),
        (1, 0, first_bucket, quantile, 0.9999999944120646),
        (1, 0, first_bucket, quantile, 0.9999999962747097),
        (1, 0, first_bucket, quantile, 0.9999999972060323),
        (1, 0, first_bucket, quantile, 0.9999999981373549),
        (1, 0, first_bucket, quantile, 0.9999999986030161),
        (1, 0, first_bucket, quantile, 0.9999999990686774),
        (1, 0, first_bucket, quantile, 0.9999999993015081),
        (1, 0, first_bucket, quantile, 0.9999999995343387),
        (1, 0, first_bucket, quantile, 0.999999999650754),
        (1, 0, first_bucket, quantile, 0.9999999997671694),
        (1, 0, first_bucket, quantile, 0.999999999825377),
        (1, 0, first_bucket, quantile, 0.9999999998835847),
        (1, 0, first_bucket, quantile, 0.9999999999126885),
        (1, 0, first_bucket, quantile, 0.9999999999417923),
        (1, 0, first_bucket, quantile, 0.9999999999563443),
        (1, 0, first_bucket, quantile, 0.9999999999708962),
        (1, 0, first_bucket, quantile, 0.9999999999781721),
        (1, 0, first_bucket, quantile, 0.9999999999854481),
        (1, 0, first_bucket, quantile, 0.9999999999890861),
        (1, 0, first_bucket, quantile, 0.999999999992724),
        (1, 0, first_bucket, quantile, 0.999999999994543),
        (1, 0, first_bucket, quantile, 0.999999999996362),
        (1, 0, first_bucket, quantile, 0.9999999999972715),
        (1, 0, first_bucket, quantile, 0.999999999998181),
        (1, 0, first_bucket, quantile, 0.9999999999986358),
        (1, 0, first_bucket, quantile, 0.9999999999990905),
        (1, 0, first_bucket, quantile, 0.9999999999993179),
        (1, 0, first_bucket, quantile, 0.9999999999995453),
        (1, 0, first_bucket, quantile, 0.9999999999996589),
        (1, 0, first_bucket, quantile, 0.9999999999997726),
        (1, 0, first_bucket, quantile, 0.9999999999998295),
        (1, 0, first_bucket, quantile, 0.9999999999998863),
        (1, 0, first_bucket, quantile, 0.9999999999999147),
        (1, 0, first_bucket, quantile, 0.9999999999999432),
        (1, 0, first_bucket, quantile, 0.9999999999999574),
        (1, 0, first_bucket, quantile, 0.9999999999999716),
        (1, 0, first_bucket, quantile, 0.9999999999999787),
        (1, 0, first_bucket, quantile, 0.9999999999999858),
        (1, 0, first_bucket, quantile, 0.9999999999999893),
        (1, 0, first_bucket, quantile, 0.9999999999999929),
        (1, 0, first_bucket, quantile, 0.9999999999999947),
        (1, 0, first_bucket, quantile, 0.9999999999999964),
        (1, 0, first_bucket, quantile, 0.9999999999999973),
        (1, 0, first_bucket, quantile, 0.9999999999999982),
        (1, 0, first_bucket, quantile, 0.9999999999999987),
        (1, 0, first_bucket, quantile, 0.9999999999999991),
        (1, 0, first_bucket, quantile, 0.9999999999999993),
        (1, 0, first_bucket, quantile, 0.9999999999999996),
        (1, 0, first_bucket, quantile, 0.9999999999999997),
        (1, 0, first_bucket, quantile, 0.9999999999999998),
        // goes to next bucket just as it reaches 1.0
        (2, 2, 2, 1.0, 1.0),
    ];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_quantiles_one_value() {
    let mut h = histo64(1, 4095, 3);

    h.record_n(1, 1).unwrap();

    let iter_values: Vec<(u64, u64, u64, f64, f64)> = h
        .iter_quantiles(2)
        .map(|v| {
            (
                v.value_iterated_to(),
                v.count_since_last_iteration(),
                v.count_at_value(),
                v.quantile(),
                v.quantile_iterated_to(),
            )
        })
        .collect();

    // at first iteration, we're already in the last index, so we should jump to 1.0 and stop
    let expected = vec![(1, 1, 1, 1.0, 0.0), (1, 0, 1, 1.0, 1.0)];

    assert_eq!(expected, iter_values);
}

#[test]
fn iter_quantiles_empty() {
    let h = histo64(1, 4095, 3);

    assert_eq!(0, h.iter_quantiles(2).count());
}

fn prepare_histo_for_logarithmic_iterator() -> Histogram<u64> {
    // two buckets
    let mut h = histo64(1, 4095, 3);

    h.record(1).unwrap();
    h.record(2).unwrap();

    // inside [2^4, 2^5)
    h.record(20).unwrap();
    h.record(25).unwrap();
    h.record(31).unwrap();

    // in 2nd half
    h.record(1500).unwrap();
    h.record(1600).unwrap();
    h.record(1700).unwrap();

    // in last sub bucket of 2nd bucket
    h.record(4096 - 1).unwrap();

    h
}

fn histo64(
    lowest_discernible_value: u64,
    highest_trackable_value: u64,
    num_significant_digits: u8,
) -> Histogram<u64> {
    Histogram::<u64>::new_with_bounds(
        lowest_discernible_value,
        highest_trackable_value,
        num_significant_digits,
    )
    .unwrap()
}
