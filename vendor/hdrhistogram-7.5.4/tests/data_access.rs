//! Tests from HistogramDataAccessTest.java

use hdrhistogram::Histogram;

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

#[allow(dead_code)]
struct Loaded {
    hist: Histogram<u64>,
    scaled_hist: Histogram<u64>,
    raw: Histogram<u64>,
    scaled_raw: Histogram<u64>,
    post: Histogram<u64>,
    scaled_post: Histogram<u64>,
}

const TRACKABLE_MAX: u64 = 3600 * 1000 * 1000;
// Store up to 2 * 10^3 in single-unit precision. Can be 5 at most.
const SIGFIG: u8 = 3;
const EINTERVAL: u64 = 10000; /* 10 msec expected EINTERVAL */
const SCALEF: u64 = 512;

fn load_histograms() -> Loaded {
    let mut hist = Histogram::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    let mut scaled_hist = Histogram::new_with_bounds(1000, TRACKABLE_MAX * SCALEF, SIGFIG).unwrap();
    let mut raw = Histogram::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    let mut scaled_raw = Histogram::new_with_bounds(1000, TRACKABLE_MAX * SCALEF, SIGFIG).unwrap();

    // Log hypothetical scenario: 100 seconds of "perfect" 1msec results, sampled
    // 100 times per second (10,000 results), followed by a 100 second pause with a single (100
    // second) recorded result. Recording is done indicating an expected EINTERVAL between samples
    // of 10 msec:
    for _ in 0..10_000 {
        let v = 1_000; // 1ms
        hist.record_correct(v, EINTERVAL).unwrap();
        scaled_hist
            .record_correct(v * SCALEF, EINTERVAL * SCALEF)
            .unwrap();
        raw += v;
        scaled_raw += v * SCALEF;
    }

    let v = 100_000_000;
    hist.record_correct(v, EINTERVAL).unwrap();
    scaled_hist
        .record_correct(v * SCALEF, EINTERVAL * SCALEF)
        .unwrap();
    raw += v;
    scaled_raw += v * SCALEF;

    let post = raw.clone_correct(EINTERVAL);
    let scaled_post = scaled_raw.clone_correct(EINTERVAL * SCALEF);

    Loaded {
        hist,
        scaled_hist,
        raw,
        scaled_raw,
        post,
        scaled_post,
    }
}

#[test]
fn scaling_equivalence() {
    let Loaded {
        hist,
        scaled_hist,
        post,
        scaled_post,
        ..
    } = load_histograms();

    assert_near!(hist.mean() * SCALEF as f64, scaled_hist.mean(), 0.000001);
    assert_eq!(hist.len(), scaled_hist.len());

    let expected_99th = hist.value_at_quantile(0.99) * 512;
    let scaled_99th = scaled_hist.value_at_quantile(0.99);

    assert_eq!(
        hist.lowest_equivalent(expected_99th),
        scaled_hist.lowest_equivalent(scaled_99th)
    );

    // averages should be equivalent
    assert_near!(hist.mean() * SCALEF as f64, scaled_hist.mean(), 0.000001);
    // total count should be the same
    assert_eq!(hist.len(), scaled_hist.len());
    // 99%'iles should be equivalent
    assert_eq!(
        scaled_hist.highest_equivalent(hist.value_at_quantile(0.99) * 512),
        scaled_hist.highest_equivalent(scaled_hist.value_at_quantile(0.99))
    );
    // Max should be equivalent
    assert_eq!(
        scaled_hist.highest_equivalent(hist.max() * 512),
        scaled_hist.max()
    );

    // Same for post-corrected:

    // averages should be equivalent
    assert_near!(post.mean() * SCALEF as f64, scaled_post.mean(), 0.000001);
    // total count should be the same
    assert_eq!(post.len(), scaled_post.len());
    // 99%'iles should be equivalent
    assert_eq!(
        post.lowest_equivalent(post.value_at_quantile(0.99)) * SCALEF,
        scaled_post.lowest_equivalent(scaled_post.value_at_quantile(0.99))
    );
    // Max should be equivalent
    assert_eq!(
        scaled_post.highest_equivalent(post.max() * 512),
        scaled_post.max()
    );
}

