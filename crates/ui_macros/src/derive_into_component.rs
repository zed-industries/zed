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
        impl component_preview::Component for #name {
            fn scope() -> &'static str {
                #scope_expr
            }

            #description_impl
        }

        #[linkme::distributed_slice(component_preview::__ALL_COMPONENTS)]
        fn register_component() {
            component_preview::COMPONENTS
                .lock()
                .unwrap()
                .push((<#name as component_preview::Component>::scope(), <#name as component_preview::Component>::name(), <#name as component_preview::Component>::description()));
        }

        #[linkme::distributed_slice(component_preview::__ALL_PREVIEWS)]
        fn register_preview() {
            if let Some(p) = <#name as component_preview::Component>::preview() {
                component_preview::PREVIEWS.lock().unwrap().push(p);
            } else {
                let _ = <#name as component_preview::ComponentPreview>::preview as fn() -> &'static str;
                component_preview::PREVIEWS.lock().unwrap().push(<#name as component_preview::ComponentPreview>::preview());
            }
        }
    };

    expanded.into()
}
