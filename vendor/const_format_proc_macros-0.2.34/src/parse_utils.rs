use crate::{spanned::Spans, utils::Peekable2, Error};

use proc_macro2::{
    token_stream::IntoIter, Delimiter, Group, Ident, Punct, Span, TokenStream as TokenStream2,
    TokenTree as TokenTree2,
};

use std::{cmp::PartialEq, ops::Range};

pub type ParseStream<'a> = &'a mut ParseBuffer;

pub struct ParseBuffer {
    iter: Peekable2<IntoIter>,
}

impl ParseBuffer {
    pub fn new(ts: TokenStream2) -> Self {
        let iter = Peekable2::new(ts);
        Self { iter }
    }

    pub fn is_empty(&mut self) -> bool {
        self.iter.is_empty()
    }

    pub fn peek(&mut self) -> Option<&TokenTree2> {
        self.iter.peek()
    }
    pub fn peek2(&mut self) -> Option<&TokenTree2> {
        self.iter.peek2()
    }

    pub fn parse_punct(&mut self, c: char) -> Result<Punct, crate::Error> {
        match self.next() {
            Some(TokenTree2::Punct(x)) if x.as_char() == c => Ok(x),
            Some(x) => Err(Error::new(x.span(), &format!("Expected a '{}' token", c))),
            None => Err(Error::new(
                Span::mixed_site(),
                &format!("Expected a '{}' token", c),
            )),
        }
    }
    pub fn parse_opt_punct(&mut self, c: char) -> Result<Option<Punct>, crate::Error> {
        match self.next() {
            Some(TokenTree2::Punct(x)) if x.as_char() == c => Ok(Some(x)),
            Some(x) => Err(Error::new(x.span(), &format!("Expected a '{}' token", c))),
            None => Ok(None),
        }
    }

    pub fn parse_ident(&mut self) -> Result<Ident, crate::Error> {
        match self.next() {
            Some(TokenTree2::Ident(x)) => Ok(x),
            Some(x) => Err(Error::new(x.span(), "Expected an identifier")),
            None => Err(Error::new(Span::mixed_site(), "Expected an identifier")),
        }
    }

    pub fn parse_paren(&mut self) -> Result<Parentheses, crate::Error> {
        match self.next() {
            Some(TokenTree2::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                Ok(Parentheses {
                    paren_span: group.span(),
                    contents: group.stream(),
                })
            }
            Some(x) => Err(Error::new(
                x.span(),
                &format!("Expected parentheses: found {}", x),
            )),
            None => Err(Error::new(
                Span::mixed_site(),
                "Expected parentheses, found nothing",
            )),
        }
    }

