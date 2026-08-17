/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Counting and non-counting Bloom filters tuned for use as ancestor filters
//! for selector matching.

use std::fmt::{self, Debug};

// The top 8 bits of the 32-bit hash value are not used by the bloom filter.
// Consumers may rely on this to pack hashes more efficiently.
pub const BLOOM_HASH_MASK: u32 = 0x00ffffff;
const KEY_SIZE: usize = 12;

const ARRAY_SIZE: usize = 1 << KEY_SIZE;
const KEY_MASK: u32 = (1 << KEY_SIZE) - 1;

/// A counting Bloom filter with 8-bit counters.
pub type BloomFilter = CountingBloomFilter<BloomStorageU8>;

/// A counting Bloom filter with parameterized storage to handle
/// counters of different sizes.  For now we assume that having two hash
/// functions is enough, but we may revisit that decision later.
///
/// The filter uses an array with 2**KeySize entries.
///
/// Assuming a well-distributed hash function, a Bloom filter with
/// array size M containing N elements and
/// using k hash function has expected false positive rate exactly
///
/// $  (1 - (1 - 1/M)^{kN})^k  $
///
/// because each array slot has a
///
/// $  (1 - 1/M)^{kN}  $
///
/// chance of being 0, and the expected false positive rate is the
/// probability that all of the k hash functions will hit a nonzero
/// slot.
///
/// For reasonable assumptions (M large, kN large, which should both
/// hold if we're worried about false positives) about M and kN this
/// becomes approximately
///
/// $$  (1 - \exp(-kN/M))^k   $$
///
/// For our special case of k == 2, that's $(1 - \exp(-2N/M))^2$,
/// or in other words
///
/// $$    N/M = -0.5 * \ln(1 - \sqrt(r))   $$
///
/// where r is the false positive rate.  This can be used to compute
/// the desired KeySize for a given load N and false positive rate r.
///
/// If N/M is assumed small, then the false positive rate can
/// further be approximated as 4*N^2/M^2.  So increasing KeySize by
/// 1, which doubles M, reduces the false positive rate by about a
/// factor of 4, and a false positive rate of 1% corresponds to
/// about M/N == 20.
///
/// What this means in practice is that for a few hundred keys using a
/// KeySize of 12 gives false positive rates on the order of 0.25-4%.
///
/// Similarly, using a KeySize of 10 would lead to a 4% false
/// positive rate for N == 100 and to quite bad false positive
/// rates for larger N.
#[derive(Clone, Default)]
pub struct CountingBloomFilter<S>
where
    S: BloomStorage,
{
    storage: S,
}

impl<S> CountingBloomFilter<S>
where
    S: BloomStorage,
{
    /// Creates a new bloom filter.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.storage = Default::default();
    }

    // Slow linear accessor to make sure the bloom filter is zeroed. This should
    // never be used in release builds.
    #[cfg(debug_assertions)]
    pub fn is_zeroed(&self) -> bool {
        self.storage.is_zeroed()
    }

    #[cfg(not(debug_assertions))]
    pub fn is_zeroed(&self) -> bool {
        unreachable!()
    }

    /// Inserts an item with a particular hash into the bloom filter.
    #[inline]
    pub fn insert_hash(&mut self, hash: u32) {
        self.storage.adjust_first_slot(hash, true);
        self.storage.adjust_second_slot(hash, true);
    }

    /// Removes an item with a particular hash from the bloom filter.
    #[inline]
    pub fn remove_hash(&mut self, hash: u32) {
        self.storage.adjust_first_slot(hash, false);
        self.storage.adjust_second_slot(hash, false);
    }

    /// Check whether the filter might contain an item with the given hash.
    /// This can sometimes return true even if the item is not in the filter,
    /// but will never return false for items that are actually in the filter.
    #[inline]
    pub fn might_contain_hash(&self, hash: u32) -> bool {
        !self.storage.first_slot_is_empty(hash) && !self.storage.second_slot_is_empty(hash)
    }
}

