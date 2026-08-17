use peg::{Parse, ParseElem, RuleResult};

/// The default implementation of the parsing traits for `[T]` expects `T` to be
/// `Copy`, as in the `[u8]` or simple enum cases. This wrapper exposes the
/// elements by `&T` reference, which is `Copy`.
pub struct SliceByRef<'a, T>(pub &'a [T]);

impl<'a , T> Parse for SliceByRef<'a, T> {
    type PositionRepr = usize;
    fn start(&self) -> usize {
        0
    }

    fn is_eof(&self, pos: usize) -> bool {
        pos >= self.0.len()
    }

    fn position_repr(&self, pos: usize) -> usize {
        pos
    }
}

impl<'a, T: 'a> ParseElem<'a> for SliceByRef<'a, T> {
    type Element = &'a T;

    fn parse_elem(&'a self, pos: usize) -> RuleResult<&'a T> {
        match self.0[pos..].first() {
            Some(c) => RuleResult::Matched(pos + 1, c),
            None => RuleResult::Failed,
        }
    }
}

#[derive(PartialEq)]
pub enum TokenType {
    Word,
    Number,
}

pub struct Token {
    pub token_type: TokenType,
    pub term: String,
}

peg::parser!{
    grammar tokenparser<'a>() for SliceByRef<'a, Token> {
        // The [] syntax works just like (and expands into) an arm of a match
        // in regular Rust, so you can use a pattern that matches one field
        // and ignores the rest
        pub rule word_by_field() = [ Token { token_type: TokenType::Word, .. } ]

        // Or capture the token as a variable and then test it with an if guard.
        pub rule word_by_eq() = [t if t.token_type == TokenType::Word]

        // You could wrap this in a rule that accepts the TokenType as an argument
        rule tok(ty: TokenType) -> &'input Token = [t if t.token_type == ty]
        pub rule number() = tok(TokenType::Number)
    }
}

fn main() {
    let word_tok = vec![
        Token { token_type: TokenType::Word, term: "foo".into() }
    ];

    let number_tok = vec![
        Token { token_type: TokenType::Number, term: "123".into() }
    ];

    assert!(tokenparser::word_by_field(&SliceByRef(&word_tok[..])).is_ok());
    assert!(tokenparser::word_by_eq(&SliceByRef(&word_tok[..])).is_ok());
    assert!(tokenparser::number(&SliceByRef(&number_tok[..])).is_ok());
}
