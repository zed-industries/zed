use std::borrow::Borrow;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io;
use std::result;

use super::{TrieSetSlice, CHUNK_SIZE};

// This implementation was pretty much cribbed from raphlinus' contribution
// to the standard library: https://github.com/rust-lang/rust/pull/33098/files
//
// The fundamental principle guiding this implementation is to take advantage
// of the fact that similar Unicode codepoints are often grouped together, and
// that most boolean Unicode properties are quite sparse over the entire space
// of Unicode codepoints.
//
// To do this, we represent sets using something like a trie (which gives us
// prefix compression). The "final" states of the trie are embedded in leaves
// or "chunks," where each chunk is a 64 bit integer. Each bit position of the
// integer corresponds to whether a particular codepoint is in the set or not.
// These chunks are not just a compact representation of the final states of
// the trie, but are also a form of suffix compression. In particular, if
// multiple ranges of 64 contiguous codepoints map have the same set membership
// ordering, then they all map to the exact same chunk in the trie.
//
// We organize this structure by partitioning the space of Unicode codepoints
// into three disjoint sets. The first set corresponds to codepoints
// [0, 0x800), the second [0x800, 0x1000) and the third [0x10000, 0x110000).
// These partitions conveniently correspond to the space of 1 or 2 byte UTF-8
// encoded codepoints, 3 byte UTF-8 encoded codepoints and 4 byte UTF-8 encoded
// codepoints, respectively.
//
// Each partition has its own tree with its own root. The first partition is
// the simplest, since the tree is completely flat. In particular, to determine
// the set membership of a Unicode codepoint (that is less than `0x800`), we
// do the following (where `cp` is the codepoint we're testing):
//
//     let chunk_address = cp >> 6;
//     let chunk_bit = cp & 0b111111;
//     let chunk = tree1[cp >> 6];
//     let is_member = 1 == ((chunk >> chunk_bit) & 1);
//
// We do something similar for the second partition:
//
//     // we subtract 0x20 since (0x800 >> 6) == 0x20.
//     let child_address = (cp >> 6) - 0x20;
//     let chunk_address = tree2_level1[child_address];
//     let chunk_bit = cp & 0b111111;
//     let chunk = tree2_level2[chunk_address];
//     let is_member = 1 == ((chunk >> chunk_bit) & 1);
//
// And so on for the third partition.
//
// Note that as a special case, if the second or third partitions are empty,
// then the trie will store empty slices for those levels. The `contains`
// check knows to return `false` in those cases.

const CHUNKS: usize = 0x110000 / CHUNK_SIZE;

/// A type alias that maps to `std::result::Result<T, ucd_trie::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur during construction of a trie.
#[derive(Clone, Debug)]
pub enum Error {
    /// This error is returned when an invalid codepoint is given to
    /// `TrieSetOwned::from_codepoints`. An invalid codepoint is a `u32` that
    /// is greater than `0x10FFFF`.
    InvalidCodepoint(u32),
    /// This error is returned when a set of Unicode codepoints could not be
    /// sufficiently compressed into the trie provided by this crate. There is
    /// no work-around for this error at this time.
    GaveUp,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::InvalidCodepoint(cp) => write!(
                f,
                "could not construct trie set containing an \
                 invalid Unicode codepoint: 0x{:X}",
                cp
            ),
            Error::GaveUp => {
                write!(f, "could not compress codepoint set into a trie")
            }
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

/// An owned trie set.
#[derive(Clone)]
pub struct TrieSetOwned {
    tree1_level1: Vec<u64>,
    tree2_level1: Vec<u8>,
    tree2_level2: Vec<u64>,
    tree3_level1: Vec<u8>,
    tree3_level2: Vec<u8>,
    tree3_level3: Vec<u64>,
}

impl fmt::Debug for TrieSetOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrieSetOwned(...)")
    }
}

