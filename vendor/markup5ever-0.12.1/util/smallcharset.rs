// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module contains a single struct [`SmallCharSet`]. See its documentation for details.
//!
//! [`SmallCharSet`]: struct.SmallCharSet.html

/// Represents a set of "small characters", those with Unicode scalar
/// values less than 64.
///
/// This is stored as a bitmap, with 1 bit for each value.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct SmallCharSet {
    pub bits: u64,
}

impl SmallCharSet {
    /// Checks whether a character (u8 value below 64) is stored in the SmallCharSet.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use markup5ever::SmallCharSet;
    /// let set = SmallCharSet {
    ///     bits: 0b00000000_01000000_00000100_00000000_00000000_00000000_00010000_00000000
    /// };
    /// assert!(set.contains(64));
    /// assert!(set.contains(b'6')); // `b'6'` is the same as 64u8
    /// ```
    #[inline]
    fn contains(&self, n: u8) -> bool {
        0 != (self.bits & (1 << (n as usize)))
    }

    /// Count the number of bytes of characters at the beginning of `buf` which are not in the set.
    ///
    /// This functionality is used in [`BufferQueue::pop_except_from`].
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate markup5ever;
    /// # fn main() {
    /// let set = small_char_set!(48 49 50); // '0' '1' '2'
    /// // `test` is 4 chars, 😁 is 4 chars, then we meet a character in the set
    /// let test_str = "test😁01232afd";
    /// assert_eq!(set.nonmember_prefix_len(test_str), 8);
    /// # }
    /// ```
    ///
    /// [`BufferQueue::pop_except_from`]: buffer_queue/struct.BufferQueue.html#method.pop_except_from
    pub fn nonmember_prefix_len(&self, buf: &str) -> u32 {
        let mut n = 0;
        for b in buf.bytes() {
            if b >= 64 || !self.contains(b) {
                n += 1;
            } else {
                break;
            }
        }
        n
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn nonmember_prefix() {
        for &c in ['&', '\0'].iter() {
            for x in 0..48u32 {
                for y in 0..48u32 {
                    let mut s = "x".repeat(x as usize);
                    s.push(c);
                    s.push_str(&"x".repeat(y as usize));
                    let set = small_char_set!('&' '\0');

                    assert_eq!(x, set.nonmember_prefix_len(&s));
                }
            }
        }
    }
}
