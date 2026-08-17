use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::time::Duration as StdDuration;

use rstest::rstest;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::{error, Duration};

#[rstest]
#[case(Duration::ZERO, 0.seconds())]
#[case(Duration::NANOSECOND, 1.nanoseconds())]
#[case(Duration::MICROSECOND, 1.microseconds())]
#[case(Duration::MILLISECOND, 1.milliseconds())]
#[case(Duration::SECOND, 1.seconds())]
#[case(Duration::MINUTE, 60.seconds())]
#[case(Duration::HOUR, 3_600.seconds())]
#[case(Duration::DAY, 86_400.seconds())]
#[case(Duration::WEEK, 604_800.seconds())]
fn unit_values(#[case] input: Duration, #[case] expected: Duration) {
    assert_eq!(input, expected);
}

#[rstest]
fn default() {
    assert_eq!(Duration::default(), Duration::ZERO);
}

#[rstest]
#[case((-1).seconds(), false)]
#[case(0.seconds(), true)]
#[case(1.seconds(), false)]
fn is_zero(#[case] input: Duration, #[case] expected: bool) {
    assert_eq!(input.is_zero(), expected);
}

#[rstest]
#[case((-1).seconds(), true)]
#[case(0.seconds(), false)]
#[case(1.seconds(), false)]
fn is_negative(#[case] input: Duration, #[case] expected: bool) {
    assert_eq!(input.is_negative(), expected);
}

#[rstest]
#[case((-1).seconds(), false)]
#[case(0.seconds(), false)]
#[case(1.seconds(), true)]
fn is_positive(#[case] input: Duration, #[case] expected: bool) {
    assert_eq!(input.is_positive(), expected);
}

#[rstest]
#[case(1.seconds(), 1.seconds())]
#[case(0.seconds(), 0.seconds())]
#[case((-1).seconds(), 1.seconds())]
#[case(Duration::new(i64::MIN, 0), Duration::MAX)]
fn abs(#[case] input: Duration, #[case] expected: Duration) {
    assert_eq!(input.abs(), expected);
}

#[rstest]
#[case(1.seconds(), 1.std_seconds())]
#[case(0.seconds(), 0.std_seconds())]
#[case((-1).seconds(), 1.std_seconds())]
fn unsigned_abs(#[case] input: Duration, #[case] expected: StdDuration) {
    assert_eq!(input.unsigned_abs(), expected);
}

#[rstest]
#[case(1, 0, 1.seconds())]
#[case(-1, 0, (-1).seconds())]
#[case(1, 2_000_000_000, 3.seconds())]
#[case(0, 0, 0.seconds())]
#[case(0, 1_000_000_000, 1.seconds())]
#[case(-1, 1_000_000_000, 0.seconds())]
#[case(-2, 1_000_000_000, (-1).seconds())]
#[case(1, -1, 999_999_999.nanoseconds())]
#[case(-1, 1, (-999_999_999).nanoseconds())]
#[case(1, 1, 1_000_000_001.nanoseconds())]
#[case(-1, -1, (-1_000_000_001).nanoseconds())]
#[case(0, 1, 1.nanoseconds())]
#[case(0, -1, (-1).nanoseconds())]
#[case(-1, 1_400_000_000, 400.milliseconds())]
#[case(-2, 1_400_000_000, (-600).milliseconds())]
#[case(-3, 1_400_000_000, (-1_600).milliseconds())]
#[case(1, -1_400_000_000, (-400).milliseconds())]
#[case(2, -1_400_000_000, 600.milliseconds())]
#[case(3, -1_400_000_000, 1_600.milliseconds())]
fn new(#[case] secs: i64, #[case] nanos: i32, #[case] expected: Duration) {
    assert_eq!(Duration::new(secs, nanos), expected);
}

#[rstest]
#[case(i64::MAX, 1_000_000_000)]
#[case(i64::MIN, -1_000_000_000)]
#[should_panic]
fn new_panic(#[case] secs: i64, #[case] nanos: i32) {
    let _ = Duration::new(secs, nanos);
}

#[rstest]
#[case(1, 604_800)]
#[case(2, 2 * 604_800)]
#[case(-1, -604_800)]
#[case(-2, -2 * 604_800)]
fn weeks(#[case] weeks_: i64, #[case] expected: i64) {
    assert_eq!(Duration::weeks(weeks_), expected.seconds());
}

#[rstest]
#[case(i64::MAX)]
#[case(i64::MIN)]
#[should_panic]
fn weeks_panic(#[case] weeks: i64) {
    let _ = Duration::weeks(weeks);
}

#[rstest]
#[case(7, 1)]
#[case(-7, -1)]
#[case(6, 0)]
#[case(-6, 0)]
fn whole_weeks(#[case] days: i64, #[case] expected: i64) {
    assert_eq!(Duration::days(days).whole_weeks(), expected);
}

#[rstest]
#[case(1, 86_400)]
#[case(2, 2 * 86_400)]
#[case(-1, -86_400)]
#[case(-2, -2 * 86_400)]
fn days(#[case] days_: i64, #[case] expected: i64) {
    assert_eq!(Duration::days(days_), expected.seconds());
}

#[rstest]
#[case(i64::MAX)]
#[case(i64::MIN)]
#[should_panic]
fn days_panic(#[case] days: i64) {
    let _ = Duration::days(days);
}

