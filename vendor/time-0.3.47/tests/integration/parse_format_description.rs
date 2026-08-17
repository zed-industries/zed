use core::num::NonZero;

use rstest::rstest;
use rstest_reuse::{apply, template};
use time::error::InvalidFormatDescription;
use time::format_description::modifier::*;
use time::format_description::{
    self, BorrowedFormatItem, Component, OwnedFormatItem, StaticFormatDescription,
};
use time::macros::format_description;

/// A modifier with its value and string representation.
///
/// This alias is used to avoid repeating the tuple in countless locations.
type M<T> = (T, &'static str);

#[rustfmt::skip] // does not format well
#[template]
#[rstest]
fn modifiers(
    #[values(
        (Padding::Space, "padding:space"),
        (Padding::Zero, "padding:zero"),
        (Padding::None, "padding:none"),
    )]
    padding: _,
    #[values(
        (false, "repr:24"),
        (true, "repr:12"),
    )]
    hour_is_12_hour_clock: _,
    #[values(
        (true, "case:upper"),
        (false, "case:lower"),
    )]
    period_is_uppercase: _,
    #[values(
        (MonthRepr::Numerical, "repr:numerical"),
        (MonthRepr::Long, "repr:long"),
        (MonthRepr::Short, "repr:short"),
    )]
    month_repr: _,
    #[values(
        (SubsecondDigits::One, "digits:1"),
        (SubsecondDigits::Two, "digits:2"),
        (SubsecondDigits::Three, "digits:3"),
        (SubsecondDigits::Four, "digits:4"),
        (SubsecondDigits::Five, "digits:5"),
        (SubsecondDigits::Six, "digits:6"),
        (SubsecondDigits::Seven, "digits:7"),
        (SubsecondDigits::Eight, "digits:8"),
        (SubsecondDigits::Nine, "digits:9"),
        (SubsecondDigits::OneOrMore, "digits:1+"),
    )]
    subsecond_digits: _,
    #[values(
        (WeekdayRepr::Short, "repr:short"),
        (WeekdayRepr::Long, "repr:long"),
        (WeekdayRepr::Sunday, "repr:sunday"),
        (WeekdayRepr::Monday, "repr:monday"),
    )]
    weekday_repr: _,
    #[values(
        (WeekNumberRepr::Iso, "repr:iso"),
        (WeekNumberRepr::Sunday, "repr:sunday"),
        (WeekNumberRepr::Monday, "repr:monday"),
    )]
    week_number_repr: _,
    #[values(
        (YearRepr::Full, "repr:full"),
        (YearRepr::Century, "repr:century"),
        (YearRepr::LastTwo, "repr:last_two"),
    )]
    year_repr: _,
    #[values(
        (YearRange::Standard, "range:standard"),
        (YearRange::Extended, "range:extended"),
    )]
    year_range: _,
    #[values(
        (false, "base:calendar"),
        (true, "base:iso_week"),
    )]
    year_is_iso_week_based: _,
    #[values(
        (false, "sign:automatic"),
        (true, "sign:mandatory"),
    )]
    sign_is_mandatory: _,
    #[values(
        (true, "one_indexed:true"),
        (false, "one_indexed:false"),
    )]
    weekday_is_one_indexed: _,
    #[values(
        (true, "case_sensitive:true"),
        (false, "case_sensitive:false"),
    )]
    case_sensitive: _,
    #[values(
        (NonZero::new(1).unwrap(), "count:1"),
        (NonZero::new(2).unwrap(), "count:2"),
        (NonZero::new(3).unwrap(), "count:3"),
        (NonZero::new(10).unwrap(), "count:10"),
        (NonZero::new(100).unwrap(), "count:100"),
        (NonZero::new(1_000).unwrap(), "count:1000"),
    )]
    ignore_count: _,
    #[values(
        (UnixTimestampPrecision::Second, "precision:second"),
        (UnixTimestampPrecision::Millisecond, "precision:millisecond"),
        (UnixTimestampPrecision::Microsecond, "precision:microsecond"),
        (UnixTimestampPrecision::Nanosecond, "precision:nanosecond"),
    )]
    unix_timestamp_precision: _,
    #[values(
        (TrailingInput::Prohibit, "trailing_input:prohibit"),
        (TrailingInput::Discard, "trailing_input:discard"),
    )]
    trailing_input: _,
) {}

