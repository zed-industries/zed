// Copyright (c) The camino Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Serde implementations for the types in this crate.
//!
//! * `Utf8Path` is an unsized type which the derive impls can't handle.
//! * `Utf8PathBuf` could be derived, but we don't depend on serde_derive to
//!   improve compile times. It's also very straightforward to implement.

use crate::{Utf8Path, Utf8PathBuf};
use serde_core::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::fmt;

struct Utf8PathBufVisitor;

impl<'a> de::Visitor<'a> for Utf8PathBufVisitor {
    type Value = Utf8PathBuf;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a UTF-8 path string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.into())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.into())
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        std::str::from_utf8(v)
            .map(Into::into)
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Bytes(v), &self))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        String::from_utf8(v)
            .map(Into::into)
            .map_err(|e| de::Error::invalid_value(de::Unexpected::Bytes(&e.into_bytes()), &self))
    }
}

impl<'de> Deserialize<'de> for Utf8PathBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(Utf8PathBufVisitor)
    }
}

impl Serialize for Utf8PathBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

struct Utf8PathVisitor;

impl<'a> de::Visitor<'a> for Utf8PathVisitor {
    type Value = &'a Utf8Path;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a borrowed UTF-8 path")
    }

    fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.as_ref())
    }

    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        std::str::from_utf8(v)
            .map(AsRef::as_ref)
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Bytes(v), &self))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a Utf8Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Utf8PathVisitor)
    }
}

impl Serialize for Utf8Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Box<Utf8Path> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Utf8PathBuf::deserialize(deserializer)?.into())
    }
}

// impl Serialize for Box<Utf8Path> comes from impl Serialize for Utf8Path.

// Can't provide impls for Arc/Rc due to orphan rule issues, but we could provide
// `with` impls in the future as requested.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Utf8PathBuf;
    use serde::{Deserialize, Serialize};
    use serde_bytes::ByteBuf;

    #[test]
    fn valid_utf8() {
        let valid_utf8 = &["", "bar", "💩"];
        for input in valid_utf8 {
            let encode = Encode {
                path: ByteBuf::from(*input),
            };
            let encoded = bincode::serialize(&encode).expect("encoded correctly");

            assert_valid_utf8::<DecodeOwned>(input, &encoded);
            assert_valid_utf8::<DecodeBorrowed>(input, &encoded);
            assert_valid_utf8::<DecodeBoxed>(input, &encoded);
        }
    }

    fn assert_valid_utf8<'de, T: TestTrait<'de>>(input: &str, encoded: &'de [u8]) {
        let output = bincode::deserialize::<T>(encoded).expect("valid UTF-8 should be fine");
        assert_eq!(
            output.path(),
            input,
            "for input, with {}, paths should match",
            T::description()
        );
        let roundtrip = bincode::serialize(&output).expect("message should roundtrip");
        assert_eq!(roundtrip, encoded, "encoded path matches");
    }

    #[test]
    fn invalid_utf8() {
        let invalid_utf8: &[(&[u8], _, _)] = &[
            (b"\xff", 0, 1),
            (b"foo\xfe", 3, 1),
            (b"a\xC3\xA9 \xED\xA0\xBD\xF0\x9F\x92\xA9", 4, 1),
        ];

        for (input, valid_up_to, error_len) in invalid_utf8 {
            let encode = Encode {
                path: ByteBuf::from(*input),
            };
            let encoded = bincode::serialize(&encode).expect("encoded correctly");

            assert_invalid_utf8::<DecodeOwned>(input, &encoded, *valid_up_to, *error_len);
            assert_invalid_utf8::<DecodeBorrowed>(input, &encoded, *valid_up_to, *error_len);
            assert_invalid_utf8::<DecodeBoxed>(input, &encoded, *valid_up_to, *error_len);
        }
    }

    fn assert_invalid_utf8<'de, T: TestTrait<'de>>(
        input: &[u8],
        encoded: &'de [u8],
        valid_up_to: usize,
        error_len: usize,
    ) {
        let error = bincode::deserialize::<T>(encoded).expect_err("invalid UTF-8 should error out");
        let utf8_error = match *error {
            bincode::ErrorKind::InvalidUtf8Encoding(utf8_error) => utf8_error,
            other => panic!(
                "for input {:?}, with {}, expected ErrorKind::InvalidUtf8Encoding, found: {}",
                input,
                T::description(),
                other
            ),
        };
        assert_eq!(
            utf8_error.valid_up_to(),
            valid_up_to,
            "for input {:?}, with {}, valid_up_to didn't match",
            input,
            T::description(),
        );
        assert_eq!(
            utf8_error.error_len(),
            Some(error_len),
            "for input {:?}, with {}, error_len didn't match",
            input,
            T::description(),
        );
    }

    #[derive(Serialize, Debug)]
    struct Encode {
        path: ByteBuf,
    }

    trait TestTrait<'de>: Serialize + Deserialize<'de> + fmt::Debug {
        fn description() -> &'static str;
        fn path(&self) -> &Utf8Path;
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[allow(unused)]
    struct DecodeOwned {
        path: Utf8PathBuf,
    }

    impl TestTrait<'_> for DecodeOwned {
        fn description() -> &'static str {
            "DecodeOwned"
        }

        fn path(&self) -> &Utf8Path {
            &self.path
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[allow(unused)]
    struct DecodeBorrowed<'a> {
        #[serde(borrow)]
        path: &'a Utf8Path,
    }

    impl<'de> TestTrait<'de> for DecodeBorrowed<'de> {
        fn description() -> &'static str {
            "DecodeBorrowed"
        }

        fn path(&self) -> &Utf8Path {
            self.path
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[allow(unused)]
    struct DecodeBoxed {
        path: Box<Utf8Path>,
    }

    impl TestTrait<'_> for DecodeBoxed {
        fn description() -> &'static str {
            "DecodeBoxed"
        }

        fn path(&self) -> &Utf8Path {
            &self.path
        }
    }
}
