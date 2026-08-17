#![cfg(feature = "bytes")]

use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::str;

use bytes::Bytes;

/// Thin wrapper around `Bytes` which guarantees that bytes are valid UTF-8 string.
/// Should be API-compatible to `String`.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Chars(Bytes);

impl Chars {
    /// New empty object.
    pub const fn new() -> Chars {
        Chars(Bytes::new())
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Try convert from `Bytes`
    pub fn from_bytes(bytes: Bytes) -> Result<Chars, str::Utf8Error> {
        str::from_utf8(&bytes)?;

        Ok(Chars(bytes))
    }

    /// Len in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Self-explanatory
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> From<&'a str> for Chars {
    fn from(src: &'a str) -> Chars {
        Chars(Bytes::copy_from_slice(src.as_bytes()))
    }
}

impl From<String> for Chars {
    fn from(src: String) -> Chars {
        Chars(Bytes::from(src))
    }
}

impl Into<String> for Chars {
    fn into(self) -> String {
        // This is safe because `Chars` is guaranteed to store a valid UTF-8 string
        unsafe { String::from_utf8_unchecked(self.0.as_ref().to_owned()) }
    }
}

impl Default for Chars {
    fn default() -> Self {
        Chars::new()
    }
}

impl Deref for Chars {
    type Target = str;

    fn deref(&self) -> &str {
        // This is safe because `Chars` is guaranteed to store a valid UTF-8 string
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}

impl Borrow<str> for Chars {
    fn borrow(&self) -> &str {
        &*self
    }
}

impl fmt::Display for Chars {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl fmt::Debug for Chars {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(test)]
mod test {
    use super::Chars;

    #[test]
    #[cfg_attr(miri, ignore)] // bytes violates SB, see https://github.com/tokio-rs/bytes/issues/522
    fn test_display_and_debug() {
        let s = "test";
        let string: String = s.into();
        let chars: Chars = s.into();

        assert_eq!(format!("{}", string), format!("{}", chars));
        assert_eq!(format!("{:?}", string), format!("{:?}", chars));
    }
}