#[rstest]
fn empty() {
    assert_eq!(format_description::parse_borrowed::<2>(""), Ok(vec![]));
    assert_eq!(
        format_description::parse_owned::<2>(""),
        Ok(OwnedFormatItem::Compound(Box::new([])))
    );
}

#[rstest]
#[case("foo bar", [b"foo bar".as_slice()])]
#[case("  leading spaces", [b"  leading spaces".as_slice()])]
#[case("trailing spaces  ", [b"trailing spaces  ".as_slice()])]
#[case("     ", [b"     ".as_slice()])]
#[case("[[", [b"[".as_slice()])]
#[case("foo[[bar", [b"foo".as_slice(), b"[".as_slice(), b"bar".as_slice()])]
fn only_literal<const N: usize>(#[case] format_description: &str, #[case] expected: [&[u8]; N]) {
    assert_eq!(
        format_description::parse(format_description),
        Ok(expected
            .into_iter()
            .map(BorrowedFormatItem::Literal)
            .collect())
    );
}

#[rstest]
#[case("[day]", Component::Day(Day::default()))]
#[case("[end]", Component::End(End::default()))]
#[case("[hour]", Component::Hour(Hour::default()))]
#[case("[minute]", Component::Minute(Minute::default()))]
#[case("[month]", Component::Month(Month::default()))]
#[case("[offset_hour]", Component::OffsetHour(OffsetHour::default()))]
#[case("[offset_minute]", Component::OffsetMinute(OffsetMinute::default()))]
#[case("[offset_second]", Component::OffsetSecond(OffsetSecond::default()))]
#[case("[ordinal]", Component::Ordinal(Ordinal::default()))]
#[case("[period]", Component::Period(Period::default()))]
#[case("[second]", Component::Second(Second::default()))]
#[case("[subsecond]", Component::Subsecond(Subsecond::default()))]
#[case("[unix_timestamp]", Component::UnixTimestamp(UnixTimestamp::default()))]
#[case("[weekday]", Component::Weekday(Weekday::default()))]
#[case("[week_number]", Component::WeekNumber(WeekNumber::default()))]
#[case("[year]", Component::Year(Year::default()))]
fn simple_component(#[case] format_description: &str, #[case] component: Component) {
    assert_eq!(
        format_description::parse(format_description),
        Ok(vec![BorrowedFormatItem::Component(component)])
    );
}

#[rstest]
fn errors() {
    use InvalidFormatDescription::*;

    macro_rules! assert_errs {
        ($($format_description:literal, $error:pat $(if $condition:expr)?,)*) => {$(
            assert!(matches!(
                format_description::parse($format_description),
                Err($error) $(if $condition)?
            ));
            assert!(matches!(
                format_description::parse_owned::<2>($format_description),
                Err($error) $(if $condition)?
            ));
        )*};
    }

    assert_errs! {
        "[ invalid ]", InvalidComponentName { name, index: 2, .. } if name == "invalid",
        "[", MissingComponentName { index: 0, .. },
        "[ ", MissingComponentName { index: 1, .. },
        "[]", MissingComponentName { index: 0, .. },
        "[day sign:mandatory]", InvalidModifier { value, index: 5, .. } if value == "sign",
        "[day sign:]", InvalidModifier { value, index: 9,.. } if value.is_empty(),
        "[day :mandatory]", InvalidModifier { value, index: 5,.. } if value.is_empty(),
        "[day sign:mandatory", UnclosedOpeningBracket { index: 0, .. },
        "[day padding:invalid]", InvalidModifier { value, index: 13, .. } if value == "invalid",
        "[ignore]", MissingRequiredModifier { name: "count", index: 1, .. },
        "[ignore count:70000]", InvalidModifier { value, index: 14, .. } if value == "70000",
    }
}

