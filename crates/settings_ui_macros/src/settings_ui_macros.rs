use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr, Token, parse_macro_input};

/// Derive macro for the `SettingsUI` marker trait.
///
/// This macro automatically implements the `SettingsUI` trait for the annotated type.
/// The `SettingsUI` trait is a marker trait used to indicate that a type can be
/// displayed in the settings UI.
///
/// # Example
///
/// ```
/// use settings::SettingsUI;
/// use settings_ui_macros::SettingsUI;
///
/// #[derive(SettingsUI)]
/// #[settings_ui(group = "Standard")]
/// struct MySettings {
///     enabled: bool,
///     count: usize,
/// }
/// ```
#[proc_macro_derive(SettingsUI, attributes(settings_ui))]
pub fn derive_settings_ui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Handle generic parameters if present
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut group_name = Option::<String>::None;

    for attr in &input.attrs {
        if attr.path().is_ident("settings_ui") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("group") {
                    if group_name.is_some() {
                        return Err(meta.error("Only one 'group' path can be specified"));
                    }
                    meta.input.parse::<Token![=]>()?;
                    let lit: LitStr = meta.input.parse()?;
                    group_name = Some(lit.value());
                }
                Ok(())
            })
            .unwrap_or_else(|e| panic!("in #[settings_ui] attribute: {}", e));
        }
    }

    let ui_item_fn_body = if let Some(group_name) = group_name {
        quote! {
            settings::SettingsUIItem { item: settings::SettingsUIItemVariant::Group{ path: #group_name, group: settings::SettingsUIItemGroup{ items: Default::default() } } }
        }
    } else {
        quote! {
            settings::SettingsUIItem { item: settings::SettingsUIItemVariant::None }
        }
    };

    let expanded = quote! {
        impl #impl_generics settings::SettingsUI for #name #ty_generics #where_clause {
            fn ui_item() -> settings::SettingsUIItem {
                #ui_item_fn_body
            }
        }
    };

    TokenStream::from(expanded)
}
