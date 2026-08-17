use num_conv::prelude::*;
use quickcheck::{Arbitrary, TestResult};
use quickcheck_macros::quickcheck;
use time::macros::{format_description, time};
use time::Weekday::*;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

macro_rules! test_shrink {
    ($type:ty,
     $fn_name:ident,
     $($method:ident()).+
     $(, min=$min_value:literal)?
    ) => {
        #[quickcheck]
        fn $fn_name(v: $type) -> TestResult {
            let method_value = v.$($method()).+;
            if method_value == test_shrink!(@min_or_zero $($min_value)?) {
                TestResult::discard()
            } else {
                TestResult::from_bool(v.shrink().any(|shrunk|
                    if method_value > 0 {
                        shrunk.$($method()).+ < v.$($method()).+
                    } else {
                        shrunk.$($method()).+ > v.$($method()).+
                    }
                ))
            }
        }
    };
    (@min_or_zero) => { 0 };
    (@min_or_zero $min:literal) => { $min };
}

macro_rules! no_panic {
    ($($x:tt)*) => {
        std::panic::catch_unwind(|| {
            $($x)*
        })
        .is_ok()
    }
}

#[quickcheck]
fn date_yo_roundtrip(d: Date) -> bool {
    Date::from_ordinal_date(d.year(), d.ordinal()) == Ok(d)
}

#[quickcheck]
fn date_ymd_roundtrip(d: Date) -> bool {
    Date::from_calendar_date(d.year(), d.month(), d.day()) == Ok(d)
}

#[quickcheck]
fn date_ywd_roundtrip(d: Date) -> bool {
    let (year, week, weekday) = d.to_iso_week_date();
    Date::from_iso_week_date(year, week, weekday) == Ok(d)
}

#[quickcheck]
fn date_format_century_last_two_equivalent(d: Date) -> bool {
    let split_format = format_description!("[year repr:century][year repr:last_two]-[month]-[day]");
    let split = d.format(&split_format).expect("formatting failed");

    let combined_format = format_description!("[year]-[month]-[day]");
    let combined = d.format(&combined_format).expect("formatting failed");

    split == combined
}

#[quickcheck]
fn date_parse_century_last_two_equivalent_extended(d: Date) -> TestResult {
    // With the extended range, there is an ambiguity when parsing a year with fewer than six
    // digits, as the first four are consumed by the century, leaving at most one for the last
    // two digits.
    if !matches!(d.year().unsigned_abs().to_string().len(), 6) {
        return TestResult::discard();
    }

    let split_format = format_description!("[year repr:century][year repr:last_two]-[month]-[day]");
    let combined_format = format_description!("[year]-[month]-[day]");
    let combined = d.format(&combined_format).expect("formatting failed");

    TestResult::from_bool(Date::parse(&combined, &split_format).expect("parsing failed") == d)
}

#[quickcheck]
fn date_parse_century_last_two_equivalent_standard(d: Date) -> TestResult {
    // With the standard range, the year must be at most four digits.
    if !matches!(d.year(), -9999..=9999) {
        return TestResult::discard();
    }

    let split_format = format_description!(
        "[year repr:century range:standard][year repr:last_two range:standard]-[month]-[day]"
    );
    let combined_format = format_description!("[year range:standard]-[month]-[day]");
    let combined = d.format(&combined_format).expect("formatting failed");

    TestResult::from_bool(Date::parse(&combined, &split_format).expect("parsing failed") == d)
}

#[quickcheck]
fn julian_day_roundtrip(d: Date) -> bool {
    Date::from_julian_day(d.to_julian_day()) == Ok(d)
}

#[quickcheck]
fn duration_roundtrip(d: Duration) -> bool {
    Duration::new(d.whole_seconds(), d.subsec_nanoseconds()) == d
}

#[quickcheck]
fn time_roundtrip(t: Time) -> bool {
    Time::from_hms_nano(t.hour(), t.minute(), t.second(), t.nanosecond()) == Ok(t)
}

#[quickcheck]
fn primitive_date_time_roundtrip(a: PrimitiveDateTime) -> bool {
    PrimitiveDateTime::new(a.date(), a.time()) == a
}

#[quickcheck]
fn utc_offset_roundtrip(o: UtcOffset) -> bool {
    let (hours, minutes, seconds) = o.as_hms();
    UtcOffset::from_hms(hours, minutes, seconds) == Ok(o)
}

#[quickcheck]
fn offset_date_time_roundtrip(a: OffsetDateTime) -> TestResult {
    // Values near the edge of what is allowed may panic if the conversion brings the underlying
    // value outside the valid range of values.
    if a.date() == Date::MIN || a.date() == Date::MAX {
        return TestResult::discard();
    }

    TestResult::from_bool(PrimitiveDateTime::new(a.date(), a.time()).assume_offset(a.offset()) == a)
}

