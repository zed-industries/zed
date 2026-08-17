use serde_test::{
    Compact, Configure, Readable, Token, assert_de_tokens, assert_de_tokens_error, assert_tokens,
};
use time::macros::{date, datetime, offset, time};
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

mod error_conditions;
mod iso8601;
mod json;
mod macros;
mod rfc2822;
mod rfc3339;
mod timestamps;

#[test]
fn time() {
    assert_tokens(
        &Time::MIDNIGHT.compact(),
        &[
            Token::Tuple { len: 4 },
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &time!(23:58:59.123_456_789).compact(),
        &[
            Token::Tuple { len: 4 },
            Token::U8(23),
            Token::U8(58),
            Token::U8(59),
            Token::U32(123_456_789),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &Time::MIDNIGHT.readable(),
        &[Token::BorrowedStr("00:00:00.0")],
    );
    assert_tokens(
        &time!(23:58:59.123_456_789).readable(),
        &[Token::BorrowedStr("23:58:59.123456789")],
    );
}

#[test]
fn time_error() {
    assert_de_tokens_error::<Compact<Time>>(
        &[
            Token::Tuple { len: 4 },
            Token::U8(24),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::TupleEnd,
        ],
        "invalid hour, expected an in-range value",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::BorrowedStr("24:00:00.0")],
        "the 'hour' component could not be parsed",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::BorrowedStr("23-00:00.0")],
        "a character literal was not valid",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::BorrowedStr("0:00:00.0")],
        "the 'hour' component could not be parsed",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::BorrowedStr("00:00:00.0x")],
        "unexpected trailing characters; the end of input was expected",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Time`",
    );
    assert_de_tokens_error::<Compact<Time>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Time`",
    );
}

#[test]
fn time_partial() {
    assert_de_tokens_error::<Compact<Time>>(
        &[Token::Tuple { len: 4 }, Token::TupleEnd],
        "expected hour",
    );
    assert_de_tokens_error::<Compact<Time>>(
        &[Token::Tuple { len: 4 }, Token::U8(0), Token::TupleEnd],
        "expected minute",
    );
    assert_de_tokens_error::<Compact<Time>>(
        &[
            Token::Tuple { len: 4 },
            Token::U8(0),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected second",
    );
    assert_de_tokens_error::<Compact<Time>>(
        &[
            Token::Tuple { len: 4 },
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected nanosecond",
    );

    assert_de_tokens_error::<Readable<Time>>(
        &[Token::Tuple { len: 4 }, Token::TupleEnd],
        "expected hour",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::Tuple { len: 4 }, Token::U8(0), Token::TupleEnd],
        "expected minute",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[
            Token::Tuple { len: 4 },
            Token::U8(0),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected second",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[
            Token::Tuple { len: 4 },
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected nanosecond",
    );
}

#[test]
fn date() {
    assert_tokens(
        &date!(-9999-001).compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I32(-9999),
            Token::U16(1),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &date!(+9999-365).compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I32(9999),
            Token::U16(365),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &date!(-9999-001).readable(),
        &[Token::BorrowedStr("-9999-01-01")],
    );
    assert_tokens(
        &date!(+9999-365).readable(),
        &[Token::BorrowedStr("9999-12-31")],
    );
}

#[test]
fn date_error() {
    assert_de_tokens_error::<Readable<Date>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Date`",
    );
    assert_de_tokens_error::<Compact<Date>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Date`",
    );
}

#[test]
fn date_partial() {
    assert_de_tokens_error::<Compact<Date>>(
        &[Token::Tuple { len: 2 }, Token::TupleEnd],
        "expected year",
    );
    assert_de_tokens_error::<Compact<Date>>(
        &[Token::Tuple { len: 2 }, Token::I32(9999), Token::TupleEnd],
        "expected day of year",
    );

    assert_de_tokens_error::<Readable<Date>>(
        &[Token::Tuple { len: 2 }, Token::TupleEnd],
        "expected year",
    );
    assert_de_tokens_error::<Readable<Date>>(
        &[Token::Tuple { len: 2 }, Token::I32(9999), Token::TupleEnd],
        "expected day of year",
    );
}

#[test]
fn primitive_date_time() {
    assert_tokens(
        &datetime!(-9999-001 0:00).compact(),
        &[
            Token::Tuple { len: 6 },
            Token::I32(-9999),
            Token::U16(1),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &datetime!(+9999-365 23:58:59.123_456_789).compact(),
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(23),
            Token::U8(58),
            Token::U8(59),
            Token::U32(123_456_789),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &datetime!(-9999-001 0:00).readable(),
        &[Token::BorrowedStr("-9999-01-01 00:00:00.0")],
    );
    assert_tokens(
        &datetime!(+9999-365 23:58:59.123_456_789).readable(),
        &[Token::BorrowedStr("9999-12-31 23:58:59.123456789")],
    );
}

#[test]
fn primitive_date_time_error() {
    assert_de_tokens_error::<Readable<PrimitiveDateTime>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `PrimitiveDateTime`",
    );
    assert_de_tokens_error::<Compact<PrimitiveDateTime>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `PrimitiveDateTime`",
    );
    assert_de_tokens_error::<Compact<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(2021),
            Token::U16(366),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::TupleEnd,
        ],
        "invalid ordinal, expected an in-range value",
    );
    assert_de_tokens_error::<Compact<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(2021),
            Token::U16(1),
            Token::U8(24),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::TupleEnd,
        ],
        "invalid hour, expected an in-range value",
    );
}

