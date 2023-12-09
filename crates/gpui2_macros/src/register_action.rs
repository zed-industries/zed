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
use syn::{parse_macro_input, DeriveInput, Error};

pub fn register_action_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let registration = register_action(&input.ident);

    let has_action_derive = input
        .attrs
        .iter()
        .find(|attr| {
            (|| {
                let meta = attr.parse_meta().ok()?;
                meta.path().is_ident("derive").then(|| match meta {
                    syn::Meta::Path(_) => None,
                    syn::Meta::NameValue(_) => None,
                    syn::Meta::List(list) => list
                        .nested
                        .iter()
                        .find(|list| match list {
                            syn::NestedMeta::Meta(meta) => meta.path().is_ident("Action"),
                            syn::NestedMeta::Lit(_) => false,
                        })
                        .map(|_| true),
                })?
            })()
            .unwrap_or(false)
        })
        .is_some();

    if has_action_derive {
        return Error::new(
            input.ident.span(),
            "The Action derive macro has already registered this action",
        )
        .into_compile_error()
        .into();
    }

    TokenStream::from(quote! {
        #input

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
