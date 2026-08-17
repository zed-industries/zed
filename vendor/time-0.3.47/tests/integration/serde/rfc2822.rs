use serde::{Deserialize, Serialize};
use serde_test::{assert_tokens, Configure, Token};
use time::serde::rfc2822;
use time::OffsetDateTime;
use time_macros::datetime;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Test {
    #[serde(with = "rfc2822")]
    dt: OffsetDateTime,
    #[serde(with = "rfc2822::option")]
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
            Token::BorrowedStr("Sat, 01 Jan 2000 00:00:00 +0000"),
            Token::Str("option_dt"),
            Token::Some,
            Token::BorrowedStr("Sat, 01 Jan 2000 00:00:00 +0000"),
            Token::StructEnd,
        ],
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
        serde_json::from_str::<Wrapper>(
            r#"{"dt": "Sat, 01 Jan 2000 00:00:00 +0000", "option_dt": null}"#
        )?,
        Wrapper::A(Test {
            dt: datetime!(2000-01-01 00:00:00 UTC),
            option_dt: None,
        })
    );

    Ok(())
}
