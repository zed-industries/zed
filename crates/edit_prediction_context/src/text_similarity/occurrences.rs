use arraydeque::ArrayDeque;
use collections::FxHasher;
use hashbrown::HashTable;
use smallvec::SmallVec;
use std::{
    fmt::Debug,
    hash::{Hash as _, Hasher as _},
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

/// Occurrence sets that can be constructed from hashes.
pub trait HashOccurrences {
    fn from_hashes(hashes: impl IntoIterator<Item = u32>) -> Self;
}

/// Multiset of hash occurrences used in similarity metrics.
#[derive(Debug, Default)]
pub struct OccurrenceMultiset {
    table: HashTable<OccurrenceEntry>,
    total_count: u32,
}

#[derive(Debug)]
struct OccurrenceEntry {
    hash: u32,
    count: u32,
}

/// Small set of hash occurrences. Since this does not track the number of times each has occurred,
/// this only implements `Similarity` and not `WeightedSimilarity`.
#[derive(Debug, Default)]
pub struct SmallOccurrenceSet<const N: usize>(SmallVec<[u32; N]>);

/// Wraps a hash occurrences set to implement n-grams, aka w-shingling. Each N length interval of
/// the input will be treated as one occurrence.
///
/// Note that this hashes the hashes it's provided - may more efficient to use a proper rolling
/// hash, especially for large N. However, didn't find a rust rolling hash implementation that
/// operated on updates larger than u8.
#[derive(Debug, Default)]
struct NGram<const N: usize, T>(T);

impl HashOccurrences for OccurrenceMultiset {
    fn from_hashes(hashes: impl IntoIterator<Item = u32>) -> Self {
        let mut this = Self::default();
        for hash in hashes {
            this.add_hash(hash);
        }
        this
    }
}

impl<const N: usize> HashOccurrences for SmallOccurrenceSet<N> {
    fn from_hashes(hashes: impl IntoIterator<Item = u32>) -> Self {
        let mut this = Self::default();
        this.0.extend(hashes);
        this.0.sort_unstable();
        this.0.dedup();
        this.0.shrink_to_fit();
        this
    }
}

impl<const N: usize, T: HashOccurrences> HashOccurrences for NGram<N, T> {
    fn from_hashes(hashes: impl IntoIterator<Item = u32>) -> Self {
        let mut window: ArrayDeque<u32, N, arraydeque::Wrapping> = ArrayDeque::new();
        NGram(T::from_hashes(hashes.into_iter().filter_map(|hash| {
            if window.push_back(hash).is_some() {
                let mut hasher = FxHasher::default();
                window.hash(&mut hasher);
                let (window_prefix, window_suffix) = window.as_slices();
                window_prefix.hash(&mut hasher);
                window_suffix.hash(&mut hasher);
                Some(hasher.finish() as u32)
            } else {
                None
            }
        })))
    }
}

impl OccurrenceMultiset {
    fn add_hash(&mut self, hash: u32) -> u32 {
        let new_count = self
            .table
            .entry(
                hash as u64,
                |entry: &OccurrenceEntry| entry.hash == hash,
                |entry| entry.hash as u64,
            )
            .and_modify(|entry| entry.count += 1)
            .or_insert(OccurrenceEntry { hash, count: 1 })
            .get()
            .count;
        self.total_count += 1;
        new_count
    }

    fn remove_hash(&mut self, hash: u32) -> u32 {
        let entry = self.table.entry(
            hash as u64,
            |entry: &OccurrenceEntry| entry.hash == hash,
            |entry| entry.hash as u64,
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

    fn contains_hash(&self, hash: u32) -> bool {
        self.get_count(hash) != 0
    }

    fn get_count(&self, hash: u32) -> u32 {
        self.table
            .find(hash as u64, |entry| entry.hash == hash)
            .map(|entry| entry.count)
            .unwrap_or(0)
    }
}

impl<const N: usize> SmallOccurrenceSet<N> {
    fn contains_hash(&self, hash: u32) -> bool {
        self.0.iter().any(|h| *h == hash)
    }
}

impl Similarity<OccurrenceMultiset> for OccurrenceMultiset {
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
        intersection as f32 / union as f32
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
        intersection as f32 / this.table.len() as f32
    }
}

impl<const N: usize> Similarity<OccurrenceMultiset> for SmallOccurrenceSet<N> {
    fn jaccard_similarity(&self, other: &OccurrenceMultiset) -> f32 {
        let intersection = self
            .0
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        let union = self.0.len() + other.table.len() - intersection;
        intersection as f32 / union as f32
    }

    fn overlap_coefficient(&self, other: &OccurrenceMultiset) -> f32 {
        let intersection = self
            .0
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        intersection as f32 / (self.0.len().min(other.table.len())) as f32
    }
}

impl<const N: usize, const O: usize> Similarity<SmallOccurrenceSet<O>> for SmallOccurrenceSet<N> {
    fn jaccard_similarity(&self, other: &SmallOccurrenceSet<O>) -> f32 {
        let intersection = self
            .0
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        let union = self.0.len() + other.0.len() - intersection;
        intersection as f32 / union as f32
    }

    fn overlap_coefficient(&self, other: &SmallOccurrenceSet<O>) -> f32 {
        let intersection = self
            .0
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        intersection as f32 / (self.0.len().min(other.0.len())) as f32
    }
}

impl WeightedSimilarity<OccurrenceMultiset> for OccurrenceMultiset {
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

impl<const N: usize, L: Similarity<R>, R> Similarity<NGram<N, R>> for NGram<N, L> {
    fn jaccard_similarity(&self, other: &NGram<N, R>) -> f32 {
        self.0.jaccard_similarity(&other.0)
    }

    fn overlap_coefficient(&self, other: &NGram<N, R>) -> f32 {
        self.0.overlap_coefficient(&other.0)
    }
}

impl<const N: usize, L: WeightedSimilarity<R>, R> WeightedSimilarity<NGram<N, R>> for NGram<N, L> {
    fn weighted_jaccard_similarity(&self, other: &NGram<N, R>) -> f32 {
        self.0.weighted_jaccard_similarity(&other.0)
    }

    fn weighted_overlap_coefficient(&self, other: &NGram<N, R>) -> f32 {
        self.0.weighted_overlap_coefficient(&other.0)
    }
}
