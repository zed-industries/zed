use proc_macro::TokenStream;

mod derive_element;
mod derive_into_element;
mod styleable_trait;
mod tailwind_lengths;

#[proc_macro]
pub fn styleable_trait(args: TokenStream) -> TokenStream {
    styleable_trait::styleable_trait(args)
}

#[proc_macro_derive(Element, attributes(element_crate))]
pub fn derive_element(input: TokenStream) -> TokenStream {
    derive_element::derive_element(input)
}

#[proc_macro_derive(IntoElement, attributes(element_crate))]
pub fn derive_into_element(input: TokenStream) -> TokenStream {
    derive_into_element::derive_into_element(input)
}

#[proc_macro_attribute]
pub fn tailwind_lengths(attr: TokenStream, item: TokenStream) -> TokenStream {
    tailwind_lengths::tailwind_lengths(attr, item)
}
