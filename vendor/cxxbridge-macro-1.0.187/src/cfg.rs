use crate::syntax::cfg::{CfgExpr, ComputedCfg};
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt as _};
use syn::{token, AttrStyle, Attribute, MacroDelimiter, Meta, MetaList, Path, Token};

impl<'a> ComputedCfg<'a> {
    pub(crate) fn into_attr(&self) -> Option<Attribute> {
        if let ComputedCfg::Leaf(CfgExpr::Unconditional) = self {
            None
        } else {
            let span = Span::call_site();
            Some(Attribute {
                pound_token: Token![#](span),
                style: AttrStyle::Outer,
                bracket_token: token::Bracket(span),
                meta: Meta::List(MetaList {
                    path: Path::from(Ident::new("cfg", span)),
                    delimiter: MacroDelimiter::Paren(token::Paren(span)),
                    tokens: self.as_meta().into_token_stream(),
                }),
            })
        }
    }

    pub(crate) fn as_meta(&self) -> impl ToTokens + '_ {
        Print {
            cfg: self,
            span: Span::call_site(),
        }
    }
}

struct Print<'a, Cfg> {
    cfg: &'a Cfg,
    span: Span,
}

impl<'a> ToTokens for Print<'a, CfgExpr> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.span;
        let print = |cfg| Print { cfg, span };
        match self.cfg {
            CfgExpr::Unconditional => unreachable!(),
            CfgExpr::Eq(ident, value) => {
                ident.to_tokens(tokens);
                if let Some(value) = value {
                    Token![=](span).to_tokens(tokens);
                    value.to_tokens(tokens);
                }
            }
            CfgExpr::All(inner) => {
                tokens.append(Ident::new("all", span));
                let mut group = TokenStream::new();
                group.append_separated(inner.iter().map(print), Token![,](span));
                tokens.append(Group::new(Delimiter::Parenthesis, group));
            }
            CfgExpr::Any(inner) => {
                tokens.append(Ident::new("any", span));
                let mut group = TokenStream::new();
                group.append_separated(inner.iter().map(print), Token![,](span));
                tokens.append(Group::new(Delimiter::Parenthesis, group));
            }
            CfgExpr::Not(inner) => {
                tokens.append(Ident::new("not", span));
                let group = print(inner).into_token_stream();
                tokens.append(Group::new(Delimiter::Parenthesis, group));
            }
        }
    }
}

impl<'a> ToTokens for Print<'a, ComputedCfg<'a>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.span;
        match *self.cfg {
            ComputedCfg::Leaf(cfg) => Print { cfg, span }.to_tokens(tokens),
            ComputedCfg::All(ref inner) => {
                tokens.append(Ident::new("all", span));
                let mut group = TokenStream::new();
                group.append_separated(
                    inner.iter().map(|&cfg| Print { cfg, span }),
                    Token![,](span),
                );
                tokens.append(Group::new(Delimiter::Parenthesis, group));
            }
            ComputedCfg::Any(ref inner) => {
                tokens.append(Ident::new("any", span));
                let mut group = TokenStream::new();
                group
                    .append_separated(inner.iter().map(|cfg| Print { cfg, span }), Token![,](span));
                tokens.append(Group::new(Delimiter::Parenthesis, group));
            }
        }
    }
}
