#[macro_use]
extern crate bencher;

use std::io::Write;
use std::time::{Duration, UNIX_EPOCH};

use bencher::Bencher;
use chrono::{DateTime, Utc};
use humantime::format_rfc3339;

fn rfc3339_humantime_seconds(b: &mut Bencher) {
    let time = UNIX_EPOCH + Duration::new(1_483_228_799, 0);
    let mut buf = Vec::with_capacity(100);
    b.iter(|| {
        buf.truncate(0);
        write!(&mut buf, "{}", format_rfc3339(time)).unwrap()
    });
}

fn rfc3339_chrono(b: &mut Bencher) {
    use chrono::format::Fixed::*;
    use chrono::format::Item;
    use chrono::format::Item::*;
    use chrono::format::Numeric::*;
    use chrono::format::Pad::*;

    let time = DateTime::<Utc>::from_timestamp(1_483_228_799, 0).unwrap();
    let mut buf = Vec::with_capacity(100);

    // formatting code from env_logger
    const ITEMS: &[Item<'static>] = {
        &[
            Numeric(Year, Zero),
            Literal("-"),
            Numeric(Month, Zero),
            Literal("-"),
            Numeric(Day, Zero),
            Literal("T"),
            Numeric(Hour, Zero),
            Literal(":"),
            Numeric(Minute, Zero),
            Literal(":"),
            Numeric(Second, Zero),
            Fixed(TimezoneOffsetZ),
        ]
    };

    b.iter(|| {
        buf.truncate(0);
        write!(
            &mut buf,
            "{}",
            time.format_with_items(ITEMS.iter().cloned())
        )
        .unwrap()
    });
}

benchmark_group!(benches, rfc3339_humantime_seconds, rfc3339_chrono);
benchmark_main!(benches);
