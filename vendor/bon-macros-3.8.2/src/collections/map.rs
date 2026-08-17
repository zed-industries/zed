use crate::util::prelude::*;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::{Expr, Token};

pub(crate) fn parse_macro_input(
    input: ParseStream<'_>,
) -> Result<Punctuated<(Expr, Expr), Token![,]>, syn::Error> {
    Punctuated::parse_terminated_with(input, parse_map_pair)
}

fn parse_map_pair(pair: ParseStream<'_>) -> Result<(Expr, Expr), syn::Error> {
    let key = pair.parse().map_err(|_| pair.error(pair))?;
    let _: Token![:] = pair.parse()?;
    let value = pair.parse()?;

    Ok((key, value))
}

pub(crate) fn generate(entries: Punctuated<(Expr, Expr), Token![,]>) -> TokenStream {
    let error =
        super::validate_expressions_are_unique("key in the map", entries.iter().map(|(k, _)| k));

    let items = entries.into_iter().map(|(key, value)| {
        quote!((
            ::core::convert::Into::into(#key),
            ::core::convert::Into::into(#value),
        ))
    });

    let output = quote! {
        ::core::iter::FromIterator::from_iter([
            #(#items),*
        ])
    };

    // We unconditionally return `output` as part of the result to make sure IDEs
    // see this output and see what input tokens map to what output tokens. This
    // way IDEs can provide better help to the developer even when there are errors.
    error
        .map(|err| {
            let err = err.write_errors();
            quote! {{
                #err
                #output
            }}
        })
        .unwrap_or(output)
}
