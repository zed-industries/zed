use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::mem;
use syn::{
    parse_macro_input, parse_quote, AttributeArgs, ItemFn, Lit, Meta, MetaNameValue, NestedMeta,
};

#[proc_macro_attribute]
pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    let mut namespace = format_ident!("gpui");

    let args = syn::parse_macro_input!(args as AttributeArgs);
    let mut max_retries = 0;
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::Path(name))
                if name.get_ident().map_or(false, |n| n == "self") =>
            {
                namespace = format_ident!("crate");
            }
            NestedMeta::Meta(Meta::NameValue(meta)) => {
                if let Some(result) = parse_retries(&meta) {
                    match result {
                        Ok(retries) => max_retries = retries,
                        Err(error) => return TokenStream::from(error.into_compile_error()),
                    }
                }
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

                if #max_retries > 0 {
                    let mut retries = 0;
                    loop {
                        let result = std::panic::catch_unwind(|| {
                            #namespace::App::test_async((), move |cx| async {
                                #inner_fn_name(cx).await;
                            });
                        });

                        match result {
                            Ok(result) => return result,
                            Err(error) => {
                                if retries < #max_retries {
                                    retries += 1;
                                    println!("retrying: attempt {}", retries);
                                } else {
                                    std::panic::resume_unwind(error);
                                }
                            }
                        }
                    }
                } else {
                    #namespace::App::test_async((), move |cx| async {
                        #inner_fn_name(cx).await;
                    });
                }
            }
        }
    } else {
        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                if #max_retries > 0 {
                    let mut retries = 0;
                    loop {
                        let result = std::panic::catch_unwind(|| {
                            #namespace::App::test((), |cx| {
                                #inner_fn_name(cx);
                            });
                        });

                        match result {
                            Ok(result) => return result,
                            Err(error) => {
                                if retries < #max_retries {
                                    retries += 1;
                                    println!("retrying: attempt {}", retries);
                                } else {
                                    std::panic::resume_unwind(error);
                                }
                            }
                        }
                    }
                } else {
                    #namespace::App::test((), |cx| {
                        #inner_fn_name(cx);
                    });
                }
            }
        }
    };
    outer_fn.attrs.extend(inner_fn_attributes);

    TokenStream::from(quote!(#outer_fn))
}

fn parse_retries(meta: &MetaNameValue) -> Option<syn::Result<usize>> {
    let ident = meta.path.get_ident();
    if ident.map_or(false, |n| n == "retries") {
        if let Lit::Int(int) = &meta.lit {
            Some(int.base10_parse())
        } else {
            Some(Err(syn::Error::new(
                meta.lit.span(),
                "retries mut be an integer",
            )))
        }
    } else {
        None
    }
}
