use proc_macro::TokenStream;

mod derive_element;
mod style_helpers;

#[proc_macro]
pub fn style_helpers(args: TokenStream) -> TokenStream {
    style_helpers::style_helpers(args)
}

#[proc_macro_derive(Element, attributes(element_crate))]
pub fn derive_element(input: TokenStream) -> TokenStream {
    derive_element::derive_element(input)
}
