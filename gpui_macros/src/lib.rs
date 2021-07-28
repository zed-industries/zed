use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::mem;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned as _, AttributeArgs, FnArg, ItemFn, Lit, Meta,
    NestedMeta, Type,
};

#[proc_macro_attribute]
pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    let mut namespace = format_ident!("gpui");

    let args = syn::parse_macro_input!(args as AttributeArgs);
    let mut max_retries = 0;
    let mut num_iterations = 1;
    let mut starting_seed = std::env::var("SEED")
        .map(|i| i.parse().expect("invalid `SEED`"))
        .ok();

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::Path(name))
                if name.get_ident().map_or(false, |n| n == "self") =>
            {
                namespace = format_ident!("crate");
            }
            NestedMeta::Meta(Meta::NameValue(meta)) => {
                let key_name = meta.path.get_ident().map(|i| i.to_string());
                let result = (|| {
                    match key_name.as_ref().map(String::as_str) {
                        Some("retries") => max_retries = parse_int(&meta.lit)?,
                        Some("iterations") => {
                            if let Ok(iters) = std::env::var("ITERATIONS") {
                                num_iterations = iters.parse().expect("invalid `ITERATIONS`");
                            } else {
                                num_iterations = parse_int(&meta.lit)?;
                            }
                        }
                        Some("seed") => {
                            if starting_seed.is_none() {
                                starting_seed = Some(parse_int(&meta.lit)?);
                            }
                        }
                        _ => {
                            return Err(TokenStream::from(
                                syn::Error::new(meta.path.span(), "invalid argument")
                                    .into_compile_error(),
                            ))
                        }
                    }
                    Ok(())
                })();

                if let Err(tokens) = result {
                    return tokens;
                }
            }
            other => {
                return TokenStream::from(
                    syn::Error::new_spanned(other, "invalid argument").into_compile_error(),
                )
            }
        }
    }
    let starting_seed = starting_seed.unwrap_or(0);

    let mut inner_fn = parse_macro_input!(function as ItemFn);
    let inner_fn_attributes = mem::take(&mut inner_fn.attrs);
    let inner_fn_name = format_ident!("_{}", inner_fn.sig.ident);
    let outer_fn_name = mem::replace(&mut inner_fn.sig.ident, inner_fn_name.clone());

    let mut outer_fn: ItemFn = if inner_fn.sig.asyncness.is_some() {
        // Pass to the test function the number of app contexts that it needs,
        // based on its parameter list.
        let mut inner_fn_args = proc_macro2::TokenStream::new();
        for (ix, arg) in inner_fn.sig.inputs.iter().enumerate() {
            if let FnArg::Typed(arg) = arg {
                if let Type::Path(ty) = &*arg.ty {
                    let last_segment = ty.path.segments.last();
                    match last_segment.map(|s| s.ident.to_string()).as_deref() {
                        Some("TestAppContext") => {
                            let first_entity_id = ix * 100_000;
                            inner_fn_args.extend(
                            quote!(#namespace::TestAppContext::new(foreground.clone(), background.clone(), #first_entity_id),)
                        );
                        }
                        Some("StdRng") => {
                            inner_fn_args.extend(quote!(rand::SeedableRng::seed_from_u64(seed)));
                        }
                        _ => {
                            return TokenStream::from(
                                syn::Error::new_spanned(arg, "invalid argument")
                                    .into_compile_error(),
                            )
                        }
                    }
                } else {
                    return TokenStream::from(
                        syn::Error::new_spanned(arg, "invalid argument").into_compile_error(),
                    );
                }
            } else {
                return TokenStream::from(
                    syn::Error::new_spanned(arg, "invalid argument").into_compile_error(),
                );
            }
        }

        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                let mut retries = 0;
                let mut i = 0;
                loop {
                    let seed = (#starting_seed + i) as u64;
                    dbg!(seed);
                    let result = std::panic::catch_unwind(|| {
                        let (foreground, background) = #namespace::executor::deterministic(seed);
                        foreground.run(#inner_fn_name(#inner_fn_args));
                    });

                    match result {
                        Ok(result) => {
                            retries = 0;
                            i += 1;
                            if i == #num_iterations {
                                return result
                            }
                        }
                        Err(error) => {
                            if retries < #max_retries {
                                retries += 1;
                                println!("retrying: attempt {}", retries);
                            } else {
                                if #num_iterations > 1 {
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
        let mut inner_fn_args = proc_macro2::TokenStream::new();
        for arg in inner_fn.sig.inputs.iter() {
            if let FnArg::Typed(arg) = arg {
                if let Type::Path(ty) = &*arg.ty {
                    let last_segment = ty.path.segments.last();
                    if let Some("StdRng") = last_segment.map(|s| s.ident.to_string()).as_deref() {
                        inner_fn_args.extend(quote!(rand::SeedableRng::seed_from_u64(seed),));
                    }
                } else {
                    inner_fn_args.extend(quote!(cx,));
                }
            } else {
                return TokenStream::from(
                    syn::Error::new_spanned(arg, "invalid argument").into_compile_error(),
                );
            }
        }

        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                let mut retries = 0;
                let mut i = 0;
                loop {
                    let seed = (#starting_seed + i) as u64;
                    dbg!(seed);
                    let result = std::panic::catch_unwind(|| {
                        #namespace::App::test(|cx| {
                            #inner_fn_name(#inner_fn_args);
                        });
                    });

                    match result {
                        Ok(result) => {
                            retries = 0;
                            i += 1;
                            if i == #num_iterations {
                                return result
                            }
                        }
                        Err(error) => {
                            if retries < #max_retries {
                                retries += 1;
                                println!("retrying: attempt {}", retries);
                            } else {
                                if #num_iterations > 1 {
                                    eprintln!("failing seed: {}", seed);
                                }
                                std::panic::resume_unwind(error);
                            }
                        }
                    }
                }
            }
        }
    };
    outer_fn.attrs.extend(inner_fn_attributes);

    TokenStream::from(quote!(#outer_fn))
}

fn parse_int(literal: &Lit) -> Result<usize, TokenStream> {
    let result = if let Lit::Int(int) = &literal {
        int.base10_parse()
    } else {
        Err(syn::Error::new(literal.span(), "must be an integer"))
    };

    result.map_err(|err| TokenStream::from(err.into_compile_error()))
}
