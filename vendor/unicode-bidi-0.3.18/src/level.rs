// Copyright 2017 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bidi Embedding Level
//!
//! See [`Level`](struct.Level.html) for more details.
//!
//! <http://www.unicode.org/reports/tr9/#BD2>

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::slice;

use super::char_data::BidiClass;

/// Embedding Level
///
/// Embedding Levels are numbers between 0 and 126 (inclusive), where even values denote a
/// left-to-right (LTR) direction and odd values a right-to-left (RTL) direction.
///
/// This struct maintains a *valid* status for level numbers, meaning that creating a new level, or
/// mutating an existing level, with the value smaller than `0` (before conversion to `u8`) or
/// larger than 125 results in an `Error`.
///
/// <http://www.unicode.org/reports/tr9/#BD2>
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Level(u8);

pub const LTR_LEVEL: Level = Level(0);
pub const RTL_LEVEL: Level = Level(1);

const MAX_DEPTH: u8 = 125;
/// During explicit level resolution, embedding level can go as high as `max_depth`.
pub const MAX_EXPLICIT_DEPTH: u8 = MAX_DEPTH;
/// During implicit level resolution, embedding level can go as high as `max_depth + 1`.
pub const MAX_IMPLICIT_DEPTH: u8 = MAX_DEPTH + 1;

/// Errors that can occur on Level creation or mutation
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Out-of-range (invalid) embedding level number.
    OutOfRangeNumber,
}

impl Level {
    /// New LTR level with smallest number value (0).
    #[inline]
    pub fn ltr() -> Level {
        LTR_LEVEL
    }

    /// New RTL level with smallest number value (1).
    #[inline]
    pub fn rtl() -> Level {
        RTL_LEVEL
    }

    /// Maximum depth of the directional status stack during implicit resolutions.
    pub fn max_implicit_depth() -> u8 {
        MAX_IMPLICIT_DEPTH
    }

    /// Maximum depth of the directional status stack during explicit resolutions.
    pub fn max_explicit_depth() -> u8 {
        MAX_EXPLICIT_DEPTH
    }

    // == Inquiries ==

    /// Create new level, fail if number is larger than `max_depth + 1`.
    #[inline]
    pub fn new(number: u8) -> Result<Level, Error> {
        if number <= MAX_IMPLICIT_DEPTH {
            Ok(Level(number))
        } else {
            Err(Error::OutOfRangeNumber)
        }
    }

    /// Create new level, fail if number is larger than `max_depth`.
    #[inline]
    pub fn new_explicit(number: u8) -> Result<Level, Error> {
        if number <= MAX_EXPLICIT_DEPTH {
            Ok(Level(number))
        } else {
            Err(Error::OutOfRangeNumber)
        }
    }

    // == Inquiries ==

    /// The level number.
    #[inline]
    pub fn number(&self) -> u8 {
        self.0
    }

    /// If this level is left-to-right.
    #[inline]
    pub fn is_ltr(&self) -> bool {
        self.0 % 2 == 0
    }

    /// If this level is right-to-left.
    #[inline]
    pub fn is_rtl(&self) -> bool {
        self.0 % 2 == 1
    }

    // == Mutators ==

    /// Raise level by `amount`, fail if number is larger than `max_depth + 1`.
    #[inline]
    pub fn raise(&mut self, amount: u8) -> Result<(), Error> {
        match self.0.checked_add(amount) {
            Some(number) => {
                if number <= MAX_IMPLICIT_DEPTH {
                    self.0 = number;
                    Ok(())
                } else {
                    Err(Error::OutOfRangeNumber)
                }
            }
            None => Err(Error::OutOfRangeNumber),
        }
    }

    /// Raise level by `amount`, fail if number is larger than `max_depth`.
    #[inline]
    pub fn raise_explicit(&mut self, amount: u8) -> Result<(), Error> {
        match self.0.checked_add(amount) {
            Some(number) => {
                if number <= MAX_EXPLICIT_DEPTH {
                    self.0 = number;
                    Ok(())
                } else {
                    Err(Error::OutOfRangeNumber)
                }
            }
            None => Err(Error::OutOfRangeNumber),
        }
    }

