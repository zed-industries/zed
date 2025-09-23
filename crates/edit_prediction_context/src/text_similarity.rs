use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

use crate::reference::Reference;

// TODO: Consider implementing sliding window similarity matching like
// https://github.com/sourcegraph/cody-public-snapshot/blob/8e20ac6c1460c08b0db581c0204658112a246eda/vscode/src/completions/context/retrievers/jaccard-similarity/bestJaccardMatch.ts
//
// That implementation could actually be more efficient - no need to track words in the window that
// are not in the query.

// TODO: Consider a flat sorted Vec<(String, usize)> representation. Intersection can just walk the
// two in parallel.

static IDENTIFIER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b\w+\b").unwrap());

// TODO: use &str or Cow<str> keys?
#[derive(Debug)]
pub struct IdentifierOccurrences {
    identifier_to_count: HashMap<String, usize>,
    total_count: usize,
}

impl IdentifierOccurrences {
    pub fn within_string(code: &str) -> Self {
        Self::from_iterator(IDENTIFIER_REGEX.find_iter(code).map(|mat| mat.as_str()))
    }

    #[allow(dead_code)]
    pub fn within_references(references: &[Reference]) -> Self {
        Self::from_iterator(
            references
                .iter()
                .map(|reference| reference.identifier.name.as_ref()),
        )
    }

    pub fn from_iterator<'a>(identifier_iterator: impl Iterator<Item = &'a str>) -> Self {
        let mut identifier_to_count = HashMap::new();
        let mut total_count = 0;
        for identifier in identifier_iterator {
            // TODO: Score matches that match case higher?
            //
            // TODO: Also include unsplit identifier?
            for identifier_part in split_identifier(identifier) {
                identifier_to_count
                    .entry(identifier_part.to_lowercase())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                total_count += 1;
            }
        }
        IdentifierOccurrences {
            identifier_to_count,
            total_count,
        }
    }
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

pub fn jaccard_similarity<'a>(
    mut set_a: &'a IdentifierOccurrences,
    mut set_b: &'a IdentifierOccurrences,
) -> f32 {
    if set_a.identifier_to_count.len() > set_b.identifier_to_count.len() {
        std::mem::swap(&mut set_a, &mut set_b);
    }
    let intersection = set_a
        .identifier_to_count
        .keys()
        .filter(|key| set_b.identifier_to_count.contains_key(*key))
        .count();
    let union = set_a.identifier_to_count.len() + set_b.identifier_to_count.len() - intersection;
    intersection as f32 / union as f32
}

// TODO
#[allow(dead_code)]
pub fn overlap_coefficient<'a>(
    mut set_a: &'a IdentifierOccurrences,
    mut set_b: &'a IdentifierOccurrences,
) -> f32 {
    if set_a.identifier_to_count.len() > set_b.identifier_to_count.len() {
        std::mem::swap(&mut set_a, &mut set_b);
    }
    let intersection = set_a
        .identifier_to_count
        .keys()
        .filter(|key| set_b.identifier_to_count.contains_key(*key))
        .count();
    intersection as f32 / set_a.identifier_to_count.len() as f32
}

// TODO
#[allow(dead_code)]
pub fn weighted_jaccard_similarity<'a>(
    mut set_a: &'a IdentifierOccurrences,
    mut set_b: &'a IdentifierOccurrences,
) -> f32 {
    if set_a.identifier_to_count.len() > set_b.identifier_to_count.len() {
        std::mem::swap(&mut set_a, &mut set_b);
    }

    let mut numerator = 0;
    let mut denominator_a = 0;
    let mut used_count_b = 0;
    for (symbol, count_a) in set_a.identifier_to_count.iter() {
        let count_b = set_b.identifier_to_count.get(symbol).unwrap_or(&0);
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
    mut set_a: &'a IdentifierOccurrences,
    mut set_b: &'a IdentifierOccurrences,
) -> f32 {
    if set_a.identifier_to_count.len() > set_b.identifier_to_count.len() {
        std::mem::swap(&mut set_a, &mut set_b);
    }

    let mut numerator = 0;
    for (symbol, count_a) in set_a.identifier_to_count.iter() {
        let count_b = set_b.identifier_to_count.get(symbol).unwrap_or(&0);
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
        let set_a = IdentifierOccurrences::within_string(
            "let mut outline_items = query_outline_items(&language, &tree, &source);",
        );
        // 14 identifier parts, 11 unique
        // Repeats: 2 "outline", 2 "language", 2 "tree"
        let set_b = IdentifierOccurrences::within_string(
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
