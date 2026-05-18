mod matcher;
mod paths;
mod strings;

use fuzzy::CharBag;
use nucleo::pattern::{AtomKind, CaseMatching, Normalization, Pattern};

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
        let pattern = Pattern::new(
            &normalized,
            CaseMatching::Ignore,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );
        let wants_case_penalty = case.is_smart() && query.chars().any(|c| c.is_uppercase());
        let query_chars =
            wants_case_penalty.then(|| query.chars().filter(|c| !c.is_whitespace()).collect());
        Some(Query {
            pattern,
            query_chars,
            char_bag: CharBag::from(query),
        })
    }
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
    candidate_chars.extend(candidate.chars());
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

/// Reconstruct byte-offset match positions from a list of matched char offsets
/// that is already sorted ascending and deduplicated.
pub(crate) fn positions_from_sorted(s: &str, sorted_char_indices: &[u32]) -> Vec<usize> {
    let mut iter = sorted_char_indices.iter().copied().peekable();
    let mut out = Vec::with_capacity(sorted_char_indices.len());
    for (char_offset, (byte_offset, _)) in s.char_indices().enumerate() {
        if iter.peek().is_none() {
            break;
        }
        if iter.next_if(|&m| m == char_offset as u32).is_some() {
            out.push(byte_offset);
        }
    }
    out
}
