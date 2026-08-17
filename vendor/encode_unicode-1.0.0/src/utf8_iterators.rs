/* Copyright 2018-2020 Torbjørn Birch Moltu
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

use crate::utf8_char::Utf8Char;
use crate::errors::EmptyStrError;
extern crate core;
use core::{u32, u64};
use core::ops::Not;
use core::fmt;
use core::borrow::Borrow;
#[cfg(feature="std")]
use std::io::{Read, Error as ioError};



/// Read or iterate over the bytes of the UTF-8 representation of a codepoint.
#[derive(Clone)]
pub struct Utf8Iterator (u32);

impl From<Utf8Char> for Utf8Iterator {
    fn from(uc: Utf8Char) -> Self {
        let used = u32::from_le_bytes(uc.to_array().0);
        // uses u64 because shifting an u32 by 32 bits is a no-op.
        let unused_set = (u64::MAX  <<  (uc.len() as u64*8)) as u32;
        Utf8Iterator(used | unused_set)
    }
}
impl From<char> for Utf8Iterator {
    fn from(c: char) -> Self {
        Self::from(Utf8Char::from(c))
    }
}
impl Iterator for Utf8Iterator {
    type Item=u8;
    fn next(&mut self) -> Option<u8> {
        let next = self.0 as u8;
        if next == 0xff {
            None
        } else {
            self.0 = (self.0 >> 8)  |  0xff_00_00_00;
            Some(next)
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(),  Some(self.len()))
    }
}
impl ExactSizeIterator for Utf8Iterator {
    fn len(&self) -> usize {// not straightforward, but possible
        let unused_bytes = self.0.not().leading_zeros() / 8;
        4 - unused_bytes as usize
    }
}
#[cfg(feature="std")]
impl Read for Utf8Iterator {
    /// Always returns Ok
    fn read(&mut self,  buf: &mut[u8]) -> Result<usize, ioError> {
        // Cannot call self.next() until I know I can write the result.
        for (i, dst) in buf.iter_mut().enumerate() {
            match self.next() {
                Some(b) => *dst = b,
                None    => return Ok(i),
            }
        }
        Ok(buf.len())
    }
}
impl fmt::Debug for Utf8Iterator {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut content = [0; 4];
        let mut i = 0;
        for b in self.clone() {
            content[i] = b;
            i += 1;
        }
        write!(fmtr, "{:?}", &content[..i])
    }
}



/// Converts an iterator of `Utf8Char` (or `&Utf8Char`)
/// to an iterator of `u8`s.
///
/// Is equivalent to calling `.flatten()` or `.flat_map()` on the original iterator,
/// but the returned iterator is ~40% faster.
///
/// The iterator also implements `Read` (if the `std` feature isn't disabled).
/// Reading will never produce an error, and calls to `.read()` and `.next()`
/// can be mixed.
///
/// The exact number of bytes cannot be known in advance, but `size_hint()`
/// gives the possible range.
/// (min: all remaining characters are ASCII, max: all require four bytes)
///
/// # Examples
///
/// From iterator of values:
///
/// ```
/// use encode_unicode::{IterExt, CharExt};
///
/// let iterator = "foo".chars().map(|c| c.to_utf8() );
/// let mut bytes = [0; 4];
/// iterator.to_bytes().zip(&mut bytes).for_each(|(b,dst)| *dst = b );
/// assert_eq!(&bytes, b"foo\0");
/// ```
///
/// From iterator of references:
///
#[cfg_attr(feature="std", doc=" ```")]
#[cfg_attr(not(feature="std"), doc=" ```no_compile")]
/// use encode_unicode::{IterExt, CharExt, Utf8Char};
///
/// let chars: Vec<Utf8Char> = "💣 bomb 💣".chars().map(|c| c.to_utf8() ).collect();
/// let bytes: Vec<u8> = chars.iter().to_bytes().collect();
/// let flat_map: Vec<u8> = chars.iter().cloned().flatten().collect();
/// assert_eq!(bytes, flat_map);
/// ```
///
/// `Read`ing from it:
///
#[cfg_attr(feature="std", doc=" ```")]
#[cfg_attr(not(feature="std"), doc=" ```no_compile")]
/// use encode_unicode::{IterExt, CharExt};
/// use std::io::Read;
///
/// let s = "Ååh‽";
/// assert_eq!(s.len(), 8);
/// let mut buf = [b'E'; 9];
/// let mut reader = s.chars().map(|c| c.to_utf8() ).to_bytes();
/// assert_eq!(reader.read(&mut buf[..]).unwrap(), 8);
/// assert_eq!(reader.read(&mut buf[..]).unwrap(), 0);
/// assert_eq!(&buf[..8], s.as_bytes());
/// assert_eq!(buf[8], b'E');
/// ```
#[derive(Clone)]
pub struct Utf8CharSplitter<U:Borrow<Utf8Char>, I:Iterator<Item=U>> {
    inner: I,
    prev: u32,
}
impl<U:Borrow<Utf8Char>, I:IntoIterator<Item=U>>
From<I> for Utf8CharSplitter<U,I::IntoIter> {
    fn from(iterable: I) -> Self {
        Utf8CharSplitter { inner: iterable.into_iter(),  prev: 0 }
    }
}
impl<U:Borrow<Utf8Char>, I:Iterator<Item=U>> Utf8CharSplitter<U,I> {
    /// Extracts the source iterator.
    ///
    /// Note that `iter.into_inner().to_bytes()` is not a no-op:  
    /// If the last returned byte from `next()` was not an ASCII character,
    /// the remaining bytes of that codepoint is lost.
    pub fn into_inner(self) -> I {
        self.inner
    }
}
impl<U:Borrow<Utf8Char>, I:Iterator<Item=U>> Iterator for Utf8CharSplitter<U,I> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.prev == 0 {
            self.inner.next().map(|u8c| {
                let array = u8c.borrow().to_array().0;
                self.prev = u32::from_le_bytes(array) >> 8;
                array[0]
            })
        } else {
            let next = self.prev as u8;
            self.prev >>= 8;
            Some(next)
        }
    }
    fn size_hint(&self) -> (usize,Option<usize>) {
        // Doesn't need to handle unlikely overflows correctly because
        // size_hint() cannot be relied upon anyway. (the trait isn't unsafe)
        let (min, max) = self.inner.size_hint();
        let add = 4 - (self.prev.leading_zeros() / 8) as usize;
        (min.wrapping_add(add), max.map(|max| max.wrapping_mul(4).wrapping_add(add) ))
    }
}
#[cfg(feature="std")]
impl<U:Borrow<Utf8Char>, I:Iterator<Item=U>> Read for Utf8CharSplitter<U,I> {
    /// Always returns `Ok`
    fn read(&mut self,  buf: &mut[u8]) -> Result<usize, ioError> {
        let mut i = 0;
        // write remaining bytes of previous codepoint
        while self.prev != 0  &&  i < buf.len() {
            buf[i] = self.prev as u8;
            self.prev >>= 8;
            i += 1;
        }
        // write whole characters
        while i < buf.len() {
            let bytes = match self.inner.next() {
                Some(u8c) => u8c.borrow().to_array().0,
                None => break
            };
            buf[i] = bytes[0];
            i += 1;
            if bytes[1] != 0 {
                let len = bytes[0].not().leading_zeros() as usize;
                let mut written = 1;
                while written < len {
                    if i < buf.len() {
                        buf[i] = bytes[written];
                        i += 1;
                        written += 1;
                    } else {
                        let bytes_as_u32 = u32::from_le_bytes(bytes);
                        self.prev = bytes_as_u32 >> (8*written);
                        return Ok(i);
                    }
                }
            }
        }
        Ok(i)
    }
}



/// An iterator over the `Utf8Char` of a string slice, and their positions.
///
/// This struct is created by the `utf8char_indices()` method from [`StrExt`](../trait.StrExt.html)
/// trait. See its documentation for more.
#[derive(Clone)]
pub struct Utf8CharIndices<'a>{
    str: &'a str,
    index: usize,
}
impl<'a> From<&'a str> for Utf8CharIndices<'a> {
    fn from(s: &str) -> Utf8CharIndices {
        Utf8CharIndices{str: s, index: 0}
    }
}
impl<'a> Utf8CharIndices<'a> {
    /// Extract the remainder of the source `str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::{StrExt, Utf8Char};
    /// let mut iter = "abc".utf8char_indices();
    /// assert_eq!(iter.next_back(), Some((2, Utf8Char::from('c'))));
    /// assert_eq!(iter.next(), Some((0, Utf8Char::from('a'))));
    /// assert_eq!(iter.as_str(), "b");
    /// ```
    pub fn as_str(&self) -> &'a str {
        &self.str[self.index..]
    }
}
impl<'a> Iterator for Utf8CharIndices<'a> {
    type Item = (usize,Utf8Char);
    fn next(&mut self) -> Option<(usize,Utf8Char)> {
        match Utf8Char::from_str_start(&self.str[self.index..]) {
            Ok((u8c, len)) => {
                let item = (self.index, u8c);
                self.index += len;
                Some(item)
            },
            Err(EmptyStrError) => None
        }
    }
    fn size_hint(&self) -> (usize,Option<usize>) {
        let len = self.str.len() - self.index;
        // For len+3 to overflow, the slice must fill all but two bytes of
        // addressable memory, and size_hint() doesn't need to be correct.
        (len.wrapping_add(3)/4, Some(len))
    }
}
impl<'a> DoubleEndedIterator for Utf8CharIndices<'a> {
    fn next_back(&mut self) -> Option<(usize,Utf8Char)> {
        // Cannot refactor out the unwrap without switching to ::from_slice()
        // since slicing the str panics if not on a boundary.
        if self.index < self.str.len() {
            let rev = self.str.bytes().rev();
            let len = 1 + rev.take_while(|b| b & 0b1100_0000 == 0b1000_0000 ).count();
            let starts = self.str.len() - len;
            let (u8c,_) = Utf8Char::from_str_start(&self.str[starts..]).unwrap();
            self.str = &self.str[..starts];
            Some((starts, u8c))
        } else {
            None
        }
    }
}
impl<'a> fmt::Debug for Utf8CharIndices<'a> {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_tuple("Utf8CharIndices")
            .field(&self.index)
            .field(&self.as_str())
            .finish()
    }
}


/// An iterator over the codepoints in a `str` represented as `Utf8Char`.
#[derive(Clone)]
pub struct Utf8Chars<'a>(Utf8CharIndices<'a>);
impl<'a> From<&'a str> for Utf8Chars<'a> {
    fn from(s: &str) -> Utf8Chars {
        Utf8Chars(Utf8CharIndices::from(s))
    }
}
impl<'a> Utf8Chars<'a> {
    /// Extract the remainder of the source `str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::{StrExt, Utf8Char};
    /// let mut iter = "abc".utf8chars();
    /// assert_eq!(iter.next(), Some(Utf8Char::from('a')));
    /// assert_eq!(iter.next_back(), Some(Utf8Char::from('c')));
    /// assert_eq!(iter.as_str(), "b");
    /// ```
    pub fn as_str(&self) -> &'a str {
        self.0.as_str()
    }
}
impl<'a> Iterator for Utf8Chars<'a> {
    type Item = Utf8Char;
    fn next(&mut self) -> Option<Utf8Char> {
        self.0.next().map(|(_,u8c)| u8c )
    }
    fn size_hint(&self) -> (usize,Option<usize>) {
        self.0.size_hint()
    }
}
impl<'a> DoubleEndedIterator for Utf8Chars<'a> {
    fn next_back(&mut self) -> Option<Utf8Char> {
        self.0.next_back().map(|(_,u8c)| u8c )
    }
}
impl<'a> fmt::Debug for Utf8Chars<'a> {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_tuple("Utf8CharIndices")
            .field(&self.as_str())
            .finish()
    }
}
