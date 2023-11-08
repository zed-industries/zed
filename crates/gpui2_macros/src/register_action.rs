// Input:
//
// struct FooBar {}

// Output:
//
// struct FooBar {}
//
// #[allow(non_snake_case)]
// #[gpui2::ctor]
// fn register_foobar_builder() {
//     gpui2::register_action_builder::<Foo>()
// }
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

pub fn register_action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let type_name = &input.ident;
    let ctor_fn_name = format_ident!("register_{}_builder", type_name);

    let expanded = quote! {
        #input
        #[allow(non_snake_case)]
        #[gpui::ctor]
        fn #ctor_fn_name() {
            gpui::register_action::<#type_name>()
        }
    };
    TokenStream::from(expanded)
}
