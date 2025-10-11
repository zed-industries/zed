use crate::text_similarity::occurrences::HashFrom;
use std::iter::Peekable;

/// Occurrence source which splits the input into runs of ascii alphanumeric or unicode characters,
/// and further splits these on ascii case transitions (camelCase and PascalCase).
#[derive(Debug)]
pub struct IdentifierParts;

impl IdentifierParts {
    pub fn within_string(text: &str) -> impl Iterator<Item = HashFrom<Self>> {
        HashedIdentifierParts::new(text.bytes())
    }

    pub fn within_strings<'a>(
        strings: impl IntoIterator<Item = &'a str>,
    ) -> impl Iterator<Item = HashFrom<Self>> {
        strings
            .into_iter()
            .flat_map(|text| HashedIdentifierParts::new(text.bytes()))
    }
}

/// Splits alphanumeric runs on camelCase, PascalCase, snake_case, and kebab-case.
struct HashedIdentifierParts<I: Iterator<Item = u8>> {
    str_bytes: Peekable<I>,
    hasher: Option<FxHasher32>,
    prev_char_is_uppercase: bool,
}

impl<I: Iterator<Item = u8>> HashedIdentifierParts<I> {
    fn new(str_bytes: I) -> Self {
        Self {
            str_bytes: str_bytes.peekable(),
            hasher: None,
            prev_char_is_uppercase: false,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for HashedIdentifierParts<I> {
    type Item = HashFrom<IdentifierParts>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.str_bytes.next() {
            let included = !ch.is_ascii() || ch.is_ascii_alphanumeric();
            if let Some(mut hasher) = self.hasher.take() {
                if !included {
                    return Some(hasher.finish().into());
                }

                // camelCase and PascalCase
                let is_uppercase = ch.is_ascii_uppercase();
                let should_split = is_uppercase
                    && (!self.prev_char_is_uppercase ||
                        // sequences like "XMLParser" -> ["XML", "Parser"]
                        self.str_bytes
                            .peek()
                            .map_or(false, |c| c.is_ascii_lowercase()));

                self.prev_char_is_uppercase = is_uppercase;

                if should_split {
                    let result = (hasher.finish() as u32).into();
                    let mut hasher = FxHasher32::default();
                    hasher.write_u8(ch.to_ascii_lowercase());
                    self.hasher = Some(hasher);
                    return Some(result);
                } else {
                    hasher.write_u8(ch.to_ascii_lowercase());
                    self.hasher = Some(hasher);
                }
            } else if included {
                let mut hasher = FxHasher32::default();
                hasher.write_u8(ch.to_ascii_lowercase());
                self.hasher = Some(hasher);
                self.prev_char_is_uppercase = ch.is_ascii_uppercase();
            }
        }

        if let Some(hasher) = self.hasher.take() {
            return Some(hasher.finish().into());
        }

        None
    }
}

/// 32-bit variant of FXHasher
struct FxHasher32(u32);

impl Default for FxHasher32 {
    fn default() -> Self {
        FxHasher32(0)
    }
}

impl FxHasher32 {
    #[inline]
    pub fn write_u8(&mut self, byte: u8) {
        self.0 = self.0.wrapping_add(byte as u32).wrapping_mul(0x93d765dd);
    }

    pub fn finish(self) -> u32 {
        self.0
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
    fn test_identifier_text_parts() {
        #[track_caller]
        fn check_identifier_parts(text: &str, expected: &[&str]) {
            assert_eq!(
                HashedIdentifierParts::new(text.bytes()).collect::<Vec<_>>(),
                expected
                    .iter()
                    .map(|part| fxhash32_ascii_lowercase(part).into())
                    .collect::<Vec<_>>()
            );
        }

        check_identifier_parts("", &[]);
        check_identifier_parts("a", &["a"]);
        check_identifier_parts("ABC", &["ABC"]);
        check_identifier_parts("abc", &["abc"]);
        check_identifier_parts("123", &["123"]);
        check_identifier_parts("snake_case", &["snake", "case"]);
        check_identifier_parts("kebab-case", &["kebab", "case"]);
        check_identifier_parts("PascalCase", &["Pascal", "Case"]);
        check_identifier_parts("camelCase", &["camel", "Case"]);
        check_identifier_parts("XMLParser", &["XML", "Parser"]);
        check_identifier_parts("a1B2c3", &["a1", "B2c3"]);
        check_identifier_parts("HTML5Parser", &["HTML5", "Parser"]);
        check_identifier_parts("_leading_underscore", &["leading", "underscore"]);
        check_identifier_parts("trailing_underscore_", &["trailing", "underscore"]);
        check_identifier_parts("--multiple--delimiters--", &["multiple", "delimiters"]);
        check_identifier_parts(
            "snake_case kebab-case PascalCase camelCase XMLParser",
            &[
                "snake", "case", "kebab", "case", "Pascal", "Case", "camel", "Case", "XML",
                "Parser",
            ],
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

    fn fxhash32_ascii_lowercase(text: &str) -> u32 {
        let mut hasher = FxHasher32::default();
        for byte in text.bytes() {
            hasher.write_u8(byte.to_ascii_lowercase());
        }
        hasher.finish() as u32
    }
}
