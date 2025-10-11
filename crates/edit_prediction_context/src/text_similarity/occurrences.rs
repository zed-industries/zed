use arraydeque::ArrayDeque;
use collections::FxHasher;
use hashbrown::HashTable;
use itertools::Itertools;
use smallvec::SmallVec;
use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
};
use util::debug_panic;

/// Similarity metrics that use a set of occurrences.
pub trait Similarity<T> {
    fn jaccard_similarity(&self, other: &T) -> f32;
    fn overlap_coefficient(&self, other: &T) -> f32;
}

/// Similarity metrics that use a multiset of occurrences.
pub trait WeightedSimilarity<T> {
    fn weighted_jaccard_similarity(&self, other: &T) -> f32;
    fn weighted_overlap_coefficient(&self, other: &T) -> f32;
}

/// Multiset of hash occurrences used in similarity metrics.
#[derive(Debug, Clone)]
pub struct Occurrences<S> {
    table: HashTable<OccurrenceEntry<S>>,
    total_count: u32,
    _source: PhantomData<S>,
}

#[derive(Debug, Copy, Clone)]
struct OccurrenceEntry<S> {
    hash: HashFrom<S>,
    count: u32,
}

/// Small set of hash occurrences. Since this does not track the number of times each has occurred,
/// this only implements `Similarity` and not `WeightedSimilarity`.
#[derive(Debug, Clone)]
pub struct SmallOccurrences<const N: usize, S> {
    hashes: SmallVec<[HashFrom<S>; N]>,
    _source: PhantomData<S>,
}

/// Occurrence hash from a particular source type.
///
/// u32 hash values are used instead of u64 since this uses half as much memory and is a bit faster.
/// The rationale for this is that these similarity metrics are being used as heuristics and
/// precision is not needed. The probability of a hash collision is 50% with 77k hashes. The
/// probability that a hash collision will actually cause different behavior is far lower.
pub struct HashFrom<S> {
    value: u32,
    _source: PhantomData<S>,
}

/// Source type for occurrences that come from n-grams, aka w-shingling. Each N length interval of
/// the input will be treated as one occurrence.
///
/// Note that this hashes the hashes it's provided for every output - may be more efficient to use a
/// proper rolling hash. Unfortunately, I didn't find a rust rolling hash implementation that
/// operated on updates larger than u8.
#[derive(Debug)]
pub struct NGram<const N: usize, S> {
    _source: PhantomData<S>,
}

impl<S> Occurrences<S> {
    pub fn new(hashes: impl IntoIterator<Item = HashFrom<S>>) -> Self {
        let mut occurrences = Occurrences::default();
        for hash in hashes {
            occurrences.add_hash(hash);
        }
        occurrences.into()
    }

    pub fn clear(&mut self) {
        self.table.clear();
        self.total_count = 0;
    }

    pub fn len(&self) -> u32 {
        self.total_count
    }

    pub fn distinct_len(&self) -> usize {
        self.table.len()
    }

    pub fn add_hash(&mut self, hash: HashFrom<S>) -> u32 {
        let new_count = self
            .table
            .entry(
                hash.value as u64,
                |entry| entry.hash == hash,
                |entry| entry.hash.value as u64,
            )
            .and_modify(|entry| entry.count += 1)
            .or_insert(OccurrenceEntry { hash, count: 1 })
            .get()
            .count;
        self.total_count += 1;
        new_count
    }

    pub fn remove_hash(&mut self, hash: HashFrom<S>) -> u32 {
        let entry = self.table.entry(
            hash.value as u64,
            |entry| entry.hash == hash,
            |entry| entry.hash.value as u64,
        );
        match entry {
            hashbrown::hash_table::Entry::Occupied(mut entry) => {
                let new_count = entry.get().count.checked_sub(1);
                if let Some(new_count) = new_count {
                    if new_count == 0 {
                        entry.remove();
                    } else {
                        entry.get_mut().count = new_count;
                    }
                    debug_assert!(self.total_count != 0);
                    self.total_count = self.total_count.saturating_sub(1);
                    new_count
                } else {
                    debug_panic!("Hash removed from occurrences more times than it was added.");
                    0
                }
            }
            hashbrown::hash_table::Entry::Vacant(_) => {
                debug_panic!("Hash removed from occurrences more times than it was added.");
                0
            }
        }
    }

    pub fn contains_hash(&self, hash: HashFrom<S>) -> bool {
        self.get_count(hash) != 0
    }

    pub fn get_count(&self, hash: HashFrom<S>) -> u32 {
        self.table
            .find(hash.value as u64, |entry| entry.hash == hash)
            .map(|entry| entry.count)
            .unwrap_or(0)
    }
}

impl<const N: usize, S> SmallOccurrences<N, S> {
    pub fn new(hashes: impl IntoIterator<Item = HashFrom<S>>) -> Self {
        let mut this = SmallOccurrences::default();
        this.hashes.extend(hashes);
        this.hashes.sort_unstable();
        this.hashes.dedup();
        this.hashes.shrink_to_fit();
        this
    }

    pub fn distinct_len(&self) -> usize {
        self.hashes.len()
    }

    fn contains_hash(&self, hash: HashFrom<S>) -> bool {
        self.hashes.iter().contains(&hash)
    }
}

impl<S> Default for Occurrences<S> {
    fn default() -> Self {
        Occurrences {
            table: Default::default(),
            total_count: 0,
            _source: PhantomData,
        }
    }
}

