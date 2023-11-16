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

    let static_slice_name =
        format_ident!("__GPUI_ACTIONS_{}", type_name.to_string().to_uppercase());

    let action_builder_fn_name = format_ident!(
        "__gpui_actions_builder_{}",
        type_name.to_string().to_lowercase()
    );

    let expanded = quote! {
        #input

        #[doc(hidden)]
        #[gpui::linkme::distributed_slice(gpui::__GPUI_ACTIONS)]
        #[linkme(crate = gpui::linkme)]
        static #static_slice_name: gpui::MacroActionBuilder = #action_builder_fn_name;

        /// This is an auto generated function, do not use.
        #[doc(hidden)]
        fn #action_builder_fn_name() -> gpui::ActionData {
            gpui::ActionData {
                name: ::std::any::type_name::<#type_name>(),
                type_id: ::std::any::TypeId::of::<#type_name>(),
                build: <#type_name as gpui::Action>::build,
            }
        }
    };

    TokenStream::from(expanded)
}
