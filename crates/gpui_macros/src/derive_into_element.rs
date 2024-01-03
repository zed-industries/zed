use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_into_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics gpui::IntoElement for #type_name #type_generics
        #where_clause
        {
            type Element = gpui::Component<Self>;

            fn element_id(&self) -> Option<gpui::ElementId> {
                None
            }

            fn into_element(self) -> Self::Element {
                gpui::Component::new(self)
            }
        }
    };

    gen.into()
}
