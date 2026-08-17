use crate::algorithm::Printer;
use crate::fixup::FixupContext;
use crate::path::PathKind;
use crate::INDENT;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use syn::{AttrStyle, Attribute, Expr, Lit, MacroDelimiter, Meta, MetaList, MetaNameValue};

impl Printer {
    pub fn outer_attrs(&mut self, attrs: &[Attribute]) {
        for attr in attrs {
            if let AttrStyle::Outer = attr.style {
                self.attr(attr);
            }
        }
    }

    pub fn inner_attrs(&mut self, attrs: &[Attribute]) {
        for attr in attrs {
            if let AttrStyle::Inner(_) = attr.style {
                self.attr(attr);
            }
        }
    }

    fn attr(&mut self, attr: &Attribute) {
        if let Some(mut doc) = value_of_attribute("doc", attr) {
            if !doc.contains('\n')
                && match attr.style {
                    AttrStyle::Outer => !doc.starts_with('/'),
                    AttrStyle::Inner(_) => true,
                }
            {
                trim_trailing_spaces(&mut doc);
                self.word(match attr.style {
                    AttrStyle::Outer => "///",
                    AttrStyle::Inner(_) => "//!",
                });
                self.word(doc);
                self.hardbreak();
                return;
            } else if can_be_block_comment(&doc)
                && match attr.style {
                    AttrStyle::Outer => !doc.starts_with(&['*', '/'][..]),
                    AttrStyle::Inner(_) => true,
                }
            {
                trim_interior_trailing_spaces(&mut doc);
                self.word(match attr.style {
                    AttrStyle::Outer => "/**",
                    AttrStyle::Inner(_) => "/*!",
                });
                self.word(doc);
                self.word("*/");
                self.hardbreak();
                return;
            }
        } else if let Some(mut comment) = value_of_attribute("comment", attr) {
            if !comment.contains('\n') {
                trim_trailing_spaces(&mut comment);
                self.word("//");
                self.word(comment);
                self.hardbreak();
                return;
            } else if can_be_block_comment(&comment) && !comment.starts_with(&['*', '!'][..]) {
                trim_interior_trailing_spaces(&mut comment);
                self.word("/*");
                self.word(comment);
                self.word("*/");
                self.hardbreak();
                return;
            }
        }

        self.word(match attr.style {
            AttrStyle::Outer => "#",
            AttrStyle::Inner(_) => "#!",
        });
        self.word("[");
        self.meta(&attr.meta);
        self.word("]");
        self.space();
    }

    fn meta(&mut self, meta: &Meta) {
        match meta {
            Meta::Path(path) => self.path(path, PathKind::Simple),
            Meta::List(meta) => self.meta_list(meta),
            Meta::NameValue(meta) => self.meta_name_value(meta),
        }
    }

    fn meta_list(&mut self, meta: &MetaList) {
        self.path(&meta.path, PathKind::Simple);
        let delimiter = match meta.delimiter {
            MacroDelimiter::Paren(_) => Delimiter::Parenthesis,
            MacroDelimiter::Brace(_) => Delimiter::Brace,
            MacroDelimiter::Bracket(_) => Delimiter::Bracket,
        };
        let group = Group::new(delimiter, meta.tokens.clone());
        self.attr_tokens(TokenStream::from(TokenTree::Group(group)));
    }

    fn meta_name_value(&mut self, meta: &MetaNameValue) {
        self.path(&meta.path, PathKind::Simple);
        self.word(" = ");
        self.expr(&meta.value, FixupContext::NONE);
    }

