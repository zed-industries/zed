mod char_bag;
mod matcher;
mod paths;
mod strings;

pub use char_bag::CharBag;
#[cfg(not(target_family = "wasm"))]
pub use paths::match_path_sets;
pub use paths::{PathMatch, PathMatchCandidate, PathMatchCandidateSet, match_fixed_path_set};
#[cfg(not(target_family = "wasm"))]
pub use strings::match_strings;
pub use strings::{StringMatch, StringMatchCandidate};
