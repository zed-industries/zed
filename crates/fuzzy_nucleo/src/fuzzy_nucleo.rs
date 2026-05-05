mod matcher;
mod paths;
mod strings;

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
