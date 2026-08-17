use crate::syntax::qualified::QualifiedName;
use std::slice::Iter;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{Expr, Ident, Lit, Meta, Token};

mod kw {
    syn::custom_keyword!(namespace);
}

#[derive(Clone, Default, PartialEq)]
pub(crate) struct Namespace {
    segments: Vec<Ident>,
}

impl Namespace {
    pub(crate) const ROOT: Self = Namespace {
        segments: Vec::new(),
    };

    pub(crate) fn iter(&self) -> Iter<Ident> {
        self.segments.iter()
    }

    pub(crate) fn parse_bridge_attr_namespace(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Namespace::ROOT);
        }

        input.parse::<kw::namespace>()?;
        input.parse::<Token![=]>()?;
        let namespace = input.parse::<Namespace>()?;
        input.parse::<Option<Token![,]>>()?;
        Ok(namespace)
    }

    pub(crate) fn parse_meta(meta: &Meta) -> Result<Self> {
        if let Meta::NameValue(meta) = meta {
            match &meta.value {
                Expr::Lit(expr) => {
                    if let Lit::Str(lit) = &expr.lit {
                        let segments = QualifiedName::parse_quoted(lit)?.segments;
                        return Ok(Namespace { segments });
                    }
                }
                Expr::Path(expr)
                    if expr.qself.is_none()
                        && expr
                            .path
                            .segments
                            .iter()
                            .all(|segment| segment.arguments.is_none()) =>
                {
                    let segments = expr
                        .path
                        .segments
                        .iter()
                        .map(|segment| segment.ident.clone())
                        .collect();
                    return Ok(Namespace { segments });
                }
                _ => {}
            }
        }
        Err(Error::new_spanned(meta, "unsupported namespace attribute"))
    }
}

impl Default for &Namespace {
    fn default() -> Self {
        const ROOT: &Namespace = &Namespace::ROOT;
        ROOT
    }
}

impl Parse for Namespace {
    fn parse(input: ParseStream) -> Result<Self> {
        let segments = QualifiedName::parse_quoted_or_unquoted(input)?.segments;
        Ok(Namespace { segments })
    }
}

impl<'a> IntoIterator for &'a Namespace {
    type Item = &'a Ident;
    type IntoIter = Iter<'a, Ident>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> FromIterator<&'a Ident> for Namespace {
    fn from_iter<I>(idents: I) -> Self
    where
        I: IntoIterator<Item = &'a Ident>,
    {
        let segments = idents.into_iter().cloned().collect();
        Namespace { segments }
    }
}
