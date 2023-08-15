use proc_macro::TokenStream;

mod derive_element;
mod tailwind_lengths;

#[proc_macro_attribute]
pub fn tailwind_lengths(attr: TokenStream, item: TokenStream) -> TokenStream {
    tailwind_lengths::tailwind_lengths(attr, item)
}
