use hashbrown::HashTable;
use regex::Regex;
use std::{
    borrow::Cow,
    hash::{Hash, Hasher as _},
    path::Path,
    sync::LazyLock,
};
use util::rel_path::RelPath;

use crate::reference::Reference;

// TODO: Consider implementing sliding window similarity matching like
// https://github.com/sourcegraph/cody-public-snapshot/blob/8e20ac6c1460c08b0db581c0204658112a246eda/vscode/src/completions/context/retrievers/jaccard-similarity/bestJaccardMatch.ts
//
// That implementation could actually be more efficient - no need to track words in the window that
// are not in the query.

// TODO: Consider a flat sorted Vec<(String, usize)> representation. Intersection can just walk the
// two in parallel.

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
        Self::from_identifiers(IDENTIFIER_REGEX.find_iter(text).map(|mat| mat.as_str()))
    }

    #[allow(dead_code)]
    pub fn within_references(references: &[Reference]) -> Self {
        Self::from_identifiers(
            references
                .iter()
                .map(|reference| reference.identifier.name.as_ref()),
        )
    }

    pub fn from_identifiers(identifiers: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let mut this = Self::default();
        // TODO: Score matches that match case higher?
        //
        // TODO: Also include unsplit identifier?
        for identifier in identifiers {
            for identifier_part in split_identifier(identifier.as_ref()) {
                this.add_hash(fx_hash(&identifier_part.to_lowercase()));
            }
        }
        this
    }

    pub fn from_worktree_path(worktree_name: Option<Cow<'_, str>>, rel_path: &RelPath) -> Self {
        if let Some(worktree_name) = worktree_name {
            Self::from_identifiers(
                std::iter::once(worktree_name)
                    .chain(iter_path_without_extension(rel_path.as_std_path())),
            )
        } else {
            Self::from_path(rel_path.as_std_path())
        }
    }

    pub fn from_path(path: &Path) -> Self {
        Self::from_identifiers(iter_path_without_extension(path))
    }

    fn add_hash(&mut self, hash: u64) {
        self.table
            .entry(
                hash,
                |entry: &OccurrenceEntry| entry.hash == hash,
                |entry| entry.hash,
            )
            .and_modify(|entry| entry.count += 1)
            .or_insert(OccurrenceEntry { hash, count: 1 });
        self.total_count += 1;
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

fn iter_path_without_extension(path: &Path) -> impl Iterator<Item = Cow<'_, str>> {
    let last_component: Option<Cow<'_, str>> = path.file_stem().map(|stem| stem.to_string_lossy());
    let mut path_components = path.components();
    path_components.next_back();
    path_components
        .map(|component| component.as_os_str().to_string_lossy())
        .chain(last_component)
}

pub fn fx_hash<T: Hash + ?Sized>(data: &T) -> u64 {
    let mut hasher = collections::FxHasher::default();
    data.hash(&mut hasher);
    hasher.finish()
}

// Splits camelcase / snakecase / kebabcase / pascalcase
//
// TODO: Make this more efficient / elegant.
fn split_identifier(identifier: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = identifier.chars().collect();

    if chars.is_empty() {
        return parts;
    }

    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];

        // Handle explicit delimiters (underscore and hyphen)
        if ch == '_' || ch == '-' {
            if i > start {
                parts.push(&identifier[start..i]);
            }
            start = i + 1;
            i += 1;
            continue;
        }

        // Handle camelCase and PascalCase transitions
        if i > 0 && i < chars.len() {
            let prev_char = chars[i - 1];

            // Transition from lowercase/digit to uppercase
            if (prev_char.is_lowercase() || prev_char.is_ascii_digit()) && ch.is_uppercase() {
                parts.push(&identifier[start..i]);
                start = i;
            }
            // Handle sequences like "XMLParser" -> ["XML", "Parser"]
            else if i + 1 < chars.len()
                && ch.is_uppercase()
                && chars[i + 1].is_lowercase()
                && prev_char.is_uppercase()
            {
                parts.push(&identifier[start..i]);
                start = i;
            }
        }

        i += 1;
    }

    // Add the last part if there's any remaining
    if start < identifier.len() {
        parts.push(&identifier[start..]);
    }

    // Filter out empty strings
    parts.into_iter().filter(|s| !s.is_empty()).collect()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_identifier() {
        assert_eq!(split_identifier("snake_case"), vec!["snake", "case"]);
        assert_eq!(split_identifier("kebab-case"), vec!["kebab", "case"]);
        assert_eq!(split_identifier("PascalCase"), vec!["Pascal", "Case"]);
        assert_eq!(split_identifier("camelCase"), vec!["camel", "Case"]);
        assert_eq!(split_identifier("XMLParser"), vec!["XML", "Parser"]);
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

    #[test]
    fn test_iter_path_without_extension() {
        let mut iter = iter_path_without_extension(Path::new(""));
        assert_eq!(iter.next(), None);

        let iter = iter_path_without_extension(Path::new("foo"));
        assert_eq!(iter.collect::<Vec<_>>(), ["foo"]);

        let iter = iter_path_without_extension(Path::new("foo/bar.txt"));
        assert_eq!(iter.collect::<Vec<_>>(), ["foo", "bar"]);

        let iter = iter_path_without_extension(Path::new("foo/bar/baz.txt"));
        assert_eq!(iter.collect::<Vec<_>>(), ["foo", "bar", "baz"]);
    }
}
