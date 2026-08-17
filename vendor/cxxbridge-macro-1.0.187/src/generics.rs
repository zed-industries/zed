use crate::syntax::instantiate::NamedImplKey;
use crate::syntax::resolve::Resolution;
use crate::syntax::types::ConditionalImpl;
use crate::syntax::{Impl, Lifetimes};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Lifetime, Token};

pub(crate) struct ImplGenerics<'a> {
    explicit_impl: Option<&'a Impl>,
    resolve: Resolution<'a>,
}

pub(crate) struct TyGenerics<'a> {
    key: &'a NamedImplKey<'a>,
    explicit_impl: Option<&'a Impl>,
    resolve: Resolution<'a>,
}

pub(crate) fn split_for_impl<'a>(
    key: &'a NamedImplKey<'a>,
    conditional_impl: &ConditionalImpl<'a>,
    resolve: Resolution<'a>,
) -> (ImplGenerics<'a>, TyGenerics<'a>) {
    let impl_generics = ImplGenerics {
        explicit_impl: conditional_impl.explicit_impl,
        resolve,
    };
    let ty_generics = TyGenerics {
        key,
        explicit_impl: conditional_impl.explicit_impl,
        resolve,
    };
    (impl_generics, ty_generics)
}

impl<'a> ToTokens for ImplGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(imp) = self.explicit_impl {
            imp.impl_generics.to_tokens(tokens);
        } else {
            self.resolve.generics.to_tokens(tokens);
        }
    }
}

impl<'a> ToTokens for TyGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(imp) = self.explicit_impl {
            imp.ty_generics.to_tokens(tokens);
        } else if !self.resolve.generics.lifetimes.is_empty() {
            let span = self.key.rust.span();
            self.key
                .lt_token
                .unwrap_or_else(|| Token![<](span))
                .to_tokens(tokens);
            self.resolve.generics.lifetimes.to_tokens(tokens);
            self.key
                .gt_token
                .unwrap_or_else(|| Token![>](span))
                .to_tokens(tokens);
        }
    }
}

pub(crate) struct UnderscoreLifetimes<'a> {
    generics: &'a Lifetimes,
}

impl Lifetimes {
    pub(crate) fn to_underscore_lifetimes(&self) -> UnderscoreLifetimes {
        UnderscoreLifetimes { generics: self }
    }
}

impl<'a> ToTokens for UnderscoreLifetimes<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Lifetimes {
            lt_token,
            lifetimes,
            gt_token,
        } = self.generics;
        lt_token.to_tokens(tokens);
        for pair in lifetimes.pairs() {
            let (lifetime, punct) = pair.into_tuple();
            let lifetime = Lifetime::new("'_", lifetime.span());
            lifetime.to_tokens(tokens);
            punct.to_tokens(tokens);
        }
        gt_token.to_tokens(tokens);
    }
}
