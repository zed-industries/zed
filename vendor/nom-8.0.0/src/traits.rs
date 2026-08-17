//! Traits input types have to implement to work with nom combinators
use core::iter::Enumerate;
use core::str::CharIndices;

use crate::error::{ErrorKind, ParseError};
use crate::internal::{Err, IResult, Needed};
use crate::lib::std::iter::Copied;
use crate::lib::std::ops::{
  Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use crate::lib::std::slice::Iter;
use crate::lib::std::str::from_utf8;
use crate::lib::std::str::Chars;
use crate::lib::std::str::FromStr;
use crate::IsStreaming;
use crate::Mode;

#[cfg(feature = "alloc")]
use crate::lib::std::string::String;
#[cfg(feature = "alloc")]
use crate::lib::std::vec::Vec;

/// Parser input types must implement this trait
pub trait Input: Clone + Sized {
  /// The current input type is a sequence of that `Item` type.
  ///
  /// Example: `u8` for `&[u8]` or `char` for `&str`
  type Item;

  /// An iterator over the input type, producing the item
  type Iter: Iterator<Item = Self::Item>;

  /// An iterator over the input type, producing the item and its byte position
  /// If we're iterating over `&str`, the position
  /// corresponds to the byte index of the character
  type IterIndices: Iterator<Item = (usize, Self::Item)>;

  /// Calculates the input length, as indicated by its name,
  /// and the name of the trait itself
  fn input_len(&self) -> usize;

  /// Returns a slice of `index` bytes. panics if index > length
  fn take(&self, index: usize) -> Self;
  /// Returns a slice starting at `index` bytes. panics if index > length
  fn take_from(&self, index: usize) -> Self;
  /// Split the stream at the `index` byte offset. panics if index > length
  fn take_split(&self, index: usize) -> (Self, Self);

  /// Returns the byte position of the first element satisfying the predicate
  fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool;

  /// Returns an iterator over the elements
  fn iter_elements(&self) -> Self::Iter;
  /// Returns an iterator over the elements and their byte offsets
  fn iter_indices(&self) -> Self::IterIndices;

  /// Get the byte offset from the element's position in the stream
  fn slice_index(&self, count: usize) -> Result<usize, Needed>;

  /// Looks for the first element of the input type for which the condition returns true,
  /// and returns the input up to this position.
  ///
  /// *streaming version*: If no element is found matching the condition, this will return `Incomplete`
  fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.position(predicate) {
      Some(n) => Ok(self.take_split(n)),
      None => Err(Err::Incomplete(Needed::new(1))),
    }
  }

  /// Looks for the first element of the input type for which the condition returns true
  /// and returns the input up to this position.
  ///
  /// Fails if the produced slice is empty.
  ///
  /// *streaming version*: If no element is found matching the condition, this will return `Incomplete`
  fn split_at_position1<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.position(predicate) {
      Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
      Some(n) => Ok(self.take_split(n)),
      None => Err(Err::Incomplete(Needed::new(1))),
    }
  }

  /// Looks for the first element of the input type for which the condition returns true,
  /// and returns the input up to this position.
  ///
  /// *complete version*: If no element is found matching the condition, this will return the whole input
  fn split_at_position_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.split_at_position(predicate) {
      Err(Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
      res => res,
    }
  }

  /// Looks for the first element of the input type for which the condition returns true
  /// and returns the input up to this position.
  ///
  /// Fails if the produced slice is empty.
  ///
  /// *complete version*: If no element is found matching the condition, this will return the whole input
  fn split_at_position1_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.split_at_position1(predicate, e) {
      Err(Err::Incomplete(_)) => {
        if self.input_len() == 0 {
          Err(Err::Error(E::from_error_kind(self.clone(), e)))
        } else {
          Ok(self.take_split(self.input_len()))
        }
      }
      res => res,
    }
  }

  /// mode version of split_at_position
  fn split_at_position_mode<OM: crate::OutputMode, P, E: ParseError<Self>>(
    &self,
    predicate: P,
  ) -> crate::PResult<OM, Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.position(predicate) {
      Some(n) => Ok((self.take_from(n), OM::Output::bind(|| self.take(n)))),
      None => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::new(1)))
        } else {
          let len = self.input_len();
          Ok((self.take_from(len), OM::Output::bind(|| self.take(len))))
        }
      }
    }
  }

  /// mode version of split_at_position
  fn split_at_position_mode1<OM: crate::OutputMode, P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> crate::PResult<OM, Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.position(predicate) {
      Some(0) => Err(Err::Error(OM::Error::bind(|| {
        E::from_error_kind(self.clone(), e)
      }))),
      Some(n) => Ok((self.take_from(n), OM::Output::bind(|| self.take(n)))),
      None => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::new(1)))
        } else {
          let len = self.input_len();
          if len == 0 {
            Err(Err::Error(OM::Error::bind(|| {
              E::from_error_kind(self.clone(), e)
            })))
          } else {
            Ok((self.take_from(len), OM::Output::bind(|| self.take(len))))
          }
        }
      }
    }
  }
}

impl<'a> Input for &'a [u8] {
  type Item = u8;
  type Iter = Copied<Iter<'a, u8>>;
  type IterIndices = Enumerate<Self::Iter>;

  fn input_len(&self) -> usize {
    self.len()
  }

  #[inline]
  fn take(&self, index: usize) -> Self {
    &self[0..index]
  }