#[rstest]
#[case(24, 1)]
#[case(-24, -1)]
#[case(23, 0)]
#[case(-23, 0)]
fn whole_days(#[case] hours: i64, #[case] expected: i64) {
    assert_eq!(Duration::hours(hours).whole_days(), expected);
}

#[rstest]
#[case(1, 3_600)]
#[case(2, 2 * 3_600)]
#[case(-1, -3_600)]
#[case(-2, -2 * 3_600)]
fn hours(#[case] hours_: i64, #[case] expected: i64) {
    assert_eq!(Duration::hours(hours_), expected.seconds());
}

#[rstest]
#[case(i64::MAX)]
#[case(i64::MIN)]
#[should_panic]
fn hours_panic(#[case] hours: i64) {
    let _ = Duration::hours(hours);
}

#[rstest]
#[case(60, 1)]
#[case(-60, -1)]
#[case(59, 0)]
#[case(-59, 0)]
fn whole_hours(#[case] minutes: i64, #[case] expected: i64) {
    assert_eq!(Duration::minutes(minutes).whole_hours(), expected);
}

#[rstest]
#[case(1, 60)]
#[case(2, 2 * 60)]
#[case(-1, -60)]
#[case(-2, -2 * 60)]
fn minutes(#[case] minutes_: i64, #[case] expected: i64) {
    assert_eq!(Duration::minutes(minutes_), expected.seconds());
}

#[rstest]
#[case(i64::MAX)]
#[case(i64::MIN)]
#[should_panic]
fn minutes_panic(#[case] minutes: i64) {
    let _ = Duration::minutes(minutes);
}

#[rstest]
#[case(60, 1)]
#[case(-60, -1)]
#[case(59, 0)]
#[case(-59, 0)]
fn whole_minutes(#[case] seconds: i64, #[case] expected: i64) {
    assert_eq!(Duration::seconds(seconds).whole_minutes(), expected);
}

#[rstest]
#[case(1, 1_000)]
#[case(2, 2 * 1_000)]
#[case(-1, -1_000)]
#[case(-2, -2 * 1_000)]
fn seconds(#[case] seconds_: i64, #[case] expected: i64) {
    assert_eq!(Duration::seconds(seconds_), expected.milliseconds());
}

#[rstest]
#[case(1)]
#[case(-1)]
#[case(60)]
#[case(-60)]
fn whole_seconds(#[case] seconds: i64) {
    assert_eq!(Duration::seconds(seconds).whole_seconds(), seconds);
}

#[rstest]
#[case(0.5, Duration::milliseconds(500))]
#[case(-0.5, Duration::milliseconds(-500))]
#[case(123.250, Duration::milliseconds(123_250))]
#[case(0.000_000_000_012, Duration::ZERO)]
fn seconds_f64(#[case] seconds: f64, #[case] expected: Duration) {
    assert_eq!(Duration::seconds_f64(seconds), expected);
}

#[rstest]
#[case(f64::MAX)]
#[case(f64::MIN)]
#[case(f64::NAN)]
#[should_panic]
fn seconds_f64_panic(#[case] seconds: f64) {
    let _ = Duration::seconds_f64(seconds);
}

#[rstest]
#[case(0.5, Duration::milliseconds(500))]
#[case(-0.5, Duration::milliseconds(-500))]
#[case(123.250, Duration::milliseconds(123_250))]
#[case(0.000_000_000_012, Duration::ZERO)]
#[case(f64::MAX, Duration::MAX)]
#[case(f64::MIN, Duration::MIN)]
#[case(f64::NAN, Duration::ZERO)]
fn saturating_seconds_f64(#[case] seconds: f64, #[case] expected: Duration) {
    assert_eq!(Duration::saturating_seconds_f64(seconds), expected);
}

#[rstest]
#[case(0.5, 0.5)]
#[case(-0.5, -0.5)]
#[case(123.250, 123.250)]
#[case(0.000_000_000_012, 0.)]
fn checked_seconds_f64_success(#[case] seconds: f64, #[case] expected: f64) {
    assert_eq!(
        Duration::checked_seconds_f64(seconds),
        Some(expected.seconds())
    );
}

#[rstest]
#[case(f64::MAX)]
#[case(f64::MIN)]
#[case(f64::NAN)]
fn checked_seconds_f64_edge_cases(#[case] seconds: f64) {
    assert_eq!(Duration::checked_seconds_f64(seconds), None);
}

#[rstest]
#[case(1.)]
#[case(-1.)]
#[case(60.)]
#[case(-60.)]
#[case(1.5)]
#[case(-1.5)]
fn as_seconds_f64(#[case] seconds: f64) {
    assert_eq!(Duration::seconds_f64(seconds).as_seconds_f64(), seconds);
}

#[rstest]
#[case(0.5, Duration::milliseconds(500))]
#[case(-0.5, Duration::milliseconds(-500))]
#[case(123.250, Duration::milliseconds(123_250))]
#[case(0.000_000_000_012, Duration::ZERO)]
fn seconds_f32_success(#[case] seconds: f32, #[case] expected: Duration) {
    assert_eq!(Duration::seconds_f32(seconds), expected);
}

#[rstest]
#[case(f32::MAX)]
#[case(f32::MIN)]
#[case(f32::NAN)]
#[should_panic]
fn seconds_f32_panic(#[case] seconds: f32) {
    let _ = Duration::seconds_f32(seconds);
}

