use crate::text_similarity::occurrences::HashFrom;
use std::hash::{Hash, Hasher as _};

/// Occurrence source which splits the input into alphanumeric characters, and further splits these
/// when cases change to handle PascalCase and camelCase.
#[derive(Debug)]
pub struct IdentifierParts;

impl IdentifierParts {
    pub fn within_string(text: &str) -> impl Iterator<Item = HashFrom<Self>> {
        IdentifierPartsIterator::new(text).map(|part| fx_hash_ascii_lowercase(part).into())
    }

    pub fn within_strings<'a>(
        strings: impl IntoIterator<Item = &'a str>,
    ) -> impl Iterator<Item = HashFrom<Self>> {
        strings.into_iter().flat_map(|text| {
            IdentifierPartsIterator::new(text).map(|part| fx_hash_ascii_lowercase(part).into())
        })
    }
}

fn fx_hash_ascii_lowercase(text: &str) -> u32 {
    // Hash lowercased text without allocating. May be possible to do this more efficiently by using
    // bit manipulation to lowercase and hash 4 bytes at a time (or 8 bytes at a time with
    // FxHasher64).
    let mut hasher = collections::FxHasher::default();
    for ch in text.chars() {
        ch.to_ascii_lowercase().hash(&mut hasher);
    }
    // TODO: Ideally should directly compute a u32 hash.
    hasher.finish() as u32
}

/// Splits alphanumeric runs on camelCase, PascalCase, snake_case, and kebab-case.
struct IdentifierPartsIterator<'a> {
    text: &'a str,
    chars: std::str::CharIndices<'a>,
    start: Option<usize>,
    prev_char_is_alphanumeric: bool,
    prev_char_is_uppercase: bool,
}

impl<'a> IdentifierPartsIterator<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            chars: text.char_indices(),
            start: None,
            prev_char_is_alphanumeric: false,
            prev_char_is_uppercase: false,
        }
    }
}

impl<'a> Iterator for IdentifierPartsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((byte_index, ch)) = self.chars.next() {
            let is_alphanumeric = ch.is_alphanumeric();

            if !is_alphanumeric {
                if let Some(start) = self.start {
                    if byte_index > start {
                        let part = &self.text[start..byte_index];
                        self.start = None;
                        return Some(part);
                    }
                }
                self.start = None;
                continue;
            }

            // camelCase and PascalCase
            let is_uppercase = ch.is_uppercase();
            let case_split_start = if is_uppercase && let Some(start) = self.start {
                let should_split = if self.prev_char_is_alphanumeric && !self.prev_char_is_uppercase
                {
                    true
                } else if self.prev_char_is_uppercase {
                    // sequences like "XMLParser" -> ["XML", "Parser"]
                    self.text[byte_index..]
                        .chars()
                        .nth(1)
                        .map_or(false, |c| c.is_ascii_lowercase())
                } else {
                    false
                };

                if should_split { Some(start) } else { None }
            } else {
                None
            };

            if let Some(start) = case_split_start {
                let part = &self.text[start..byte_index];
                self.start = Some(byte_index);
                self.prev_char_is_alphanumeric = is_alphanumeric;
                self.prev_char_is_uppercase = is_uppercase;
                return Some(part);
            } else if self.start.is_none() && is_alphanumeric {
                self.start = Some(byte_index);
                self.prev_char_is_alphanumeric = is_alphanumeric;
                self.prev_char_is_uppercase = is_uppercase;
            }
        }

        if let Some(start) = self.start
            && start < self.text.len()
        {
            self.start = None;
            return Some(&self.text[start..]);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Similarity as _, WeightedSimilarity as _,
        text_similarity::occurrences::{Occurrences, SmallOccurrences},
    };

    use super::*;

    #[test]
    fn test_split_identifier() {
        fn identifier_parts<'a>(text: &'a str) -> Vec<&'a str> {
            IdentifierPartsIterator::new(text).collect()
        }

        assert_eq!(
            identifier_parts("snake_case kebab-case PascalCase camelCase XMLParser"),
            vec![
                "snake", "case", "kebab", "case", "Pascal", "Case", "camel", "Case", "XML",
                "Parser"
            ]
        );
        assert_eq!(identifier_parts("snake_case"), vec!["snake", "case"]);
        assert_eq!(identifier_parts("kebab-case"), vec!["kebab", "case"]);
        assert_eq!(identifier_parts("PascalCase"), vec!["Pascal", "Case"]);
        assert_eq!(identifier_parts("camelCase"), vec!["camel", "Case"]);
        assert_eq!(identifier_parts("XMLParser"), vec!["XML", "Parser"]);
        assert_eq!(identifier_parts(""), Vec::<&str>::new());
        assert_eq!(identifier_parts("a"), vec!["a"]);
        assert_eq!(identifier_parts("ABC"), vec!["ABC"]);
        assert_eq!(identifier_parts("abc"), vec!["abc"]);
        assert_eq!(identifier_parts("123"), vec!["123"]);
        assert_eq!(identifier_parts("a1B2c3"), vec!["a1", "B2c3"]);
        assert_eq!(identifier_parts("HTML5Parser"), vec!["HTML5", "Parser"]);
        assert_eq!(
            identifier_parts("_leading_underscore"),
            vec!["leading", "underscore"]
        );
        assert_eq!(
            identifier_parts("trailing_underscore_"),
            vec!["trailing", "underscore"]
        );
        assert_eq!(
            identifier_parts("--multiple--delimiters--"),
            vec!["multiple", "delimiters"]
        );
    }

    #[test]
    fn test_similarity_functions() {
        // 10 identifier parts, 8 unique
        // Repeats: 2 "outline", 2 "items"
        let multiset_a = Occurrences::new(IdentifierParts::within_string(
            "let mut outline_items = query_outline_items(&language, &tree, &source);",
        ));
        let set_a = SmallOccurrences::<8, IdentifierParts>::new(IdentifierParts::within_string(
            "let mut outline_items = query_outline_items(&language, &tree, &source);",
        ));
        // 14 identifier parts, 11 unique
        // Repeats: 2 "outline", 2 "language", 2 "tree"
        let set_b = Occurrences::new(IdentifierParts::within_string(
            "pub fn query_outline_items(language: &Language, tree: &Tree, source: &str) -> Vec<OutlineItem> {",
        ));

        // 6 overlaps: "outline", "items", "query", "language", "tree", "source"
        // 7 non-overlaps: "let", "mut", "pub", "fn", "vec", "item", "str"
        assert_eq!(multiset_a.jaccard_similarity(&set_b), 6.0 / (6.0 + 7.0));
        assert_eq!(set_a.jaccard_similarity(&set_b), 6.0 / (6.0 + 7.0));

        // Numerator is one more than before due to both having 2 "outline".
        // Denominator is the same except for 3 more due to the non-overlapping duplicates
        assert_eq!(
            multiset_a.weighted_jaccard_similarity(&set_b),
            7.0 / (7.0 + 7.0 + 3.0)
        );

        // Numerator is the same as jaccard_similarity. Denominator is the size of the smaller set, 8.
        assert_eq!(multiset_a.overlap_coefficient(&set_b), 6.0 / 8.0);
        assert_eq!(set_a.overlap_coefficient(&set_b), 6.0 / 8.0);

        // Numerator is the same as weighted_jaccard_similarity. Denominator is the total weight of
        // the smaller set, 10.
        assert_eq!(multiset_a.weighted_overlap_coefficient(&set_b), 7.0 / 10.0);
    }
}
