pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;

use super::*;

/// Sqlite query builder.
#[derive(Default, Debug)]
pub struct SqliteQueryBuilder;

const QUOTE: Quote = Quote(b'"', b'"');

impl GenericBuilder for SqliteQueryBuilder {}

impl SchemaBuilder for SqliteQueryBuilder {}

impl QuotedBuilder for SqliteQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for SqliteQueryBuilder {
    fn escape_string(&self, string: &str) -> String {
        string.replace('\'', "''")
    }

    fn unescape_string(&self, string: &str) -> String {
        string.replace("''", "'")
    }
}

impl TableRefBuilder for SqliteQueryBuilder {}

impl PrecedenceDecider for SqliteQueryBuilder {
    fn inner_expr_well_known_greater_precedence(
        &self,
        inner: &SimpleExpr,
        outer_oper: &Oper,
    ) -> bool {
        common_inner_expr_well_known_greater_precedence(inner, outer_oper)
    }
}

impl OperLeftAssocDecider for SqliteQueryBuilder {
    fn well_known_left_associative(&self, op: &BinOper) -> bool {
        common_well_known_left_associative(op)
    }
}
