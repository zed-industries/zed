use std::cmp::Ordering;

use rstest::rstest;
use time::macros::offset;
use time::{OffsetDateTime, UtcOffset};

#[test]
fn utc_is_zero() {
    assert_eq!(offset!(UTC), offset!(+0));
}

#[rstest]
#[case(0, 0, 0, offset!(UTC))]
#[case(0, 0, 1, offset!(+0:00:01))]
#[case(0, 0, -1, offset!(-0:00:01))]
#[case(1, 0, 0, offset!(+1))]
#[case(-1, 0, 0, offset!(-1))]
#[case(23, 59, 0, offset!(+23:59))]
#[case(-23, -59, 0, offset!(-23:59))]
#[case(23, 59, 59, offset!(+23:59:59))]
#[case(-23, -59, -59, offset!(-23:59:59))]
#[case(1, 2, 3, offset!(+1:02:03))]
#[case(1, -2, -3, offset!(+1:02:03))]
#[case(0, 2, -3, offset!(+0:02:03))]
fn from_hms(
    #[case] hours: i8,
    #[case] minutes: i8,
    #[case] seconds: i8,
    #[case] expected: UtcOffset,
) {
    assert_eq!(UtcOffset::from_hms(hours, minutes, seconds), Ok(expected));
}

#[rstest]
#[case(0, offset!(UTC))]
#[case(1, offset!(+0:00:01))]
#[case(-1, offset!(-0:00:01))]
#[case(3_600, offset!(+1))]
#[case(-3_600, offset!(-1))]
#[case(86_340, offset!(+23:59))]
#[case(-86_340, offset!(-23:59))]
#[case(86_399, offset!(+23:59:59))]
#[case(-86_399, offset!(-23:59:59))]
fn from_whole_seconds(#[case] seconds: i32, #[case] expected: UtcOffset) {
    assert_eq!(UtcOffset::from_whole_seconds(seconds), Ok(expected));
}

#[rstest]
#[case(offset!(UTC), (0, 0, 0))]
#[case(offset!(+0:00:01), (0, 0, 1))]
#[case(offset!(-0:00:01), (0, 0, -1))]
#[case(offset!(+1), (1, 0, 0))]
#[case(offset!(-1), (-1, 0, 0))]
#[case(offset!(+23:59), (23, 59, 0))]
#[case(offset!(-23:59), (-23, -59, 0))]
#[case(offset!(+23:59:59), (23, 59, 59))]
#[case(offset!(-23:59:59), (-23, -59, -59))]
fn as_hms(#[case] offset: UtcOffset, #[case] expected: (i8, i8, i8)) {
    assert_eq!(offset.as_hms(), expected);
}

#[rstest]
#[case(offset!(+1:02:03), 1)]
#[case(offset!(-1:02:03), -1)]
fn whole_hours(#[case] offset: UtcOffset, #[case] expected: i8) {
    assert_eq!(offset.whole_hours(), expected);
}

#[rstest]
#[case(offset!(+1:02:03), 62)]
#[case(offset!(-1:02:03), -62)]
fn whole_minutes(#[case] offset: UtcOffset, #[case] expected: i16) {
    assert_eq!(offset.whole_minutes(), expected);
}

#[rstest]
#[case(offset!(+1:02:03), 2)]
#[case(offset!(-1:02:03), -2)]
fn minutes_past_hour(#[case] offset: UtcOffset, #[case] expected: i8) {
    assert_eq!(offset.minutes_past_hour(), expected);
}

#[rstest]
#[case(offset!(UTC), 0)]
#[case(offset!(+0:00:01), 1)]
#[case(offset!(-0:00:01), -1)]
#[case(offset!(+1), 3_600)]
#[case(offset!(-1), -3_600)]
#[case(offset!(+23:59), 86_340)]
#[case(offset!(-23:59), -86_340)]
#[case(offset!(+23:59:59), 86_399)]
#[case(offset!(-23:59:59), -86_399)]
fn whole_seconds(#[case] offset: UtcOffset, #[case] expected: i32) {
    assert_eq!(offset.whole_seconds(), expected);
}

