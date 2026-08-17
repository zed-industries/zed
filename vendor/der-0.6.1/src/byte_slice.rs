//! Common handling for types backed by byte slices with enforcement of a
//! library-level length limitation i.e. `Length::max()`.

use crate::{
    str_slice::StrSlice, DecodeValue, DerOrd, EncodeValue, Error, Header, Length, Reader, Result,
    Writer,
};
use core::cmp::Ordering;

/// Byte slice newtype which respects the `Length::max()` limit.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ByteSlice<'a> {
    /// Precomputed `Length` (avoids possible panicking conversions)
    length: Length,

    /// Inner value
    inner: &'a [u8],
}

impl<'a> ByteSlice<'a> {
    /// Constant value representing an empty byte slice.
    pub const EMPTY: Self = Self {
        length: Length::ZERO,
        inner: &[],
    };

    /// Create a new [`ByteSlice`], ensuring that the provided `slice` value
    /// is shorter than `Length::max()`.
    pub fn new(slice: &'a [u8]) -> Result<Self> {
        Ok(Self {
            length: Length::try_from(slice.len())?,
            inner: slice,
        })
    }

    /// Borrow the inner byte slice
    pub fn as_slice(&self) -> &'a [u8] {
        self.inner
    }

    /// Get the [`Length`] of this [`ByteSlice`]
    pub fn len(self) -> Length {
        self.length
    }

    /// Is this [`ByteSlice`] empty?
    pub fn is_empty(self) -> bool {
        self.len() == Length::ZERO
    }
}

impl AsRef<[u8]> for ByteSlice<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> DecodeValue<'a> for ByteSlice<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        reader.read_slice(header.length).and_then(Self::new)
    }
}

impl EncodeValue for ByteSlice<'_> {
    fn value_len(&self) -> Result<Length> {
        Ok(self.length)
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        writer.write(self.as_ref())
    }
}

impl Default for ByteSlice<'_> {
    fn default() -> Self {
        Self {
            length: Length::ZERO,
            inner: &[],
        }
    }
}

impl DerOrd for ByteSlice<'_> {
    fn der_cmp(&self, other: &Self) -> Result<Ordering> {
        Ok(self.as_slice().cmp(other.as_slice()))
    }
}

impl<'a> From<&'a [u8; 1]> for ByteSlice<'a> {
    fn from(byte: &'a [u8; 1]) -> ByteSlice<'a> {
        Self {
            length: Length::ONE,
            inner: byte,
        }
    }
}

impl<'a> From<StrSlice<'a>> for ByteSlice<'a> {
    fn from(s: StrSlice<'a>) -> ByteSlice<'a> {
        let bytes = s.as_bytes();
        debug_assert_eq!(bytes.len(), usize::try_from(s.length).expect("overflow"));

        ByteSlice {
            inner: bytes,
            length: s.length,
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for ByteSlice<'a> {
    type Error = Error;

    fn try_from(slice: &'a [u8]) -> Result<Self> {
        Self::new(slice)
    }
}
