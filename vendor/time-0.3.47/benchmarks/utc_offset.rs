use criterion::Bencher;
use time::{OffsetDateTime, UtcOffset};

setup_benchmark! {
    "UtcOffset",

    fn from_hms(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::from_hms(0, 0, 0));
    }

    fn from_whole_seconds(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::from_whole_seconds(0));
    }

    fn as_hms(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::UTC.as_hms());
    }

    fn whole_hours(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::UTC.whole_hours());
    }

    fn whole_minutes(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::UTC.whole_minutes());
    }

    fn minutes_past_hour(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::UTC.minutes_past_hour());
    }

    fn whole_seconds(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::UTC.whole_seconds());
    }

    fn seconds_past_minute(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::UTC.seconds_past_minute());
    }

    fn is_utc(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::UTC.is_utc());
    }

    fn is_positive(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::UTC.is_positive());
    }

    fn is_negative(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::UTC.is_negative());
    }

    fn local_offset_at(ben: &mut Bencher<'_>) {
        ben.iter(|| UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH));
    }

    fn current_local_offset(ben: &mut Bencher<'_>) {
        ben.iter(UtcOffset::current_local_offset);
    }
}
