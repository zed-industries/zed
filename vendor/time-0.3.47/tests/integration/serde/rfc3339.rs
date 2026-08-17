use serde::{Deserialize, Serialize};
use serde_test::{
    assert_de_tokens_error, assert_ser_tokens_error, assert_tokens, Configure, Token,
};
use time::macros::datetime;
use time::serde::rfc3339;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Test {
    #[serde(with = "rfc3339")]
    dt: OffsetDateTime,
    #[serde(with = "rfc3339::option")]
    option_dt: Option<OffsetDateTime>,
}

#[test]
fn serialize_deserialize() {
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
        option_dt: Some(datetime!(2000-01-01 00:00:00 UTC)),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("2000-01-01T00:00:00Z"),
            Token::Str("option_dt"),
            Token::Some,
            Token::BorrowedStr("2000-01-01T00:00:00Z"),
            Token::StructEnd,
        ],
    );
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
        option_dt: None,
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("2000-01-01T00:00:00Z"),
            Token::Str("option_dt"),
            Token::None,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<Test>(
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("bad"),
            Token::StructEnd,
        ],
        "the 'year' component could not be parsed",
    );
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 +00:00:01),
        option_dt: None,
    };
    assert_ser_tokens_error::<Test>(
        &value,
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
        ],
        "The offset_second component cannot be formatted into the requested format.",
    );
}

#[test]
fn parse_json() -> serde_json::Result<()> {
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    #[serde(untagged)]
    enum Wrapper {
        A(Test),
    }
    assert_eq!(
        serde_json::from_str::<Wrapper>("{\"dt\": \"2000-01-01T00:00:00Z\", \"option_dt\": null}")?,
        Wrapper::A(Test {
            dt: datetime!(2000-01-01 00:00:00 UTC),
            option_dt: None,
        })
    );

    Ok(())
}

#[test]
fn issue_479() -> serde_json::Result<()> {
    const A: &str = r#"{
        "date": "2022-05-01T10:20:42.123Z"
    }"#;

    const B: &str = r#"{
        "date": "2022-05-01T10:20:42.123Z",
        "maybe_date": "2022-05-01T10:20:42.123Z"
    }"#;

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct S {
        #[serde(with = "time::serde::rfc3339")]
        date: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339::option", default)]
        maybe_date: Option<OffsetDateTime>,
    }

    let a = serde_json::from_str::<S>(A)?;
    let b = serde_json::from_str::<S>(B)?;

    assert_eq!(
        a,
        S {
            date: datetime!(2022-05-01 10:20:42.123 UTC),
            maybe_date: None
        }
    );
    assert_eq!(
        b,
        S {
            date: datetime!(2022-05-01 10:20:42.123 UTC),
            maybe_date: Some(datetime!(2022-05-01 10:20:42.123 UTC))
        }
    );

    Ok(())
}
