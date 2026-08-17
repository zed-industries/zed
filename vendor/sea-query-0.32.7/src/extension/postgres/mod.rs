pub use expr::*;
pub use extension::*;
pub use func::*;
pub use ltree::*;
pub use select::*;
pub use types::*;

use crate::types::BinOper;

pub(crate) mod expr;
pub(crate) mod extension;
pub(crate) mod func;
pub(crate) mod interval;
pub(crate) mod ltree;
pub(crate) mod select;
pub(crate) mod types;

/// Postgres-specific binary operators.
///
/// For all supported operators (including the standard ones), see [`BinOper`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgBinOper {
    ILike,
    NotILike,
    Matches,
    Contains,
    Contained,
    Concatenate,
    Overlap,
    Similarity,
    WordSimilarity,
    StrictWordSimilarity,
    SimilarityDistance,
    WordSimilarityDistance,
    StrictWordSimilarityDistance,
    /// `->`. Retrieves JSON field as JSON value.
    GetJsonField,
    /// `->>`. Retrieves JSON field and casts it to an appropriate SQL type.
    CastJsonField,
    /// `~` Regex operator.
    Regex,
    /// `~*`. Regex operator with case insensitive matching.
    RegexCaseInsensitive,
    #[cfg(feature = "postgres-vector")]
    EuclideanDistance,
    #[cfg(feature = "postgres-vector")]
    NegativeInnerProduct,
    #[cfg(feature = "postgres-vector")]
    CosineDistance,
}

impl From<PgBinOper> for BinOper {
    fn from(o: PgBinOper) -> Self {
        Self::PgOperator(o)
    }
}
