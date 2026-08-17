use alloc::string::String;

use pp_rs::{
    pp::Preprocessor,
    token::{PreprocessorError, Punct, TokenValue as PPTokenValue},
};

use super::{
    ast::Precision,
    token::{Directive, DirectiveKind, Token, TokenValue},
    types::parse_type,
};
use crate::{FastHashMap, Span, StorageAccess};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct LexerResult {
    pub kind: LexerResultKind,
    pub meta: Span,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LexerResultKind {
    Token(Token),
    Directive(Directive),
    Error(PreprocessorError),
}

pub struct Lexer<'a> {
    pp: Preprocessor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, defines: &'a FastHashMap<String, String>) -> Self {
        let mut pp = Preprocessor::new(input);
        for (define, value) in defines {
            pp.add_define(define, value).unwrap(); //TODO: handle error
        }
        Lexer { pp }
    }
}

impl Iterator for Lexer<'_> {
    type Item = LexerResult;
    fn next(&mut self) -> Option<Self::Item> {
        let pp_token = match self.pp.next()? {
            Ok(t) => t,
            Err((err, loc)) => {
                return Some(LexerResult {
                    kind: LexerResultKind::Error(err),
                    meta: loc.into(),
                });
            }
        };

        let meta = pp_token.location.into();
        let value = match pp_token.value {
            PPTokenValue::Extension(extension) => {
                return Some(LexerResult {
                    kind: LexerResultKind::Directive(Directive {
                        kind: DirectiveKind::Extension,
                        tokens: extension.tokens,
                    }),
                    meta,
                })
            }
            PPTokenValue::Float(float) => TokenValue::FloatConstant(float),
            PPTokenValue::Ident(ident) => {
                match ident.as_str() {
                    // Qualifiers
                    "layout" => TokenValue::Layout,
                    "in" => TokenValue::In,
                    "out" => TokenValue::Out,
                    "uniform" => TokenValue::Uniform,
                    "buffer" => TokenValue::Buffer,
                    "shared" => TokenValue::Shared,
                    "invariant" => TokenValue::Invariant,
                    "flat" => TokenValue::Interpolation(crate::Interpolation::Flat),
                    "noperspective" => TokenValue::Interpolation(crate::Interpolation::Linear),
                    "smooth" => TokenValue::Interpolation(crate::Interpolation::Perspective),
                    "centroid" => TokenValue::Sampling(crate::Sampling::Centroid),
                    "sample" => TokenValue::Sampling(crate::Sampling::Sample),
                    "const" => TokenValue::Const,
                    "inout" => TokenValue::InOut,
                    "precision" => TokenValue::Precision,
                    "highp" => TokenValue::PrecisionQualifier(Precision::High),
                    "mediump" => TokenValue::PrecisionQualifier(Precision::Medium),
                    "lowp" => TokenValue::PrecisionQualifier(Precision::Low),
                    "restrict" => TokenValue::Restrict,
                    "readonly" => TokenValue::MemoryQualifier(StorageAccess::LOAD),
                    "writeonly" => TokenValue::MemoryQualifier(StorageAccess::STORE),
                    // values
                    "true" => TokenValue::BoolConstant(true),
                    "false" => TokenValue::BoolConstant(false),
                    // jump statements
                    "continue" => TokenValue::Continue,
                    "break" => TokenValue::Break,
                    "return" => TokenValue::Return,
                    "discard" => TokenValue::Discard,
                    // selection statements
                    "if" => TokenValue::If,
                    "else" => TokenValue::Else,
                    "switch" => TokenValue::Switch,
                    "case" => TokenValue::Case,
                    "default" => TokenValue::Default,
                    // iteration statements
                    "while" => TokenValue::While,
                    "do" => TokenValue::Do,
                    "for" => TokenValue::For,
                    // types
                    "void" => TokenValue::Void,
                    "struct" => TokenValue::Struct,
                    word => match parse_type(word) {
                        Some(t) => TokenValue::TypeName(t),
                        None => TokenValue::Identifier(String::from(word)),
                    },
                }
            }
            PPTokenValue::Integer(integer) => TokenValue::IntConstant(integer),
            PPTokenValue::Punct(punct) => match punct {
                // Compound assignments
                Punct::AddAssign => TokenValue::AddAssign,
                Punct::SubAssign => TokenValue::SubAssign,
                Punct::MulAssign => TokenValue::MulAssign,
                Punct::DivAssign => TokenValue::DivAssign,
                Punct::ModAssign => TokenValue::ModAssign,
                Punct::LeftShiftAssign => TokenValue::LeftShiftAssign,
                Punct::RightShiftAssign => TokenValue::RightShiftAssign,
                Punct::AndAssign => TokenValue::AndAssign,
                Punct::XorAssign => TokenValue::XorAssign,
                Punct::OrAssign => TokenValue::OrAssign,

                // Two character punctuation
                Punct::Increment => TokenValue::Increment,
                Punct::Decrement => TokenValue::Decrement,
                Punct::LogicalAnd => TokenValue::LogicalAnd,
                Punct::LogicalOr => TokenValue::LogicalOr,
                Punct::LogicalXor => TokenValue::LogicalXor,
                Punct::LessEqual => TokenValue::LessEqual,
                Punct::GreaterEqual => TokenValue::GreaterEqual,
                Punct::EqualEqual => TokenValue::Equal,
                Punct::NotEqual => TokenValue::NotEqual,
                Punct::LeftShift => TokenValue::LeftShift,
                Punct::RightShift => TokenValue::RightShift,

                // Parenthesis or similar
                Punct::LeftBrace => TokenValue::LeftBrace,
                Punct::RightBrace => TokenValue::RightBrace,
                Punct::LeftParen => TokenValue::LeftParen,
                Punct::RightParen => TokenValue::RightParen,
                Punct::LeftBracket => TokenValue::LeftBracket,
                Punct::RightBracket => TokenValue::RightBracket,

                // Other one character punctuation
                Punct::LeftAngle => TokenValue::LeftAngle,
                Punct::RightAngle => TokenValue::RightAngle,
                Punct::Semicolon => TokenValue::Semicolon,
                Punct::Comma => TokenValue::Comma,
                Punct::Colon => TokenValue::Colon,
                Punct::Dot => TokenValue::Dot,
                Punct::Equal => TokenValue::Assign,
                Punct::Bang => TokenValue::Bang,
                Punct::Minus => TokenValue::Dash,
                Punct::Tilde => TokenValue::Tilde,
                Punct::Plus => TokenValue::Plus,
                Punct::Star => TokenValue::Star,
                Punct::Slash => TokenValue::Slash,
                Punct::Percent => TokenValue::Percent,
                Punct::Pipe => TokenValue::VerticalBar,
                Punct::Caret => TokenValue::Caret,
                Punct::Ampersand => TokenValue::Ampersand,
                Punct::Question => TokenValue::Question,
            },
            PPTokenValue::Pragma(pragma) => {
                return Some(LexerResult {
                    kind: LexerResultKind::Directive(Directive {
                        kind: DirectiveKind::Pragma,
                        tokens: pragma.tokens,
                    }),
                    meta,
                })
            }
            PPTokenValue::Version(version) => {
                return Some(LexerResult {
                    kind: LexerResultKind::Directive(Directive {
                        kind: DirectiveKind::Version {
                            is_first_directive: version.is_first_directive,
                        },
                        tokens: version.tokens,
                    }),
                    meta,
                })
            }
        };

        Some(LexerResult {
            kind: LexerResultKind::Token(Token { value, meta }),
            meta,
        })
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use pp_rs::token::{Integer, Location, Token as PPToken, TokenValue as PPTokenValue};

    use super::{
        super::token::{Directive, DirectiveKind, Token, TokenValue},
        Lexer, LexerResult, LexerResultKind,
    };
    use crate::Span;

    #[test]
    fn lex_tokens() {
        let defines = crate::FastHashMap::default();

        // line comments
        let mut lex = Lexer::new("#version 450\nvoid main () {}", &defines);
        let mut location = Location::default();
        location.start = 9;
        location.end = 12;
        assert_eq!(
            lex.next().unwrap(),
            LexerResult {
                kind: LexerResultKind::Directive(Directive {
                    kind: DirectiveKind::Version {
                        is_first_directive: true
                    },
                    tokens: vec![PPToken {
                        value: PPTokenValue::Integer(Integer {
                            signed: true,
                            value: 450,
                            width: 32
                        }),
                        location
                    }]
                }),
                meta: Span::new(1, 8)
            }
        );
        assert_eq!(
            lex.next().unwrap(),
            LexerResult {
                kind: LexerResultKind::Token(Token {
                    value: TokenValue::Void,
                    meta: Span::new(13, 17)
                }),
                meta: Span::new(13, 17)
            }
        );
        assert_eq!(
            lex.next().unwrap(),
            LexerResult {
                kind: LexerResultKind::Token(Token {
                    value: TokenValue::Identifier("main".into()),
                    meta: Span::new(18, 22)
                }),
                meta: Span::new(18, 22)
            }
        );
        assert_eq!(
            lex.next().unwrap(),
            LexerResult {
                kind: LexerResultKind::Token(Token {
                    value: TokenValue::LeftParen,
                    meta: Span::new(23, 24)
                }),
                meta: Span::new(23, 24)
            }
        );
        assert_eq!(
            lex.next().unwrap(),
            LexerResult {
                kind: LexerResultKind::Token(Token {
                    value: TokenValue::RightParen,
                    meta: Span::new(24, 25)
                }),
                meta: Span::new(24, 25)
            }
        );
        assert_eq!(
            lex.next().unwrap(),
            LexerResult {
                kind: LexerResultKind::Token(Token {
                    value: TokenValue::LeftBrace,
                    meta: Span::new(26, 27)
                }),
                meta: Span::new(26, 27)
            }
        );
        assert_eq!(
            lex.next().unwrap(),
            LexerResult {
                kind: LexerResultKind::Token(Token {
                    value: TokenValue::RightBrace,
                    meta: Span::new(27, 28)
                }),
                meta: Span::new(27, 28)
            }
        );
        assert_eq!(lex.next(), None);
    }
}