    /// Lower level by `amount`, fail if number goes below zero.
    #[inline]
    pub fn lower(&mut self, amount: u8) -> Result<(), Error> {
        match self.0.checked_sub(amount) {
            Some(number) => {
                self.0 = number;
                Ok(())
            }
            None => Err(Error::OutOfRangeNumber),
        }
    }

    // == Helpers ==

    /// The next LTR (even) level greater than this, or fail if number is larger than `max_depth`.
    #[inline]
    pub fn new_explicit_next_ltr(&self) -> Result<Level, Error> {
        Level::new_explicit((self.0 + 2) & !1)
    }

    /// The next RTL (odd) level greater than this, or fail if number is larger than `max_depth`.
    #[inline]
    pub fn new_explicit_next_rtl(&self) -> Result<Level, Error> {
        Level::new_explicit((self.0 + 1) | 1)
    }

    /// The lowest RTL (odd) level greater than or equal to this, or fail if number is larger than
    /// `max_depth + 1`.
    #[inline]
    pub fn new_lowest_ge_rtl(&self) -> Result<Level, Error> {
        Level::new(self.0 | 1)
    }

    /// Generate a character type based on a level (as specified in steps X10 and N2).
    #[inline]
    pub fn bidi_class(&self) -> BidiClass {
        if self.is_rtl() {
            BidiClass::R
        } else {
            BidiClass::L
        }
    }

    pub fn vec(v: &[u8]) -> Vec<Level> {
        v.iter().map(|&x| x.into()).collect()
    }

    /// Converts a byte slice to a slice of Levels
    ///
    /// Does _not_ check if each level is within bounds (`<=` [`MAX_IMPLICIT_DEPTH`]),
    /// which is not a requirement for safety but is a requirement for correctness of the algorithm.
    pub fn from_slice_unchecked(v: &[u8]) -> &[Level] {
        debug_assert_eq!(core::mem::size_of::<u8>(), core::mem::size_of::<Level>());
        unsafe {
            // Safety: The two arrays are the same size and layout-compatible since
            // Level is `repr(transparent)` over `u8`
            slice::from_raw_parts(v as *const [u8] as *const u8 as *const Level, v.len())
        }
    }
}

/// If levels has any RTL (odd) level
///
/// This information is usually used to skip re-ordering of text when no RTL level is present
#[inline]
pub fn has_rtl(levels: &[Level]) -> bool {
    levels.iter().any(|&lvl| lvl.is_rtl())
}

impl From<Level> for u8 {
    /// Convert to the level number
    #[inline]
    fn from(val: Level) -> Self {
        val.number()
    }
}

impl From<u8> for Level {
    /// Create level by number
    #[inline]
    fn from(number: u8) -> Level {
        Level::new(number).expect("Level number error")
    }
}

/// Used for matching levels in conformance tests
impl<'a> PartialEq<&'a str> for Level {
    #[inline]
    fn eq(&self, s: &&'a str) -> bool {
        *s == "x" || *s == self.0.to_string()
    }
}

