// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::char::is_combining_mark;
use super::UnicodeNormalization;
use core::char;

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

#[test]
fn test_nfd() {
    macro_rules! t {
        ($input: expr, $expected: expr) => {
            assert_eq!($input.nfd().to_string(), $expected);
            // A dummy iterator that is not std::str::Chars directly;
            // note that `id_func` is used to ensure `Clone` implementation
            assert_eq!(
                $input.chars().map(|c| c).nfd().collect::<String>(),
                $expected
            );
        };
    }
    t!("abc", "abc");
    t!("\u{1e0b}\u{1c4}", "d\u{307}\u{1c4}");
    t!("\u{2026}", "\u{2026}");
    t!("\u{2126}", "\u{3a9}");
    t!("\u{1e0b}\u{323}", "d\u{323}\u{307}");
    t!("\u{1e0d}\u{307}", "d\u{323}\u{307}");
    t!("a\u{301}", "a\u{301}");
    t!("\u{301}a", "\u{301}a");
    t!("\u{d4db}", "\u{1111}\u{1171}\u{11b6}");
    t!("\u{ac1c}", "\u{1100}\u{1162}");
}

#[test]
fn test_nfkd() {
    macro_rules! t {
        ($input: expr, $expected: expr) => {
            assert_eq!($input.nfkd().to_string(), $expected);
        };
    }
    t!("abc", "abc");
    t!("\u{1e0b}\u{1c4}", "d\u{307}DZ\u{30c}");
    t!("\u{2026}", "...");
    t!("\u{2126}", "\u{3a9}");
    t!("\u{1e0b}\u{323}", "d\u{323}\u{307}");
    t!("\u{1e0d}\u{307}", "d\u{323}\u{307}");
    t!("a\u{301}", "a\u{301}");
    t!("\u{301}a", "\u{301}a");
    t!("\u{d4db}", "\u{1111}\u{1171}\u{11b6}");
    t!("\u{ac1c}", "\u{1100}\u{1162}");
}

#[test]
fn test_nfc() {
    macro_rules! t {
        ($input: expr, $expected: expr) => {
            assert_eq!($input.nfc().to_string(), $expected);
        };
    }
    t!("abc", "abc");
    t!("\u{1e0b}\u{1c4}", "\u{1e0b}\u{1c4}");
    t!("\u{2026}", "\u{2026}");
    t!("\u{2126}", "\u{3a9}");
    t!("\u{1e0b}\u{323}", "\u{1e0d}\u{307}");
    t!("\u{1e0d}\u{307}", "\u{1e0d}\u{307}");
    t!("a\u{301}", "\u{e1}");
    t!("\u{301}a", "\u{301}a");
    t!("\u{d4db}", "\u{d4db}");
    t!("\u{ac1c}", "\u{ac1c}");
    t!(
        "a\u{300}\u{305}\u{315}\u{5ae}b",
        "\u{e0}\u{5ae}\u{305}\u{315}b"
    );
}

#[test]
fn test_nfkc() {
    macro_rules! t {
        ($input: expr, $expected: expr) => {
            assert_eq!($input.nfkc().to_string(), $expected);
        };
    }
    t!("abc", "abc");
    t!("\u{1e0b}\u{1c4}", "\u{1e0b}D\u{17d}");
    t!("\u{2026}", "...");
    t!("\u{2126}", "\u{3a9}");
    t!("\u{1e0b}\u{323}", "\u{1e0d}\u{307}");
    t!("\u{1e0d}\u{307}", "\u{1e0d}\u{307}");
    t!("a\u{301}", "\u{e1}");
    t!("\u{301}a", "\u{301}a");
    t!("\u{d4db}", "\u{d4db}");
    t!("\u{ac1c}", "\u{ac1c}");
    t!(
        "a\u{300}\u{305}\u{315}\u{5ae}b",
        "\u{e0}\u{5ae}\u{305}\u{315}b"
    );
}

#[test]
fn test_normalize_char() {
    assert_eq!('\u{2126}'.nfd().to_string(), "\u{3a9}")
}

#[test]
fn test_is_combining_mark_ascii() {
    for cp in 0..0x7f {
        assert!(!is_combining_mark(char::from_u32(cp).unwrap()));
    }
}

#[test]
fn test_is_combining_mark_misc() {
    // https://github.com/unicode-rs/unicode-normalization/issues/16
    // U+11C3A BHAIKSUKI VOWEL SIGN O
    // Category: Mark, Nonspacing [Mn]
    assert!(is_combining_mark('\u{11C3A}'));

    // U+11C3F BHAIKSUKI SIGN VIRAMA
    // Category: Mark, Nonspacing [Mn]
    assert!(is_combining_mark('\u{11C3F}'));
}