  fn take_from(&self, index: usize) -> Self {
    &self[index..]
  }
  #[inline]
  fn take_split(&self, index: usize) -> (Self, Self) {
    let (prefix, suffix) = self.split_at(index);
    (suffix, prefix)
  }

  #[inline]
  fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool,
  {
    self.iter().position(|b| predicate(*b))
  }

  #[inline]
  fn iter_elements(&self) -> Self::Iter {
    self.iter().copied()
  }

  #[inline]
  fn iter_indices(&self) -> Self::IterIndices {
    self.iter_elements().enumerate()
  }

  #[inline]
  fn slice_index(&self, count: usize) -> Result<usize, Needed> {
    if self.len() >= count {
      Ok(count)
    } else {
      Err(Needed::new(count - self.len()))
    }
  }

  #[inline(always)]
  fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.iter().position(|c| predicate(*c)) {
      Some(i) => Ok(self.take_split(i)),
      None => Err(Err::Incomplete(Needed::new(1))),
    }
  }

  #[inline(always)]
  fn split_at_position1<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.iter().position(|c| predicate(*c)) {
      Some(0) => Err(Err::Error(E::from_error_kind(self, e))),
      Some(i) => Ok(self.take_split(i)),
      None => Err(Err::Incomplete(Needed::new(1))),
    }
  }

  fn split_at_position_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.iter().position(|c| predicate(*c)) {
      Some(i) => Ok(self.take_split(i)),
      None => Ok(self.take_split(self.len())),
    }
  }

  #[inline(always)]
  fn split_at_position1_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.iter().position(|c| predicate(*c)) {
      Some(0) => Err(Err::Error(E::from_error_kind(self, e))),
      Some(i) => Ok(self.take_split(i)),
      None => {
        if self.is_empty() {
          Err(Err::Error(E::from_error_kind(self, e)))
        } else {
          Ok(self.take_split(self.len()))
        }
      }
    }
  }

  /// mode version of split_at_position
  #[inline(always)]
  fn split_at_position_mode<OM: crate::OutputMode, P, E: ParseError<Self>>(
    &self,
    predicate: P,
  ) -> crate::PResult<OM, Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.iter().position(|c| predicate(*c)) {
      Some(n) => Ok((self.take_from(n), OM::Output::bind(|| self.take(n)))),
      None => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::new(1)))
        } else {
          Ok((
            self.take_from(self.len()),
            OM::Output::bind(|| self.take(self.len())),
          ))
        }
      }
    }
  }

  /// mode version of split_at_position
  #[inline(always)]
  fn split_at_position_mode1<OM: crate::OutputMode, P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> crate::PResult<OM, Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.iter().position(|c| predicate(*c)) {
      Some(0) => Err(Err::Error(OM::Error::bind(|| E::from_error_kind(self, e)))),
      Some(n) => Ok((self.take_from(n), OM::Output::bind(|| self.take(n)))),
      None => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::new(1)))
        } else if self.is_empty() {
          Err(Err::Error(OM::Error::bind(|| E::from_error_kind(self, e))))
        } else {
          Ok((
            self.take_from(self.len()),
            OM::Output::bind(|| self.take(self.len())),
          ))
        }
      }
    }
  }
}

impl<'a> Input for &'a str {
  type Item = char;
  type Iter = Chars<'a>;
  type IterIndices = CharIndices<'a>;

  fn input_len(&self) -> usize {
    self.len()
  }

  #[inline]
  fn take(&self, index: usize) -> Self {
    &self[..index]
  }

  #[inline]
  fn take_from(&self, index: usize) -> Self {
    &self[index..]
  }

  // return byte index
  #[inline]
  fn take_split(&self, index: usize) -> (Self, Self) {
    let (prefix, suffix) = self.split_at(index);
    (suffix, prefix)
  }

  fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool,
  {
    self.find(predicate)
  }

  #[inline]
  fn iter_elements(&self) -> Self::Iter {
    self.chars()
  }

  #[inline]
  fn iter_indices(&self) -> Self::IterIndices {
    self.char_indices()
  }

  #[inline]
  fn slice_index(&self, count: usize) -> Result<usize, Needed> {
    let mut cnt = 0;
    for (index, _) in self.char_indices() {
      if cnt == count {
        return Ok(index);
      }
      cnt += 1;
    }
    if cnt == count {
      return Ok(self.len());
    }
    Err(Needed::Unknown)
  }

