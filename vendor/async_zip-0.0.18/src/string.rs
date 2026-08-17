// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::error::{Result, ZipError};

/// A string encoding supported by this crate.
#[derive(Debug, Clone, Copy)]
pub enum StringEncoding {
    Utf8,
    Raw,
}

/// A string wrapper for handling different encodings.
#[derive(Debug, Clone)]
pub struct ZipString {
    encoding: StringEncoding,
    raw: Vec<u8>,
    alternative: Option<Vec<u8>>,
}

impl ZipString {
    /// Constructs a new encoded string from its raw bytes and its encoding type.
    ///
    /// # Note
    /// If the provided encoding is [`StringEncoding::Utf8`] but the raw bytes are not valid UTF-8 (ie. a call to
    /// `std::str::from_utf8()` fails), the encoding is defaulted back to [`StringEncoding::Raw`].
    pub fn new(raw: Vec<u8>, mut encoding: StringEncoding) -> Self {
        if let StringEncoding::Utf8 = encoding {
            if std::str::from_utf8(&raw).is_err() {
                encoding = StringEncoding::Raw;
            }
        }

        Self { encoding, raw, alternative: None }
    }

    /// Constructs a new encoded string from utf-8 data, with an alternative in native MBCS encoding.
    pub fn new_with_alternative(utf8: String, alternative: Vec<u8>) -> Self {
        Self { encoding: StringEncoding::Utf8, raw: utf8.into_bytes(), alternative: Some(alternative) }
    }

    /// Returns the raw bytes for this string.
    pub fn as_bytes(&self) -> &[u8] {
        &self.raw
    }

    /// Returns the encoding type for this string.
    pub fn encoding(&self) -> StringEncoding {
        self.encoding
    }

    /// Returns the alternative bytes (in native MBCS encoding) for this string.
    pub fn alternative(&self) -> Option<&[u8]> {
        self.alternative.as_deref()
    }

    /// Returns the raw bytes converted into a string slice.
    ///
    /// # Note
    /// A call to this method will only succeed if the encoding type is [`StringEncoding::Utf8`].
    pub fn as_str(&self) -> Result<&str> {
        if !matches!(self.encoding, StringEncoding::Utf8) {
            return Err(ZipError::StringNotUtf8);
        }

        // SAFETY:
        // "The bytes passed in must be valid UTF-8.'
        //
        // This function will error if self.encoding is not StringEncoding::Utf8.
        //
        // self.encoding is only ever StringEncoding::Utf8 if this variant was provided to the constructor AND the
        // call to `std::str::from_utf8()` within the constructor succeeded. Mutable access to the inner vector is
        // never given and no method implemented on this type mutates the inner vector.

        Ok(unsafe { std::str::from_utf8_unchecked(&self.raw) })
    }

    /// Returns the raw bytes converted to an owned string.
    ///
    /// # Note
    /// A call to this method will only succeed if the encoding type is [`StringEncoding::Utf8`].
    pub fn into_string(self) -> Result<String> {
        if !matches!(self.encoding, StringEncoding::Utf8) {
            return Err(ZipError::StringNotUtf8);
        }

        // SAFETY: See above.
        Ok(unsafe { String::from_utf8_unchecked(self.raw) })
    }

    /// Returns the alternative bytes (in native MBCS encoding) converted to the owned.
    pub fn into_alternative(self) -> Option<Vec<u8>> {
        self.alternative
    }

    /// Returns whether this string is encoded as utf-8 without an alternative.
    pub fn is_utf8_without_alternative(&self) -> bool {
        matches!(self.encoding, StringEncoding::Utf8) && self.alternative.is_none()
    }
}

impl From<String> for ZipString {
    fn from(value: String) -> Self {
        Self { encoding: StringEncoding::Utf8, raw: value.into_bytes(), alternative: None }
    }
}

impl From<&str> for ZipString {
    fn from(value: &str) -> Self {
        Self { encoding: StringEncoding::Utf8, raw: value.as_bytes().to_vec(), alternative: None }
    }
}
