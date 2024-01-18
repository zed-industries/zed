use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::mem;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned as _, AttributeArgs, FnArg, ItemFn, Lit, Meta,
    NestedMeta, Type,
};

pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as AttributeArgs);
    let mut max_retries = 0;
    let mut num_iterations = 1;
    let mut on_failure_fn_name = quote!(None);

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(meta)) => {
                let key_name = meta.path.get_ident().map(|i| i.to_string());
                let result = (|| {
                    match key_name.as_deref() {
                        Some("retries") => max_retries = parse_int(&meta.lit)?,
                        Some("iterations") => num_iterations = parse_int(&meta.lit)?,
                        Some("on_failure") => {
                            if let Lit::Str(name) = meta.lit {
                                let mut path = syn::Path {
                                    leading_colon: None,
                                    segments: Default::default(),
                                };
                                for part in name.value().split("::") {
                                    path.segments.push(Ident::new(part, name.span()).into());
                                }
                                on_failure_fn_name = quote!(Some(#path));
                            } else {
                                return Err(TokenStream::from(
                                    syn::Error::new(
                                        meta.lit.span(),
                                        "on_failure argument must be a string",
                                    )
                                    .into_compile_error(),
                                ));
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
        let mut cx_vars = proc_macro2::TokenStream::new();
        let mut cx_teardowns = proc_macro2::TokenStream::new();
        let mut inner_fn_args = proc_macro2::TokenStream::new();
        for (ix, arg) in inner_fn.sig.inputs.iter().enumerate() {
            if let FnArg::Typed(arg) = arg {
                if let Type::Path(ty) = &*arg.ty {
                    let last_segment = ty.path.segments.last();
                    match last_segment.map(|s| s.ident.to_string()).as_deref() {
                        Some("StdRng") => {
                            inner_fn_args.extend(quote!(rand::SeedableRng::seed_from_u64(_seed),));
                            continue;
                        }
                        Some("BackgroundExecutor") => {
                            inner_fn_args.extend(quote!(gpui::BackgroundExecutor::new(
                                std::sync::Arc::new(dispatcher.clone()),
                            ),));
                            continue;
                        }
                        _ => {}
                    }
                } else if let Type::Reference(ty) = &*arg.ty {
                    if let Type::Path(ty) = &*ty.elem {
                        let last_segment = ty.path.segments.last();
                        if let Some("TestAppContext") =
                            last_segment.map(|s| s.ident.to_string()).as_deref()
                        {
                            let cx_varname = format_ident!("cx_{}", ix);
                            cx_vars.extend(quote!(
                                let mut #cx_varname = gpui::TestAppContext::new(
                                    dispatcher.clone(),
                                    Some(stringify!(#outer_fn_name)),
                                );
                            ));
                            cx_teardowns.extend(quote!(
                                dispatcher.run_until_parked();
                                #cx_varname.quit();
                                dispatcher.run_until_parked();
                            ));
                            inner_fn_args.extend(quote!(&mut #cx_varname,));
                            continue;
                        }
                    }
                }
            }

            return TokenStream::from(
                syn::Error::new_spanned(arg, "invalid argument").into_compile_error(),
            );
        }

        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                gpui::run_test(
                    #num_iterations as u64,
                    #max_retries,
                    &mut |dispatcher, _seed| {
                        let executor = gpui::BackgroundExecutor::new(std::sync::Arc::new(dispatcher.clone()));
                        #cx_vars
                        executor.block_test(#inner_fn_name(#inner_fn_args));
                        #cx_teardowns
                    },
                    #on_failure_fn_name
                );
            }
        }
    } else {
        // Pass to the test function the number of app contexts that it needs,
        // based on its parameter list.
        let mut cx_vars = proc_macro2::TokenStream::new();
        let mut cx_teardowns = proc_macro2::TokenStream::new();
        let mut inner_fn_args = proc_macro2::TokenStream::new();
        for (ix, arg) in inner_fn.sig.inputs.iter().enumerate() {
            if let FnArg::Typed(arg) = arg {
                if let Type::Path(ty) = &*arg.ty {
                    let last_segment = ty.path.segments.last();

                    if let Some("StdRng") = last_segment.map(|s| s.ident.to_string()).as_deref() {
                        inner_fn_args.extend(quote!(rand::SeedableRng::seed_from_u64(_seed),));
                        continue;
                    }
                } else if let Type::Reference(ty) = &*arg.ty {
                    if let Type::Path(ty) = &*ty.elem {
                        let last_segment = ty.path.segments.last();
                        match last_segment.map(|s| s.ident.to_string()).as_deref() {
                            Some("AppContext") => {
                                let cx_varname = format_ident!("cx_{}", ix);
                                let cx_varname_lock = format_ident!("cx_{}_lock", ix);
                                cx_vars.extend(quote!(
                                    let mut #cx_varname = gpui::TestAppContext::new(
                                       dispatcher.clone(),
                                       Some(stringify!(#outer_fn_name))
                                    );
                                    let mut #cx_varname_lock = #cx_varname.app.borrow_mut();
                                ));
                                inner_fn_args.extend(quote!(&mut #cx_varname_lock,));
                                cx_teardowns.extend(quote!(
                                    drop(#cx_varname_lock);
                                    dispatcher.run_until_parked();
                                    #cx_varname.update(|cx| { cx.quit() });
                                    dispatcher.run_until_parked();
                                ));
                                continue;
                            }
                            Some("TestAppContext") => {
                                let cx_varname = format_ident!("cx_{}", ix);
                                cx_vars.extend(quote!(
                                    let mut #cx_varname = gpui::TestAppContext::new(
                                        dispatcher.clone(),
                                        Some(stringify!(#outer_fn_name))
                                    );
                                ));
                                cx_teardowns.extend(quote!(
                                    dispatcher.run_until_parked();
                                    #cx_varname.quit();
                                    dispatcher.run_until_parked();
                                ));
                                inner_fn_args.extend(quote!(&mut #cx_varname,));
                                continue;
                            }
                            _ => {}
                        }
                    }
                }
            }

            return TokenStream::from(
                syn::Error::new_spanned(arg, "invalid argument").into_compile_error(),
            );
        }

        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                gpui::run_test(
                    #num_iterations as u64,
                    #max_retries,
                    &mut |dispatcher, _seed| {
                        #cx_vars
                        #inner_fn_name(#inner_fn_args);
                        #cx_teardowns
                    },
                    #on_failure_fn_name,
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