  #[inline(always)]
  fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.find(predicate) {
      // The position i is returned from str::find() which means it is within the bounds of the string
      Some(i) => {
        let (str1, str2) = self.split_at(i);
        Ok((str2, str1))
      }
      None => Err(Err::Incomplete(Needed::new(1))),
    }
  }

  #[inline(always)]
  fn split_at_position1<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.find(predicate) {
      Some(0) => Err(Err::Error(E::from_error_kind(self, e))),
      // The position i is returned from str::find() which means it is within the bounds of the string
      Some(i) => {
        let (str1, str2) = self.split_at(i);
        Ok((str2, str1))
      }
      None => Err(Err::Incomplete(Needed::new(1))),
    }
  }

  #[inline(always)]
  fn split_at_position_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.find(predicate) {
      // The position i is returned from str::find() which means it is within the bounds of the string
      Some(i) => {
        let (str1, str2) = self.split_at(i);
        Ok((str2, str1))
      }
      None => Ok(self.split_at(0)),
    }
  }

  #[inline(always)]
  fn split_at_position1_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.find(predicate) {
      Some(0) => Err(Err::Error(E::from_error_kind(self, e))),
      // The position i is returned from str::find() which means it is within the bounds of the string
      Some(i) => {
        let (str1, str2) = self.split_at(i);
        Ok((str2, str1))
      }
      None => {
        if self.is_empty() {
          Err(Err::Error(E::from_error_kind(self, e)))
        } else {
          // the end of slice is a char boundary
          let (str1, str2) = self.split_at(self.len());
          Ok((str2, str1))
        }
      }
    }
  }

  /// mode version of split_at_position
  #[inline(always)]
  fn split_at_position_mode<OM: crate::OutputMode, P, E: ParseError<Self>>(
    &self,
    predicate: P,
  ) -> crate::PResult<OM, Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.find(predicate) {
      Some(n) => unsafe {
        // find() returns a byte index that is already in the slice at a char boundary
        Ok((
          self.get_unchecked(n..),
          OM::Output::bind(|| self.get_unchecked(..n)),
        ))
      },
      None => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::new(1)))
        } else {
          // the end of slice is a char boundary
          unsafe {
            Ok((
              self.get_unchecked(self.len()..),
              OM::Output::bind(|| self.get_unchecked(..self.len())),
            ))
          }
        }
      }
    }
  }

  /// mode version of split_at_position
  #[inline(always)]
  fn split_at_position_mode1<OM: crate::OutputMode, P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> crate::PResult<OM, Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.find(predicate) {
      Some(0) => Err(Err::Error(OM::Error::bind(|| E::from_error_kind(self, e)))),
      Some(n) => unsafe {
        // find() returns a byte index that is already in the slice at a char boundary
        Ok((
          self.get_unchecked(n..),
          OM::Output::bind(|| self.get_unchecked(..n)),
        ))
      },
      None => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::new(1)))
        } else if self.is_empty() {
          Err(Err::Error(OM::Error::bind(|| E::from_error_kind(self, e))))
        } else {
          // the end of slice is a char boundary
          unsafe {
            Ok((
              self.get_unchecked(self.len()..),
              OM::Output::bind(|| self.get_unchecked(..self.len())),
            ))
          }
        }
      }
    }
  }
}

/// Useful functions to calculate the offset between slices and show a hexdump of a slice
pub trait Offset {
  /// Offset between the first byte of self and the first byte of the argument
  /// the argument must be a part of self, otherwise this can fail with arithmetic
  /// underflows as it compares byte offsets
  fn offset(&self, second: &Self) -> usize;
}

impl Offset for [u8] {
  fn offset(&self, second: &Self) -> usize {
    let fst = self.as_ptr();
    let snd = second.as_ptr();

    snd as usize - fst as usize
  }
}

impl<'a> Offset for &'a [u8] {
  fn offset(&self, second: &Self) -> usize {
    let fst = self.as_ptr();
    let snd = second.as_ptr();

    snd as usize - fst as usize
  }
}

impl Offset for str {
  fn offset(&self, second: &Self) -> usize {
    let fst = self.as_ptr();
    let snd = second.as_ptr();

    snd as usize - fst as usize
  }
}

impl<'a> Offset for &'a str {
  fn offset(&self, second: &Self) -> usize {
    let fst = self.as_ptr();
    let snd = second.as_ptr();

    snd as usize - fst as usize
  }
}

/// Helper trait for types that can be viewed as a byte slice
pub trait AsBytes {
  /// Casts the input type to a byte slice
  fn as_bytes(&self) -> &[u8];
}

impl<'a> AsBytes for &'a str {
  #[inline(always)]
  fn as_bytes(&self) -> &[u8] {
    (*self).as_bytes()
  }
}

impl AsBytes for str {
  #[inline(always)]
  fn as_bytes(&self) -> &[u8] {
    self.as_ref()
  }
}

impl<'a> AsBytes for &'a [u8] {
  #[inline(always)]
  fn as_bytes(&self) -> &[u8] {
    self
  }
}

impl AsBytes for [u8] {
  #[inline(always)]
  fn as_bytes(&self) -> &[u8] {
    self
  }
}

impl<'a, const N: usize> AsBytes for &'a [u8; N] {
  #[inline(always)]
  fn as_bytes(&self) -> &[u8] {
    self.as_slice()
  }
}

impl<const N: usize> AsBytes for [u8; N] {
  #[inline(always)]
  fn as_bytes(&self) -> &[u8] {
    self
  }
}

/// Transforms common types to a char for basic token parsing
#[allow(clippy::len_without_is_empty)]
pub trait AsChar: Copy {
  /// makes a char from self
  fn as_char(self) -> char;

  /// Tests that self is an alphabetic character
  ///
  /// Warning: for `&str` it recognizes alphabetic
  /// characters outside of the 52 ASCII letters
  fn is_alpha(self) -> bool;

  /// Tests that self is an alphabetic character
  /// or a decimal digit
  fn is_alphanum(self) -> bool;
  /// Tests that self is a decimal digit
  fn is_dec_digit(self) -> bool;
  /// Tests that self is an hex digit
  fn is_hex_digit(self) -> bool;
  /// Tests that self is an octal digit
  fn is_oct_digit(self) -> bool;
  /// Tests that self is a binary digit
  fn is_bin_digit(self) -> bool;
  /// Gets the len in bytes for self
  fn len(self) -> usize;
  /// Tests that self is ASCII space or tab
  fn is_space(self) -> bool;
  /// Tests if byte is ASCII newline: \n
  fn is_newline(self) -> bool;
}