#[test]
fn primitive_date_time_partial() {
    assert_de_tokens_error::<Compact<PrimitiveDateTime>>(
        &[Token::Tuple { len: 6 }, Token::TupleEnd],
        "expected year",
    );
    assert_de_tokens_error::<Compact<PrimitiveDateTime>>(
        &[Token::Tuple { len: 6 }, Token::I32(9999), Token::TupleEnd],
        "expected day of year",
    );
    assert_de_tokens_error::<Compact<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::TupleEnd,
        ],
        "expected hour",
    );
    assert_de_tokens_error::<Compact<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(23),
            Token::TupleEnd,
        ],
        "expected minute",
    );
    assert_de_tokens_error::<Compact<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(23),
            Token::U8(58),
            Token::TupleEnd,
        ],
        "expected second",
    );
    assert_de_tokens_error::<Compact<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(23),
            Token::U8(58),
            Token::U8(59),
            Token::TupleEnd,
        ],
        "expected nanosecond",
    );

    assert_de_tokens_error::<Readable<PrimitiveDateTime>>(
        &[Token::Tuple { len: 6 }, Token::TupleEnd],
        "expected year",
    );
    assert_de_tokens_error::<Readable<PrimitiveDateTime>>(
        &[Token::Tuple { len: 6 }, Token::I32(9999), Token::TupleEnd],
        "expected day of year",
    );
    assert_de_tokens_error::<Readable<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::TupleEnd,
        ],
        "expected hour",
    );
    assert_de_tokens_error::<Readable<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(23),
            Token::TupleEnd,
        ],
        "expected minute",
    );
    assert_de_tokens_error::<Readable<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(23),
            Token::U8(58),
            Token::TupleEnd,
        ],
        "expected second",
    );
    assert_de_tokens_error::<Readable<PrimitiveDateTime>>(
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(23),
            Token::U8(58),
            Token::U8(59),
            Token::TupleEnd,
        ],
        "expected nanosecond",
    );
}

