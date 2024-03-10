use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_render(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let type_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics gpui::Render for #type_name #type_generics
        #where_clause
        {
            fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::Element {
                gpui::Empty
            }
        }
    };

    gen.into()
}