impl TrieSetOwned {
    fn new(all: &[bool]) -> Result<TrieSetOwned> {
        let mut bitvectors = Vec::with_capacity(CHUNKS);
        for i in 0..CHUNKS {
            let mut bitvector = 0u64;
            for j in 0..CHUNK_SIZE {
                if all[i * CHUNK_SIZE + j] {
                    bitvector |= 1 << j;
                }
            }
            bitvectors.push(bitvector);
        }

        let tree1_level1 =
            bitvectors.iter().cloned().take(0x800 / CHUNK_SIZE).collect();

        let (mut tree2_level1, mut tree2_level2) = compress_postfix_leaves(
            &bitvectors[0x800 / CHUNK_SIZE..0x10000 / CHUNK_SIZE],
        )?;
        if tree2_level2.len() == 1 && tree2_level2[0] == 0 {
            tree2_level1.clear();
            tree2_level2.clear();
        }

        let (mid, mut tree3_level3) = compress_postfix_leaves(
            &bitvectors[0x10000 / CHUNK_SIZE..0x110000 / CHUNK_SIZE],
        )?;
        let (mut tree3_level1, mut tree3_level2) =
            compress_postfix_mid(&mid, 64)?;
        if tree3_level3.len() == 1 && tree3_level3[0] == 0 {
            tree3_level1.clear();
            tree3_level2.clear();
            tree3_level3.clear();
        }

        Ok(TrieSetOwned {
            tree1_level1,
            tree2_level1,
            tree2_level2,
            tree3_level1,
            tree3_level2,
            tree3_level3,
        })
    }

    /// Create a new trie set from a set of Unicode scalar values.
    ///
    /// This returns an error if a set could not be sufficiently compressed to
    /// fit into a trie.
    pub fn from_scalars<I, C>(scalars: I) -> Result<TrieSetOwned>
    where
        I: IntoIterator<Item = C>,
        C: Borrow<char>,
    {
        let mut all = vec![false; 0x110000];
        for s in scalars {
            all[*s.borrow() as usize] = true;
        }
        TrieSetOwned::new(&all)
    }

    /// Create a new trie set from a set of Unicode scalar values.
    ///
    /// This returns an error if a set could not be sufficiently compressed to
    /// fit into a trie. This also returns an error if any of the given
    /// codepoints are greater than `0x10FFFF`.
    pub fn from_codepoints<I, C>(codepoints: I) -> Result<TrieSetOwned>
    where
        I: IntoIterator<Item = C>,
        C: Borrow<u32>,
    {
        let mut all = vec![false; 0x110000];
        for cp in codepoints {
            let cp = *cp.borrow();
            if cp > 0x10FFFF {
                return Err(Error::InvalidCodepoint(cp));
            }
            all[cp as usize] = true;
        }
        TrieSetOwned::new(&all)
    }

    /// Return this set as a slice.
    #[inline(always)]
    pub fn as_slice(&self) -> TrieSetSlice<'_> {
        TrieSetSlice {
            tree1_level1: &self.tree1_level1,
            tree2_level1: &self.tree2_level1,
            tree2_level2: &self.tree2_level2,
            tree3_level1: &self.tree3_level1,
            tree3_level2: &self.tree3_level2,
            tree3_level3: &self.tree3_level3,
        }
    }

    /// Returns true if and only if the given Unicode scalar value is in this
    /// set.
    pub fn contains_char(&self, c: char) -> bool {
        self.as_slice().contains_char(c)
    }

    /// Returns true if and only if the given codepoint is in this set.
    ///
    /// If the given value exceeds the codepoint range (i.e., it's greater
    /// than `0x10FFFF`), then this returns false.
    pub fn contains_u32(&self, cp: u32) -> bool {
        self.as_slice().contains_u32(cp)
    }
}

fn compress_postfix_leaves(chunks: &[u64]) -> Result<(Vec<u8>, Vec<u64>)> {
    let mut root = vec![];
    let mut children = vec![];
    let mut bychild = HashMap::new();
    for &chunk in chunks {
        if !bychild.contains_key(&chunk) {
            let start = bychild.len();
            if start > ::std::u8::MAX as usize {
                return Err(Error::GaveUp);
            }
            bychild.insert(chunk, start as u8);
            children.push(chunk);
        }
        root.push(bychild[&chunk]);
    }
    Ok((root, children))
}

fn compress_postfix_mid(
    chunks: &[u8],
    chunk_size: usize,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut root = vec![];
    let mut children = vec![];
    let mut bychild = HashMap::new();
    for i in 0..(chunks.len() / chunk_size) {
        let chunk = &chunks[i * chunk_size..(i + 1) * chunk_size];
        if !bychild.contains_key(chunk) {
            let start = bychild.len();
            if start > ::std::u8::MAX as usize {
                return Err(Error::GaveUp);
            }
            bychild.insert(chunk, start as u8);
            children.extend(chunk);
        }
        root.push(bychild[chunk]);
    }
    Ok((root, children))
}

#[cfg(test)]
mod tests {
    use super::TrieSetOwned;
    use crate::general_category;
    use std::collections::HashSet;

