use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics, Ident, Lit, Meta,
    WhereClause,
};

use crate::derive_into_element::impl_into_element;

pub fn derive_element(input: TokenStream) -> TokenStream {
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

    let impl_into_element = impl_into_element(
        &impl_generics,
        &crate_name,
        &view_type_name,
        &type_name,
        &type_generics,
        &where_clause,
    );

    let gen = quote! {
        impl #impl_generics #crate_name::element::Element<#view_type_name> for #type_name #type_generics
        #where_clause
        {
            type Layout = #crate_name::element::AnyElement<V>;

            fn metadata(&mut self) -> &mut #crate_name::element::ElementMetadata<V> {
                &mut self.metadata
            }

            fn layout(
                &mut self,
                view: &mut V,
                cx: &mut #crate_name::element::LayoutContext<V>,
            ) -> anyhow::Result<(taffy::tree::NodeId, Self::Layout)> {
                let mut element = self.render(view, cx).into_any();
                let node_id = element.layout(view, cx)?;
                Ok((node_id, element))
            }

            fn paint<'a>(
                &mut self,
                layout: #crate_name::element::Layout<'a, Self::Layout>,
                view: &mut V,
                cx: &mut #crate_name::element::PaintContext<V>,
            ) -> anyhow::Result<()> {
                layout.from_element.paint(view, cx)?;
                Ok(())
            }
        }

        #impl_into_element
    };

    gen.into()
}
