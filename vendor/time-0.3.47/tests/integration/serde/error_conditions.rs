use serde::{Deserialize, Serialize, Serializer};
use serde_test::{assert_ser_tokens_error, Token};
use time::macros::{datetime, format_description};
use time::{error, OffsetDateTime};

/// Trigger `time::error::Format::StdIo` errors.
///
/// `StdIo` won't be reached during normal serde operation: it's instantiated
/// only during calls to `format_into()`, but most `Serializable`
/// implementations will only call `format()` because serde `Serializer`
/// interface doesn't expose the underlying `io::Write`.
///
/// Therefore, we need a contrived serializer to trigger coverage.
fn serialize<S: Serializer>(datetime: &OffsetDateTime, _serializer: S) -> Result<S::Ok, S::Error> {
    Err(datetime
        .format_into(
            &mut [0u8; 0].as_mut_slice(),
            format_description!("nonempty format description"),
        )
        .map_err(error::Format::into_invalid_serde_value::<S>)
        .expect_err("Writing to a zero-length buffer should always error."))
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TestBadIo {
    #[serde(serialize_with = "serialize")]
    dt: OffsetDateTime,
}

#[test]
fn custom_serialize_io_error() {
    let value = TestBadIo {
        dt: datetime!(2000-01-01 00:00 -4:00),
    };
    assert_ser_tokens_error::<TestBadIo>(
        &value,
        &[
            Token::Struct {
                name: "TestBadIo",
                len: 1,
            },
            Token::Str("dt"),
        ],
        "failed to write whole buffer",
    );
}
