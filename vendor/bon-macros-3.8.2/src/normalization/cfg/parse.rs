use crate::util::prelude::*;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod kw {
    syn::custom_keyword!(__cfgs);
}

pub(crate) fn parse_predicate_results(tokens: TokenStream) -> Result<Option<PredicateResults>> {
    let results: WrapOption<PredicateResults> = syn::parse2(tokens)?;
    Ok(results.0)
}

// Newtypes over an `Option` to be able to implement trait on it
#[derive(Debug)]
struct WrapOption<T>(Option<T>);

/// Represents a special directive inserted at the beginning of the macro parameters
/// that has the syntax `@cfgs(true, false, true)`. It delivers the results of cfg
/// evaluations to the macro.
#[derive(Debug)]
pub(crate) struct PredicateResults {
    pub(crate) results: Vec<bool>,
    pub(crate) recursion_counter: usize,
    pub(crate) rest: TokenStream,
}

impl Parse for WrapOption<PredicateResults> {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if !input.peek(kw::__cfgs) {
            // We need to exhaust the input stream to avoid a "unexpected token" error
            input.parse::<TokenStream>()?;

            return Ok(Self(None));
        }

        input.parse::<kw::__cfgs>()?;

        let results;
        syn::parenthesized!(results in input);

        let recursion_counter: syn::LitInt = results.parse()?;
        let recursion_counter = recursion_counter.base10_parse::<usize>()?;

        results.parse::<syn::Token![,]>()?;

        let results: Vec<bool> =
            Punctuated::<syn::LitBool, syn::Token![,]>::parse_terminated(&results)?
                .into_iter()
                .map(|bool| bool.value)
                .collect();

        let results = PredicateResults {
            results,
            recursion_counter,
            rest: input.parse()?,
        };

        Ok(Self(Some(results)))
    }
}

pub(crate) enum CfgSyntax {
    Cfg(TokenStream),
    CfgAttr(CfgAttr),
}

impl CfgSyntax {
    pub(crate) fn from_meta(meta: &syn::Meta) -> Result<Option<Self>> {
        let meta = match meta {
            syn::Meta::List(meta) => meta,
            _ => return Ok(None),
        };

        if meta.path.is_ident("cfg") {
            return Ok(Some(Self::Cfg(meta.tokens.clone())));
        }

        if meta.path.is_ident("cfg_attr") {
            let cfg_attr = syn::parse2(meta.tokens.clone())?;
            return Ok(Some(Self::CfgAttr(cfg_attr)));
        }

        Ok(None)
    }
}

pub(crate) struct CfgAttr {
    pub(crate) predicate: Box<syn::Meta>,
    pub(crate) then_branch: Punctuated<syn::Meta, syn::Token![,]>,
}

impl Parse for CfgAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let predicate = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let then_branch = Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated(input)?;

        Ok(Self {
            predicate,
            then_branch,
        })
    }
}
