use anyhow::{Error, Result};

use encoding::Encoding;

/// A wrapper around `encoding::Encoding` to implement `Send` and `Sync`.
/// Since the reference is static, it is safe to send it across threads.
pub struct EncodingWrapper(&'static dyn Encoding);

unsafe impl Send for EncodingWrapper {}
unsafe impl Sync for EncodingWrapper {}

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
pub async fn to_utf8<'a>(input: Vec<u8>, encoding: EncodingWrapper) -> Result<String> {
    Ok(encoding.decode(input).await?)
}

/// Convert a UTF-8 string to a byte vector in a specified encoding.
pub async fn from_utf8<'a>(input: String, target: EncodingWrapper) -> Result<Vec<u8>> {
    Ok(target.encode(input).await?)
}
