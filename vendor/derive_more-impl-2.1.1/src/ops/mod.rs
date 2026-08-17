//! Implementations of [`ops`]-related derive macros.
//!
//! [`ops`]: std::ops

#[cfg(feature = "add")]
pub(crate) mod add;
#[cfg(feature = "add_assign")]
pub(crate) mod add_assign;
#[cfg(feature = "mul")]
pub(crate) mod mul;
#[cfg(feature = "mul_assign")]
pub(crate) mod mul_assign;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse_quote;

#[cfg(doc)]
use crate::utils::attr;
use crate::utils::{
    pattern_matching::FieldsExt as _, structural_inclusion::TypeExt as _,
    GenericsSearch, HashSet,
};

/// Indices of [`syn::Field`]s marked with an [`attr::Skip`].
type SkippedFields = HashSet<usize>;

#[cfg(any(feature = "add_assign", feature = "mul_assign"))]
/// Expansion of a macro for generating a structural trait implementation with a `&mut self` method
/// receiver for an enum or a struct.
struct AssignStructuralExpansion<'i> {
    /// [`syn::Ident`] of the implemented trait.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    trait_ty: syn::Ident,

    /// [`syn::Ident`] and [`syn::Receiver`] of the implemented method in trait.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    method_ident: syn::Ident,

    /// [`syn::Ident`] and [`syn::Generics`] of the implementor enum/struct.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    self_ty: (&'i syn::Ident, &'i syn::Generics),

    /// [`syn::Fields`] of the enum/struct to be used in this [`AssignStructuralExpansion`].
    variants: Vec<(Option<&'i syn::Ident>, &'i syn::Fields, SkippedFields)>,

    /// Indicator whether this expansion is for an enum.
    is_enum: bool,
}