#[rstest]
#[case(0.5, Duration::milliseconds(500))]
#[case(-0.5, Duration::milliseconds(-500))]
#[case(123.250, Duration::milliseconds(123_250))]
#[case(0.000_000_000_012, Duration::ZERO)]
#[case(f32::MAX, Duration::MAX)]
#[case(f32::MIN, Duration::MIN)]
#[case(f32::NAN, Duration::ZERO)]
fn saturating_seconds_f32(#[case] seconds: f32, #[case] expected: Duration) {
    assert_eq!(Duration::saturating_seconds_f32(seconds), expected);
}

#[rstest]
#[case(0.5, 0.5)]
#[case(-0.5, -0.5)]
#[case(123.250, 123.250)]
#[case(0.000_000_000_012, 0.0)]
fn checked_seconds_f32_success(#[case] seconds: f32, #[case] expected: f64) {
    assert_eq!(
        Duration::checked_seconds_f32(seconds),
        Some(expected.seconds())
    );
}

#[rstest]
#[case(f32::MAX)]
#[case(f32::MIN)]
#[case(f32::NAN)]
fn checked_seconds_f32_none(#[case] seconds: f32) {
    assert_eq!(Duration::checked_seconds_f32(seconds), None);
}

#[rstest]
#[case(1.0, 1.0)]
#[case(-1.0, -1.0)]
#[case(60.0, 60.0)]
#[case(-60.0, -60.0)]
#[case(1.5, 1.5)]
#[case(-1.5, -1.5)]
fn as_seconds_f32(#[case] seconds: f32, #[case] expected: f32) {
    assert_eq!(Duration::seconds_f32(seconds).as_seconds_f32(), expected);
}

#[rstest]
#[case(1, 1_000)]
#[case(-1, -1_000)]
fn milliseconds(#[case] input: i64, #[case] expected: i64) {
    assert_eq!(Duration::milliseconds(input), expected.microseconds());
}

#[rstest]
#[case(1.seconds(), 1_000)]
#[case((-1).seconds(), -1_000)]
#[case(1.milliseconds(), 1)]
#[case((-1).milliseconds(), -1)]
fn whole_milliseconds(#[case] input: Duration, #[case] expected: i128) {
    assert_eq!(input.whole_milliseconds(), expected);
}

#[rstest]
#[case(1.4.seconds(), 400)]
#[case((-1.4).seconds(), -400)]
fn subsec_milliseconds(#[case] duration: Duration, #[case] expected: i16) {
    assert_eq!(duration.subsec_milliseconds(), expected);
}

#[rstest]
#[case(1, 1_000)]
#[case(-1, -1_000)]
fn microseconds(#[case] input: i64, #[case] expected: i64) {
    assert_eq!(Duration::microseconds(input), expected.nanoseconds());
}

#[rstest]
#[case(1.milliseconds(), 1_000)]
#[case((-1).milliseconds(), -1_000)]
#[case(1.microseconds(), 1)]
#[case((-1).microseconds(), -1)]
fn whole_microseconds(#[case] input: Duration, #[case] expected: i128) {
    assert_eq!(input.whole_microseconds(), expected);
}

#[rstest]
#[case(1.0004.seconds(), 400)]
#[case((-1.0004).seconds(), -400)]
fn subsec_microseconds(#[case] duration: Duration, #[case] expected: i32) {
    assert_eq!(duration.subsec_microseconds(), expected);
}

#[rstest]
#[case(1, 1.microseconds() / 1_000)]
#[case(-1, (-1).microseconds() / 1_000)]
fn nanoseconds(#[case] input: i64, #[case] expected: Duration) {
    assert_eq!(Duration::nanoseconds(input), expected);
}

#[rstest]
#[case(1.microseconds(), 1_000)]
#[case((-1).microseconds(), -1_000)]
#[case(1.nanoseconds(), 1)]
#[case((-1).nanoseconds(), -1)]
fn whole_nanoseconds(#[case] input: Duration, #[case] expected: i128) {
    assert_eq!(input.whole_nanoseconds(), expected);
}

#[rstest]
#[case(1.000_000_4.seconds(), 400)]
#[case((-1.000_000_4).seconds(), -400)]
fn subsec_nanoseconds(#[case] duration: Duration, #[case] expected: i32) {
    assert_eq!(duration.subsec_nanoseconds(), expected);
}

#[rstest]
#[case(5.seconds(), 5.seconds(), 10.seconds())]
#[case((-5).seconds(), 5.seconds(), 0.seconds())]
#[case(1.seconds(), (-1).milliseconds(), 999.milliseconds())]
fn checked_add_some(#[case] a: Duration, #[case] b: Duration, #[case] expected: Duration) {
    assert_eq!(a.checked_add(b), Some(expected));
}

#[rstest]
#[case(Duration::MAX, 1.nanoseconds())]
#[case(5.seconds(), Duration::MAX)]
#[case(Duration::MIN, Duration::MIN)]
fn checked_add_none(#[case] a: Duration, #[case] b: Duration) {
    assert_eq!(a.checked_add(b), None);
}

#[rstest]
#[case(5.seconds(), 5.seconds(), 0.seconds())]
#[case(5.seconds(), 10.seconds(), (-5).seconds())]
fn checked_sub_some(#[case] a: Duration, #[case] b: Duration, #[case] expected: Duration) {
    assert_eq!(a.checked_sub(b), Some(expected));
}

#[rstest]
#[case(Duration::MIN, 1.nanoseconds())]
#[case(5.seconds(), Duration::MIN)]
#[case(Duration::MAX, Duration::MIN)]
fn checked_sub_none(#[case] a: Duration, #[case] b: Duration) {
    assert_eq!(a.checked_sub(b), None);
}

