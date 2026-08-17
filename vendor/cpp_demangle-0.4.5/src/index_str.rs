//! Provides the `IndexStr` type to keep track of a substring's index into its
//! original string is.

use alloc::string::String;
use core::fmt;
use core::ops::{Range, RangeFrom, RangeTo};

/// The `IndexStr` type allows us to take substrings from an original input and
/// keep track of what index the substring is at in the original input.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IndexStr<'a> {
    idx: usize,
    string: &'a [u8],
}

#[allow(dead_code)]
impl<'a> IndexStr<'a> {
    /// Construct a new `IndexStr` (with `index == 0`) from the given input.
    #[inline]
    pub fn new(string: &'a [u8]) -> IndexStr<'a> {
        IndexStr {
            idx: 0,
            string: string,
        }
    }

    /// Return the length of the string.
    #[inline]
    pub fn len(&self) -> usize {
        self.string.len()
    }

    /// Return true if the string is empty, false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    /// Get the index into the original input that this `IndexStr` is at.
    #[inline]
    pub fn index(&self) -> usize {
        self.idx
    }

    /// Peek at the next byte in this `IndexStr`.
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        self.as_ref().get(0).cloned()
    }

    /// Peek at the second next byte in this `IndexStr`.
    #[inline]
    pub fn peek_second(&self) -> Option<u8> {
        self.as_ref().get(1).cloned()
    }

    /// Split the string in two at the given index, resulting in the tuple where
    /// the first item has range `[0, idx)`, and the second has range `[idx,
    /// len)`.
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    pub fn split_at(&self, idx: usize) -> (IndexStr<'a>, IndexStr<'a>) {
        (self.range_to(..idx), self.range_from(idx..))
    }

    /// The same as `split_at`, but returns an `Option` rather than panicking
    /// when the index is out of bounds.
    #[inline]
    pub fn try_split_at(&self, idx: usize) -> Option<(IndexStr<'a>, IndexStr<'a>)> {
        if idx > self.len() {
            None
        } else {
            Some(self.split_at(idx))
        }
    }

    /// Pop the next byte off the front of this string, returning it and the new
    /// tail string, or `None` if this string is empty.
    #[inline]
    pub fn next(&self) -> Option<(u8, IndexStr<'a>)> {
        if self.is_empty() {
            None
        } else {
            let byte = self.string[0];
            Some((byte, self.range_from(1..)))
        }
    }

    /// Pop the next byte off the front of this string, returning it and the new
    /// tail string, or the given error if this string is empty.
    #[inline]
    pub fn next_or<E>(&self, error: E) -> Result<(u8, IndexStr<'a>), E> {
        self.next().ok_or(error)
    }
}

/// # Range Methods
///
/// Unfortunately, `std::ops::Index` *must* return a reference, so we can't
/// implement `Index<Range<usize>>` to return a new `IndexStr` the way we would
/// like to. Instead, we abandon fancy indexing operators and have these plain
/// old methods.
///
/// All of these methods panic on an out-of-bounds index.
#[allow(dead_code)]
impl<'a> IndexStr<'a> {
    /// Take the given `start..end` range of the underlying string and return a
    /// new `IndexStr`.
    #[inline]
    pub fn range(&self, idx: Range<usize>) -> IndexStr<'a> {
        IndexStr {
            idx: self.idx + idx.start,
            string: &self.string[idx],
        }
    }

    /// Take the given `start..` range of the underlying string and return a new
    /// `IndexStr`.
    #[inline]
    pub fn range_from(&self, idx: RangeFrom<usize>) -> IndexStr<'a> {
        IndexStr {
            idx: self.idx + idx.start,
            string: &self.string[idx],
        }
    }

    /// Take the given `..end` range of the underlying string and return a new
    /// `IndexStr`.
    #[inline]
    pub fn range_to(&self, idx: RangeTo<usize>) -> IndexStr<'a> {
        IndexStr {
            idx: self.idx,
            string: &self.string[idx],
        }
    }
}

impl<'a> AsRef<[u8]> for IndexStr<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.string
    }
}

impl<'a> From<&'a [u8]> for IndexStr<'a> {
    fn from(s: &'a [u8]) -> IndexStr<'a> {
        IndexStr::new(s)
    }
}

impl<'a> Into<&'a [u8]> for IndexStr<'a> {
    fn into(self) -> &'a [u8] {
        self.string
    }
}

impl<'a, 'b> PartialEq<&'a [u8]> for IndexStr<'b> {
    fn eq(&self, rhs: &&[u8]) -> bool {
        self.string == *rhs
    }
}

impl<'a> fmt::Debug for IndexStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IndexStr {{ idx: {}, string: \"{}\" }}",
            self.idx,
            String::from_utf8_lossy(self.as_ref())
        )
    }
}