#[test]
fn offset_date_time() {
    assert_tokens(
        &datetime!(-9999-001 0:00 UTC)
            .to_offset(offset!(+23:58:59))
            .compact(),
        &[
            Token::Tuple { len: 9 },
            Token::I32(-9999),
            Token::U16(1),
            Token::U8(23),
            Token::U8(58),
            Token::U8(59),
            Token::U32(0),
            Token::I8(23),
            Token::I8(58),
            Token::I8(59),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &datetime!(+9999-365 23:58:59.123_456_789 UTC)
            .to_offset(offset!(-23:58:59))
            .compact(),
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(123_456_789),
            Token::I8(-23),
            Token::I8(-58),
            Token::I8(-59),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &datetime!(-9999-001 0:00 UTC)
            .to_offset(offset!(+23:58:59))
            .readable(),
        &[Token::BorrowedStr("-9999-01-01 23:58:59.0 +23:58:59")],
    );
    assert_tokens(
        &datetime!(+9999-365 23:58:59.123_456_789 UTC)
            .to_offset(offset!(-23:58:59))
            .readable(),
        &[Token::BorrowedStr(
            "9999-12-31 00:00:00.123456789 -23:58:59",
        )],
    );
}

#[test]
fn offset_date_time_error() {
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected an `OffsetDateTime`",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected an `OffsetDateTime`",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(2021),
            Token::U16(366),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::I8(0),
            Token::I8(0),
            Token::I8(0),
            Token::TupleEnd,
        ],
        "invalid ordinal, expected an in-range value",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(2021),
            Token::U16(1),
            Token::U8(24),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::I8(0),
            Token::I8(0),
            Token::I8(0),
            Token::TupleEnd,
        ],
        "invalid hour, expected an in-range value",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(2021),
            Token::U16(1),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::I8(26),
            Token::I8(0),
            Token::I8(0),
            Token::TupleEnd,
        ],
        "invalid offset hour, expected an in-range value",
    );
    // the Deserialize impl does not recognize leap second times as valid
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(2021),
            Token::U16(365),
            Token::U8(23),
            Token::U8(59),
            Token::U8(60),
            Token::U32(0),
            Token::I8(0),
            Token::I8(0),
            Token::I8(0),
            Token::TupleEnd,
        ],
        "invalid second, expected an in-range value",
    );
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[Token::BorrowedStr("2021-12-31 23:59:60.0 +00:00:00")],
        "second was not in range",
    );
}

#[test]
fn offset_date_time_partial() {
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[Token::Tuple { len: 9 }, Token::TupleEnd],
        "expected year",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[Token::Tuple { len: 9 }, Token::I32(9999), Token::TupleEnd],
        "expected day of year",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::TupleEnd,
        ],
        "expected hour",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected minute",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected second",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected nanosecond",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(123_456_789),
            Token::TupleEnd,
        ],
        "expected offset hours",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(123_456_789),
            Token::I8(-23),
            Token::TupleEnd,
        ],
        "expected offset minutes",
    );
    assert_de_tokens_error::<Compact<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(123_456_789),
            Token::I8(-23),
            Token::I8(-58),
            Token::TupleEnd,
        ],
        "expected offset seconds",
    );

    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[Token::Tuple { len: 9 }, Token::TupleEnd],
        "expected year",
    );
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[Token::Tuple { len: 9 }, Token::I32(9999), Token::TupleEnd],
        "expected day of year",
    );
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::TupleEnd,
        ],
        "expected hour",
    );
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected minute",
    );
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected second",
    );
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::TupleEnd,
        ],
        "expected nanosecond",
    );
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(123_456_789),
            Token::TupleEnd,
        ],
        "expected offset hours",
    );
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(123_456_789),
            Token::I8(-23),
            Token::TupleEnd,
        ],
        "expected offset minutes",
    );
    assert_de_tokens_error::<Readable<OffsetDateTime>>(
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(123_456_789),
            Token::I8(-23),
            Token::I8(-58),
            Token::TupleEnd,
        ],
        "expected offset seconds",
    );
}

#[test]
fn utc_offset() {
    assert_tokens(
        &offset!(-23:58:59).compact(),
        &[
            Token::Tuple { len: 3 },
            Token::I8(-23),
            Token::I8(-58),
            Token::I8(-59),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &offset!(+23:58:59).compact(),
        &[
            Token::Tuple { len: 3 },
            Token::I8(23),
            Token::I8(58),
            Token::I8(59),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &offset!(-23:58:59).readable(),
        &[Token::BorrowedStr("-23:58:59")],
    );
    assert_tokens(
        &offset!(+23:58:59).readable(),
        &[Token::BorrowedStr("+23:58:59")],
    );
}

#[test]
fn utc_offset_error() {
    assert_de_tokens_error::<Readable<UtcOffset>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `UtcOffset`",
    );
    assert_de_tokens_error::<Compact<UtcOffset>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `UtcOffset`",
    );
    assert_de_tokens_error::<Compact<UtcOffset>>(
        &[
            Token::Tuple { len: 3 },
            Token::I8(26),
            Token::I8(0),
            Token::I8(0),
            Token::TupleEnd,
        ],
        "invalid offset hour, expected an in-range value",
    );
}