#[rstest]
#[case(5.seconds(), 2, 10.seconds())]
#[case(5.seconds(), -2, (-10).seconds())]
#[case(5.seconds(), 0, Duration::ZERO)]
fn checked_mul_some(#[case] duration: Duration, #[case] rhs: i32, #[case] expected: Duration) {
    assert_eq!(duration.checked_mul(rhs), Some(expected));
}

#[rstest]
#[case(Duration::MIN, -1)]
#[case(Duration::MAX, 2)]
#[case(Duration::MIN, 2)]
fn checked_mul_none(#[case] duration: Duration, #[case] rhs: i32) {
    assert_eq!(duration.checked_mul(rhs), None);
}

#[rstest]
#[case(10.seconds(), 2, 5.seconds())]
#[case(10.seconds(), -2, (-5).seconds())]
fn checked_div_some(#[case] duration: Duration, #[case] rhs: i32, #[case] expected: Duration) {
    assert_eq!(duration.checked_div(rhs), Some(expected));
}

#[rstest]
#[case(1.seconds(), 0)]
#[case(Duration::MIN, -1)]
fn checked_div_none(#[case] duration: Duration, #[case] rhs: i32) {
    assert_eq!(duration.checked_div(rhs), None);
}

#[rstest]
fn checked_div_regression() {
    assert_eq!(
        Duration::new(1, 1).checked_div(7),
        Some(Duration::new(0, 142_857_143)) // manually verified
    );
}

#[rstest]
#[case(5.seconds(), Some((-5).seconds()))]
#[case((-5).seconds(), Some(5.seconds()))]
#[case(Duration::MIN, None)]
fn checked_neg(#[case] duration: Duration, #[case] expected: Option<Duration>) {
    assert_eq!(duration.checked_neg(), expected);
}

#[rstest]
#[case(5.seconds(), 5.seconds(), 10.seconds())]
#[case(Duration::MAX, 1.nanoseconds(), Duration::MAX)]
#[case(Duration::MAX, 1.seconds(), Duration::MAX)]
#[case(Duration::MIN, (-1).nanoseconds(), Duration::MIN)]
#[case(Duration::MIN, (-1).seconds(), Duration::MIN)]
#[case((-5).seconds(), 5.seconds(), Duration::ZERO)]
#[case(1_600.milliseconds(), 1_600.milliseconds(), 3_200.milliseconds())]
#[case(1.seconds(), (-1).milliseconds(), 999.milliseconds())]
fn saturating_add(#[case] a: Duration, #[case] b: Duration, #[case] expected: Duration) {
    assert_eq!(a.saturating_add(b), expected);
}

#[rstest]
#[case(5.seconds(), 5.seconds(), Duration::ZERO)]
#[case(Duration::MIN, 1.nanoseconds(), Duration::MIN)]
#[case(Duration::MAX, (-1).nanoseconds(), Duration::MAX)]
#[case(Duration::MAX, (-1).seconds(), Duration::MAX)]
#[case(5.seconds(), 10.seconds(), (-5).seconds())]
#[case((-1_600).milliseconds(), 1_600.milliseconds(), (-3_200).milliseconds())]
#[case(0.seconds(), Duration::MIN, Duration::MIN)]
#[case(Duration::MIN, 5.seconds(), Duration::MIN)]
#[case(1_200.milliseconds(), 600.milliseconds(), 600.milliseconds())]
#[case((-1_200).milliseconds(), (-600).milliseconds(), (-600).milliseconds())]
fn saturating_sub(#[case] a: Duration, #[case] b: Duration, #[case] expected: Duration) {
    assert_eq!(a.saturating_sub(b), expected);
}

#[rstest]
#[case(5.seconds(), 2, 10.seconds())]
#[case(5.seconds(), -2, (-10).seconds())]
#[case(5.seconds(), 0, Duration::ZERO)]
#[case(Duration::MAX, 2, Duration::MAX)]
#[case(Duration::MIN, 2, Duration::MIN)]
#[case(Duration::MAX, -2, Duration::MIN)]
#[case(Duration::MIN, -2, Duration::MAX)]
#[case(
    Duration::new(1_844_674_407_370_955_161, 600_000_000),
    5,
    Duration::MAX
)]
#[case(Duration::new(1_844_674_407_370_955_161, 800_000_000), -5, Duration::MIN)]
fn saturating_mul(#[case] duration: Duration, #[case] rhs: i32, #[case] expected: Duration) {
    assert_eq!(duration.saturating_mul(rhs), expected);
}

#[rstest]
#[timeout(StdDuration::from_millis(100))]
fn time_fn() {
    #[expect(deprecated)]
    let (time, value) = Duration::time_fn(|| {
        std::thread::sleep(1.std_milliseconds());
        0
    });

    assert!(time >= 1.milliseconds());
    assert_eq!(value, 0);
}

#[rstest]
#[case(0.seconds(), "0s")]
#[case(60.days(), "60d")]
#[case((-48).hours(), "-2d")]
#[case(48.hours(), "2d")]
#[case(1.minutes(), "1m")]
#[case(10.minutes(), "10m")]
#[case(1.seconds(), "1s")]
#[case(10.seconds(), "10s")]
#[case(1.milliseconds(), "1ms")]
#[case(10.milliseconds(), "10ms")]
#[case(100.milliseconds(), "100ms")]
#[case(1.microseconds(), "1µs")]
#[case(10.microseconds(), "10µs")]
#[case(100.microseconds(), "100µs")]
#[case(1.nanoseconds(), "1ns")]
#[case(10.nanoseconds(), "10ns")]
#[case(100.nanoseconds(), "100ns")]
fn display_basic(#[case] duration: Duration, #[case] expected: &str) {
    assert_eq!(duration.to_string(), expected);
}

