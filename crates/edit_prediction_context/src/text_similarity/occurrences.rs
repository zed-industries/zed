use arraydeque::ArrayDeque;
use collections::FxHasher;
use hashbrown::HashTable;
use smallvec::SmallVec;
use std::{
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

/// Occurrence sets that can be constructed from hashes.
pub trait HashOccurrences {
    fn from_hashes(hashes: impl IntoIterator<Item = u32>) -> Self;
}

/// Multiset of hash occurrences used in similarity metrics.
#[derive(Debug, Clone, Default)]
pub struct OccurrencesMultiset {
    table: HashTable<OccurrenceEntry>,
    total_count: u32,
}

#[derive(Debug, Clone)]
struct OccurrenceEntry {
    hash: u32,
    count: u32,
}

/// Small set of hash occurrences. Since this does not track the number of times each has occurred,
/// this only implements `Similarity` and not `WeightedSimilarity`.
#[derive(Debug, Default)]
pub struct SmallOccurrencesSet<const N: usize>(SmallVec<[u32; N]>);

pub type HashFrom<S> = FromSource<u32, S>;
pub type Occurrences<S> = FromSource<OccurrencesMultiset, S>;
pub type SmallOccurrences<const N: usize, S> = FromSource<SmallOccurrencesSet<N>, S>;

/// Indicates that a value comes from a particular source type. This provides safety, as it helps
/// ensure that the same input preprocessing is used when computing similarity metrics for
/// occurrences.
#[derive(Debug, Clone, Default)]
pub struct FromSource<T, S> {
    value: T,
    _source: PhantomData<S>,
}

/// Source type for occurrences that come from n-grams, aka w-shingling. Each N length interval of
/// the input will be treated as one occurrence.
///
/// Note that this hashes the hashes it's provided for every output - may be more efficient to use a
/// proper rolling hash. Unfortunately, I didn't find a rust rolling hash implementation that
/// operated on updates larger than u8.
struct NGram<const N: usize, S>(S);

impl<S> Occurrences<S> {
    pub fn new(hashes: impl IntoIterator<Item = HashFrom<S>>) -> Self {
        let mut occurrences = OccurrencesMultiset::default();
        for hash in hashes {
            occurrences.add_hash(hash.value);
        }
        occurrences.into()
    }
}

impl OccurrencesMultiset {
    pub fn len(&self) -> u32 {
        self.total_count
    }

    pub fn distinct_len(&self) -> usize {
        self.table.len()
    }

    pub fn add_hash(&mut self, hash: u32) -> u32 {
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

    pub fn remove_hash(&mut self, hash: u32) -> u32 {
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

    pub fn contains_hash(&self, hash: u32) -> bool {
        self.get_count(hash) != 0
    }

    pub fn get_count(&self, hash: u32) -> u32 {
        self.table
            .find(hash as u64, |entry| entry.hash == hash)
            .map(|entry| entry.count)
            .unwrap_or(0)
    }
}

impl<const N: usize, S> SmallOccurrences<N, S> {
    pub fn new(hashes: impl IntoIterator<Item = HashFrom<S>>) -> Self {
        let mut occurrences = SmallOccurrencesSet::default();
        occurrences
            .0
            .extend(hashes.into_iter().map(|hash| hash.value));
        occurrences.0.sort_unstable();
        occurrences.0.dedup();
        occurrences.0.shrink_to_fit();
        occurrences.into()
    }
}

impl<const N: usize> SmallOccurrencesSet<N> {
    fn contains_hash(&self, hash: u32) -> bool {
        self.0.iter().any(|h| *h == hash)
    }
}

impl Similarity<OccurrencesMultiset> for OccurrencesMultiset {
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

impl<const N: usize> Similarity<OccurrencesMultiset> for SmallOccurrencesSet<N> {
    fn jaccard_similarity(&self, other: &OccurrencesMultiset) -> f32 {
        let intersection = self
            .0
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        let union = self.0.len() + other.table.len() - intersection;
        intersection as f32 / union as f32
    }

    fn overlap_coefficient(&self, other: &OccurrencesMultiset) -> f32 {
        let intersection = self
            .0
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        intersection as f32 / (self.0.len().min(other.table.len())) as f32
    }
}

impl<const N: usize, const O: usize> Similarity<SmallOccurrencesSet<O>> for SmallOccurrencesSet<N> {
    fn jaccard_similarity(&self, other: &SmallOccurrencesSet<O>) -> f32 {
        let intersection = self
            .0
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        let union = self.0.len() + other.0.len() - intersection;
        intersection as f32 / union as f32
    }

    fn overlap_coefficient(&self, other: &SmallOccurrencesSet<O>) -> f32 {
        let intersection = self
            .0
            .iter()
            .filter(|hash| other.contains_hash(**hash))
            .count();
        intersection as f32 / (self.0.len().min(other.0.len())) as f32
    }
}

impl WeightedSimilarity<OccurrencesMultiset> for OccurrencesMultiset {
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

impl<V, S> From<V> for FromSource<V, S> {
    fn from(value: V) -> Self {
        Self {
            value,
            _source: PhantomData,
        }
    }
}

impl<V: Hash, S> Hash for FromSource<V, S> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.value.hash(hasher);
    }
}

impl<LI: Similarity<RI>, RI, S> Similarity<FromSource<RI, S>> for FromSource<LI, S> {
    fn jaccard_similarity(&self, other: &FromSource<RI, S>) -> f32 {
        self.value.jaccard_similarity(&other.value)
    }

    fn overlap_coefficient(&self, other: &FromSource<RI, S>) -> f32 {
        self.value.overlap_coefficient(&other.value)
    }
}

impl<LI: WeightedSimilarity<RI>, RI, S> WeightedSimilarity<FromSource<RI, S>>
    for FromSource<LI, S>
{
    fn weighted_jaccard_similarity(&self, other: &FromSource<RI, S>) -> f32 {
        self.value.weighted_jaccard_similarity(&other.value)
    }

    fn weighted_overlap_coefficient(&self, other: &FromSource<RI, S>) -> f32 {
        self.value.weighted_overlap_coefficient(&other.value)
    }
}

struct NGramIterator<const N: usize, S, I> {
    hashes: I,
    window: ArrayDeque<HashFrom<S>, N, arraydeque::Wrapping>,
}

impl<const N: usize, S, I> NGramIterator<N, S, I>
where
    I: Iterator<Item = HashFrom<S>>,
{
    fn new<V: Hash>(hashes: I) -> Self {
        Self {
            hashes,
            window: ArrayDeque::new(),
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
            if self.window.push_back(hash).is_some() {
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
