mod matcher;
mod paths;
mod strings;
pub use paths::{
    PathMatch, PathMatchCandidate, PathMatchCandidateSet, match_fixed_path_set, match_path_sets,
};
pub use strings::{StringMatch, StringMatchCandidate, match_strings};

pub(crate) struct Cancelled;
