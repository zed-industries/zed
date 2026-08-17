/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! UTF-8 string byte buffer representation with validation amortization.

use bytes::Bytes;
use std::str::Utf8Error;

/// UTF-8 string byte buffer representation with validation amortization.
/// When `StrBytes` is constructed from a `&str` or `String`, its underlying bytes are assumed
/// to be valid UTF-8. Otherwise, if constructed from a byte source, the construction will
/// be fallible.
///
/// Example construction from a `&str`:
/// ```rust
/// use aws_smithy_types::str_bytes::StrBytes;
///
/// let value: StrBytes = "example".into();
/// assert_eq!("example", value.as_str());
/// assert_eq!(b"example", &value.as_bytes()[..]);
/// ```
///
/// Example construction from `Bytes`:
/// ```rust
/// use bytes::Bytes;
/// use aws_smithy_types::str_bytes::StrBytes;
///
/// let bytes = Bytes::from_static(b"example");
/// let value: StrBytes = bytes.try_into().expect("valid utf-8");
/// assert_eq!("example", value.as_str());
/// assert_eq!(b"example", &value.as_bytes()[..]);
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StrBytes {
    bytes: Bytes,
}

impl StrBytes {
    fn new(bytes: Bytes) -> Self {
        StrBytes { bytes }
    }

    /// Returns the underlying `Bytes` representation.
    pub fn as_bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Returns the `StrBytes` value as a `&str`.
    pub fn as_str(&self) -> &str {
        // Safety: StrBytes can only be constructed from a valid UTF-8 string
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..]) }
    }

    /// Tries to create a `StrBytes` from a slice, or returns a `Utf8Error` if the slice
    /// is not valid UTF-8.
    pub fn try_copy_from_slice(slice: &[u8]) -> Result<Self, Utf8Error> {
        match std::str::from_utf8(slice) {
            Ok(_) => Ok(StrBytes::new(Bytes::copy_from_slice(slice))),
            Err(err) => Err(err),
        }
    }

    /// Creates a `StrBytes` from a `&str`.
    pub fn copy_from_str(string: &str) -> Self {
        StrBytes::new(Bytes::copy_from_slice(string.as_bytes()))
    }
}

impl From<String> for StrBytes {
    fn from(value: String) -> Self {
        StrBytes::new(Bytes::from(value))
    }
}

impl From<&'static str> for StrBytes {
    fn from(value: &'static str) -> Self {
        StrBytes::new(Bytes::from(value))
    }
}

impl TryFrom<&'static [u8]> for StrBytes {
    type Error = Utf8Error;

    fn try_from(value: &'static [u8]) -> Result<Self, Self::Error> {
        match std::str::from_utf8(value) {
            Ok(_) => Ok(StrBytes::new(Bytes::from(value))),
            Err(err) => Err(err),
        }
    }
}

impl TryFrom<Vec<u8>> for StrBytes {
    type Error = Utf8Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match std::str::from_utf8(&value[..]) {
            Ok(_) => Ok(StrBytes::new(Bytes::from(value))),
            Err(err) => Err(err),
        }
    }
}

impl TryFrom<Bytes> for StrBytes {
    type Error = Utf8Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        match std::str::from_utf8(&bytes[..]) {
            Ok(_) => Ok(StrBytes::new(bytes)),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str_bytes::StrBytes;
    use bytes::Bytes;
    use std::str::Utf8Error;

    #[test]
    fn invalid_utf8_correctly_errors() {
        let invalid_utf8 = &[0xC3, 0x28][..];
        assert!(std::str::from_utf8(invalid_utf8).is_err());

        let result: Result<StrBytes, Utf8Error> = invalid_utf8.try_into();
        assert!(result.is_err());

        let result: Result<StrBytes, Utf8Error> = invalid_utf8.to_vec().try_into();
        assert!(result.is_err());

        let result: Result<StrBytes, Utf8Error> = Bytes::from_static(invalid_utf8).try_into();
        assert!(result.is_err());
    }

    #[test]
    fn valid_utf8() {
        let valid_utf8 = "hello";
        let str_bytes: StrBytes = valid_utf8.into();
        assert_eq!(valid_utf8.as_bytes(), str_bytes.as_bytes());
        assert_eq!(valid_utf8, str_bytes.as_str());
        assert_eq!(valid_utf8, str_bytes.as_str());
    }

    #[test]
    fn equals() {
        let str_bytes: StrBytes = "test".into();
        assert_eq!(str_bytes, str_bytes);
        let other_bytes: StrBytes = "foo".into();
        assert_ne!(str_bytes, other_bytes);
    }
}
