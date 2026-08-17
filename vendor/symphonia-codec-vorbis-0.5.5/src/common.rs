// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// As defined in section 9.2.1 of the Vorbis I specification.
///
/// The `ilog` function returns the position number (1 through n) of the highest set bit in the two’s
/// complement integer value `x`.
#[inline(always)]
pub fn ilog(x: u32) -> u32 {
    32 - x.leading_zeros()
}

pub struct BitSetIterator<'a> {
    bits: &'a [u32],
    pos: usize,
    count: usize,
}

impl Iterator for BitSetIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }

        for bits in &self.bits[self.pos >> 5..] {
            let bits_read = self.pos & 0x1f;

            let offset = (bits >> bits_read).trailing_zeros() as usize;

            if offset < 32 - bits_read {
                self.pos += offset + 1;
                self.count -= 1;
                return Some(self.pos - 1);
            }
            else {
                self.pos += 32 - bits_read;
            }
        }

        None
    }
}

// TODO: When const generics allow division, switch to that.
macro_rules! decl_bitset {
    ($name:ident, $size:expr) => {
        #[derive(Default)]
        pub struct $name {
            bits: [u32; $size >> 5],
            bit_count: usize,
        }

        impl $name {
            #[inline(always)]
            pub fn set(&mut self, idx: usize) {
                if !self.is_set(idx) {
                    self.bits[idx >> 5] |= 1 << (idx & 0x1f);
                    self.bit_count += 1;
                }
            }

            // #[inline(always)]
            // pub fn unset(&mut self, idx: usize) {
            //     if self.is_set(idx) {
            //         self.bits[idx >> 5] &= !(1 << (idx & 0x1f));
            //         self.bit_count -= 1;
            //     }
            // }

            #[inline(always)]
            pub fn is_set(&self, idx: usize) -> bool {
                self.bits[idx >> 5] & (1 << (idx & 0x1f)) != 0
            }

            #[inline(always)]
            pub fn count(&self) -> usize {
                self.bit_count
            }

            #[inline(always)]
            pub fn iter(&self) -> BitSetIterator<'_> {
                BitSetIterator { bits: &self.bits, pos: 0, count: self.bit_count }
            }
        }
    };
}

decl_bitset!(BitSet256, 256);

#[cfg(test)]
mod tests {
    use super::BitSet256;

    #[test]
    fn verify_bitset_count() {
        let mut bitset: BitSet256 = Default::default();

        bitset.set(1);
        bitset.set(2);
        bitset.set(56);
        bitset.set(64);
        bitset.set(127);
        bitset.set(128);
        bitset.set(250);
        bitset.set(251);
        bitset.set(252);
        bitset.set(253);
        bitset.set(254);
        bitset.set(255);

        assert_eq!(bitset.count(), 12);
    }

    #[test]
    fn verify_bitset_iter() {
        let mut bitset: BitSet256 = Default::default();

        assert_eq!(bitset.count(), 0);

        for _ in bitset.iter() {
            panic!("Should be empty!");
        }

        bitset.set(1);
        bitset.set(2);
        bitset.set(56);
        bitset.set(64);
        bitset.set(127);
        bitset.set(128);
        bitset.set(250);
        bitset.set(251);
        bitset.set(252);
        bitset.set(253);
        bitset.set(254);
        bitset.set(255);

        let mut iter = bitset.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(56));
        assert_eq!(iter.next(), Some(64));
        assert_eq!(iter.next(), Some(127));
        assert_eq!(iter.next(), Some(128));
        assert_eq!(iter.next(), Some(250));
        assert_eq!(iter.next(), Some(251));
        assert_eq!(iter.next(), Some(252));
        assert_eq!(iter.next(), Some(253));
        assert_eq!(iter.next(), Some(254));
        assert_eq!(iter.next(), Some(255));
        assert_eq!(iter.next(), None);
    }
}
