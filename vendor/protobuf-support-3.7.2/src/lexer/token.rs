use crate::lexer::json_number_lit::JsonNumberLit;
use crate::lexer::lexer_impl::LexerError;
use crate::lexer::lexer_impl::LexerResult;
use crate::lexer::loc::Loc;
use crate::lexer::num_lit::NumLit;
use crate::lexer::str_lit::StrLit;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Symbol(char),
    // Protobuf tokenizer has separate tokens for int and float.
    // Tokens do not include sign.
    IntLit(u64),
    FloatLit(f64),
    JsonNumber(JsonNumberLit),
    // including quotes
    StrLit(StrLit),
}

impl Token {
    /// Back to original
    pub fn format(&self) -> String {
        match self {
            &Token::Ident(ref s) => s.clone(),
            &Token::Symbol(c) => c.to_string(),
            &Token::IntLit(ref i) => i.to_string(),
            &Token::StrLit(ref s) => s.quoted(),
            &Token::FloatLit(ref f) => f.to_string(),
            &Token::JsonNumber(ref f) => f.to_string(),
        }
    }

    pub fn to_num_lit(&self) -> LexerResult<NumLit> {
        match self {
            &Token::IntLit(i) => Ok(NumLit::U64(i)),
            &Token::FloatLit(f) => Ok(NumLit::F64(f)),
            _ => Err(LexerError::IncorrectInput),
        }
    }
}

#[derive(Clone)]
pub struct TokenWithLocation {
    pub token: Token,
    pub loc: Loc,
}
