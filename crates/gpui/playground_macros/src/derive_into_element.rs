use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics, Ident, Lit, Meta,
    WhereClause,
};

pub fn derive_into_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = ast.ident;

    let crate_name: String = ast
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path.is_ident("element_crate") {
                match attr.parse_meta() {
                    Ok(Meta::NameValue(nv)) => {
                        if let Lit::Str(s) = nv.lit {
                            Some(s.value())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .unwrap_or_else(|| String::from("playground"));
    let crate_name = format_ident!("{}", crate_name);

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
        &crate_name,
        &view_type_name,
        &type_name,
        &type_generics,
        &where_clause,
    )
    .into()
}

pub fn impl_into_element(
    impl_generics: &syn::ImplGenerics<'_>,
    crate_name: &Ident,
    view_type_name: &Ident,
    type_name: &Ident,
    type_generics: &Option<syn::TypeGenerics<'_>>,
    where_clause: &Option<&WhereClause>,
) -> proc_macro2::TokenStream {
    quote! {
        impl #impl_generics #crate_name::element::IntoElement<#view_type_name> for #type_name #type_generics
        #where_clause
        {
            type Element = Self;

            fn into_element(self) -> Self {
                self
            }
        }
    }
}
