// Copyright 2019 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Lookups of unicode properties using minimal perfect hashing.

use crate::perfect_hash::mph_lookup;
use crate::tables::*;

/// Look up the canonical combining class for a codepoint.
///
/// The value returned is as defined in the Unicode Character Database.
pub fn canonical_combining_class(c: char) -> u8 {
    mph_lookup(
        c.into(),
        CANONICAL_COMBINING_CLASS_SALT,
        CANONICAL_COMBINING_CLASS_KV,
        u8_lookup_fk,
        u8_lookup_fv,
        0,
    )
}

pub(crate) fn composition_table(c1: char, c2: char) -> Option<char> {
    if c1 < '\u{10000}' && c2 < '\u{10000}' {
        mph_lookup(
            (c1 as u32) << 16 | (c2 as u32),
            COMPOSITION_TABLE_SALT,
            COMPOSITION_TABLE_KV,
            pair_lookup_fk,
            pair_lookup_fv_opt,
            None,
        )
    } else {
        composition_table_astral(c1, c2)
    }
}

pub(crate) fn canonical_fully_decomposed(c: char) -> Option<&'static [char]> {
    mph_lookup(
        c.into(),
        CANONICAL_DECOMPOSED_SALT,
        CANONICAL_DECOMPOSED_KV,
        pair_lookup_fk,
        pair_lookup_fv_opt,
        None,
    )
    .map(|(start, len)| &CANONICAL_DECOMPOSED_CHARS[start as usize..][..len as usize])
}

pub(crate) fn compatibility_fully_decomposed(c: char) -> Option<&'static [char]> {
    mph_lookup(
        c.into(),
        COMPATIBILITY_DECOMPOSED_SALT,
        COMPATIBILITY_DECOMPOSED_KV,
        pair_lookup_fk,
        pair_lookup_fv_opt,
        None,
    )
    .map(|(start, len)| &COMPATIBILITY_DECOMPOSED_CHARS[start as usize..][..len as usize])
}

pub(crate) fn cjk_compat_variants_fully_decomposed(c: char) -> Option<&'static [char]> {
    mph_lookup(
        c.into(),
        CJK_COMPAT_VARIANTS_DECOMPOSED_SALT,
        CJK_COMPAT_VARIANTS_DECOMPOSED_KV,
        pair_lookup_fk,
        pair_lookup_fv_opt,
        None,
    )
    .map(|(start, len)| &CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS[start as usize..][..len as usize])
}

/// Return whether the given character is a combining mark (`General_Category=Mark`)
pub fn is_combining_mark(c: char) -> bool {
    mph_lookup(
        c.into(),
        COMBINING_MARK_SALT,
        COMBINING_MARK_KV,
        bool_lookup_fk,
        bool_lookup_fv,
        false,
    )
}

pub fn stream_safe_trailing_nonstarters(c: char) -> usize {
    mph_lookup(
        c.into(),
        TRAILING_NONSTARTERS_SALT,
        TRAILING_NONSTARTERS_KV,
        u8_lookup_fk,
        u8_lookup_fv,
        0,
    ) as usize
}

/// Extract the key in a 24 bit key and 8 bit value packed in a u32.
#[inline]
fn u8_lookup_fk(kv: u32) -> u32 {
    kv >> 8
}

/// Extract the value in a 24 bit key and 8 bit value packed in a u32.
#[inline]
fn u8_lookup_fv(kv: u32) -> u8 {
    (kv & 0xff) as u8
}

/// Extract the key for a boolean lookup.
#[inline]
fn bool_lookup_fk(kv: u32) -> u32 {
    kv
}

/// Extract the value for a boolean lookup.
#[inline]
fn bool_lookup_fv(_kv: u32) -> bool {
    true
}

/// Extract the key in a pair.
#[inline]
fn pair_lookup_fk<T>(kv: (u32, T)) -> u32 {
    kv.0
}

/// Extract the value in a pair, returning an option.
#[inline]
fn pair_lookup_fv_opt<T>(kv: (u32, T)) -> Option<T> {
    Some(kv.1)
}