#[test]
fn total_count() {
    let Loaded { hist, raw, .. } = load_histograms();

    assert_eq!(raw.len(), 10001);
    assert_eq!(hist.len(), 20000);
}

#[test]
fn get_max_value() {
    let Loaded { hist, .. } = load_histograms();

    assert!(hist.equivalent(hist.max(), 100000000));
}

#[test]
fn get_min_value() {
    let Loaded { hist, .. } = load_histograms();

    assert!(hist.equivalent(hist.min(), 1000));
}

#[test]
fn get_mean() {
    let Loaded { hist, raw, .. } = load_histograms();

    // direct avg. of raw results
    let expected_raw_mean = ((10000.0 * 1000.0) + (1.0 * 100000000.0)) / 10001.0;
    // avg. 1 msec for half the time, and 50 sec for other half
    let expected_mean = (1000.0 + 50000000.0) / 2.0;

    // We expect to see the mean to be accurate to ~3 decimal points (~0.1%):
    assert_near!(raw.mean(), expected_raw_mean, 0.001);
    assert_near!(hist.mean(), expected_mean, 0.001);
}

#[test]
fn get_stdev() {
    let Loaded { hist, raw, .. } = load_histograms();

    // direct avg. of raw results
    let expected_raw_mean: f64 = ((10000.0 * 1000.0) + (1.0 * 100000000.0)) / 10001.0;
    let expected_raw_std_dev = (((10000.0 * (1000_f64 - expected_raw_mean).powi(2))
        + (100000000_f64 - expected_raw_mean).powi(2))
        / 10001.0)
        .sqrt();

    // avg. 1 msec for half the time, and 50 sec for other half
    let expected_mean = (1000.0 + 50000000.0) / 2_f64;
    let mut expected_square_deviation_sum = 10000.0 * (1000_f64 - expected_mean).powi(2);

    let mut value = 10000_f64;
    while value <= 100000000.0 {
        expected_square_deviation_sum += (value - expected_mean).powi(2);
        value += 10000.0;
    }
    let expected_std_dev = (expected_square_deviation_sum / 20000.0).sqrt();

    // We expect to see the standard deviations to be accurate to ~3 decimal points (~0.1%):
    assert_near!(raw.stdev(), expected_raw_std_dev, 0.001);
    assert_near!(hist.stdev(), expected_std_dev, 0.001);
}

#[test]
fn quantiles() {
    let Loaded { hist, raw, .. } = load_histograms();

    assert_near!(raw.value_at_quantile(0.3), 1000.0, 0.001);
    assert_near!(raw.value_at_quantile(0.99), 1000.0, 0.001);
    assert_near!(raw.value_at_quantile(0.9999), 1000.0, 0.001);
    assert_near!(raw.value_at_quantile(0.99999), 100000000.0, 0.001);
    assert_near!(raw.value_at_quantile(1.0), 100000000.0, 0.001);

    assert_near!(hist.value_at_quantile(0.3), 1000.0, 0.001);
    assert_near!(hist.value_at_quantile(0.5), 1000.0, 0.001);
    assert_near!(hist.value_at_quantile(0.75), 50000000.0, 0.001);
    assert_near!(hist.value_at_quantile(0.9), 80000000.0, 0.001);
    assert_near!(hist.value_at_quantile(0.99), 98000000.0, 0.001);
    assert_near!(hist.value_at_quantile(0.99999), 100000000.0, 0.001);
    assert_near!(hist.value_at_quantile(1.0), 100000000.0, 0.001);
}

#[test]
fn large_quantile() {
    let largest_value = 1000000000000_u64;
    let mut h = Histogram::<u64>::new_with_max(largest_value, 5).unwrap();
    h += largest_value;
    assert!(h.value_at_quantile(1.0) > 0);
}

