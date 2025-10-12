use crate::text_similarity::occurrences::HashFrom;
use arraydeque::ArrayDeque;
use std::{borrow::Cow, iter::Peekable, marker::PhantomData, path::Path};
use util::rel_path::RelPath;

pub trait OccurrenceSource {
    fn occurrences_in_utf8_bytes(
        str_bytes: impl IntoIterator<Item = u8>,
    ) -> impl Iterator<Item = HashFrom<Self>>;

    fn occurrences_in_str(str: &str) -> impl Iterator<Item = HashFrom<Self>> {
        Self::occurrences_in_utf8_bytes(str.bytes())
    }

    /// Includes worktree name and omits file extension.
    fn occurrences_in_worktree_path(
        worktree_name: Option<Cow<'_, str>>,
        rel_path: &RelPath,
    ) -> impl Iterator<Item = HashFrom<Self>> {
        if let Some(worktree_name) = worktree_name {
            itertools::Either::Left(
                Self::occurrences_in_utf8_bytes(cow_str_into_bytes(worktree_name))
                    .chain(Self::occurrences_in_path(rel_path.as_std_path())),
            )
        } else {
            itertools::Either::Right(Self::occurrences_in_path(rel_path.as_std_path()))
        }
    }

    /// Occurrences from a path, the omitting file extension. Note that this does not split on
    /// components (they are omitted by `IdentifierParts` but not `CodeParts`).
    fn occurrences_in_path(path: &Path) -> impl Iterator<Item = HashFrom<Self>> {
        let path_bytes = path.as_os_str().as_encoded_bytes();
        let bytes = if let Some(extension) = path.extension() {
            &path_bytes[0..path_bytes.len() - extension.as_encoded_bytes().len()]
        } else {
            path_bytes
        };
        Self::occurrences_in_utf8_bytes(bytes.iter().cloned())
    }
}

/// Occurrences source for finding relevant code by matching parts of identifiers.
///
/// * Splits the input into runs of ascii alphanumeric or unicode characters
/// * Splits these on ascii case transitions, handling camelCase and PascalCase
/// * Lowercases each part
#[derive(Debug)]
pub struct IdentifierParts;

/// Occurrences source for finding similar code, by including full identifiers and sequences of
/// symbols.
///
/// * Splits the input on ascii whitespace
/// * Splits these into runs of ascii punctuation or alphanumeric/unicode characters
///
/// Due to common use in identifiers, `_` and `-` are not treated as punctuation. This is consistent
/// with not splitting on case transitions.
pub struct CodeParts;

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

impl OccurrenceSource for IdentifierParts {
    fn occurrences_in_utf8_bytes(
        str_bytes: impl IntoIterator<Item = u8>,
    ) -> impl Iterator<Item = HashFrom<Self>> {
        HashedIdentifierParts::new(str_bytes.into_iter())
    }
}

impl OccurrenceSource for CodeParts {
    fn occurrences_in_utf8_bytes(
        str_bytes: impl IntoIterator<Item = u8>,
    ) -> impl Iterator<Item = HashFrom<Self>> {
        HashedCodeParts::new(str_bytes.into_iter())
    }
}

impl<const N: usize, S: OccurrenceSource> OccurrenceSource for NGram<N, S> {
    fn occurrences_in_utf8_bytes(
        str_bytes: impl IntoIterator<Item = u8>,
    ) -> impl Iterator<Item = HashFrom<NGram<N, S>>> {
        NGramIterator {
            hashes: S::occurrences_in_utf8_bytes(str_bytes),
            window: ArrayDeque::new(),
            _source: PhantomData,
        }
    }
}

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

struct HashedCodeParts<I: Iterator<Item = u8>> {
    str_bytes: Peekable<I>,
    // TODO: Since this doesn't do lowercasing, it might be more efficient to find str slices and
    // hash those, instead of hashing a byte at a time. This would be a bit complex with chunked
    // input, though.
    hasher: Option<FxHasher32>,
    prev_char_is_punctuation: bool,
}