impl<S> Debug for CountingBloomFilter<S>
where
    S: BloomStorage,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut slots_used = 0;
        for i in 0..ARRAY_SIZE {
            if !self.storage.slot_is_empty(i) {
                slots_used += 1;
            }
        }
        write!(f, "BloomFilter({}/{})", slots_used, ARRAY_SIZE)
    }
}

pub trait BloomStorage: Clone + Default {
    fn slot_is_empty(&self, index: usize) -> bool;
    fn adjust_slot(&mut self, index: usize, increment: bool);
    fn is_zeroed(&self) -> bool;

    #[inline]
    fn first_slot_is_empty(&self, hash: u32) -> bool {
        self.slot_is_empty(Self::first_slot_index(hash))
    }

    #[inline]
    fn second_slot_is_empty(&self, hash: u32) -> bool {
        self.slot_is_empty(Self::second_slot_index(hash))
    }

    #[inline]
    fn adjust_first_slot(&mut self, hash: u32, increment: bool) {
        self.adjust_slot(Self::first_slot_index(hash), increment)
    }

    #[inline]
    fn adjust_second_slot(&mut self, hash: u32, increment: bool) {
        self.adjust_slot(Self::second_slot_index(hash), increment)
    }

    #[inline]
    fn first_slot_index(hash: u32) -> usize {
        hash1(hash) as usize
    }

    #[inline]
    fn second_slot_index(hash: u32) -> usize {
        hash2(hash) as usize
    }
}

/// Storage class for a CountingBloomFilter that has 8-bit counters.
pub struct BloomStorageU8 {
    counters: [u8; ARRAY_SIZE],
}

impl BloomStorage for BloomStorageU8 {
    #[inline]
    fn adjust_slot(&mut self, index: usize, increment: bool) {
        let slot = &mut self.counters[index];
        if *slot != 0xff {
            // full
            if increment {
                *slot += 1;
            } else {
                *slot -= 1;
            }
        }
    }

    #[inline]
    fn slot_is_empty(&self, index: usize) -> bool {
        self.counters[index] == 0
    }

    #[inline]
    fn is_zeroed(&self) -> bool {
        self.counters.iter().all(|x| *x == 0)
    }
}

impl Default for BloomStorageU8 {
    fn default() -> Self {
        BloomStorageU8 {
            counters: [0; ARRAY_SIZE],
        }
    }
}

impl Clone for BloomStorageU8 {
    fn clone(&self) -> Self {
        BloomStorageU8 {
            counters: self.counters,
        }
    }
}

/// Storage class for a CountingBloomFilter that has 1-bit counters.
pub struct BloomStorageBool {
    counters: [u8; ARRAY_SIZE / 8],
}

impl BloomStorage for BloomStorageBool {
    #[inline]
    fn adjust_slot(&mut self, index: usize, increment: bool) {
        let bit = 1 << (index % 8);
        let byte = &mut self.counters[index / 8];

        // Since we have only one bit for storage, decrementing it
        // should never do anything.  Assert against an accidental
        // decrementing of a bit that was never set.
        assert!(
            increment || (*byte & bit) != 0,
            "should not decrement if slot is already false"
        );

        if increment {
            *byte |= bit;
        }
    }

    #[inline]
    fn slot_is_empty(&self, index: usize) -> bool {
        let bit = 1 << (index % 8);
        (self.counters[index / 8] & bit) == 0
    }

    #[inline]
    fn is_zeroed(&self) -> bool {
        self.counters.iter().all(|x| *x == 0)
    }
}

impl Default for BloomStorageBool {
    fn default() -> Self {
        BloomStorageBool {
            counters: [0; ARRAY_SIZE / 8],
        }
    }
}

impl Clone for BloomStorageBool {
    fn clone(&self) -> Self {
        BloomStorageBool {
            counters: self.counters,
        }
    }
}

#[inline]
fn hash1(hash: u32) -> u32 {
    hash & KEY_MASK
}

#[inline]
fn hash2(hash: u32) -> u32 {
    (hash >> KEY_SIZE) & KEY_MASK
}