#[test]
fn quantile_atorbelow() {
    let Loaded { hist, raw, .. } = load_histograms();
    assert_near!(0.9999, raw.quantile_below(5000), 0.0001);
    assert_near!(0.5, hist.quantile_below(5000), 0.0001);
    assert_near!(1.0, hist.quantile_below(100000000_u64), 0.0001);
}

#[test]
fn quantile_below_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    for i in 0..1024 {
        h.record_n(i, u64::max_value() - 1).unwrap();
    }

    // really it should be 0.5 but it saturates at u64::max_value()
    assert_eq!(1.0, h.quantile_below(512));
}

#[test]
fn quantile_below_value_beyond_max() {
    let mut h = Histogram::<u64>::new_with_bounds(1, 100_000, 3).unwrap();

    for i in 0..1024 {
        h.record(i).unwrap();
    }

    // also a bunch at maximum value, should be included in the resulting quantile
    for _ in 0..1024 {
        h.record(100_000).unwrap();
    }

    assert_eq!(1.0, h.quantile_below(u64::max_value()));
}

#[test]
fn count_between() {
    let Loaded { hist, raw, .. } = load_histograms();
    assert_eq!(raw.count_between(1000, 1000), 10000);
    assert_eq!(raw.count_between(5000, 150000000), 1);
    assert_eq!(hist.count_between(5000, 150000000), 10000);
}

#[test]
fn count_between_high_beyond_max() {
    let mut h = Histogram::<u64>::new_with_bounds(1, 100_000, 3).unwrap();
    // largest expressible value will land in last index
    h.record((1 << 17) - 1).unwrap();

    assert_eq!(1, h.count_between(50, 300_000));
}

#[test]
fn count_between_low_and_high_beyond_max() {
    let mut h = Histogram::<u64>::new_with_bounds(1, 100_000, 3).unwrap();
    // largest expressible value will land in last index
    h.record((1 << 17) - 1).unwrap();

    assert_eq!(1, h.count_between(200_000, 300_000));
}

#[test]
fn count_between_saturates() {
    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    for i in 0..1024 {
        h.record_n(i, u64::max_value() - 1).unwrap();
    }

    assert_eq!(u64::max_value(), h.count_between(100, 200));
}

#[test]
fn count_at() {
    let Loaded { hist, raw, .. } = load_histograms();
    assert_eq!(raw.count_between(10000, 10010), 0);
    assert_eq!(hist.count_between(10000, 10010), 1);
    assert_eq!(raw.count_at(1000), 10000);
    assert_eq!(hist.count_at(1000), 10000);
}

#[test]
fn count_at_beyond_max_value() {
    let mut h = Histogram::<u64>::new_with_bounds(1, 100_000, 3).unwrap();
    // largest expressible value will land in last index
    h.record((1 << 17) - 1).unwrap();

    assert_eq!(1, h.count_at(u64::max_value()));
}

#[test]
fn quantile_iter() {
    let Loaded { hist, .. } = load_histograms();
    for v in hist.iter_quantiles(5 /* ticks per half */) {
        assert_eq!(
            v.value_iterated_to(),
            hist.highest_equivalent(hist.value_at_quantile(v.quantile()))
        );
    }
}

#[test]
fn linear_iter_raw() {
    let Loaded { raw, .. } = load_histograms();

    // Note that using linear buckets should work "as expected" as long as the number of linear
    // buckets is lower than the resolution level determined by
    // largest_value_with_single_unit_resolution (2000 in this case). Above that count, some of the
    // linear buckets can end up rounded up in size (to the nearest local resolution unit level),
    // which can result in a smaller number of buckets that expected covering the range.

    // Iterate raw data using linear buckets of 100 msec each.
    let mut num = 0;
    for (i, v) in raw.iter_linear(100_000).enumerate() {
        match i {
            // Raw Linear 100 msec bucket # 0 added a count of 10000
            0 => assert_eq!(v.count_since_last_iteration(), 10_000),
            // Raw Linear 100 msec bucket # 999 added a count of 1
            999 => assert_eq!(v.count_since_last_iteration(), 1),
            // Remaining raw Linear 100 msec buckets add a count of 0
            _ => assert_eq!(v.count_since_last_iteration(), 0),
        }
        num += 1;
    }
    assert_eq!(num, 1_000);
}

