#[macro_use]
extern crate bencher;

use bencher::Bencher;
use chrono::DateTime;
use humantime::parse_rfc3339;

fn rfc3339_humantime_seconds(b: &mut Bencher) {
    b.iter(|| parse_rfc3339("2018-02-13T23:08:32Z").unwrap());
}

fn datetime_utc_parse_seconds(b: &mut Bencher) {
    b.iter(|| DateTime::parse_from_rfc3339("2018-02-13T23:08:32Z").unwrap());
}

fn rfc3339_humantime_millis(b: &mut Bencher) {
    b.iter(|| parse_rfc3339("2018-02-13T23:08:32.123Z").unwrap());
}

fn datetime_utc_parse_millis(b: &mut Bencher) {
    b.iter(|| DateTime::parse_from_rfc3339("2018-02-13T23:08:32.123Z").unwrap());
}

fn rfc3339_humantime_nanos(b: &mut Bencher) {
    b.iter(|| parse_rfc3339("2018-02-13T23:08:32.123456983Z").unwrap());
}

fn datetime_utc_parse_nanos(b: &mut Bencher) {
    b.iter(|| DateTime::parse_from_rfc3339("2018-02-13T23:08:32.123456983Z").unwrap());
}

benchmark_group!(
    benches,
    rfc3339_humantime_seconds,
    datetime_utc_parse_seconds,
    rfc3339_humantime_millis,
    datetime_utc_parse_millis,
    rfc3339_humantime_nanos,
    datetime_utc_parse_nanos
);
benchmark_main!(benches);
