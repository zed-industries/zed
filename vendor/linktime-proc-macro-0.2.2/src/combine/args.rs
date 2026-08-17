use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use proc_macro::{Delimiter, Span, TokenStream, TokenTree};

use super::parse_stream;

pub(super) trait Argument {
    type Arg;
    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>>;
}
pub(super) trait ArgumentIdent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum OutputType {
    Ident,
    String,
    ISize,
    RawString,
}

impl FromStr for OutputType {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ident" => Ok(OutputType::Ident),
            "string" => Ok(OutputType::String),
            "isize" => Ok(OutputType::ISize),
            _ => Err(Cow::Borrowed(
                "Invalid output type (expected ident, string, or isize)",
            )),
        }
    }
}
impl ArgumentIdent for OutputType {}

impl<T: ArgumentIdent + FromStr<Err = Cow<'static, str>>> Argument for T {
    type Arg = T;

    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>> {
        if let TokenTree::Ident(ident) = token {
            Ok(T::from_str(ident.to_string().as_str())?)
        } else {
            Err(Cow::Borrowed("Expected an ident"))
        }
    }
}

impl Argument for TokenStream {
    type Arg = TokenStream;

    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>> {
        match token {
            TokenTree::Group(group) => Ok(group.stream()),
            TokenTree::Literal(literal) => {
                Ok(TokenStream::from_iter([TokenTree::Literal(literal)]))
            }
            TokenTree::Ident(ident) => Ok(TokenStream::from_iter([TokenTree::Ident(ident)])),
            TokenTree::Punct(punct) => Ok(TokenStream::from_iter([TokenTree::Punct(punct)])),
        }
    }
}

impl Argument for Span {
    type Arg = Span;

    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>> {
        match token {
            TokenTree::Group(group) => Ok(group
                .stream()
                .into_iter()
                .next()
                .expect("Expected a non-empty group")
                .span()),
            TokenTree::Literal(literal) => Ok(literal.span()),
            TokenTree::Ident(ident) => Ok(ident.span()),
            TokenTree::Punct(punct) => Ok(punct.span()),
        }
    }
}

impl Argument for TokenTree {
    type Arg = TokenTree;

    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>> {
        Ok(token)
    }
}

impl Argument for String {
    type Arg = String;

    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>> {
        let stream = <TokenStream as Argument>::from_token_tree(token)?;
        let mut s = String::with_capacity(32);
        parse_stream(&mut s, "input", OutputType::String, stream);
        Ok(s)
    }
}

impl Argument for usize {
    type Arg = usize;

    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>> {
        let s = <String as Argument>::from_token_tree(token)?;
        let (radix, s) = radix(&s);
        let s = s.replace('_', "");
        Ok(usize::from_str_radix(&s, radix)
            .map_err(|e| format!("Expected a numeric literal, got `{}`", e))?)
    }
}

impl Argument for isize {
    type Arg = isize;

    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>> {
        let s = <String as Argument>::from_token_tree(token)?;
        if s.strip_prefix('-').is_some() {
            let (radix, s) = radix(&s);
            let s = s.replace('_', "");
            Ok(isize::from_str_radix(&s, radix)
                .map_err(|e| format!("Expected a numeric literal, got `{}`", e))?
                .checked_neg()
                .expect("Expected a valid isize"))
        } else {
            let (radix, s) = radix(&s);
            let s = s.replace('_', "");
            Ok(isize::from_str_radix(&s, radix)
                .map_err(|e| format!("Expected a numeric literal, got `{}`", e))?)
        }
    }
}

fn radix(s: &str) -> (u32, &str) {
    if let Some(s) = s.strip_prefix("0x") {
        (16, s)
    } else if let Some(s) = s.strip_prefix("0o") {
        (8, s)
    } else if let Some(s) = s.strip_prefix("0b") {
        (2, s)
    } else {
        (10, s)
    }
}

#[derive(Default)]
pub(super) struct TranslateString {
    chars: Vec<(char, Option<char>)>,
    len: usize,
}

impl TranslateString {
    fn new(chars: Vec<(char, Option<char>)>) -> Self {
        let len = chars
            .iter()
            .map(|(from, to)| {
                let range = *from..=to.unwrap_or(*from);
                *range.end() as u32 - *range.start() as u32 + 1
            })
            .sum::<u32>() as usize;
        Self { chars, len }
    }

    /// Find the index of the character, including within ranges.
    pub(super) fn find(&self, c: char) -> Option<usize> {
        let mut i = 0;
        for (from, to) in &self.chars {
            let range = *from..=to.unwrap_or(*from);
            let range_len = *range.end() as u32 - *range.start() as u32 + 1;
            if range.contains(&c) {
                return Some(i + (c as usize - *from as usize));
            }
            i += range_len as usize;
        }
        None
    }

    pub(super) fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    pub(super) fn len(&self) -> usize {
        self.len
    }