impl AsChar for u8 {
  #[inline]
  fn as_char(self) -> char {
    self as char
  }
  #[inline]
  fn is_alpha(self) -> bool {
    matches!(self, 0x41..=0x5A | 0x61..=0x7A)
  }
  #[inline]
  fn is_alphanum(self) -> bool {
    self.is_alpha() || self.is_dec_digit()
  }
  #[inline]
  fn is_dec_digit(self) -> bool {
    matches!(self, 0x30..=0x39)
  }
  #[inline]
  fn is_hex_digit(self) -> bool {
    matches!(self, 0x30..=0x39 | 0x41..=0x46 | 0x61..=0x66)
  }
  #[inline]
  fn is_oct_digit(self) -> bool {
    matches!(self, 0x30..=0x37)
  }
  #[inline]
  fn is_bin_digit(self) -> bool {
    matches!(self, 0x30..=0x31)
  }
  #[inline]
  fn len(self) -> usize {
    1
  }
  #[inline]
  fn is_space(self) -> bool {
    self == b' ' || self == b'\t'
  }
  fn is_newline(self) -> bool {
    self == b'\n'
  }
}
impl<'a> AsChar for &'a u8 {
  #[inline]
  fn as_char(self) -> char {
    *self as char
  }
  #[inline]
  fn is_alpha(self) -> bool {
    matches!(*self, 0x41..=0x5A | 0x61..=0x7A)
  }
  #[inline]
  fn is_alphanum(self) -> bool {
    self.is_alpha() || self.is_dec_digit()
  }
  #[inline]
  fn is_dec_digit(self) -> bool {
    matches!(*self, 0x30..=0x39)
  }
  #[inline]
  fn is_hex_digit(self) -> bool {
    matches!(*self, 0x30..=0x39 | 0x41..=0x46 | 0x61..=0x66)
  }
  #[inline]
  fn is_oct_digit(self) -> bool {
    matches!(*self, 0x30..=0x37)
  }
  #[inline]
  fn is_bin_digit(self) -> bool {
    matches!(*self, 0x30..=0x31)
  }
  #[inline]
  fn len(self) -> usize {
    1
  }
  #[inline]
  fn is_space(self) -> bool {
    *self == b' ' || *self == b'\t'
  }
  fn is_newline(self) -> bool {
    *self == b'\n'
  }
}

impl AsChar for char {
  #[inline]
  fn as_char(self) -> char {
    self
  }
  #[inline]
  fn is_alpha(self) -> bool {
    self.is_ascii_alphabetic()
  }
  #[inline]
  fn is_alphanum(self) -> bool {
    self.is_alpha() || self.is_dec_digit()
  }
  #[inline]
  fn is_dec_digit(self) -> bool {
    self.is_ascii_digit()
  }
  #[inline]
  fn is_hex_digit(self) -> bool {
    self.is_ascii_hexdigit()
  }
  #[inline]
  fn is_oct_digit(self) -> bool {
    self.is_digit(8)
  }
  #[inline]
  fn is_bin_digit(self) -> bool {
    self.is_digit(2)
  }
  #[inline]
  fn len(self) -> usize {
    self.len_utf8()
  }
  #[inline]
  fn is_space(self) -> bool {
    self == ' ' || self == '\t'
  }
  fn is_newline(self) -> bool {
    self == '\n'
  }
}

impl<'a> AsChar for &'a char {
  #[inline]
  fn as_char(self) -> char {
    *self
  }
  #[inline]
  fn is_alpha(self) -> bool {
    self.is_ascii_alphabetic()
  }
  #[inline]
  fn is_alphanum(self) -> bool {
    self.is_alpha() || self.is_dec_digit()
  }
  #[inline]
  fn is_dec_digit(self) -> bool {
    self.is_ascii_digit()
  }
  #[inline]
  fn is_hex_digit(self) -> bool {
    self.is_ascii_hexdigit()
  }
  #[inline]
  fn is_oct_digit(self) -> bool {
    self.is_digit(8)
  }
  #[inline]
  fn is_bin_digit(self) -> bool {
    self.is_digit(2)
  }
  #[inline]
  fn len(self) -> usize {
    self.len_utf8()
  }
  #[inline]
  fn is_space(self) -> bool {
    *self == ' ' || *self == '\t'
  }
  fn is_newline(self) -> bool {
    *self == '\n'
  }
}

/// Indicates whether a comparison was successful, an error, or
/// if more data was needed
#[derive(Debug, Eq, PartialEq)]
pub enum CompareResult {
  /// Comparison was successful
  Ok,
  /// We need more data to be sure
  Incomplete,
  /// Comparison failed
  Error,
}

/// Abstracts comparison operations
pub trait Compare<T> {
  /// Compares self to another value for equality
  fn compare(&self, t: T) -> CompareResult;
  /// Compares self to another value for equality
  /// independently of the case.
  ///
  /// Warning: for `&str`, the comparison is done
  /// by lowercasing both strings and comparing
  /// the result. This is a temporary solution until
  /// a better one appears
  fn compare_no_case(&self, t: T) -> CompareResult;
}

