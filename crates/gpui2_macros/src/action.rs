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
use syn::{parse_macro_input, DeriveInput, Error};

use crate::register_action::register_action;

pub fn action(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    if input.generics.lt_token.is_some() {
        return Error::new(name.span(), "Actions must be a concrete type")
            .into_compile_error()
            .into();
    }

    let is_unit_struct = match input.data {
        syn::Data::Struct(struct_data) => struct_data.fields.is_empty(),
        syn::Data::Enum(_) => false,
        syn::Data::Union(_) => false,
    };

    let build_impl = if is_unit_struct {
        quote! {
            let _ = value;
            Ok(std::boxed::Box::new(Self {}))
        }
    } else {
        quote! {
            Ok(std::boxed::Box::new(gpui::serde_json::from_value::<Self>(value)?))
        }
    };

    let register_action = register_action(&name);

    let output = quote! {
        const _: fn() = || {
            fn assert_impl<T: ?Sized + for<'a> gpui::serde::Deserialize<'a> +  ::std::cmp::PartialEq + ::std::clone::Clone>() {}
            assert_impl::<#name>();
        };

        impl gpui::Action for #name {
            fn name(&self) -> &'static str
            {
                ::std::any::type_name::<#name>()
            }

            fn debug_name() -> &'static str
            where
                Self: ::std::marker::Sized
            {
                ::std::any::type_name::<#name>()
            }

            fn build(value: gpui::serde_json::Value) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>>
            where
                Self: ::std::marker::Sized {
                    #build_impl
            }

            fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
                action
                    .as_any()
                    .downcast_ref::<Self>()
                    .map_or(false, |a| self == a)
            }

            fn boxed_clone(&self) ->  std::boxed::Box<dyn gpui::Action> {
                ::std::boxed::Box::new(self.clone())
            }

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }
        }

        #register_action
    };

    TokenStream::from(output)
}
