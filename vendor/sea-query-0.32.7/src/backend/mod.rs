//! Translating the SQL AST into engine-specific SQL statements.

use crate::*;

#[cfg(feature = "backend-mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-mysql")))]
mod mysql;
#[cfg(feature = "backend-postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-postgres")))]
mod postgres;
#[cfg(feature = "backend-sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend-sqlite")))]
mod sqlite;

#[cfg(feature = "backend-mysql")]
pub use mysql::*;
#[cfg(feature = "backend-postgres")]
pub use postgres::*;
#[cfg(feature = "backend-sqlite")]
pub use sqlite::*;

mod foreign_key_builder;
mod index_builder;
mod query_builder;
mod table_builder;
mod table_ref_builder;

pub use self::foreign_key_builder::*;
pub use self::index_builder::*;
pub use self::query_builder::*;
pub use self::table_builder::*;
pub use self::table_ref_builder::*;

pub trait GenericBuilder: QueryBuilder + SchemaBuilder {}

pub trait SchemaBuilder: TableBuilder + IndexBuilder + ForeignKeyBuilder {}

pub trait QuotedBuilder {
    /// The type of quote the builder uses.
    fn quote(&self) -> Quote;
}

pub trait EscapeBuilder {
    /// Escape a SQL string literal
    fn escape_string(&self, string: &str) -> String {
        string
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\'', "\\'")
            .replace('\0', "\\0")
            .replace('\x08', "\\b")
            .replace('\x09', "\\t")
            .replace('\x1a', "\\z")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    }

    /// Unescape a SQL string literal
    fn unescape_string(&self, string: &str) -> String {
        let mut escape = false;
        let mut output = String::new();
        for c in string.chars() {
            if !escape && c == '\\' {
                escape = true;
            } else if escape {
                write!(
                    output,
                    "{}",
                    match c {
                        '0' => '\0',
                        'b' => '\x08',
                        't' => '\x09',
                        'z' => '\x1a',
                        'n' => '\n',
                        'r' => '\r',
                        c => c,
                    }
                )
                .unwrap();
                escape = false;
            } else {
                write!(output, "{c}").unwrap();
            }
        }
        output
    }
}

pub trait PrecedenceDecider {
    // This method decides which precedence relations should lead to dropped parentheses.
    // There will be more fine grained precedence relations than the ones represented here,
    // but dropping parentheses due to these relations can be confusing for readers.
    fn inner_expr_well_known_greater_precedence(
        &self,
        inner: &SimpleExpr,
        outer_oper: &Oper,
    ) -> bool;
}

pub trait OperLeftAssocDecider {
    // This method decides if the left associativity of an operator should lead to dropped parentheses.
    // Not all known left associative operators are necessarily included here,
    // as dropping them may in some cases be confusing to readers.
    fn well_known_left_associative(&self, op: &BinOper) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum Oper {
    UnOper(UnOper),
    BinOper(BinOper),
}

impl From<UnOper> for Oper {
    fn from(value: UnOper) -> Self {
        Oper::UnOper(value)
    }
}

impl From<BinOper> for Oper {
    fn from(value: BinOper) -> Self {
        Oper::BinOper(value)
    }
}

impl Oper {
    pub(crate) fn is_logical(&self) -> bool {
        matches!(
            self,
            Oper::UnOper(UnOper::Not) | Oper::BinOper(BinOper::And) | Oper::BinOper(BinOper::Or)
        )
    }

    pub(crate) fn is_between(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::Between) | Oper::BinOper(BinOper::NotBetween)
        )
    }

    pub(crate) fn is_like(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::Like) | Oper::BinOper(BinOper::NotLike)
        )
    }

    pub(crate) fn is_in(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::In) | Oper::BinOper(BinOper::NotIn)
        )
    }

    pub(crate) fn is_is(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::Is) | Oper::BinOper(BinOper::IsNot)
        )
    }

    pub(crate) fn is_shift(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::LShift) | Oper::BinOper(BinOper::RShift)
        )
    }

    pub(crate) fn is_arithmetic(&self) -> bool {
        match self {
            Oper::BinOper(b) => {
                matches!(
                    b,
                    BinOper::Mul | BinOper::Div | BinOper::Mod | BinOper::Add | BinOper::Sub
                )
            }
            _ => false,
        }
    }

    pub(crate) fn is_comparison(&self) -> bool {
        match self {
            Oper::BinOper(b) => {
                matches!(
                    b,
                    BinOper::SmallerThan
                        | BinOper::SmallerThanOrEqual
                        | BinOper::Equal
                        | BinOper::GreaterThanOrEqual
                        | BinOper::GreaterThan
                        | BinOper::NotEqual
                )
            }
            _ => false,
        }
    }
}