macro_rules! placeholder {
    ($($x:tt)*) => {
        " {}"
    };
}

macro_rules! parse_with_modifiers {
    ($modifier_name:literal, $($modifier:ident),+) => {
        format_description::parse(
            &format!(
                concat!(
                    "[",
                    $modifier_name,
                    $(placeholder!($modifier),)+
                    "]",
                ),
                $($modifier.1),+
            )
        )
    };
}

#[apply(modifiers)]
fn day_component(padding: M<Padding>) {
    assert_eq!(
        parse_with_modifiers!("day", padding),
        Ok(vec![BorrowedFormatItem::Component(Component::Day(
            Day::default().with_padding(padding.0)
        ))])
    );
}

#[apply(modifiers)]
fn end_component(trailing_input: M<TrailingInput>) {
    assert_eq!(
        parse_with_modifiers!("end", trailing_input),
        Ok(vec![BorrowedFormatItem::Component(Component::End(
            End::default().with_trailing_input(trailing_input.0)
        ))])
    );
}

#[apply(modifiers)]
fn minute_component(padding: M<Padding>) {
    assert_eq!(
        parse_with_modifiers!("minute", padding),
        Ok(vec![BorrowedFormatItem::Component(Component::Minute(
            Minute::default().with_padding(padding.0)
        ))])
    );
}

#[apply(modifiers)]
fn offset_minute_component(padding: M<Padding>) {
    assert_eq!(
        parse_with_modifiers!("offset_minute", padding),
        Ok(vec![BorrowedFormatItem::Component(
            Component::OffsetMinute(OffsetMinute::default().with_padding(padding.0))
        )])
    );
}

#[apply(modifiers)]
fn offset_second_component(padding: M<Padding>) {
    assert_eq!(
        parse_with_modifiers!("offset_second", padding),
        Ok(vec![BorrowedFormatItem::Component(
            Component::OffsetSecond(OffsetSecond::default().with_padding(padding.0))
        )])
    );
}

#[apply(modifiers)]
fn ordinal_component(padding: M<Padding>) {
    assert_eq!(
        parse_with_modifiers!("ordinal", padding),
        Ok(vec![BorrowedFormatItem::Component(Component::Ordinal(
            Ordinal::default().with_padding(padding.0)
        ))])
    );
}

#[apply(modifiers)]
fn second_component(padding: M<Padding>) {
    assert_eq!(
        parse_with_modifiers!("second", padding),
        Ok(vec![BorrowedFormatItem::Component(Component::Second(
            Second::default().with_padding(padding.0)
        ))])
    );
}

#[apply(modifiers)]
fn hour_component(padding: M<Padding>, hour_is_12_hour_clock: M<bool>) {
    assert_eq!(
        parse_with_modifiers!("hour", padding, hour_is_12_hour_clock),
        Ok(vec![BorrowedFormatItem::Component(Component::Hour(
            Hour::default()
                .with_padding(padding.0)
                .with_is_12_hour_clock(hour_is_12_hour_clock.0)
        ))])
    );
}

#[apply(modifiers)]
fn month_component(padding: M<Padding>, case_sensitive: M<bool>, month_repr: M<MonthRepr>) {
    assert_eq!(
        parse_with_modifiers!("month", padding, case_sensitive, month_repr),
        Ok(vec![BorrowedFormatItem::Component(Component::Month(
            Month::default()
                .with_padding(padding.0)
                .with_repr(month_repr.0)
                .with_case_sensitive(case_sensitive.0)
        ))])
    );
}

#[apply(modifiers)]
fn period_component(case_sensitive: M<bool>, period_is_uppercase: M<bool>) {
    assert_eq!(
        parse_with_modifiers!("period", period_is_uppercase, case_sensitive),
        Ok(vec![BorrowedFormatItem::Component(Component::Period(
            Period::default()
                .with_is_uppercase(period_is_uppercase.0)
                .with_case_sensitive(case_sensitive.0)
        ))])
    );
}

