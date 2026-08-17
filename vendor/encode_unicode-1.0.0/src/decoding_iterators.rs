/* Copyright 2018-2020 Torbjørn Birch Moltu
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

//! Iterators that turn multiple `u8`s or `u16`s into `Utf*Char`s, but can fail.
//!
//! To be predictable, all errors consume one element each.
//!
//! The iterator adaptors produce neither offset nor element length to work
//! well with other adaptors,
//! while the slice iterators yield both to make more advanced use cases easy.

use crate::errors::{Utf16FirstUnitError, Utf16PairError, Utf8Error};
use crate::errors::Utf16SliceError::*;
use crate::errors::Utf16PairError::*;
use crate::errors::Utf8ErrorKind::*;
use crate::utf8_char::Utf8Char;
use crate::utf16_char::Utf16Char;
use crate::traits::U16UtfExt;
extern crate core;
use core::borrow::Borrow;
use core::fmt::{self, Debug};
use core::iter::Chain;
use core::option;


/// Decodes UTF-8 characters from a byte iterator into `Utf8Char`s.
///
/// See [`IterExt::to_utf8chars()`](../trait.IterExt.html#tymethod.to_utf8chars)
/// for examples and error handling.
#[derive(Clone, Default)]
pub struct Utf8CharMerger<B:Borrow<u8>, I:Iterator<Item=B>> {
    iter: I,
    /// number of bytes that were read before an error was detected
    after_err_leftover: u8,
    /// stack because it simplifies popping.
    after_err_stack: [u8; 3],
}
impl<B:Borrow<u8>, I:Iterator<Item=B>, T:IntoIterator<IntoIter=I,Item=B>>
From<T> for Utf8CharMerger<B, I> {
    fn from(t: T) -> Self {
        Utf8CharMerger {
            iter: t.into_iter(),
            after_err_leftover: 0,
            after_err_stack: [0; 3],
        }
    }
}
impl<B:Borrow<u8>, I:Iterator<Item=B>> Utf8CharMerger<B,I> {
    /// Extract the inner iterator.
    ///
    /// If the last item produced by `.next()` was an `Err`,
    /// up to three following bytes might be missing.  
    /// The exact number of missing bytes for each error type should not be relied on.
    ///
    /// # Examples
    ///
    /// Three bytes swallowed:
    /// ```
    /// # use encode_unicode::IterExt;
    /// let mut merger = b"\xf4\xa1\xb2FS".iter().to_utf8chars();
    /// assert!(merger.next().unwrap().is_err());
    /// let mut inner: std::slice::Iter<u8> = merger.into_inner();
    /// assert_eq!(inner.next(), Some(&b'S')); // b'\xa1', b'\xb2' and b'F' disappeared
    /// ```
    ///
    /// All bytes present:
    /// ```
    /// # use encode_unicode::IterExt;
    /// let mut merger = b"\xb0FS".iter().to_utf8chars();
    /// assert!(merger.next().unwrap().is_err());
    /// assert_eq!(merger.into_inner().next(), Some(&b'F'));
    /// ```
    ///
    /// Two bytes missing:
    /// ```
    /// # use encode_unicode::IterExt;
    /// let mut merger = b"\xe0\x80\x80FS".iter().to_utf8chars();
    /// assert!(merger.next().unwrap().is_err());
    /// assert_eq!(merger.into_inner().next(), Some(&b'F'));
    /// ```
    pub fn into_inner(self) -> I {
        self.iter
    }

    fn save(&mut self,  bytes: &[u8;4],  len: usize) {
        // forget bytes[0] and push the others onto self.after_err_stack (in reverse).
        for &after_err in bytes[1..len].iter().rev() {
            self.after_err_stack[self.after_err_leftover as usize] = after_err;
            self.after_err_leftover += 1;
        }
    }
    /// Reads len-1 bytes into bytes[1..]
    fn extra(&mut self,  bytes: &mut[u8;4],  len: usize) -> Result<(),Utf8Error> {
        // This is the only function that pushes onto after_err_stack,
        // and it checks that all bytes are continuation bytes before fetching the next one.
        // Therefore only the last byte retrieved can be a non-continuation byte.
        // That last byte is also the last to be retrieved from after_err.
        //
        // Before this function is called, there has been retrieved at least one byte.
        // If that byte was a continuation byte, next() produces an error
        // and won't call this function.
        // Therefore, we know that after_err is empty at this point.
        // This means that we can use self.iter directly, and knows where to start pushing
        debug_assert_eq!(self.after_err_leftover, 0, "first: {:#02x}, stack: {:?}", bytes[0], self.after_err_stack);
        for i in 1..len {
            if let Some(extra) = self.iter.next() {
                let extra = *extra.borrow();
                bytes[i] = extra;
                if extra & 0b1100_0000 != 0b1000_0000 {
                    // not a continuation byte
                    self.save(bytes, i+1);
                    return Err(Utf8Error{ kind: InterruptedSequence })
                }
            } else {
                self.save(bytes, i);
                return Err(Utf8Error{ kind: TooFewBytes });
            }
        }
        Ok(())
    }
}
impl<B:Borrow<u8>, I:Iterator<Item=B>> Iterator for Utf8CharMerger<B,I> {
    type Item = Result<Utf8Char,Utf8Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let first: u8;
        if self.after_err_leftover != 0 {
            self.after_err_leftover -= 1;
            first = self.after_err_stack[self.after_err_leftover as usize];
        } else if let Some(next) = self.iter.next() {
            first = *next.borrow();
        } else {
            return None;
        }

        unsafe {
            let mut bytes = [first, 0, 0, 0];
            let ok = match first {
                0b0000_0000..=0b0111_1111 => {/*1 and */Ok(())},
                0b1100_0010..=0b1101_1111 => {//2 and not overlong
                    self.extra(&mut bytes, 2) // no extra validation required
                },
                0b1110_0000..=0b1110_1111 => {//3
                    if let Err(e) = self.extra(&mut bytes, 3) {
                        Err(e)
                    } else if bytes[0] == 0b1110_0000  &&  bytes[1] <= 0b10_011111 {
                        self.save(&bytes, 3);
                        Err(Utf8Error{ kind: OverlongEncoding })
                    } else if bytes[0] == 0b1110_1101  &&  bytes[1] & 0b11_100000 == 0b10_100000 {
                        self.save(&bytes, 3);
                        Err(Utf8Error{ kind: Utf16ReservedCodepoint })
                    } else {
                        Ok(())
                    }
                },
                0b1111_0000..=0b1111_0100 => {//4
                    if let Err(e) = self.extra(&mut bytes, 4) {
                        Err(e)
                    } else if bytes[0] == 0b11110_000  &&  bytes[1] <= 0b10_001111 {
                        self.save(&bytes, 4);
                        Err(Utf8Error{ kind: OverlongEncoding })
                    } else if bytes[0] == 0b11110_100  &&  bytes[1] > 0b10_001111 {
                        self.save(&bytes, 4);
                        Err(Utf8Error{ kind: TooHighCodepoint })
                    } else {
                        Ok(())
                    }
                },
                0b1000_0000..=0b1011_1111 => {// continuation byte
                    Err(Utf8Error{ kind: UnexpectedContinuationByte })
                },
                0b1100_0000..=0b1100_0001 => {// 2 and overlong
                    Err(Utf8Error{ kind: NonUtf8Byte })
                },
                0b1111_0101..=0b1111_0111 => {// 4 and too high codepoint
                    Err(Utf8Error{ kind: NonUtf8Byte })
                },
                0b1111_1000..=0b1111_1111 => {
                    Err(Utf8Error{ kind: NonUtf8Byte })
                },
            };
            Some(ok.map(|()| Utf8Char::from_array_unchecked(bytes) ))
        }
    }
    fn size_hint(&self) -> (usize,Option<usize>) {
        let (iter_min, iter_max) = self.iter.size_hint();
        // cannot be exact, so KISS
        let min = iter_min / 4; // don't bother rounding up or accounting for after_err
        // handle edge case of max > usize::MAX-3 just in case.
        // Using wrapping_add() wouldn't violate any API contract as the trait isn't unsafe.
        let max = iter_max.and_then(|max| {
            max.checked_add(self.after_err_leftover as usize)
        });
        (min, max)
    }
}
impl<B:Borrow<u8>, I:Iterator<Item=B>+Debug> Debug for Utf8CharMerger<B,I> {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut in_order = [0u8; 3];
        for i in 0..self.after_err_leftover as usize {
            in_order[i] = self.after_err_stack[self.after_err_leftover as usize - i - 1];
        }
        fmtr.debug_struct("Utf8CharMerger")
            .field("buffered", &&in_order[..self.after_err_leftover as usize])
            .field("inner", &self.iter)
            .finish()
    }
}


