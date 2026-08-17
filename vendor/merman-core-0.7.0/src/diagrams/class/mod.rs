lalrpop_util::lalrpop_mod!(
    #[allow(clippy::empty_line_after_outer_attr)]
    class_grammar,
    "/diagrams/class_grammar.rs"
);

pub(crate) const LINE_SOLID: i32 = 0;
pub(crate) const LINE_DOTTED: i32 = 1;

pub(crate) const REL_AGGREGATION: i32 = 0;
pub(crate) const REL_EXTENSION: i32 = 1;
pub(crate) const REL_COMPOSITION: i32 = 2;
pub(crate) const REL_DEPENDENCY: i32 = 3;
pub(crate) const REL_LOLLIPOP: i32 = 4;
pub(crate) const REL_NONE: i32 = -1;

pub(super) const MERMAID_DOM_ID_PREFIX: &str = "classId-";

mod ast;
mod db;
mod fast;
mod lexer;
mod parse;

#[cfg(test)]
mod tests;

pub use parse::{parse_class, parse_class_typed};

pub(crate) use ast::{Action, Relation, RelationData};
pub(crate) use lexer::{LexError, Tok};
