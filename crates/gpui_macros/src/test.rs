use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::mem;
use syn::{
    self, Expr, ExprLit, FnArg, ItemFn, Lit, Meta, MetaList, PathSegment, Token, Type,
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
};

struct Args {
    seeds: Vec<u64>,
    max_retries: usize,
    max_iterations: usize,
    on_failure_fn_name: proc_macro2::TokenStream,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut seeds = Vec::<u64>::new();
        let mut max_retries = 0;
        let mut max_iterations = 1;
        let mut on_failure_fn_name = quote!(None);

        let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

        for meta in metas {
            let ident = {
                let meta_path = match &meta {
                    Meta::NameValue(meta) => &meta.path,
                    Meta::List(list) => &list.path,
                    Meta::Path(path) => {
                        return Err(syn::Error::new(path.span(), "invalid path argument"));
                    }
                };
                let Some(ident) = meta_path.get_ident() else {
                    return Err(syn::Error::new(meta_path.span(), "unexpected path"));
                };
                ident.to_string()
            };

            match (&meta, ident.as_str()) {
                (Meta::NameValue(meta), "retries") => {
                    max_retries = parse_usize_from_expr(&meta.value)?
                }
                (Meta::NameValue(meta), "iterations") => {
                    max_iterations = parse_usize_from_expr(&meta.value)?
                }
                (Meta::NameValue(meta), "on_failure") => {
                    let Expr::Lit(ExprLit {
                        lit: Lit::Str(name),
                        ..
                    }) = &meta.value
                    else {
                        return Err(syn::Error::new(
                            meta.value.span(),
                            "on_failure argument must be a string",
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
                (Meta::NameValue(meta), "seed") => {
                    seeds = vec![parse_usize_from_expr(&meta.value)? as u64]
                }
                (Meta::List(list), "seeds") => seeds = parse_u64_array(list)?,
                (Meta::Path(_), _) => {
                    return Err(syn::Error::new(meta.span(), "invalid path argument"));
                }
                (_, _) => {
                    return Err(syn::Error::new(meta.span(), "invalid argument name"));
                }
            }
        }

        Ok(Args {
            seeds,
            max_retries,
            max_iterations,
            on_failure_fn_name,
        })
    }
}

pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as Args);
    let mut inner_fn = match syn::parse::<ItemFn>(function) {
        Ok(f) => f,
        Err(err) => return error_to_stream(err),
    };

    let inner_fn_attributes = mem::take(&mut inner_fn.attrs);
    let inner_fn_name = format_ident!("_{}", inner_fn.sig.ident);
    let outer_fn_name = mem::replace(&mut inner_fn.sig.ident, inner_fn_name.clone());

    let result = generate_test_function(
        args,
        inner_fn,
        inner_fn_attributes,
        inner_fn_name,
        outer_fn_name,
    );
    match result {
        Ok(tokens) => tokens,
        Err(tokens) => tokens,
    }
}

fn generate_test_function(
    args: Args,
    inner_fn: ItemFn,
    inner_fn_attributes: Vec<syn::Attribute>,
    inner_fn_name: Ident,
    outer_fn_name: Ident,
) -> Result<TokenStream, TokenStream> {
    let seeds = &args.seeds;
    let max_retries = args.max_retries;
    let num_iterations = args.max_iterations;
    let on_failure_fn_name = &args.on_failure_fn_name;
    let seeds = quote!( #(#seeds),* );

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
                } else if let Type::Reference(ty) = &*arg.ty
                    && let Type::Path(ty) = &*ty.elem
                {
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
                            #cx_varname.executor().forbid_parking();
                            #cx_varname.quit();
                            dispatcher.run_until_parked();
                        ));
                        inner_fn_args.extend(quote!(&mut #cx_varname,));
                        continue;
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
                } else if let Type::Reference(ty) = &*arg.ty
                    && let Type::Path(ty) = &*ty.elem
                {
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
                                    #cx_varname.update(|cx| { cx.background_executor().forbid_parking(); cx.quit(); });
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
                                #cx_varname.executor().forbid_parking();
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

fn parse_usize_from_expr(expr: &Expr) -> Result<usize, syn::Error> {
    let Expr::Lit(ExprLit {
        lit: Lit::Int(int), ..
    }) = expr
    else {
        return Err(syn::Error::new(expr.span(), "expected an integer"));
    };
    int.base10_parse()
        .map_err(|_| syn::Error::new(int.span(), "failed to parse integer"))
}

fn parse_u64_array(meta_list: &MetaList) -> Result<Vec<u64>, syn::Error> {
    let mut result = Vec::new();
    let tokens = &meta_list.tokens;
    let parser = |input: ParseStream| {
        let exprs = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        for expr in exprs {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(int), ..
            }) = expr
            {
                let value: usize = int.base10_parse()?;
                result.push(value as u64);
            } else {
                return Err(syn::Error::new(expr.span(), "expected an integer"));
            }
        }
        Ok(())
    };
    syn::parse::Parser::parse2(parser, tokens.clone())?;
    Ok(result)
}

fn error_with_message(message: &str, spanned: impl Spanned) -> TokenStream {
    error_to_stream(syn::Error::new(spanned.span(), message))
}

fn error_to_stream(err: syn::Error) -> TokenStream {
    TokenStream::from(err.into_compile_error())
}