/// An [`Utf8CharMerger`](struct.Utf8CharMerger.html) that also produces
/// offsets and lengths, but can only iterate over slices.
///
/// See [`SliceExt::utf8char_indices()`](../trait.SliceExt.html#tymethod.utf8char_indices)
/// for examples and error handling.
#[derive(Clone, Default)]
pub struct Utf8CharDecoder<'a> {
    slice: &'a[u8],
    index: usize,
}
impl<'a> From<&'a[u8]> for Utf8CharDecoder<'a> {
    fn from(s: &[u8]) -> Utf8CharDecoder {
        Utf8CharDecoder { slice: s, index: 0 }
    }
}
impl<'a> Utf8CharDecoder<'a> {
    /// Extract the remainder of the source slice.
    ///
    /// # Examples
    ///
    /// Unlike `Utf8CharMerger::into_inner()`, bytes directly after an error
    /// are never swallowed:
    /// ```
    /// # use encode_unicode::SliceExt;
    /// let mut iter = b"\xf4\xa1\xb2FS".utf8char_indices();
    /// assert!(iter.next().unwrap().1.is_err());
    /// assert_eq!(iter.as_slice(), b"\xa1\xb2FS");
    /// ```
    pub fn as_slice(&self) -> &'a[u8] {
        &self.slice[self.index..]
    }
}
impl<'a> Iterator for Utf8CharDecoder<'a> {
    type Item = (usize, Result<Utf8Char,Utf8Error>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.index;
        match Utf8Char::from_slice_start(&self.slice[self.index..]) {
            Ok((u8c, len)) => {
                self.index += len;
                Some((start, Ok(u8c), len))
            },
            Err(_) if self.slice.len() <= self.index => None,
            Err(e) => {
                self.index += 1;
                Some((start, Err(e), 1))
            }
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize,Option<usize>) {
        let bytes = self.slice.len() - self.index;
        // Cannot be exact, so KISS and don't bother rounding up.
        // The slice is unlikely be full of 4-byte codepoints, so buffers
        // allocated with the lower bound will have to be grown anyway.
        (bytes/4, Some(bytes))
    }
}
impl<'a> DoubleEndedIterator for Utf8CharDecoder<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.slice.len() {
            let extras = self.slice.iter()
                .rev()
                .take_while(|&b| b & 0b1100_0000 == 0b1000_0000 )
                .count();
            let starts = self.slice.len() - (extras+1);
            match Utf8Char::from_slice_start(&self.slice[starts..]) {
                Ok((u8c,len)) if len == 1+extras => {
                    self.slice = &self.slice[..starts];
                    Some((starts, Ok(u8c), len))
                },
                // This enures errors for every byte in both directions,
                // but means overlong and codepoint errors will be turned into
                // tooshort errors.
                Err(e) if extras == 0 => {
                    self.slice = &self.slice[..self.slice.len()-1];
                    Some((self.slice.len()-1, Err(e), 1))
                },
                _ => {
                    self.slice = &self.slice[..self.slice.len()-1];
                    Some((self.slice.len()-1, Err(Utf8Error{ kind: UnexpectedContinuationByte }), 1))
                },
            }
        } else {
            None
        }
    }
}
impl<'a> Debug for Utf8CharDecoder<'a> {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Utf8CharDecoder {{ bytes[{}..]: {:?} }}", self.index, self.as_slice())
    }
}



