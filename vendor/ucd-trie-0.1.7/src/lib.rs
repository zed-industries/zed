/*!
The ucd-trie crate provides a compressed trie set specifically tailored for
Unicode codepoints. The principle use case for such a trie is to represent
properties defined by Unicode that correspond to sets of Unicode codepoints.
(These properties are formally called boolean properties or "single valued"
properties. See
[UTR#23 S3.3](https://www.unicode.org/reports/tr23/#PropertyTypeDefinitions)
for more details.)

This crate has two principle types: `TrieSetOwned` and `TrieSetSlice`,
corresponding to a similar split as there is between `Vec<T>` and `&[T]`.
`TrieSetOwned` is the only way to construct a trie from a set of Unicode
codepoints.

The intended use of this library is to embed a static instance of
`TrieSetSlice` into your source code, and then use its methods as defined in
this crate to test membership. (The `ucd-generate` tool can likely generate
this code for you.)

Finally, while this crate uses the standard library by default, it provides
`no_std` functionality by disabling the `std` feature. When `no_std` is
enabled, then `TrieSetOwned` is not provided. Instead, only `TrieSetSlice` is
provided, which means `no_std` crates can still embed tries into their code.
*/

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

#[cfg(feature = "std")]
pub use crate::owned::{Error, Result, TrieSetOwned};

#[cfg(test)]
#[allow(dead_code)]
mod general_category;
#[cfg(feature = "std")]
mod owned;

const CHUNK_SIZE: usize = 64;

/// A type alias for `TrieSetSlice<'static>`.
pub type TrieSet = TrieSetSlice<'static>;

/// A borrowed trie set.
#[derive(Clone, Copy)]
pub struct TrieSetSlice<'a> {
    /// first tree, one level
    #[doc(hidden)]
    pub tree1_level1: &'a [u64],
    /// second tree, first level
    #[doc(hidden)]
    pub tree2_level1: &'a [u8],
    /// second tree, second level
    #[doc(hidden)]
    pub tree2_level2: &'a [u64],
    /// third tree, first level
    #[doc(hidden)]
    pub tree3_level1: &'a [u8],
    /// third tree, second level
    #[doc(hidden)]
    pub tree3_level2: &'a [u8],
    /// third tree, third level
    #[doc(hidden)]
    pub tree3_level3: &'a [u64],
}

impl<'a> fmt::Debug for TrieSetSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrieSetSlice(...)")
    }
}

impl<'a> TrieSetSlice<'a> {
    /// Returns true if and only if the given Unicode scalar value is in this
    /// set.
    pub fn contains_char(&self, c: char) -> bool {
        self.contains(c as usize)
    }

    /// Returns true if and only if the given codepoint is in this set.
    ///
    /// If the given value exceeds the codepoint range (i.e., it's greater
    /// than `0x10FFFF`), then this returns false.
    pub fn contains_u32(&self, cp: u32) -> bool {
        if cp > 0x10FFFF {
            return false;
        }
        self.contains(cp as usize)
    }

    #[inline(always)]
    fn contains(&self, cp: usize) -> bool {
        if cp < 0x800 {
            self.chunk_contains(cp, self.tree1_level1[cp >> 6])
        } else if cp < 0x10000 {
            let leaf = match self.tree2_level1.get((cp >> 6) - 0x20) {
                None => return false,
                Some(&leaf) => leaf,
            };
            self.chunk_contains(cp, self.tree2_level2[leaf as usize])
        } else {
            let child = match self.tree3_level1.get((cp >> 12) - 0x10) {
                None => return false,
                Some(&child) => child,
            };
            let i = ((child as usize) * CHUNK_SIZE) + ((cp >> 6) & 0b111111);
            let leaf = self.tree3_level2[i];
            self.chunk_contains(cp, self.tree3_level3[leaf as usize])
        }
    }

    #[inline(always)]
    fn chunk_contains(&self, cp: usize, chunk: u64) -> bool {
        ((chunk >> (cp & 0b111111)) & 1) == 1
    }
}
