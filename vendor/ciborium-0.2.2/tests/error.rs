// SPDX-License-Identifier: Apache-2.0

use ciborium::{
    de::{from_reader, Error},
    ser::into_writer,
    value::Value,
};
use rstest::rstest;

#[rstest(bytes, error,
    // Invalid value
    case("1e", Error::Syntax(0)),

    // Indeterminate integers are invalid
    case("1f", Error::Syntax(0)),

    // Indeterminate integer in an array
    case("83011f03", Error::Syntax(2)),

    // Integer in a string continuation
    case("7F616101FF", Error::Syntax(3)),

    // Bytes in a string continuation
    case("7F61614101FF", Error::Syntax(3)),

    // Invalid UTF-8
    case("62C328", Error::Syntax(0)),

    // Invalid UTF-8 in a string continuation
    case("7F62C328FF", Error::Syntax(1)),
)]
fn test(bytes: &str, error: Error<std::io::Error>) {
    let bytes = hex::decode(bytes).unwrap();

    let correct = match error {
        Error::Io(..) => panic!(),
        Error::Syntax(x) => ("syntax", Some(x), None),
        Error::Semantic(x, y) => ("semantic", x, Some(y)),
        Error::RecursionLimitExceeded => panic!(),
    };

    let result: Result<Value, _> = from_reader(&bytes[..]);
    let actual = match result.unwrap_err() {
        Error::Io(..) => panic!(),
        Error::Syntax(x) => ("syntax", Some(x), None),
        Error::Semantic(x, y) => ("semantic", x, Some(y)),
        Error::RecursionLimitExceeded => panic!(),
    };

    assert_eq!(correct, actual);
}

#[test]
fn test_long_utf8_deserialization() {
    let s = (0..2000).map(|_| 'ボ').collect::<String>();
    let mut v = Vec::new();
    into_writer(&s, &mut v).unwrap();
    let _: String = from_reader(&*v).unwrap();
}