#[rstest]
#[case(offset!(+1:02:03), 3)]
#[case(offset!(-1:02:03), -3)]
fn seconds_past_minute(#[case] offset: UtcOffset, #[case] expected: i8) {
    assert_eq!(offset.seconds_past_minute(), expected);
}

#[rstest]
#[case(offset!(UTC), true)]
#[case(offset!(+0:00:01), false)]
#[case(offset!(-0:00:01), false)]
#[case(offset!(+1), false)]
#[case(offset!(-1), false)]
#[case(offset!(+23:59), false)]
#[case(offset!(-23:59), false)]
#[case(offset!(+23:59:59), false)]
#[case(offset!(-23:59:59), false)]
fn is_utc(#[case] offset: UtcOffset, #[case] expected: bool) {
    assert_eq!(offset.is_utc(), expected);
}

#[rstest]
#[case(offset!(UTC), false)]
#[case(offset!(+0:00:01), true)]
#[case(offset!(-0:00:01), false)]
#[case(offset!(+1), true)]
#[case(offset!(-1), false)]
#[case(offset!(+23:59), true)]
#[case(offset!(-23:59), false)]
#[case(offset!(+23:59:59), true)]
#[case(offset!(-23:59:59), false)]
fn is_positive(#[case] offset: UtcOffset, #[case] expected: bool) {
    assert_eq!(offset.is_positive(), expected);
}

#[rstest]
#[case(offset!(UTC), false)]
#[case(offset!(+0:00:01), false)]
#[case(offset!(-0:00:01), true)]
#[case(offset!(+1), false)]
#[case(offset!(-1), true)]
#[case(offset!(+23:59), false)]
#[case(offset!(-23:59), true)]
#[case(offset!(+23:59:59), false)]
#[case(offset!(-23:59:59), true)]
fn is_negative(#[case] offset: UtcOffset, #[case] expected: bool) {
    assert_eq!(offset.is_negative(), expected);
}

#[rstest]
#[case(offset!(UTC), offset!(UTC), Ordering::Equal)]
#[case(offset!(+1), offset!(+1), Ordering::Equal)]
#[case(offset!(-1), offset!(-1), Ordering::Equal)]
#[case(offset!(+1), offset!(UTC), Ordering::Greater)]
#[case(offset!(UTC), offset!(-1), Ordering::Greater)]
#[case(offset!(-1), offset!(+1), Ordering::Less)]
#[case(offset!(+23:59), offset!(+23:58), Ordering::Greater)]
#[case(offset!(-23:59), offset!(-23:58), Ordering::Less)]
#[case(offset!(+23:59:59), offset!(+23:59:58), Ordering::Greater)]
#[case(offset!(-23:59:59), offset!(-23:59:58), Ordering::Less)]
fn ordering(#[case] a: UtcOffset, #[case] b: UtcOffset, #[case] expected: Ordering) {
    assert_eq!(a.cmp(&b), expected);
}

#[rstest]
#[case(offset!(UTC), offset!(UTC))]
#[case(offset!(+0:00:01), offset!(-0:00:01))]
#[case(offset!(-0:00:01), offset!(+0:00:01))]
#[case(offset!(+1), offset!(-1))]
#[case(offset!(-1), offset!(+1))]
#[case(offset!(+23:59), offset!(-23:59))]
#[case(offset!(-23:59), offset!(+23:59))]
#[case(offset!(+23:59:59), offset!(-23:59:59))]
#[case(offset!(-23:59:59), offset!(+23:59:59))]
fn neg(#[case] offset: UtcOffset, #[case] expected: UtcOffset) {
    assert_eq!(-offset, expected);
}

#[test]
fn local_offset_at() {
    assert!(UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH).is_ok());
}

#[test]
fn current_local_offset() {
    assert!(UtcOffset::current_local_offset().is_ok());
}

#[test]
fn local_offset_success_when_multithreaded() {
    std::thread::spawn(|| {
        assert!(UtcOffset::current_local_offset().is_ok());
    })
    .join()
    .expect("failed to join thread");
}
