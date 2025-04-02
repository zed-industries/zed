mod char_bag;
mod matcher;
mod paths;
mod strings;

pub use char_bag::CharBag;
pub use paths::{
    PathMatch, PathMatchCandidate, PathMatchCandidateSet, match_fixed_path_set, match_path_sets,
};
pub use strings::{StringMatch, StringMatchCandidate, match_strings};