#[cfg(any(feature = "add_assign", feature = "mul_assign"))]
impl AssignStructuralExpansion<'_> {
    /// Generates body of the method implementation for this [`StructuralExpansion`].
    fn body(&self) -> TokenStream {
        let method_path = {
            let trait_ty = &self.trait_ty;
            let method_ident = &self.method_ident;

            quote! { derive_more::core::ops::#trait_ty::#method_ident }
        };

        let match_arms = self
            .variants
            .iter()
            .map(|(variant, all_fields, skipped_fields)| {
                let variant = variant.map(|variant| quote! { :: #variant });
                let self_pat = all_fields.non_exhaustive_arm_pattern("__self_", skipped_fields);
                let rhs_pat = all_fields.non_exhaustive_arm_pattern("__rhs_", skipped_fields);

                let fields_exprs = (0..all_fields.len())
                    .filter(|num| !skipped_fields.contains(num))
                    .map(|num| {
                        let self_val = format_ident!("__self_{num}");
                        let rhs_val = format_ident!("__rhs_{num}");

                        quote! { #method_path(#self_val, #rhs_val); }
                    });

                quote! {
                    (Self #variant #self_pat, Self #variant #rhs_pat) => { #( #fields_exprs )* }
                }
            })
            .collect::<Vec<_>>();

        let wrong_variant_arm = (self.is_enum && match_arms.len() > 1).then(|| {
            quote! { _ => {} }
        });

        quote! {
            match (self, __rhs) {
                #( #match_arms )*
                #wrong_variant_arm
            }
        }
    }
}

#[cfg(any(feature = "add_assign", feature = "mul_assign"))]
impl ToTokens for AssignStructuralExpansion<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let trait_ty = &self.trait_ty;
        let method_ident = &self.method_ident;

        let ty = self.self_ty.0;
        let (_, ty_generics, _) = self.self_ty.1.split_for_impl();
        let implementor_ty: syn::Type = parse_quote! { #ty #ty_generics };
        let self_ty: syn::Type = parse_quote! { Self };

        let generics_search = GenericsSearch::from(self.self_ty.1);
        let mut generics = self.self_ty.1.clone();
        for (_, all_fields, skipped_fields) in &self.variants {
            for field_ty in all_fields.iter().enumerate().filter_map(|(n, field)| {
                (!skipped_fields.contains(&n)).then_some(&field.ty)
            }) {
                if generics_search.any_in(field_ty)
                    && !field_ty.contains_type_structurally(&self_ty)
                    && !field_ty.contains_type_structurally(&implementor_ty)
                {
                    generics.make_where_clause().predicates.push(parse_quote! {
                        #field_ty: derive_more::core::ops:: #trait_ty
                    });
                }
            }
        }
        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let body = self.body();

        quote! {
            #[allow(private_bounds)]
            #[automatically_derived]
            impl #impl_generics derive_more::core::ops:: #trait_ty for #implementor_ty
                 #where_clause
            {
                #[inline]
                #[track_caller]
                fn #method_ident(&mut self, __rhs: Self) {
                    #body
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[cfg(any(feature = "add", feature = "mul"))]
/// Expansion of a macro for generating a structural trait implementation with a `self` method
/// receiver for an enum or a struct.
struct StructuralExpansion<'i> {
    /// [`syn::Ident`] of the implemented trait.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    trait_ty: syn::Ident,

    /// [`syn::Ident`] and [`syn::Receiver`] of the implemented method in trait.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    method_ident: syn::Ident,

    /// [`syn::Ident`] and [`syn::Generics`] of the implementor enum/struct.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    self_ty: (&'i syn::Ident, &'i syn::Generics),

    /// [`syn::Fields`] of the enum/struct to be used in this [`StructuralExpansion`].
    variants: Vec<(Option<&'i syn::Ident>, &'i syn::Fields, SkippedFields)>,

    /// Indicator whether this expansion is for an enum.
    is_enum: bool,
}

#[cfg(any(feature = "add", feature = "mul"))]
impl StructuralExpansion<'_> {
    /// Generates body of the method implementation for this [`StructuralExpansion`].
    fn body(&self) -> TokenStream {
        // TODO: Try remove once MSRV is bumped up.
        // Special case: empty enum.
        if self.is_enum && self.variants.is_empty() {
            return quote! { match self {} };
        }

        let method_name = self.method_ident.to_string();
        let method_path = {
            let trait_ty = &self.trait_ty;
            let method_ident = &self.method_ident;

            parse_quote! { derive_more::core::ops::#trait_ty::#method_ident }
        };

        let match_arms = self
            .variants
            .iter()
            .map(|(variant, all_fields, skipped_fields)| {
                let variant = variant.map(|variant| quote! { :: #variant });
                let self_pat = all_fields.exhaustive_arm_pattern("__self_");
                let rhs_pat = all_fields.exhaustive_arm_pattern("__rhs_");

                let expr = if matches!(all_fields, syn::Fields::Unit) {
                    quote! {
                        derive_more::core::result::Result::Err(derive_more::BinaryError::Unit(
                            derive_more::UnitError::new(#method_name)
                        ))
                    }
                } else {
                    let fields_expr = all_fields.arm_expr(&method_path, skipped_fields);
                    if self.is_enum {
                        quote! { derive_more::core::result::Result::Ok(Self #variant #fields_expr) }
                    } else {
                        quote! { Self #variant #fields_expr }
                    }
                };

                quote! {
                    (Self #variant #self_pat, Self #variant #rhs_pat) => #expr,
                }
            })
            .collect::<Vec<_>>();

        let wrong_variant_arm = (self.is_enum && match_arms.len() > 1).then(|| {
            quote! {
                _ => derive_more::core::result::Result::Err(derive_more::BinaryError::Mismatch(
                    derive_more::WrongVariantError::new(#method_name)
                )),
            }
        });

        quote! {
            match (self, __rhs) {
                #( #match_arms )*
                #wrong_variant_arm
            }
        }
    }
}

#[cfg(any(feature = "add", feature = "mul"))]
impl ToTokens for StructuralExpansion<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let trait_ty = &self.trait_ty;
        let method_ident = &self.method_ident;

        let ty = self.self_ty.0;
        let (_, ty_generics, _) = self.self_ty.1.split_for_impl();
        let implementor_ty: syn::Type = parse_quote! { #ty #ty_generics };
        let self_ty: syn::Type = parse_quote! { Self };

        let output_ty = if self.is_enum {
            parse_quote! { derive_more::core::result::Result<#self_ty, derive_more::BinaryError> }
        } else {
            self_ty.clone()
        };

        let generics_search = GenericsSearch::from(self.self_ty.1);
        let mut generics = self.self_ty.1.clone();
        for (_, all_fields, skipped_fields) in &self.variants {
            for field_ty in all_fields.iter().enumerate().filter_map(|(n, field)| {
                (!skipped_fields.contains(&n)).then_some(&field.ty)
            }) {
                if generics_search.any_in(field_ty)
                    && !field_ty.contains_type_structurally(&self_ty)
                    && !field_ty.contains_type_structurally(&implementor_ty)
                {
                    generics.make_where_clause().predicates.push(parse_quote! {
                        #field_ty: derive_more::core::ops:: #trait_ty <Output = #field_ty>
                    });
                }
            }
        }
        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let body = self.body();

        quote! {
            #[allow(private_bounds)]
            #[automatically_derived]
            impl #impl_generics derive_more::core::ops:: #trait_ty for #implementor_ty
                 #where_clause
            {
                type Output = #output_ty;

                #[inline]
                #[track_caller]
                fn #method_ident(self, __rhs: Self) -> Self::Output {
                    #body
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[cfg(any(feature = "add", feature = "mul"))]
/// Extension of [`syn::Fields`] used by a [`StructuralExpansion`].
trait StructuralExpansionFieldsExt {
    /// Generates a resulting expression with these [`syn::Fields`] in a matched arm of a `match`
    /// expression, by applying the specified method.
    fn arm_expr(
        &self,
        method: &syn::Path,
        skipped_indices: &SkippedFields,
    ) -> TokenStream;
}

#[cfg(any(feature = "add", feature = "mul"))]
impl StructuralExpansionFieldsExt for syn::Fields {
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
                        let rhs_val = format_ident!("__rhs_{num}");
                        quote! { #name: #method_path(#self_val, #rhs_val) }
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
                        let rhs_val = format_ident!("__rhs_{num}");
                        quote! { #method_path(#self_val, #rhs_val) }
                    }
                });
                quote! {( #( #fields , )* )}
            }
            Self::Unit => quote! {},
        }
    }
}
