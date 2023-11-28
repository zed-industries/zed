mod action;
mod derive_into_element;
mod register_action;
mod style_helpers;
mod test;

use proc_macro::TokenStream;

#[proc_macro_derive(Action)]
pub fn action(input: TokenStream) -> TokenStream {
    action::action(input)
}

#[proc_macro_attribute]
pub fn register_action(attr: TokenStream, item: TokenStream) -> TokenStream {
    register_action::register_action_macro(attr, item)
}

#[proc_macro_derive(IntoElement)]
pub fn derive_into_element(input: TokenStream) -> TokenStream {
    derive_into_element::derive_into_element(input)
}

#[proc_macro]
pub fn style_helpers(input: TokenStream) -> TokenStream {
    style_helpers::style_helpers(input)
}

#[proc_macro_attribute]
pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    test::test(args, function)
}