impl<I: Iterator<Item = u8>> HashedCodeParts<I> {
    fn new(str_bytes: I) -> Self {
        Self {
            str_bytes: str_bytes.peekable(),
            hasher: None,
            prev_char_is_punctuation: false,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for HashedCodeParts<I> {
    type Item = HashFrom<CodeParts>;

    fn next(&mut self) -> Option<Self::Item> {
        fn is_punctuation(ch: u8) -> bool {
            ch.is_ascii_punctuation() && ch != b'_' && ch != b'-'
        }

        while let Some(ch) = self.str_bytes.next() {
            let included = !ch.is_ascii() || !ch.is_ascii_whitespace();
            if let Some(mut hasher) = self.hasher.take() {
                if !included {
                    return Some(hasher.finish().into());
                }

                let is_punctuation = is_punctuation(ch);
                let should_split = is_punctuation != self.prev_char_is_punctuation;
                self.prev_char_is_punctuation = is_punctuation;

                if should_split {
                    let result = (hasher.finish() as u32).into();
                    let mut hasher = FxHasher32::default();
                    hasher.write_u8(ch);
                    self.hasher = Some(hasher);
                    return Some(result);
                } else {
                    hasher.write_u8(ch);
                    self.hasher = Some(hasher);
                }
            } else if included {
                let mut hasher = FxHasher32::default();
                hasher.write_u8(ch);
                self.hasher = Some(hasher);
                self.prev_char_is_punctuation = is_punctuation(ch);
            }
        }

        if let Some(hasher) = self.hasher.take() {
            return Some(hasher.finish().into());
        }

        None
    }
}

struct NGramIterator<const N: usize, S, I> {
    hashes: I,
    window: ArrayDeque<u32, N, arraydeque::Wrapping>,
    _source: PhantomData<S>,
}

impl<const N: usize, S, I> Iterator for NGramIterator<N, S, I>
where
    I: Iterator<Item = HashFrom<S>>,
{
    type Item = HashFrom<NGram<N, S>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(hash) = self.hashes.next() {
            if self.window.push_back(hash.into()).is_some() {
                let mut hasher = FxHasher32::default();
                for hash in &self.window {
                    hasher.write_u32(*hash);
                }
                return Some(hasher.finish().into());
            }
        }
        return None;
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
    pub fn write_u8(&mut self, value: u8) {
        self.write_u32(value as u32);
    }

    #[inline]
    pub fn write_u32(&mut self, value: u32) {
        self.0 = self.0.wrapping_add(value).wrapping_mul(0x93d765dd);
    }

    pub fn finish(self) -> u32 {
        self.0
    }
}

/// Converts a `Cow<'_, str>` into an iterator of bytes.
fn cow_str_into_bytes(text: Cow<'_, str>) -> impl Iterator<Item = u8> {
    match text {
        Cow::Borrowed(text) => itertools::Either::Left(text.bytes()),
        Cow::Owned(text) => itertools::Either::Right(text.into_bytes().into_iter()),
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
    fn test_identifier_parts() {
        #[track_caller]
        fn check(text: &str, expected: &[&str]) {
            assert_eq!(
                IdentifierParts::occurrences_in_str(text).collect::<Vec<_>>(),
                expected
                    .iter()
                    .map(|part| string_fxhash32(part).into())
                    .collect::<Vec<_>>()
            );
        }

        check("", &[]);
        check("a", &["a"]);
        check("abc", &["abc"]);
        check("ABC", &["abc"]);
        check("123", &["123"]);
        check("snake_case", &["snake", "case"]);
        check("kebab-case", &["kebab", "case"]);
        check("PascalCase", &["pascal", "case"]);
        check("camelCase", &["camel", "case"]);
        check("XMLParser", &["xml", "parser"]);
        check("a1B2c3", &["a1", "b2c3"]);
        check("HTML5Parser", &["html5", "parser"]);
        check("_leading_underscore", &["leading", "underscore"]);
        check("trailing_underscore_", &["trailing", "underscore"]);
        check("--multiple--delimiters--", &["multiple", "delimiters"]);
        check(
            "snake_case kebab-case PascalCase camelCase XMLParser",
            &[
                "snake", "case", "kebab", "case", "pascal", "case", "camel", "case", "xml",
                "parser",
            ],
        );
    }

    #[test]
    fn test_code_parts() {
        #[track_caller]
        fn check(text: &str, expected: &[&str]) {
            assert_eq!(
                CodeParts::occurrences_in_str(text).collect::<Vec<_>>(),
                expected
                    .iter()
                    .map(|part| string_fxhash32(part).into())
                    .collect::<Vec<_>>()
            );
        }

        check("", &[]);
        check("a", &["a"]);
        check("ABC", &["ABC"]);
        check("ABC", &["ABC"]);
        check(
            "pub fn write_u8(&mut self, byte: u8) {",
            &[
                "pub", "fn", "write_u8", "(&", "mut", "self", ",", "byte", ":", "u8", ")", "{",
            ],
        );
        check(
            "snake_case kebab-case PascalCase camelCase XMLParser _leading_underscore --multiple--delimiters--",
            &[
                "snake_case",
                "kebab-case",
                "PascalCase",
                "camelCase",
                "XMLParser",
                "_leading_underscore",
                "--multiple--delimiters--",
            ],
        );
    }

    #[test]
    fn test_similarity_functions() {
        // 10 identifier parts, 8 unique
        // Repeats: 2 "outline", 2 "items"
        let multiset_a = Occurrences::new(IdentifierParts::occurrences_in_str(
            "let mut outline_items = query_outline_items(&language, &tree, &source);",
        ));
        let set_a =
            SmallOccurrences::<8, IdentifierParts>::new(IdentifierParts::occurrences_in_str(
                "let mut outline_items = query_outline_items(&language, &tree, &source);",
            ));
        // 14 identifier parts, 11 unique
        // Repeats: 2 "outline", 2 "language", 2 "tree"
        let set_b = Occurrences::new(IdentifierParts::occurrences_in_str(
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

    fn string_fxhash32(text: &str) -> u32 {
        let mut hasher = FxHasher32::default();
        for byte in text.bytes() {
            hasher.write_u8(byte);
        }
        hasher.finish() as u32
    }
}
