//! # interpolate_name
//!
//! `interpolate_name` consists in a set of procedural macro attributes
//! geared towards reduce the boilerplate while writing repetitive tests.
//!
//! - `interpolate_test`: a quick way to test the same function by passing specific
//! arguments and have a test entry for each of them.
//! - `interpolate_name`: a simple function renamer that can be combined
//! with macros to support more complex patterns.

extern crate proc_macro;

use syn::{parse_macro_input, Attribute, ItemFn};

use proc_macro2::{Ident, Span, TokenStream, TokenTree};

use quote::{quote_spanned, ToTokens};

fn fn_name(item: TokenStream) -> Ident {
    let mut tokens = item.into_iter();
    let found = tokens
        .find(|tok| {
            if let TokenTree::Ident(word) = tok {
                if word == "fn" {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .is_some();

    if !found {
        panic!("the macro attribute applies only to functions")
    }

    match tokens.next() {
        Some(TokenTree::Ident(word)) => word,
        _ => panic!("failed to find function name"),
    }
}

fn fn_attrs_name(item: TokenStream) -> (TokenStream, Ident) {
    let mut tokens = item.into_iter();

    let attrs = tokens
        .by_ref()
        .take_while(|tok| {
            if let TokenTree::Ident(word) = tok {
                if word == "fn" {
                    false
                } else {
                    true
                }
            } else {
                true
            }
        })
        .collect::<Vec<_>>();

    let name = match tokens.next() {
        Some(TokenTree::Ident(word)) => word,
        _ => panic!("the macro attribute applies only to functions"),
    };

    (TokenStream::from_iter(attrs.into_iter()), name)
}

/// Rename the decorated function by appending  `_` and the provided `specifier`.
///
/// ``` no_run
/// use interpolate_name::interpolate_name;
///
/// #[interpolate_name(spec)]
/// fn foo() {}
/// ```
///
/// produces:
/// ``` no_run
/// fn foo_spec() {}
/// ```
#[proc_macro_attribute]
pub fn interpolate_name(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens = TokenStream::from(attr).into_iter().collect::<Vec<_>>();
    if tokens.len() != 1 {
        panic!("expected #[interpolate_name(specifier)]");
    }

    let specifier = match &tokens[0] {
        TokenTree::Literal(tt) => tt.to_string(),
        TokenTree::Ident(tt) => tt.to_string(),
        _ => panic!("expected #[interpolate_name(specifier)]"),
    };

    let item = TokenStream::from(item);
    let (attrs, name) = fn_attrs_name(item.clone());
    let interpolated_name = Ident::new(
        &format!("{}_{}", name.to_string(), specifier),
        Span::call_site(),
    );

    let ret: TokenStream = quote_spanned! {
        proc_macro2::Span::call_site() =>
        #attrs
        fn #interpolated_name() {
            #item

            return #name ();
        }
    }
    .into();

    ret.into()
}

use std::iter::FromIterator;

/// Generate a new test that calls the decorated function with the provided arguments.
///
/// The test function name is the same as the called plus `_` and `specifier`.
/// Can decorate the same function multiple times.
///
/// ``` no_run
/// use interpolate_name::interpolate_test;
///
/// #[interpolate_test(some, "some", "arguments", 1)]
/// #[interpolate_test(name, "other", "arguments", 42)]
/// fn foo(a: &str, b: &str, c: usize) {
///     println!("{} {} {}", a, b,c);
/// }
/// ```
///
/// produces:
/// ``` no_run
/// #[test]
/// fn foo_some() { foo("some", "arguments", 1); }
/// #[test]
/// fn foo_name() { foo("other", "arguments", 42); }
/// ```
#[proc_macro_attribute]
pub fn interpolate_test(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens = TokenStream::from(attr).into_iter().collect::<Vec<_>>();
    if tokens.len() < 3 {
        panic!("expected #[interpolate_test(specifier, arguments)]");
    }

    let specifier = match &tokens[0] {
        TokenTree::Literal(tt) => tt.to_string(),
        TokenTree::Ident(tt) => tt.to_string(),
        _ => panic!("expected #[interpolate_test(specifier, arguments)]"),
    };

    match &tokens[1] {
        TokenTree::Punct(p) if p.as_char() == ',' => {}
        _ => panic!("expected #[interpolate_test(specifier, arguments)]"),
    }

    let args = TokenStream::from_iter(tokens.into_iter().skip(2));

    let mut syn_item = parse_macro_input!(item as ItemFn);
    let (mut interpolate_attrs, attrs): (Vec<Attribute>, Vec<Attribute>) =
        syn_item.attrs.iter().cloned().partition(|attr| {
            let found = if let Some(seg) = attr.path().segments.last() {
                seg.ident == "interpolate_test"
            } else {
                false
            };
            found
        });
    let append_attrs = !interpolate_attrs.is_empty();
    if append_attrs {
        interpolate_attrs.extend(attrs.iter().cloned());
    }
    syn_item.attrs = interpolate_attrs;
    let attrs = TokenStream::from_iter(attrs.iter().map(|it| it.into_token_stream()));
    let item = TokenStream::from(syn_item.into_token_stream());
    let name = fn_name(item.clone());
    let interpolated_name = Ident::new(
        &format!("{}_{}", name.to_string(), specifier),
        Span::call_site(),
    );

    let ret: TokenStream = quote_spanned! {
        proc_macro2::Span::call_site() =>
        #item

        #[test]
        #attrs
        fn #interpolated_name() {
            return #name (#args);
        }
    }
    .into();

    ret.into()
}
