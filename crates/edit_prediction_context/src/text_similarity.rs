use hashbrown::HashTable;
use regex::Regex;
use std::{
    collections::VecDeque,
    fmt::Debug,
    hash::{Hash, Hasher as _},
    sync::LazyLock,
};
use util::debug_panic;

use crate::reference::Reference;

// Variants to consider:
//
// * Score matches that match case higher?
//
// * Also include unsplit identifier?
//
// * N-grams
//
// * Flat sorted Vec<(String, usize)> representation - more compact / efficient to iterate.
// Intersection can just walk two in parallel.

static IDENTIFIER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b\w+\b").unwrap());

/// Multiset of text occurrences for text similarity that only stores hashes and counts.
#[derive(Debug, Default)]
pub struct Occurrences {
    table: HashTable<OccurrenceEntry>,
    total_count: usize,
}

#[derive(Debug)]
struct OccurrenceEntry {
    hash: u64,
    count: usize,
}

impl Occurrences {
    pub fn within_string(text: &str) -> Self {
        Self::from_hashes(hashes_of_lowercase_identifier_parts(text))
    }

    #[allow(dead_code)]
    pub fn within_references(references: &[Reference]) -> Self {
        Self::from_hashes(
            references
                .iter()
                .flat_map(|reference| split_identifier(reference.identifier.name.as_ref()))
                .map(|part| fx_hash(&part.to_ascii_lowercase())),
        )
    }

    pub fn from_identifiers<'a>(identifiers: impl IntoIterator<Item = &'a str>) -> Self {
        let mut this = Self::default();
        for identifier in identifiers {
            for identifier_part in split_identifier(identifier) {
                this.add_hash(fx_hash(&identifier_part.to_ascii_lowercase()));
            }
        }
        this
    }

    pub fn from_hashes(hashes: impl IntoIterator<Item = u64>) -> Self {
        let mut this = Self::default();
        for hash in hashes {
            this.add_hash(hash);
        }
        this
    }

    fn add_hash(&mut self, hash: u64) -> usize {
        let new_count = self
            .table
            .entry(
                hash,
                |entry: &OccurrenceEntry| entry.hash == hash,
                |entry| entry.hash,
            )
            .and_modify(|entry| entry.count += 1)
            .or_insert(OccurrenceEntry { hash, count: 1 })
            .get()
            .count;
        self.total_count += 1;
        new_count
    }

    fn remove_hash(&mut self, hash: u64) -> usize {
        let entry = self.table.entry(
            hash,
            |entry: &OccurrenceEntry| entry.hash == hash,
            |entry| entry.hash,
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

    fn contains_hash(&self, hash: u64) -> bool {
        self.get_count(hash) != 0
    }

    fn get_count(&self, hash: u64) -> usize {
        self.table
            .find(hash, |entry| entry.hash == hash)
            .map(|entry| entry.count)
            .unwrap_or(0)
    }
}

pub fn hashes_of_lowercase_identifier_parts(text: &str) -> impl Iterator<Item = u64> {
    IDENTIFIER_REGEX
        .find_iter(text)
        .flat_map(|mat| split_identifier(mat.as_str()))
        .map(|part| fx_hash(&part.to_ascii_lowercase()))
}

fn fx_hash<T: Hash + ?Sized>(data: &T) -> u64 {
    let mut hasher = collections::FxHasher::default();
    data.hash(&mut hasher);
    hasher.finish()
}

// Splits camelcase / snakecase / kebabcase / pascalcase
fn split_identifier(identifier: &str) -> IdentifierSplitIterator<'_> {
    IdentifierSplitIterator::new(identifier)
}

struct IdentifierSplitIterator<'a> {
    input: &'a str,
    chars: std::str::CharIndices<'a>,
    start: usize,
    prev_char_is_lowercase: bool,
    prev_char_is_uppercase: bool,
    prev_char_is_ascii_digit: bool,
}