/// Used for matching levels in conformance tests
impl PartialEq<String> for Level {
    #[inline]
    fn eq(&self, s: &String) -> bool {
        self == &s.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Level::new(0), Ok(Level(0)));
        assert_eq!(Level::new(1), Ok(Level(1)));
        assert_eq!(Level::new(10), Ok(Level(10)));
        assert_eq!(Level::new(125), Ok(Level(125)));
        assert_eq!(Level::new(126), Ok(Level(126)));
        assert_eq!(Level::new(127), Err(Error::OutOfRangeNumber));
        assert_eq!(Level::new(255), Err(Error::OutOfRangeNumber));
    }

    #[test]
    fn test_new_explicit() {
        assert_eq!(Level::new_explicit(0), Ok(Level(0)));
        assert_eq!(Level::new_explicit(1), Ok(Level(1)));
        assert_eq!(Level::new_explicit(10), Ok(Level(10)));
        assert_eq!(Level::new_explicit(125), Ok(Level(125)));
        assert_eq!(Level::new_explicit(126), Err(Error::OutOfRangeNumber));
        assert_eq!(Level::new_explicit(255), Err(Error::OutOfRangeNumber));
    }

    #[test]
    fn test_is_ltr() {
        assert_eq!(Level(0).is_ltr(), true);
        assert_eq!(Level(1).is_ltr(), false);
        assert_eq!(Level(10).is_ltr(), true);
        assert_eq!(Level(11).is_ltr(), false);
        assert_eq!(Level(124).is_ltr(), true);
        assert_eq!(Level(125).is_ltr(), false);
    }

    #[test]
    fn test_is_rtl() {
        assert_eq!(Level(0).is_rtl(), false);
        assert_eq!(Level(1).is_rtl(), true);
        assert_eq!(Level(10).is_rtl(), false);
        assert_eq!(Level(11).is_rtl(), true);
        assert_eq!(Level(124).is_rtl(), false);
        assert_eq!(Level(125).is_rtl(), true);
    }

    #[test]
    fn test_raise() {
        let mut level = Level::ltr();
        assert_eq!(level.number(), 0);
        assert!(level.raise(100).is_ok());
        assert_eq!(level.number(), 100);
        assert!(level.raise(26).is_ok());
        assert_eq!(level.number(), 126);
        assert!(level.raise(1).is_err()); // invalid!
        assert!(level.raise(250).is_err()); // overflow!
        assert_eq!(level.number(), 126);
    }

    #[test]
    fn test_raise_explicit() {
        let mut level = Level::ltr();
        assert_eq!(level.number(), 0);
        assert!(level.raise_explicit(100).is_ok());
        assert_eq!(level.number(), 100);
        assert!(level.raise_explicit(25).is_ok());
        assert_eq!(level.number(), 125);
        assert!(level.raise_explicit(1).is_err()); // invalid!
        assert!(level.raise_explicit(250).is_err()); // overflow!
        assert_eq!(level.number(), 125);
    }

    #[test]
    fn test_lower() {
        let mut level = Level::rtl();
        assert_eq!(level.number(), 1);
        assert!(level.lower(1).is_ok());
        assert_eq!(level.number(), 0);
        assert!(level.lower(1).is_err()); // underflow!
        assert!(level.lower(250).is_err()); // underflow!
        assert_eq!(level.number(), 0);
    }

    #[test]
    fn test_has_rtl() {
        assert_eq!(has_rtl(&Level::vec(&[0, 0, 0])), false);
        assert_eq!(has_rtl(&Level::vec(&[0, 1, 0])), true);
        assert_eq!(has_rtl(&Level::vec(&[0, 2, 0])), false);
        assert_eq!(has_rtl(&Level::vec(&[0, 125, 0])), true);
        assert_eq!(has_rtl(&Level::vec(&[0, 126, 0])), false);
    }

    #[test]
    fn test_into() {
        let level = Level::rtl();
        let number: u8 = level.into();
        assert_eq!(1u8, number);
    }

    #[test]
    fn test_vec() {
        assert_eq!(
            Level::vec(&[0, 1, 125]),
            vec![Level(0), Level(1), Level(125)]
        );
    }

    #[test]
    fn test_str_eq() {
        assert_eq!(Level::vec(&[0, 1, 4, 125]), vec!["0", "1", "x", "125"]);
        assert_ne!(Level::vec(&[0, 1, 4, 125]), vec!["0", "1", "5", "125"]);
    }

    #[test]
    fn test_string_eq() {
        assert_eq!(
            Level::vec(&[0, 1, 4, 125]),
            vec!["0".to_string(), "1".to_string(), "x".to_string(), "125".to_string()]
        );
    }
}

#[cfg(all(feature = "serde", test))]
mod serde_tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_statics() {
        assert_tokens(
            &Level::ltr(),
            &[Token::NewtypeStruct { name: "Level" }, Token::U8(0)],
        );
        assert_tokens(
            &Level::rtl(),
            &[Token::NewtypeStruct { name: "Level" }, Token::U8(1)],
        );
    }

    #[test]
    fn test_new() {
        let level = Level::new(42).unwrap();
        assert_tokens(
            &level,
            &[Token::NewtypeStruct { name: "Level" }, Token::U8(42)],
        );
    }
}
