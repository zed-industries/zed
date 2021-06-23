use std::mem;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, AttributeArgs, ItemFn, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    let mut namespace = format_ident!("gpui");

    let args = syn::parse_macro_input!(args as AttributeArgs);
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::Path(name))
                if name.get_ident().map_or(false, |n| n == "self") =>
            {
                namespace = format_ident!("crate");
            }
            other => {
                return TokenStream::from(
                    syn::Error::new_spanned(other, "invalid argument").into_compile_error(),
                )
            }
        }
    }

    let mut inner_fn = parse_macro_input!(function as ItemFn);
    let inner_fn_attributes = mem::take(&mut inner_fn.attrs);
    let inner_fn_name = format_ident!("_{}", inner_fn.sig.ident);
    let outer_fn_name = mem::replace(&mut inner_fn.sig.ident, inner_fn_name.clone());
    let mut outer_fn: ItemFn = if inner_fn.sig.asyncness.is_some() {
        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                #namespace::App::test_async(move |cx| async {
                    #inner_fn_name(cx).await;
                });
            }
        }
    } else {
        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                #namespace::App::test(|cx| {
                    #inner_fn_name(cx);
                });
            }
        }
    };
    outer_fn.attrs.extend(inner_fn_attributes);

    TokenStream::from(quote!(#outer_fn))
}
