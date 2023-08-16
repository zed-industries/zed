use proc_macro::TokenStream;

mod derive_element;
mod tailwind_lengths;

#[proc_macro_attribute]
pub fn tailwind_lengths(attr: TokenStream, item: TokenStream) -> TokenStream {
    tailwind_lengths::tailwind_lengths(attr, item)
}

#[proc_macro_derive(Element, attributes(element_crate))]
pub fn derive_element(input: TokenStream) -> TokenStream {
    derive_element::derive_element(input)
}