    fn attr_tokens(&mut self, tokens: TokenStream) {
        let mut stack = Vec::new();
        stack.push((tokens.into_iter().peekable(), Delimiter::None));
        let mut space = Self::nbsp as fn(&mut Self);

        #[derive(PartialEq)]
        enum State {
            Word,
            Punct,
            TrailingComma,
        }

        use State::*;
        let mut state = Word;

        while let Some((tokens, delimiter)) = stack.last_mut() {
            match tokens.next() {
                Some(TokenTree::Ident(ident)) => {
                    if let Word = state {
                        space(self);
                    }
                    self.ident(&ident);
                    state = Word;
                }
                Some(TokenTree::Punct(punct)) => {
                    let ch = punct.as_char();
                    if let (Word, '=') = (state, ch) {
                        self.nbsp();
                    }
                    if ch == ',' && tokens.peek().is_none() {
                        self.trailing_comma(true);
                        state = TrailingComma;
                    } else {
                        self.token_punct(ch);
                        if ch == '=' {
                            self.nbsp();
                        } else if ch == ',' {
                            space(self);
                        }
                        state = Punct;
                    }
                }
                Some(TokenTree::Literal(literal)) => {
                    if let Word = state {
                        space(self);
                    }
                    self.token_literal(&literal);
                    state = Word;
                }
                Some(TokenTree::Group(group)) => {
                    let delimiter = group.delimiter();
                    let stream = group.stream();
                    match delimiter {
                        Delimiter::Parenthesis => {
                            self.word("(");
                            self.cbox(INDENT);
                            self.zerobreak();
                            state = Punct;
                        }
                        Delimiter::Brace => {
                            self.word("{");
                            state = Punct;
                        }
                        Delimiter::Bracket => {
                            self.word("[");
                            state = Punct;
                        }
                        Delimiter::None => {}
                    }
                    stack.push((stream.into_iter().peekable(), delimiter));
                    space = Self::space;
                }
                None => {
                    match delimiter {
                        Delimiter::Parenthesis => {
                            if state != TrailingComma {
                                self.zerobreak();
                            }
                            self.offset(-INDENT);
                            self.end();
                            self.word(")");
                            state = Punct;
                        }
                        Delimiter::Brace => {
                            self.word("}");
                            state = Punct;
                        }
                        Delimiter::Bracket => {
                            self.word("]");
                            state = Punct;
                        }
                        Delimiter::None => {}
                    }
                    stack.pop();
                    if stack.is_empty() {
                        space = Self::nbsp;
                    }
                }
            }
        }
    }
}

fn value_of_attribute(requested: &str, attr: &Attribute) -> Option<String> {
    let value = match &attr.meta {
        Meta::NameValue(meta) if meta.path.is_ident(requested) => &meta.value,
        _ => return None,
    };
    let lit = match value {
        Expr::Lit(expr) if expr.attrs.is_empty() => &expr.lit,
        _ => return None,
    };
    match lit {
        Lit::Str(string) => Some(string.value()),
        _ => None,
    }
}

pub fn has_outer(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let AttrStyle::Outer = attr.style {
            return true;
        }
    }
    false
}

pub fn has_inner(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let AttrStyle::Inner(_) = attr.style {
            return true;
        }
    }
    false
}

fn trim_trailing_spaces(doc: &mut String) {
    doc.truncate(doc.trim_end_matches(' ').len());
}

fn trim_interior_trailing_spaces(doc: &mut String) {
    if !doc.contains(" \n") {
        return;
    }
    let mut trimmed = String::with_capacity(doc.len());
    let mut lines = doc.split('\n').peekable();
    while let Some(line) = lines.next() {
        if lines.peek().is_some() {
            trimmed.push_str(line.trim_end_matches(' '));
            trimmed.push('\n');
        } else {
            trimmed.push_str(line);
        }
    }
    *doc = trimmed;
}

fn can_be_block_comment(value: &str) -> bool {
    let mut depth = 0usize;
    let bytes = value.as_bytes();
    let mut i = 0usize;
    let upper = bytes.len() - 1;

    while i < upper {
        if bytes[i] == b'/' && bytes[i + 1] == b'*' {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'*' && bytes[i + 1] == b'/' {
            if depth == 0 {
                return false;
            }
            depth -= 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    depth == 0 && !value.ends_with('/')
}