#[apply(modifiers)]
fn weekday_component(
    case_sensitive: M<bool>,
    weekday_is_one_indexed: M<bool>,
    weekday_repr: M<WeekdayRepr>,
) {
    assert_eq!(
        parse_with_modifiers!(
            "weekday",
            case_sensitive,
            weekday_is_one_indexed,
            weekday_repr
        ),
        Ok(vec![BorrowedFormatItem::Component(Component::Weekday(
            Weekday::default()
                .with_repr(weekday_repr.0)
                .with_one_indexed(weekday_is_one_indexed.0)
                .with_case_sensitive(case_sensitive.0)
        ))])
    );
}

#[apply(modifiers)]
fn week_number_component(padding: M<Padding>, week_number_repr: M<WeekNumberRepr>) {
    assert_eq!(
        parse_with_modifiers!("week_number", padding, week_number_repr),
        Ok(vec![BorrowedFormatItem::Component(Component::WeekNumber(
            WeekNumber::default()
                .with_padding(padding.0)
                .with_repr(week_number_repr.0)
        ))])
    );
}

#[apply(modifiers)]
fn offset_hour_component(padding: M<Padding>, sign_is_mandatory: M<bool>) {
    assert_eq!(
        parse_with_modifiers!("offset_hour", padding, sign_is_mandatory),
        Ok(vec![BorrowedFormatItem::Component(Component::OffsetHour(
            OffsetHour::default()
                .with_padding(padding.0)
                .with_sign_is_mandatory(sign_is_mandatory.0)
        ))])
    );
}

#[apply(modifiers)]
fn year_component(
    padding: M<Padding>,
    year_repr: M<YearRepr>,
    year_range: M<YearRange>,
    year_is_iso_week_based: M<bool>,
    sign_is_mandatory: M<bool>,
) {
    assert_eq!(
        parse_with_modifiers!(
            "year",
            padding,
            year_repr,
            year_range,
            year_is_iso_week_based,
            sign_is_mandatory
        ),
        Ok(vec![BorrowedFormatItem::Component(Component::Year(
            Year::default()
                .with_padding(padding.0)
                .with_repr(year_repr.0)
                .with_range(year_range.0)
                .with_iso_week_based(year_is_iso_week_based.0)
                .with_sign_is_mandatory(sign_is_mandatory.0)
        ))])
    );
}

#[apply(modifiers)]
fn unix_timestamp_component(
    sign_is_mandatory: M<bool>,
    unix_timestamp_precision: M<UnixTimestampPrecision>,
) {
    assert_eq!(
        parse_with_modifiers!(
            "unix_timestamp",
            sign_is_mandatory,
            unix_timestamp_precision
        ),
        Ok(vec![BorrowedFormatItem::Component(
            Component::UnixTimestamp(
                UnixTimestamp::default()
                    .with_sign_is_mandatory(sign_is_mandatory.0)
                    .with_precision(unix_timestamp_precision.0)
            )
        )])
    );
}

#[apply(modifiers)]
fn subsecond_component(subsecond_digits: M<SubsecondDigits>) {
    assert_eq!(
        parse_with_modifiers!("subsecond", subsecond_digits),
        Ok(vec![BorrowedFormatItem::Component(Component::Subsecond(
            Subsecond::default().with_digits(subsecond_digits.0)
        ))]),
    );
}

#[apply(modifiers)]
fn ignore_component(ignore_count: M<NonZero<u16>>) {
    assert_eq!(
        parse_with_modifiers!("ignore", ignore_count),
        Ok(vec![BorrowedFormatItem::Component(Component::Ignore(
            Ignore::count(ignore_count.0)
        ))])
    );
}