impl<'a> IdentifierSplitIterator<'a> {
    fn new(identifier: &'a str) -> Self {
        Self {
            input: identifier,
            chars: identifier.char_indices(),
            start: 0,
            prev_char_is_lowercase: false,
            prev_char_is_uppercase: false,
            prev_char_is_ascii_digit: false,
        }
    }
}

impl<'a> Iterator for IdentifierSplitIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((byte_index, ch)) = self.chars.next() {
            if ch == '_' || ch == '-' {
                self.prev_char_is_lowercase = false;
                self.prev_char_is_uppercase = false;
                self.prev_char_is_ascii_digit = false;
                if byte_index > self.start {
                    let part = &self.input[self.start..byte_index];
                    self.start = byte_index + 1;
                    return Some(part);
                }
                self.start = byte_index + 1;
                continue;
            }

            // Handle camelCase and PascalCase transitions
            let should_split = if byte_index <= self.start {
                false
            } else {
                if (self.prev_char_is_lowercase || self.prev_char_is_ascii_digit)
                    && ch.is_uppercase()
                {
                    // lowercase/digit to uppercase
                    true
                } else if ch.is_uppercase() && self.prev_char_is_uppercase {
                    // Handle sequences like "XMLParser" -> ["XML", "Parser"]
                    self.input[byte_index..]
                        .chars()
                        .nth(1)
                        .map_or(false, |c| c.is_lowercase())
                } else {
                    false
                }
            };

            self.prev_char_is_lowercase = ch.is_lowercase();
            self.prev_char_is_uppercase = ch.is_uppercase();
            self.prev_char_is_ascii_digit = ch.is_ascii_digit();

            if should_split {
                let part = &self.input[self.start..byte_index];
                self.start = byte_index;
                return Some(part);
            }
        }

        if self.start < self.input.len() {
            let part = &self.input[self.start..];
            self.start = self.input.len();
            if !part.is_empty() {
                return Some(part);
            }
        }

        None
    }
}

pub fn jaccard_similarity<'a>(mut set_a: &'a Occurrences, mut set_b: &'a Occurrences) -> f32 {
    if set_a.table.len() > set_b.table.len() {
        std::mem::swap(&mut set_a, &mut set_b);
    }
    let intersection = set_a
        .table
        .iter()
        .filter(|entry| set_b.contains_hash(entry.hash))
        .count();
    let union = set_a.table.len() + set_b.table.len() - intersection;
    intersection as f32 / union as f32
}

// TODO
#[allow(dead_code)]
pub fn overlap_coefficient<'a>(mut set_a: &'a Occurrences, mut set_b: &'a Occurrences) -> f32 {
    if set_a.table.len() > set_b.table.len() {
        std::mem::swap(&mut set_a, &mut set_b);
    }
    let intersection = set_a
        .table
        .iter()
        .filter(|entry| set_b.contains_hash(entry.hash))
        .count();
    intersection as f32 / set_a.table.len() as f32
}

// TODO
#[allow(dead_code)]
pub fn weighted_jaccard_similarity<'a>(
    mut set_a: &'a Occurrences,
    mut set_b: &'a Occurrences,
) -> f32 {
    if set_a.table.len() > set_b.table.len() {
        std::mem::swap(&mut set_a, &mut set_b);
    }

    let mut numerator = 0;
    let mut denominator_a = 0;
    let mut used_count_b = 0;
    for entry_a in set_a.table.iter() {
        let count_a = entry_a.count;
        let count_b = set_b.get_count(entry_a.hash);
        numerator += count_a.min(count_b);
        denominator_a += count_a.max(count_b);
        used_count_b += count_b;
    }

    let denominator = denominator_a + (set_b.total_count - used_count_b);
    if denominator == 0 {
        0.0
    } else {
        numerator as f32 / denominator as f32
    }
}

