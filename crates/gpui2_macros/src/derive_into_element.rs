use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics, Ident, WhereClause,
};

pub fn derive_into_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = ast.ident;

    let placeholder_view_generics: Generics = parse_quote! { <V: 'static> };
    let placeholder_view_type_name: Ident = parse_quote! { V };
    let view_type_name: Ident;
    let impl_generics: syn::ImplGenerics<'_>;
    let type_generics: Option<syn::TypeGenerics<'_>>;
    let where_clause: Option<&'_ WhereClause>;

    match ast.generics.params.iter().find_map(|param| {
        if let GenericParam::Type(type_param) = param {
            Some(type_param.ident.clone())
        } else {
            None
        }
    }) {
        Some(type_name) => {
            view_type_name = type_name;
            let generics = ast.generics.split_for_impl();
            impl_generics = generics.0;
            type_generics = Some(generics.1);
            where_clause = generics.2;
        }
        _ => {
            view_type_name = placeholder_view_type_name;
            let generics = placeholder_view_generics.split_for_impl();
            impl_generics = generics.0;
            type_generics = None;
            where_clause = generics.2;
        }
    }

    impl_into_element(
        &impl_generics,
        &view_type_name,
        &type_name,
        &type_generics,
        &where_clause,
    )
    .into()
}

pub fn impl_into_element(
    impl_generics: &syn::ImplGenerics<'_>,
    view_type_name: &Ident,
    type_name: &Ident,
    type_generics: &Option<syn::TypeGenerics<'_>>,
    where_clause: &Option<&WhereClause>,
) -> proc_macro2::TokenStream {
    quote! {
        impl #impl_generics playground::element::IntoElement<#view_type_name> for #type_name #type_generics
        #where_clause
        {
            type Element = Self;

            fn into_element(self) -> Self {
                self
            }
        }
    }
}
