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
    let mut iterations = 1;
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::Path(name))
                if name.get_ident().map_or(false, |n| n == "self") =>
            {
                namespace = format_ident!("crate");
            }
            NestedMeta::Meta(Meta::NameValue(meta)) => {
                if let Some(result) = parse_int_meta(&meta, "retries") {
                    match result {
                        Ok(value) => max_retries = value,
                        Err(error) => return TokenStream::from(error.into_compile_error()),
                    }
                } else if let Some(result) = parse_int_meta(&meta, "iterations") {
                    match result {
                        Ok(value) => iterations = value,
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

    // Pass to the test function the number of app contexts that it needs,
    // based on its parameter list.
    let inner_fn_args = (0..inner_fn.sig.inputs.len())
        .map(|i| {
            let first_entity_id = i * 100_000;
            quote!(#namespace::TestAppContext::new(foreground.clone(), background.clone(), #first_entity_id),)
        })
        .collect::<proc_macro2::TokenStream>();

    let mut outer_fn: ItemFn = if inner_fn.sig.asyncness.is_some() {
        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                let mut retries = 0;
                let mut seed = 0;
                loop {
                    let result = std::panic::catch_unwind(|| {
                        let (foreground, background) = #namespace::executor::deterministic(seed as u64);
                        foreground.run(#inner_fn_name(#inner_fn_args));
                    });

                    match result {
                        Ok(result) => {
                            seed += 1;
                            retries = 0;
                            if seed == #iterations {
                                return result
                            }
                        }
                        Err(error) => {
                            if retries < #max_retries {
                                retries += 1;
                                println!("retrying: attempt {}", retries);
                            } else {
                                if #iterations > 1 {
                                    eprintln!("failing seed: {}", seed);
                                }
                                std::panic::resume_unwind(error);
                            }
                        }
                    }
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
                            #namespace::App::test(|cx| {
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
                    #namespace::App::test(|cx| {
                        #inner_fn_name(cx);
                    });
                }
            }
        }
    };
    outer_fn.attrs.extend(inner_fn_attributes);

    TokenStream::from(quote!(#outer_fn))
}

fn parse_int_meta(meta: &MetaNameValue, name: &str) -> Option<syn::Result<usize>> {
    let ident = meta.path.get_ident();
    if ident.map_or(false, |n| n == name) {
        if let Lit::Int(int) = &meta.lit {
            Some(int.base10_parse())
        } else {
            Some(Err(syn::Error::new(
                meta.lit.span(),
                format!("{} mut be an integer", name),
            )))
        }
    } else {
        None
    }
}
