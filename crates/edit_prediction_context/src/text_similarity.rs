mod identifier_parts;
mod occurrences;

pub use identifier_parts::IdentifierParts;
pub use occurrences::{OccurrencesMultiset, Similarity, WeightedSimilarity};

// Variants to consider experimenting:
//
// * Also include unsplit identifier (or a hash of its hashes), so that full identifier matches get
// a higher score.
//
// * Inclusion of both unmodified and lowercased identifier parts, so that case matches get a higher
// score.
//
//     - If this is implemented then SmallOccurrenceSet::from_hashes should do some eager deduping based
//     on last added.