fn lowercase_byte(c: u8) -> u8 {
  match c {
    b'A'..=b'Z' => c - b'A' + b'a',
    _ => c,
  }
}

impl<'a, 'b> Compare<&'b [u8]> for &'a [u8] {
  #[inline(always)]
  fn compare(&self, t: &'b [u8]) -> CompareResult {
    let pos = self.iter().zip(t.iter()).position(|(a, b)| a != b);

    match pos {
      Some(_) => CompareResult::Error,
      None => {
        if self.len() >= t.len() {
          CompareResult::Ok
        } else {
          CompareResult::Incomplete
        }
      }
    }
  }

  #[inline(always)]
  fn compare_no_case(&self, t: &'b [u8]) -> CompareResult {
    if self
      .iter()
      .zip(t)
      .any(|(a, b)| lowercase_byte(*a) != lowercase_byte(*b))
    {
      CompareResult::Error
    } else if self.len() < t.len() {
      CompareResult::Incomplete
    } else {
      CompareResult::Ok
    }
  }
}

impl<'a, 'b> Compare<&'b str> for &'a [u8] {
  #[inline(always)]
  fn compare(&self, t: &'b str) -> CompareResult {
    self.compare(AsBytes::as_bytes(t))
  }
  #[inline(always)]
  fn compare_no_case(&self, t: &'b str) -> CompareResult {
    self.compare_no_case(AsBytes::as_bytes(t))
  }
}

impl<'a, 'b> Compare<&'b str> for &'a str {
  #[inline(always)]
  fn compare(&self, t: &'b str) -> CompareResult {
    self.as_bytes().compare(t.as_bytes())
  }

  //FIXME: this version is too simple and does not use the current locale
  #[inline(always)]
  fn compare_no_case(&self, t: &'b str) -> CompareResult {
    let pos = self
      .chars()
      .zip(t.chars())
      .position(|(a, b)| a.to_lowercase().ne(b.to_lowercase()));

    match pos {
      Some(_) => CompareResult::Error,
      None => {
        if self.len() >= t.len() {
          CompareResult::Ok
        } else {
          CompareResult::Incomplete
        }
      }
    }
  }
}

impl<'a, 'b> Compare<&'b [u8]> for &'a str {
  #[inline(always)]
  fn compare(&self, t: &'b [u8]) -> CompareResult {
    AsBytes::as_bytes(self).compare(t)
  }
  #[inline(always)]
  fn compare_no_case(&self, t: &'b [u8]) -> CompareResult {
    AsBytes::as_bytes(self).compare_no_case(t)
  }
}

/// Look for a token in self
pub trait FindToken<T> {
  /// Returns true if self contains the token
  fn find_token(&self, token: T) -> bool;
}

impl<'a> FindToken<u8> for &'a [u8] {
  fn find_token(&self, token: u8) -> bool {
    memchr::memchr(token, self).is_some()
  }
}

impl<'a> FindToken<u8> for &'a str {
  fn find_token(&self, token: u8) -> bool {
    self.as_bytes().find_token(token)
  }
}

impl<'a, 'b> FindToken<&'a u8> for &'b [u8] {
  fn find_token(&self, token: &u8) -> bool {
    self.find_token(*token)
  }
}

impl<'a, 'b> FindToken<&'a u8> for &'b str {
  fn find_token(&self, token: &u8) -> bool {
    self.as_bytes().find_token(token)
  }
}

impl<'a> FindToken<char> for &'a [u8] {
  fn find_token(&self, token: char) -> bool {
    self.iter().any(|i| *i == token as u8)
  }
}

impl<'a> FindToken<char> for &'a str {
  fn find_token(&self, token: char) -> bool {
    self.chars().any(|i| i == token)
  }
}

impl<'a> FindToken<char> for &'a [char] {
  fn find_token(&self, token: char) -> bool {
    self.iter().any(|i| *i == token)
  }
}

impl<'a, 'b> FindToken<&'a char> for &'b [char] {
  fn find_token(&self, token: &char) -> bool {
    self.find_token(*token)
  }
}

/// Look for a substring in self
pub trait FindSubstring<T> {
  /// Returns the byte position of the substring if it is found
  fn find_substring(&self, substr: T) -> Option<usize>;
}

impl<'a, 'b> FindSubstring<&'b [u8]> for &'a [u8] {
  fn find_substring(&self, substr: &'b [u8]) -> Option<usize> {
    if substr.len() > self.len() {
      return None;
    }

    let (&substr_first, substr_rest) = match substr.split_first() {
      Some(split) => split,
      // an empty substring is found at position 0
      // This matches the behavior of str.find("").
      None => return Some(0),
    };

    if substr_rest.is_empty() {
      return memchr::memchr(substr_first, self);
    }

    let mut offset = 0;
    let haystack = &self[..self.len() - substr_rest.len()];

    while let Some(position) = memchr::memchr(substr_first, &haystack[offset..]) {
      offset += position;
      let next_offset = offset + 1;
      if &self[next_offset..][..substr_rest.len()] == substr_rest {
        return Some(offset);
      }

      offset = next_offset;
    }

    None
  }
}

impl<'a, 'b> FindSubstring<&'b str> for &'a [u8] {
  fn find_substring(&self, substr: &'b str) -> Option<usize> {
    self.find_substring(AsBytes::as_bytes(substr))
  }
}

