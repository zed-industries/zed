use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Lit, Meta, MetaList, MetaNameValue, NestedMeta, parse_macro_input};

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
        let scope_str = s.clone();
        quote! {
            fn scope() -> Option<component::ComponentScope> {
                Some(component::ComponentScope::from(#scope_str))
            }
        }
    } else {
        quote! {
            fn scope() -> Option<component::ComponentScope> {
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

    let register_component_name = syn::Ident::new(
        &format!(
            "__register_component_{}",
            Casing::to_case(&name.to_string(), Case::Snake)
        ),
        name.span(),
    );
    let register_preview_name = syn::Ident::new(
        &format!(
            "__register_preview_{}",
            Casing::to_case(&name.to_string(), Case::Snake)
        ),
        name.span(),
    );

    let expanded = quote! {
        impl component::Component for #name {
            #scope_impl

            fn name() -> &'static str {
                stringify!(#name)
            }

            #description_impl
        }

        #[linkme::distributed_slice(component::__ALL_COMPONENTS)]
        fn #register_component_name() {
            component::register_component::<#name>();
        }

        #[linkme::distributed_slice(component::__ALL_PREVIEWS)]
        fn #register_preview_name() {
            component::register_preview::<#name>();
        }
    };

    expanded.into()
}
