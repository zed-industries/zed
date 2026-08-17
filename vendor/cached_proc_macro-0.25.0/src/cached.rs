use crate::helpers::*;
use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use std::cmp::PartialEq;
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_str, Block, Ident, ItemFn, ReturnType, Type};

#[derive(Debug, Default, FromMeta, Eq, PartialEq)]
enum SyncWriteMode {
    #[default]
    Default,
    ByKey,
}

#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    unbound: bool,
    #[darling(default)]
    size: Option<usize>,
    #[darling(default)]
    time: Option<u64>,
    #[darling(default)]
    time_refresh: bool,
    #[darling(default)]
    key: Option<String>,
    #[darling(default)]
    convert: Option<String>,
    #[darling(default)]
    result: bool,
    #[darling(default)]
    option: bool,
    #[darling(default)]
    sync_writes: Option<SyncWriteMode>,
    #[darling(default)]
    with_cached_flag: bool,
    #[darling(default)]
    ty: Option<String>,
    #[darling(default)]
    create: Option<String>,
    #[darling(default)]
    result_fallback: bool,
}

pub fn cached(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let input = parse_macro_input!(input as ItemFn);

    // pull out the parts of the input
    let mut attributes = input.attrs;
    let visibility = input.vis;
    let signature = input.sig;
    let body = input.block;

    // pull out the parts of the function signature
    let fn_ident = signature.ident.clone();
    let inputs = signature.inputs.clone();
    let output = signature.output.clone();
    let asyncness = signature.asyncness;
    let generics = signature.generics.clone();

    let input_tys = get_input_types(&inputs);
    let input_names = get_input_names(&inputs);

    // pull out the output type
    let output_ty = match &output {
        ReturnType::Default => quote! {()},
        ReturnType::Type(_, ty) => quote! {#ty},
    };

    let output_span = output_ty.span();
    let output_ts = TokenStream::from(output_ty.clone());
    let output_parts = get_output_parts(&output_ts);
    let output_string = output_parts.join("::");
    let output_type_display = output_ts.to_string().replace(' ', "");

    if check_with_cache_flag(args.with_cached_flag, output_string) {
        return with_cache_flag_error(output_span, output_type_display);
    }

    let cache_value_ty = find_value_type(args.result, args.option, &output, output_ty);

    // make the cache identifier
    let cache_ident = match args.name {
        Some(ref name) => Ident::new(name, fn_ident.span()),
        None => Ident::new(&fn_ident.to_string().to_uppercase(), fn_ident.span()),
    };

    let (cache_key_ty, key_convert_block) =
        make_cache_key_type(&args.key, &args.convert, &args.ty, input_tys, &input_names);

    // make the cache type and create statement
    let (cache_ty, cache_create) = match (
        &args.unbound,
        &args.size,
        &args.time,
        &args.ty,
        &args.create,
        &args.time_refresh,
    ) {
        (true, None, None, None, None, _) => {
            let cache_ty = quote! {cached::UnboundCache<#cache_key_ty, #cache_value_ty>};
            let cache_create = quote! {cached::UnboundCache::new()};
            (cache_ty, cache_create)
        }
        (false, Some(size), None, None, None, _) => {
            let cache_ty = quote! {cached::SizedCache<#cache_key_ty, #cache_value_ty>};
            let cache_create = quote! {cached::SizedCache::with_size(#size)};
            (cache_ty, cache_create)
        }
        (false, None, Some(time), None, None, time_refresh) => {
            let cache_ty = quote! {cached::TimedCache<#cache_key_ty, #cache_value_ty>};
            let cache_create = quote! {cached::TimedCache::with_lifespan_and_refresh(Duration::from_secs(#time), #time_refresh)};
            (cache_ty, cache_create)
        }
        (false, Some(size), Some(time), None, None, time_refresh) => {
            let cache_ty = quote! {cached::TimedSizedCache<#cache_key_ty, #cache_value_ty>};
            let cache_create = quote! {cached::TimedSizedCache::with_size_and_lifespan_and_refresh(#size, Duration::from_secs(#time), #time_refresh)};
            (cache_ty, cache_create)
        }
        (false, None, None, None, None, _) => {
            let cache_ty = quote! {cached::UnboundCache<#cache_key_ty, #cache_value_ty>};
            let cache_create = quote! {cached::UnboundCache::new()};
            (cache_ty, cache_create)
        }
        (false, None, None, Some(type_str), Some(create_str), _) => {
            let ty = parse_str::<Type>(type_str).expect("unable to parse cache type");

            let cache_create =
                parse_str::<Block>(create_str).expect("unable to parse cache create block");

            (quote! { #ty }, quote! { #cache_create })
        }
        (false, None, None, Some(_), None, _) => {
            panic!("type requires create to also be set")
        }
        (false, None, None, None, Some(_), _) => {
            panic!("create requires type to also be set")
        }
        _ => panic!(
            "cache types (unbound, size and/or time, or type and create) are mutually exclusive"
        ),
    };

    // make the set cache and return cache blocks
    let (set_cache_block, return_cache_block) = match (&args.result, &args.option) {
        (false, false) => {
            let set_cache_block = quote! { cache.cache_set(key, result.clone()); };
            let return_cache_block = if args.with_cached_flag {
                quote! { let mut r = result.to_owned(); r.was_cached = true; return r }
            } else {
                quote! { return result.to_owned() }
            };
            (set_cache_block, return_cache_block)
        }
        (true, false) => {
            let set_cache_block = quote! {
                if let Ok(result) = &result {
                    cache.cache_set(key, result.clone());
                }
            };
            let return_cache_block = if args.with_cached_flag {
                quote! { let mut r = result.to_owned(); r.was_cached = true; return Ok(r) }
            } else {
                quote! { return Ok(result.to_owned()) }
            };
            (set_cache_block, return_cache_block)
        }
        (false, true) => {
            let set_cache_block = quote! {
                if let Some(result) = &result {
                    cache.cache_set(key, result.clone());
                }
            };
            let return_cache_block = if args.with_cached_flag {
                quote! { let mut r = result.to_owned(); r.was_cached = true; return Some(r) }
            } else {
                quote! { return Some(result.clone()) }
            };
            (set_cache_block, return_cache_block)
        }
        _ => panic!("the result and option attributes are mutually exclusive"),
    };

    if args.result_fallback && args.sync_writes.is_some() {
        panic!("result_fallback and sync_writes are mutually exclusive");
    }

    let set_cache_and_return = quote! {
        #set_cache_block
        result
    };

    let no_cache_fn_ident = Ident::new(&format!("{}_no_cache", &fn_ident), fn_ident.span());

    let lock;
    let function_no_cache;
    let function_call;
    let ty;
    if asyncness.is_some() {
        lock = match args.sync_writes {
            Some(SyncWriteMode::ByKey) => quote! {
                let mut locks = #cache_ident.lock().await;
                let lock = locks
                    .entry(key.clone())
                    .or_insert_with(|| std::sync::Arc::new(::cached::async_sync::Mutex::new(#cache_create)))
                    .clone();
                drop(locks);
                let mut cache = lock.lock().await;
            },
            _ => quote! {
                let mut cache = #cache_ident.lock().await;
            },
        };

        function_no_cache = quote! {
            async fn #no_cache_fn_ident #generics (#inputs) #output #body
        };

        function_call = quote! {
            let result = #no_cache_fn_ident(#(#input_names),*).await;
        };

        ty = match args.sync_writes {
            Some(SyncWriteMode::ByKey) => quote! {
                #visibility static #cache_ident: ::cached::once_cell::sync::Lazy<::cached::async_sync::Mutex<std::collections::HashMap<#cache_key_ty, std::sync::Arc<::cached::async_sync::Mutex<#cache_ty>>>>> = ::cached::once_cell::sync::Lazy::new(|| ::cached::async_sync::Mutex::new(std::collections::HashMap::new()));
            },
            _ => quote! {
                #visibility static #cache_ident: ::cached::once_cell::sync::Lazy<::cached::async_sync::Mutex<#cache_ty>> = ::cached::once_cell::sync::Lazy::new(|| ::cached::async_sync::Mutex::new(#cache_create));
            },
        };
    } else {
        lock = match args.sync_writes {
            Some(SyncWriteMode::ByKey) => quote! {
                let mut locks = #cache_ident.lock().unwrap();
                let lock = locks.entry(key.clone()).or_insert_with(|| std::sync::Arc::new(std::sync::Mutex::new(#cache_create))).clone();
                drop(locks);
                let mut cache = lock.lock().unwrap();
            },
            _ => quote! {
                let mut cache = #cache_ident.lock().unwrap();
            },
        };

        function_no_cache = quote! {
            fn #no_cache_fn_ident #generics (#inputs) #output #body
        };

        function_call = quote! {
            let result = #no_cache_fn_ident(#(#input_names),*);
        };

        ty = match args.sync_writes {
            Some(SyncWriteMode::ByKey) => quote! {
                #visibility static #cache_ident: ::cached::once_cell::sync::Lazy<std::sync::Mutex<std::collections::HashMap<#cache_key_ty, std::sync::Arc<std::sync::Mutex<#cache_ty>>>>> = ::cached::once_cell::sync::Lazy::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));
            },
            _ => quote! {
                #visibility static #cache_ident: ::cached::once_cell::sync::Lazy<std::sync::Mutex<#cache_ty>> = ::cached::once_cell::sync::Lazy::new(|| std::sync::Mutex::new(#cache_create));
            },
        }
    }

    let prime_do_set_return_block = quote! {
        // try to get a lock first
        #lock
        // run the function and cache the result
        #function_call
        #set_cache_and_return
    };

    let do_set_return_block = if args.sync_writes.is_some() {
        quote! {
            #lock
            if let Some(result) = cache.cache_get(&key) {
                #return_cache_block
            }
            #function_call
            #set_cache_and_return
        }
    } else if args.result_fallback {
        quote! {
            let old_val = {
                #lock
                let (result, has_expired) = cache.cache_get_expired(&key);
                if let (Some(result), false) = (&result, has_expired) {
                    #return_cache_block
                }
                result
            };
            #function_call
            #lock
            let result = match (result.is_err(), old_val) {
                (true, Some(old_val)) => {
                    Ok(old_val)
                }
                _ => result
            };
            #set_cache_and_return
        }
    } else {
        quote! {
            {
                #lock
                if let Some(result) = cache.cache_get(&key) {
                    #return_cache_block
                }
            }
            #function_call
            #lock
            #set_cache_and_return
        }
    };

    let signature_no_muts = get_mut_signature(signature);

    // create a signature for the cache-priming function
    let prime_fn_ident = Ident::new(&format!("{}_prime_cache", &fn_ident), fn_ident.span());
    let mut prime_sig = signature_no_muts.clone();
    prime_sig.ident = prime_fn_ident;

    // make cached static, cached function and prime cached function doc comments
    let cache_ident_doc = format!("Cached static for the [`{}`] function.", fn_ident);
    let no_cache_fn_indent_doc = format!("Origin of the cached function [`{}`].", fn_ident);
    let prime_fn_indent_doc = format!("Primes the cached function [`{}`].", fn_ident);
    let cache_fn_doc_extra = format!(
        "This is a cached function that uses the [`{}`] cached static.",
        cache_ident
    );
    fill_in_attributes(&mut attributes, cache_fn_doc_extra);

    // put it all together
    let expanded = quote! {
        // Cached static
        #[doc = #cache_ident_doc]
        #ty
        // No cache function (origin of the cached function)
        #[doc = #no_cache_fn_indent_doc]
        #visibility #function_no_cache
        // Cached function
        #(#attributes)*
        #visibility #signature_no_muts {
            use cached::Cached;
            use cached::CloneCached;
            let key = #key_convert_block;
            #do_set_return_block
        }
        // Prime cached function
        #[doc = #prime_fn_indent_doc]
        #[allow(dead_code)]
        #(#attributes)*
        #visibility #prime_sig {
            use cached::Cached;
            let key = #key_convert_block;
            #prime_do_set_return_block
        }
    };

    expanded.into()
}
