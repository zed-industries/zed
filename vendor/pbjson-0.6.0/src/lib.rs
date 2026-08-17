//! `pbjson` is a set of crates to automatically generate [`serde::Serialize`] and
//! [`serde::Deserialize`] implementations for [prost][1] generated structs that
//! are compliant with the [protobuf JSON mapping][2]
//!
//! See [pbjson-build][3] for usage instructions
//!
//! [1]: https://github.com/tokio-rs/prost
//! [2]: https://developers.google.com/protocol-buffers/docs/proto3#json
//! [3]: https://docs.rs/pbjson-build
//!
#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls, rust_2018_idioms)]
#![warn(
    missing_debug_implementations,
    clippy::explicit_iter_loop,
    clippy::use_self,
    clippy::clone_on_ref_ptr,
    clippy::future_not_send
)]

#[doc(hidden)]
pub mod private {
    /// Re-export base64
    pub use base64;

    use base64::engine::DecodePaddingMode;
    use base64::engine::{GeneralPurpose, GeneralPurposeConfig};
    use base64::Engine;
    use serde::de::Visitor;
    use serde::Deserialize;
    use std::borrow::Cow;
    use std::str::FromStr;

    /// Used to parse a number from either a string or its raw representation
    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Ord, Eq)]
    pub struct NumberDeserialize<T>(pub T);

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Content<'a, T> {
        #[serde(borrow)]
        Str(Cow<'a, str>),
        Number(T),
    }

    impl<'de, T> serde::Deserialize<'de> for NumberDeserialize<T>
    where
        T: FromStr + serde::Deserialize<'de>,
        <T as FromStr>::Err: std::error::Error,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let content = Content::deserialize(deserializer)?;
            Ok(Self(match content {
                Content::Str(v) => v.parse().map_err(serde::de::Error::custom)?,
                Content::Number(v) => v,
            }))
        }
    }

    struct Base64Visitor;

    impl<'de> Visitor<'de> for Base64Visitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a base64 string")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            const INDIFFERENT_PAD: GeneralPurposeConfig = GeneralPurposeConfig::new()
                .with_decode_padding_mode(DecodePaddingMode::Indifferent);
            const STANDARD_INDIFFERENT_PAD: GeneralPurpose =
                GeneralPurpose::new(&base64::alphabet::STANDARD, INDIFFERENT_PAD);
            const URL_SAFE_INDIFFERENT_PAD: GeneralPurpose =
                GeneralPurpose::new(&base64::alphabet::URL_SAFE, INDIFFERENT_PAD);

            let decoded = STANDARD_INDIFFERENT_PAD
                .decode(s)
                .or_else(|e| match e {
                    // Either standard or URL-safe base64 encoding are accepted
                    //
                    // The difference being URL-safe uses `-` and `_` instead of `+` and `/`
                    //
                    // Therefore if we error out on those characters, try again with
                    // the URL-safe character set
                    base64::DecodeError::InvalidByte(_, c) if c == b'-' || c == b'_' => {
                        URL_SAFE_INDIFFERENT_PAD.decode(s)
                    }
                    _ => Err(e),
                })
                .map_err(serde::de::Error::custom)?;
            Ok(decoded)
        }
    }

    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Ord, Eq)]
    pub struct BytesDeserialize<T>(pub T);

    impl<'de, T> Deserialize<'de> for BytesDeserialize<T>
    where
        T: From<Vec<u8>>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Ok(Self(deserializer.deserialize_str(Base64Visitor)?.into()))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use base64::Engine;
        use bytes::Bytes;
        use rand::prelude::*;
        use serde::de::value::{BorrowedStrDeserializer, Error};

        #[test]
        fn test_bytes() {
            for _ in 0..20 {
                let mut rng = thread_rng();
                let len = rng.gen_range(50..100);
                let raw: Vec<_> = std::iter::from_fn(|| Some(rng.gen())).take(len).collect();

                for config in [
                    base64::engine::general_purpose::STANDARD,
                    base64::engine::general_purpose::STANDARD_NO_PAD,
                    base64::engine::general_purpose::URL_SAFE,
                    base64::engine::general_purpose::URL_SAFE_NO_PAD,
                ] {
                    let encoded = config.encode(&raw);

                    let deserializer = BorrowedStrDeserializer::<'_, Error>::new(&encoded);
                    let a: Bytes = BytesDeserialize::deserialize(deserializer).unwrap().0;
                    let b: Vec<u8> = BytesDeserialize::deserialize(deserializer).unwrap().0;

                    assert_eq!(raw.as_slice(), &a);
                    assert_eq!(raw.as_slice(), &b);
                }
            }
        }
    }
}
