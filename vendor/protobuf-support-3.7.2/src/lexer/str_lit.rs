use std::fmt;
use std::string::FromUtf8Error;

use crate::lexer::lexer_impl::Lexer;
use crate::lexer::parser_language::ParserLanguage;

#[derive(Debug, thiserror::Error)]
pub enum StrLitDecodeError {
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("String literal decode error")]
    OtherError,
}

pub type StrLitDecodeResult<T> = Result<T, StrLitDecodeError>;

/// String literal, both `string` and `bytes`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StrLit {
    pub escaped: String,
}

impl fmt::Display for StrLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", &self.escaped)
    }
}

impl StrLit {
    /// May fail if not valid UTF8
    pub fn decode_utf8(&self) -> StrLitDecodeResult<String> {
        let mut lexer = Lexer::new(&self.escaped, ParserLanguage::Json);
        let mut r = Vec::new();
        while !lexer.eof() {
            r.extend(
                lexer
                    .next_str_lit_bytes()
                    .map_err(|_| StrLitDecodeError::OtherError)?
                    .bytes(),
            );
        }
        Ok(String::from_utf8(r)?)
    }

    pub fn decode_bytes(&self) -> StrLitDecodeResult<Vec<u8>> {
        let mut lexer = Lexer::new(&self.escaped, ParserLanguage::Json);
        let mut r = Vec::new();
        while !lexer.eof() {
            r.extend(
                lexer
                    .next_str_lit_bytes()
                    .map_err(|_| StrLitDecodeError::OtherError)?
                    .bytes(),
            );
        }
        Ok(r)
    }

    pub fn quoted(&self) -> String {
        format!("\"{}\"", self.escaped)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::str_lit::StrLit;

    #[test]
    fn decode_utf8() {
        assert_eq!(
            "\u{1234}".to_owned(),
            StrLit {
                escaped: "\\341\\210\\264".to_owned()
            }
            .decode_utf8()
            .unwrap()
        )
    }
}
