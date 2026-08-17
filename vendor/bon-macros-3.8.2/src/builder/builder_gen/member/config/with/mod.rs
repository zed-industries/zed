mod closure;

pub(crate) use closure::*;

use crate::util::prelude::*;
use darling::FromMeta;

#[derive(Debug)]
pub(crate) enum WithConfig {
    /// Closure syntax e.g. `#[builder(with = |param: Type| body)]`
    Closure(SetterClosure),

    /// Well-known path [`Option::Some`]
    Some(syn::Path),

    /// Well-known path [`FromIterator::from_iter`] or `<_>::from_iter`
    FromIter(syn::ExprPath),
}

impl WithConfig {
    pub(crate) fn as_closure(&self) -> Option<&SetterClosure> {
        match self {
            Self::Closure(closure) => Some(closure),
            _ => None,
        }
    }
}

impl FromMeta for WithConfig {
    fn from_meta(meta: &syn::Meta) -> darling::Result<Self> {
        let err = || {
            err!(
                meta,
                "expected a closure e.g. `#[builder(with = |param: T| expression)]` or \
                a well-known function path which could be one of:\n\
                - #[builder(with = Some)]\n\
                - #[builder(with = FromIterator::from_iter)]\n\
                - #[builder(with = <_>::from_iter)] (same as above, but shorter)",
            )
        };

        let name_val = match meta {
            syn::Meta::NameValue(meta) => meta,
            _ => return Err(err()),
        };

        if let syn::Expr::Closure(_) = name_val.value {
            return SetterClosure::from_meta(meta).map(Self::Closure);
        }

        let path = match &name_val.value {
            syn::Expr::Path(path) => path,
            _ => return Err(err()),
        };

        crate::parsing::reject_attrs(&path.attrs)?;

        if *path == syn::parse_quote!(Some) {
            return Ok(Self::Some(path.path.clone()));
        }

        if *path == syn::parse_quote!(FromIterator::from_iter)
            || *path == syn::parse_quote!(<_>::from_iter)
        {
            return Ok(Self::FromIter(path.clone()));
        }

        Err(err())
    }
}
