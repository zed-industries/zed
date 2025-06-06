use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_into_element(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let r#gen = quote! {
        impl #impl_generics gpui::IntoElement for #type_name #type_generics
        #where_clause
        {
            type Element = gpui::Component<Self>;

            #[track_caller]
            fn into_element(self) -> Self::Element {
                gpui::Component::new(self)
            }
        }
    };

    r#gen.into()
}
