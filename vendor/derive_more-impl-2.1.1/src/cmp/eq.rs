//! Implementation of an [`Eq`] derive macro.

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, spanned::Spanned as _};

use crate::utils::{
    attr::{self, ParseMultiple as _},
    structural_inclusion::TypeExt as _,
    HashSet,
};

/// Expands an [`Eq`] derive macro.
pub fn expand(input: &syn::DeriveInput, _: &'static str) -> syn::Result<TokenStream> {
    let attr_name = format_ident!("eq");
    let secondary_attr_name = format_ident!("partial_eq");

    let mut fields_types = HashSet::default();
    match &input.data {
        syn::Data::Struct(data) => {
            let mut is_skipped = false;
            for attr_name in [&attr_name, &secondary_attr_name] {
                if attr::Skip::parse_attrs(&input.attrs, attr_name)?.is_some() {
                    is_skipped = true;
                    break;
                }
            }
            if !is_skipped {
                'fields: for field in &data.fields {
                    for attr_name in [&attr_name, &secondary_attr_name] {
                        if attr::Skip::parse_attrs(&field.attrs, attr_name)?.is_some() {
                            continue 'fields;
                        }
                    }
                    _ = fields_types.insert(&field.ty);
                }
            }
        }
        syn::Data::Enum(data) => {
            'variants: for variant in &data.variants {
                for attr_name in [&attr_name, &secondary_attr_name] {
                    if attr::Skip::parse_attrs(&variant.attrs, attr_name)?.is_some() {
                        continue 'variants;
                    }
                }
                'fields: for field in &variant.fields {
                    for attr_name in [&attr_name, &secondary_attr_name] {
                        if attr::Skip::parse_attrs(&field.attrs, attr_name)?.is_some() {
                            continue 'fields;
                        }
                    }
                    _ = fields_types.insert(&field.ty);
                }
            }
        }
        syn::Data::Union(data) => {
            return Err(syn::Error::new(
                data.union_token.span(),
                "`Eq` cannot be derived for unions",
            ))
        }
    }

    Ok(StructuralExpansion {
        self_ty: (&input.ident, &input.generics),
        fields_types,
    }
    .into_token_stream())
}

/// Expansion of a macro for generating a structural [`Eq`] implementation of an enum or a struct.
struct StructuralExpansion<'i> {
    /// [`syn::Ident`] and [`syn::Generics`] of the enum/struct.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    self_ty: (&'i syn::Ident, &'i syn::Generics),

    /// [`syn::Type`]s of the enum/struct fields to be asserted for implementing [`Eq`].
    fields_types: HashSet<&'i syn::Type>,
}

impl ToTokens for StructuralExpansion<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = self.self_ty.0;
        let (_, ty_generics, _) = self.self_ty.1.split_for_impl();

        let mut asserted_types = vec![];
        let mut generics = self.self_ty.1.clone();
        if !generics.params.is_empty() {
            generics
                .make_where_clause()
                .predicates
                .push(parse_quote! { Self: derive_more::core::cmp::PartialEq });
        }
        {
            let self_ty: syn::Type = parse_quote! { Self };
            let implementor_ty: syn::Type = parse_quote! { #ty #ty_generics };
            for field_ty in &self.fields_types {
                if field_ty.contains_type_structurally(&self_ty)
                    || field_ty.contains_type_structurally(&implementor_ty)
                {
                    asserted_types.push(field_ty);
                } else {
                    generics
                        .make_where_clause()
                        .predicates
                        .push(parse_quote! { #field_ty: derive_more::core::cmp::Eq });
                }
            }
        }
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        // For the types containing the implementor type structurally, we should place a static
        // assertion instead of generating a trait bound, because recursive trait bound will result
        // in the `error[E0275]: overflow evaluating the requirement`.
        // https://doc.rust-lang.org/stable/error_codes/E0275.html
        let assert_eq_inherent_method = (!asserted_types.is_empty()).then(|| {
            quote! {
                #[allow(dead_code, private_bounds)]
                #[automatically_derived]
                #[doc(hidden)]
                impl #impl_generics #ty #ty_generics #where_clause {
                    #[doc(hidden)]
                    const fn __derive_more_assert_eq() {
                        #(let _: derive_more::__private::AssertParamIsEq<#asserted_types>;)*
                    }
                }
            }
        });

        quote! {
            #[allow(private_bounds)]
            #[automatically_derived]
            impl #impl_generics derive_more::core::cmp::Eq for #ty #ty_generics
                 #where_clause
            {}

            #assert_eq_inherent_method
        }
        .to_tokens(tokens);
    }
}
