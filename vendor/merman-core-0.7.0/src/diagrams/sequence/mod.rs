lalrpop_util::lalrpop_mod!(
    #[allow(
        clippy::empty_line_after_outer_attr,
        clippy::type_complexity,
        clippy::result_large_err
    )]
    sequence_grammar,
    "/diagrams/sequence_grammar.rs"
);

// Mermaid 11.15.x sequence diagram constants (SequenceDB.LINETYPE / PLACEMENT).
const LINETYPE_NOTE: i32 = 2;
const LINETYPE_LOOP_START: i32 = 10;
const LINETYPE_LOOP_END: i32 = 11;
const LINETYPE_ALT_START: i32 = 12;
const LINETYPE_ALT_ELSE: i32 = 13;
const LINETYPE_ALT_END: i32 = 14;
const LINETYPE_OPT_START: i32 = 15;
const LINETYPE_OPT_END: i32 = 16;
const LINETYPE_ACTIVE_START: i32 = 17;
const LINETYPE_ACTIVE_END: i32 = 18;
const LINETYPE_PAR_START: i32 = 19;
const LINETYPE_PAR_AND: i32 = 20;
const LINETYPE_PAR_END: i32 = 21;
const LINETYPE_RECT_START: i32 = 22;
const LINETYPE_RECT_END: i32 = 23;
const LINETYPE_AUTONUMBER: i32 = 26;
const LINETYPE_CRITICAL_START: i32 = 27;
const LINETYPE_CRITICAL_OPTION: i32 = 28;
const LINETYPE_CRITICAL_END: i32 = 29;
const LINETYPE_BREAK_START: i32 = 30;
const LINETYPE_BREAK_END: i32 = 31;
const LINETYPE_PAR_OVER_START: i32 = 32;
const LINETYPE_CENTRAL_CONNECTION: i32 = 59;
const LINETYPE_CENTRAL_CONNECTION_REVERSE: i32 = 60;
const LINETYPE_CENTRAL_CONNECTION_DUAL: i32 = 61;

const PLACEMENT_LEFT_OF: i32 = 0;
const PLACEMENT_RIGHT_OF: i32 = 1;
const PLACEMENT_OVER: i32 = 2;

mod ast;
mod db;
mod lexer;
mod parse;
mod render_model;

pub(crate) use ast::Action;
pub(crate) use lexer::{LexError, Tok};

pub use parse::{parse_sequence, parse_sequence_model_for_render};
pub use render_model::{
    SequenceActor, SequenceAutonumber, SequenceBox, SequenceDiagramRenderModel, SequenceMessage,
    SequenceMessagePayload, SequenceNote,
};
