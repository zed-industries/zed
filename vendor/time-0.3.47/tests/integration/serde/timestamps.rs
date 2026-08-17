use serde::{Deserialize, Serialize};
use serde_test::{assert_de_tokens_error, assert_tokens, Configure, Token};
use time::macros::datetime;
use time::serde::timestamp;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Test {
    #[serde(with = "timestamp")]
    dt: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestMilliseconds {
    #[serde(with = "timestamp::milliseconds")]
    dt: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestMicroseconds {
    #[serde(with = "timestamp::microseconds")]
    dt: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestNanoseconds {
    #[serde(with = "timestamp::nanoseconds")]
    dt: OffsetDateTime,
}

#[test]
fn serialize_timestamp() {
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "Test",
                len: 1,
            },
            Token::Str("dt"),
            Token::I64(946684800),
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<Test>(
        &[
            Token::Struct {
                name: "Test",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
}

#[test]
fn serialize_timestamp_milliseconds() -> serde_json::Result<()> {
    let value_milliseconds = TestMilliseconds {
        dt: datetime!(2000-01-01 00:00:00.999 UTC),
    };
    assert_de_tokens_error::<TestMilliseconds>(
        &[
            Token::Struct {
                name: "TestMilliseconds",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i128",
    );
    // serde_test does not support I128, see: https://github.com/serde-rs/test/issues/18
    let milliseconds_str = r#"{"dt":946684800999}"#;
    let deserialized_milliseconds: TestMilliseconds = serde_json::from_str(milliseconds_str)?;
    let serialized_milliseconds = serde_json::to_string(&value_milliseconds)?;
    assert_eq!(value_milliseconds.dt, deserialized_milliseconds.dt);
    assert_eq!(serialized_milliseconds, milliseconds_str);
    Ok(())
}

#[test]
fn serialize_timestamp_microseconds() -> serde_json::Result<()> {
    let value_microseconds = TestMicroseconds {
        dt: datetime!(2000-01-01 00:00:00.999_999 UTC),
    };
    assert_de_tokens_error::<TestMicroseconds>(
        &[
            Token::Struct {
                name: "TestMicroseconds",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i128",
    );
    // serde_test does not support I128, see: https://github.com/serde-rs/test/issues/18
    let microseconds_str = r#"{"dt":946684800999999}"#;
    let deserialized_microseconds: TestMicroseconds = serde_json::from_str(microseconds_str)?;
    let serialized_microseconds = serde_json::to_string(&value_microseconds)?;
    assert_eq!(value_microseconds.dt, deserialized_microseconds.dt);
    assert_eq!(serialized_microseconds, microseconds_str);
    Ok(())
}

#[test]
fn serialize_timestamp_nanoseconds() -> serde_json::Result<()> {
    let value_nanoseconds = TestNanoseconds {
        dt: datetime!(2000-01-01 00:00:00.999_999_999 UTC),
    };
    assert_de_tokens_error::<TestNanoseconds>(
        &[
            Token::Struct {
                name: "TestNanoseconds",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i128",
    );
    // serde_test does not support I128, see: https://github.com/serde-rs/test/issues/18
    let nanoseconds_str = r#"{"dt":946684800999999999}"#;
    let deserialized_nanoseconds: TestNanoseconds = serde_json::from_str(nanoseconds_str)?;
    let serialized_nanoseconds = serde_json::to_string(&value_nanoseconds)?;
    assert_eq!(value_nanoseconds.dt, deserialized_nanoseconds.dt);
    assert_eq!(serialized_nanoseconds, nanoseconds_str);
    Ok(())
}