pub fn weighted_overlap_coefficient<'a>(
    mut set_a: &'a Occurrences,
    mut set_b: &'a Occurrences,
) -> f32 {
    if set_a.table.len() > set_b.table.len() {
        std::mem::swap(&mut set_a, &mut set_b);
    }

    let mut numerator = 0;
    for entry_a in set_a.table.iter() {
        let count_a = entry_a.count;
        let count_b = set_b.get_count(entry_a.hash);
        numerator += count_a.min(count_b);
    }

    let denominator = set_a.total_count.min(set_b.total_count);
    if denominator == 0 {
        0.0
    } else {
        numerator as f32 / denominator as f32
    }
}

pub struct SlidingWindow<Target, Id> {
    target: Target,
    intersection: Occurrences,
    regions: VecDeque<WeightedOverlapRegion<Id>>,
    numerator: usize,
    window_count: usize,
    jaccard_denominator_part: usize,
}

pub struct WeightedOverlapRegion<Id> {
    id: Id,
    added_hashes: Vec<u64>,
    numerator_delta: usize,
    window_count_delta: usize,
    jaccard_denominator_delta: usize,
}

impl AsRef<Occurrences> for Occurrences {
    fn as_ref(&self) -> &Occurrences {
        self
    }
}

impl<Id: Debug + PartialEq, Target: AsRef<Occurrences>> SlidingWindow<Target, Id> {
    pub fn new(target: Target, capacity: usize) -> Self {
        let jaccard_denominator_part = target.as_ref().total_count;
        Self {
            target,
            intersection: Occurrences::default(),
            regions: VecDeque::with_capacity(capacity),
            numerator: 0,
            window_count: 0,
            jaccard_denominator_part,
        }
    }

    pub fn add(&mut self, id: Id, hashes: impl IntoIterator<Item = u64>) {
        let mut added_hashes = Vec::new();
        let mut numerator_delta = 0;
        let mut jaccard_denominator_delta = 0;
        let mut window_count_delta = 0;
        for hash in hashes {
            window_count_delta += 1;
            let target_count = self.target.as_ref().get_count(hash);
            if target_count > 0 {
                added_hashes.push(hash);
                let window_count = self.intersection.add_hash(hash);
                if window_count <= target_count {
                    numerator_delta += 1;
                } else {
                    jaccard_denominator_delta += 1;
                }
            }
        }
        self.numerator += numerator_delta;
        self.window_count += window_count_delta;
        self.jaccard_denominator_part += jaccard_denominator_delta;
        self.regions.push_back(WeightedOverlapRegion {
            id,
            added_hashes,
            numerator_delta,
            window_count_delta,
            jaccard_denominator_delta,
        });
    }

    pub fn remove(&mut self, id: Id) {
        let removed;
        #[cfg(debug_assertions)]
        {
            removed = self
                .regions
                .pop_front()
                .expect("No sliding window region to remove");
            debug_assert_eq!(removed.id, id);
        }

        #[cfg(not(debug_assertions))]
        {
            removed = self.regions.pop_front();
            let Some(removed) = removed else {
                return;
            };
        }

        for hash in removed.added_hashes {
            self.intersection.remove_hash(hash);
        }

        if let Some(numerator) = self.numerator.checked_sub(removed.numerator_delta)
            && let Some(window_count) = self.window_count.checked_sub(removed.window_count_delta)
            && let Some(jaccard_denominator_part) = self
                .jaccard_denominator_part
                .checked_sub(removed.jaccard_denominator_delta)
        {
            self.numerator = numerator;
            self.window_count = window_count;
            self.jaccard_denominator_part = jaccard_denominator_part;
        } else {
            debug_panic!("bug: underflow in sliding window text similarity");
        }
    }

    pub fn weighted_overlap_coefficient(&self) -> f32 {
        let denominator = self.target.as_ref().total_count.min(self.window_count);
        self.numerator as f32 / denominator as f32
    }

