use core::slice;
use std::borrow::Cow;
use std::fmt;
use std::mem::take;
use std::ops::{Bound, RangeBounds};

use nucleo_matcher::{chars, Utf32Str};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Utf32String {
    /// A string represented as ASCII encoded bytes.
    /// Correctness invariant: must only contain valid ASCII (<=127)
    Ascii(Box<str>),
    /// A string represented as an array of unicode codepoints (basically UTF-32).
    Unicode(Box<[char]>),
}

impl Default for Utf32String {
    fn default() -> Self {
        Self::Ascii(String::new().into_boxed_str())
    }
}
impl Utf32String {
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Utf32String::Unicode(codepoints) => codepoints.len(),
            Utf32String::Ascii(ascii_bytes) => ascii_bytes.len(),
        }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Utf32String::Unicode(codepoints) => codepoints.is_empty(),
            Utf32String::Ascii(ascii_bytes) => ascii_bytes.is_empty(),
        }
    }

    /// Same as `slice` but accepts a u32 range for convenience since
    /// those are the indices returned by the matcher
    #[inline]
    pub fn slice(&self, range: impl RangeBounds<u32>) -> Utf32Str {
        let start = match range.start_bound() {
            Bound::Included(&start) => start as usize,
            Bound::Excluded(&start) => start as usize + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end as usize + 1,
            Bound::Excluded(&end) => end as usize,
            Bound::Unbounded => self.len(),
        };
        match self {
            Utf32String::Ascii(bytes) => Utf32Str::Ascii(&bytes.as_bytes()[start..end]),
            Utf32String::Unicode(codepoints) => Utf32Str::Unicode(&codepoints[start..end]),
        }
    }

    #[inline]
    pub fn is_ascii(&self) -> bool {
        matches!(self, Utf32String::Ascii(_))
    }

    #[inline]
    pub fn get(&self, idx: u32) -> char {
        match self {
            Utf32String::Ascii(bytes) => bytes.as_bytes()[idx as usize] as char,
            Utf32String::Unicode(codepoints) => codepoints[idx as usize],
        }
    }

    #[inline]
    pub fn last(&self) -> char {
        match self {
            Utf32String::Ascii(bytes) => bytes.as_bytes()[bytes.len() - 1] as char,
            Utf32String::Unicode(codepoints) => codepoints[codepoints.len() - 1],
        }
    }

    #[inline]
    pub fn chars(&self) -> Chars<'_> {
        match self {
            Utf32String::Ascii(bytes) => Chars::Ascii(bytes.as_bytes().iter()),
            Utf32String::Unicode(codepoints) => Chars::Unicode(codepoints.iter()),
        }
    }

    #[inline]
    pub fn push_str(&mut self, text: &str) {
        let mut codeboints = match take(self) {
            Utf32String::Ascii(bytes) if text.is_ascii() => {
                let mut bytes = bytes.into_string();
                bytes.push_str(text);
                *self = Self::Ascii(bytes.into_boxed_str());
                return;
            }
            Utf32String::Ascii(bytes) => bytes.bytes().map(|c| c as char).collect(),
            Utf32String::Unicode(codepoints) => Vec::from(codepoints),
        };
        codeboints.extend(chars::graphemes(text));
        *self = Utf32String::Unicode(codeboints.into_boxed_slice());
    }

    #[inline]
    pub fn push(&mut self, c: char) {
        let mut codeboints = match take(self) {
            Utf32String::Ascii(bytes) if c.is_ascii() => {
                let mut bytes = bytes.into_string();
                bytes.push(c);
                *self = Self::Ascii(bytes.into_boxed_str());
                return;
            }
            Utf32String::Ascii(bytes) => bytes.bytes().map(|c| c as char).collect(),
            Utf32String::Unicode(codepoints) => Vec::from(codepoints),
        };
        codeboints.push(c);
        *self = Utf32String::Unicode(codeboints.into_boxed_slice());
    }
}

impl From<&str> for Utf32String {
    #[inline]
    fn from(value: &str) -> Self {
        if value.is_ascii() {
            Self::Ascii(value.to_owned().into_boxed_str())
        } else {
            Self::Unicode(chars::graphemes(value).collect())
        }
    }
}

impl From<Box<str>> for Utf32String {
    fn from(value: Box<str>) -> Self {
        if value.is_ascii() {
            Self::Ascii(value)
        } else {
            Self::Unicode(chars::graphemes(&value).collect())
        }
    }
}

impl From<String> for Utf32String {
    #[inline]
    fn from(value: String) -> Self {
        value.into_boxed_str().into()
    }
}

impl<'a> From<Cow<'a, str>> for Utf32String {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        match value {
            Cow::Borrowed(value) => value.into(),
            Cow::Owned(value) => value.into(),
        }
    }
}

pub enum Chars<'a> {
    Ascii(slice::Iter<'a, u8>),
    Unicode(slice::Iter<'a, char>),
}
impl<'a> Iterator for Chars<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Chars::Ascii(iter) => iter.next().map(|&c| c as char),
            Chars::Unicode(iter) => iter.next().copied(),
        }
    }
}

impl fmt::Debug for Utf32String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        for c in self.chars() {
            for c in c.escape_debug() {
                write!(f, "{c}")?
            }
        }
        write!(f, "\"")
    }
}

impl fmt::Display for Utf32String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.chars() {
            write!(f, "{c}")?
        }
        Ok(())
    }
}