#[quickcheck]
fn unix_timestamp_roundtrip(odt: OffsetDateTime) -> TestResult {
    match odt.date() {
        Date::MIN | Date::MAX => TestResult::discard(),
        _ => TestResult::from_bool({
            // nanoseconds are not stored in the basic Unix timestamp
            let odt = odt - Duration::nanoseconds(odt.nanosecond().into());
            OffsetDateTime::from_unix_timestamp(odt.unix_timestamp()) == Ok(odt)
        }),
    }
}

#[quickcheck]
fn unix_timestamp_nanos_roundtrip(odt: OffsetDateTime) -> TestResult {
    match odt.date() {
        Date::MIN | Date::MAX => TestResult::discard(),
        _ => TestResult::from_bool(
            OffsetDateTime::from_unix_timestamp_nanos(odt.unix_timestamp_nanos()) == Ok(odt),
        ),
    }
}

#[quickcheck]
fn number_from_monday_roundtrip(w: Weekday) -> bool {
    Monday.nth_next(w.number_from_monday() + 7 - 1) == w
}

#[quickcheck]
fn number_from_sunday_roundtrip(w: Weekday) -> bool {
    Sunday.nth_next(w.number_from_sunday() + 7 - 1) == w
}

#[quickcheck]
fn number_days_from_monday_roundtrip(w: Weekday) -> bool {
    Monday.nth_next(w.number_days_from_monday()) == w
}

#[quickcheck]
fn number_days_from_sunday_roundtrip(w: Weekday) -> bool {
    Sunday.nth_next(w.number_days_from_sunday()) == w
}

#[quickcheck]
fn weekday_supports_arbitrary(w: Weekday) -> bool {
    (1..=7).contains(&w.number_from_monday())
}

#[quickcheck]
fn weekday_can_shrink(w: Weekday) -> bool {
    match w {
        Monday => w.shrink().next().is_none(),
        _ => w.shrink().next() == Some(w.previous()),
    }
}

#[quickcheck]
fn month_supports_arbitrary(m: Month) -> bool {
    (1..=12).contains(&u8::from(m))
}

#[quickcheck]
fn month_can_shrink(m: Month) -> bool {
    match m {
        Month::January => m.shrink().next().is_none(),
        _ => m.shrink().next() == Some(m.previous()),
    }
}

#[quickcheck]
fn date_replace_year(date: Date, year: i32) -> bool {
    date.replace_year(year) == Date::from_calendar_date(year, date.month(), date.day())
}

#[quickcheck]
fn date_replace_month(date: Date, month: Month) -> bool {
    date.replace_month(month) == Date::from_calendar_date(date.year(), month, date.day())
}

#[quickcheck]
fn date_replace_day(date: Date, day: u8) -> bool {
    date.replace_day(day) == Date::from_calendar_date(date.year(), date.month(), day)
}

#[quickcheck]
fn time_replace_hour(time: Time, hour: u8) -> bool {
    time.replace_hour(hour)
        == Time::from_hms_nano(hour, time.minute(), time.second(), time.nanosecond())
}

#[quickcheck]
fn pdt_replace_year(pdt: PrimitiveDateTime, year: i32) -> bool {
    pdt.replace_year(year)
        == Date::from_calendar_date(year, pdt.month(), pdt.day())
            .map(|date| date.with_time(pdt.time()))
}

#[quickcheck]
fn pdt_replace_month(pdt: PrimitiveDateTime, month: Month) -> bool {
    pdt.replace_month(month)
        == Date::from_calendar_date(pdt.year(), month, pdt.day())
            .map(|date| date.with_time(pdt.time()))
}

#[quickcheck]
fn pdt_replace_day(pdt: PrimitiveDateTime, day: u8) -> bool {
    pdt.replace_day(day)
        == Date::from_calendar_date(pdt.year(), pdt.month(), day)
            .map(|date| date.with_time(pdt.time()))
}

// Regression test for #481
#[quickcheck]
fn time_sub_time_no_panic(time_a: Time, time_b: Time) -> bool {
    no_panic! {
        let _ = time_a - time_b;
        let _ = time_b - time_a;
    }
}

#[quickcheck]
fn time_duration_until_since_range(time_a: Time, time_b: Time) -> bool {
    let a_until_b = time_a.duration_until(time_b);
    let b_until_a = time_b.duration_until(time_a);

    let a_since_b = time_a.duration_since(time_b);
    let b_since_a = time_b.duration_since(time_a);

    (Duration::ZERO..Duration::DAY).contains(&a_until_b)
        && (Duration::ZERO..Duration::DAY).contains(&b_until_a)
        && (Duration::ZERO..Duration::DAY).contains(&a_since_b)
        && (Duration::ZERO..Duration::DAY).contains(&b_since_a)
}

