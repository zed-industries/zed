//! Implementation of [`ops::Add`]-like derive macros.

#[cfg(doc)]
use std::ops;

use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens as _};
use syn::spanned::Spanned as _;

use super::{SkippedFields, StructuralExpansion};
use crate::utils::attr::{self, ParseMultiple as _};

/// Expands an [`ops::Add`]-like derive macro.
///
/// Available macros:
/// - [`Add`](ops::Add)
/// - [`BitAnd`](ops::BitAnd)
/// - [`BitOr`](ops::BitOr)
/// - [`BitXor`](ops::BitXor)
/// - [`Sub`](ops::Sub)
pub fn expand(input: &syn::DeriveInput, trait_name: &str) -> syn::Result<TokenStream> {
    let trait_name = normalize_trait_name(trait_name);
    let attr_name = format_ident!("{}", trait_name_to_attribute_name(trait_name));

    let mut variants = vec![];
    match &input.data {
        syn::Data::Struct(data) => {
            if let Some(skip) = attr::Skip::parse_attrs(&input.attrs, &attr_name)? {
                return Err(syn::Error::new(
                    skip.span,
                    format!(
                        "`#[{attr_name}({})]` attribute can be placed only on struct fields",
                        skip.item.name(),
                    ),
                ));
            } else if matches!(data.fields, syn::Fields::Unit) {
                return Err(syn::Error::new(
                    data.struct_token.span(),
                    format!("`{trait_name}` cannot be derived for unit structs"),
                ));
            }
            let mut skipped_fields = SkippedFields::default();
            for (n, field) in data.fields.iter().enumerate() {
                if attr::Skip::parse_attrs(&field.attrs, &attr_name)?.is_some() {
                    _ = skipped_fields.insert(n);
                }
            }
            if data.fields.len() == skipped_fields.len() {
                return Err(syn::Error::new(
                    data.struct_token.span(),
                    format!(
                        "`{trait_name}` cannot be derived for structs with all the fields being \
                         skipped",
                    ),
                ));
            }
            variants.push((None, &data.fields, skipped_fields));
        }
        syn::Data::Enum(data) => {
            if let Some(skip) = attr::Skip::parse_attrs(&input.attrs, &attr_name)? {
                return Err(syn::Error::new(
                    skip.span,
                    format!(
                        "`#[{attr_name}({})]` attribute can be placed only on enum fields",
                        skip.item.name(),
                    ),
                ));
            }
            for variant in &data.variants {
                if let Some(skip) = attr::Skip::parse_attrs(&variant.attrs, &attr_name)?
                {
                    return Err(syn::Error::new(
                        skip.span,
                        format!(
                            "`#[{attr_name}({})]` attribute can be placed only on variant fields",
                            skip.item.name(),
                        ),
                    ));
                }
                let mut skipped_fields = SkippedFields::default();
                for (n, field) in variant.fields.iter().enumerate() {
                    if attr::Skip::parse_attrs(&field.attrs, &attr_name)?.is_some() {
                        _ = skipped_fields.insert(n);
                    }
                }
                if !matches!(variant.fields, syn::Fields::Unit)
                    && variant.fields.len() == skipped_fields.len()
                {
                    return Err(syn::Error::new(
                        variant.span(),
                        format!(
                            "`{trait_name}` cannot be derived for enum with all the fields being \
                             skipped in its variants",
                        ),
                    ));
                }
                variants.push((Some(&variant.ident), &variant.fields, skipped_fields));
            }
        }
        syn::Data::Union(data) => {
            return Err(syn::Error::new(
                data.union_token.span(),
                format!("`{trait_name}` cannot be derived for unions"),
            ));
        }
    }

    Ok(StructuralExpansion {
        trait_ty: format_ident!("{trait_name}"),
        method_ident: format_ident!("{}", trait_name_to_method_name(trait_name)),
        self_ty: (&input.ident, &input.generics),
        variants,
        is_enum: matches!(input.data, syn::Data::Enum(_)),
    }
    .into_token_stream())
}

/// Matches the provided derive macro `name` to appropriate actual trait name.
fn normalize_trait_name(name: &str) -> &'static str {
    match name {
        "Add" => "Add",
        "BitAnd" => "BitAnd",
        "BitOr" => "BitOr",
        "BitXor" => "BitXor",
        "Sub" => "Sub",
        _ => unimplemented!(),
    }
}

/// Matches the provided [`ops::Add`]-like trait `name` to its attribute's name.
fn trait_name_to_attribute_name(name: &str) -> &'static str {
    trait_name_to_method_name(name)
}

/// Matches the provided [`ops::Add`]-like trait `name` to its method name.
fn trait_name_to_method_name(name: &str) -> &'static str {
    match name {
        "Add" => "add",
        "BitAnd" => "bitand",
        "BitOr" => "bitor",
        "BitXor" => "bitxor",
        "Sub" => "sub",
        _ => unimplemented!(),
    }
}
