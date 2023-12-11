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
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse_macro_input;

pub fn register_action_macro(ident: TokenStream) -> TokenStream {
    let name = parse_macro_input!(ident as Ident);
    let registration = register_action(&name);

    TokenStream::from(quote! {
        #registration
    })
}

pub(crate) fn register_action(type_name: &Ident) -> proc_macro2::TokenStream {
    let static_slice_name =
        format_ident!("__GPUI_ACTIONS_{}", type_name.to_string().to_uppercase());

    let action_builder_fn_name = format_ident!(
        "__gpui_actions_builder_{}",
        type_name.to_string().to_lowercase()
    );

    quote! {
        #[doc(hidden)]
        #[gpui::linkme::distributed_slice(gpui::__GPUI_ACTIONS)]
        #[linkme(crate = gpui::linkme)]
        static #static_slice_name: gpui::MacroActionBuilder = #action_builder_fn_name;

        /// This is an auto generated function, do not use.
        #[doc(hidden)]
        fn #action_builder_fn_name() -> gpui::ActionData {
            gpui::ActionData {
                name: <#type_name as gpui::Action>::debug_name(),
                type_id: ::std::any::TypeId::of::<#type_name>(),
                build: <#type_name as gpui::Action>::build,
            }
        }
    }
}
