mod occurrences;
mod sliding_window;
mod source;

pub use occurrences::*;
pub use sliding_window::*;
pub use source::*;

// Variants to consider trying:
//
// * Also include unsplit identifier (or a hash of its hashes), so that full identifier matches get
// a higher score.
//
// * Inclusion of both unmodified and lowercased identifier parts, so that case matches get a higher
// score.
//
//     - If this is implemented then SmallOccurrenceSet::from_hashes should do some eager deduping based
//     on last added.
