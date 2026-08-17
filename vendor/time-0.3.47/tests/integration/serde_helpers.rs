use serde::{Deserialize, Serialize};
use serde_test::{Token, assert_de_tokens_error, assert_ser_tokens_error, assert_tokens};
use time::OffsetDateTime;
use time::macros::datetime;

#[test]
fn success() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Full(
        #[serde(with = "time::serde::rfc2822")] OffsetDateTime,
        #[serde(with = "time::serde::rfc2822::option")] Option<OffsetDateTime>,
        #[serde(with = "time::serde::rfc2822::option")] Option<OffsetDateTime>,
        #[serde(with = "time::serde::rfc3339")] OffsetDateTime,
        #[serde(with = "time::serde::rfc3339::option")] Option<OffsetDateTime>,
        #[serde(with = "time::serde::rfc3339::option")] Option<OffsetDateTime>,
        #[serde(with = "time::serde::timestamp")] OffsetDateTime,
        #[serde(with = "time::serde::timestamp::option")] Option<OffsetDateTime>,
        #[serde(with = "time::serde::timestamp::option")] Option<OffsetDateTime>,
    );

    assert_tokens(
        &Full(
            datetime!(2021-01-02 03:04:05 UTC),
            Some(datetime!(2021-01-02 03:04:05 UTC)),
            None,
            datetime!(2021-01-02 03:04:05 UTC),
            Some(datetime!(2021-01-02 03:04:05 UTC)),
            None,
            datetime!(2021-01-02 03:04:05 UTC),
            Some(datetime!(2021-01-02 03:04:05 UTC)),
            None,
        ),
        &[
            Token::TupleStruct {
                name: "Full",
                len: 9,
            },
            Token::Str("Sat, 02 Jan 2021 03:04:05 +0000"),
            Token::Some,
            Token::Str("Sat, 02 Jan 2021 03:04:05 +0000"),
            Token::None,
            Token::Str("2021-01-02T03:04:05Z"),
            Token::Some,
            Token::Str("2021-01-02T03:04:05Z"),
            Token::None,
            Token::I64(1_609_556_645),
            Token::Some,
            Token::I64(1_609_556_645),
            Token::None,
            Token::TupleStructEnd,
        ],
    );
}

#[test]
fn rfc2822_error() {
    #[derive(Serialize, Deserialize)]
    struct Rfc2822(
        #[serde(with = "time::serde::rfc2822")] OffsetDateTime,
        #[serde(with = "time::serde::rfc2822::option")] Option<OffsetDateTime>,
        #[serde(with = "time::serde::rfc2822::option")] Option<OffsetDateTime>,
    );

    assert_ser_tokens_error(
        &Rfc2822(
            datetime!(2021-01-02 03:04:05 +0:00:01),
            Some(datetime!(2021-01-02 03:04:05 UTC)),
            None,
        ),
        &[Token::TupleStruct {
            name: "Rfc2822",
            len: 3,
        }],
        "The offset_second component cannot be formatted into the requested format.",
    );
    assert_ser_tokens_error(
        &Rfc2822(
            datetime!(2021-01-02 03:04:05 UTC),
            Some(datetime!(2021-01-02 03:04:05 +0:00:01)),
            None,
        ),
        &[
            Token::TupleStruct {
                name: "Rfc2822",
                len: 3,
            },
            Token::Str("Sat, 02 Jan 2021 03:04:05 +0000"),
        ],
        "The offset_second component cannot be formatted into the requested format.",
    );
    assert_de_tokens_error::<Rfc2822>(
        &[
            Token::TupleStruct {
                name: "Rfc2822",
                len: 3,
            },
            Token::Bool(false),
        ],
        "invalid type: boolean `false`, expected an RFC2822-formatted `OffsetDateTime`",
    );
    assert_de_tokens_error::<Rfc2822>(
        &[
            Token::TupleStruct {
                name: "Rfc2822",
                len: 3,
            },
            Token::Str("Sat, 02 Jan 2021 03:04:05 +0000"),
            Token::Bool(false),
        ],
        "invalid type: boolean `false`, expected an RFC2822-formatted `Option<OffsetDateTime>`",
    );
    assert_de_tokens_error::<Rfc2822>(
        &[
            Token::TupleStruct {
                name: "Rfc2822",
                len: 3,
            },
            Token::Str("Sat, 02 Jan 2021 03:04:05 +0000"),
            Token::Some,
            Token::Bool(false),
        ],
        "invalid type: boolean `false`, expected an RFC2822-formatted `OffsetDateTime`",
    );
}

#[test]
fn rfc3339_error() {
    #[derive(Serialize, Deserialize)]
    struct Rfc3339(
        #[serde(with = "time::serde::rfc3339")] OffsetDateTime,
        #[serde(with = "time::serde::rfc3339::option")] Option<OffsetDateTime>,
        #[serde(with = "time::serde::rfc3339::option")] Option<OffsetDateTime>,
    );

    assert_ser_tokens_error(
        &Rfc3339(
            datetime!(2021-01-02 03:04:05 +0:00:01),
            Some(datetime!(2021-01-02 03:04:05 UTC)),
            None,
        ),
        &[Token::TupleStruct {
            name: "Rfc3339",
            len: 3,
        }],
        "The offset_second component cannot be formatted into the requested format.",
    );
    assert_ser_tokens_error(
        &Rfc3339(
            datetime!(2021-01-02 03:04:05 UTC),
            Some(datetime!(2021-01-02 03:04:05 +0:00:01)),
            None,
        ),
        &[
            Token::TupleStruct {
                name: "Rfc3339",
                len: 3,
            },
            Token::Str("2021-01-02T03:04:05Z"),
        ],
        "The offset_second component cannot be formatted into the requested format.",
    );
    assert_de_tokens_error::<Rfc3339>(
        &[
            Token::TupleStruct {
                name: "Rfc3339",
                len: 3,
            },
            Token::Bool(false),
        ],
        "invalid type: boolean `false`, expected an RFC3339-formatted `OffsetDateTime`",
    );
    assert_de_tokens_error::<Rfc3339>(
        &[
            Token::TupleStruct {
                name: "Rfc3339",
                len: 3,
            },
            Token::Str("2021-01-02T03:04:05Z"),
            Token::Bool(false),
        ],
        "invalid type: boolean `false`, expected an RFC3339-formatted `Option<OffsetDateTime>`",
    );
    assert_de_tokens_error::<Rfc3339>(
        &[
            Token::TupleStruct {
                name: "Rfc3339",
                len: 3,
            },
            Token::Str("2021-01-02T03:04:05Z"),
            Token::Some,
            Token::Bool(false),
        ],
        "invalid type: boolean `false`, expected an RFC3339-formatted `OffsetDateTime`",
    );
}

#[test]
fn timestamp_error() {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    struct Timestamp(#[serde(with = "time::serde::timestamp")] OffsetDateTime);

    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    struct OptTimestamp(#[serde(with = "time::serde::timestamp::option")] Option<OffsetDateTime>);

    assert_de_tokens_error::<Timestamp>(
        &[Token::Bool(false)],
        "invalid type: boolean `false`, expected i64",
    );
    assert_de_tokens_error::<OptTimestamp>(
        &[Token::Some, Token::Bool(false)],
        "invalid type: boolean `false`, expected i64",
    );
    assert_de_tokens_error::<Timestamp>(
        &[Token::I64(100_000_000_000_000)],
        "invalid timestamp, expected an in-range value",
    );
    assert_de_tokens_error::<OptTimestamp>(
        &[Token::Some, Token::I64(-100_000_000_000_000)],
        "invalid timestamp, expected an in-range value",
    );
}