impl<'a, 'b> FindSubstring<&'b str> for &'a str {
  //returns byte index
  fn find_substring(&self, substr: &'b str) -> Option<usize> {
    self.find(substr)
  }
}

/// Used to integrate `str`'s `parse()` method
pub trait ParseTo<R> {
  /// Succeeds if `parse()` succeeded. The byte slice implementation
  /// will first convert it to a `&str`, then apply the `parse()` function
  fn parse_to(&self) -> Option<R>;
}

impl<'a, R: FromStr> ParseTo<R> for &'a [u8] {
  fn parse_to(&self) -> Option<R> {
    from_utf8(self).ok().and_then(|s| s.parse().ok())
  }
}

impl<'a, R: FromStr> ParseTo<R> for &'a str {
  fn parse_to(&self) -> Option<R> {
    self.parse().ok()
  }
}

impl<'a, const N: usize> Compare<[u8; N]> for &'a [u8] {
  #[inline(always)]
  fn compare(&self, t: [u8; N]) -> CompareResult {
    self.compare(&t[..])
  }

  #[inline(always)]
  fn compare_no_case(&self, t: [u8; N]) -> CompareResult {
    self.compare_no_case(&t[..])
  }
}

impl<'a, 'b, const N: usize> Compare<&'b [u8; N]> for &'a [u8] {
  #[inline(always)]
  fn compare(&self, t: &'b [u8; N]) -> CompareResult {
    self.compare(&t[..])
  }

  #[inline(always)]
  fn compare_no_case(&self, t: &'b [u8; N]) -> CompareResult {
    self.compare_no_case(&t[..])
  }
}

impl<const N: usize> FindToken<u8> for [u8; N] {
  fn find_token(&self, token: u8) -> bool {
    memchr::memchr(token, &self[..]).is_some()
  }
}

impl<'a, const N: usize> FindToken<&'a u8> for [u8; N] {
  fn find_token(&self, token: &u8) -> bool {
    self.find_token(*token)
  }
}

/// Abstracts something which can extend an `Extend`.
/// Used to build modified input slices in `escaped_transform`
pub trait ExtendInto {
  /// The current input type is a sequence of that `Item` type.
  ///
  /// Example: `u8` for `&[u8]` or `char` for `&str`
  type Item;

  /// The type that will be produced
  type Extender;

  /// Create a new `Extend` of the correct type
  fn new_builder(&self) -> Self::Extender;
  /// Accumulate the input into an accumulator
  fn extend_into(&self, acc: &mut Self::Extender);
}

#[cfg(feature = "alloc")]
impl ExtendInto for [u8] {
  type Item = u8;
  type Extender = Vec<u8>;

  #[inline]
  fn new_builder(&self) -> Vec<u8> {
    Vec::new()
  }
  #[inline]
  fn extend_into(&self, acc: &mut Vec<u8>) {
    acc.extend(self.iter().cloned());
  }
}

#[cfg(feature = "alloc")]
impl ExtendInto for &[u8] {
  type Item = u8;
  type Extender = Vec<u8>;

  #[inline]
  fn new_builder(&self) -> Vec<u8> {
    Vec::new()
  }
  #[inline]
  fn extend_into(&self, acc: &mut Vec<u8>) {
    acc.extend_from_slice(self);
  }
}

#[cfg(feature = "alloc")]
impl ExtendInto for str {
  type Item = char;
  type Extender = String;

  #[inline]
  fn new_builder(&self) -> String {
    String::new()
  }
  #[inline]
  fn extend_into(&self, acc: &mut String) {
    acc.push_str(self);
  }
}

#[cfg(feature = "alloc")]
impl ExtendInto for &str {
  type Item = char;
  type Extender = String;

  #[inline]
  fn new_builder(&self) -> String {
    String::new()
  }
  #[inline]
  fn extend_into(&self, acc: &mut String) {
    acc.push_str(self);
  }
}

#[cfg(feature = "alloc")]
impl ExtendInto for char {
  type Item = char;
  type Extender = String;

  #[inline]
  fn new_builder(&self) -> String {
    String::new()
  }
  #[inline]
  fn extend_into(&self, acc: &mut String) {
    acc.push(*self);
  }
}

/// Helper trait to convert numbers to usize.
///
/// By default, usize implements `From<u8>` and `From<u16>` but not
/// `From<u32>` and `From<u64>` because that would be invalid on some
/// platforms. This trait implements the conversion for platforms
/// with 32 and 64 bits pointer platforms
pub trait ToUsize {
  /// converts self to usize
  fn to_usize(&self) -> usize;
}

impl ToUsize for u8 {
  #[inline]
  fn to_usize(&self) -> usize {
    *self as usize
  }
}

impl ToUsize for u16 {
  #[inline]
  fn to_usize(&self) -> usize {
    *self as usize
  }
}

