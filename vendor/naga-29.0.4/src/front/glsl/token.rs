pub use pp_rs::token::{Float, Integer, Location, Token as PPToken};

use alloc::{string::String, vec::Vec};

use super::ast::Precision;
use crate::{Interpolation, Sampling, Span, Type};

impl From<Location> for Span {
    fn from(loc: Location) -> Self {
        Span::new(loc.start, loc.end)
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Token {
    pub value: TokenValue,
    pub meta: Span,
}

/// A token passed from the lexing used in the parsing.
///
/// This type is exported since it's returned in the
/// [`InvalidToken`](super::ErrorKind::InvalidToken) error.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    Identifier(String),

    FloatConstant(Float),
    IntConstant(Integer),
    BoolConstant(bool),

    Layout,
    In,
    Out,
    InOut,
    Uniform,
    Buffer,
    Const,
    Shared,

    Restrict,
    /// A `glsl` memory qualifier such as `writeonly`
    ///
    /// The associated [`crate::StorageAccess`] is the access being allowed
    /// (for example `writeonly` has an associated value of [`crate::StorageAccess::STORE`])
    MemoryQualifier(crate::StorageAccess),

    Invariant,
    Interpolation(Interpolation),
    Sampling(Sampling),
    Precision,
    PrecisionQualifier(Precision),

    Continue,
    Break,
    Return,
    Discard,

    If,
    Else,
    Switch,
    Case,
    Default,
    While,
    Do,
    For,

    Void,
    Struct,
    TypeName(Type),

    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    LeftShiftAssign,
    RightShiftAssign,
    AndAssign,
    XorAssign,
    OrAssign,

    Increment,
    Decrement,

    LogicalOr,
    LogicalAnd,
    LogicalXor,

    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,

    LeftShift,
    RightShift,

    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,

    Comma,
    Semicolon,
    Colon,
    Dot,
    Bang,
    Dash,
    Tilde,
    Plus,
    Star,
    Slash,
    Percent,
    VerticalBar,
    Caret,
    Ampersand,
    Question,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Directive {
    pub kind: DirectiveKind,
    pub tokens: Vec<PPToken>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DirectiveKind {
    Version { is_first_directive: bool },
    Extension,
    Pragma,
}
