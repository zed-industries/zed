use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics};

use crate::derive_into_element::impl_into_element;

pub fn derive_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = ast.ident;
    let placeholder_view_generics: Generics = parse_quote! { <V: 'static> };

    let (impl_generics, type_generics, where_clause, view_type_name, lifetimes) =
        if let Some(first_type_param) = ast.generics.params.iter().find_map(|param| {
            if let GenericParam::Type(type_param) = param {
                Some(type_param.ident.clone())
            } else {
                None
            }
        }) {
            let mut lifetimes = vec![];
            for param in ast.generics.params.iter() {
                if let GenericParam::Lifetime(lifetime_def) = param {
                    lifetimes.push(lifetime_def.lifetime.clone());
                }
            }
            let generics = ast.generics.split_for_impl();
            (
                generics.0,
                Some(generics.1),
                generics.2,
                first_type_param,
                lifetimes,
            )
        } else {
            let generics = placeholder_view_generics.split_for_impl();
            let placeholder_view_type_name: Ident = parse_quote! { V };
            (
                generics.0,
                None,
                generics.2,
                placeholder_view_type_name,
                vec![],
            )
        };

    let lifetimes = if !lifetimes.is_empty() {
        quote! { <#(#lifetimes),*> }
    } else {
        quote! {}
    };

    let impl_into_element = impl_into_element(
        &impl_generics,
        &view_type_name,
        &type_name,
        &type_generics,
        &where_clause,
    );

    let gen = quote! {
        impl #impl_generics playground::element::Element<#view_type_name> for #type_name #type_generics
        #where_clause
        {
            type PaintState = playground::element::AnyElement<#view_type_name #lifetimes>;

            fn layout(
                &mut self,
                view: &mut V,
                cx: &mut playground::element::LayoutContext<V>,
            ) -> anyhow::Result<(playground::element::LayoutId, Self::PaintState)> {
                let mut rendered_element = self.render(view, cx).into_element().into_any();
                let layout_id = rendered_element.layout(view, cx)?;
                Ok((layout_id, rendered_element))
            }

            fn paint(
                &mut self,
                view: &mut V,
                layout: &playground::element::Layout,
                rendered_element: &mut Self::PaintState,
                cx: &mut playground::element::PaintContext<V>,
            ) {
                rendered_element.paint(view, layout.bounds.origin(), cx);
            }
        }

        #impl_into_element
    };

    gen.into()
}