impl<const N: usize, S> Default for SmallOccurrences<N, S> {
    fn default() -> Self {
        SmallOccurrences {
            hashes: SmallVec::new(),
            _source: PhantomData,
        }
    }
}

impl<S> Similarity<Occurrences<S>> for Occurrences<S> {
    fn jaccard_similarity<'a>(&'a self, mut other: &'a Self) -> f32 {
        let mut this = self;
        if this.table.len() > other.table.len() {
            std::mem::swap(&mut this, &mut other);
        }
        let intersection = this
            .table
            .iter()
            .filter(|entry| other.contains_hash(entry.hash))
            .count();
        let union = this.table.len() + other.table.len() - intersection;
        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    fn overlap_coefficient<'a>(&'a self, mut other: &'a Self) -> f32 {
        let mut this = self;
        if this.table.len() > other.table.len() {
            std::mem::swap(&mut this, &mut other);
        }
        let intersection = this
            .table
            .iter()
            .filter(|entry| other.contains_hash(entry.hash))
            .count();
        let smaller = this.table.len();
        if smaller == 0 {
            0.0
        } else {
            intersection as f32 / smaller as f32
        }
    }
}

impl<const N: usize, S> Similarity<Occurrences<S>> for SmallOccurrences<N, S> {
    fn jaccard_similarity(&self, other: &Occurrences<S>) -> f32 {
        let intersection = self
            .hashes
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        let union = self.hashes.len() + other.table.len() - intersection;
        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    fn overlap_coefficient(&self, other: &Occurrences<S>) -> f32 {
        let intersection = self
            .hashes
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        let smaller = self.hashes.len().min(other.table.len());
        if smaller == 0 {
            0.0
        } else {
            intersection as f32 / smaller as f32
        }
    }
}

impl<const N: usize, const O: usize, S> Similarity<SmallOccurrences<O, S>>
    for SmallOccurrences<N, S>
{
    fn jaccard_similarity(&self, other: &SmallOccurrences<O, S>) -> f32 {
        let intersection = self
            .hashes
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        let union = self.hashes.len() + other.hashes.len() - intersection;
        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    fn overlap_coefficient(&self, other: &SmallOccurrences<O, S>) -> f32 {
        let intersection = self
            .hashes
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        let smaller = self.hashes.len().min(other.hashes.len());
        if smaller == 0 {
            0.0
        } else {
            intersection as f32 / smaller as f32
        }
    }
}

impl<S> WeightedSimilarity<Occurrences<S>> for Occurrences<S> {
    fn weighted_jaccard_similarity<'a>(&'a self, mut other: &'a Self) -> f32 {
        let mut this = self;
        if this.table.len() > other.table.len() {
            std::mem::swap(&mut this, &mut other);
        }

        let mut numerator = 0;
        let mut this_denominator = 0;
        let mut other_used_count = 0;
        for entry in this.table.iter() {
            let this_count = entry.count;
            let other_count = other.get_count(entry.hash);
            numerator += this_count.min(other_count);
            this_denominator += this_count.max(other_count);
            other_used_count += other_count;
        }

        let denominator = this_denominator + (other.total_count - other_used_count);
        if denominator == 0 {
            0.0
        } else {
            numerator as f32 / denominator as f32
        }
    }

    fn weighted_overlap_coefficient<'a>(&'a self, mut other: &'a Self) -> f32 {
        let mut this = self;
        if this.table.len() > other.table.len() {
            std::mem::swap(&mut this, &mut other);
        }

        let mut numerator = 0;
        for entry in this.table.iter() {
            let this_count = entry.count;
            let other_count = other.get_count(entry.hash);
            numerator += this_count.min(other_count);
        }

        let denominator = this.total_count.min(other.total_count);
        if denominator == 0 {
            0.0
        } else {
            numerator as f32 / denominator as f32
        }
    }
}

impl<S> From<u32> for HashFrom<S> {
    fn from(value: u32) -> Self {
        Self {
            value,
            _source: PhantomData,
        }
    }
}

impl<S> Debug for HashFrom<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<S> PartialEq for HashFrom<S> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<S> PartialOrd for HashFrom<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S> Ord for HashFrom<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<S> Clone for HashFrom<S> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _source: PhantomData,
        }
    }
}

impl<S> Eq for HashFrom<S> {}
impl<S> Copy for HashFrom<S> {}

struct NGramIterator<const N: usize, S, I> {
    hashes: I,
    window: ArrayDeque<u32, N, arraydeque::Wrapping>,
    _source: PhantomData<S>,
}

impl<const N: usize, S> NGram<N, S> {
    pub fn iterator<I: IntoIterator<Item = HashFrom<S>>>(
        hashes: I,
    ) -> impl Iterator<Item = HashFrom<NGram<N, S>>> {
        NGramIterator {
            hashes: hashes.into_iter(),
            window: ArrayDeque::new(),
            _source: PhantomData,
        }
    }
}

impl<const N: usize, S, I> Iterator for NGramIterator<N, S, I>
where
    I: Iterator<Item = HashFrom<S>>,
{
    type Item = HashFrom<NGram<N, S>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(hash) = self.hashes.next() {
            if self.window.push_back(hash.value).is_some() {
                let mut hasher = FxHasher::default();
                let (window_prefix, window_suffix) = self.window.as_slices();
                window_prefix.hash(&mut hasher);
                window_suffix.hash(&mut hasher);
                return Some((hasher.finish() as u32).into());
            }
        }
        return None;
    }
}

impl<S> AsRef<Occurrences<S>> for Occurrences<S> {
    fn as_ref(&self) -> &Occurrences<S> {
        self
    }
}
