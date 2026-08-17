use super::{reject_attrs, reject_syntax};
use crate::util::prelude::*;
use darling::FromMeta;

/// Utility type for parsing simple closure syntax that only allows [`syn::PatIdent`]
/// inputs and rejects any attributes and prefix keywords like `async`, `move`, `for`
/// on the closure.
#[derive(Debug)]
pub(crate) struct SimpleClosure {
    pub(crate) inputs: Vec<SimpleClosureInput>,
    pub(crate) body: Box<syn::Expr>,
    pub(crate) output: syn::ReturnType,
}

#[derive(Debug)]
pub(crate) struct SimpleClosureInput {
    pub(crate) pat: syn::PatIdent,
    pub(crate) ty: Option<Box<syn::Type>>,
}

impl FromMeta for SimpleClosure {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        let err = || {
            let path = darling::util::path_to_string(meta.path());
            err!(
                meta,
                "expected a closure e.g. `{path} = |param: T| expression`"
            )
        };

        let meta = match meta {
            syn::Meta::NameValue(meta) => meta,
            _ => return Err(err()),
        };

        let closure = match &meta.value {
            syn::Expr::Closure(closure) => closure,
            _ => return Err(err()),
        };

        reject_syntax("`for<...>` syntax", &closure.lifetimes)?;
        reject_syntax("`const` keyword", &closure.constness)?;
        reject_syntax("`static` keyword", &closure.movability)?;
        reject_syntax("`async` keyword", &closure.asyncness)?;
        reject_syntax("`move` keyword", &closure.capture)?;
        reject_attrs(&closure.attrs)?;

        let inputs = closure
            .clone()
            .inputs
            .into_iter()
            .map(|input| match input {
                syn::Pat::Ident(pat) => SimpleClosureInput::from_pat_ident(pat),
                syn::Pat::Type(pat) => SimpleClosureInput::from_pat_type(pat),
                _ => bail!(&input, "expected a simple identifier pattern"),
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            inputs,
            body: closure.body.clone(),
            output: closure.output.clone(),
        })
    }
}

impl SimpleClosureInput {
    fn from_pat_ident(pat: syn::PatIdent) -> Result<Self> {
        reject_attrs(&pat.attrs)?;
        reject_syntax("`ref` keyword", &pat.by_ref)?;
        Ok(Self { pat, ty: None })
    }

    fn from_pat_type(input: syn::PatType) -> Result<Self> {
        reject_attrs(&input.attrs)?;

        let ident = match *input.pat {
            syn::Pat::Ident(pat) => Self::from_pat_ident(pat)?.pat,
            _ => bail!(&input.pat, "expected a simple identifier pattern"),
        };

        Ok(Self {
            pat: ident,
            ty: Some(input.ty),
        })
    }
}