#[rstest]
#[case(1.days(), "1d")]
#[case(26.hours(), "1d2h")]
#[case(1_563.minutes(), "1d2h3m")]
#[case(93_784.seconds(), "1d2h3m4s")]
#[case(93_784_005.milliseconds(), "1d2h3m4s5ms")]
#[case(93_784_005_006.microseconds(), "1d2h3m4s5ms6µs")]
#[case(93_784_005_006_007.nanoseconds(), "1d2h3m4s5ms6µs7ns")]
fn display_compound(#[case] duration: Duration, #[case] expected: &str) {
    assert_eq!(duration.to_string(), expected);
}

#[rstest]
#[case(0.seconds(), 3, "0.000s")]
#[case(60.days(), 3, "60.000d")]
#[case((-48).hours(), 3, "-2.000d")]
#[case(48.hours(), 3, "2.000d")]
#[case(1.minutes(), 3, "1.000m")]
#[case(10.minutes(), 3, "10.000m")]
#[case(1.seconds(), 3, "1.000s")]
#[case(10.seconds(), 3, "10.000s")]
#[case(1.milliseconds(), 3, "1.000ms")]
#[case(10.milliseconds(), 3, "10.000ms")]
#[case(100.milliseconds(), 3, "100.000ms")]
#[case(1.microseconds(), 3, "1.000µs")]
#[case(10.microseconds(), 3, "10.000µs")]
#[case(100.microseconds(), 3, "100.000µs")]
#[case(1.nanoseconds(), 3, "1.000ns")]
#[case(10.nanoseconds(), 3, "10.000ns")]
#[case(100.nanoseconds(), 3, "100.000ns")]
#[case(1.days(), 3, "1.000d")]
#[case(26.hours(), 3, "1.083d")]
#[case(1_563.minutes(), 4, "1.0854d")]
#[case(93_784.seconds(), 5, "1.08546d")]
#[case(93_784_005.milliseconds(), 6, "1.085463d")]
#[case(93_784_005_006.microseconds(), 9, "1.085463021d")]
#[case(93_784_005_006_007.nanoseconds(), 12, "1.085463020903d")]
fn display_precision(#[case] duration: Duration, #[case] precision: usize, #[case] expected: &str) {
    assert_eq!(format!("{duration:.precision$}"), expected);
}

#[rstest]
#[case(0.std_seconds(), 0.seconds())]
#[case(1.std_seconds(), 1.seconds())]
fn try_from_std_duration_success(#[case] std_duration: StdDuration, #[case] expected: Duration) {
    assert_eq!(Duration::try_from(std_duration), Ok(expected));
}

#[rstest]
#[case(u64::MAX.std_seconds(), error::ConversionRange)]
fn try_from_std_duration_error(
    #[case] std_duration: StdDuration,
    #[case] expected: error::ConversionRange,
) {
    assert_eq!(Duration::try_from(std_duration), Err(expected));
}

#[rstest]
#[case(0.seconds(), 0.std_seconds())]
#[case(1.seconds(), 1.std_seconds())]
fn try_to_std_duration_success(#[case] duration: Duration, #[case] expected: StdDuration) {
    assert_eq!(StdDuration::try_from(duration), Ok(expected));
}

#[rstest]
#[case((-1).seconds())]
#[case((-500).milliseconds())]
fn try_to_std_duration_error(#[case] duration: Duration) {
    assert_eq!(StdDuration::try_from(duration), Err(error::ConversionRange));
}

#[rstest]
#[case(1.seconds(), 1.seconds(), 2.seconds())]
#[case(500.milliseconds(), 500.milliseconds(), 1.seconds())]
#[case(1.seconds(), (-1).seconds(), 0.seconds())]
fn add(#[case] lhs: Duration, #[case] rhs: Duration, #[case] expected: Duration) {
    assert_eq!(lhs + rhs, expected);
}

#[rstest]
#[case(1.seconds(), 1.std_seconds(), 2.seconds())]
#[case(500.milliseconds(), 500.std_milliseconds(), 1.seconds())]
#[case((-1).seconds(), 1.std_seconds(), 0.seconds())]
fn add_std(#[case] lhs: Duration, #[case] rhs: StdDuration, #[case] expected: Duration) {
    assert_eq!(lhs + rhs, expected);
}

#[rstest]
#[case(1.std_seconds(), 1.seconds(), 2.seconds())]
#[case(500.std_milliseconds(), 500.milliseconds(), 1.seconds())]
#[case(1.std_seconds(), (-1).seconds(), 0.seconds())]
fn std_add(#[case] lhs: StdDuration, #[case] rhs: Duration, #[case] expected: Duration) {
    assert_eq!(lhs + rhs, expected);
}

#[rstest]
#[case(1.seconds(), 1.seconds(), 2.seconds())]
#[case(500.milliseconds(), 500.milliseconds(), 1.seconds())]
#[case(1.seconds(), (-1).seconds(), 0.seconds())]
fn add_assign(#[case] mut duration: Duration, #[case] other: Duration, #[case] expected: Duration) {
    duration += other;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.seconds(), 1.std_seconds(), 2.seconds())]
