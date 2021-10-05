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
    let mut starting_seed = 0;

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
                        Some("iterations") => num_iterations = parse_int(&meta.lit)?,
                        Some("seed") => starting_seed = parse_int(&meta.lit)?,
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

    let mut inner_fn = parse_macro_input!(function as ItemFn);
    if max_retries > 0 && num_iterations > 1 {
        return TokenStream::from(
            syn::Error::new_spanned(inner_fn, "retries and randomized iterations can't be mixed")
                .into_compile_error(),
        );
    }
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
                            inner_fn_args.extend(quote!(
                                #namespace::TestAppContext::new(
                                    foreground_platform.clone(),
                                    platform.clone(),
                                    foreground.clone(),
                                    background.clone(),
                                    font_cache.clone(),
                                    #first_entity_id,
                                ),
                            ));
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

                let is_randomized = #num_iterations > 1;
                let mut num_iterations = #num_iterations as u64;
                let mut starting_seed = #starting_seed as u64;
                if is_randomized {
                    if let Ok(value) = std::env::var("SEED") {
                        starting_seed = value.parse().expect("invalid SEED variable");
                    }
                    if let Ok(value) = std::env::var("ITERATIONS") {
                        num_iterations = value.parse().expect("invalid ITERATIONS variable");
                    }
                }

                let mut atomic_seed = std::sync::atomic::AtomicU64::new(starting_seed as u64);
                let mut retries = 0;

                loop {
                    let result = std::panic::catch_unwind(|| {
                        let foreground_platform = std::rc::Rc::new(#namespace::platform::test::foreground_platform());
                        let platform = std::sync::Arc::new(#namespace::platform::test::platform());
                        let font_system = #namespace::Platform::fonts(platform.as_ref());
                        let font_cache = std::sync::Arc::new(#namespace::FontCache::new(font_system));

                        loop {
                            let seed = atomic_seed.load(std::sync::atomic::Ordering::SeqCst);
                            if seed >= starting_seed + num_iterations {
                                break;
                            }

                            if is_randomized {
                                dbg!(seed);
                            }

                            let (foreground, background) = #namespace::executor::deterministic(seed);
                            foreground.run(#inner_fn_name(#inner_fn_args));
                            atomic_seed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        }
                    });

                    match result {
                        Ok(result) => {
                            break;
                        }
                        Err(error) => {
                            if retries < #max_retries {
                                retries += 1;
                                println!("retrying: attempt {}", retries);
                            } else {
                                if is_randomized {
                                    eprintln!("failing seed: {}", atomic_seed.load(std::sync::atomic::Ordering::SeqCst));
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

                #namespace::test::run_sync_test(
                    #num_iterations as u64,
                    #starting_seed as u64,
                    #max_retries,
                    &mut |cx, seed| #inner_fn_name(#inner_fn_args)
                );
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