#[test]
fn linear_iter_corrected() {
    let Loaded { hist, .. } = load_histograms();

    let mut num = 0;
    let mut total_added_counts = 0;
    // Iterate data using linear buckets of 10 msec each.
    for (i, v) in hist.iter_linear(10_000).enumerate() {
        if i == 0 {
            assert_eq!(v.count_since_last_iteration(), 10_000);
        }

        // Because value resolution is low enough (3 digits) that multiple linear buckets will end
        // up residing in a single value-equivalent range, some linear buckets will have counts of
        // 2 or more, and some will have 0 (when the first bucket in the equivalent range was the
        // one that got the total count bump). However, we can still verify the sum of counts added
        // in all the buckets...
        total_added_counts += v.count_since_last_iteration();
        num += 1;
    }
    // There should be 10000 linear buckets of size 10000 usec between 0 and 100 sec.
    assert_eq!(num, 10_000);
    assert_eq!(total_added_counts, 20_000);

    num = 0;
    total_added_counts = 0;
    // Iterate data using linear buckets of 1 msec each.
    for (i, v) in hist.iter_linear(1_000).enumerate() {
        if i == 1 {
            assert_eq!(v.count_since_last_iteration(), 10_000);
        }

        // Because value resolution is low enough (3 digits) that multiple linear buckets will end
        // up residing in a single value-equivalent range, some linear buckets will have counts of
        // 2 or more, and some will have 0 (when the first bucket in the equivalent range was the
        // one that got the total count bump). However, we can still verify the sum of counts added
        // in all the buckets...
        total_added_counts += v.count_since_last_iteration();
        num += 1
    }

    // You may ask "why 100007 and not 100000?" for the value below? The answer is that at this
    // fine a linear stepping resolution, the final populated sub-bucket (at 100 seconds with 3
    // decimal point resolution) is larger than our liner stepping, and holds more than one linear
    // 1 msec step in it.
    //
    // Since we only know we're done with linear iteration when the next iteration step will step
    // out of the last populated bucket, there is not way to tell if the iteration should stop at
    // 100000 or 100007 steps. The proper thing to do is to run to the end of the sub-bucket
    // quanta...
    assert_eq!(num, 100_007);
    assert_eq!(total_added_counts, 20_000);
}

#[test]
fn iter_log() {
    let Loaded { hist, raw, .. } = load_histograms();

    // Iterate raw data using logarithmic buckets starting at 10 msec.
    let mut num = 0;
    for (i, v) in raw.iter_log(10000, 2.0).enumerate() {
        match i {
            // Raw logarithmic 10 msec bucket # 0 added a count of 10000
            0 => assert_eq!(v.count_since_last_iteration(), 10000),
            // Raw logarithmic 10 msec bucket # 14 added a count of 1
            14 => assert_eq!(v.count_since_last_iteration(), 1),
            // Remaining raw logarithmic 100 msec buckets add a count of 0
            _ => assert_eq!(v.count_since_last_iteration(), 0),
        }
        num += 1;
    }
    assert_eq!(num - 1, 14);

    num = 0;
    let mut total_added_counts = 0;
    for (i, v) in hist.iter_log(10000, 2.0).enumerate() {
        if i == 0 {
            assert_eq!(v.count_since_last_iteration(), 10000);
        }
        total_added_counts += v.count_since_last_iteration();
        num += 1;
    }
    // There should be 14 Logarithmic buckets of size 10000 usec between 0 and 100 sec.
    assert_eq!(num - 1, 14);
    assert_eq!(total_added_counts, 20000);
}

