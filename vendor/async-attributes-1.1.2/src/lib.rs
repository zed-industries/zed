//! Experimental language-level polyfills for Async Rust.
//!
//! # Examples
//!
//! ```
//! #[async_attributes::main]
//! async fn main() {
//!     println!("Hello, world!");
//! }
//! ```
//!
//! # About
//!
//! Async Rust is a work in progress. The language has enabled us to do some
//! fantastic things, but not everything is figured out yet. This crate exists
//! to polyfill language-level support for async idioms before they can be part
//! of the language.
//!
//! A great example of this is `async fn main`, which we first introduced as
//! part of the [`runtime`](https://docs.rs/runtime/0.3.0-alpha.7/runtime/) crate.
//! Its premise is that if `async fn` is required for every `await` call, it
//! makes sense to apply that even to `fn main`. Unfortunately this would
//! require compiler support to enable, so we've provided an experimental
//! polyfill for it in the mean time.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

/// Enables an async main function.
///
/// # Examples
///
/// ```ignore
/// #[async_std::main]
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
#[cfg(not(test))] // NOTE: exporting main breaks tests, we should file an issue.
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if name != "main" {
        return TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[async_std::main]"),
        });
    }

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote! {
        #vis fn main() #ret {
            #(#attrs)*
            async fn main(#inputs) #ret {
                #body
            }

            async_std::task::block_on(async {
                main().await
            })
        }

    };

    result.into()
}

/// Enables an async test function.
///
/// # Examples
///
/// ```ignore
/// #[async_std::test]
/// async fn my_test() -> std::io::Result<()> {
///     assert_eq!(2 * 2, 4);
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote! {
        #[::core::prelude::v1::test]
        #(#attrs)*
        #vis fn #name() #ret {
            async_std::task::block_on(async { #body })
        }
    };

    result.into()
}

/// Enables an async benchmark function.
///
/// # Examples
///
/// ```ignore
/// #![feature(test)]
/// extern crate test;
///
/// #[async_std::bench]
/// async fn bench_1(b: &mut test::Bencher) {
///     b.iter(|| {
///         println!("hello world");
///     })
/// }
/// ```
#[proc_macro_attribute]
pub fn bench(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let args = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    if !args.is_empty() {
        return TokenStream::from(quote_spanned! { args.span() =>
            compile_error!("async benchmarks don't take any arguments"),
        });
    }

    let result = quote! {
        #[::core::prelude::v1::bench]
        #(#attrs)*
        #vis fn #name(b: &mut test::Bencher) #ret {
            task::block_on(task::spawn(async {
                #body
            }))
        }
    };

    result.into()
}