#[test]
fn utc_offset_partial() {
    assert_de_tokens_error::<Compact<UtcOffset>>(
        &[Token::Tuple { len: 0 }, Token::TupleEnd],
        "expected offset hours",
    );
    assert_de_tokens_error::<Readable<UtcOffset>>(
        &[Token::Tuple { len: 0 }, Token::TupleEnd],
        "expected offset hours",
    );

    let value = offset!(+23);
    assert_de_tokens::<Compact<UtcOffset>>(
        &value.compact(),
        &[Token::Tuple { len: 1 }, Token::I8(23), Token::TupleEnd],
    );
    let value = offset!(+23);
    assert_de_tokens::<Readable<UtcOffset>>(
        &value.readable(),
        &[Token::Tuple { len: 1 }, Token::I8(23), Token::TupleEnd],
    );

    let value = offset!(+23:58);
    assert_de_tokens::<Compact<UtcOffset>>(
        &value.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I8(23),
            Token::I8(58),
            Token::TupleEnd,
        ],
    );
    let value = offset!(+23:58);
    assert_de_tokens::<Readable<UtcOffset>>(
        &value.readable(),
        &[
            Token::Tuple { len: 2 },
            Token::I8(23),
            Token::I8(58),
            Token::TupleEnd,
        ],
    );
}

#[test]
fn duration() {
    assert_tokens(
        &Duration::MIN.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I64(i64::MIN),
            Token::I32(-999_999_999),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &Duration::MAX.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I64(i64::MAX),
            Token::I32(999_999_999),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &Duration::MIN.readable(),
        &[Token::BorrowedStr("-9223372036854775808.999999999")],
    );
    assert_tokens(
        &Duration::MAX.readable(),
        &[Token::BorrowedStr("9223372036854775807.999999999")],
    );
    assert_tokens(
        &Duration::ZERO.readable(),
        &[Token::BorrowedStr("0.000000000")],
    );
    assert_tokens(
        &Duration::nanoseconds(123).readable(),
        &[Token::BorrowedStr("0.000000123")],
    );
    assert_tokens(
        &Duration::nanoseconds(-123).readable(),
        &[Token::BorrowedStr("-0.000000123")],
    );
}

#[test]
fn duration_error() {
    assert_de_tokens_error::<Readable<Duration>>(
        &[Token::BorrowedStr("x")],
        r#"invalid value: string "x", expected a decimal point"#,
    );
    assert_de_tokens_error::<Readable<Duration>>(
        &[Token::BorrowedStr("x.0")],
        r#"invalid value: string "x", expected seconds"#,
    );
    assert_de_tokens_error::<Readable<Duration>>(
        &[Token::BorrowedStr("0.x")],
        r#"invalid value: string "x", expected nanoseconds"#,
    );
    assert_de_tokens_error::<Readable<Duration>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Duration`",
    );
    assert_de_tokens_error::<Compact<Duration>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Duration`",
    );
}

#[test]
fn duration_partial() {
    assert_de_tokens_error::<Compact<Duration>>(
        &[Token::Tuple { len: 2 }, Token::TupleEnd],
        "expected seconds",
    );
    assert_de_tokens_error::<Compact<Duration>>(
        &[
            Token::Tuple { len: 2 },
            Token::I64(i64::MAX),
            Token::TupleEnd,
        ],
        "expected nanoseconds",
    );

    assert_de_tokens_error::<Readable<Duration>>(
        &[Token::Tuple { len: 2 }, Token::TupleEnd],
        "expected seconds",
    );
    assert_de_tokens_error::<Readable<Duration>>(
        &[
            Token::Tuple { len: 2 },
            Token::I64(i64::MAX),
            Token::TupleEnd,
        ],
        "expected nanoseconds",
    );
}

