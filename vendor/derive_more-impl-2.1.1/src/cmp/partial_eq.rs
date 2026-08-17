//! Implementation of a [`PartialEq`] derive macro.

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_quote,
    punctuated::{self, Punctuated},
    spanned::Spanned as _,
};

use crate::utils::{
    attr::{self, ParseMultiple as _},
    pattern_matching::FieldsExt as _,
    structural_inclusion::TypeExt as _,
    GenericsSearch, HashSet,
};

/// Expands a [`PartialEq`] derive macro.
pub fn expand(input: &syn::DeriveInput, _: &'static str) -> syn::Result<TokenStream> {
    let attr_name = format_ident!("partial_eq");
    let secondary_attr_name = format_ident!("eq");

    let mut has_skipped_variants = false;
    let mut variants = vec![];

    match &input.data {
        syn::Data::Struct(data) => {
            for attr_name in [&attr_name, &secondary_attr_name] {
                if attr::Skip::parse_attrs(&input.attrs, attr_name)?.is_some() {
                    has_skipped_variants = true;
                    break;
                }
            }
            if !has_skipped_variants {
                let mut skipped_fields = SkippedFields::default();
                'fields: for (n, field) in data.fields.iter().enumerate() {
                    for attr_name in [&attr_name, &secondary_attr_name] {
                        if attr::Skip::parse_attrs(&field.attrs, attr_name)?.is_some() {
                            _ = skipped_fields.insert(n);
                            continue 'fields;
                        }
                    }
                }
                variants.push((None, &data.fields, skipped_fields));
            }
        }
        syn::Data::Enum(data) => {
            'variants: for variant in &data.variants {
                for attr_name in [&attr_name, &secondary_attr_name] {
                    if attr::Skip::parse_attrs(&variant.attrs, attr_name)?.is_some() {
                        has_skipped_variants = true;
                        continue 'variants;
                    }
                }
                let mut skipped_fields = SkippedFields::default();
                'fields: for (n, field) in variant.fields.iter().enumerate() {
                    for attr_name in [&attr_name, &secondary_attr_name] {
                        if attr::Skip::parse_attrs(&field.attrs, attr_name)?.is_some() {
                            _ = skipped_fields.insert(n);
                            continue 'fields;
                        }
                    }
                }
                variants.push((Some(&variant.ident), &variant.fields, skipped_fields));
            }
        }
        syn::Data::Union(data) => {
            return Err(syn::Error::new(
                data.union_token.span(),
                "`PartialEq` cannot be derived for unions",
            ))
        }
    }

    Ok(StructuralExpansion {
        self_ty: (&input.ident, &input.generics),
        variants,
        has_skipped_variants,
        is_enum: matches!(input.data, syn::Data::Enum(_)),
    }
    .into_token_stream())
}

/// Indices of [`syn::Field`]s marked with an [`attr::Skip`].
type SkippedFields = HashSet<usize>;

/// Expansion of a macro for generating a structural [`PartialEq`] implementation of an enum or a
/// struct.
struct StructuralExpansion<'i> {
    /// [`syn::Ident`] and [`syn::Generics`] of the enum/struct.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    self_ty: (&'i syn::Ident, &'i syn::Generics),

    /// [`syn::Fields`] of the enum/struct to be compared in this [`StructuralExpansion`].
    variants: Vec<(Option<&'i syn::Ident>, &'i syn::Fields, SkippedFields)>,

    /// Indicator whether some original enum variants where skipped with an [`attr::Skip`].
    has_skipped_variants: bool,

    /// Indicator whether this expansion is for an enum.
    is_enum: bool,
}