    pub(super) fn char(&self, index: usize) -> char {
        let mut i = index as u32;
        let mut last_char = ' ';
        for (from, to) in &self.chars {
            let range = *from..=to.unwrap_or(*from);
            last_char = *range.end();
            let range_len = *range.end() as u32 - *range.start() as u32 + 1;
            if i < range_len {
                return char::from_u32(*range.start() as u32 + i).unwrap();
            }
            i -= range_len;
        }
        last_char
    }
}

pub(super) enum PatternString {
    Range(PatternRange),
    String(String),
}

pub(super) struct PatternRange {
    negated: bool,
    chars: HashSet<char>,
    ranges: Vec<(char, char)>,
}

impl PatternRange {
    pub(super) fn is_match(&self, c: char) -> bool {
        if self.negated {
            !self.chars.contains(&c)
                && !self
                    .ranges
                    .iter()
                    .any(|(start, end)| c >= *start && c <= *end)
        } else {
            self.chars.contains(&c)
                || self
                    .ranges
                    .iter()
                    .any(|(start, end)| c >= *start && c <= *end)
        }
    }
}

impl Argument for PatternString {
    type Arg = PatternString;

    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>> {
        match token {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
                // Parse as regex-group-lite
                let mut s = String::with_capacity(32);
                parse_stream(&mut s, "input", OutputType::RawString, group.stream());

                let mut chars = HashSet::new();
                let mut ranges = Vec::new();
                let mut it = s.chars();
                let mut last_char = it.next();
                let negated = if last_char == Some('^') {
                    last_char = None;
                    true
                } else {
                    false
                };
                while let Some(c) = it.next() {
                    match (c, last_char.take()) {
                        ('-', Some(last_char)) => {
                            let next = it.next();
                            if let Some(next) = next {
                                ranges.push((last_char, next));
                            } else {
                                chars.insert(last_char);
                                chars.insert('-');
                            }
                        }
                        (c, None) => {
                            last_char = Some(c);
                        }
                        (c, Some(last)) => {
                            chars.insert(last);
                            last_char = Some(c);
                        }
                    }
                }
                if let Some(last_char) = last_char {
                    chars.insert(last_char);
                }
                Ok(PatternString::Range(PatternRange {
                    negated,
                    chars,
                    ranges,
                }))
            }
            _ => {
                let s = <String as Argument>::from_token_tree(token)?;
                Ok(PatternString::String(s))
            }
        }
    }
}

impl Argument for TranslateString {
    type Arg = TranslateString;

    fn from_token_tree(token: TokenTree) -> Result<Self::Arg, Cow<'static, str>> {
        let s = <String as Argument>::from_token_tree(token)?;

        let mut it = s.chars();
        let mut last_char = it.next();
        let mut chars = Vec::new();
        while let Some(c) = it.next() {
            match (c, last_char.take()) {
                ('-', Some(last)) => {
                    let next = it.next();
                    if let Some(next) = next {
                        chars.push((last, Some(next)));
                    } else {
                        chars.push((last, None));
                    }
                }
                (c, Some(last)) => {
                    chars.push((last, None));
                    last_char = Some(c);
                }
                (c, None) => {
                    last_char = Some(c);
                }
            }
        }
        if let Some(last_char) = last_char {
            chars.push((last_char, None));
        }
        Ok(TranslateString::new(chars))
    }
}

/// Parse arguments from inside of a group: `(key=value, key2=value2, ...)`. Each value is a token tree
/// (a group, literal, ident, or punctuation).
///
/// Keys must be idents.
#[allow(unknown_lints, tail_expr_drop_order)]
pub(super) fn parse_arguments(item: TokenStream, func: &str) -> HashMap<String, TokenTree> {
    let mut args = HashMap::new();
    let mut item = item.into_iter();
    loop {
        match item.next() {
            Some(TokenTree::Ident(ident)) => match item.next() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
                    args.insert(
                        ident.to_string(),
                        item.next()
                            .unwrap_or_else(|| panic!("{func}: Expected a value after =")),
                    );
                }
                Some(token) => {
                    panic!("{func}: Expected a key=value pair, got {:?}", token);
                }
                None => {
                    panic!("{func}: Expected = after key");
                }
            },
            None => {
                return args;
            }
            Some(token) => {
                panic!(
                    "{func}: Expected a sequence of key=value pairs, got {:?}",
                    token
                );
            }
        }
    }
}

pub(super) fn pop_argument<T: Argument>(
    args: &mut HashMap<String, TokenTree>,
    func: &str,
    key: &str,
) -> T::Arg {
    let Some(token) = args.remove(key) else {
        panic!("{func}: Missing required argument '{key}'");
    };
    match T::from_token_tree(token) {
        Ok(arg) => arg,
        Err(e) => {
            panic!("{func}: Invalid argument '{key}': {e}");
        }
    }
}

pub(super) fn pop_optional_argument<T: Argument>(
    args: &mut HashMap<String, TokenTree>,
    func: &str,
    key: &str,
) -> Option<T::Arg> {
    let token = args.remove(key)?;
    match T::from_token_tree(token) {
        Ok(arg) => Some(arg),
        Err(e) => {
            panic!("{func}: Invalid argument '{key}': {e}");
        }
    }
}
