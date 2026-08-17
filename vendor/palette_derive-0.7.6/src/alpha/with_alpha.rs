use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::quote;
use syn::{parse_quote, DeriveInput, Generics, Ident, Type};

use crate::{
    meta::{
        parse_field_attributes, parse_namespaced_attributes, FieldAttributes, IdentOrIndex,
        TypeItemAttributes,
    },
    util,
};

pub fn derive(item: TokenStream) -> ::std::result::Result<TokenStream, Vec<::syn::parse::Error>> {
    let DeriveInput {
        ident,
        generics: original_generics,
        data,
        attrs,
        ..
    } = syn::parse(item).map_err(|error| vec![error])?;
    let generics = original_generics;

    let (item_meta, item_errors) = parse_namespaced_attributes::<TypeItemAttributes>(attrs);

    let (fields_meta, field_errors) = if let syn::Data::Struct(struct_data) = data {
        parse_field_attributes::<FieldAttributes>(struct_data.fields)
    } else {
        return Err(vec![syn::Error::new(
            Span::call_site(),
            "only structs are supported",
        )]);
    };

    let implementation = if let Some((alpha_property, alpha_type)) = fields_meta.alpha_property {
        implement_for_internal_alpha(&ident, &generics, &alpha_property, &alpha_type, &item_meta)
    } else {
        implement_for_external_alpha(&ident, &generics, &item_meta)
    };

    let item_errors = item_errors
        .into_iter()
        .map(|error| error.into_compile_error());
    let field_errors = field_errors
        .into_iter()
        .map(|error| error.into_compile_error());

    Ok(quote! {
        #(#item_errors)*
        #(#field_errors)*

        #implementation
    }
    .into())
}

fn implement_for_internal_alpha(
    ident: &Ident,
    generics: &Generics,
    alpha_property: &IdentOrIndex,
    alpha_type: &Type,
    item_meta: &TypeItemAttributes,
) -> TokenStream2 {
    let with_alpha_trait_path = util::path(["WithAlpha"], item_meta.internal);
    let stimulus_trait_path = util::path(["stimulus", "Stimulus"], item_meta.internal);

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics #with_alpha_trait_path<#alpha_type> for #ident #type_generics #where_clause {
            type Color = Self;
            type WithAlpha = Self;

            fn with_alpha(mut self, alpha: #alpha_type) -> Self::WithAlpha {
                self.#alpha_property = alpha;
                self
            }

            fn without_alpha(mut self) -> Self::Color {
                self.#alpha_property = #stimulus_trait_path::max_intensity();
                self
            }

            fn split(mut self) -> (Self::Color, #alpha_type) {
                let opaque_alpha = #stimulus_trait_path::max_intensity();
                let alpha = core::mem::replace(&mut self.#alpha_property, opaque_alpha);
                (self, alpha)
            }
        }
    }
}

fn implement_for_external_alpha(
    ident: &Ident,
    generics: &Generics,
    item_meta: &TypeItemAttributes,
) -> TokenStream2 {
    let with_alpha_trait_path = util::path(["WithAlpha"], item_meta.internal);
    let stimulus_trait_path = util::path(["stimulus", "Stimulus"], item_meta.internal);
    let alpha_path = util::path(["Alpha"], item_meta.internal);

    let (_, type_generics, _) = generics.split_for_impl();

    let alpha_type: Type = parse_quote!(_A);
    let mut impl_generics = generics.clone();
    impl_generics.params.push(parse_quote!(_A));
    impl_generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(_A: #stimulus_trait_path));
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics #with_alpha_trait_path<#alpha_type> for #ident #type_generics #where_clause {
            type Color = Self;
            type WithAlpha = #alpha_path<Self, #alpha_type>;

            fn with_alpha(self, alpha: #alpha_type) -> Self::WithAlpha {
                #alpha_path {
                    color: self,
                    alpha
                }
            }

            fn without_alpha(self) -> Self::Color {
                self
            }

            fn split(self) -> (Self::Color, #alpha_type) {
                (self, #stimulus_trait_path::max_intensity())
            }
        }
    }
}