#[case(500.milliseconds(), 500.std_milliseconds(), 1.seconds())]
#[case((-1).seconds(), 1.std_seconds(), 0.seconds())]
fn add_assign_std(
    #[case] mut duration: Duration,
    #[case] other: StdDuration,
    #[case] expected: Duration,
) {
    duration += other;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.std_seconds(), 1.seconds(), 2.seconds())]
#[case(500.std_milliseconds(), 500.milliseconds(), 1.seconds())]
#[case(1.std_seconds(), (-1).seconds(), 0.seconds())]
fn std_add_assign(
    #[case] mut duration: StdDuration,
    #[case] other: Duration,
    #[case] expected: Duration,
) {
    duration += other;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.seconds(), (-1).seconds())]
#[case((-1).seconds(), 1.seconds())]
#[case(0.seconds(), 0.seconds())]
fn neg(#[case] duration: Duration, #[case] expected: Duration) {
    assert_eq!(-duration, expected);
}

#[rstest]
#[case(1.seconds(), 1.seconds(), 0.seconds())]
#[case(1_500.milliseconds(), 500.milliseconds(), 1.seconds())]
#[case(1.seconds(), (-1).seconds(), 2.seconds())]
fn sub(#[case] lhs: Duration, #[case] rhs: Duration, #[case] expected: Duration) {
    assert_eq!(lhs - rhs, expected);
}

#[rstest]
#[case(1.seconds(), 1.std_seconds(), 0.seconds())]
#[case(1_500.milliseconds(), 500.std_milliseconds(), 1.seconds())]
#[case((-1).seconds(), 1.std_seconds(), -(2.seconds()))]
fn sub_std(#[case] lhs: Duration, #[case] rhs: StdDuration, #[case] expected: Duration) {
    assert_eq!(lhs - rhs, expected);
}

#[rstest]
#[case(1.std_seconds(), 1.seconds(), 0.seconds())]
#[case(1_500.std_milliseconds(), 500.milliseconds(), 1.seconds())]
#[case(1.std_seconds(), (-1).seconds(), 2.seconds())]
fn std_sub(#[case] lhs: StdDuration, #[case] rhs: Duration, #[case] expected: Duration) {
    assert_eq!(lhs - rhs, expected);
}

#[rstest]
#[case(1.seconds(), 1.seconds(), 0.seconds())]
#[case(1_500.milliseconds(), 500.milliseconds(), 1.seconds())]
#[case(1.seconds(), (-1).seconds(), 2.seconds())]
fn sub_assign(#[case] mut duration: Duration, #[case] other: Duration, #[case] expected: Duration) {
    duration -= other;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.seconds(), 1.std_seconds(), 0.seconds())]
#[case(1_500.milliseconds(), 500.std_milliseconds(), 1.seconds())]
#[case((-1).seconds(), 1.std_seconds(), -(2.seconds()))]
fn sub_assign_std(
    #[case] mut duration: Duration,
    #[case] other: StdDuration,
    #[case] expected: Duration,
) {
    duration -= other;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.std_seconds(), 1.seconds(), 0.seconds())]
#[case(1_500.std_milliseconds(), 500.milliseconds(), 1.seconds())]
#[case(1.std_seconds(), (-1).seconds(), 2.seconds())]
fn std_sub_assign(
    #[case] mut duration: StdDuration,
    #[case] other: Duration,
    #[case] expected: Duration,
) {
    duration -= other;
    assert_eq!(duration, expected);
}

#[rstest]
#[should_panic]
fn std_sub_assign_overflow() {
    let mut duration = 1.std_seconds();
    duration -= 2.seconds();
}

#[rstest]
#[case(1.seconds(), 2, 2.seconds())]
#[case(1.seconds(), -2, (-2).seconds())]
fn mul_int_success(#[case] duration: Duration, #[case] rhs: i32, #[case] expected: Duration) {
    assert_eq!(duration * rhs, expected);
}

#[rstest]
#[case(Duration::MAX, 2)]
#[case(Duration::MIN, 2)]
#[should_panic]
fn mul_int_panic(#[case] duration: Duration, #[case] rhs: i32) {
    let _ = duration * rhs;
}

#[rstest]
#[case(1.seconds(), 2, 2.seconds())]
#[case(1.seconds(), -2, (-2).seconds())]
fn mul_int_assign(#[case] mut duration: Duration, #[case] rhs: i32, #[case] expected: Duration) {
    duration *= rhs;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(2, 1.seconds(), 2.seconds())]
#[case(-2, 1.seconds(), (-2).seconds())]
fn int_mul(#[case] lhs: i32, #[case] duration: Duration, #[case] expected: Duration) {
    assert_eq!(lhs * duration, expected);
}

#[rstest]
#[case(1.seconds(), 2, 500.milliseconds())]
#[case(1.seconds(), -2, (-500).milliseconds())]
fn div_int(#[case] duration: Duration, #[case] rhs: i32, #[case] expected: Duration) {
    assert_eq!(duration / rhs, expected);
}

#[rstest]
#[case(1.seconds(), 2, 500.milliseconds())]
#[case(1.seconds(), -2, (-500).milliseconds())]
fn div_int_assign(#[case] mut duration: Duration, #[case] rhs: i32, #[case] expected: Duration) {
    duration /= rhs;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.seconds(), 0.5.seconds(), 2.)]
