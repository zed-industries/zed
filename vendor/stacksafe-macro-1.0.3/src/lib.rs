// Copyright 2025 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Procedural macro implementation for the `stacksafe` crate.
//!
//! This crate provides the `#[stacksafe]` attribute macro that transforms functions
//! to use automatic stack growth, preventing stack overflow in deeply recursive scenarios.

use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::Item;
use syn::Path;
use syn::ReturnType;
use syn::Type;
use syn::parse_quote;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn stacksafe(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = TokenStream::from(args);
    let item = TokenStream::from(item);
    match stacksafe_impl(args, item) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn stacksafe_impl(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let mut crate_path: Option<Path> = None;
    let arg_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("crate") {
            if crate_path.is_some() {
                return Err(meta.error("duplicate attribute parameter `crate`"));
            }
            crate_path = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error(format!(
                "unknown attribute parameter `{}`",
                meta.path.to_token_stream()
            )))
        }
    });
    syn::parse::Parser::parse2(arg_parser, args)?;

    let mut item_fn = match syn::parse2::<Item>(item)? {
        Item::Fn(item_fn) => item_fn,
        item => {
            return Err(syn::Error::new_spanned(
                item,
                "#[stacksafe] can only be applied to functions",
            ));
        }
    };

    if item_fn.sig.asyncness.is_some() {
        return Err(syn::Error::new(
            item_fn.sig.asyncness.span(),
            "#[stacksafe] does not support async functions",
        ));
    }

    if item_fn.sig.constness.is_some() {
        return Err(syn::Error::new(
            item_fn.sig.constness.span(),
            "#[stacksafe] does not support const functions",
        ));
    }

    let ret = match &item_fn.sig.output {
        // Closures cannot use `impl Trait` return types, so omit the return
        // type and let the compiler infer it.
        ReturnType::Type(_, ty) if matches!(**ty, Type::ImplTrait(_)) => None,
        ret => Some(ret),
    };

    let stacksafe_crate = crate_path.unwrap_or_else(|| parse_quote!(::stacksafe));
    let block = &item_fn.block;
    let wrapped_block = quote! {
        {
            #stacksafe_crate::internal::stacker::maybe_grow(
                #stacksafe_crate::get_minimum_stack_size(),
                #stacksafe_crate::get_stack_allocation_size(),
                #stacksafe_crate::internal::with_protected(move || #ret { #block })
            )
        }
    };

    *item_fn.block = syn::parse2(wrapped_block)?;
    Ok(item_fn.into_token_stream())
}
