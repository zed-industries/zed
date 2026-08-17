// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

pub(crate) fn to_escaped_string<T: fmt::Debug>(x: &T) -> String {
    // FIXME: don't allocate twice
    let string = format!("{x:?}");
    string.chars().flat_map(|c| c.escape_default()).collect()
}

/// If `c` is an ASCII letter, return the corresponding lowercase
/// letter, otherwise None.
pub(crate) fn lower_ascii_letter(c: char) -> Option<char> {
    match c {
        'a'..='z' => Some(c),
        'A'..='Z' => Some((c as u8 - b'A' + b'a') as char),
        _ => None,
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::lower_ascii_letter;

    #[test]
    fn lower_letter_a_is_a() {
        assert_eq!(lower_ascii_letter('a'), Some('a'));
    }

    #[test]
    fn lower_letter_A_is_a() {
        assert_eq!(lower_ascii_letter('A'), Some('a'));
    }

    #[test]
    fn lower_letter_symbol_is_None() {
        assert_eq!(lower_ascii_letter('!'), None);
    }

    #[test]
    fn lower_letter_nonascii_is_None() {
        assert_eq!(lower_ascii_letter('\u{a66e}'), None);
    }
}
