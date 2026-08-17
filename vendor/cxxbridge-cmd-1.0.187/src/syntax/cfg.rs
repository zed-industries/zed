use indexmap::{indexset as set, IndexSet as Set};
use proc_macro2::Ident;
use std::hash::{Hash, Hasher};
use std::iter;
use std::mem;
use syn::parse::{Error, ParseStream, Result};
use syn::{parenthesized, token, Attribute, LitStr, Token};

#[derive(Clone)]
pub(crate) enum CfgExpr {
    Unconditional,
    Eq(Ident, Option<LitStr>),
    All(Vec<CfgExpr>),
    Any(Vec<CfgExpr>),
    Not(Box<CfgExpr>),
}

#[derive(Clone)]
pub(crate) enum ComputedCfg<'a> {
    Leaf(&'a CfgExpr),
    All(Set<&'a CfgExpr>),
    Any(Set<ComputedCfg<'a>>),
}

impl CfgExpr {
    pub(crate) fn merge_and(&mut self, expr: CfgExpr) {
        if let CfgExpr::Unconditional = self {
            *self = expr;
        } else if let CfgExpr::Unconditional = expr {
            // drop
        } else if let CfgExpr::All(list) = self {
            list.push(expr);
        } else {
            let prev = mem::replace(self, CfgExpr::Unconditional);
            *self = CfgExpr::All(vec![prev, expr]);
        }
    }
}

impl<'a> ComputedCfg<'a> {
    pub(crate) fn all(one: &'a CfgExpr, two: &'a CfgExpr) -> Self {
        if let (cfg, CfgExpr::Unconditional) | (CfgExpr::Unconditional, cfg) = (one, two) {
            ComputedCfg::Leaf(cfg)
        } else if one == two {
            ComputedCfg::Leaf(one)
        } else {
            ComputedCfg::All(set![one, two])
        }
    }

    pub(crate) fn merge_or(&mut self, other: impl Into<ComputedCfg<'a>>) {
        let other = other.into();
        if let ComputedCfg::Leaf(CfgExpr::Unconditional) = self {
            // drop
        } else if let ComputedCfg::Leaf(CfgExpr::Unconditional) = other {
            *self = other;
        } else if *self == other {
            // drop
        } else if let ComputedCfg::Any(list) = self {
            list.insert(other);
        } else {
            let prev = mem::replace(self, ComputedCfg::Any(Set::new()));
            let ComputedCfg::Any(list) = self else {
                unreachable!();
            };
            list.extend([prev, other]);
        }
    }
}

impl<'a> From<&'a CfgExpr> for ComputedCfg<'a> {
    fn from(cfg: &'a CfgExpr) -> Self {
        ComputedCfg::Leaf(cfg)
    }
}

impl Eq for CfgExpr {}

impl PartialEq for CfgExpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CfgExpr::Unconditional, CfgExpr::Unconditional) => true,
            (CfgExpr::Eq(this_ident, None), CfgExpr::Eq(other_ident, None)) => {
                this_ident == other_ident
            }
            (
                CfgExpr::Eq(this_ident, Some(this_value)),
                CfgExpr::Eq(other_ident, Some(other_value)),
            ) => {
                this_ident == other_ident
                    && this_value.token().to_string() == other_value.token().to_string()
            }
            (CfgExpr::All(this), CfgExpr::All(other))
            | (CfgExpr::Any(this), CfgExpr::Any(other)) => this == other,
            (CfgExpr::Not(this), CfgExpr::Not(other)) => this == other,
            (_, _) => false,
        }
    }
}

impl Hash for CfgExpr {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        mem::discriminant(self).hash(hasher);
        match self {
            CfgExpr::Unconditional => {}
            CfgExpr::Eq(ident, value) => {
                ident.hash(hasher);
                // syn::LitStr does not have its own Hash impl
                value.as_ref().map(LitStr::value).hash(hasher);
            }
            CfgExpr::All(inner) | CfgExpr::Any(inner) => inner.hash(hasher),
            CfgExpr::Not(inner) => inner.hash(hasher),
        }
    }
}

impl<'a> Eq for ComputedCfg<'a> {}

impl<'a> PartialEq for ComputedCfg<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ComputedCfg::Leaf(this), ComputedCfg::Leaf(other)) => this == other,
            // For the purpose of deduplicating the contents of an `all` or
            // `any`, we only consider sets equal if they contain the same cfgs
            // in the same order.
            (ComputedCfg::All(this), ComputedCfg::All(other)) => {
                this.len() == other.len()
                    && iter::zip(this, other).all(|(this, other)| this == other)
            }
            (ComputedCfg::Any(this), ComputedCfg::Any(other)) => {
                this.len() == other.len()
                    && iter::zip(this, other).all(|(this, other)| this == other)
            }
            (_, _) => false,
        }
    }
}

impl<'a> Hash for ComputedCfg<'a> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        mem::discriminant(self).hash(hasher);
        match self {
            ComputedCfg::Leaf(cfg) => cfg.hash(hasher),
            ComputedCfg::All(inner) => inner.iter().for_each(|cfg| cfg.hash(hasher)),
            ComputedCfg::Any(inner) => inner.iter().for_each(|cfg| cfg.hash(hasher)),
        }
    }
}

pub(crate) fn parse_attribute(attr: &Attribute) -> Result<CfgExpr> {
    attr.parse_args_with(|input: ParseStream| {
        let cfg_expr = input.call(parse_single)?;
        input.parse::<Option<Token![,]>>()?;
        Ok(cfg_expr)
    })
}

fn parse_single(input: ParseStream) -> Result<CfgExpr> {
    let ident: Ident = input.parse()?;
    let lookahead = input.lookahead1();
    if input.peek(token::Paren) {
        let content;
        parenthesized!(content in input);
        if ident == "all" {
            let list = content.call(parse_multiple)?;
            Ok(CfgExpr::All(list))
        } else if ident == "any" {
            let list = content.call(parse_multiple)?;
            Ok(CfgExpr::Any(list))
        } else if ident == "not" {
            let expr = content.call(parse_single)?;
            content.parse::<Option<Token![,]>>()?;
            Ok(CfgExpr::Not(Box::new(expr)))
        } else {
            Err(Error::new(ident.span(), "unrecognized cfg expression"))
        }
    } else if lookahead.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        let string: LitStr = input.parse()?;
        Ok(CfgExpr::Eq(ident, Some(string)))
    } else if lookahead.peek(Token![,]) || input.is_empty() {
        Ok(CfgExpr::Eq(ident, None))
    } else {
        Err(lookahead.error())
    }
}

fn parse_multiple(input: ParseStream) -> Result<Vec<CfgExpr>> {
    let mut vec = Vec::new();
    while !input.is_empty() {
        let expr = input.call(parse_single)?;
        vec.push(expr);
        if input.is_empty() {
            break;
        }
        input.parse::<Token![,]>()?;
    }
    Ok(vec)
}
