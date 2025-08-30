//! Encoding and decoding utilities using the `encoding` crate.
use std::fmt::Debug;

use anyhow::{Error, Result};
use encoding::Encoding;
use serde::{Deserialize, de::Visitor};

/// A wrapper around `encoding::Encoding` to implement `Send` and `Sync`.
/// Since the reference is static, it is safe to send it across threads.
pub struct EncodingWrapper(&'static dyn Encoding);

impl Debug for EncodingWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EncodingWrapper")
            .field(&self.0.name())
            .finish()
    }
}

pub struct EncodingWrapperVisitor;

impl<'vi> Visitor<'vi> for EncodingWrapperVisitor {
    type Value = EncodingWrapper;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid encoding name")
    }

    fn visit_str<E: serde::de::Error>(self, encoding: &str) -> Result<EncodingWrapper, E> {
        Ok(EncodingWrapper(
            encoding::label::encoding_from_whatwg_label(encoding)
                .ok_or_else(|| serde::de::Error::custom("Invalid Encoding"))?,
        ))
    }

    fn visit_string<E: serde::de::Error>(self, encoding: String) -> Result<EncodingWrapper, E> {
        Ok(EncodingWrapper(
            encoding::label::encoding_from_whatwg_label(&encoding)
                .ok_or_else(|| serde::de::Error::custom("Invalid Encoding"))?,
        ))
    }
}

impl<'de> Deserialize<'de> for EncodingWrapper {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EncodingWrapperVisitor)
    }
}

impl PartialEq for EncodingWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.name() == other.0.name()
    }
}

unsafe impl Send for EncodingWrapper {}
unsafe impl Sync for EncodingWrapper {}

impl Clone for EncodingWrapper {
    fn clone(&self) -> Self {
        EncodingWrapper(self.0)
    }
}

impl EncodingWrapper {
    pub fn new(encoding: &'static dyn Encoding) -> EncodingWrapper {
        EncodingWrapper(encoding)
    }

    pub async fn decode(&self, input: Vec<u8>) -> Result<String> {
        match self.0.decode(&input, encoding::DecoderTrap::Replace) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::msg(e.to_string())),
        }
    }

    pub async fn encode(&self, input: String) -> Result<Vec<u8>> {
        match self.0.encode(&input, encoding::EncoderTrap::Replace) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::msg(e.to_string())),
        }
    }
}

/// Convert a byte vector from a specified encoding to a UTF-8 string.
pub async fn to_utf8(input: Vec<u8>, encoding: EncodingWrapper) -> Result<String> {
    encoding.decode(input).await
}

/// Convert a UTF-8 string to a byte vector in a specified encoding.
pub async fn from_utf8(input: String, target: EncodingWrapper) -> Result<Vec<u8>> {
    target.encode(input).await
}
