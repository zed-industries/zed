// Input:
//
// #[action]
// struct Foo {
//   bar: String,
// }

// Output:
//
// #[gpui::register_action]
// #[derive(gpui::serde::Deserialize, std::cmp::PartialEq, std::clone::Clone, std::default::Default, std::fmt::Debug)]
// struct Foo {
//   bar: String,
// }

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let attrs = input
        .attrs
        .into_iter()
        .filter(|attr| !attr.path.is_ident("action"))
        .collect::<Vec<_>>();

    let attributes = quote! {
        #[gpui::register_action]
        #[derive(gpui::serde::Deserialize, std::cmp::PartialEq, std::clone::Clone, std::default::Default, std::fmt::Debug)]
        #(#attrs)*
    };
    let visibility = input.vis;

    let output = match input.data {
        syn::Data::Struct(ref struct_data) => {
            let fields = &struct_data.fields;
            quote! {
                #attributes
                #visibility struct #name #fields
            }
        }
        syn::Data::Enum(ref enum_data) => {
            let variants = &enum_data.variants;
            quote! {
                #attributes
                #visibility enum #name { #variants }
            }
        }
        _ => panic!("Expected a struct or an enum."),
    };

    TokenStream::from(output)
}
