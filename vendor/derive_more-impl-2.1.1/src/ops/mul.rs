//! Implementation of [`ops::Mul`]-like derive macros.

#[cfg(doc)]
use std::ops;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, spanned::Spanned as _};

use super::{SkippedFields, StructuralExpansion};
use crate::utils::{
    attr::{self, ParseMultiple as _},
    pattern_matching::FieldsExt as _,
    structural_inclusion::TypeExt as _,
};

/// Expands an [`ops::Mul`]-like derive macro.
///
/// Available macros:
/// - [`Div`](ops::Div)
/// - [`Mul`](ops::Mul)
/// - [`Rem`](ops::Rem)
/// - [`Shl`](ops::Shl)
/// - [`Shr`](ops::Shr)
pub fn expand(input: &syn::DeriveInput, trait_name: &str) -> syn::Result<TokenStream> {
    let trait_name = normalize_trait_name(trait_name);
    let attr_name = format_ident!("{}", trait_name_to_attribute_name(trait_name));

    match &input.data {
        syn::Data::Struct(data) if matches!(data.fields, syn::Fields::Unit) => {
            Err(syn::Error::new(
                data.struct_token.span(),
                format!("`{trait_name}` cannot be derived for unit structs"),
            ))
        }
        syn::Data::Union(data) => Err(syn::Error::new(
            data.union_token.span(),
            format!("`{trait_name}` cannot be derived for unions"),
        )),
        _ => {
            if attr::Forward::parse_attrs(&input.attrs, &attr_name)?.is_some() {
                expand_structural(input, trait_name, attr_name)
                    .map(ToTokens::into_token_stream)
            } else {
                expand_scalar(input, trait_name, attr_name)
                    .map(ToTokens::into_token_stream)
            }
        }
    }
}

/// Expands an [`ops::Mul`]-like derive macro in a structural manner.
fn expand_structural<'i>(
    input: &'i syn::DeriveInput,
    trait_name: &str,
    attr_name: syn::Ident,
) -> syn::Result<StructuralExpansion<'i>> {
    let mut variants = vec![];
    match &input.data {
        syn::Data::Struct(data) => {
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
        syn::Data::Union(_) => unreachable!(),
    }

    Ok(StructuralExpansion {
        trait_ty: format_ident!("{trait_name}"),
        method_ident: format_ident!("{}", trait_name_to_method_name(trait_name)),
        self_ty: (&input.ident, &input.generics),
        variants,
        is_enum: matches!(input.data, syn::Data::Enum(_)),
    })
}

/// Expands an [`ops::Mul`]-like derive macro in a scalar manner.
fn expand_scalar<'i>(
    input: &'i syn::DeriveInput,
    trait_name: &str,
    attr_name: syn::Ident,
) -> syn::Result<ScalarExpansion<'i>> {
    let mut skipped_fields = SkippedFields::default();
    let fields = match &input.data {
        syn::Data::Struct(data) => {
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
            &data.fields
        }
        syn::Data::Enum(data) => {
            return Err(syn::Error::new(
                data.enum_token.span(),
                format!(
                    "`{trait_name}` can be derived for enums only when using \
                     `#[{attr_name}(forward)]` attribute",
                ),
            ));
        }
        syn::Data::Union(_) => unreachable!(),
    };

    Ok(ScalarExpansion {
        trait_ty: format_ident!("{trait_name}"),
        method_ident: format_ident!("{}", trait_name_to_method_name(trait_name)),
        self_ty: (&input.ident, &input.generics),
        fields,
        skipped_fields,
    })
}

/// Expansion of a macro for generating a scalar [`ops::Mul`]-like trait implementation for a
/// struct.
struct ScalarExpansion<'i> {
    /// [`syn::Ident`] of the implemented trait.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    trait_ty: syn::Ident,

    /// [`syn::Ident`] and [`syn::Receiver`] of the implemented method in trait.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    method_ident: syn::Ident,

    /// [`syn::Ident`] and [`syn::Generics`] of the implementor struct.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    self_ty: (&'i syn::Ident, &'i syn::Generics),

    /// [`syn::Fields`] of the struct to be used in this [`ScalarExpansion`].
    fields: &'i syn::Fields,

    /// Indices of the struct [`syn::Fields`] marked with an [`attr::Skip`].
    skipped_fields: SkippedFields,
}