impl ToUsize for usize {
  #[inline]
  fn to_usize(&self) -> usize {
    *self
  }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl ToUsize for u32 {
  #[inline]
  fn to_usize(&self) -> usize {
    *self as usize
  }
}

#[cfg(target_pointer_width = "64")]
impl ToUsize for u64 {
  #[inline]
  fn to_usize(&self) -> usize {
    *self as usize
  }
}

/// Equivalent From implementation to avoid orphan rules in bits parsers
pub trait ErrorConvert<E> {
  /// Transform to another error type
  fn convert(self) -> E;
}

impl<I> ErrorConvert<(I, ErrorKind)> for ((I, usize), ErrorKind) {
  fn convert(self) -> (I, ErrorKind) {
    ((self.0).0, self.1)
  }
}

impl<I> ErrorConvert<((I, usize), ErrorKind)> for (I, ErrorKind) {
  fn convert(self) -> ((I, usize), ErrorKind) {
    ((self.0, 0), self.1)
  }
}

use crate::error;
impl<I> ErrorConvert<error::Error<I>> for error::Error<(I, usize)> {
  fn convert(self) -> error::Error<I> {
    error::Error {
      input: self.input.0,
      code: self.code,
    }
  }
}

impl<I> ErrorConvert<error::Error<(I, usize)>> for error::Error<I> {
  fn convert(self) -> error::Error<(I, usize)> {
    error::Error {
      input: (self.input, 0),
      code: self.code,
    }
  }
}

impl ErrorConvert<()> for () {
  fn convert(self) {}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "std")))]
/// Helper trait to show a byte slice as a hex dump
pub trait HexDisplay {
  /// Converts the value of `self` to a hex dump, returning the owned
  /// `String`.
  fn to_hex(&self, chunk_size: usize) -> String;

  /// Converts the value of `self` to a hex dump beginning at `from` address, returning the owned
  /// `String`.
  fn to_hex_from(&self, chunk_size: usize, from: usize) -> String;
}

#[cfg(feature = "std")]
static CHARS: &[u8] = b"0123456789abcdef";

#[cfg(feature = "std")]
impl HexDisplay for [u8] {
  #[allow(unused_variables)]
  fn to_hex(&self, chunk_size: usize) -> String {
    self.to_hex_from(chunk_size, 0)
  }

  #[allow(unused_variables)]
  fn to_hex_from(&self, chunk_size: usize, from: usize) -> String {
    let mut v = Vec::with_capacity(self.len() * 3);
    let mut i = from;
    for chunk in self.chunks(chunk_size) {
      let s = format!("{:08x}", i);
      for &ch in s.as_bytes().iter() {
        v.push(ch);
      }
      v.push(b'\t');

      i += chunk_size;

      for &byte in chunk {
        v.push(CHARS[(byte >> 4) as usize]);
        v.push(CHARS[(byte & 0xf) as usize]);
        v.push(b' ');
      }
      if chunk_size > chunk.len() {
        for j in 0..(chunk_size - chunk.len()) {
          v.push(b' ');
          v.push(b' ');
          v.push(b' ');
        }
      }
      v.push(b'\t');

      for &byte in chunk {
        if matches!(byte, 32..=126 | 128..=255) {
          v.push(byte);
        } else {
          v.push(b'.');
        }
      }
      v.push(b'\n');
    }

    String::from_utf8_lossy(&v[..]).into_owned()
  }
}

#[cfg(feature = "std")]
impl HexDisplay for str {
  #[allow(unused_variables)]
  fn to_hex(&self, chunk_size: usize) -> String {
    self.to_hex_from(chunk_size, 0)
  }

  #[allow(unused_variables)]
  fn to_hex_from(&self, chunk_size: usize, from: usize) -> String {
    self.as_bytes().to_hex_from(chunk_size, from)
  }
}

/// A saturating iterator for usize.
pub struct SaturatingIterator {
  count: usize,
}

impl Iterator for SaturatingIterator {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    let old_count = self.count;
    self.count = self.count.saturating_add(1);
    Some(old_count)
  }
}

/// Abstractions for range-like types.
pub trait NomRange<Idx> {
  /// The saturating iterator type.
  type Saturating: Iterator<Item = Idx>;
  /// The bounded iterator type.
  type Bounded: Iterator<Item = Idx>;

  /// `true` if `item` is contained in the range.
  fn contains(&self, item: &Idx) -> bool;

  /// Returns the bounds of this range.
  fn bounds(&self) -> (Bound<Idx>, Bound<Idx>);

  /// `true` if the range is inverted.
  fn is_inverted(&self) -> bool;

  /// Creates a saturating iterator.
  /// A saturating iterator counts the number of iterations starting from 0 up to the upper bound of this range.
  /// If the upper bound is infinite the iterator saturates at the largest representable value of its type and
  /// returns it for all further elements.
  fn saturating_iter(&self) -> Self::Saturating;

  /// Creates a bounded iterator.
  /// A bounded iterator counts the number of iterations starting from 0 up to the upper bound of this range.
  /// If the upper bounds is infinite the iterator counts up until the amount of iterations has reached the
  /// largest representable value of its type and then returns `None` for all further elements.
  fn bounded_iter(&self) -> Self::Bounded;
}

impl NomRange<usize> for Range<usize> {
  type Saturating = Range<usize>;
  type Bounded = Range<usize>;

  fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
    (Bound::Included(self.start), Bound::Excluded(self.end))
  }

  fn contains(&self, item: &usize) -> bool {
    RangeBounds::contains(self, item)
  }

  fn is_inverted(&self) -> bool {
    self.start >= self.end
  }

  fn saturating_iter(&self) -> Self::Saturating {
    if self.end == 0 {
      Range::default()
    } else {
      0..self.end - 1
    }
  }

  fn bounded_iter(&self) -> Self::Bounded {
    if self.end == 0 {
      Range::default()
    } else {
      0..self.end - 1
    }
  }
}

