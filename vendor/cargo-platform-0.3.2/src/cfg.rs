use crate::error::{ParseError, ParseErrorKind::*};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter;
use std::str::{self, FromStr};

/// A cfg expression.
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Debug)]
pub enum CfgExpr {
    Not(Box<CfgExpr>),
    All(Vec<CfgExpr>),
    Any(Vec<CfgExpr>),
    Value(Cfg),
    True,
    False,
}

/// A cfg value.
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Debug)]
pub enum Cfg {
    /// A named cfg value, like `unix`.
    Name(Ident),
    /// A key/value cfg pair, like `target_os = "linux"`.
    KeyPair(Ident, String),
}

/// A identifier
#[derive(Eq, Ord, PartialOrd, Clone, Debug)]
pub struct Ident {
    /// The identifier
    pub name: String,
    /// Is this a raw ident: `r#async`
    ///
    /// It's mainly used for display and doesn't take
    /// part in the hash or equality (`foo` == `r#foo`).
    pub raw: bool,
}

#[derive(PartialEq)]
enum Token<'a> {
    LeftParen,
    RightParen,
    Ident(bool, &'a str),
    Comma,
    Equals,
    String(&'a str),
}

/// The list of keywords.
///
/// We should consider all the keywords, but some are conditional on
/// the edition so for now we just consider true/false.
///
/// <https://doc.rust-lang.org/reference/keywords.html>
pub(crate) const KEYWORDS: &[&str; 2] = &["true", "false"];

#[derive(Clone)]
struct Tokenizer<'a> {
    s: iter::Peekable<str::CharIndices<'a>>,
    orig: &'a str,
}

struct Parser<'a> {
    t: Tokenizer<'a>,
}

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        self.name == *other
    }
}

impl PartialEq<Ident> for Ident {
    fn eq(&self, other: &Ident) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.raw {
            f.write_str("r#")?;
        }
        f.write_str(&*self.name)
    }
}

impl FromStr for Cfg {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Cfg, Self::Err> {
        let mut p = Parser::new(s);
        let e = p.cfg()?;
        if let Some(rest) = p.rest() {
            return Err(ParseError::new(
                p.t.orig,
                UnterminatedExpression(rest.to_string()),
            ));
        }
        Ok(e)
    }
}

impl fmt::Display for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cfg::Name(ref s) => s.fmt(f),
            Cfg::KeyPair(ref k, ref v) => write!(f, "{} = \"{}\"", k, v),
        }
    }
}

impl CfgExpr {
    /// Utility function to check if the key, "cfg(..)" matches the `target_cfg`
    pub fn matches_key(key: &str, target_cfg: &[Cfg]) -> bool {
        if key.starts_with("cfg(") && key.ends_with(')') {
            let cfg = &key[4..key.len() - 1];

            CfgExpr::from_str(cfg)
                .ok()
                .map(|ce| ce.matches(target_cfg))
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn matches(&self, cfg: &[Cfg]) -> bool {
        match *self {
            CfgExpr::Not(ref e) => !e.matches(cfg),
            CfgExpr::All(ref e) => e.iter().all(|e| e.matches(cfg)),
            CfgExpr::Any(ref e) => e.iter().any(|e| e.matches(cfg)),
            CfgExpr::Value(ref e) => cfg.contains(e),
            CfgExpr::True => true,
            CfgExpr::False => false,
        }
    }
}

impl FromStr for CfgExpr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<CfgExpr, Self::Err> {
        let mut p = Parser::new(s);
        let e = p.expr()?;
        if let Some(rest) = p.rest() {
            return Err(ParseError::new(
                p.t.orig,
                UnterminatedExpression(rest.to_string()),
            ));
        }
        Ok(e)
    }
}

impl fmt::Display for CfgExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CfgExpr::Not(ref e) => write!(f, "not({})", e),
            CfgExpr::All(ref e) => write!(f, "all({})", CommaSep(e)),
            CfgExpr::Any(ref e) => write!(f, "any({})", CommaSep(e)),
            CfgExpr::Value(ref e) => write!(f, "{}", e),
            CfgExpr::True => write!(f, "true"),
            CfgExpr::False => write!(f, "false"),
        }
    }
}

struct CommaSep<'a, T>(&'a [T]);

