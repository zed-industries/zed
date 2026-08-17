use crate::{Error, Result};
use proc_macro2::{Delimiter, Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use std::iter::Peekable;

pub(crate) fn parse_input(
    input: TokenStream,
) -> Result<(Vec<Attribute>, Vec<TokenTree>, TokenTree)> {
    let mut input = input.into_iter().peekable();
    let mut attrs = Vec::new();

    while let Some(attr) = parse_next_attr(&mut input)? {
        attrs.push(attr);
    }

    let sig = parse_signature(&mut input);
    let body = input.next().ok_or_else(|| {
        Error::new(
            Span::call_site(),
            "`#[proc_macro_error]` can be applied only to functions".to_string(),
        )
    })?;

    Ok((attrs, sig, body))
}

fn parse_next_attr(
    input: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<Option<Attribute>> {
    let shebang = match input.peek() {
        Some(TokenTree::Punct(ref punct)) if punct.as_char() == '#' => input.next().unwrap(),
        _ => return Ok(None),
    };

    let group = match input.peek() {
        Some(TokenTree::Group(ref group)) if group.delimiter() == Delimiter::Bracket => {
            let res = group.clone();
            input.next();
            res
        }
        other => {
            let span = other.map_or(Span::call_site(), |tt| tt.span());
            return Err(Error::new(span, "expected `[`".to_string()));
        }
    };

    let path = match group.stream().into_iter().next() {
        Some(TokenTree::Ident(ident)) => Some(ident),
        _ => None,
    };

    Ok(Some(Attribute {
        shebang,
        group: TokenTree::Group(group),
        path,
    }))
}

fn parse_signature(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Vec<TokenTree> {
    let mut sig = Vec::new();
    loop {
        match input.peek() {
            Some(TokenTree::Group(ref group)) if group.delimiter() == Delimiter::Brace => {
                return sig;
            }
            None => return sig,
            _ => sig.push(input.next().unwrap()),
        }
    }
}

pub(crate) struct Attribute {
    pub(crate) shebang: TokenTree,
    pub(crate) group: TokenTree,
    pub(crate) path: Option<Ident>,
}

impl Attribute {
    pub(crate) fn path_is_ident(&self, ident: &str) -> bool {
        self.path.as_ref().map_or(false, |p| *p == ident)
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, ts: &mut TokenStream) {
        self.shebang.to_tokens(ts);
        self.group.to_tokens(ts);
    }
}
