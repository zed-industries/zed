use crate::base::Bytes;
use std::fmt;
use std::ops;

/// Original position in the parsed document
///
/// Source locations are not affected by document rewriting.
#[derive(Clone)]
pub struct SourceLocation(ops::Range<usize>);

impl SourceLocation {
    #[inline]
    pub(crate) const fn from_start_len(start: usize, len: usize) -> Self {
        Self(ops::Range {
            start,
            end: start + len,
        })
    }

    /// Absolute start/end position in bytes
    ///
    /// Currently we don't track line numbers, only byte positions.
    ///
    /// The offset is in bytes, not characters. It referes to the input data,
    /// in the input's original character encoding.
    #[inline]
    #[doc(alias = "line")]
    #[must_use]
    pub fn bytes(&self) -> ops::Range<usize> {
        self.0.clone()
    }
}

pub(crate) type SpannedRawBytes<'input> = Spanned<RawBytes<'input>>;

impl<'i> From<Spanned<Bytes<'i>>> for SpannedRawBytes<'i> {
    fn from(s: Spanned<Bytes<'i>>) -> Self {
        Self {
            bytes: RawBytes::Original(s.bytes.as_slice()),
            source_location_byte_start: s.source_location_byte_start,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Spanned<B> {
    bytes: B,
    source_location_byte_start: usize,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum RawBytes<'input> {
    Original(&'input [u8]),
    /// Keeps the len of the original
    Modified(usize),
}

impl<'input> SpannedRawBytes<'input> {
    #[inline]
    pub fn len(&self) -> usize {
        match self.bytes {
            RawBytes::Original(s) => s.len(),
            RawBytes::Modified(l) => l,
        }
    }

    #[inline]
    pub fn set_modified(&mut self) {
        // optimizes to branchless
        self.bytes = RawBytes::Modified(self.len());
    }

    #[inline]
    pub fn original(&self) -> Option<&'input [u8]> {
        match self.bytes {
            RawBytes::Original(s) => Some(s),
            RawBytes::Modified(_) => None,
        }
    }

    #[inline]
    pub fn source_location(&self) -> SourceLocation {
        SourceLocation::from_start_len(self.source_location_byte_start, self.len())
    }
}

impl<'input> Spanned<Bytes<'input>> {
    pub(crate) const fn new(source_location_byte_start: usize, input_raw: Bytes<'input>) -> Self {
        Self {
            bytes: input_raw,
            source_location_byte_start,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    pub fn as_slice(&self) -> &'input [u8] {
        self.bytes.as_slice()
    }

    #[inline]
    pub fn source_location(&self) -> SourceLocation {
        SourceLocation::from_start_len(self.source_location_byte_start, self.len())
    }
}

impl fmt::Debug for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}B...{}B", self.0.start, self.0.end)
    }
}