/// Decodes UTF-16 characters from a `u16` iterator into `Utf16Char`s.
///
/// See [`IterExt::to_utf16chars()`](../trait.IterExt.html#tymethod.to_utf16chars)
/// for examples and error handling.
#[derive(Clone, Default)]
pub struct Utf16CharMerger<B:Borrow<u16>, I:Iterator<Item=B>> {
    iter: I,
    /// Used when a trailing surrogate was expected, the u16 can be any value.
    prev: Option<B>,
}
impl<B:Borrow<u16>, I:Iterator<Item=B>, T:IntoIterator<IntoIter=I,Item=B>>
From<T> for Utf16CharMerger<B,I> {
    fn from(t: T) -> Self {
        Utf16CharMerger { iter: t.into_iter(),  prev: None }
    }
}
impl<B:Borrow<u16>, I:Iterator<Item=B>> Utf16CharMerger<B,I> {
    /// Extract the inner iterator.
    ///
    /// If the last item produced was an `Err`, the first unit might be missing.
    ///
    /// # Examples
    ///
    /// Unit right after an error missing
    /// ```
    /// # use encode_unicode::IterExt;
    /// # use encode_unicode::error::Utf16PairError;
    /// let mut merger = [0xd901, 'F' as u16, 'S' as u16].iter().to_utf16chars();
    /// assert_eq!(merger.next(), Some(Err(Utf16PairError::UnmatchedLeadingSurrogate)));
    /// let mut inner: std::slice::Iter<u16> = merger.into_inner();
    /// assert_eq!(inner.next(), Some('S' as u16).as_ref()); // 'F' was consumed by Utf16CharMerger
    /// ```
    ///
    /// Error that doesn't swallow any units
    /// ```
    /// # use encode_unicode::IterExt;
    /// # use encode_unicode::error::Utf16PairError;
    /// let mut merger = [0xde00, 'F' as u16, 'S' as u16].iter().to_utf16chars();
    /// assert_eq!(merger.next(), Some(Err(Utf16PairError::UnexpectedTrailingSurrogate)));
    /// let mut inner: std::slice::Iter<u16> = merger.into_inner();
    /// assert_eq!(inner.next(), Some('F' as u16).as_ref()); // not consumed
    /// ```
    pub fn into_inner(self) -> I {
        self.iter
    }
    /// Returns an iterator over the remaining units.
    /// Unlike `into_inner()` this will never drop any units.
    ///
    /// The exact type of the returned iterator should not be depended on.
    ///
    /// # Examples
    ///
    /// ```
    /// # use encode_unicode::IterExt;
    /// # use encode_unicode::error::Utf16PairError;
    /// let slice = [0xd901, 'F' as u16, 'S' as u16];
    /// let mut merger = slice.iter().to_utf16chars();
    /// assert_eq!(merger.next(), Some(Err(Utf16PairError::UnmatchedLeadingSurrogate)));
    /// let mut remaining = merger.into_remaining_units();
    /// assert_eq!(remaining.next(), Some('F' as u16).as_ref());
    /// ```
    pub fn into_remaining_units(self) -> Chain<option::IntoIter<B>,I> {
        self.prev.into_iter().chain(self.iter)
    }
}
impl<B:Borrow<u16>, I:Iterator<Item=B>> Iterator for Utf16CharMerger<B,I> {
    type Item = Result<Utf16Char,Utf16PairError>;
    fn next(&mut self) -> Option<Self::Item> {
        let first = self.prev.take().or_else(|| self.iter.next() );
        first.map(|first| unsafe {
            match first.borrow().utf16_needs_extra_unit() {
                Ok(false) => Ok(Utf16Char::from_array_unchecked([*first.borrow(), 0])),
                Ok(true) => match self.iter.next() {
                    Some(second) => match second.borrow().utf16_needs_extra_unit() {
                        Err(Utf16FirstUnitError) => Ok(Utf16Char::from_tuple_unchecked((
                            *first.borrow(),
                            Some(*second.borrow())
                        ))),
                        Ok(_) => {
                            self.prev = Some(second);
                            Err(Utf16PairError::UnmatchedLeadingSurrogate)
                        }
                    },
                    None => Err(Utf16PairError::Incomplete)
                },
                Err(Utf16FirstUnitError) => Err(Utf16PairError::UnexpectedTrailingSurrogate),
            }
        })
    }
    fn size_hint(&self) -> (usize,Option<usize>) {
        let (iter_min, iter_max) = self.iter.size_hint();
        // cannot be exact, so KISS
        let min = iter_min / 2; // don't bother rounding up or accounting for self.prev
        let max = match (iter_max, &self.prev) {
            (Some(max), &Some(_)) => max.checked_add(1),
            (max, _) => max,
        };
        (min, max)
    }
}
impl<B:Borrow<u16>, I:Iterator<Item=B>+Debug> Debug for Utf16CharMerger<B,I> {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("Utf16CharMerger")
            .field("buffered", &self.prev.as_ref().map(|b| *b.borrow() ))
            .field("inner", &self.iter)
            .finish()
    }
}


