// Input:
//
// #[action]
// struct Foo {}

// Output:
//
// #[gpui::register_action]
// #[derive(gpui::serde::Deserialize, std::cmp::PartialEq, std::clone::Clone, std::default::Default, std::fmt::Debug)]
// struct Foo {}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let data = &input.data;

    let data_tokens = quote! { #data }.into();

    let expanded = quote! {
        #[gpui::register_action]
        #[derive(gpui::serde::Deserialize, std::cmp::PartialEq, std::clone::Clone, std::default::Default, std::fmt::Debug)]
        struct #name { #data }
    };

    TokenStream::from(expanded)
}
