// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Marker types for formats.
//!
//! This module defines the types and traits used to mark a `Tendril`
//! with the format of data it contains. It includes those formats
//! for which `Tendril` supports at least some operations without
//! conversion.
//!
//! To convert a string tendril to/from a byte tendril in an arbitrary
//! character encoding, see the `encode` and `decode` methods on
//! `Tendril`.
//!
//! `Tendril` operations may become memory-unsafe if data invalid for
//! the format sneaks in. For that reason, these traits require
//! `unsafe impl`.

use std::default::Default;
use std::{char, mem, str};

use futf::{self, Codepoint, Meaning};

/// Implementation details.
///
/// You don't need these unless you are implementing
/// a new format.
pub mod imp {
    use std::default::Default;
    use std::{iter, mem, slice};

    /// Describes how to fix up encodings when concatenating.
    ///
    /// We can drop characters on either side of the splice,
    /// and insert up to 4 bytes in the middle.
    pub struct Fixup {
        pub drop_left: u32,
        pub drop_right: u32,
        pub insert_len: u32,
        pub insert_bytes: [u8; 4],
    }

    impl Default for Fixup {
        #[inline(always)]
        fn default() -> Fixup {
            Fixup {
                drop_left: 0,
                drop_right: 0,
                insert_len: 0,
                insert_bytes: [0; 4],
            }
        }
    }

    #[inline(always)]
    unsafe fn from_u32_unchecked(n: u32) -> char {
        mem::transmute(n)
    }

    pub struct SingleByteCharIndices<'a> {
        inner: iter::Enumerate<slice::Iter<'a, u8>>,
    }

    impl<'a> Iterator for SingleByteCharIndices<'a> {
        type Item = (usize, char);

        #[inline]
        fn next(&mut self) -> Option<(usize, char)> {
            self.inner
                .next()
                .map(|(i, &b)| unsafe { (i, from_u32_unchecked(b as u32)) })
        }
    }

    impl<'a> SingleByteCharIndices<'a> {
        #[inline]
        pub fn new(buf: &'a [u8]) -> SingleByteCharIndices<'a> {
            SingleByteCharIndices {
                inner: buf.iter().enumerate(),
            }
        }
    }
}

/// Trait for format marker types.
///
/// The type implementing this trait is usually not instantiated.
/// It's used with a phantom type parameter of `Tendril`.
pub unsafe trait Format {
    /// Check whether the buffer is valid for this format.
    fn validate(buf: &[u8]) -> bool;

    /// Check whether the buffer is valid for this format.
    ///
    /// You may assume the buffer is a prefix of a valid buffer.
    #[inline]
    fn validate_prefix(buf: &[u8]) -> bool {
        <Self as Format>::validate(buf)
    }

    /// Check whether the buffer is valid for this format.
    ///
    /// You may assume the buffer is a suffix of a valid buffer.
    #[inline]
    fn validate_suffix(buf: &[u8]) -> bool {
        <Self as Format>::validate(buf)
    }

    /// Check whether the buffer is valid for this format.
    ///
    /// You may assume the buffer is a contiguous subsequence
    /// of a valid buffer, but not necessarily a prefix or
    /// a suffix.
    #[inline]
    fn validate_subseq(buf: &[u8]) -> bool {
        <Self as Format>::validate(buf)
    }

    /// Compute any fixup needed when concatenating buffers.
    ///
    /// The default is to do nothing.
    ///
    /// The function is `unsafe` because it may assume the input
    /// buffers are already valid for the format. Also, no
    /// bounds-checking is performed on the return value!
    #[inline(always)]
    unsafe fn fixup(_lhs: &[u8], _rhs: &[u8]) -> imp::Fixup {
        Default::default()
    }
}

/// Indicates that one format is a subset of another.
///
/// The subset format can be converted to the superset format
/// for free.
pub unsafe trait SubsetOf<Super>: Format
where
    Super: Format,
{
    /// Validate the *other* direction of conversion; check if
    /// this buffer from the superset format conforms to the
    /// subset format.
    ///
    /// The default calls `Self::validate`, but some conversions
    /// may implement a check which is cheaper than validating
    /// from scratch.
    fn revalidate_subset(x: &[u8]) -> bool {
        Self::validate(x)
    }
}

/// Indicates a format which corresponds to a Rust slice type,
/// representing exactly the same invariants.
pub unsafe trait SliceFormat: Format + Sized {
    type Slice: ?Sized + Slice;
}

