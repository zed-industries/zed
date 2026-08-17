use peg::{Parse, ParseElem, ParseLiteral, ParseSlice, RuleResult};
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

#[derive(Debug, Clone)]
pub struct FlatTokenStream {
    tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub enum Token {
    Ident(Ident),
    Literal(Literal),
    Punct(Punct),
    Begin(Group, usize),
    End(Delimiter, Span),
}

impl Token {
    fn span(&self) -> Span {
        match self {
            Token::Ident(i) => i.span(),
            Token::Literal(l) => l.span(),
            Token::Punct(p) => p.span(),
            Token::Begin(g, _) => g.span(),
            Token::End(_, span) => span.clone(),
        }
    }
}

impl FlatTokenStream {
    pub fn new(stream: TokenStream) -> FlatTokenStream {
        let mut tokens = vec![];

        fn flatten(tokens: &mut Vec<Token>, tree: TokenTree) {
            match tree {
                TokenTree::Ident(i) => tokens.push(Token::Ident(i)),
                TokenTree::Literal(l) => tokens.push(Token::Literal(l)),
                TokenTree::Punct(p) => tokens.push(Token::Punct(p)),
                TokenTree::Group(g) => {
                    let start_pos = tokens.len();

                    tokens.push(Token::End(g.delimiter(), g.span())); // placeholder
                    for tree in g.stream() {
                        flatten(tokens, tree);
                    }
                    tokens.push(Token::End(g.delimiter(), g.span()));

                    let end_pos = tokens.len();
                    tokens[start_pos] = Token::Begin(g, end_pos);
                }
            }
        }

        for tree in stream {
            flatten(&mut tokens, tree);
        }

        FlatTokenStream { tokens }
    }

    pub fn next_span(&self, pos: usize) -> RuleResult<Span> {
        match self.tokens.get(pos) {
            Some(t) => RuleResult::Matched(pos, t.span()),
            _ => RuleResult::Failed,
        }
    }

    pub fn ident(&self, pos: usize) -> RuleResult<Ident> {
        match self.tokens.get(pos) {
            Some(Token::Ident(i)) => RuleResult::Matched(pos + 1, i.clone()),
            _ => RuleResult::Failed,
        }
    }

    pub fn literal(&self, pos: usize) -> RuleResult<Literal> {
        match self.tokens.get(pos) {
            Some(Token::Literal(i)) => RuleResult::Matched(pos + 1, i.clone()),
            _ => RuleResult::Failed,
        }
    }

    pub fn group(&self, pos: usize, delim: Delimiter) -> RuleResult<Group> {
        match self.tokens.get(pos) {
            Some(Token::Begin(g, n)) if g.delimiter() == delim => {
                RuleResult::Matched(*n, g.clone())
            }
            _ => RuleResult::Failed,
        }
    }

    pub fn eat_until(&self, initial_pos: usize, end: char) -> RuleResult<()> {
        let mut pos = initial_pos;
        loop {
            match self.tokens.get(pos) {
                Some(Token::Begin(_, n)) => pos = *n,
                Some(Token::Ident(_)) | Some(Token::Literal(_)) => pos += 1,
                Some(Token::Punct(p)) if p.as_char() != end => pos += 1,
                _ if pos != initial_pos => return RuleResult::Matched(pos, ()),
                _ => return RuleResult::Failed,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sp(pub Span, pub usize);

impl ::std::fmt::Display for Sp {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(fmt, "{:?} ({})", self.0, self.1)
    }
}

impl Parse for FlatTokenStream {
    type PositionRepr = Sp;
    fn start(&self) -> usize {
        0
    }

    fn is_eof(&self, pos: usize) -> bool {
        pos >= self.tokens.len()
    }

    fn position_repr(&self, pos: usize) -> Sp {
        let span = self.tokens.get(pos)
            .map_or_else(
                || Span::call_site(),
                |t| t.span()
            );
        Sp(span, pos)
    }
}

impl<'input> ParseElem<'input> for FlatTokenStream {
    type Element = &'input Token;

    fn parse_elem(&'input self, pos: usize) -> RuleResult<&'input Token> {
        match self.tokens.get(pos) {
            Some(c) => RuleResult::Matched(pos + 1, c),
            None => RuleResult::Failed,
        }
    }
}

fn delimiter_start(d: Delimiter) -> &'static str {
    match d {
        Delimiter::Brace => "{",
        Delimiter::Bracket => "[",
        Delimiter::Parenthesis => "(",
        _ => "",
    }
}

fn delimiter_end(d: Delimiter) -> &'static str {
    match d {
        Delimiter::Brace => "}",
        Delimiter::Bracket => "]",
        Delimiter::Parenthesis => ")",
        _ => "",
    }
}

impl ParseLiteral for FlatTokenStream {
    fn parse_string_literal(&self, pos: usize, literal: &str) -> RuleResult<()> {
        match self.tokens.get(pos) {
            Some(Token::Ident(i)) if i.to_string() == literal => RuleResult::Matched(pos + 1, ()),
            Some(Token::Punct(p)) if literal.starts_with(p.as_char()) => {
                if literal.len() == 1 {
                    RuleResult::Matched(pos + 1, ())
                } else if p.spacing() == Spacing::Joint {
                    self.parse_string_literal(pos + 1, &literal[1..])
                } else {
                    RuleResult::Failed
                }
            }
            Some(Token::Begin(g, _)) if delimiter_start(g.delimiter()) == literal => {
                RuleResult::Matched(pos + 1, ())
            }
            Some(Token::End(d, _)) if delimiter_end(*d) == literal => {
                RuleResult::Matched(pos + 1, ())
            }
            _ => RuleResult::Failed,
        }
    }
}

impl<'input> ParseSlice<'input> for FlatTokenStream {
    type Slice = TokenStream;
    fn parse_slice(&'input self, p1: usize, p2: usize) -> TokenStream {
        let mut ts = TokenStream::new();
        let mut pos = p1;

        while pos < p2 {
            let (t, next_pos): (TokenTree, usize) = match &self.tokens[pos] {
                Token::Ident(i) => (i.clone().into(), pos + 1),
                Token::Literal(l) => (l.clone().into(), pos + 1),
                Token::Punct(p) => (p.clone().into(), pos + 1),
                Token::Begin(g, end) => (g.clone().into(), *end),
                Token::End(..) => panic!("$-expr containing unmatched group end"),
            };
            ts.extend(Some(t));
            pos = next_pos;
        }

        assert_eq!(pos, p2, "$-expr containing unmatched group start");

        ts
    }
}
