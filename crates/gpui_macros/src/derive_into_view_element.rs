use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_into_view_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let r#gen = quote! {
        impl #impl_generics gpui::IntoElement for #type_name #type_generics
        #where_clause
        {
            type Element = gpui::ViewElement<Self>;

            fn into_element(self) -> Self::Element {
                let style = gpui::View::style(&self);
                let element = gpui::ViewElement::new(self);
                match style {
                    Some(s) => element.cached(s),
                    None => element,
                }
            }
        }
    };

    r#gen.into()
}
