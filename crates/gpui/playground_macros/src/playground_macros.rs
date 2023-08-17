use proc_macro::TokenStream;

mod derive_element;
mod derive_into_element;
mod derive_overrides;
mod tailwind_lengths;

#[proc_macro_derive(Element, attributes(element_crate))]
pub fn derive_element(input: TokenStream) -> TokenStream {
    derive_element::derive_element(input)
}

#[proc_macro_derive(IntoElement, attributes(element_crate))]
pub fn derive_into_element(input: TokenStream) -> TokenStream {
    derive_into_element::derive_into_element(input)
}

#[proc_macro_derive(Overrides, attributes(overrides_crate))]
pub fn derive_overrides(input: TokenStream) -> TokenStream {
    derive_overrides::derive_overrides(input)
}

#[proc_macro_attribute]
pub fn tailwind_lengths(attr: TokenStream, item: TokenStream) -> TokenStream {
    tailwind_lengths::tailwind_lengths(attr, item)
}
