use std::num::NonZero;

use time::format_description::well_known::{Iso8601, Rfc2822, Rfc3339};
use time::format_description::{BorrowedFormatItem, Component, OwnedFormatItem, modifier};
use time::macros::{date, datetime, offset, time, utc_datetime};
use time::parsing::Parsed;
use time::{
    Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcDateTime, UtcOffset, Weekday, error,
    format_description as fd,
};

macro_rules! invalid_literal {
    () => {
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidLiteral { .. },
        ))
    };
}
macro_rules! invalid_component {
    ($name:literal) => {
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent($name),
        ))
    };
}

#[test]
#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
fn rfc_2822() -> time::Result<()> {
    assert_eq!(
        OffsetDateTime::parse("Sat, 02 Jan 2021 03:04:05 GMT", &Rfc2822)?,
        datetime!(2021-01-02 03:04:05 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("Sat, 02 Jan 2021 03:04:05 UT", &Rfc2822)?,
        datetime!(2021-01-02 03:04:05 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("Sat, 02 Jan 2021 03:04:05 +0000", &Rfc2822)?,
        datetime!(2021-01-02 03:04:05 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("Sat, 02 Jan 2021 03:04:05 +0607", &Rfc2822)?,
        datetime!(2021-01-02 03:04:05 +06:07),
    );
    assert_eq!(
        OffsetDateTime::parse("Sat, 02 Jan 2021 03:04:05 -0607", &Rfc2822)?,
        datetime!(2021-01-02 03:04:05 -06:07),
    );
    assert_eq!(
        OffsetDateTime::parse("Fri, 31 Dec 2021 23:59:60 Z", &Rfc2822)?,
        datetime!(2021-12-31 23:59:59.999_999_999 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("Fri, 31 Dec 2021 23:59:60 z", &Rfc2822)?,
        datetime!(2021-12-31 23:59:59.999_999_999 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("Fri, 31 Dec 2021 23:59:60 a", &Rfc2822)?,
        datetime!(2021-12-31 23:59:59.999_999_999 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("Fri, 31 Dec 2021 23:59:60 A", &Rfc2822)?,
        datetime!(2021-12-31 23:59:59.999_999_999 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("Fri, 31 Dec 2021 17:52:60 -0607", &Rfc2822)?,
        datetime!(2021-12-31 17:52:59.999_999_999 -06:07),
    );
    assert_eq!(
        OffsetDateTime::parse("Sat, 01 Jan 2022 06:06:60 +0607", &Rfc2822)?,
        datetime!(2022-01-01 06:06:59.999_999_999 +06:07),
    );

    assert_eq!(
        UtcDateTime::parse("Sat, 02 Jan 2021 03:04:05 GMT", &Rfc2822)?,
        utc_datetime!(2021-01-02 03:04:05),
    );
    assert_eq!(
        UtcDateTime::parse("Sat, 02 Jan 2021 03:04:05 UT", &Rfc2822)?,
        utc_datetime!(2021-01-02 03:04:05),
    );
    assert_eq!(
        UtcDateTime::parse("Sat, 02 Jan 2021 03:04:05 +0000", &Rfc2822)?,
        utc_datetime!(2021-01-02 03:04:05),
    );
    assert_eq!(
        UtcDateTime::parse("Sat, 02 Jan 2021 03:04:05 +0607", &Rfc2822)?,
        datetime!(2021-01-02 03:04:05 +06:07).to_utc(),
    );
    assert_eq!(
        UtcDateTime::parse("Sat, 02 Jan 2021 03:04:05 -0607", &Rfc2822)?,
        datetime!(2021-01-02 03:04:05 -06:07).to_utc(),
    );
    assert_eq!(
        UtcDateTime::parse("Fri, 31 Dec 2021 23:59:60 Z", &Rfc2822)?,
        utc_datetime!(2021-12-31 23:59:59.999_999_999),
    );
    assert_eq!(
        UtcDateTime::parse("Fri, 31 Dec 2021 23:59:60 z", &Rfc2822)?,
        utc_datetime!(2021-12-31 23:59:59.999_999_999),
    );
    assert_eq!(
        UtcDateTime::parse("Fri, 31 Dec 2021 23:59:60 a", &Rfc2822)?,
        utc_datetime!(2021-12-31 23:59:59.999_999_999),
    );
    assert_eq!(
        UtcDateTime::parse("Fri, 31 Dec 2021 23:59:60 A", &Rfc2822)?,
        utc_datetime!(2021-12-31 23:59:59.999_999_999),
    );
    assert_eq!(
        UtcDateTime::parse("Fri, 31 Dec 2021 17:52:60 -0607", &Rfc2822)?,
        datetime!(2021-12-31 17:52:59.999_999_999 -06:07).to_utc(),
    );
    assert_eq!(
        UtcDateTime::parse("Sat, 01 Jan 2022 06:06:60 +0607", &Rfc2822)?,
        datetime!(2022-01-01 06:06:59.999_999_999 +06:07).to_utc(),
    );

    assert_eq!(
        Date::parse("Sat, 02 Jan 2021 03:04:05 GMT", &Rfc2822)?,
        date!(2021-01-02)
    );
    assert_eq!(
        Date::parse("Sat, 02 Jan 2021 03:04:05 +0607", &Rfc2822)?,
        date!(2021-01-02)
    );
    assert_eq!(
        Date::parse("Sat, 02 Jan 2021 03:04:05 -0607", &Rfc2822)?,
        date!(2021-01-02)
    );
    assert_eq!(
        Date::parse("Sat, 02 Jan 21 03:04:05 -0607", &Rfc2822)?,
        date!(2021-01-02)
    );
    assert_eq!(
        Date::parse("Sat, 02 Jan 71 03:04:05 -0607", &Rfc2822)?,
        date!(1971-01-02)
    );

    assert_eq!(
        OffsetDateTime::parse("Sat,(\\\u{a})02 Jan 2021 03:04:05 GMT", &Rfc2822)?,
        datetime!(2021-01-02 03:04:05 UTC),
    );
    #[rustfmt::skip]
    assert_eq!(
        Time::parse(
            "  \t Sat,\r\n \
            (\tfoo012FOO!)\
            (\u{1}\u{b}\u{e}\u{7f})\
            (\\\u{0})\
            (\\\u{1}\\\u{9}\\\u{28}\\\u{29}\\\\u{5c}\\\u{7f})\
            (\\\n\\\u{b})\
            02 \r\n  \r\n Jan 2021 03:04:05 GMT",
            &Rfc2822
        )?,
        time!(03:04:05)
    );

    Ok(())
}

#[test]
fn issue_661() -> time::Result<()> {
    assert_eq!(
        OffsetDateTime::parse("02 Jan 2021 03:04:05 +0607", &Rfc2822)?,
        datetime!(2021-01-02 03:04:05 +06:07),
    );
    assert_eq!(
        Date::parse("02 Jan 2021 03:04:05 +0607", &Rfc2822)?,
        date!(2021-01-02)
    );

    Ok(())
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn rfc_2822_err() {
    // In the first test, the "weekday" component is invalid, we're actually testing the whitespace
    // parser. The error is because the parser attempts and fails to parse the whitespace, but it's
    // optional so it backtracks and attempts to parse the weekday (while still having leading
    // whitespace). The weekday is also optional, so it backtracks and attempts to parse the day.
    // This component is required, so it fails at this point.
    assert!(matches!(
        OffsetDateTime::parse(" \r\nM", &Rfc2822),
        invalid_component!("day")
    ));

    assert!(matches!(
        OffsetDateTime::parse("Mon:", &Rfc2822),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, o2", &Rfc2822),
        invalid_component!("day")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02_", &Rfc2822),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 jxn", &Rfc2822),
        invalid_component!("month")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan_", &Rfc2822),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan abcd", &Rfc2822),
        invalid_component!("year")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 1899", &Rfc2822),
        invalid_component!("year")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021_", &Rfc2822),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 21_", &Rfc2822),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021 ab", &Rfc2822),
        invalid_component!("hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021 03_", &Rfc2822),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021 03:ab", &Rfc2822),
        invalid_component!("minute")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021 03:04_", &Rfc2822),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021 03:04:ab", &Rfc2822),
        invalid_component!("second")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021 03:04:05_", &Rfc2822),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021 03:04 6", &Rfc2822),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021 03:04:05 -6", &Rfc2822),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Mon, 02 Jan 2021 03:04:05 -060", &Rfc2822),
        invalid_component!("offset minute")
    ));
    assert!(matches!(
        OffsetDateTime::parse("Fri, 31 Dec 2021 23:59:61 Z", &Rfc2822),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component)))
            if component.name() == "second" && !component.is_conditional()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Fri, 31 Dec 2021 03:04:60 Z", &Rfc2822),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component)))
            if component.name() == "second" && component.is_conditional()
    ));
    assert!(matches!(
        OffsetDateTime::parse("Fri, 30 Dec 2021 23:59:60 Z", &Rfc2822),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component)))
            if component.name() == "second" && component.is_conditional()
    ));
}