/// Indicates a format which contains characters from Unicode
/// (all of it, or some proper subset).
pub unsafe trait CharFormat<'a>: Format {
    /// Iterator for characters and their byte indices.
    type Iter: Iterator<Item = (usize, char)>;

    /// Iterate over the characters of the string and their byte
    /// indices.
    ///
    /// You may assume the buffer is *already validated* for `Format`.
    unsafe fn char_indices(buf: &'a [u8]) -> Self::Iter;

    /// Encode the character as bytes and pass them to a continuation.
    ///
    /// Returns `Err(())` iff the character cannot be represented.
    fn encode_char<F>(ch: char, cont: F) -> Result<(), ()>
    where
        F: FnOnce(&[u8]);
}

/// Indicates a Rust slice type that is represented in memory as bytes.
pub unsafe trait Slice {
    /// Access the raw bytes of the slice.
    fn as_bytes(&self) -> &[u8];

    /// Convert a byte slice to this kind of slice.
    ///
    /// You may assume the buffer is *already validated*
    /// for `Format`.
    unsafe fn from_bytes(x: &[u8]) -> &Self;

    /// Convert a byte slice to this kind of slice.
    ///
    /// You may assume the buffer is *already validated*
    /// for `Format`.
    unsafe fn from_mut_bytes(x: &mut [u8]) -> &mut Self;
}

/// Marker type for uninterpreted bytes.
///
/// Validation will never fail for this format.
#[derive(Copy, Clone, Default, Debug)]
pub struct Bytes;

unsafe impl Format for Bytes {
    #[inline(always)]
    fn validate(_: &[u8]) -> bool {
        true
    }
}

unsafe impl SliceFormat for Bytes {
    type Slice = [u8];
}

unsafe impl Slice for [u8] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self
    }

    #[inline(always)]
    unsafe fn from_bytes(x: &[u8]) -> &[u8] {
        x
    }

    #[inline(always)]
    unsafe fn from_mut_bytes(x: &mut [u8]) -> &mut [u8] {
        x
    }
}

/// Marker type for ASCII text.
#[derive(Copy, Clone, Default, Debug)]
pub struct ASCII;

unsafe impl Format for ASCII {
    #[inline]
    fn validate(buf: &[u8]) -> bool {
        buf.iter().all(|&n| n <= 127)
    }

    #[inline(always)]
    fn validate_prefix(_: &[u8]) -> bool {
        true
    }

    #[inline(always)]
    fn validate_suffix(_: &[u8]) -> bool {
        true
    }

    #[inline(always)]
    fn validate_subseq(_: &[u8]) -> bool {
        true
    }
}

unsafe impl SubsetOf<UTF8> for ASCII {}
unsafe impl SubsetOf<Latin1> for ASCII {}

unsafe impl<'a> CharFormat<'a> for ASCII {
    type Iter = imp::SingleByteCharIndices<'a>;

    #[inline]
    unsafe fn char_indices(buf: &'a [u8]) -> imp::SingleByteCharIndices<'a> {
        imp::SingleByteCharIndices::new(buf)
    }

    #[inline]
    fn encode_char<F>(ch: char, cont: F) -> Result<(), ()>
    where
        F: FnOnce(&[u8]),
    {
        let n = ch as u32;
        if n > 0x7F {
            return Err(());
        }
        cont(&[n as u8]);
        Ok(())
    }
}

/// Marker type for UTF-8 text.
#[derive(Copy, Clone, Default, Debug)]
pub struct UTF8;

unsafe impl Format for UTF8 {
    #[inline]
    fn validate(buf: &[u8]) -> bool {
        str::from_utf8(buf).is_ok()
    }

    #[inline]
    fn validate_prefix(buf: &[u8]) -> bool {
        if buf.len() == 0 {
            return true;
        }
        match futf::classify(buf, buf.len() - 1) {
            Some(Codepoint {
                meaning: Meaning::Whole(_),
                ..
            }) => true,
            _ => false,
        }
    }

    #[inline]
    fn validate_suffix(buf: &[u8]) -> bool {
        if buf.len() == 0 {
            return true;
        }
        match futf::classify(buf, 0) {
            Some(Codepoint {
                meaning: Meaning::Whole(_),
                ..
            }) => true,
            _ => false,
        }
    }

    #[inline]
    fn validate_subseq(buf: &[u8]) -> bool {
        <Self as Format>::validate_prefix(buf) && <Self as Format>::validate_suffix(buf)
    }
}

unsafe impl SubsetOf<WTF8> for UTF8 {}

unsafe impl SliceFormat for UTF8 {
    type Slice = str;
}

unsafe impl Slice for str {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        str::as_bytes(self)
    }

    #[inline(always)]
    unsafe fn from_bytes(x: &[u8]) -> &str {
        str::from_utf8_unchecked(x)
    }

    #[inline(always)]
    unsafe fn from_mut_bytes(x: &mut [u8]) -> &mut str {
        mem::transmute(x)
    }
}

unsafe impl<'a> CharFormat<'a> for UTF8 {
    type Iter = str::CharIndices<'a>;

