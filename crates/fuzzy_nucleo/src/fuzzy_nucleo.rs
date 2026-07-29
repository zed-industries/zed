mod matcher;
mod paths;
mod strings;

use std::borrow::Cow;

use fuzzy::CharBag;
use nucleo::Utf32Str;
use nucleo::pattern::{Atom, AtomKind, CaseMatching, Normalization, Pattern};
use unicode_normalization::{IsNormalized, UnicodeNormalization, is_nfc_quick};
use unicode_segmentation::UnicodeSegmentation;

pub use paths::{
    PathMatch, PathMatchCandidate, PathMatchCandidateSet, match_fixed_path_set, match_path_sets,
};
pub use strings::{StringMatch, StringMatchCandidate, match_strings, match_strings_async};

pub(crate) struct Cancelled;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Case {
    Smart,
    Ignore,
}

impl Case {
    pub fn smart_if_uppercase_in(query: &str) -> Self {
        if query.chars().any(|c| c.is_uppercase()) {
            Self::Smart
        } else {
            Self::Ignore
        }
    }

    pub fn is_smart(self) -> bool {
        matches!(self, Self::Smart)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LengthPenalty {
    On,
    Off,
}

impl LengthPenalty {
    pub fn from_bool(on: bool) -> Self {
        if on { Self::On } else { Self::Off }
    }

    pub fn is_on(self) -> bool {
        matches!(self, Self::On)
    }
}

// Matching is always case-insensitive at the nucleo level — using
// `CaseMatching::Smart` there would *reject* candidates whose capitalization
// doesn't match the query, breaking pickers like the command palette
// (`"Editor: Backspace"` against the action named `"editor: backspace"`).
// `Case::Smart` is honored as a *scoring hint* instead: when the query
// contains uppercase, candidates whose matched characters disagree in case
// are downranked by a per-mismatch penalty rather than dropped.
pub(crate) struct Query {
    pub(crate) pattern: Pattern,
    /// Non-whitespace query chars in input order, populated only when a smart-case
    /// penalty will actually be charged. Aligns 1:1 with the indices appended by
    /// `Pattern::indices` (atom-order, needle-order within each atom).
    pub(crate) query_chars: Option<Vec<char>>,
    pub(crate) char_bag: CharBag,
}

impl Query {
    pub(crate) fn build(query: &str, case: Case) -> Option<Self> {
        if query.chars().all(char::is_whitespace) {
            return None;
        }
        let normalized = query.split_whitespace().collect::<Vec<_>>().join(" ");
        let grapheme_atoms = pattern_grapheme_atoms(&normalized);
        let mut pattern = Pattern::default();
        // Nucleo 0.3.1 retains the escape before a space in non-ASCII atoms, so construct
        // atoms from the already-unescaped graphemes to keep the needle and metadata aligned.
        pattern.atoms.extend(grapheme_atoms.iter().map(|graphemes| {
            Atom::new(
                &graphemes.concat(),
                CaseMatching::Ignore,
                Normalization::Smart,
                AtomKind::Fuzzy,
                false,
            )
        }));
        let wants_case_penalty = case.is_smart() && query.chars().any(|c| c.is_uppercase());
        let query_chars = wants_case_penalty.then(|| {
            grapheme_atoms
                .into_iter()
                .flatten()
                .filter_map(|grapheme| grapheme.chars().next())
                .collect()
        });
        Some(Query {
            pattern,
            query_chars,
            char_bag: CharBag::from(query),
        })
    }
}

pub(crate) fn pattern_grapheme_atoms(pattern: &str) -> Vec<Vec<&str>> {
    let mut saw_backslash = false;
    pattern
        .split(move |character| {
            saw_backslash = match character {
                ' ' if !saw_backslash => return true,
                '\\' => true,
                _ => false,
            };
            false
        })
        .filter(|atom| !atom.is_empty())
        .map(|atom| {
            let graphemes = atom.graphemes(true).collect::<Vec<_>>();
            let mut result = Vec::with_capacity(graphemes.len());
            let mut index = 0;
            while index < graphemes.len() {
                if graphemes[index] == "\\" && graphemes.get(index + 1) == Some(&" ") {
                    index += 1;
                }
                result.push(graphemes[index]);
                index += 1;
            }
            result
        })
        .collect()
}

#[inline]
pub(crate) fn count_case_mismatches(
    query_chars: Option<&[char]>,
    matched_chars: &[u32],
    candidate: &str,
    candidate_chars: &mut Vec<char>,
) -> u32 {
    let Some(query_chars) = query_chars else {
        return 0;
    };
    if query_chars.len() != matched_chars.len() {
        return 0;
    }
    candidate_chars.clear();
    candidate_chars.extend(nucleo::chars::graphemes(candidate));
    let mut mismatches: u32 = 0;
    for (&query_char, &pos) in query_chars.iter().zip(matched_chars) {
        if let Some(&candidate_char) = candidate_chars.get(pos as usize)
            && candidate_char != query_char
            && candidate_char.eq_ignore_ascii_case(&query_char)
        {
            mismatches += 1;
        }
    }
    mismatches
}

const SMART_CASE_PENALTY_PER_MISMATCH: f64 = 0.9;

#[inline]
pub(crate) fn case_penalty(mismatches: u32) -> f64 {
    if mismatches == 0 {
        1.0
    } else {
        SMART_CASE_PENALTY_PER_MISMATCH.powi(mismatches as i32)
    }
}

pub(crate) fn normalize_nfc(s: &str) -> Cow<'_, str> {
    match is_nfc_quick(s.chars()) {
        IsNormalized::Yes => Cow::Borrowed(s),
        IsNormalized::No => Cow::Owned(s.nfc().collect()),
        IsNormalized::Maybe => {
            let normalized = s.nfc().collect::<String>();
            if normalized == s {
                Cow::Borrowed(s)
            } else {
                Cow::Owned(normalized)
            }
        }
    }
}

pub(crate) fn utf32_str<'a>(s: &'a str, buffer: &'a mut Vec<char>) -> Utf32Str<'a> {
    if s.is_ascii() {
        Utf32Str::Ascii(s.as_bytes())
    } else {
        // Nucleo 0.3.1 can classify decomposed non-ASCII text as Ascii when every grapheme starts
        // with ASCII, so construct the Unicode representation explicitly.
        buffer.clear();
        buffer.extend(nucleo::chars::graphemes(s));
        Utf32Str::Unicode(buffer)
    }
}

/// Reconstruct original UTF-8 byte positions from sorted, deduplicated Nucleo
/// indices. Nucleo indices represent extended grapheme clusters rather than Unicode scalars.
pub(crate) fn positions_from_sorted(s: &str, sorted_grapheme_indices: &[u32]) -> Vec<usize> {
    if s.is_ascii() {
        return sorted_grapheme_indices
            .iter()
            .filter_map(|&index| usize::try_from(index).ok())
            .filter(|&index| index < s.len())
            .collect();
    }

    let mut matched_indices = sorted_grapheme_indices.iter().copied().peekable();
    let mut positions = Vec::with_capacity(sorted_grapheme_indices.len());
    for (grapheme_index, (grapheme_start, grapheme)) in s.grapheme_indices(true).enumerate() {
        if matched_indices.peek().is_none() {
            break;
        }
        if matched_indices
            .next_if(|&matched_index| matched_index == grapheme_index as u32)
            .is_some()
        {
            positions.extend(
                grapheme
                    .char_indices()
                    .map(|(offset, _)| grapheme_start + offset),
            );
        }
    }
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_ascii_text_uses_unicode_utf32_representation() {
        let mut buffer = Vec::new();

        assert!(matches!(
            utf32_str("o\u{308}", &mut buffer),
            Utf32Str::Unicode(_)
        ));
    }

    #[test]
    fn maps_grapheme_indices_to_complete_original_graphemes() {
        assert_eq!(positions_from_sorted("a\u{308}b", &[0]), vec![0, 1]);
        assert_eq!(
            positions_from_sorted("1\u{fe0f}\u{20e3}2", &[0]),
            vec![0, 1, 4]
        );
        assert_eq!(positions_from_sorted("a\u{308}b", &[1]), vec![3]);
    }

    #[test]
    fn nfc_normalization_borrows_normalized_text_and_owns_changed_text() {
        assert!(matches!(normalize_nfc("grössen"), Cow::Borrowed(_)));
        assert!(matches!(normalize_nfc("gro\u{308}ssen"), Cow::Owned(_)));
    }

    #[test]
    fn query_graphemes_unescape_spaces_in_non_ascii_atoms() {
        assert_eq!(
            pattern_grapheme_atoms("grö\\ file other"),
            vec![
                vec!["g", "r", "ö", " ", "f", "i", "l", "e"],
                vec!["o", "t", "h", "e", "r"]
            ]
        );
        assert_eq!(
            pattern_grapheme_atoms("ascii\\ atom"),
            vec![vec!["a", "s", "c", "i", "i", " ", "a", "t", "o", "m"]]
        );
    }
}