impl ToTokens for ScalarExpansion<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let trait_ty = &self.trait_ty;
        let method_ident = &self.method_ident;

        let ty = self.self_ty.0;
        let (_, ty_generics, _) = self.self_ty.1.split_for_impl();
        let implementor_ty: syn::Type = parse_quote! { #ty #ty_generics };
        let self_ty: syn::Type = parse_quote! { Self };
        let rhs_ty: syn::TypeParam = parse_quote! { __derive_more_Rhs };

        let mut generics = self.self_ty.1.clone();
        generics.params.push(rhs_ty.clone().into());
        let mut used_fields_count = 0;
        for field_ty in self.fields.iter().enumerate().filter_map(|(n, field)| {
            (!self.skipped_fields.contains(&n)).then_some(&field.ty)
        }) {
            if !field_ty.contains_type_structurally(&self_ty)
                && !field_ty.contains_type_structurally(&implementor_ty)
            {
                generics.make_where_clause().predicates.push(parse_quote! {
                    #field_ty: derive_more::core::ops:: #trait_ty <#rhs_ty, Output = #field_ty>
                });
            }
            used_fields_count += 1;
        }
        if used_fields_count > 1 {
            generics.make_where_clause().predicates.push(parse_quote! {
                #rhs_ty: derive_more::core::marker::Copy
            });
        }
        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let body = {
            let method_path =
                parse_quote! { derive_more::core::ops::#trait_ty::#method_ident };
            let self_pat = self.fields.exhaustive_arm_pattern("__self_");
            let fields_expr = self.fields.arm_expr(&method_path, &self.skipped_fields);

            quote! {
                match self {
                    Self #self_pat => Self #fields_expr,
                }
            }
        };

        quote! {
            #[allow(private_bounds)]
            #[automatically_derived]
            impl #impl_generics derive_more::core::ops:: #trait_ty<#rhs_ty> for #implementor_ty
                 #where_clause
            {
                type Output = #self_ty;

                #[inline]
                #[track_caller]
                fn #method_ident(self, __rhs: #rhs_ty) -> Self::Output {
                    #body
                }
            }
        }
        .to_tokens(tokens);
    }
}

/// Extension of [`syn::Fields`] used by a [`ScalarExpansion`].
trait ScalarExpansionFieldsExt {
    /// Generates a resulting expression with these [`syn::Fields`] in a matched arm of a `match`
    /// expression, by applying the specified method.
    fn arm_expr(
        &self,
        method: &syn::Path,
        skipped_indices: &SkippedFields,
    ) -> TokenStream;
}

impl ScalarExpansionFieldsExt for syn::Fields {
    fn arm_expr(
        &self,
        method_path: &syn::Path,
        skipped_indices: &SkippedFields,
    ) -> TokenStream {
        match self {
            Self::Named(fields) => {
                let fields = fields.named.iter().enumerate().map(|(num, field)| {
                    let name = &field.ident;
                    let self_val = format_ident!("__self_{num}");
                    if skipped_indices.contains(&num) {
                        quote! { #name: #self_val }
                    } else {
                        quote! { #name: #method_path(#self_val, __rhs) }
                    }
                });
                quote! {{ #( #fields , )* }}
            }
            Self::Unnamed(fields) => {
                let fields = (0..fields.unnamed.len()).map(|num| {
                    let self_val = format_ident!("__self_{num}");
                    if skipped_indices.contains(&num) {
                        quote! { #self_val }
                    } else {
                        quote! { #method_path(#self_val, __rhs) }
                    }
                });
                quote! {( #( #fields , )* )}
            }
            Self::Unit => quote! {},
        }
    }
}

/// Matches the provided derive macro `name` to appropriate actual trait name.
fn normalize_trait_name(name: &str) -> &'static str {
    match name {
        "Div" => "Div",
        "Mul" => "Mul",
        "Rem" => "Rem",
        "Shl" => "Shl",
        "Shr" => "Shr",
        _ => unimplemented!(),
    }
}

/// Matches the provided [`ops::Mul`]-like trait `name` to its attribute's name.
fn trait_name_to_attribute_name(name: &str) -> &'static str {
    trait_name_to_method_name(name)
}

/// Matches the provided [`ops::Mul`]-like trait `name` to its method name.
fn trait_name_to_method_name(name: &str) -> &'static str {
    match name {
        "Div" => "div",
        "Mul" => "mul",
        "Rem" => "rem",
        "Shl" => "shl",
        "Shr" => "shr",
        _ => unimplemented!(),
    }
}
