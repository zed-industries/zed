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

    let scope_expr = match scope_val {
        Some(s) => quote! { #s },
        None => {
            return syn::Error::new_spanned(&input.ident, "Missing `scope` attribute")
                .to_compile_error()
                .into();
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
            fn scope() -> &'static str {
                #scope_expr
            }

            fn name() -> &'static str {
                stringify!(#name)
            }

            #description_impl
        }

        #[linkme::distributed_slice(component_system::__ALL_COMPONENTS)]
        fn __register_component() {
            component_system::COMPONENTS
                .lock()
                .unwrap()
                .push((
                    <#name as component_system::Component>::scope(),
                    <#name as component_system::Component>::name(),
                    <#name as component_system::Component>::description()
                ));
        }

        #[linkme::distributed_slice(component_system::__ALL_PREVIEWS)]
        fn __register_preview() {
            component_system::PREVIEWS
                .lock()
                .unwrap()
                .push(<#name as component_system::Component>::name());
        }
    };

    expanded.into()
}