    pub fn weighted_jaccard_similarity(&self) -> f32 {
        let mut denominator = self.jaccard_denominator_part;
        if let Some(other_denominator_part) =
            self.window_count.checked_sub(self.intersection.total_count)
        {
            denominator += other_denominator_part;
        } else {
            debug_panic!("bug: underflow in sliding window text similarity");
        }
        self.numerator as f32 / denominator as f32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_identifier() {
        assert_eq!(
            split_identifier("snake_case").collect::<Vec<_>>(),
            vec!["snake", "case"]
        );
        assert_eq!(
            split_identifier("kebab-case").collect::<Vec<_>>(),
            vec!["kebab", "case"]
        );
        assert_eq!(
            split_identifier("PascalCase").collect::<Vec<_>>(),
            vec!["Pascal", "Case"]
        );
        assert_eq!(
            split_identifier("camelCase").collect::<Vec<_>>(),
            vec!["camel", "Case"]
        );
        assert_eq!(
            split_identifier("XMLParser").collect::<Vec<_>>(),
            vec!["XML", "Parser"]
        );
        assert_eq!(split_identifier("").collect::<Vec<_>>(), Vec::<&str>::new());
        assert_eq!(split_identifier("a").collect::<Vec<_>>(), vec!["a"]);
        assert_eq!(split_identifier("ABC").collect::<Vec<_>>(), vec!["ABC"]);
        assert_eq!(split_identifier("abc").collect::<Vec<_>>(), vec!["abc"]);
        assert_eq!(split_identifier("123").collect::<Vec<_>>(), vec!["123"]);
        assert_eq!(
            split_identifier("a1B2c3").collect::<Vec<_>>(),
            vec!["a1", "B2c3"]
        );
        assert_eq!(
            split_identifier("HTML5Parser").collect::<Vec<_>>(),
            vec!["HTML5", "Parser"]
        );
        assert_eq!(
            split_identifier("_leading_underscore").collect::<Vec<_>>(),
            vec!["leading", "underscore"]
        );
        assert_eq!(
            split_identifier("trailing_underscore_").collect::<Vec<_>>(),
            vec!["trailing", "underscore"]
        );
        assert_eq!(
            split_identifier("--multiple--delimiters--").collect::<Vec<_>>(),
            vec!["multiple", "delimiters"]
        );
    }

    #[test]
    fn test_similarity_functions() {
        // 10 identifier parts, 8 unique
        // Repeats: 2 "outline", 2 "items"
        let set_a = Occurrences::within_string(
            "let mut outline_items = query_outline_items(&language, &tree, &source);",
        );
        // 14 identifier parts, 11 unique
        // Repeats: 2 "outline", 2 "language", 2 "tree"
        let set_b = Occurrences::within_string(
            "pub fn query_outline_items(language: &Language, tree: &Tree, source: &str) -> Vec<OutlineItem> {",
        );

        // 6 overlaps: "outline", "items", "query", "language", "tree", "source"
        // 7 non-overlaps: "let", "mut", "pub", "fn", "vec", "item", "str"
        assert_eq!(jaccard_similarity(&set_a, &set_b), 6.0 / (6.0 + 7.0));

        // Numerator is one more than before due to both having 2 "outline".
        // Denominator is the same except for 3 more due to the non-overlapping duplicates
        assert_eq!(
            weighted_jaccard_similarity(&set_a, &set_b),
            7.0 / (7.0 + 7.0 + 3.0)
        );

        // Numerator is the same as jaccard_similarity. Denominator is the size of the smaller set, 8.
        assert_eq!(overlap_coefficient(&set_a, &set_b), 6.0 / 8.0);

        // Numerator is the same as weighted_jaccard_similarity. Denominator is the total weight of
        // the smaller set, 10.
        assert_eq!(weighted_overlap_coefficient(&set_a, &set_b), 7.0 / 10.0);
    }
}
