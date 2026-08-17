//! [![github]](https://github.com/dtolnay/enumn)&ensp;[![crates-io]](https://crates.io/crates/enumn)&ensp;[![docs-rs]](https://docs.rs/enumn)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! Convert number to enum.
//!
//! This crate provides a derive macro to generate a function for converting a
//! primitive integer into the corresponding variant of an enum.
//!
//! The generated function is named `n` and has the following signature:
//!
//! ```
//! # const IGNORE: &str = stringify! {
//! impl YourEnum {
//!     pub fn n(value: Repr) -> Option<Self>;
//! }
//! # };
//! ```
//!
//! where `Repr` is an integer type of the right size as described in more
//! detail below.
//!
//! # Example
//!
//! ```
//! use enumn::N;
//!
//! #[derive(PartialEq, Debug, N)]
//! enum Status {
//!     LegendaryTriumph,
//!     QualifiedSuccess,
//!     FortuitousRevival,
//!     IndeterminateStalemate,
//!     RecoverableSetback,
//!     DireMisadventure,
//!     AbjectFailure,
//! }
//!
//! fn main() {
//!     let s = Status::n(1);
//!     assert_eq!(s, Some(Status::QualifiedSuccess));
//!
//!     let s = Status::n(9);
//!     assert_eq!(s, None);
//! }
//! ```
//!
//! # Signature
//!
//! The generated signature depends on whether the enum has a `#[repr(..)]`
//! attribute. If a `repr` is specified, the input to `n` will be required to be
//! of that type.
//!
//! ```
//! #[derive(enumn::N)]
//! # enum E0 {
//! #     IGNORE
//! # }
//! #
//! #[repr(u8)]
//! enum E {
//!     /* ... */
//!     # IGNORE
//! }
//!
//! // expands to:
//! impl E {
//!     pub fn n(value: u8) -> Option<Self> {
//!         /* ... */
//!         # unimplemented!()
//!     }
//! }
//! ```
//!
//! On the other hand if no `repr` is specified then we get a signature that is
//! generic over a variety of possible types.
//!
//! ```
//! # enum E {}
//! #
//! impl E {
//!     pub fn n<REPR: Into<i64>>(value: REPR) -> Option<Self> {
//!         /* ... */
//!         # unimplemented!()
//!     }
//! }
//! ```
//!
//! # Discriminants
//!
//! The conversion respects explicitly specified enum discriminants. Consider
//! this enum:
//!
//! ```
//! #[derive(enumn::N)]
//! enum Letter {
//!     A = 65,
//!     B = 66,
//! }
//! ```
//!
//! Here `Letter::n(65)` would return `Some(Letter::A)`.

#![doc(html_root_url = "https://docs.rs/enumn/0.1.14")]
#![allow(
    clippy::missing_panics_doc,
    clippy::needless_doctest_main,
    clippy::single_match_else
)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Ident};

#[proc_macro_derive(N)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = match input.data {
        Data::Enum(data) => data.variants,
        Data::Struct(_) | Data::Union(_) => panic!("input must be an enum"),
    };

    for variant in &variants {
        match variant.fields {
            Fields::Unit => {}
            Fields::Named(_) | Fields::Unnamed(_) => {
                let span = variant.ident.span();
                let err = Error::new(span, "enumn: variant with data is not supported");
                return err.to_compile_error().into();
            }
        }
    }

    // Parse repr attribute like #[repr(u16)].
    let mut repr = None;
    for attr in input.attrs {
        if attr.path().is_ident("repr") {
            if let Ok(name) = attr.parse_args::<Ident>() {
                match name.to_string().as_str() {
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32"
                    | "i64" | "i128" | "isize" => {
                        repr = Some(quote!(#name));
                    }
                    _ => {}
                }
            }
        }
    }

    let signature;
    let value;
    match repr {
        Some(ref repr) => {
            signature = quote! {
                fn n(value: #repr)
            };
            value = quote!(value);
        }
        None => {
            repr = Some(quote!(i64));
            signature = quote! {
                fn n<REPR: Into<i64>>(value: REPR)
            };
            value = quote! {
                <REPR as Into<i64>>::into(value)
            };
        }
    }

    let ident = input.ident;
    let declare_discriminants = variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            const #variant: #repr = #ident::#variant as #repr;
        }
    });
    let match_discriminants = variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            discriminant::#variant => Some(#ident::#variant),
        }
    });

    TokenStream::from(quote! {
        impl #ident {
            pub #signature -> Option<Self> {
                #[allow(non_camel_case_types)]
                struct discriminant;
                #[allow(non_upper_case_globals)]
                impl discriminant {
                    #(#declare_discriminants)*
                }
                match #value {
                    #(#match_discriminants)*
                    _ => None,
                }
            }
        }
    })
}