    pub fn parse_unwrap_paren<F, T>(&mut self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce(ParseStream<'_>) -> Result<T, crate::Error>,
    {
        if matches!(self.peek(), Some(TokenTree2::Group(x)) if x.delimiter() == Delimiter::Parenthesis )
        {
            if let Some(TokenTree2::Group(group)) = self.next() {
                ParseBuffer::new(group.stream()).parse_unwrap_tt(f)
            } else {
                unreachable!("But I peeked for a Parenthesis delimited TokenTree::Group!!")
            }
        } else {
            f(self)
        }
    }

    pub fn parse_unwrap_group<F, T>(&mut self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce(ParseStream<'_>) -> Result<T, crate::Error>,
    {
        if let Some(TokenTree2::Group(group)) = self.next() {
            ParseBuffer::new(group.stream()).parse_unwrap_tt(f)
        } else {
            f(self)
        }
    }

    pub fn parse_token_stream_and_span(&mut self) -> (TokenStream2, Spans) {
        let mut start = match self.peek() {
            Some(x) => x.span(),
            None => Span::call_site(),
        };

        let mut end = start;

        let ts = self
            .inspect(|tt| {
                end = tt.span();
                if let Some(next) = start.join(end) {
                    start = next;
                }
            })
            .collect::<TokenStream2>();

        (ts, Spans { start, end })
    }

    /// Unwraps a none-delimited token tree to parse a type,
    /// if the first token is not a none-delimited token tree it parses the type in
    /// the passed in ParseStream.
    pub fn parse_unwrap_tt<F, T>(&mut self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce(ParseStream<'_>) -> Result<T, crate::Error>,
    {
        if matches!(self.peek(), Some(TokenTree2::Group(x)) if x.delimiter() == Delimiter::None ) {
            if let Some(TokenTree2::Group(group)) = self.next() {
                ParseBuffer::new(group.stream()).parse_unwrap_tt(f)
            } else {
                unreachable!("But I peeked for a None delimited TokenTree::Group!!")
            }
        } else {
            f(self)
        }
    }
}

impl Iterator for ParseBuffer {
    type Item = TokenTree2;

    fn next(&mut self) -> Option<TokenTree2> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct Parentheses {
    #[allow(dead_code)]
    pub paren_span: Span,
    pub contents: TokenStream2,
}

///////////////////////////////////////////////////////////////////////////////

pub struct LitStr {
    value: String,
    pub rawness: StrRawness,
    pub inside_lit: Range<usize>,
    pub span: Span,
}

impl LitStr {
    pub fn value(&self) -> &str {
        &self.value[self.inside_lit.clone()]
    }
    pub(crate) fn parse_from_literal(literal: &proc_macro2::Literal) -> Result<Self, Error> {
        let mut value = literal.to_string();
        // Ignoring the quote characters
        let mut range = 1..value.len() - 1;
        let span = literal.span();

        let is_raw = if let Some(suffix) = value.strip_prefix('r') {
            let hashes = suffix.bytes().take_while(|x| *x == b'#').count();

            if value.as_bytes()[1 + hashes] != b'"' {
                return Err(Error::new(
                    span,
                    &format!("Expected a string literal, found: {}", literal),
                ));
            }

            // Ignoring the r and hashes
            range.start += 1 + hashes;
            range.end -= hashes;
            Some(hashes as u32)
        } else {
            let mut matches = value.match_indices(r#"\u"#).peekable();
            if matches.peek().is_some() {
                let mut prev_end = 0;
                let mut new = String::with_capacity(value.len());

                for (pos, _) in matches {
                    new.push_str(&value[prev_end..pos]);

                    let past_open = pos + 3;

                    let off_close = value[pos..].find('}').unwrap();

                    let c = &value[past_open..pos + off_close];
                    let c = u32::from_str_radix(c, 16).unwrap();
                    let c = std::char::from_u32(c).unwrap();

                    // if matches!(c, '\\' | '"') {
                    //     new.push('\\');
                    // }
                    new.push(c);

                    prev_end = pos + off_close + 1;
                }
                new.push_str(&value[prev_end..]);
                value = new;
            }

            range = 1..value.len() - 1;

            None
        };

        Ok(Self {
            value,
            rawness: StrRawness { is_raw, span },
            inside_lit: range,
            span,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StrRawness {
    is_raw: Option<u32>,
    span: Span,
}

impl PartialEq for StrRawness {
    fn eq(&self, other: &Self) -> bool {
        self.is_raw == other.is_raw
    }
}

impl StrRawness {
    #[cfg(test)]
    pub fn dummy() -> Self {
        Self {
            is_raw: Some(4),
            span: Span::mixed_site(),
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    /// Tokenizes a slice of the parsed string literal.
    pub fn tokenize_sub(&self, str: &str) -> TokenStream2 {
        let mut buffer = String::new();
        match self.is_raw {
            Some(hashes) => {
                let hashes = hashes as usize;
                buffer.reserve(3 + hashes + str.len() + hashes);
                buffer.push('r');
                let hashes = (0..hashes).map(|_| '#');
                buffer.extend(hashes.clone());
                buffer.push('"');
                buffer.push_str(str);
                buffer.push('"');
                buffer.extend(hashes);
            }
            None => {
                buffer.reserve(2 + str.len());
                buffer.push('"');
                buffer.push_str(str);
                buffer.push('"');
            }
        }

        buffer
            .parse::<TokenStream2>()
            .unwrap()
            .set_span_recursive(self.span)
    }
}

///////////////////////////////////////////////////////////////////////////////

pub trait TokenTreeExt: Sized {
    fn as_token_tree(&self) -> &TokenTree2;
    fn into_token_tree(self) -> TokenTree2;

    fn is_punct(&self, c: char) -> bool {
        matches!(self.as_token_tree(), TokenTree2::Punct(p)  if p.as_char() == c)
    }

    #[allow(dead_code)]
    fn is_paren(&self) -> bool {
        matches!(
            self.as_token_tree(),
            TokenTree2::Group(g) if g.delimiter() == Delimiter::Parenthesis
        )
    }

    #[allow(dead_code)]
    fn is_ident(&self, ident: &str) -> bool {
        matches!(self.as_token_tree(), TokenTree2::Ident(x)  if x == ident)
    }

    fn set_span_recursive(self, span: Span) -> TokenTree2 {
        let mut tt = self.into_token_tree();

        tt.set_span(span);
        if let TokenTree2::Group(group) = tt {
            let delim = group.delimiter();
            let stream = group.stream().set_span_recursive(span);
            tt = TokenTree2::Group(Group::new(delim, stream));
        }
        tt.set_span(span);
        tt
    }
}

impl TokenTreeExt for TokenTree2 {
    fn as_token_tree(&self) -> &TokenTree2 {
        self
    }

    fn into_token_tree(self) -> TokenTree2 {
        self
    }
}

///////////////////////////////////////////////////////////////////////////////

pub trait TokenStream2Ext: Sized {
    fn into_token_stream(self) -> TokenStream2;

    fn set_span_recursive(self, span: Span) -> TokenStream2 {
        self.into_token_stream()
            .into_iter()
            .map(|tt| tt.set_span_recursive(span))
            .collect()
    }
}

impl TokenStream2Ext for TokenStream2 {
    fn into_token_stream(self) -> TokenStream2 {
        self
    }
}

///////////////////////////////////////////////////////////////////////////////

pub trait MyParse: Sized {
    fn parse(input: ParseStream<'_>) -> Result<Self, crate::Error>;

    fn parse_token_stream_1(input: proc_macro::TokenStream) -> Result<Self, crate::Error> {
        Self::parse(&mut ParseBuffer::new(TokenStream2::from(input)))
    }

    #[allow(dead_code)]
    fn parse_token_stream_2(input: TokenStream2) -> Result<Self, crate::Error> {
        Self::parse(&mut ParseBuffer::new(input))
    }
}

///////////////////////////////////////////////////////////////////////////////
