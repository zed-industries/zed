use crate::util::prelude::*;
use syn::parse::{ParseStream, Parser};

pub(crate) fn privatize_fn(input: TokenStream) -> TokenStream {
    transform
        .parse2(input)
        .unwrap_or_else(syn::Error::into_compile_error)
}

fn transform(input: ParseStream<'_>) -> syn::Result<TokenStream> {
    let outer_attrs = syn::Attribute::parse_outer(input)?;
    let vis: syn::Visibility = input.parse()?;

    let mut sig: syn::Signature = input.parse()?;
    privatize_fn_name(&mut sig);

    let rest: TokenStream = input.parse()?;

    Ok(quote::quote! {
        #(#outer_attrs)* #vis #sig #rest
    })
}

pub(crate) fn privatize_fn_name(sig: &mut syn::Signature) {
    // We don't generate a random name to ensure reproducible builds. Some of
    // `bon`'s users use custom build systems where the output of the macro is
    // cached and the stability of the output is depended upon.
    sig.ident = quote::format_ident!("__orig_{}", sig.ident.raw_name());
}
