//! Tests from HistogramAutosizingTest.java

use hdrhistogram::Histogram;

#[test]
fn histogram_autosizing_edges() {
    let mut histogram = Histogram::<u64>::new(3).unwrap();
    histogram += (1_u64 << 62) - 1;
    assert_eq!(histogram.buckets(), 52);
    assert_eq!(histogram.distinct_values(), 54272);
    histogram += u64::max_value();
    assert_eq!(histogram.buckets(), 54);
    // unit magnitude = floor(log_2 (1 / 2)) = 0
    // sub bucket count magnitude = floor(log_2 (2 * 10^3)) = 10
    // sub bucket half count mag = 9
    // sub bucket count = 2^(sbhcm + 1) = 2^9 = 1024
    // total array size = (54 + 1) * (sub bucket count / 2) = 56320
    assert_eq!(histogram.distinct_values(), 56320);
}

#[test]
fn histogram_autosizing() {
    let mut histogram = Histogram::<u64>::new(3).unwrap();
    for i in 0..63 {
        histogram += 1_u64 << i;
    }
    assert_eq!(histogram.buckets(), 53);
    assert_eq!(histogram.distinct_values(), 55296);
}

#[test]
fn autosizing_add() {
    let mut histogram1 = Histogram::<u64>::new(2).unwrap();
    let mut histogram2 = Histogram::<u64>::new(2).unwrap();

    histogram1 += 1000_u64;
    histogram1 += 1000000000_u64;

    histogram2 += &histogram1;
    assert!(histogram2.equivalent(histogram2.max(), 1000000000_u64));
}

#[test]
fn autosizing_across_continuous_range() {
    let mut histogram = Histogram::<u64>::new(2).unwrap();

    for i in 0..10000000_u64 {
        histogram += i;
    }
}

#[test]
fn can_acess_autoresize_setting() {
    let mut histogram = Histogram::<u64>::new(2).unwrap();
    assert!(histogram.is_auto_resize());

    histogram.auto(false);
    assert!(!histogram.is_auto_resize());
}