impl NomRange<usize> for RangeInclusive<usize> {
  type Saturating = Range<usize>;
  type Bounded = Range<usize>;

  fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
    (Bound::Included(*self.start()), Bound::Included(*self.end()))
  }

  fn contains(&self, item: &usize) -> bool {
    RangeBounds::contains(self, item)
  }

  fn is_inverted(&self) -> bool {
    !RangeInclusive::contains(self, self.start())
  }

  fn saturating_iter(&self) -> Self::Saturating {
    0..*self.end()
  }

  fn bounded_iter(&self) -> Self::Bounded {
    0..*self.end()
  }
}

impl NomRange<usize> for RangeFrom<usize> {
  type Saturating = SaturatingIterator;
  type Bounded = Range<usize>;

  fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
    (Bound::Included(self.start), Bound::Unbounded)
  }

  fn contains(&self, item: &usize) -> bool {
    RangeBounds::contains(self, item)
  }

  fn is_inverted(&self) -> bool {
    false
  }

  fn saturating_iter(&self) -> Self::Saturating {
    SaturatingIterator { count: 0 }
  }

  fn bounded_iter(&self) -> Self::Bounded {
    0..usize::MAX
  }
}

impl NomRange<usize> for RangeTo<usize> {
  type Saturating = Range<usize>;
  type Bounded = Range<usize>;

  fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
    (Bound::Unbounded, Bound::Excluded(self.end))
  }

  fn contains(&self, item: &usize) -> bool {
    RangeBounds::contains(self, item)
  }

  fn is_inverted(&self) -> bool {
    false
  }

  fn saturating_iter(&self) -> Self::Saturating {
    if self.end == 0 {
      Range::default()
    } else {
      0..self.end - 1
    }
  }

  fn bounded_iter(&self) -> Self::Bounded {
    if self.end == 0 {
      Range::default()
    } else {
      0..self.end - 1
    }
  }
}

impl NomRange<usize> for RangeToInclusive<usize> {
  type Saturating = Range<usize>;
  type Bounded = Range<usize>;

  fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
    (Bound::Unbounded, Bound::Included(self.end))
  }

  fn contains(&self, item: &usize) -> bool {
    RangeBounds::contains(self, item)
  }

  fn is_inverted(&self) -> bool {
    false
  }

  fn saturating_iter(&self) -> Self::Saturating {
    0..self.end
  }

  fn bounded_iter(&self) -> Self::Bounded {
    0..self.end
  }
}

impl NomRange<usize> for RangeFull {
  type Saturating = SaturatingIterator;
  type Bounded = Range<usize>;

  fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
    (Bound::Unbounded, Bound::Unbounded)
  }

  fn contains(&self, item: &usize) -> bool {
    RangeBounds::contains(self, item)
  }

  fn is_inverted(&self) -> bool {
    false
  }

  fn saturating_iter(&self) -> Self::Saturating {
    SaturatingIterator { count: 0 }
  }

  fn bounded_iter(&self) -> Self::Bounded {
    0..usize::MAX
  }
}

impl NomRange<usize> for usize {
  type Saturating = Range<usize>;
  type Bounded = Range<usize>;

  fn bounds(&self) -> (Bound<usize>, Bound<usize>) {
    (Bound::Included(*self), Bound::Included(*self))
  }

  fn contains(&self, item: &usize) -> bool {
    self == item
  }

  fn is_inverted(&self) -> bool {
    false
  }

  fn saturating_iter(&self) -> Self::Saturating {
    0..*self
  }

  fn bounded_iter(&self) -> Self::Bounded {
    0..*self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_offset_u8() {
    let s = b"abcd123";
    let a = &s[..];
    let b = &a[2..];
    let c = &a[..4];
    let d = &a[3..5];
    assert_eq!(a.offset(b), 2);
    assert_eq!(a.offset(c), 0);
    assert_eq!(a.offset(d), 3);
  }

  #[test]
  fn test_offset_str() {
    let a = "abcřèÂßÇd123";
    let b = &a[7..];
    let c = &a[..5];
    let d = &a[5..9];
    assert_eq!(a.offset(b), 7);
    assert_eq!(a.offset(c), 0);
    assert_eq!(a.offset(d), 5);
  }

  #[test]
  fn test_slice_index() {
    let a = "abcřèÂßÇd123";
    assert_eq!(a.slice_index(0), Ok(0));
    assert_eq!(a.slice_index(2), Ok(2));
  }

  #[test]
  fn test_slice_index_utf8() {
    let a = "a¡€💢€¡a";

    for (c, len) in a.chars().zip([1, 2, 3, 4, 3, 2, 1]) {
      assert_eq!(c.len(), len);
    }

    assert_eq!(a.slice_index(0), Ok(0));
    assert_eq!(a.slice_index(1), Ok(1));
    assert_eq!(a.slice_index(2), Ok(3));
    assert_eq!(a.slice_index(3), Ok(6));
    assert_eq!(a.slice_index(4), Ok(10));
    assert_eq!(a.slice_index(5), Ok(13));
    assert_eq!(a.slice_index(6), Ok(15));
    assert_eq!(a.slice_index(7), Ok(16));

    assert!(a.slice_index(8).is_err());
  }
}