#[case(2.seconds(), 0.25.seconds(), 8.)]
fn div(#[case] lhs: Duration, #[case] rhs: Duration, #[case] expected: f64) {
    assert_eq!(lhs / rhs, expected);
}

#[rstest]
#[case(1.seconds(), 0.5.std_seconds(), 2.)]
#[case(2.seconds(), 0.25.std_seconds(), 8.)]
fn div_std(#[case] lhs: Duration, #[case] rhs: StdDuration, #[case] expected: f64) {
    assert_eq!(lhs / rhs, expected);
}

#[rstest]
#[case(1.std_seconds(), 0.5.seconds(), 2.)]
#[case(2.std_seconds(), 0.25.seconds(), 8.)]
// #[expect(clippy::float_cmp)]
fn std_div(#[case] lhs: StdDuration, #[case] rhs: Duration, #[case] expected: f64) {
    assert_eq!(lhs / rhs, expected);
}

#[rstest]
#[case(1.seconds(), 1.5, 1_500.milliseconds())]
#[case(1.seconds(), 2.5, 2_500.milliseconds())]
#[case(1.seconds(), -1.5, (-1_500).milliseconds())]
#[case(1.seconds(), 0., 0.seconds())]
fn mul_f32(#[case] duration: Duration, #[case] rhs: f32, #[case] expected: Duration) {
    assert_eq!(duration * rhs, expected);
}

#[rstest]
#[case(1.seconds(), 1.5, 1_500.milliseconds())]
#[case(1.seconds(), 2.5, 2_500.milliseconds())]
#[case(1.seconds(), -1.5, (-1_500).milliseconds())]
#[case(1.seconds(), 0., 0.seconds())]
fn mul_f64(#[case] duration: Duration, #[case] rhs: f64, #[case] expected: Duration) {
    assert_eq!(duration * rhs, expected);
}

#[rstest]
#[case(1.5, 1.seconds(), 1_500.milliseconds())]
#[case(2.5, 1.seconds(), 2_500.milliseconds())]
#[case(-1.5, 1.seconds(), (-1_500).milliseconds())]
#[case(0., 1.seconds(), 0.seconds())]
fn f32_mul(#[case] lhs: f32, #[case] duration: Duration, #[case] expected: Duration) {
    assert_eq!(lhs * duration, expected);
}

#[rstest]
#[case(1.5, 1.seconds(), 1_500.milliseconds())]
#[case(2.5, 1.seconds(), 2_500.milliseconds())]
#[case(-1.5, 1.seconds(), (-1_500).milliseconds())]
#[case(0., 1.seconds(), 0.seconds())]
fn f64_mul(#[case] lhs: f64, #[case] duration: Duration, #[case] expected: Duration) {
    assert_eq!(lhs * duration, expected);
}

#[rstest]
#[case(1.seconds(), 1.5, 1_500.milliseconds())]
#[case(1.seconds(), 2.5, 2_500.milliseconds())]
#[case(1.seconds(), -1.5, (-1_500).milliseconds())]
#[case(1.seconds(), 0., 0.seconds())]
fn mul_f32_assign(#[case] mut duration: Duration, #[case] rhs: f32, #[case] expected: Duration) {
    duration *= rhs;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.seconds(), 1.5, 1_500.milliseconds())]
#[case(1.seconds(), 2.5, 2_500.milliseconds())]
#[case(1.seconds(), -1.5, (-1_500).milliseconds())]
#[case(1.seconds(), 0., 0.seconds())]
fn mul_f64_assign(#[case] mut duration: Duration, #[case] rhs: f64, #[case] expected: Duration) {
    duration *= rhs;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.seconds(), 1., 1.seconds())]
#[case(1.seconds(), 2., 500.milliseconds())]
#[case(1.seconds(), 4., 250.milliseconds())]
#[case(1.seconds(), 0.25, 4.seconds())]
#[case(1.seconds(), -1., (-1).seconds())]
fn div_f32(#[case] duration: Duration, #[case] rhs: f32, #[case] expected: Duration) {
    assert_eq!(duration / rhs, expected);
}

#[rstest]
#[case(1.seconds(), 1., 1.seconds())]
#[case(1.seconds(), 2., 500.milliseconds())]
#[case(1.seconds(), 4., 250.milliseconds())]
#[case(1.seconds(), 0.25, 4.seconds())]
#[case(1.seconds(), -1., (-1).seconds())]
fn div_f64(#[case] duration: Duration, #[case] rhs: f64, #[case] expected: Duration) {
    assert_eq!(duration / rhs, expected);
}

#[rstest]
#[case(1.seconds(), 1., 1.seconds())]
#[case(1.seconds(), 2., 500.milliseconds())]
#[case(1.seconds(), 4., 250.milliseconds())]
#[case(1.seconds(), 0.25, 4.seconds())]
#[case(1.seconds(), -1., (-1).seconds())]
fn div_f32_assign(#[case] mut duration: Duration, #[case] rhs: f32, #[case] expected: Duration) {
    duration /= rhs;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.seconds(), 1., 1.seconds())]
#[case(1.seconds(), 2., 500.milliseconds())]
#[case(1.seconds(), 4., 250.milliseconds())]
#[case(1.seconds(), 0.25, 4.seconds())]
#[case(1.seconds(), -1., (-1).seconds())]
fn div_f64_assign(#[case] mut duration: Duration, #[case] rhs: f64, #[case] expected: Duration) {
    duration /= rhs;
    assert_eq!(duration, expected);
}

