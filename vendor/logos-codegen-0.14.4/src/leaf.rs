use std::cmp::{Ord, Ordering};
use std::fmt::{self, Debug, Display};

use proc_macro2::{Span, TokenStream};
use syn::{spanned::Spanned, Ident};

use crate::graph::{Disambiguate, Node};
use crate::util::MaybeVoid;

#[derive(Clone)]
pub struct Leaf<'t> {
    pub ident: Option<&'t Ident>,
    pub span: Span,
    pub priority: usize,
    pub field: MaybeVoid,
    pub callback: Option<Callback>,
}

#[derive(Clone)]
pub enum Callback {
    Label(TokenStream),
    Inline(Box<InlineCallback>),
    Skip(Span),
}

#[derive(Clone)]
pub struct InlineCallback {
    pub arg: Ident,
    pub body: TokenStream,
    pub span: Span,
}

impl From<InlineCallback> for Callback {
    fn from(inline: InlineCallback) -> Callback {
        Callback::Inline(Box::new(inline))
    }
}

impl Callback {
    pub fn span(&self) -> Span {
        match self {
            Callback::Label(tokens) => tokens.span(),
            Callback::Inline(inline) => inline.span,
            Callback::Skip(span) => *span,
        }
    }
}

impl<'t> Leaf<'t> {
    pub fn new(ident: &'t Ident, span: Span) -> Self {
        Leaf {
            ident: Some(ident),
            span,
            priority: 0,
            field: MaybeVoid::Void,
            callback: None,
        }
    }

    pub fn new_skip(span: Span) -> Self {
        Leaf {
            ident: None,
            span,
            priority: 0,
            field: MaybeVoid::Void,
            callback: Some(Callback::Skip(span)),
        }
    }

    pub fn callback(mut self, callback: Option<Callback>) -> Self {
        self.callback = callback;
        self
    }

    pub fn field(mut self, field: MaybeVoid) -> Self {
        self.field = field;
        self
    }

    pub fn priority(mut self, priority: usize) -> Self {
        self.priority = priority;
        self
    }
}

impl Disambiguate for Leaf<'_> {
    fn cmp(left: &Leaf, right: &Leaf) -> Ordering {
        Ord::cmp(&left.priority, &right.priority)
    }
}

impl<'t> From<Leaf<'t>> for Node<Leaf<'t>> {
    fn from(leaf: Leaf<'t>) -> Self {
        Node::Leaf(leaf)
    }
}

impl Debug for Leaf<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "::{}", self)?;

        match self.callback {
            Some(Callback::Label(ref label)) => write!(f, " ({})", label),
            Some(Callback::Inline(_)) => f.write_str(" (<inline>)"),
            Some(Callback::Skip(_)) => f.write_str(" (<skip>)"),
            None => Ok(()),
        }
    }
}

impl Display for Leaf<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ident {
            Some(ident) => Display::fmt(ident, f),
            None => f.write_str("<skip>"),
        }
    }
}
