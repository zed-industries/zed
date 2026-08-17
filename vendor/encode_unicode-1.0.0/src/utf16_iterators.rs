/* Copyright 2018-2019 Torbjørn Birch Moltu
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

use crate::traits::CharExt;
use crate::utf16_char::Utf16Char;
use crate::errors::EmptyStrError;
extern crate core;
use core::fmt;
use core::borrow::Borrow;

// Invalid values that says the field is consumed or empty.
const FIRST_USED: u16 = 0x_dc_00;
const SECOND_USED: u16 = 0;

/// Iterate over the units of the UTF-16 representation of a codepoint.
#[derive(Clone)]
pub struct Utf16Iterator {
    first: u16,
    second: u16,
}
impl From<char> for Utf16Iterator {
    fn from(c: char) -> Self {
        Self::from(c.to_utf16())
    }
}
impl From<Utf16Char> for Utf16Iterator {
    fn from(uc: Utf16Char) -> Self {
        let (first, second) = uc.to_tuple();
        let second = second.unwrap_or(SECOND_USED);
        Utf16Iterator{first, second}
    }
}
impl Iterator for Utf16Iterator {
    type Item=u16;
    fn next(&mut self) -> Option<u16> {
        match (self.first, self.second) {
            (FIRST_USED, SECOND_USED)  =>  {                            None        },
            (FIRST_USED, second     )  =>  {self.second = SECOND_USED;  Some(second)},
            (first     ,      _     )  =>  {self.first = FIRST_USED;    Some(first )},
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}
impl ExactSizeIterator for Utf16Iterator {
    fn len(&self) -> usize {
        (if self.first == FIRST_USED {0} else {1}) +
        (if self.second == SECOND_USED {0} else {1})
    }
}
impl fmt::Debug for Utf16Iterator {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut clone = self.clone();
        match (clone.next(), clone.next()) {
            (Some(one), None)  => write!(fmtr, "[{}]", one),
            (Some(a), Some(b)) => write!(fmtr, "[{}, {}]", a, b),
            (None,  _)         => write!(fmtr, "[]"),
        }
    }
}



/// Converts an iterator of `Utf16Char` (or `&Utf16Char`)
/// to an iterator of `u16`s.
///
/// Is equivalent to calling `.flatten()` or `.flat_map()` on the original iterator,
/// but the returned iterator is about twice as fast.
///
/// The exact number of units cannot be known in advance, but `size_hint()`
/// gives the possible range.
///
/// # Examples
///
/// From iterator of values:
///
/// ```
/// use encode_unicode::{IterExt, CharExt};
///
/// let iterator = "foo".chars().map(|c| c.to_utf16() );
/// let mut units = [0; 4];
/// iterator.to_units().zip(&mut units).for_each(|(u,dst)| *dst = u );
/// assert_eq!(units, ['f' as u16, 'o' as u16, 'o' as u16, 0]);
/// ```
///
/// From iterator of references:
///
#[cfg_attr(feature="std", doc=" ```")]
#[cfg_attr(not(feature="std"), doc=" ```no_compile")]
/// use encode_unicode::{IterExt, CharExt, Utf16Char};
///
/// // (💣 takes two units)
/// let chars: Vec<Utf16Char> = "💣 bomb 💣".chars().map(|c| c.to_utf16() ).collect();
/// let units: Vec<u16> = chars.iter().to_units().collect();
/// let flat_map: Vec<u16> = chars.iter().cloned().flatten().collect();
/// assert_eq!(units, flat_map);
/// ```
#[derive(Clone)]
pub struct Utf16CharSplitter<U:Borrow<Utf16Char>, I:Iterator<Item=U>> {
    inner: I,
    prev_second: u16,
}
impl<U:Borrow<Utf16Char>, I:IntoIterator<Item=U>>
From<I> for Utf16CharSplitter<U, I::IntoIter> {
    fn from(iterable: I) -> Self {
        Utf16CharSplitter { inner: iterable.into_iter(),  prev_second: 0 }
    }
}
impl<U:Borrow<Utf16Char>, I:Iterator<Item=U>> Utf16CharSplitter<U,I> {
    /// Extracts the source iterator.
    ///
    /// Note that `iter.into_inner().to_units()` is not a no-op:  
    /// If the last returned unit from `next()` was a leading surrogate,
    /// the trailing surrogate is lost.
    pub fn into_inner(self) -> I {
        self.inner
    }
}
impl<U:Borrow<Utf16Char>, I:Iterator<Item=U>> Iterator for Utf16CharSplitter<U,I> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.prev_second == 0 {
            self.inner.next().map(|u16c| {
                let units = u16c.borrow().to_array();
                self.prev_second = units[1];
                units[0]
            })
        } else {
            let prev_second = self.prev_second;
            self.prev_second = 0;
            Some(prev_second)
        }
    }
    fn size_hint(&self) -> (usize,Option<usize>) {
        // Doesn't need to handle unlikely overflows correctly because
        // size_hint() cannot be relied upon anyway. (the trait isn't unsafe)
        let (min, max) = self.inner.size_hint();
        let add = if self.prev_second == 0 {0} else {1};
        (min.wrapping_add(add), max.map(|max| max.wrapping_mul(2).wrapping_add(add) ))
    }
}



/// An iterator over the codepoints in a `str` represented as `Utf16Char`.
#[derive(Clone)]
pub struct Utf16CharIndices<'a>{
    str: &'a str,
    index: usize,
}
impl<'a> From<&'a str> for Utf16CharIndices<'a> {
    fn from(s: &str) -> Utf16CharIndices {
        Utf16CharIndices{str: s, index: 0}
    }
}
impl<'a> Utf16CharIndices<'a> {
    /// Extract the remainder of the source `str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::{StrExt, Utf16Char};
    /// let mut iter = "abc".utf16char_indices();
    /// assert_eq!(iter.next_back(), Some((2, Utf16Char::from('c'))));
    /// assert_eq!(iter.next(), Some((0, Utf16Char::from('a'))));
    /// assert_eq!(iter.as_str(), "b");
    /// ```
    pub fn as_str(&self) -> &'a str {
        &self.str[self.index..]
    }
}
impl<'a> Iterator for Utf16CharIndices<'a> {
    type Item = (usize,Utf16Char);
    fn next(&mut self) -> Option<(usize,Utf16Char)> {
        match Utf16Char::from_str_start(&self.str[self.index..]) {
            Ok((u16c, bytes)) => {
                let item = (self.index, u16c);
                self.index += bytes;
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
impl<'a> DoubleEndedIterator for Utf16CharIndices<'a> {
    fn next_back(&mut self) -> Option<(usize,Utf16Char)> {
        if self.index < self.str.len() {
            let rev = self.str.bytes().rev();
            let len = 1 + rev.take_while(|b| b & 0b1100_0000 == 0b1000_0000 ).count();
            let starts = self.str.len() - len;
            let (u16c,_) = Utf16Char::from_str_start(&self.str[starts..]).unwrap();
            self.str = &self.str[..starts];
            Some((starts, u16c))
        } else {
            None
        }
    }
}
impl<'a> fmt::Debug for Utf16CharIndices<'a> {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_tuple("Utf16CharIndices")
            .field(&self.index)
            .field(&self.as_str())
            .finish()
    }
}


/// An iterator over the codepoints in a `str` represented as `Utf16Char`.
#[derive(Clone)]
pub struct Utf16Chars<'a>(Utf16CharIndices<'a>);
impl<'a> From<&'a str> for Utf16Chars<'a> {
    fn from(s: &str) -> Utf16Chars {
        Utf16Chars(Utf16CharIndices::from(s))
    }
}
impl<'a> Utf16Chars<'a> {
    /// Extract the remainder of the source `str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use encode_unicode::{StrExt, Utf16Char};
    /// let mut iter = "abc".utf16chars();
    /// assert_eq!(iter.next(), Some(Utf16Char::from('a')));
    /// assert_eq!(iter.next_back(), Some(Utf16Char::from('c')));
    /// assert_eq!(iter.as_str(), "b");
    /// ```
    pub fn as_str(&self) -> &'a str {
        self.0.as_str()
    }
}
impl<'a> Iterator for Utf16Chars<'a> {
    type Item = Utf16Char;
    fn next(&mut self) -> Option<Utf16Char> {
        self.0.next().map(|(_,u16c)| u16c )
    }
    fn size_hint(&self) -> (usize,Option<usize>) {
        self.0.size_hint()
    }
}
impl<'a> DoubleEndedIterator for Utf16Chars<'a> {
    fn next_back(&mut self) -> Option<Utf16Char> {
        self.0.next_back().map(|(_,u16c)| u16c )
    }
}
impl<'a> fmt::Debug for Utf16Chars<'a> {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_tuple("Utf16Chars")
            .field(&self.as_str())
            .finish()
    }
}