#[test]
fn weekday() {
    use Weekday::*;
    assert_tokens(&Monday.compact(), &[Token::U8(1)]);
    assert_tokens(&Tuesday.compact(), &[Token::U8(2)]);
    assert_tokens(&Wednesday.compact(), &[Token::U8(3)]);
    assert_tokens(&Thursday.compact(), &[Token::U8(4)]);
    assert_tokens(&Friday.compact(), &[Token::U8(5)]);
    assert_tokens(&Saturday.compact(), &[Token::U8(6)]);
    assert_tokens(&Sunday.compact(), &[Token::U8(7)]);

    assert_tokens(&Monday.readable(), &[Token::BorrowedStr("Monday")]);
    assert_tokens(&Tuesday.readable(), &[Token::BorrowedStr("Tuesday")]);
    assert_tokens(&Wednesday.readable(), &[Token::BorrowedStr("Wednesday")]);
    assert_tokens(&Thursday.readable(), &[Token::BorrowedStr("Thursday")]);
    assert_tokens(&Friday.readable(), &[Token::BorrowedStr("Friday")]);
    assert_tokens(&Saturday.readable(), &[Token::BorrowedStr("Saturday")]);
    assert_tokens(&Sunday.readable(), &[Token::BorrowedStr("Sunday")]);
}

#[test]
fn weekday_error() {
    assert_de_tokens_error::<Compact<Weekday>>(
        &[Token::U8(0)],
        "invalid value: integer `0`, expected a value in the range 1..=7",
    );
    assert_de_tokens_error::<Readable<Weekday>>(
        &[Token::BorrowedStr("NotADay")],
        r#"invalid value: string "NotADay", expected a `Weekday`"#,
    );
    assert_de_tokens_error::<Readable<Weekday>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Weekday`",
    );
    assert_de_tokens_error::<Compact<Weekday>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Weekday`",
    );
}

#[test]
fn month() {
    use Month::*;
    assert_tokens(&January.compact(), &[Token::U8(1)]);
    assert_tokens(&February.compact(), &[Token::U8(2)]);
    assert_tokens(&March.compact(), &[Token::U8(3)]);
    assert_tokens(&April.compact(), &[Token::U8(4)]);
    assert_tokens(&May.compact(), &[Token::U8(5)]);
    assert_tokens(&June.compact(), &[Token::U8(6)]);
    assert_tokens(&July.compact(), &[Token::U8(7)]);
    assert_tokens(&August.compact(), &[Token::U8(8)]);
    assert_tokens(&September.compact(), &[Token::U8(9)]);
    assert_tokens(&October.compact(), &[Token::U8(10)]);
    assert_tokens(&November.compact(), &[Token::U8(11)]);
    assert_tokens(&December.compact(), &[Token::U8(12)]);

    assert_tokens(&January.readable(), &[Token::BorrowedStr("January")]);
    assert_tokens(&February.readable(), &[Token::BorrowedStr("February")]);
    assert_tokens(&March.readable(), &[Token::BorrowedStr("March")]);
    assert_tokens(&April.readable(), &[Token::BorrowedStr("April")]);
    assert_tokens(&May.readable(), &[Token::BorrowedStr("May")]);
    assert_tokens(&June.readable(), &[Token::BorrowedStr("June")]);
    assert_tokens(&July.readable(), &[Token::BorrowedStr("July")]);
    assert_tokens(&August.readable(), &[Token::BorrowedStr("August")]);
    assert_tokens(&September.readable(), &[Token::BorrowedStr("September")]);
    assert_tokens(&October.readable(), &[Token::BorrowedStr("October")]);
    assert_tokens(&November.readable(), &[Token::BorrowedStr("November")]);
    assert_tokens(&December.readable(), &[Token::BorrowedStr("December")]);
}

#[test]
fn month_error() {
    assert_de_tokens_error::<Compact<Month>>(
        &[Token::U8(0)],
        "invalid value: integer `0`, expected a value in the range 1..=12",
    );
    assert_de_tokens_error::<Readable<Month>>(
        &[Token::BorrowedStr("NotAMonth")],
        r#"invalid value: string "NotAMonth", expected a `Month`"#,
    );
    assert_de_tokens_error::<Readable<Month>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Month`",
    );
    assert_de_tokens_error::<Compact<Month>>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected a `Month`",
    );
}