impl<'a, T: fmt::Display> fmt::Display for CommaSep<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Parser<'a> {
        Parser {
            t: Tokenizer {
                s: s.char_indices().peekable(),
                orig: s,
            },
        }
    }

    fn expr(&mut self) -> Result<CfgExpr, ParseError> {
        match self.peek() {
            Some(Ok(Token::Ident(false, op @ "all")))
            | Some(Ok(Token::Ident(false, op @ "any"))) => {
                self.t.next();
                let mut e = Vec::new();
                self.eat(&Token::LeftParen)?;
                while !self.r#try(&Token::RightParen) {
                    e.push(self.expr()?);
                    if !self.r#try(&Token::Comma) {
                        self.eat(&Token::RightParen)?;
                        break;
                    }
                }
                if op == "all" {
                    Ok(CfgExpr::All(e))
                } else {
                    Ok(CfgExpr::Any(e))
                }
            }
            Some(Ok(Token::Ident(false, "not"))) => {
                self.t.next();
                self.eat(&Token::LeftParen)?;
                let e = self.expr()?;
                self.eat(&Token::RightParen)?;
                Ok(CfgExpr::Not(Box::new(e)))
            }
            Some(Ok(..)) => self.cfg().map(|v| match v {
                Cfg::Name(n) if n == "true" => CfgExpr::True,
                Cfg::Name(n) if n == "false" => CfgExpr::False,
                v => CfgExpr::Value(v),
            }),
            Some(Err(..)) => Err(self.t.next().unwrap().err().unwrap()),
            None => Err(ParseError::new(
                self.t.orig,
                IncompleteExpr("start of a cfg expression"),
            )),
        }
    }

    fn cfg(&mut self) -> Result<Cfg, ParseError> {
        match self.t.next() {
            Some(Ok(Token::Ident(raw, name))) => {
                let e = if self.r#try(&Token::Equals) {
                    let val = match self.t.next() {
                        Some(Ok(Token::String(s))) => s,
                        Some(Ok(t)) => {
                            return Err(ParseError::new(
                                self.t.orig,
                                UnexpectedToken {
                                    expected: "a string",
                                    found: t.classify(),
                                },
                            ));
                        }
                        Some(Err(e)) => return Err(e),
                        None => {
                            return Err(ParseError::new(self.t.orig, IncompleteExpr("a string")));
                        }
                    };
                    Cfg::KeyPair(
                        Ident {
                            name: name.to_string(),
                            raw,
                        },
                        val.to_string(),
                    )
                } else {
                    Cfg::Name(Ident {
                        name: name.to_string(),
                        raw,
                    })
                };
                Ok(e)
            }
            Some(Ok(t)) => Err(ParseError::new(
                self.t.orig,
                UnexpectedToken {
                    expected: "identifier",
                    found: t.classify(),
                },
            )),
            Some(Err(e)) => Err(e),
            None => Err(ParseError::new(self.t.orig, IncompleteExpr("identifier"))),
        }
    }

    fn peek(&mut self) -> Option<Result<Token<'a>, ParseError>> {
        self.t.clone().next()
    }

    fn r#try(&mut self, token: &Token<'a>) -> bool {
        match self.peek() {
            Some(Ok(ref t)) if token == t => {}
            _ => return false,
        }
        self.t.next();
        true
    }

    fn eat(&mut self, token: &Token<'a>) -> Result<(), ParseError> {
        match self.t.next() {
            Some(Ok(ref t)) if token == t => Ok(()),
            Some(Ok(t)) => Err(ParseError::new(
                self.t.orig,
                UnexpectedToken {
                    expected: token.classify(),
                    found: t.classify(),
                },
            )),
            Some(Err(e)) => Err(e),
            None => Err(ParseError::new(
                self.t.orig,
                IncompleteExpr(token.classify()),
            )),
        }
    }

    /// Returns the rest of the input from the current location.
    fn rest(&self) -> Option<&str> {
        let mut s = self.t.s.clone();
        loop {
            match s.next() {
                Some((_, ' ')) => {}
                Some((start, _ch)) => return Some(&self.t.orig[start..]),
                None => return None,
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, ParseError>;

    fn next(&mut self) -> Option<Result<Token<'a>, ParseError>> {
        loop {
            match self.s.next() {
                Some((_, ' ')) => {}
                Some((_, '(')) => return Some(Ok(Token::LeftParen)),
                Some((_, ')')) => return Some(Ok(Token::RightParen)),
                Some((_, ',')) => return Some(Ok(Token::Comma)),
                Some((_, '=')) => return Some(Ok(Token::Equals)),
                Some((start, '"')) => {
                    while let Some((end, ch)) = self.s.next() {
                        if ch == '"' {
                            return Some(Ok(Token::String(&self.orig[start + 1..end])));
                        }
                    }
                    return Some(Err(ParseError::new(self.orig, UnterminatedString)));
                }
                Some((start, ch)) if is_ident_start(ch) => {
                    let (start, raw) = if ch == 'r' {
                        if let Some(&(_pos, '#')) = self.s.peek() {
                            // starts with `r#` is a raw ident
                            self.s.next();
                            if let Some((start, ch)) = self.s.next() {
                                if is_ident_start(ch) {
                                    (start, true)
                                } else {
                                    // not a starting ident character
                                    return Some(Err(ParseError::new(
                                        self.orig,
                                        UnexpectedChar(ch),
                                    )));
                                }
                            } else {
                                // not followed by a ident, error out
                                return Some(Err(ParseError::new(
                                    self.orig,
                                    IncompleteExpr("identifier"),
                                )));
                            }
                        } else {
                            // starts with `r` but not does continue with `#`
                            // cannot be a raw ident
                            (start, false)
                        }
                    } else {
                        // do not start with `r`, cannot be a raw ident
                        (start, false)
                    };
                    while let Some(&(end, ch)) = self.s.peek() {
                        if !is_ident_rest(ch) {
                            return Some(Ok(Token::Ident(raw, &self.orig[start..end])));
                        } else {
                            self.s.next();
                        }
                    }
                    return Some(Ok(Token::Ident(raw, &self.orig[start..])));
                }
                Some((_, ch)) => {
                    return Some(Err(ParseError::new(self.orig, UnexpectedChar(ch))));
                }
                None => return None,
            }
        }
    }
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_rest(ch: char) -> bool {
    is_ident_start(ch) || ch.is_ascii_digit()
}

impl<'a> Token<'a> {
    fn classify(&self) -> &'static str {
        match *self {
            Token::LeftParen => "`(`",
            Token::RightParen => "`)`",
            Token::Ident(..) => "an identifier",
            Token::Comma => "`,`",
            Token::Equals => "`=`",
            Token::String(..) => "a string",
        }
    }
}