#[rstest]
fn optional() {
    assert_eq!(
        format_description::parse_owned::<2>("[optional [:[year]]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Compound(Box::new([
                OwnedFormatItem::Literal(Box::new(*b":")),
                OwnedFormatItem::Component(Component::Year(Default::default()))
            ]))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>("[optional [[year]]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Component(Component::Year(Default::default()))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [\[]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Literal(Box::new(*b"["))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [ \[ ]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Compound(Box::new([
                OwnedFormatItem::Literal(Box::new(*b" ")),
                OwnedFormatItem::Literal(Box::new(*b"[")),
                OwnedFormatItem::Literal(Box::new(*b" ")),
            ]))
        )))
    );
}

#[rstest]
fn first() {
    assert_eq!(
        format_description::parse_owned::<2>("[first [a]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a"))
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>("[first [a] [b]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a")),
            OwnedFormatItem::Literal(Box::new(*b"b")),
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>("[first [a][b]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a")),
            OwnedFormatItem::Literal(Box::new(*b"b")),
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[first [a][\[]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a")),
            OwnedFormatItem::Literal(Box::new(*b"[")),
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[first [a][\[\[]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a")),
            OwnedFormatItem::Compound(Box::new([
                OwnedFormatItem::Literal(Box::new(*b"[")),
                OwnedFormatItem::Literal(Box::new(*b"[")),
            ]))
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>(
            "[first [[period case:upper]] [[period case:lower]] ]"
        ),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Component(Component::Period(
                Period::default()
                    .with_is_uppercase(true)
                    .with_case_sensitive(true)
            )),
            OwnedFormatItem::Component(Component::Period(
                Period::default()
                    .with_is_uppercase(false)
                    .with_case_sensitive(true)
            )),
        ])))
    );
}