impl StructuralExpansion<'_> {
    /// Generates body of the [`PartialEq::eq()`]/[`PartialEq::ne()`] method implementation for this
    /// [`StructuralExpansion`], if it's required.
    fn body(&self, eq: bool) -> Option<TokenStream> {
        // Special case: empty enum (also, no need for `ne()` method in this case).
        if self.is_enum && self.variants.is_empty() && !self.has_skipped_variants {
            return eq.then(|| quote! { match *self {} });
        }

        let no_fields_result = quote! { #eq };

        // Special case: fully skipped struct (also, no need for `ne()` method in this case).
        if !self.is_enum && self.variants.is_empty() && self.has_skipped_variants {
            return eq.then_some(no_fields_result);
        }
        // Special case: no fields to compare in struct/single-variant enum (also, no need for
        // `ne()` method in this case).
        if !(self.is_enum && self.has_skipped_variants)
            && self.variants.len() == 1
            && (self.variants[0].1.is_empty()
                || self.variants[0].1.len() == self.variants[0].2.len())
        {
            return eq.then_some(no_fields_result);
        }

        let (cmp, chain) = if eq {
            (quote! { == }, quote! { && })
        } else {
            (quote! { != }, quote! { || })
        };

        let discriminants_cmp = (self.variants.len() > 1 || self.has_skipped_variants)
            .then(|| {
                quote! {
                    derive_more::core::mem::discriminant(self) #cmp
                        derive_more::core::mem::discriminant(__other)
                }
            });

        let match_arms = self
            .variants
            .iter()
            .filter_map(|(variant, all_fields, skipped_fields)| {
                if all_fields.is_empty() || skipped_fields.len() == all_fields.len() {
                    return None;
                }

                let variant = variant.map(|variant| quote! { :: #variant });
                let self_pattern = all_fields
                    .non_exhaustive_arm_pattern("__self_", skipped_fields);
                let other_pattern = all_fields
                    .non_exhaustive_arm_pattern("__other_", skipped_fields);

                let mut val_eqs = (0..all_fields.len())
                    .filter(|num| !skipped_fields.contains(num))
                    .map(|num| {
                        let self_val = format_ident!("__self_{num}");
                        let other_val = format_ident!("__other_{num}");
                        punctuated::Pair::Punctuated(quote! { #self_val #cmp #other_val }, &chain)
                    })
                    .collect::<Punctuated<TokenStream, _>>();
                _ = val_eqs.pop_punct();

                Some(quote! {
                    (Self #variant #self_pattern, Self #variant #other_pattern) => { #val_eqs }
                })
            })
            .collect::<Vec<_>>();
        let match_expr = (!match_arms.is_empty()).then(|| {
            let no_fields_arm = (match_arms.len() != self.variants.len()
                || self.has_skipped_variants)
                .then(|| {
                    quote! { _ => #no_fields_result }
                });
            let unreachable_arm = (self.variants.len() > 1
                && no_fields_arm.is_none())
            .then(|| {
                quote! {
                    // SAFETY: This arm is never reachable due to `mem::discriminant()` comparison
                    //         preceding the expanded `match (self, other)` expression, but is
                    //         required by it when there is more than one variant.
                    _ => unsafe { derive_more::core::hint::unreachable_unchecked() },
                }
            });

            quote! {
                match (self, __other) {
                    #( #match_arms , )*
                    #no_fields_arm
                    #unreachable_arm
                }
            }
        });

        // If there is only `mem::discriminant()` comparison, there is no need to generate `ne()`
        // method in the expansion, as its default implementation will do just fine.
        if !eq && discriminants_cmp.is_some() && match_expr.is_none() {
            return None;
        }

        let chain =
            (discriminants_cmp.is_some() && match_expr.is_some()).then_some(chain);

        Some(quote! {
            #discriminants_cmp #chain #match_expr
        })
    }
}

impl ToTokens for StructuralExpansion<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = self.self_ty.0;
        let (_, ty_generics, _) = self.self_ty.1.split_for_impl();

        let generics_search = GenericsSearch::from(self.self_ty.1);
        let mut generics = self.self_ty.1.clone();
        {
            let self_ty: syn::Type = parse_quote! { Self };
            let implementor_ty: syn::Type = parse_quote! { #ty #ty_generics };
            for (_, all_fields, skipped_fields) in &self.variants {
                for field_ty in
                    all_fields.iter().enumerate().filter_map(|(n, field)| {
                        (!skipped_fields.contains(&n)).then_some(&field.ty)
                    })
                {
                    if generics_search.any_in(field_ty)
                        && !field_ty.contains_type_structurally(&self_ty)
                        && !field_ty.contains_type_structurally(&implementor_ty)
                    {
                        generics.make_where_clause().predicates.push(parse_quote! {
                            #field_ty: derive_more::core::cmp::PartialEq
                        });
                    }
                }
            }
        }
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let eq_method = self.body(true).map(|body| {
            quote! {
                #[inline]
                fn eq(&self, __other: &Self) -> bool { #body }
            }
        });
        let ne_method = self.body(false).map(|body| {
            quote! {
                #[inline]
                fn ne(&self, __other: &Self) -> bool { #body }
            }
        });

        quote! {
            #[allow(private_bounds)]
            #[automatically_derived]
            impl #impl_generics derive_more::core::cmp::PartialEq for #ty #ty_generics
                 #where_clause
            {
                #eq_method
                #ne_method
            }
        }
        .to_tokens(tokens);
    }
}
