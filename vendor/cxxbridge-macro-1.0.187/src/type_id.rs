use crate::syntax::qualified::QualifiedName;
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::ext::IdentExt;

pub(crate) enum Crate {
    Cxx,
    DollarCrate(TokenTree),
}

impl ToTokens for Crate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Crate::Cxx => tokens.extend(quote!(::cxx)),
            Crate::DollarCrate(krate) => krate.to_tokens(tokens),
        }
    }
}

// "folly::File" => `(f, o, l, l, y, (), F, i, l, e)`
pub(crate) fn expand(krate: Crate, arg: QualifiedName) -> TokenStream {
    let mut ids = Vec::new();

    for word in arg.segments {
        if !ids.is_empty() {
            ids.push(quote!(()));
        }
        for ch in word.unraw().to_string().chars() {
            ids.push(match ch {
                'A'..='Z' | 'a'..='z' => {
                    let t = format_ident!("{}", ch);
                    quote!(#krate::#t)
                }
                '0'..='9' | '_' => {
                    let t = format_ident!("_{}", ch);
                    quote!(#krate::#t)
                }
                _ => quote!([(); #ch as _]),
            });
        }
    }

    quote! { (#(#ids,)*) }
}