#[rstest]
fn backslash_escape() {
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [\]]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Literal(Box::new(*b"]"))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [\[]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Literal(Box::new(*b"["))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [\\]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Literal(Box::new(*br"\"))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"\\"),
        Ok(OwnedFormatItem::Literal(Box::new(*br"\")))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"\["),
        Ok(OwnedFormatItem::Literal(Box::new(*br"[")))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"\]"),
        Ok(OwnedFormatItem::Literal(Box::new(*br"]")))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"foo\\"),
        Ok(OwnedFormatItem::Compound(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"foo")),
            OwnedFormatItem::Literal(Box::new(*br"\")),
        ])))
    );
    assert_eq!(
        format_description::parse_borrowed::<2>(r"\\"),
        Ok(vec![BorrowedFormatItem::Literal(br"\")])
    );
    assert_eq!(
        format_description::parse_borrowed::<2>(r"\["),
        Ok(vec![BorrowedFormatItem::Literal(br"[")])
    );
    assert_eq!(
        format_description::parse_borrowed::<2>(r"\]"),
        Ok(vec![BorrowedFormatItem::Literal(br"]")])
    );
    assert_eq!(
        format_description::parse_borrowed::<2>(r"foo\\"),
        Ok(vec![
            BorrowedFormatItem::Literal(b"foo"),
            BorrowedFormatItem::Literal(br"\"),
        ])
    );
}

#[rstest]
#[case(r"\a", 1)]
#[case(r"\", 0)]
fn backslash_escape_error(#[case] format_description: &str, #[case] expected_index: usize) {
    assert!(matches!(
        format_description::parse_owned::<2>(format_description),
        Err(InvalidFormatDescription::Expected {
            what: "valid escape sequence",
            index,
            ..
        }) if index == expected_index
    ));
    assert!(matches!(
        format_description::parse_borrowed::<2>(format_description),
        Err(InvalidFormatDescription::Expected {
            what: "valid escape sequence",
            index,
            ..
        }) if index == expected_index
    ));
}

#[rstest]
fn nested_v1_error() {
    assert!(matches!(
        format_description::parse_owned::<2>("[optional [[[]]"),
        Err(InvalidFormatDescription::MissingComponentName { index: 11, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional [ [[ ]]"),
        Err(InvalidFormatDescription::MissingComponentName { index: 12, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[first [a][[[]]"),
        Err(InvalidFormatDescription::UnclosedOpeningBracket { index: 0, .. })
    ));
}

#[rstest]
fn nested_error() {
    use InvalidFormatDescription::*;

    assert!(matches!(
        format_description::parse("[optional []]"),
        Err(NotSupported {
            what: "optional item",
            context: "runtime-parsed format descriptions",
            index: 0,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse("[first []]"),
        Err(NotSupported {
            what: "'first' item",
            context: "runtime-parsed format descriptions",
            index: 0,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[year [month]]"),
        Err(InvalidModifier { value, index: 6, .. }) if value == "["
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional[]]"),
        Err(Expected {
            what: "whitespace after `optional`",
            index: 8,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[first[]]"),
        Err(Expected {
            what: "whitespace after `first`",
            index: 5,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional []"),
        Err(UnclosedOpeningBracket { index: 0, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[first []"),
        Err(UnclosedOpeningBracket { index: 0, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional ["),
        Err(UnclosedOpeningBracket { index: 10, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional [[year"),
        Err(UnclosedOpeningBracket { index: 11, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional "),
        Err(Expected {
            what: "opening bracket",
            index: 9,
            ..
        })
    ));
}

#[rstest]
#[case("[", "missing component name at byte index 0")]
#[case("[foo", "unclosed opening bracket at byte index 0")]
#[case("[foo]", "invalid component name `foo` at byte index 1")]
#[case("[day bar]", "invalid modifier `bar` at byte index 5")]
#[case("[]", "missing component name at byte index 0")]
#[case(
    "[optional []]",
    "optional item is not supported in runtime-parsed format descriptions at byte index 0"
)]
#[case(
    "[ignore]",
    "missing required modifier `count` for component at byte index 1"
)]
fn error_display(#[case] format_description: &str, #[case] error: &str) {
    // la10736/rstest#217
    #[expect(clippy::unwrap_used, reason = "purpose of the test")]
    let test = || {
        assert_eq!(
            format_description::parse(format_description)
                .unwrap_err()
                .to_string(),
            error
        );
    };

    test();
}

#[rstest]
#[case("[optional ", "expected opening bracket at byte index 9")]
fn error_display_owned(#[case] format_description: &str, #[case] error: &str) {
    // la10736/rstest#217
    #[expect(clippy::unwrap_used, reason = "purpose of the test")]
    let test = || {
        assert_eq!(
            format_description::parse_owned::<2>(format_description)
                .unwrap_err()
                .to_string(),
            error
        )
    };

    test();
}

#[rstest]
fn rfc_3339() {
    assert_eq!(
        format_description::parse(
            "[year]-[month repr:numerical]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour \
             sign:mandatory]:[offset_minute]"
        ),
        Ok(vec![
            BorrowedFormatItem::Component(Component::Year(
                Year::default()
                    .with_padding(Padding::Zero)
                    .with_repr(YearRepr::Full)
                    .with_iso_week_based(false)
                    .with_sign_is_mandatory(false)
            )),
            BorrowedFormatItem::Literal(b"-"),
            BorrowedFormatItem::Component(Component::Month(
                Month::default().with_padding(Padding::Zero)
            )),
            BorrowedFormatItem::Literal(b"-"),
            BorrowedFormatItem::Component(Component::Day(
                Day::default().with_padding(Padding::Zero)
            )),
            BorrowedFormatItem::Literal(b"T"),
            BorrowedFormatItem::Component(Component::Hour(
                Hour::default()
                    .with_padding(Padding::Zero)
                    .with_is_12_hour_clock(false)
            )),
            BorrowedFormatItem::Literal(b":"),
            BorrowedFormatItem::Component(Component::Minute(
                Minute::default().with_padding(Padding::Zero)
            )),
            BorrowedFormatItem::Literal(b":"),
            BorrowedFormatItem::Component(Component::Second(
                Second::default().with_padding(Padding::Zero)
            )),
            BorrowedFormatItem::Literal(b"."),
            BorrowedFormatItem::Component(Component::Subsecond(
                Subsecond::default().with_digits(SubsecondDigits::OneOrMore)
            )),
            BorrowedFormatItem::Component(Component::OffsetHour(
                OffsetHour::default()
                    .with_padding(Padding::Zero)
                    .with_sign_is_mandatory(true)
            )),
            BorrowedFormatItem::Literal(b":"),
            BorrowedFormatItem::Component(Component::OffsetMinute(
                OffsetMinute::default().with_padding(Padding::Zero)
            ))
        ])
    );
}

#[rstest]
#[case("foo", format_description!("foo"))]
#[case("%a", format_description!("[weekday repr:short]"))]
#[case("%A", format_description!("[weekday]"))]
#[case("%b", format_description!("[month repr:short]"))]
#[case("%B", format_description!("[month repr:long]"))]
#[case("%C", format_description!("[year repr:century]"))]
#[case("%d", format_description!("[day]"))]
#[case("%e", format_description!("[day padding:space]"))]
#[case("%g", format_description!("[year repr:last_two base:iso_week]"))]
#[case("%G", format_description!("[year base:iso_week]"))]
#[case("%h", format_description!("[month repr:short]"))]
#[case("%H", format_description!("[hour]"))]
#[case("%I", format_description!("[hour repr:12]"))]
#[case("%j", format_description!("[ordinal]"))]
#[case("%k", format_description!("[hour padding:space]"))]
#[case("%l", format_description!("[hour repr:12 padding:space]"))]
#[case("%m", format_description!("[month]"))]
#[case("%M", format_description!("[minute]"))]
#[case("%n", format_description!("\n"))]
#[case("%p", format_description!("[period]"))]
#[case("%P", format_description!("[period case:lower]"))]
#[case("%s", format_description!("[unix_timestamp]"))]
#[case("%S", format_description!("[second]"))]
#[case("%t", format_description!("\t"))]
#[case("%u", format_description!("[weekday repr:monday]"))]
#[case("%U", format_description!("[week_number repr:sunday]"))]
#[case("%V", format_description!("[week_number]"))]
#[case("%w", format_description!("[weekday repr:sunday]"))]
#[case("%W", format_description!("[week_number repr:monday]"))]
#[case("%y", format_description!("[year repr:last_two]"))]
#[case("%Y", format_description!("[year]"))]
#[case("%%", format_description!("%"))]
fn strftime_equivalence(
    #[case] strftime: &str,
    #[case] custom: StaticFormatDescription,
) -> time::Result<()> {
    let borrowed = format_description::parse_strftime_borrowed(strftime)?;
    let owned = format_description::parse_strftime_owned(strftime)?;

    assert_eq!(borrowed, custom);
    assert_eq!(owned, OwnedFormatItem::from(custom));

    Ok(())
}

#[rstest]
#[case(
    "%c",
    "[weekday repr:short] [month repr:short] [day padding:space] [hour]:[minute]:[second] [year]"
)]
#[case("%D", "[month]/[day]/[year repr:last_two]")]
#[case("%F", "[year]-[month repr:numerical]-[day]")]
#[case("%r", "[hour repr:12]:[minute]:[second] [period]")]
#[case("%R", "[hour]:[minute]")]
#[case("%T", "[hour]:[minute]:[second]")]
#[case("%x", "[month]/[day]/[year repr:last_two]")]
#[case("%X", "[hour]:[minute]:[second]")]
#[case("%z", "[offset_hour sign:mandatory][offset_minute]")]
fn strftime_compound_equivalence(#[case] strftime: &str, #[case] custom: &str) -> time::Result<()> {
    let borrowed = format_description::parse_strftime_borrowed(strftime)?;
    let owned = format_description::parse_strftime_owned(strftime)?;
    let custom = format_description::parse(custom)?;
    // Until equality is implemented better, we need to convert to a compound.
    let custom = vec![BorrowedFormatItem::Compound(&custom)];

    assert_eq!(borrowed, custom);
    assert_eq!(owned, OwnedFormatItem::from(custom));

    Ok(())
}
