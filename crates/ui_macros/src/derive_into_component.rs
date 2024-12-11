use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta, MetaList, MetaNameValue, NestedMeta};

pub fn derive_into_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut scope_val = None;
    let mut description_val = None;

    for attr in &input.attrs {
        if attr.path.is_ident("component") {
            if let Ok(Meta::List(MetaList { nested, .. })) = attr.parse_meta() {
                for item in nested {
                    if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                        path,
                        lit: Lit::Str(s),
                        ..
                    })) = item
                    {
                        let ident = path.get_ident().map(|i| i.to_string()).unwrap_or_default();
                        if ident == "scope" {
                            scope_val = Some(s.value());
                        } else if ident == "description" {
                            description_val = Some(s.value());
                        }
                    }
                }
            }
        }
    }

    let name = &input.ident;

    let scope_impl = if let Some(s) = scope_val {
        quote! {
            fn scope() -> Option<&'static str> {
                Some(#s)
            }
        }
    } else {
        quote! {
            fn scope() -> Option<&'static str> {
                None
            }
        }
    };

    let description_impl = if let Some(desc) = description_val {
        quote! {
            fn description() -> Option<&'static str> {
                Some(#desc)
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl component_system::Component for #name {
            #scope_impl

            fn name() -> &'static str {
                stringify!(#name)
            }

            #description_impl
        }

        #[linkme::distributed_slice(component_system::__ALL_COMPONENTS)]
        fn __register_component() {
            component_system::register_component::<#name>();
        }

        #[linkme::distributed_slice(component_system::__ALL_PREVIEWS)]
        fn __register_preview() {
            component_system::register_preview::<#name>();
        }
    };

    expanded.into()
}
