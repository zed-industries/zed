pub use expr::SqliteExpr;

use crate::types::BinOper;

mod expr;

/// SQLite-specific binary operators.
///
/// For all supported operators (including the standard ones), see [`BinOper`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteBinOper {
    /// `GLOB`
    Glob,
    /// `MATCH`.
    Match,
    /// `->`. Retrieves JSON field as JSON value.
    GetJsonField,
    /// `->>`. Retrieves JSON field and casts it to an appropriate SQL type.
    CastJsonField,
}

impl From<SqliteBinOper> for BinOper {
    fn from(o: SqliteBinOper) -> Self {
        Self::SqliteOperator(o)
    }
}