    fn mk(scalars: &[char]) -> TrieSetOwned {
        TrieSetOwned::from_scalars(scalars).unwrap()
    }

    fn ranges_to_set(ranges: &[(u32, u32)]) -> Vec<u32> {
        let mut set = vec![];
        for &(start, end) in ranges {
            for cp in start..end + 1 {
                set.push(cp);
            }
        }
        set
    }

    #[test]
    fn set1() {
        let set = mk(&['a']);
        assert!(set.contains_char('a'));
        assert!(!set.contains_char('b'));
        assert!(!set.contains_char('β'));
        assert!(!set.contains_char('☃'));
        assert!(!set.contains_char('😼'));
    }

    #[test]
    fn set_combined() {
        let set = mk(&['a', 'b', 'β', '☃', '😼']);
        assert!(set.contains_char('a'));
        assert!(set.contains_char('b'));
        assert!(set.contains_char('β'));
        assert!(set.contains_char('☃'));
        assert!(set.contains_char('😼'));

        assert!(!set.contains_char('c'));
        assert!(!set.contains_char('θ'));
        assert!(!set.contains_char('⛇'));
        assert!(!set.contains_char('🐲'));
    }

    // Basic tests on all of the general category sets. We check that
    // membership is correct on every Unicode codepoint... because we can.

    macro_rules! category_test {
        ($name:ident, $ranges:ident) => {
            #[test]
            fn $name() {
                let set = ranges_to_set(general_category::$ranges);
                let hashset: HashSet<u32> = set.iter().cloned().collect();
                let trie = TrieSetOwned::from_codepoints(&set).unwrap();
                for cp in 0..0x110000 {
                    assert!(trie.contains_u32(cp) == hashset.contains(&cp));
                }
                // Test that an invalid codepoint is treated correctly.
                assert!(!trie.contains_u32(0x110000));
                assert!(!hashset.contains(&0x110000));
            }
        };
    }

    category_test!(gencat_cased_letter, CASED_LETTER);
    category_test!(gencat_close_punctuation, CLOSE_PUNCTUATION);
    category_test!(gencat_connector_punctuation, CONNECTOR_PUNCTUATION);
    category_test!(gencat_control, CONTROL);
    category_test!(gencat_currency_symbol, CURRENCY_SYMBOL);
    category_test!(gencat_dash_punctuation, DASH_PUNCTUATION);
    category_test!(gencat_decimal_number, DECIMAL_NUMBER);
    category_test!(gencat_enclosing_mark, ENCLOSING_MARK);
    category_test!(gencat_final_punctuation, FINAL_PUNCTUATION);
    category_test!(gencat_format, FORMAT);
    category_test!(gencat_initial_punctuation, INITIAL_PUNCTUATION);
    category_test!(gencat_letter, LETTER);
    category_test!(gencat_letter_number, LETTER_NUMBER);
    category_test!(gencat_line_separator, LINE_SEPARATOR);
    category_test!(gencat_lowercase_letter, LOWERCASE_LETTER);
    category_test!(gencat_math_symbol, MATH_SYMBOL);
    category_test!(gencat_mark, MARK);
    category_test!(gencat_modifier_letter, MODIFIER_LETTER);
    category_test!(gencat_modifier_symbol, MODIFIER_SYMBOL);
    category_test!(gencat_nonspacing_mark, NONSPACING_MARK);
    category_test!(gencat_number, NUMBER);
    category_test!(gencat_open_punctuation, OPEN_PUNCTUATION);
    category_test!(gencat_other, OTHER);
    category_test!(gencat_other_letter, OTHER_LETTER);
    category_test!(gencat_other_number, OTHER_NUMBER);
    category_test!(gencat_other_punctuation, OTHER_PUNCTUATION);
    category_test!(gencat_other_symbol, OTHER_SYMBOL);
    category_test!(gencat_paragraph_separator, PARAGRAPH_SEPARATOR);
    category_test!(gencat_private_use, PRIVATE_USE);
    category_test!(gencat_punctuation, PUNCTUATION);
    category_test!(gencat_separator, SEPARATOR);
    category_test!(gencat_space_separator, SPACE_SEPARATOR);
    category_test!(gencat_spacing_mark, SPACING_MARK);
    category_test!(gencat_surrogate, SURROGATE);
    category_test!(gencat_symbol, SYMBOL);
    category_test!(gencat_titlecase_letter, TITLECASE_LETTER);
    category_test!(gencat_unassigned, UNASSIGNED);
    category_test!(gencat_uppercase_letter, UPPERCASE_LETTER);
}
