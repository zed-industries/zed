use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_register_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let reg_fn_name = syn::Ident::new(
        &format!("__component_registry_internal_register_{}", name),
        name.span(),
    );
    let expanded = quote! {
        const _: () = {
            struct AssertComponent<T: component::Component>(::std::marker::PhantomData<T>);
            let _ = AssertComponent::<#name>(::std::marker::PhantomData);
        };

        #[allow(non_snake_case)]
        #[linkme::distributed_slice(component::__ALL_COMPONENTS)]
        fn #reg_fn_name() {
            component::register_component::<#name>();
        }
    };
    expanded.into()
}