#[test]
fn create_and_insert_some_stuff() {
    use rustc_hash::FxHasher;
    use std::hash::{Hash, Hasher};
    use std::mem::transmute;

    fn hash_as_str(i: usize) -> u32 {
        let mut hasher = FxHasher::default();
        let s = i.to_string();
        s.hash(&mut hasher);
        let hash: u64 = hasher.finish();
        (hash >> 32) as u32 ^ (hash as u32)
    }

    let mut bf = BloomFilter::new();

    // Statically assert that ARRAY_SIZE is a multiple of 8, which
    // BloomStorageBool relies on.
    unsafe {
        transmute::<[u8; ARRAY_SIZE % 8], [u8; 0]>([]);
    }

    for i in 0_usize..1000 {
        bf.insert_hash(hash_as_str(i));
    }

    for i in 0_usize..1000 {
        assert!(bf.might_contain_hash(hash_as_str(i)));
    }

    let false_positives = (1001_usize..2000)
        .filter(|i| bf.might_contain_hash(hash_as_str(*i)))
        .count();

    assert!(false_positives < 190, "{} is not < 190", false_positives); // 19%.

    for i in 0_usize..100 {
        bf.remove_hash(hash_as_str(i));
    }

    for i in 100_usize..1000 {
        assert!(bf.might_contain_hash(hash_as_str(i)));
    }

    let false_positives = (0_usize..100)
        .filter(|i| bf.might_contain_hash(hash_as_str(*i)))
        .count();

    assert!(false_positives < 20, "{} is not < 20", false_positives); // 20%.

    bf.clear();

    for i in 0_usize..2000 {
        assert!(!bf.might_contain_hash(hash_as_str(i)));
    }
}

#[cfg(feature = "bench")]
#[cfg(test)]
mod bench {
    extern crate test;
    use super::BloomFilter;

    #[derive(Default)]
    struct HashGenerator(u32);

    impl HashGenerator {
        fn next(&mut self) -> u32 {
            // Each hash is split into two twelve-bit segments, which are used
            // as an index into an array. We increment each by 64 so that we
            // hit the next cache line, and then another 1 so that our wrapping
            // behavior leads us to different entries each time.
            //
            // Trying to simulate cold caches is rather difficult with the cargo
            // benchmarking setup, so it may all be moot depending on the number
            // of iterations that end up being run. But we might as well.
            self.0 += (65) + (65 << super::KEY_SIZE);
            self.0
        }
    }

    #[bench]
    fn create_insert_1000_remove_100_lookup_100(b: &mut test::Bencher) {
        b.iter(|| {
            let mut gen1 = HashGenerator::default();
            let mut gen2 = HashGenerator::default();
            let mut bf = BloomFilter::new();
            for _ in 0_usize..1000 {
                bf.insert_hash(gen1.next());
            }
            for _ in 0_usize..100 {
                bf.remove_hash(gen2.next());
            }
            for _ in 100_usize..200 {
                test::black_box(bf.might_contain_hash(gen2.next()));
            }
        });
    }

    #[bench]
    fn might_contain_10(b: &mut test::Bencher) {
        let bf = BloomFilter::new();
        let mut gen = HashGenerator::default();
        b.iter(|| {
            for _ in 0..10 {
                test::black_box(bf.might_contain_hash(gen.next()));
            }
        });
    }

    #[bench]
    fn clear(b: &mut test::Bencher) {
        let mut bf = Box::new(BloomFilter::new());
        b.iter(|| test::black_box(&mut bf).clear());
    }

    #[bench]
    fn insert_10(b: &mut test::Bencher) {
        let mut bf = BloomFilter::new();
        let mut gen = HashGenerator::default();
        b.iter(|| {
            for _ in 0..10 {
                test::black_box(bf.insert_hash(gen.next()));
            }
        });
    }

    #[bench]
    fn remove_10(b: &mut test::Bencher) {
        let mut bf = BloomFilter::new();
        let mut gen = HashGenerator::default();
        // Note: this will underflow, and that's ok.
        b.iter(|| {
            for _ in 0..10 {
                bf.remove_hash(gen.next())
            }
        });
    }
}