/// An [`Utf16CharMerger`](struct.Utf16CharMerger.html) that also produces
/// offsets and lengths, but can only iterate over slices.
///
/// See [`SliceExt::utf16char_indices()`](../trait.SliceExt.html#tymethod.utf16char_indices)
/// for examples and error handling.
#[derive(Clone, Default)]
pub struct Utf16CharDecoder<'a> {
    slice: &'a[u16],
    index: usize,
}
impl<'a> From<&'a[u16]> for Utf16CharDecoder<'a> {
    fn from(s: &'a[u16]) -> Self {
        Utf16CharDecoder{ slice: s,  index: 0 }
    }
}
impl<'a> Utf16CharDecoder<'a> {
    /// Extract the remainder of the source slice.
    ///
    /// # Examples
    ///
    /// Unlike `Utf16CharMerger::into_inner()`, the unit after an error is never swallowed:
    /// ```
    /// # use encode_unicode::SliceExt;
    /// # use encode_unicode::error::Utf16PairError;
    /// let mut iter = [0xd901, 'F' as u16, 'S' as u16].utf16char_indices();
    /// assert_eq!(iter.next(), Some((0, Err(Utf16PairError::UnmatchedLeadingSurrogate), 1)));
    /// assert_eq!(iter.as_slice(), &['F' as u16, 'S' as u16]);
    /// ```
    pub fn as_slice(&self) -> &[u16] {
        &self.slice[self.index..]
    }
}
impl<'a> Iterator for Utf16CharDecoder<'a> {
    type Item = (usize,Result<Utf16Char,Utf16PairError>,usize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item>  {
        let start = self.index;
        match Utf16Char::from_slice_start(self.as_slice()) {
            Ok((u16c,len)) => {
                self.index += len;
                Some((start, Ok(u16c), len))
            },
            Err(EmptySlice) => None,
            Err(FirstIsTrailingSurrogate) => {
                self.index += 1;
                Some((start, Err(UnexpectedTrailingSurrogate), 1))
            },
            Err(SecondIsNotTrailingSurrogate) => {
                self.index += 1;
                Some((start, Err(UnmatchedLeadingSurrogate), 1))
            },
            Err(MissingSecond) => {
                self.index = self.slice.len();
                Some((start, Err(Incomplete), 1))
            }
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize,Option<usize>) {
        let units = self.slice.len() - self.index;
        // Cannot be exact, so KISS and don't bother rounding up.
        // The slice is unlikely be full of surrogate pairs, so buffers
        // allocated with the lower bound will have to be grown anyway.
        (units/2, Some(units))
    }
}
impl<'a> Debug for Utf16CharDecoder<'a> {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Utf16CharDecoder {{ units[{}..]: {:?} }}", self.index, self.as_slice())
    }
}
