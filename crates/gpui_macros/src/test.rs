use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::mem;
use syn::{
    AttributeArgs, FnArg, ItemFn, Lit, Meta, MetaList, NestedMeta, PathSegment, Type, parse_quote,
    spanned::Spanned,
};

pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as AttributeArgs);
    try_test(args, function).unwrap_or_else(|err| err)
}

fn try_test(args: Vec<NestedMeta>, function: TokenStream) -> Result<TokenStream, TokenStream> {
    let mut seeds = Vec::<u64>::new();
    let mut max_retries = 0;
    let mut num_iterations = 1;
    let mut on_failure_fn_name = quote!(None);

    for arg in args {
        let NestedMeta::Meta(arg) = arg else {
            return Err(error_with_message("unexpected literal", arg));
        };

        let ident = {
            let meta_path = match &arg {
                Meta::NameValue(meta) => &meta.path,
                Meta::List(list) => &list.path,
                Meta::Path(path) => return Err(error_with_message("invalid path argument", path)),
            };
            let Some(ident) = meta_path.get_ident() else {
                return Err(error_with_message("unexpected path", meta_path));
            };
            ident.to_string()
        };

        match (&arg, ident.as_str()) {
            (Meta::NameValue(meta), "retries") => max_retries = parse_usize(&meta.lit)?,
            (Meta::NameValue(meta), "iterations") => num_iterations = parse_usize(&meta.lit)?,
            (Meta::NameValue(meta), "on_failure") => {
                let Lit::Str(name) = &meta.lit else {
                    return Err(error_with_message(
                        "on_failure argument must be a string",
                        &meta.lit,
                    ));
                };
                let segments = name
                    .value()
                    .split("::")
                    .map(|part| PathSegment::from(Ident::new(part, name.span())))
                    .collect();
                let path = syn::Path {
                    leading_colon: None,
                    segments,
                };
                on_failure_fn_name = quote!(Some(#path));
            }
            (Meta::NameValue(meta), "seed") => seeds = vec![parse_usize(&meta.lit)? as u64],
            (Meta::List(list), "seeds") => seeds = parse_u64_array(&list)?,
            (Meta::Path(path), _) => {
                return Err(error_with_message("invalid path argument", path));
            }
            (_, _) => {
                return Err(error_with_message("invalid argument name", arg));
            }
        }
    }
    let seeds = quote!( #(#seeds),* );

    let mut inner_fn = syn::parse::<ItemFn>(function).map_err(error_to_stream)?;
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
                                let mut #cx_varname = gpui::TestAppContext::build(
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

            return Err(error_with_message("invalid function signature", arg));
        }

        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                gpui::run_test(
                    #num_iterations,
                    &[#seeds],
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
                            Some("App") => {
                                let cx_varname = format_ident!("cx_{}", ix);
                                let cx_varname_lock = format_ident!("cx_{}_lock", ix);
                                cx_vars.extend(quote!(
                                    let mut #cx_varname = gpui::TestAppContext::build(
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
                                    let mut #cx_varname = gpui::TestAppContext::build(
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

            return Err(error_with_message("invalid function signature", arg));
        }

        parse_quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                gpui::run_test(
                    #num_iterations,
                    &[#seeds],
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

    Ok(TokenStream::from(quote!(#outer_fn)))
}

fn parse_usize(literal: &Lit) -> Result<usize, TokenStream> {
    let Lit::Int(int) = &literal else {
        return Err(error_with_message("expected an usize", literal));
    };
    int.base10_parse().map_err(error_to_stream)
}

fn parse_u64_array(meta_list: &MetaList) -> Result<Vec<u64>, TokenStream> {
    meta_list
        .nested
        .iter()
        .map(|meta| {
            if let NestedMeta::Lit(literal) = &meta {
                parse_usize(literal).map(|value| value as u64)
            } else {
                Err(error_with_message("expected an integer", meta.span()))
            }
        })
        .collect()
}

fn error_with_message(message: &str, spanned: impl Spanned) -> TokenStream {
    error_to_stream(syn::Error::new(spanned.span(), message))
}

fn error_to_stream(err: syn::Error) -> TokenStream {
    TokenStream::from(err.into_compile_error())
}
