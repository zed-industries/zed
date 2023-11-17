mod action;
mod derive_component;
mod register_action;
mod style_helpers;
mod test;

use proc_macro::TokenStream;

#[proc_macro]
pub fn style_helpers(args: TokenStream) -> TokenStream {
    style_helpers::style_helpers(args)
}

#[proc_macro_derive(Action)]
pub fn action(input: TokenStream) -> TokenStream {
    action::action(input)
}

#[proc_macro_attribute]
pub fn register_action(attr: TokenStream, item: TokenStream) -> TokenStream {
    register_action::register_action_macro(attr, item)
}

#[proc_macro_derive(Component, attributes(component))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    derive_component::derive_component(input)
}

#[proc_macro_attribute]
pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    test::test(args, function)
}
