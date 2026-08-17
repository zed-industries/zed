use super::Range;
use encoding_rs::{Encoding, WINDOWS_1252};
use std::borrow::Cow;
use std::fmt::{self, Debug};
use std::ops::Deref;
use std::str;

/// An error used to indicate that an encoded string has replacements and can't be converted losslessly.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct HasReplacementsError;

/// A thin wrapper around byte slice with handy APIs attached
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub(crate) struct Bytes<'b>(&'b [u8]);

/// A thin wrapper around either byte slice or owned bytes with some handy APIs attached
#[derive(Clone, PartialEq, Eq, Hash)]
#[allow(unnameable_types)] // it's pub only for integration test
#[repr(transparent)]
pub struct BytesCow<'b>(Cow<'b, [u8]>);

impl<'b> BytesCow<'b> {
    pub(crate) fn borrow_from_str_without_replacements(
        string: impl Into<Cow<'b, str>>,
        encoding: &'static Encoding,
    ) -> Result<Self, HasReplacementsError> {
        let (has_replacements, bytes) = Self::borrowed(string, encoding);
        if !has_replacements {
            Ok(bytes)
        } else {
            Err(HasReplacementsError)
        }
    }

    fn borrowed(string: impl Into<Cow<'b, str>>, encoding: &'static Encoding) -> (bool, Self) {
        let string = string.into();
        let (res, _, has_replacements) = encoding.encode(&string);

        (
            has_replacements,
            Self(match res {
                Cow::Borrowed(_) => match string {
                    Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
                    Cow::Owned(s) => Cow::Owned(s.into_bytes()),
                },
                Cow::Owned(bytes) => Cow::Owned(bytes),
            }),
        )
    }

    #[inline]
    pub(crate) fn into_owned(self) -> BytesCow<'static> {
        BytesCow(Cow::Owned(self.0.into_owned()))
    }

    #[inline]
    pub(crate) fn as_ref(&self) -> Bytes<'_> {
        Bytes(&self.0)
    }

    pub(crate) fn as_string(&self, encoding: &'static Encoding) -> String {
        self.as_ref().as_string(encoding)
    }

    pub(crate) fn as_lowercase_string(&self, encoding: &'static Encoding) -> String {
        self.as_ref().as_lowercase_string(encoding)
    }
}

impl BytesCow<'static> {
    #[inline]
    pub(crate) fn owned_from_str<'copied>(
        string: impl Into<Cow<'copied, str>>,
        encoding: &'static Encoding,
    ) -> Self {
        Self::owned(string, encoding).1
    }

    #[inline]
    pub(crate) fn owned_from_str_without_replacements<'copied>(
        string: impl Into<Cow<'copied, str>>,
        encoding: &'static Encoding,
    ) -> Result<Self, HasReplacementsError> {
        let (has_replacements, bytes) = Self::owned(string, encoding);
        if !has_replacements {
            Ok(bytes)
        } else {
            Err(HasReplacementsError)
        }
    }

    fn owned<'tmp>(string: impl Into<Cow<'tmp, str>>, encoding: &'static Encoding) -> (bool, Self) {
        let string = string.into();
        let (res, _, has_replacements) = encoding.encode(&string);

        (
            has_replacements,
            BytesCow(Cow::Owned(match res {
                Cow::Owned(bytes) => bytes,
                Cow::Borrowed(_) => string.into_owned().into_bytes(),
            })),
        )
    }
}

impl<'b> Bytes<'b> {
    #[inline]
    pub(crate) fn new(bytes: &'b [u8]) -> Self {
        Self(bytes)
    }

    #[inline]
    pub fn as_string(&self, encoding: &'static Encoding) -> String {
        encoding.decode(self.0).0.into_owned()
    }

    #[inline]
    pub fn as_lowercase_string(&self, encoding: &'static Encoding) -> String {
        encoding.decode(self.0).0.to_ascii_lowercase()
    }

    #[inline]
    pub(crate) const fn as_slice(&self) -> &'b [u8] {
        self.0
    }

    #[inline]
    pub(crate) fn slice(&self, range: Range) -> Self {
        debug_assert!(self.0.get(range.start..range.end).is_some());
        // Optimizes to panic-free branchless
        let end = range.end.min(self.0.len());
        let start = range.start.min(end);
        Self(&self.0[start..end])
    }

    #[inline]
    pub fn split_at(&self, pos: usize) -> (Self, Self) {
        let (before, after) = self.0.split_at(pos);
        (Self(before), Self(after))
    }

    #[inline]
    pub(crate) fn opt_slice(&self, range: Option<Range>) -> Option<Self> {
        let range = range?;
        self.0.get(range.start..range.end).map(Self)
    }

    pub(crate) fn as_debug_string(&self) -> String {
        // NOTE: use WINDOWS_1252 (superset of ASCII) encoding here as
        // the most safe variant since we don't know which actual encoding
        // has been used for bytes.
        self.as_string(WINDOWS_1252)
    }
}

impl<'b> From<Cow<'b, [u8]>> for BytesCow<'b> {
    #[inline]
    fn from(bytes: Cow<'b, [u8]>) -> Self {
        Self(bytes)
    }
}

impl<'b> From<Bytes<'b>> for BytesCow<'b> {
    #[inline]
    fn from(bytes: Bytes<'b>) -> Self {
        Self(Cow::Borrowed(bytes.0))
    }
}

impl<'b> From<&'b [u8]> for BytesCow<'b> {
    #[inline]
    fn from(bytes: &'b [u8]) -> Self {
        Self(bytes.into())
    }
}

impl Debug for BytesCow<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'b> From<BytesCow<'b>> for Box<[u8]> {
    #[inline]
    fn from(bytes: BytesCow<'b>) -> Self {
        match bytes.0 {
            Cow::Owned(v) if v.len() == v.capacity() => v.into_boxed_slice(),
            _ => Self::from(&bytes.0[..]),
        }
    }
}

impl Debug for Bytes<'_> {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}`", self.as_debug_string())
    }
}

impl Deref for BytesCow<'_> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for Bytes<'_> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.0
    }
}