    #[inline]
    unsafe fn char_indices(buf: &'a [u8]) -> str::CharIndices<'a> {
        str::from_utf8_unchecked(buf).char_indices()
    }

    #[inline]
    fn encode_char<F>(ch: char, cont: F) -> Result<(), ()>
    where
        F: FnOnce(&[u8]),
    {
        cont(ch.encode_utf8(&mut [0_u8; 4]).as_bytes());
        Ok(())
    }
}

/// Marker type for WTF-8 text.
///
/// See the [WTF-8 spec](https://simonsapin.github.io/wtf-8/).
#[derive(Copy, Clone, Default, Debug)]
pub struct WTF8;

#[inline]
fn wtf8_meaningful(m: Meaning) -> bool {
    match m {
        Meaning::Whole(_) | Meaning::LeadSurrogate(_) | Meaning::TrailSurrogate(_) => true,
        _ => false,
    }
}

unsafe impl Format for WTF8 {
    #[inline]
    fn validate(buf: &[u8]) -> bool {
        let mut i = 0;
        let mut prev_lead = false;
        while i < buf.len() {
            let codept = unwrap_or_return!(futf::classify(buf, i), false);
            if !wtf8_meaningful(codept.meaning) {
                return false;
            }
            i += codept.bytes.len();
            prev_lead = match codept.meaning {
                Meaning::TrailSurrogate(_) if prev_lead => return false,
                Meaning::LeadSurrogate(_) => true,
                _ => false,
            };
        }

        true
    }

    #[inline]
    fn validate_prefix(buf: &[u8]) -> bool {
        if buf.len() == 0 {
            return true;
        }
        match futf::classify(buf, buf.len() - 1) {
            Some(c) => wtf8_meaningful(c.meaning),
            _ => false,
        }
    }

    #[inline]
    fn validate_suffix(buf: &[u8]) -> bool {
        if buf.len() == 0 {
            return true;
        }
        match futf::classify(buf, 0) {
            Some(c) => wtf8_meaningful(c.meaning),
            _ => false,
        }
    }

    #[inline]
    fn validate_subseq(buf: &[u8]) -> bool {
        <Self as Format>::validate_prefix(buf) && <Self as Format>::validate_suffix(buf)
    }

    #[inline]
    unsafe fn fixup(lhs: &[u8], rhs: &[u8]) -> imp::Fixup {
        const ERR: &'static str = "WTF8: internal error";

        if lhs.len() >= 3 && rhs.len() >= 3 {
            if let (
                Some(Codepoint {
                    meaning: Meaning::LeadSurrogate(hi),
                    ..
                }),
                Some(Codepoint {
                    meaning: Meaning::TrailSurrogate(lo),
                    ..
                }),
            ) = (futf::classify(lhs, lhs.len() - 1), futf::classify(rhs, 0))
            {
                let mut fixup = imp::Fixup {
                    drop_left: 3,
                    drop_right: 3,
                    insert_len: 0,
                    insert_bytes: [0_u8; 4],
                };

                let n = 0x10000 + ((hi as u32) << 10) + (lo as u32);

                let ch = char::from_u32(n).expect(ERR);
                fixup.insert_len = ch.encode_utf8(&mut fixup.insert_bytes).len() as u32;

                return fixup;
            }
        }

        Default::default()
    }
}

/// Marker type for the single-byte encoding of the first 256 Unicode codepoints.
///
/// This is IANA's "ISO-8859-1". It's ISO's "ISO 8859-1" with the addition of the
/// C0 and C1 control characters from ECMA-48 / ISO 6429.
///
/// Not to be confused with WHATWG's "latin1" or "iso8859-1" labels (or the
/// many other aliases), which actually stand for Windows-1252.
#[derive(Copy, Clone, Default, Debug)]
pub struct Latin1;

unsafe impl Format for Latin1 {
    #[inline(always)]
    fn validate(_: &[u8]) -> bool {
        true
    }

    #[inline(always)]
    fn validate_prefix(_: &[u8]) -> bool {
        true
    }

    #[inline(always)]
    fn validate_suffix(_: &[u8]) -> bool {
        true
    }

    #[inline(always)]
    fn validate_subseq(_: &[u8]) -> bool {
        true
    }
}

unsafe impl<'a> CharFormat<'a> for Latin1 {
    type Iter = imp::SingleByteCharIndices<'a>;

    #[inline]
    unsafe fn char_indices(buf: &'a [u8]) -> imp::SingleByteCharIndices<'a> {
        imp::SingleByteCharIndices::new(buf)
    }

    #[inline]
    fn encode_char<F>(ch: char, cont: F) -> Result<(), ()>
    where
        F: FnOnce(&[u8]),
    {
        let n = ch as u32;
        if n > 0xFF {
            return Err(());
        }
        cont(&[n as u8]);
        Ok(())
    }
}
