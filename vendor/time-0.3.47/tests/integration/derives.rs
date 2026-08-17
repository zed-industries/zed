use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;

use time::error::{self, ConversionRange, IndeterminateOffset, TryFromParsed};
use time::ext::NumericalDuration;
use time::format_description::{self, modifier, well_known, Component, BorrowedFormatItem, OwnedFormatItem};
use time::macros::{date, offset, time, utc_datetime, datetime};
use time::parsing::Parsed;
use time::{Duration, Error, Month, Time, Weekday};
#[expect(deprecated)]
use time::Instant;

macro_rules! assert_cloned_eq {
    ($x:expr) => {
        assert_eq!($x.clone(), $x)
    };
}

fn component_range_error() -> error::ComponentRange {
    Time::from_hms(24, 0, 0).expect_err("24 is not a valid hour")
}

fn invalid_format_description() -> error::InvalidFormatDescription {
    format_description::parse("[").expect_err("format description is invalid")
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn clone() {
    #[expect(deprecated)]
    let instant = Instant::now();
    assert_cloned_eq!(date!(2021-001));
    assert_cloned_eq!(time!(0:00));
    assert_cloned_eq!(offset!(UTC));
    assert_cloned_eq!(datetime!(2021-001 0:00));
    assert_cloned_eq!(datetime!(2021-001 0:00 UTC));
    assert_cloned_eq!(utc_datetime!(2021-001 0:00));
    assert_cloned_eq!(Weekday::Monday);
    assert_cloned_eq!(Month::January);
    assert_cloned_eq!(Duration::ZERO);
    assert_cloned_eq!(instant);
    assert_cloned_eq!(IndeterminateOffset);
    assert_cloned_eq!(ConversionRange);
    assert_cloned_eq!(invalid_format_description());
    assert_cloned_eq!(TryFromParsed::InsufficientInformation);
    #[expect(clippy::clone_on_copy)] // needed for coverage
    let _ = Parsed::new().clone();
    assert_cloned_eq!(error::Parse::ParseFromDescription(
        error::ParseFromDescription::InvalidComponent("foo")
    ));
    assert_cloned_eq!(error::DifferentVariant);
    assert_cloned_eq!(error::InvalidVariant);
    assert_cloned_eq!(error::ParseFromDescription::InvalidComponent("foo"));
    assert_cloned_eq!(Component::OffsetSecond(modifier::OffsetSecond::default()));
    assert_cloned_eq!(well_known::Rfc2822);
    assert_cloned_eq!(well_known::Rfc3339);
    assert_cloned_eq!(well_known::Iso8601::DEFAULT);
    assert_cloned_eq!(well_known::iso8601::FormattedComponents::None);
    assert_cloned_eq!(well_known::iso8601::DateKind::Calendar);
    assert_cloned_eq!(well_known::iso8601::TimePrecision::Hour {
        decimal_digits: None
    });
    assert_cloned_eq!(well_known::iso8601::OffsetPrecision::Hour);
    assert_cloned_eq!(well_known::iso8601::FormattedComponents::None);
    assert_cloned_eq!(component_range_error());
    assert_cloned_eq!(BorrowedFormatItem::Literal(b""));

    assert_cloned_eq!(modifier::Day::default());
    assert_cloned_eq!(modifier::MonthRepr::default());
    assert_cloned_eq!(modifier::Month::default());
    assert_cloned_eq!(modifier::Ordinal::default());
    assert_cloned_eq!(modifier::WeekdayRepr::default());
    assert_cloned_eq!(modifier::Weekday::default());
    assert_cloned_eq!(modifier::WeekNumberRepr::default());
    assert_cloned_eq!(modifier::WeekNumber::default());
    assert_cloned_eq!(modifier::YearRepr::default());
    assert_cloned_eq!(modifier::Year::default());
    assert_cloned_eq!(modifier::Hour::default());
    assert_cloned_eq!(modifier::Minute::default());
    assert_cloned_eq!(modifier::Period::default());
    assert_cloned_eq!(modifier::Second::default());
    assert_cloned_eq!(modifier::SubsecondDigits::default());
    assert_cloned_eq!(modifier::Subsecond::default());
    assert_cloned_eq!(modifier::OffsetHour::default());
    assert_cloned_eq!(modifier::OffsetMinute::default());
    assert_cloned_eq!(modifier::OffsetSecond::default());
    assert_cloned_eq!(modifier::Padding::default());
}

#[test]
fn hash() {
    let mut hasher = DefaultHasher::new();
    date!(2021-001).hash(&mut hasher);
    time!(0:00).hash(&mut hasher);
    offset!(UTC).hash(&mut hasher);
    datetime!(2021-001 0:00).hash(&mut hasher);
    datetime!(2021-001 0:00 UTC).hash(&mut hasher);
    utc_datetime!(2021-001 0:00).hash(&mut hasher);
    Weekday::Monday.hash(&mut hasher);
    Month::January.hash(&mut hasher);
    #[expect(deprecated)]
    Instant::now().hash(&mut hasher);
    Duration::ZERO.hash(&mut hasher);
    component_range_error().hash(&mut hasher);
}

#[test]
fn partial_ord() {
    #[expect(deprecated)]
    let instant = Instant::now();
    assert_eq!(offset!(UTC).partial_cmp(&offset!(+1)), Some(Ordering::Less));
    assert_eq!(
        offset!(+1).partial_cmp(&offset!(UTC)),
        Some(Ordering::Greater)
    );
    assert_eq!(
        (instant - 1.seconds()).partial_cmp(&instant),
        Some(Ordering::Less)
    );
    assert_eq!(
        (instant + 1.seconds()).partial_cmp(&instant),
        Some(Ordering::Greater)
    );
}

#[test]
fn ord() {
    assert_eq!(offset!(UTC).cmp(&offset!(+1)), Ordering::Less);
    assert_eq!(offset!(+1).cmp(&offset!(UTC)), Ordering::Greater);
    assert_eq!(offset!(UTC).cmp(&offset!(UTC)), Ordering::Equal);
}

#[test]
fn debug() {
    macro_rules! debug_all {
        () => {};
        (#[$meta:meta] $x:expr; $($rest:tt)*) => {
            #[$meta]
            let _unused = format!("{:?}", $x);
            debug_all!($($rest)*);
        };
        ($x:expr; $($rest:tt)*) => {
            let _unused = format!("{:?}", $x);
            debug_all!($($rest)*);
        };
    }

    debug_all! {
        utc_datetime!(2021-001 0:00);
        Duration::ZERO;
        IndeterminateOffset;
        ConversionRange;
        TryFromParsed::InsufficientInformation;
        Parsed::new();
        #[expect(deprecated)]
        Instant::now();
        error::ParseFromDescription::InvalidComponent("foo");
        error::Format::InvalidComponent("foo");
        well_known::Rfc2822;
        well_known::Rfc3339;
        well_known::Iso8601::DEFAULT;
        well_known::iso8601::FormattedComponents::None;
        well_known::iso8601::DateKind::Calendar;
        well_known::iso8601::TimePrecision::Hour { decimal_digits: None };
        well_known::iso8601::OffsetPrecision::Hour;
        well_known::iso8601::Config::DEFAULT;
        component_range_error();
        Error::ConversionRange(ConversionRange);

        modifier::Day::default();
        modifier::MonthRepr::default();
        modifier::Month::default();
        modifier::Ordinal::default();
        modifier::WeekdayRepr::default();
        modifier::Weekday::default();
        modifier::WeekNumberRepr::default();
        modifier::WeekNumber::default();
        modifier::YearRepr::default();
        modifier::Year::default();
        modifier::Hour::default();
        modifier::Minute::default();
        modifier::Period::default();
        modifier::Second::default();
        modifier::SubsecondDigits::default();
        modifier::Subsecond::default();
        modifier::OffsetHour::default();
        modifier::OffsetMinute::default();
        modifier::OffsetSecond::default();
        modifier::Padding::default();

        BorrowedFormatItem::Literal(b"abcdef");
        BorrowedFormatItem::Compound(&[BorrowedFormatItem::Component(Component::Day(modifier::Day::default()))]);
        BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(&[]));
        BorrowedFormatItem::First(&[]);
        OwnedFormatItem::from(BorrowedFormatItem::Literal(b"abcdef"));
        OwnedFormatItem::from(BorrowedFormatItem::Compound(&[BorrowedFormatItem::Component(Component::Day(modifier::Day::default()))]));
        OwnedFormatItem::from(BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(&[])));
        OwnedFormatItem::from(BorrowedFormatItem::First(&[]));
    }
}