#[quickcheck]
fn time_duration_until_since_arithmetic(time_a: Time, time_b: Time) -> bool {
    let a_until_b = time_a.duration_until(time_b);
    let b_until_a = time_b.duration_until(time_a);

    let a_since_b = time_a.duration_since(time_b);
    let b_since_a = time_b.duration_since(time_a);

    time_a + a_until_b == time_b
        && time_b + b_until_a == time_a
        && time_b + a_since_b == time_a
        && time_a + b_since_a == time_b
}

#[quickcheck]
fn from_julian_day_no_panic(julian_day: i32) -> TestResult {
    if !(Date::MIN.to_julian_day()..=Date::MAX.to_julian_day()).contains(&julian_day) {
        return TestResult::discard();
    }

    TestResult::from_bool(
        std::panic::catch_unwind(|| {
            let _ = Date::from_julian_day(julian_day);
        })
        .is_ok(),
    )
}

#[quickcheck]
fn odt_eq_no_panic(left: OffsetDateTime, right: OffsetDateTime) -> bool {
    no_panic! {
        let _ = left == right;
    }
}

#[quickcheck]
fn odt_ord_no_panic(left: OffsetDateTime, right: OffsetDateTime) -> bool {
    no_panic! {
        let _ = left < right;
        let _ = left > right;
    }
}

#[quickcheck]
fn odt_sub_no_panic(left: OffsetDateTime, right: OffsetDateTime) -> bool {
    no_panic! {
        let _ = left - right;
    }
}

#[quickcheck]
fn odt_to_offset_no_panic(odt: OffsetDateTime, offset: UtcOffset) -> TestResult {
    let offset_difference = offset.whole_seconds() - odt.offset().whole_seconds();
    if Date::MIN
        .midnight()
        .assume_utc()
        .checked_add(Duration::seconds(offset_difference.extend()))
        .is_none()
        || Date::MAX
            .with_time(time!(23:59:59.999_999_999))
            .assume_utc()
            .checked_add(Duration::seconds(offset_difference.extend()))
            .is_none()
    {
        return TestResult::discard();
    }

    TestResult::from_bool(no_panic! {
        let _ = odt.to_offset(offset);
    })
}

#[quickcheck]
fn odt_replace_offset_no_panic(odt: OffsetDateTime, offset: UtcOffset) -> TestResult {
    if Date::MIN
        .midnight()
        .assume_offset(odt.offset())
        .checked_add(Duration::seconds(offset.whole_seconds().extend()))
        .is_none()
        || Date::MAX
            .with_time(time!(23:59:59.999_999_999))
            .assume_offset(odt.offset())
            .checked_add(Duration::seconds(offset.whole_seconds().extend()))
            .is_none()
    {
        return TestResult::discard();
    }

    TestResult::from_bool(no_panic! {
        let _ = odt.replace_offset(offset);
    })
}

test_shrink!(Date, date_can_shrink_year, year());
test_shrink!(Date, date_can_shrink_ordinal, ordinal(), min = 1);

test_shrink!(Duration, duration_can_shrink_seconds, whole_seconds());
test_shrink!(Duration, duration_can_shrink_ns, subsec_nanoseconds());

test_shrink!(Time, time_can_shrink_hour, hour());
test_shrink!(Time, time_can_shrink_minute, minute());
test_shrink!(Time, time_can_shrink_second, second());
test_shrink!(Time, time_can_shrink_nanosecond, nanosecond());

test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_year,
    year()
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_ordinal,
    ordinal(),
    min = 1
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_hour,
    hour()
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_minute,
    minute()
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_second,
    second()
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_nanosecond,
    nanosecond()
);

test_shrink!(UtcOffset, utc_offset_can_shrink, whole_seconds());

test_shrink!(
    OffsetDateTime,
    offset_date_time_can_shrink_offset,
    offset().whole_seconds()
);
test_shrink!(OffsetDateTime, offset_date_time_can_shrink_year, year());
test_shrink!(
    OffsetDateTime,
    offset_date_time_can_shrink_ordinal,
    ordinal(),
    min = 1
);
test_shrink!(OffsetDateTime, offset_date_time_can_shrink_hour, hour());
test_shrink!(OffsetDateTime, offset_date_time_can_shrink_minute, minute());
test_shrink!(OffsetDateTime, offset_date_time_can_shrink_second, second());
test_shrink!(
    OffsetDateTime,
    offset_date_time_can_shrink_nanosecond,
    nanosecond()
);