#[test]
fn rfc_3339() -> time::Result<()> {
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-12-31T23:59:60Z", &Rfc3339)?,
        datetime!(2021-12-31 23:59:59.999_999_999 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2015-07-01T00:59:60+01:00", &Rfc3339)?,
        datetime!(2015-06-30 23:59:59.999_999_999 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.1Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.1 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.12Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.12 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.1234Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_4 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.12345Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_45 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.1234567Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_7 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.12345678Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_78 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456789Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_789 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456789-01:02", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_789 -01:02),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456789+01:02", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_789 +01:02),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123-00:01", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123 -00:01),
    );

    assert_eq!(
        Date::parse("2021-01-02T03:04:05Z", &Rfc3339)?,
        date!(2021-01-02),
    );
    assert_eq!(
        Date::parse("2021-01-02T03:04:05.123+01:02", &Rfc3339)?,
        date!(2021-01-02),
    );
    assert_eq!(
        Date::parse("2021-01-02T03:04:05.123-01:02", &Rfc3339)?,
        date!(2021-01-02),
    );

    assert_eq!(
        UtcOffset::parse("2021-01-02T03:04:05Z", &Rfc3339)?,
        offset!(UTC),
    );
    assert_eq!(
        UtcOffset::parse("2021-01-02T03:04:05.123+01:02", &Rfc3339)?,
        offset!(+01:02),
    );
    assert_eq!(
        UtcOffset::parse("2021-01-02T03:04:05.123-01:02", &Rfc3339)?,
        offset!(-01:02),
    );
    assert_eq!(
        UtcOffset::parse("2021-01-02T03:04:05.123-00:01", &Rfc3339)?,
        offset!(-00:01),
    );

    // Any separator is allowed by RFC 3339, not just `T`.
    assert_eq!(
        OffsetDateTime::parse("2021-01-02 03:04:05Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02$03:04:05Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05 UTC),
    );

    Ok(())
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn rfc_3339_err() {
    assert!(matches!(
        PrimitiveDateTime::parse("x", &Rfc3339),
        invalid_component!("year")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-x", &Rfc3339),
        invalid_component!("month")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-0", &Rfc3339),
        invalid_component!("month")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-0", &Rfc3339),
        invalid_component!("day")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01", &Rfc3339),
        invalid_component!("separator")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T0", &Rfc3339),
        invalid_component!("hour")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:0", &Rfc3339),
        invalid_component!("minute")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:0", &Rfc3339),
        invalid_component!("second")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00.x", &Rfc3339),
        invalid_component!("subsecond")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00x", &Rfc3339),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00+0", &Rfc3339),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00+00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00+00:0", &Rfc3339),
        invalid_component!("offset minute")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-13-01T00:00:00Z", &Rfc3339),
        invalid_component!("month")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-02T03:04:60Z", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "second"
    ));

    // Conversions to offset-unaware types do not perform special treatment for leap seconds
    // even if the input could refer to one.
    assert!(matches!(
        PrimitiveDateTime::parse("2022-01-01T00:59:60+01:00", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "second"
    ));
    assert!(matches!(
        Time::parse("2021-12-31T23:04:60Z", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "second"
    ));

    assert!(matches!(
        OffsetDateTime::parse("2021-01-02T03:04:05Z ", &Rfc3339),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("x", &Rfc3339),
        invalid_component!("year")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-x", &Rfc3339),
        invalid_component!("month")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-0", &Rfc3339),
        invalid_component!("month")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-0", &Rfc3339),
        invalid_component!("day")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01", &Rfc3339),
        invalid_component!("separator")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T0", &Rfc3339),
        invalid_component!("hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:0", &Rfc3339),
        invalid_component!("minute")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:0", &Rfc3339),
        invalid_component!("second")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00.x", &Rfc3339),
        invalid_component!("subsecond")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00x", &Rfc3339),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+0", &Rfc3339),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+00:0", &Rfc3339),
        invalid_component!("offset minute")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+24:00", &Rfc3339),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+00:60", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "offset minute"
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-13-01T00:00:00Z", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "month"
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-12-31T23:59:61Z", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "second" && !component.is_conditional()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-02T23:59:60Z", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "second" && component.is_conditional()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-12-31T03:04:60Z", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "second" && component.is_conditional()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-12-31T23:59:60+01:00", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "second" && component.is_conditional()
    ));
}

#[test]
fn iso_8601() {
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05Z", &Iso8601::DEFAULT),
        Ok(datetime!(2021-01-02 03:04:05 UTC))
    );
    assert_eq!(
        OffsetDateTime::parse("2021-002T03:04:05Z", &Iso8601::DEFAULT),
        Ok(datetime!(2021-002 03:04:05 UTC))
    );
    assert_eq!(
        OffsetDateTime::parse("2021-W01-2T03:04:05Z", &Iso8601::DEFAULT),
        Ok(datetime!(2021-W01-2 03:04:05 UTC))
    );
    assert_eq!(
        OffsetDateTime::parse("-002021-01-02T03:04:05+01:00", &Iso8601::DEFAULT),
        Ok(datetime!(-002021-01-02 03:04:05 +01:00))
    );
    assert_eq!(
        OffsetDateTime::parse("20210102T03.1Z", &Iso8601::DEFAULT),
        Ok(datetime!(2021-01-02 03:06:00 UTC))
    );
    assert_eq!(
        OffsetDateTime::parse("2021002T0304.1Z", &Iso8601::DEFAULT),
        Ok(datetime!(2021-002 03:04:06 UTC))
    );
    assert_eq!(
        OffsetDateTime::parse("2021W012T030405.1-0100", &Iso8601::DEFAULT),
        Ok(datetime!(2021-W01-2 03:04:05.1 -01:00))
    );
    assert_eq!(
        OffsetDateTime::parse("20210102T03Z", &Iso8601::DEFAULT),
        Ok(datetime!(2021-01-02 03:00:00 UTC))
    );
    assert_eq!(
        OffsetDateTime::parse("20210102T0304Z", &Iso8601::DEFAULT),
        Ok(datetime!(2021-01-02 03:04:00 UTC))
    );

    assert_eq!(
        UtcDateTime::parse("2021-01-02T03:04:05Z", &Iso8601::DEFAULT),
        Ok(utc_datetime!(2021-01-02 03:04:05))
    );
    assert_eq!(
        UtcDateTime::parse("2021-002T03:04:05Z", &Iso8601::DEFAULT),
        Ok(utc_datetime!(2021-002 03:04:05))
    );
    assert_eq!(
        UtcDateTime::parse("2021-W01-2T03:04:05Z", &Iso8601::DEFAULT),
        Ok(utc_datetime!(2021-W01-2 03:04:05))
    );
    assert_eq!(
        UtcDateTime::parse("-002021-01-02T03:04:05+01:00", &Iso8601::DEFAULT),
        Ok(datetime!(-002021-01-02 03:04:05 +01:00).to_utc())
    );
    assert_eq!(
        UtcDateTime::parse("20210102T03.1Z", &Iso8601::DEFAULT),
        Ok(utc_datetime!(2021-01-02 03:06:00))
    );
    assert_eq!(
        UtcDateTime::parse("2021002T0304.1Z", &Iso8601::DEFAULT),
        Ok(utc_datetime!(2021-002 03:04:06))
    );
    assert_eq!(
        UtcDateTime::parse("2021W012T030405.1-0100", &Iso8601::DEFAULT),
        Ok(datetime!(2021-W01-2 03:04:05.1 -01:00).to_utc())
    );
    assert_eq!(
        UtcDateTime::parse("20210102T03Z", &Iso8601::DEFAULT),
        Ok(utc_datetime!(2021-01-02 03:00:00))
    );
    assert_eq!(
        UtcDateTime::parse("20210102T0304Z", &Iso8601::DEFAULT),
        Ok(utc_datetime!(2021-01-02 03:04:00))
    );
    assert_eq!(UtcOffset::parse("+07", &Iso8601::DEFAULT), Ok(offset!(+7)));
    assert_eq!(
        UtcOffset::parse("+0304", &Iso8601::DEFAULT),
        Ok(offset!(+03:04))
    );
    assert_eq!(
        PrimitiveDateTime::parse("2022-07-22T12:52:50.349409", &Iso8601::DEFAULT),
        Ok(datetime!(2022-07-22 12:52:50.349409000))
    );
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn iso_8601_error() {
    assert!(matches!(
        OffsetDateTime::parse("20210102T03:04Z", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("20210102T03.", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-0102", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-x", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-Wx", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-W012", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-W01-x", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-02T03:x", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-02T03:04x", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-02T03:04:", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert_eq!(
        OffsetDateTime::parse("01:02", &Iso8601::DEFAULT),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );

    assert!(matches!(
        UtcDateTime::parse("20210102T03:04Z", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("20210102T03.", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("2021-0102", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("2021-01-x", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("2021-Wx", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("2021-W012", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("2021-W01-x", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("2021-01-02T03:x", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("2021-01-02T03:04x", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("2021-01-02T03:04:", &Iso8601::DEFAULT),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert_eq!(
        UtcDateTime::parse("01:02", &Iso8601::DEFAULT),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );
}

#[test]
fn parse_time() -> time::Result<()> {
    let format_input_output = [
        (fd::parse("[hour repr:12] [period]")?, "01 PM", time!(1 PM)),
        (fd::parse("[hour]")?, "12", time!(12:00)),
        (
            fd::parse("[hour]:[minute]:[second]")?,
            "13:02:03",
            time!(13:02:03),
        ),
        (
            fd::parse("[hour repr:12]:[minute] [period]")?,
            "01:02 PM",
            time!(1:02 PM),
        ),
        (fd::parse("[hour]:[minute]")?, "01:02", time!(1:02)),
        (
            fd::parse("[hour repr:12]:[minute] [period]")?,
            "01:02 AM",
            time!(1:02 AM),
        ),
        (fd::parse("[hour]:[minute]")?, "01:02", time!(1:02)),
        (fd::parse("[hour repr:12] [period]")?, "12 AM", time!(12 AM)),
        (fd::parse("[hour repr:12] [period]")?, "12 PM", time!(12 PM)),
    ];

    for (format_description, input, output) in &format_input_output {
        assert_eq!(&Time::parse(input, format_description)?, output);
        assert_eq!(
            &Time::parse(input, &OwnedFormatItem::from(format_description))?,
            output
        );
        assert_eq!(
            &Time::parse(
                input,
                [OwnedFormatItem::from(format_description)].as_slice()
            )?,
            output
        );
    }

    Ok(())
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn parse_time_err() -> time::Result<()> {
    assert_eq!(
        Time::try_from(Parsed::new()),
        Err(error::TryFromParsed::InsufficientInformation)
    );
    assert_eq!(
        Time::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );
    assert_eq!(
        Time::parse("12:34", &fd::parse("[hour]:[second]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );
    assert_eq!(
        Time::parse("12:34", &fd::parse("[hour]:[subsecond]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );
    assert!(matches!(
        Time::parse("13 PM", &fd::parse("[hour repr:12] [period]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("hour")
        ))
    ));
    assert!(matches!(
        Time::parse(" ", &fd::parse("")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        Time::parse("a", &fd::parse("[subsecond digits:1]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("1a", &fd::parse("[subsecond digits:2]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("1a", &fd::parse_owned::<2>("[subsecond digits:2]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse(
            "1a",
            [fd::parse_owned::<2>("[subsecond digits:2]")?].as_slice()
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("12a", &fd::parse("[subsecond digits:3]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("123a", &fd::parse("[subsecond digits:4]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("1234a", &fd::parse("[subsecond digits:5]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("12345a", &fd::parse("[subsecond digits:6]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("123456a", &fd::parse("[subsecond digits:7]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("1234567a", &fd::parse("[subsecond digits:8]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("12345678a", &fd::parse("[subsecond digits:9]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));

    Ok(())
}

#[test]
fn parse_date() -> time::Result<()> {
    let format_input_output = [
        (
            fd::parse("[year]-[month]-[day]")?,
            "2021-01-02",
            date!(2021-01-02),
        ),
        (
            fd::parse("[year repr:century range:standard][year repr:last_two]-[month]-[day]")?,
            "2021-01-02",
            date!(2021-01-02),
        ),
        (fd::parse("[year]-[ordinal]")?, "2021-002", date!(2021-002)),
        (
            fd::parse("[year base:iso_week]-W[week_number]-[weekday repr:monday]")?,
            "2020-W53-6",
            date!(2021-01-02),
        ),
        (
            fd::parse("[year]-W[week_number repr:monday]-[weekday repr:monday]")?,
            "2021-W00-6",
            date!(2021-01-02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2021-W00-6",
            date!(2021-01-02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2023-W01-1",
            date!(2023-01-02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2022-W00-7",
            date!(2022-01-02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2026-W00-5",
            date!(2026-01-02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2025-W00-4",
            date!(2025-01-02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2019-W00-3",
            date!(2019-01-02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2018-W01-2",
            date!(2018-01-02),
        ),
        (
            fd::parse(
                "[year padding:space]-W[week_number repr:sunday padding:none]-[weekday \
                 repr:sunday]",
            )?,
            " 201-W01-2",
            date!(201-01-06),
        ),
    ];

    for (format_description, input, output) in &format_input_output {
        assert_eq!(&Date::parse(input, format_description)?, output);
        assert_eq!(
            &Date::parse(input, &OwnedFormatItem::from(format_description))?,
            output
        );
    }

    Ok(())
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn parse_date_err() -> time::Result<()> {
    assert_eq!(
        Date::try_from(Parsed::new()),
        Err(error::TryFromParsed::InsufficientInformation)
    );
    assert_eq!(
        Date::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );
    assert!(matches!(
        Date::parse("a", &fd::parse("[year]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));
    assert!(matches!(
        Date::parse("0001", &fd::parse("[year sign:mandatory]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));
    assert!(matches!(
        Date::parse("0a", &fd::parse("[year repr:last_two]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));
    assert!(matches!(
        Date::parse("2021-366", &fd::parse("[year]-[ordinal]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "ordinal"
    ));
    assert!(matches!(
        Date::parse("2021-12-32", &fd::parse("[year]-[month]-[day]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("day")
        ))
    ));
    assert!(matches!(
        Date::parse("2021-02-30", &fd::parse("[year]-[month]-[day]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "day"
    ));
    assert!(matches!(
        Date::parse("2019-W53-1", &fd::parse("[year base:iso_week]-W[week_number]-[weekday repr:monday]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "week"
    ));
    assert!(matches!(
        Date::parse(
            "2021-W54-1",
            &fd::parse("[year base:iso_week]-W[week_number]-[weekday repr:monday]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("week number")
        ))
    ));
    assert!(matches!(
        Date::parse("2019-W53-1", &fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:monday]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "ordinal"
    ));
    assert!(matches!(
        Date::parse(
            "2021-W54-1",
            &fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:monday]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("week number")
        ))
    ));
    assert!(matches!(
        Date::parse("2019-W53-1", &fd::parse("[year]-W[week_number repr:monday]-[weekday repr:monday]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "ordinal"
    ));
    assert!(matches!(
        Date::parse(
            "2021-W54-1",
            &fd::parse("[year]-W[week_number repr:monday]-[weekday repr:monday]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("week number")
        ))
    ));
    assert!(matches!(
        Date::parse("Ja", &fd::parse("[month repr:short]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("month")
        ))
    ));
    assert!(matches!(
        Date::parse("  2a21", &fd::parse("[year padding:space]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));

    Ok(())
}

#[test]
fn parse_offset() -> time::Result<()> {
    // Regression check for #522.
    assert_eq!(
        UtcOffset::parse(
            "-00:01",
            &fd::parse("[offset_hour sign:mandatory]:[offset_minute]")?,
        ),
        Ok(offset!(-00:01)),
    );
    assert_eq!(
        UtcOffset::parse(
            "-00:00:01",
            &fd::parse("[offset_hour sign:mandatory]:[offset_minute]:[offset_second]")?,
        ),
        Ok(offset!(-00:00:01)),
    );

    Ok(())
}

#[test]
fn parse_offset_err() -> time::Result<()> {
    assert_eq!(
        UtcOffset::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );
    assert_eq!(
        UtcOffset::parse("01", &fd::parse("[offset_hour sign:mandatory]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("offset hour")
        ))
    );
    assert!(matches!(
        UtcOffset::parse("24", &fd::parse("[offset_hour]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("offset hour")
        ))
    ));
    assert!(matches!(
        UtcOffset::parse("00:60", &fd::parse("[offset_hour]:[offset_minute]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("offset minute")
        ))
    ));
    assert!(matches!(
        UtcOffset::parse(
            "00:00:60",
            &fd::parse("[offset_hour]:[offset_minute]:[offset_second]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("offset second")
        ))
    ));

    Ok(())
}

#[test]
fn parse_primitive_date_time() -> time::Result<()> {
    assert_eq!(
        PrimitiveDateTime::parse("2023-07-27 23", &fd::parse("[year]-[month]-[day] [hour]")?),
        Ok(datetime!(2023-07-27 23:00))
    );

    Ok(())
}

#[test]
fn parse_primitive_date_time_err() -> time::Result<()> {
    assert_eq!(
        PrimitiveDateTime::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );
    assert!(matches!(
        PrimitiveDateTime::parse(
            "2021-001 13 PM",
            &fd::parse("[year]-[ordinal] [hour repr:12] [period]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("hour")
        ))
    ));
    assert!(matches!(
        PrimitiveDateTime::parse(
            "2023-07-27 23:30",
            &fd::parse("[year]-[month]-[day] [hour]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));

    Ok(())
}

#[test]
fn parse_offset_date_time_err() -> time::Result<()> {
    assert_eq!(
        OffsetDateTime::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );
    assert!(matches!(
        OffsetDateTime::parse("x", &fd::parse("[year]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse(
            "2021-001 12 PM +25",
            &fd::parse("[year]-[ordinal] [hour repr:12] [period] [offset_hour sign:mandatory]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("offset hour")
        ))
    ));

    Ok(())
}

#[test]
fn parse_utc_date_time() -> time::Result<()> {
    assert_eq!(
        UtcDateTime::parse("2023-07-27 23", &fd::parse("[year]-[month]-[day] [hour]")?),
        Ok(utc_datetime!(2023-07-27 23:00))
    );

    Ok(())
}

#[test]
fn parse_utc_date_time_err() -> time::Result<()> {
    assert_eq!(
        UtcDateTime::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation
        ))
    );
    assert!(matches!(
        UtcDateTime::parse(
            "2021-001 13 PM",
            &fd::parse("[year]-[ordinal] [hour repr:12] [period]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("hour")
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse(
            "2023-07-27 23:30",
            &fd::parse("[year]-[month]-[day] [hour]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse("x", &fd::parse("[year]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));
    assert!(matches!(
        UtcDateTime::parse(
            "2021-001 12 PM +25",
            &fd::parse("[year]-[ordinal] [hour repr:12] [period] [offset_hour sign:mandatory]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("offset hour")
        ))
    ));

    Ok(())
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn parse_components() -> time::Result<()> {
    macro_rules! parse_component {
        ($component:expr, $input:expr, $(_. $property:ident() == $expected:expr);+ $(;)?) => {
            let mut parsed = Parsed::new();
            parsed.parse_component($input, $component)?;
            $(assert_eq!(parsed.$property(), $expected);)+
        };
    }

    parse_component!(
        Component::Year(
            modifier::Year::default()
                .with_padding(modifier::Padding::Zero)
                .with_repr(modifier::YearRepr::Full)
                .with_range(modifier::YearRange::Extended)
                .with_iso_week_based(false)
                .with_sign_is_mandatory(false)
        ),
        b"2021",
        _.year() == Some(2021)
    );
    parse_component!(
        Component::Year(modifier::Year::default()
            .with_padding(modifier::Padding::Zero)
            .with_repr(modifier::YearRepr::Century)
            .with_range(modifier::YearRange::Extended)
            .with_iso_week_based(false)
            .with_sign_is_mandatory(false)
        ),
        b"20",
        _.year_century() == Some(20);
        _.year_century_is_negative() == Some(false);
    );
    parse_component!(
        Component::Year(
            modifier::Year::default()
                .with_padding(modifier::Padding::Zero)
                .with_repr(modifier::YearRepr::LastTwo)
                .with_range(modifier::YearRange::Extended)
                .with_iso_week_based(false)
                .with_sign_is_mandatory(false)
        ),
        b"21",
        _.year_last_two() == Some(21)
    );
    parse_component!(
        Component::Year(
            modifier::Year::default()
                .with_padding(modifier::Padding::Zero)
                .with_repr(modifier::YearRepr::Full)
                .with_range(modifier::YearRange::Extended)
                .with_iso_week_based(true)
                .with_sign_is_mandatory(false)
        ),
        b"2021",
        _.iso_year() == Some(2021)
    );
    parse_component!(
        Component::Year(modifier::Year::default()
            .with_padding(modifier::Padding::Zero)
            .with_repr(modifier::YearRepr::Century)
            .with_range(modifier::YearRange::Extended)
            .with_iso_week_based(true)
            .with_sign_is_mandatory(false)
        ),
        b"20",
        _.iso_year_century() == Some(20);
        _.iso_year_century_is_negative() == Some(false);
    );
    parse_component!(
        Component::Year(
            modifier::Year::default()
                .with_padding(modifier::Padding::Zero)
                .with_repr(modifier::YearRepr::LastTwo)
                .with_range(modifier::YearRange::Extended)
                .with_iso_week_based(true)
                .with_sign_is_mandatory(false)
        ),
        b"21",
        _.iso_year_last_two() == Some(21)
    );
    parse_component!(
        Component::Month(
            modifier::Month::default()
                .with_padding(modifier::Padding::Space)
                .with_repr(modifier::MonthRepr::Numerical)
        ),
        b" 1",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Month(
            modifier::Month::default()
                .with_padding(modifier::Padding::None)
                .with_repr(modifier::MonthRepr::Short)
                .with_case_sensitive(true)
        ),
        b"Jan",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Month(
            modifier::Month::default()
                .with_padding(modifier::Padding::None)
                .with_repr(modifier::MonthRepr::Short)
                .with_case_sensitive(false)
        ),
        b"jAn",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Month(
            modifier::Month::default()
                .with_padding(modifier::Padding::None)
                .with_repr(modifier::MonthRepr::Long)
                .with_case_sensitive(true)
        ),
        b"January",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Month(
            modifier::Month::default()
                .with_padding(modifier::Padding::None)
                .with_repr(modifier::MonthRepr::Long)
                .with_case_sensitive(false)
        ),
        b"jAnUaRy",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Ordinal(modifier::Ordinal::default().with_padding(modifier::Padding::Zero)),
        b"012",
        _.ordinal() == 12.try_into().ok()
    );
    parse_component!(
        Component::Weekday(
            modifier::Weekday::default()
                .with_repr(modifier::WeekdayRepr::Short)
                .with_one_indexed(false)
                .with_case_sensitive(true)
        ),
        b"Sun",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(
            modifier::Weekday::default()
                .with_repr(modifier::WeekdayRepr::Short)
                .with_one_indexed(false)
                .with_case_sensitive(false)
        ),
        b"sUn",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(
            modifier::Weekday::default()
                .with_repr(modifier::WeekdayRepr::Long)
                .with_one_indexed(false)
                .with_case_sensitive(true)
        ),
        b"Sunday",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(
            modifier::Weekday::default()
                .with_repr(modifier::WeekdayRepr::Long)
                .with_one_indexed(false)
                .with_case_sensitive(false)
        ),
        b"sUnDaY",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(
            modifier::Weekday::default()
                .with_repr(modifier::WeekdayRepr::Sunday)
                .with_one_indexed(false)
        ),
        b"0",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(
            modifier::Weekday::default()
                .with_repr(modifier::WeekdayRepr::Sunday)
                .with_one_indexed(true)
        ),
        b"1",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(
            modifier::Weekday::default()
                .with_repr(modifier::WeekdayRepr::Monday)
                .with_one_indexed(false)
        ),
        b"6",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(
            modifier::Weekday::default()
                .with_repr(modifier::WeekdayRepr::Monday)
                .with_one_indexed(true)
        ),
        b"7",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::WeekNumber(
            modifier::WeekNumber::default()
                .with_padding(modifier::Padding::None)
                .with_repr(modifier::WeekNumberRepr::Sunday)
        ),
        b"2",
        _.sunday_week_number() == Some(2)
    );
    parse_component!(
        Component::WeekNumber(
            modifier::WeekNumber::default()
                .with_padding(modifier::Padding::None)
                .with_repr(modifier::WeekNumberRepr::Monday)
        ),
        b"2",
        _.monday_week_number() == Some(2)
    );
    parse_component!(
        Component::WeekNumber(
            modifier::WeekNumber::default()
                .with_padding(modifier::Padding::None)
                .with_repr(modifier::WeekNumberRepr::Iso)
        ),
        b"2",
        _.iso_week_number() == 2.try_into().ok()
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::One)
        ),
        b"1",
        _.subsecond() == Some(100_000_000)
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::Two)
        ),
        b"12",
        _.subsecond() == Some(120_000_000)
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::Three)
        ),
        b"123",
        _.subsecond() == Some(123_000_000)
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::Four)
        ),
        b"1234",
        _.subsecond() == Some(123_400_000)
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::Five)
        ),
        b"12345",
        _.subsecond() == Some(123_450_000)
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::Six)
        ),
        b"123456",
        _.subsecond() == Some(123_456_000)
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::Seven)
        ),
        b"1234567",
        _.subsecond() == Some(123_456_700)
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::Eight)
        ),
        b"12345678",
        _.subsecond() == Some(123_456_780)
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::Nine)
        ),
        b"123456789",
        _.subsecond() == Some(123_456_789)
    );
    parse_component!(
        Component::Subsecond(
            modifier::Subsecond::default().with_digits(modifier::SubsecondDigits::OneOrMore)
        ),
        b"123456789",
        _.subsecond() == Some(123_456_789)
    );
    parse_component!(
        Component::Period(
            modifier::Period::default()
                .with_is_uppercase(false)
                .with_case_sensitive(true)
        ),
        b"am",
        _.hour_12_is_pm() == Some(false)
    );
    parse_component!(
        Component::Period(
            modifier::Period::default()
                .with_is_uppercase(false)
                .with_case_sensitive(false)
        ),
        b"aM",
        _.hour_12_is_pm() == Some(false)
    );
    let mut parsed = Parsed::new();
    let result = parsed.parse_component(
        b"abcdef",
        Component::Ignore(modifier::Ignore::count(const { NonZero::new(3).unwrap() })),
    )?;
    assert_eq!(result, b"def");
    let mut parsed = Parsed::new();
    let result = parsed.parse_component(
        b"abcdef",
        Component::Ignore(modifier::Ignore::count(const { NonZero::new(7).unwrap() })),
    );
    assert!(matches!(
        result,
        Err(error::ParseFromDescription::InvalidComponent("ignore"))
    ));
    parse_component!(
        Component::UnixTimestamp(
            modifier::UnixTimestamp::default()
                .with_precision(modifier::UnixTimestampPrecision::Second)
                .with_sign_is_mandatory(false)
        ),
        b"1234567890",
        _.unix_timestamp_nanos() == Some(1_234_567_890_000_000_000)
    );
    parse_component!(
        Component::UnixTimestamp(
            modifier::UnixTimestamp::default()
                .with_precision(modifier::UnixTimestampPrecision::Millisecond)
                .with_sign_is_mandatory(false)
        ),
        b"1234567890123",
        _.unix_timestamp_nanos() == Some(1_234_567_890_123_000_000)
    );
    parse_component!(
        Component::UnixTimestamp(
            modifier::UnixTimestamp::default()
                .with_precision(modifier::UnixTimestampPrecision::Microsecond)
                .with_sign_is_mandatory(false)
        ),
        b"1234567890123456",
        _.unix_timestamp_nanos() == Some(1_234_567_890_123_456_000)
    );
    parse_component!(
        Component::UnixTimestamp(
            modifier::UnixTimestamp::default()
                .with_precision(modifier::UnixTimestampPrecision::Nanosecond)
                .with_sign_is_mandatory(false)
        ),
        b"1234567890123456789",
        _.unix_timestamp_nanos() == Some(1_234_567_890_123_456_789)
    );
    parse_component!(
        Component::UnixTimestamp(
            modifier::UnixTimestamp::default()
                .with_precision(modifier::UnixTimestampPrecision::Nanosecond)
                .with_sign_is_mandatory(false)
        ),
        b"-1234567890123456789",
        _.unix_timestamp_nanos() == Some(-1_234_567_890_123_456_789)
    );

    Ok(())
}

#[test]
fn parse_optional() -> time::Result<()> {
    // Ensure full parsing works as expected.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01-02",
        &BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(&fd::parse(
            "[year]-[month]-[day]",
        )?)),
    )?;
    assert!(remaining_input.is_empty());
    assert_eq!(parsed.year(), Some(2021));
    assert_eq!(parsed.month(), Some(Month::January));
    assert_eq!(parsed.day().map(NonZero::get), Some(2));

    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01-02",
        &OwnedFormatItem::from(BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(
            &fd::parse("[year]-[month]-[day]")?,
        ))),
    )?;
    assert!(remaining_input.is_empty());
    assert_eq!(parsed.year(), Some(2021));
    assert_eq!(parsed.month(), Some(Month::January));
    assert_eq!(parsed.day().map(NonZero::get), Some(2));

    // Ensure a successful partial parse *does not* mutate `parsed`.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01",
        &BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(&fd::parse(
            "[year]-[month]-[day]",
        )?)),
    )?;
    assert_eq!(remaining_input, b"2021-01");
    assert!(parsed.year().is_none());
    assert!(parsed.month().is_none());
    assert!(parsed.day().is_none());

    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01",
        &OwnedFormatItem::from(BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(
            &fd::parse("[year]-[month]-[day]")?,
        ))),
    )?;
    assert_eq!(remaining_input, b"2021-01");
    assert!(parsed.year().is_none());
    assert!(parsed.month().is_none());
    assert!(parsed.day().is_none());

    Ok(())
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn parse_first() -> time::Result<()> {
    // Ensure the first item is parsed correctly.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01-02",
        &BorrowedFormatItem::First(&[BorrowedFormatItem::Compound(&fd::parse(
            "[year]-[month]-[day]",
        )?)]),
    )?;
    assert!(remaining_input.is_empty());
    assert_eq!(parsed.year(), Some(2021));
    assert_eq!(parsed.month(), Some(Month::January));
    assert_eq!(parsed.day().map(NonZero::get), Some(2));

    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01-02",
        &OwnedFormatItem::from(BorrowedFormatItem::First(&[BorrowedFormatItem::Compound(
            &fd::parse("[year]-[month]-[day]")?,
        )])),
    )?;
    assert!(remaining_input.is_empty());
    assert_eq!(parsed.year(), Some(2021));
    assert_eq!(parsed.month(), Some(Month::January));
    assert_eq!(parsed.day().map(NonZero::get), Some(2));

    // Ensure an empty slice is a no-op success.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(b"2021-01-02", &BorrowedFormatItem::First(&[]))?;
    assert_eq!(remaining_input, b"2021-01-02");
    assert!(parsed.year().is_none());
    assert!(parsed.month().is_none());
    assert!(parsed.day().is_none());

    let mut parsed = Parsed::new();
    let remaining_input =
        parsed.parse_item(b"2021-01-02", &OwnedFormatItem::First(Box::new([])))?;
    assert_eq!(remaining_input, b"2021-01-02");
    assert!(parsed.year().is_none());
    assert!(parsed.month().is_none());
    assert!(parsed.day().is_none());

    // Ensure success when the first item fails.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01-02",
        &BorrowedFormatItem::First(&[
            BorrowedFormatItem::Compound(&fd::parse("[period]")?),
            BorrowedFormatItem::Compound(&fd::parse("x")?),
            BorrowedFormatItem::Compound(&fd::parse("[year]-[month]-[day]")?),
        ]),
    )?;
    assert!(remaining_input.is_empty());
    assert_eq!(parsed.year(), Some(2021));
    assert_eq!(parsed.month(), Some(Month::January));
    assert_eq!(parsed.day().map(NonZero::get), Some(2));

    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01-02",
        &OwnedFormatItem::from(BorrowedFormatItem::First(&[
            BorrowedFormatItem::Compound(&fd::parse("[period]")?),
            BorrowedFormatItem::Compound(&fd::parse("x")?),
            BorrowedFormatItem::Compound(&fd::parse("[year]-[month]-[day]")?),
        ])),
    )?;
    assert!(remaining_input.is_empty());
    assert_eq!(parsed.year(), Some(2021));
    assert_eq!(parsed.month(), Some(Month::January));
    assert_eq!(parsed.day().map(NonZero::get), Some(2));

    // Ensure the first error is returned.
    let mut parsed = Parsed::new();
    let err = parsed
        .parse_item(
            b"2021-01-02",
            &BorrowedFormatItem::First(&[
                BorrowedFormatItem::Compound(&fd::parse("[period]")?),
                BorrowedFormatItem::Compound(&fd::parse("x")?),
            ]),
        )
        .expect_err("parsing should fail");
    assert_eq!(err, error::ParseFromDescription::InvalidComponent("period"));

    let mut parsed = Parsed::new();
    let err = parsed
        .parse_item(
            b"2021-01-02",
            &OwnedFormatItem::from(BorrowedFormatItem::First(&[
                BorrowedFormatItem::Compound(&fd::parse("[period]")?),
                BorrowedFormatItem::Compound(&fd::parse("x")?),
            ])),
        )
        .expect_err("parsing should fail");
    assert_eq!(err, error::ParseFromDescription::InvalidComponent("period"));

    Ok(())
}

#[test]
fn parse_unix_timestamp() -> time::Result<()> {
    assert_eq!(
        OffsetDateTime::parse("1234567890", &fd::parse("[unix_timestamp]")?)?,
        datetime!(2009-02-13 23:31:30 UTC)
    );
    assert_eq!(
        OffsetDateTime::parse(
            "1234567890123",
            &fd::parse("[unix_timestamp precision:millisecond]")?
        )?,
        datetime!(2009-02-13 23:31:30.123 UTC)
    );
    assert_eq!(
        OffsetDateTime::parse(
            "1234567890123456",
            &fd::parse("[unix_timestamp precision:microsecond]")?
        )?,
        datetime!(2009-02-13 23:31:30.123456 UTC)
    );
    assert_eq!(
        OffsetDateTime::parse(
            "1234567890123456789",
            &fd::parse("[unix_timestamp precision:nanosecond]")?
        )?,
        datetime!(2009-02-13 23:31:30.123456789 UTC)
    );

    Ok(())
}

#[test]
fn parse_unix_timestamp_err() -> time::Result<()> {
    assert_eq!(
        OffsetDateTime::parse("1234567890", &fd::parse("[unix_timestamp sign:mandatory]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("unix_timestamp")
        ))
    );
    assert_eq!(
        OffsetDateTime::parse("a", &fd::parse("[unix_timestamp precision:second]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("unix_timestamp")
        ))
    );
    assert_eq!(
        OffsetDateTime::parse("a", &fd::parse("[unix_timestamp precision:millisecond]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("unix_timestamp")
        ))
    );
    assert_eq!(
        OffsetDateTime::parse("a", &fd::parse("[unix_timestamp precision:microsecond]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("unix_timestamp")
        ))
    );
    assert_eq!(
        OffsetDateTime::parse("a", &fd::parse("[unix_timestamp precision:nanosecond]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("unix_timestamp")
        ))
    );

    Ok(())
}

#[test]
fn issue_601() {
    let date = OffsetDateTime::parse(
        "1234567890.123",
        &fd::parse("[unix_timestamp].[subsecond digits:3]").expect("format description is valid"),
    );

    assert_eq!(date, Ok(datetime!(2009-02-13 23:31:30.123 +00:00:00)));
}

#[test]
fn end() -> time::Result<()> {
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"",
        &BorrowedFormatItem::Component(Component::End(modifier::End::default())),
    );
    assert_eq!(remaining_input, Ok(b"".as_slice()));

    assert_eq!(
        Time::parse("00:00", &fd::parse("[hour]:[minute][end]")?),
        Ok(time!(0:00))
    );
    assert_eq!(
        Time::parse(
            "00:00abcdef",
            &fd::parse("[hour]:[minute][end trailing_input:discard]")?
        ),
        Ok(time!(0:00))
    );
    assert_eq!(
        Time::parse(
            "00:00:00",
            &fd::parse_owned::<2>("[hour]:[minute][optional [[end]]]:[second]")?
        ),
        Ok(time!(0:00))
    );
    assert!(matches!(
        Time::parse(
            "00:00:00",
            &fd::parse_owned::<2>("[hour]:[minute][end]:[second]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::UnexpectedTrailingCharacters { .. }
        ))
    ));
    assert!(matches!(
        Time::parse(
            "00:00:00",
            &fd::parse_owned::<2>("[hour]:[minute][end trailing_input:discard]:[second]")?
        ),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidLiteral { .. }
        ))
    ));

    Ok(())
}