#[test]
fn iter_recorded() {
    let Loaded { hist, raw, .. } = load_histograms();

    // Iterate raw data by stepping through every value that has a count recorded:
    let mut num = 0;
    for (i, v) in raw.iter_recorded().enumerate() {
        match i {
            // Raw recorded value bucket # 0 added a count of 10000
            0 => assert_eq!(v.count_since_last_iteration(), 10000),
            // Remaining recorded value buckets add a count of 1
            _ => assert_eq!(v.count_since_last_iteration(), 1),
        }
        num += 1;
    }
    assert_eq!(num, 2);

    num = 0;
    let mut total_added_counts = 0;
    for (i, v) in hist.iter_recorded().enumerate() {
        if i == 0 {
            assert_eq!(v.count_since_last_iteration(), 10000);
        }

        // The count in a recorded iterator value should never be zero
        assert_ne!(v.count_at_value(), 0);
        // The count in a recorded iterator value should exactly match the amount added since the
        // last iteration
        assert_eq!(v.count_at_value(), v.count_since_last_iteration());

        total_added_counts += v.count_since_last_iteration();
        num += 1;
    }
    assert_eq!(total_added_counts, 20000);
}

#[test]
fn iter_all() {
    let Loaded { hist, raw, .. } = load_histograms();

    // Iterate raw data by stepping through every value that has a count recorded:
    let mut num = 0;
    for (i, v) in raw.iter_all().enumerate() {
        if i == 1000 {
            assert_eq!(v.count_since_last_iteration(), 10000);
        } else if hist.equivalent(v.value_iterated_to(), 100000000) {
            assert_eq!(v.count_since_last_iteration(), 1);
        } else {
            assert_eq!(v.count_since_last_iteration(), 0);
        }

        // TODO: also test total count and total value once the iterator exposes this
        num += 1;
    }
    assert_eq!(num, hist.distinct_values());

    num = 0;
    let mut total_added_counts = 0;
    // HistogramIterationValue v1 = null;
    for (i, v) in hist.iter_all().enumerate() {
        // v1 = v;
        if i == 1000 {
            assert_eq!(v.count_since_last_iteration(), 10000);
        }

        // The count in iter_all buckets should exactly match the amount added since the last
        // iteration
        assert_eq!(v.count_at_value(), v.count_since_last_iteration());
        total_added_counts += v.count_since_last_iteration();
        num += 1;
    }
    assert_eq!(num, hist.distinct_values());
    assert_eq!(total_added_counts, 20000);
}

#[test]
fn linear_iter_steps() {
    let mut histogram = Histogram::<u64>::new(2).unwrap();
    histogram += 193;
    histogram += 0;
    histogram += 1;
    histogram += 64;
    histogram += 128;
    assert_eq!(histogram.iter_linear(64).count(), 4);
}

#[test]
fn value_duplication() {
    let Loaded { hist, .. } = load_histograms();
    let histogram1 = hist.clone();

    let mut num = 0;
    let mut ranges = Vec::with_capacity(histogram1.distinct_values());
    let mut counts = Vec::with_capacity(histogram1.distinct_values());
    for v in histogram1.iter_all() {
        if v.count_since_last_iteration() > 0 {
            ranges.push(v.value_iterated_to());
            counts.push(v.count_since_last_iteration());
        }
        num += 1;
    }
    assert_eq!(num, histogram1.distinct_values());

    let mut histogram2 = Histogram::new_with_max(TRACKABLE_MAX, SIGFIG).unwrap();
    for i in 0..ranges.len() {
        histogram2.record_n(ranges[i], counts[i]).unwrap();
    }

    assert_eq!(
        histogram1, histogram2,
        "histograms should be equal after re-recording"
    );
}

#[test]
fn total_count_exceeds_bucket_type() {
    let mut h: Histogram<u8> = Histogram::new(3).unwrap();

    for _ in 0..200 {
        h.record(100).unwrap();
    }

    for _ in 0..200 {
        h.record(100_000).unwrap();
    }

    assert_eq!(400, h.len());
}