#[rstest]
#[case(1.seconds(), 1.seconds(), true)]
#[case(0.seconds(), 0.seconds(), true)]
#[case(1.seconds(), 2.seconds(), false)]
#[case(1.seconds(), (-1).seconds(), false)]
#[case((-1).seconds(), (-1).seconds(), true)]
#[case(40.seconds(), 1.minutes(), false)]
fn partial_eq(#[case] lhs: Duration, #[case] rhs: Duration, #[case] expected: bool) {
    assert_eq_ne!(lhs, rhs, expected);
}

#[rstest]
#[case(1.seconds(), 1.std_seconds(), true)]
#[case(0.seconds(), 0.std_seconds(), true)]
#[case(1.seconds(), 2.std_seconds(), false)]
#[case((-1).seconds(), 1.std_seconds(), false)]
#[case(40.seconds(), 1.std_minutes(), false)]
fn partial_eq_std(#[case] lhs: Duration, #[case] rhs: StdDuration, #[case] expected: bool) {
    assert_eq_ne!(lhs, rhs, expected);
}

#[rstest]
#[case(1.std_seconds(), 1.seconds(), true)]
#[case(0.std_seconds(), 0.seconds(), true)]
#[case(2.std_seconds(), 1.seconds(), false)]
#[case(1.std_seconds(), (-1).seconds(), false)]
#[case(1.std_minutes(), 40.seconds(), false)]
fn std_partial_eq(#[case] lhs: StdDuration, #[case] rhs: Duration, #[case] expected: bool) {
    assert_eq_ne!(lhs, rhs, expected);
}

#[rstest]
#[case(0.seconds(), 0.seconds(), Equal)]
#[case(1.seconds(), 0.seconds(), Greater)]
#[case(1.seconds(), (-1).seconds(), Greater)]
#[case((-1).seconds(), 1.seconds(), Less)]
#[case(0.seconds(), (-1).seconds(), Greater)]
#[case(0.seconds(), 1.seconds(), Less)]
#[case((-1).seconds(), 0.seconds(), Less)]
#[case(1.minutes(), 1.seconds(), Greater)]
#[case((-1).minutes(), (-1).seconds(), Less)]
fn partial_ord(#[case] lhs: Duration, #[case] rhs: Duration, #[case] expected: Ordering) {
    assert_eq!(lhs.partial_cmp(&rhs), Some(expected));
}

#[rstest]
#[case(0.seconds(), 0.std_seconds(), Equal)]
#[case(1.seconds(), 0.std_seconds(), Greater)]
#[case((-1).seconds(), 1.std_seconds(), Less)]
#[case(0.seconds(), 1.std_seconds(), Less)]
#[case((-1).seconds(), 0.std_seconds(), Less)]
#[case(1.minutes(), 1.std_seconds(), Greater)]
#[case(0.seconds(), u64::MAX.std_seconds(), Less)]
fn partial_ord_std(#[case] lhs: Duration, #[case] rhs: StdDuration, #[case] expected: Ordering) {
    assert_eq!(lhs.partial_cmp(&rhs), Some(expected));
}

#[rstest]
#[case(0.std_seconds(), 0.seconds(), Equal)]
#[case(1.std_seconds(), 0.seconds(), Greater)]
#[case(1.std_seconds(), (-1).seconds(), Greater)]
#[case(0.std_seconds(), (-1).seconds(), Greater)]
#[case(0.std_seconds(), 1.seconds(), Less)]
#[case(1.std_minutes(), 1.seconds(), Greater)]
fn std_partial_ord(#[case] lhs: StdDuration, #[case] rhs: Duration, #[case] expected: Ordering) {
    assert_eq!(lhs.partial_cmp(&rhs), Some(expected));
}

#[rstest]
#[case(0.seconds(), 0.seconds(), Equal)]
#[case(1.seconds(), 0.seconds(), Greater)]
#[case(1.seconds(), (-1).seconds(), Greater)]
#[case((-1).seconds(), 1.seconds(), Less)]
#[case(0.seconds(), (-1).seconds(), Greater)]
#[case(0.seconds(), 1.seconds(), Less)]
#[case((-1).seconds(), 0.seconds(), Less)]
#[case(1.minutes(), 1.seconds(), Greater)]
#[case((-1).minutes(), (-1).seconds(), Less)]
#[case(100.nanoseconds(), 200.nanoseconds(), Less)]
#[case((-100).nanoseconds(), (-200).nanoseconds(), Greater)]
fn ord(#[case] lhs: Duration, #[case] rhs: Duration, #[case] expected: Ordering) {
    assert_eq!(lhs.cmp(&rhs), expected);
}

#[rstest]
fn arithmetic_regression() {
    let added = 1.6.seconds() + 1.6.seconds();
    assert_eq!(added.whole_seconds(), 3);
    assert_eq!(added.subsec_milliseconds(), 200);

    let subtracted = 1.6.seconds() - (-1.6).seconds();
    assert_eq!(subtracted.whole_seconds(), 3);
    assert_eq!(subtracted.subsec_milliseconds(), 200);
}

#[rstest]
fn sum_iter_ref() {
    let i = [1.6.seconds(), 1.6.seconds()];
    let sum = i.iter().sum::<Duration>();
    assert_eq!(sum, 3.2.seconds());
}

#[rstest]
fn sum_iter() {
    let i = [1.6.seconds(), 1.6.seconds()];
    let sum = i.into_iter().sum::<Duration>();
    assert_eq!(sum, 3.2.seconds());
}
