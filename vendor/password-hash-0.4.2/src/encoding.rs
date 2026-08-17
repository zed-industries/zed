//! Base64 encoding variants.

use base64ct::{
    Base64Bcrypt, Base64Crypt, Base64Unpadded as B64, Encoding as _, Error as B64Error,
};

/// Base64 encoding variants.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Encoding {
    /// "B64" encoding: standard Base64 without padding.
    ///
    /// ```text
    /// [A-Z]      [a-z]      [0-9]      +     /
    /// 0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2b, 0x2f
    /// ```
    /// <https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#b64>
    B64,

    /// bcrypt encoding.
    ///
    /// ```text
    /// ./         [A-Z]      [a-z]     [0-9]
    /// 0x2e-0x2f, 0x41-0x5a, 0x61-0x7a, 0x30-0x39
    /// ```
    Bcrypt,

    /// `crypt(3)` encoding.
    ///
    /// ```text
    /// [.-9]      [A-Z]      [a-z]
    /// 0x2e-0x39, 0x41-0x5a, 0x61-0x7a
    /// ```
    Crypt,
}

impl Default for Encoding {
    fn default() -> Self {
        Self::B64
    }
}

impl Encoding {
    /// Decode a Base64 string into the provided destination buffer.
    pub fn decode(self, src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], B64Error> {
        match self {
            Self::B64 => B64::decode(src, dst),
            Self::Bcrypt => Base64Bcrypt::decode(src, dst),
            Self::Crypt => Base64Crypt::decode(src, dst),
        }
    }

    /// Encode the input byte slice as Base64.
    ///
    /// Writes the result into the provided destination slice, returning an
    /// ASCII-encoded Base64 string value.
    pub fn encode<'a>(self, src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, B64Error> {
        match self {
            Self::B64 => B64::encode(src, dst),
            Self::Bcrypt => Base64Bcrypt::encode(src, dst),
            Self::Crypt => Base64Crypt::encode(src, dst),
        }
        .map_err(Into::into)
    }

    /// Get the length of Base64 produced by encoding the given bytes.
    pub fn encoded_len(self, bytes: &[u8]) -> usize {
        match self {
            Self::B64 => B64::encoded_len(bytes),
            Self::Bcrypt => Base64Bcrypt::encoded_len(bytes),
            Self::Crypt => Base64Crypt::encoded_len(bytes),
        }
    }
}
