#![recursion_limit = "128"]
#![cfg_attr(
    feature = "force_exhaustive_checks",
    feature(non_exhaustive_omitted_patterns_lint)
)]
#![allow(clippy::needless_lifetimes)]

extern crate proc_macro;
use proc_macro::TokenStream;
#[cfg(feature = "schema")]
use proc_macro2::Span;
use syn::{DeriveInput, Error, ItemEnum, ItemStruct, ItemUnion, Path};

///  by convention, local to borsh-derive crate, imports from proc_macro (1) are not allowed in `internals` module or in any of its submodules.
mod internals;

use crate::internals::attributes::item;

#[cfg(feature = "schema")]
use internals::schema;
use internals::{cratename, deserialize, serialize};

fn check_attrs_get_cratename(input: &TokenStream) -> Result<Path, Error> {
    let input = input.clone();

    let derive_input = syn::parse::<DeriveInput>(input)?;

    item::check_attributes(&derive_input)?;

    cratename::get(&derive_input.attrs)
}

/// ---
///
/// moved to docs of **Derive Macro** `BorshSerialize` in `borsh` crate
#[proc_macro_derive(BorshSerialize, attributes(borsh))]
pub fn borsh_serialize(input: TokenStream) -> TokenStream {
    let cratename = match check_attrs_get_cratename(&input) {
        Ok(cratename) => cratename,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let res = if let Ok(input) = syn::parse::<ItemStruct>(input.clone()) {
        serialize::structs::process(&input, cratename)
    } else if let Ok(input) = syn::parse::<ItemEnum>(input.clone()) {
        serialize::enums::process(&input, cratename)
    } else if let Ok(input) = syn::parse::<ItemUnion>(input) {
        serialize::unions::process(&input, cratename)
    } else {
        // Derive macros can only be defined on structs, enums, and unions.
        unreachable!()
    };
    TokenStream::from(match res {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    })
}

/// ---
///
/// moved to docs of **Derive Macro** `BorshDeserialize` in `borsh` crate
#[proc_macro_derive(BorshDeserialize, attributes(borsh))]
pub fn borsh_deserialize(input: TokenStream) -> TokenStream {
    let cratename = match check_attrs_get_cratename(&input) {
        Ok(cratename) => cratename,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let res = if let Ok(input) = syn::parse::<ItemStruct>(input.clone()) {
        deserialize::structs::process(&input, cratename)
    } else if let Ok(input) = syn::parse::<ItemEnum>(input.clone()) {
        deserialize::enums::process(&input, cratename)
    } else if let Ok(input) = syn::parse::<ItemUnion>(input) {
        deserialize::unions::process(&input, cratename)
    } else {
        // Derive macros can only be defined on structs, enums, and unions.
        unreachable!()
    };
    TokenStream::from(match res {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    })
}

/// ---
///
/// moved to docs of **Derive Macro** `BorshSchema` in `borsh` crate
#[cfg(feature = "schema")]
#[proc_macro_derive(BorshSchema, attributes(borsh))]
pub fn borsh_schema(input: TokenStream) -> TokenStream {
    let cratename = match check_attrs_get_cratename(&input) {
        Ok(cratename) => cratename,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let res = if let Ok(input) = syn::parse::<ItemStruct>(input.clone()) {
        schema::structs::process(&input, cratename)
    } else if let Ok(input) = syn::parse::<ItemEnum>(input.clone()) {
        schema::enums::process(&input, cratename)
    } else if syn::parse::<ItemUnion>(input).is_ok() {
        Err(syn::Error::new(
            Span::call_site(),
            "Borsh schema does not support unions yet.",
        ))
    } else {
        // Derive macros can only be defined on structs, enums, and unions.
        unreachable!()
    };
    TokenStream::from(match res {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    })
}
