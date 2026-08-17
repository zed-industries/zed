use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    orig: String,
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ParseErrorKind {
    UnterminatedString,
    UnexpectedChar(char),
    UnexpectedToken {
        expected: &'static str,
        found: &'static str,
    },
    IncompleteExpr(&'static str),
    UnterminatedExpression(String),
    InvalidTarget(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to parse `{}` as a cfg expression: {}",
            self.orig, self.kind
        )
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseErrorKind::*;
        match self {
            UnterminatedString => write!(f, "unterminated string in cfg"),
            UnexpectedChar(ch) => write!(
                f,
                "unexpected character `{}` in cfg, expected parens, a comma, \
                 an identifier, or a string",
                ch
            ),
            UnexpectedToken { expected, found } => {
                write!(f, "expected {}, found {}", expected, found)
            }
            IncompleteExpr(expected) => {
                write!(f, "expected {}, but cfg expression ended", expected)
            }
            UnterminatedExpression(s) => {
                write!(f, "unexpected content `{}` found after cfg expression", s)
            }
            InvalidTarget(s) => write!(f, "invalid target specifier: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub fn new(orig: &str, kind: ParseErrorKind) -> ParseError {
        ParseError {
            kind,
            orig: orig.to_string(),
        }
    }
}
