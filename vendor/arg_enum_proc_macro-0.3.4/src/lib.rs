//! # arg_enum_proc_macro
//!
//! This crate consists in a procedural macro derive that provides the
//! same implementations that clap the [`clap::arg_enum`][1] macro provides:
//! [`std::fmt::Display`], [`std::str::FromStr`] and a `variants()` function.
//!
//! By using a procedural macro it allows documenting the enum fields
//! correctly and avoids the requirement of expanding the macro to use
//! the structure with [cbindgen](https://crates.io/crates/cbindgen).
//!
//! [1]: https://docs.rs/clap/2.32.0/clap/macro.arg_enum.html
//!

#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro2::{Literal, Punct, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use std::iter::FromIterator;

use syn::Lit::{self};
use syn::Meta::{self};
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Ident, LitStr};

/// Implement [`std::fmt::Display`], [`std::str::FromStr`] and `variants()`.
///
/// The invocation:
/// ``` no_run
/// use arg_enum_proc_macro::ArgEnum;
///
/// #[derive(ArgEnum)]
/// enum Foo {
///     A,
///     /// Describe B
///     #[arg_enum(alias = "Bar")]
///     B,
///     /// Describe C
///     /// Multiline
///     #[arg_enum(name = "Baz")]
///     C,
/// }
/// ```
///
/// produces:
/// ``` no_run
/// enum Foo {
///     A,
///     B,
///     C
/// }
/// impl ::std::str::FromStr for Foo {
///     type Err = String;
///
///     fn from_str(s: &str) -> ::std::result::Result<Self,Self::Err> {
///         match s {
///             "A" | _ if s.eq_ignore_ascii_case("A") => Ok(Foo::A),
///             "B" | _ if s.eq_ignore_ascii_case("B") => Ok(Foo::B),
///             "Bar" | _ if s.eq_ignore_ascii_case("Bar") => Ok(Foo::B),
///             "Baz" | _ if s.eq_ignore_ascii_case("Baz") => Ok(Foo::C),
///             _ => Err({
///                 let v = vec![ "A", "B", "Bar", "Baz" ];
///                 format!("valid values: {}", v.join(" ,"))
///             }),
///         }
///     }
/// }
/// impl ::std::fmt::Display for Foo {
///     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
///         match *self {
///             Foo::A => write!(f, "A"),
///             Foo::B => write!(f, "B"),
///             Foo::C => write!(f, "C"),
///         }
///     }
/// }
///
/// impl Foo {
///     /// Returns an array of valid values which can be converted into this enum.
///     #[allow(dead_code)]
///     pub fn variants() -> [&'static str; 4] {
///         [ "A", "B", "Bar", "Baz", ]
///     }
///     #[allow(dead_code)]
///     pub fn descriptions() -> [(&'static [&'static str], &'static [&'static str]) ;3] {
///         [(&["A"], &[]),
///          (&["B", "Bar"], &[" Describe B"]),
///          (&["Baz"], &[" Describe C", " Multiline"]),]
///     }
/// }
/// ```
#[proc_macro_derive(ArgEnum, attributes(arg_enum))]
pub fn arg_enum(items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(items as DeriveInput);

    let name = input.ident;
    let variants = if let Data::Enum(data) = input.data {
        data.variants
    } else {
        panic!("Only enum supported");
    };

    let all_variants: Vec<(TokenTree, &Ident)> = variants
        .iter()
        .flat_map(|item| {
            let id = &item.ident;
            if !item.fields.is_empty() {
                panic!(
                    "Only enum with unit variants are supported! \n\
                    Variant {}::{} is not an unit variant",
                    name,
                    &id.to_string()
                );
            }

            let lit: TokenTree = Literal::string(&id.to_string()).into();
            let mut all_lits = vec![(lit, id)];
            item.attrs
                .iter()
                .filter(|attr| attr.path().is_ident("arg_enum"))
                // .flat_map(|attr| {
                .for_each(|attr| {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("alias") {
                            let val = meta.value()?;
                            let alias: Literal = val.parse()?;
                            all_lits.push((alias.into(), id));
                        }
                        if meta.path.is_ident("name") {
                            let val = meta.value()?;
                            let name: Literal = val.parse()?;
                            all_lits[0] = (name.into(), id);
                        }
                        Ok(())
                    })
                    .unwrap();
                });
            all_lits.into_iter()
        })
        .collect();

    let len = all_variants.len();

    let from_str_match = all_variants.iter().flat_map(|(lit, id)| {
        let pat: TokenStream = quote! {
            #lit | _ if s.eq_ignore_ascii_case(#lit) => Ok(#name::#id),
        };

        pat.into_iter()
    });

    let from_str_match = TokenStream::from_iter(from_str_match);

    let all_descriptions: Vec<(Vec<TokenTree>, Vec<LitStr>)> = variants
        .iter()
        .map(|item| {
            let id = &item.ident;
            let description = item
                .attrs
                .iter()
                .filter_map(|attr| {
                    let expr = match &attr.meta {
                        Meta::NameValue(name_value) if name_value.path.is_ident("doc") => {
                            Some(name_value.value.to_owned())
                        }
                        _ =>
                        // non #[doc = "..."] attributes are not our concern
                        // we leave them for rustc to handle
                        {
                            None
                        }
                    };

                    expr.and_then(|expr| {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Str(s), ..
                        }) = expr
                        {
                            Some(s)
                        } else {
                            None
                        }
                    })
                })
                .collect();
            let lit: TokenTree = Literal::string(&id.to_string()).into();
            let mut all_names = vec![lit];
            item.attrs
                .iter()
                .filter(|attr| attr.path().is_ident("arg_enum"))
                // .flat_map(|attr| {
                .for_each(|attr| {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("alias") {
                            let val = meta.value()?;
                            let alias: Literal = val.parse()?;
                            all_names.push(alias.into());
                        }
                        if meta.path.is_ident("name") {
                            let val = meta.value()?;
                            let name: Literal = val.parse()?;
                            all_names[0] = name.into();
                        }
                        Ok(())
                    })
                    .unwrap();
                });

            (all_names, description)
        })
        .collect();

    let display_match = variants.iter().flat_map(|item| {
        let id = &item.ident;
        let lit: TokenTree = Literal::string(&id.to_string()).into();

        let pat: TokenStream = quote! {
            #name::#id => write!(f, #lit),
        };

        pat.into_iter()
    });

    let display_match = TokenStream::from_iter(display_match);

    let comma: TokenTree = Punct::new(',', proc_macro2::Spacing::Alone).into();
    let array_items = all_variants
        .iter()
        .flat_map(|(tok, _id)| vec![tok.clone(), comma.clone()].into_iter());

    let array_items = TokenStream::from_iter(array_items);

    let array_descriptions = all_descriptions.iter().map(|(names, descr)| {
        quote! {
            (&[ #(#names),* ], &[ #(#descr),* ]),
        }
    });
    let array_descriptions = TokenStream::from_iter(array_descriptions);

    let len_descriptions = all_descriptions.len();

    let ret: TokenStream = quote_spanned! {
        Span::call_site() =>
        impl ::std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> ::std::result::Result<Self,Self::Err> {
                match s {
                    #from_str_match
                    _ => {
                        let values = [ #array_items ];

                        Err(format!("valid values: {}", values.join(" ,")))
                    }
                }
            }
        }
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match *self {
                    #display_match
                }
            }
        }
        impl #name {
            #[allow(dead_code)]
            /// Returns an array of valid values which can be converted into this enum.
            pub fn variants() -> [&'static str; #len] {
                [ #array_items ]
            }
            #[allow(dead_code)]
            /// Returns an array of touples (variants, description)
            pub fn descriptions() -> [(&'static [&'static str], &'static [&'static str]); #len_descriptions] {
                [ #array_descriptions ]
            }
        }
    };

    ret.into()
}
