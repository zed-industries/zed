use heck::{ToSnakeCase as _, ToTitleCase as _};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, LitStr, Token, parse_macro_input};

/// Derive macro for the `SettingsUi` marker trait.
///
/// This macro automatically implements the `SettingsUi` trait for the annotated type.
/// The `SettingsUi` trait is a marker trait used to indicate that a type can be
/// displayed in the settings UI.
///
/// # Example
///
/// ```
/// use settings::SettingsUi;
///
/// #[derive(SettingsUi)]
/// #[settings_ui(group = "Standard")]
/// struct MySettings {
///     enabled: bool,
///     count: usize,
/// }
/// ```
#[proc_macro_derive(SettingsUi, attributes(settings_ui))]
pub fn derive_settings_ui(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Handle generic parameters if present
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut group_name = Option::<String>::None;
    let mut path_name = Option::<String>::None;

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
                } else if meta.path.is_ident("path") {
                    // todo(settings_ui) rely entirely on settings_key, remove path attribute
                    if path_name.is_some() {
                        return Err(meta.error("Only one 'path' can be specified, either with `path` in `settings_ui` or with `settings_key`"));
                    }
                    meta.input.parse::<Token![=]>()?;
                    let lit: LitStr = meta.input.parse()?;
                    path_name = Some(lit.value());
                }
                Ok(())
            })
            .unwrap_or_else(|e| panic!("in #[settings_ui] attribute: {}", e));
        } else if let Some(settings_key) = parse_setting_key_attr(attr) {
            // todo(settings_ui) either remove fallback key or handle it here
            if path_name.is_some() && settings_key.key.is_some() {
                panic!("Both 'path' and 'settings_key' are specified. Must specify only one");
            }
            path_name = settings_key.key;
        }
    }

    let ui_item_fn_body = generate_ui_item_body(group_name.as_ref(), path_name.as_ref(), &input);

    // todo(settings_ui): make group name optional, repurpose group as tag indicating item is group, and have "title" tag for custom title
    let title = group_name.unwrap_or(input.ident.to_string().to_title_case());

    let ui_entry_fn_body = map_ui_item_to_entry(path_name.as_deref(), &title, quote! { Self });

    let expanded = quote! {
        impl #impl_generics settings::SettingsUi for #name #ty_generics #where_clause {
            fn settings_ui_item() -> settings::SettingsUiItem {
                #ui_item_fn_body
            }

            fn settings_ui_entry() -> settings::SettingsUiEntry {
                #ui_entry_fn_body
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn extract_type_from_option(ty: TokenStream) -> TokenStream {
    match option_inner_type(ty.clone()) {
        Some(inner_type) => inner_type,
        None => ty,
    }
}

fn option_inner_type(ty: TokenStream) -> Option<TokenStream> {
    let ty = syn::parse2::<syn::Type>(ty).ok()?;
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    let arg = args.args.first()?;
    let syn::GenericArgument::Type(ty) = arg else {
        return None;
    };
    return Some(ty.to_token_stream());
}

fn map_ui_item_to_entry(path: Option<&str>, title: &str, ty: TokenStream) -> TokenStream {
    let ty = extract_type_from_option(ty);
    let path = path.map_or_else(|| quote! {None}, |path| quote! {Some(#path)});
    quote! {
        settings::SettingsUiEntry {
            title: #title,
            path: #path,
            item: #ty::settings_ui_item(),
        }
    }
}

fn generate_ui_item_body(
    group_name: Option<&String>,
    path_name: Option<&String>,
    input: &syn::DeriveInput,
) -> TokenStream {
    match (group_name, path_name, &input.data) {
        (_, _, Data::Union(_)) => unimplemented!("Derive SettingsUi for Unions"),
        (None, _, Data::Struct(_)) => quote! {
            settings::SettingsUiItem::None
        },
        (Some(_), _, Data::Struct(data_struct)) => {
            let fields = data_struct
                .fields
                .iter()
                .filter(|field| {
                    !field.attrs.iter().any(|attr| {
                        let mut has_skip = false;
                        if attr.path().is_ident("settings_ui") {
                            let _ = attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("skip") {
                                    has_skip = true;
                                }
                                Ok(())
                            });
                        }

                        has_skip
                    })
                })
                .map(|field| {
                    (
                        field.ident.clone().expect("tuple fields").to_string(),
                        field.ty.to_token_stream(),
                    )
                })
                // todo(settings_ui): Re-format field name as nice title, and support setting different title with attr
                .map(|(name, ty)| map_ui_item_to_entry(Some(&name), &name.to_title_case(), ty));

            quote! {
                settings::SettingsUiItem::Group(settings::SettingsUiItemGroup{ items: vec![#(#fields),*] })
            }
        }
        (None, _, Data::Enum(data_enum)) => {
            let mut lowercase = false;
            let mut snake_case = false;
            for attr in &input.attrs {
                if attr.path().is_ident("serde") {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("rename_all") {
                            meta.input.parse::<Token![=]>()?;
                            let lit = meta.input.parse::<LitStr>()?.value();
                            lowercase = lit == "lowercase";
                            snake_case = lit == "snake_case";
                        }
                        Ok(())
                    })
                    .ok();
                }
            }
            let length = data_enum.variants.len();

            let variants = data_enum.variants.iter().map(|variant| {
                let string = variant.ident.clone().to_string();

                let title = string.to_title_case();
                let string = if lowercase {
                    string.to_lowercase()
                } else if snake_case {
                    string.to_snake_case()
                } else {
                    string
                };

                (string, title)
            });

            let (variants, labels): (Vec<_>, Vec<_>) = variants.unzip();

            if length > 6 {
                quote! {
                    settings::SettingsUiItem::Single(settings::SettingsUiItemSingle::DropDown{ variants: &[#(#variants),*], labels: &[#(#labels),*] })
                }
            } else {
                quote! {
                    settings::SettingsUiItem::Single(settings::SettingsUiItemSingle::ToggleGroup{ variants: &[#(#variants),*], labels: &[#(#labels),*] })
                }
            }
        }
        // todo(settings_ui) discriminated unions
        (_, _, Data::Enum(_)) => quote! {
            settings::SettingsUiItem::None
        },
    }
}

struct SettingsKey {
    key: Option<String>,
    fallback_key: Option<String>,
}

fn parse_setting_key_attr(attr: &syn::Attribute) -> Option<SettingsKey> {
    if !attr.path().is_ident("settings_key") {
        return None;
    }

    let mut settings_key = SettingsKey {
        key: None,
        fallback_key: None,
    };

    let mut found_none = false;

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("None") {
            found_none = true;
        } else if meta.path.is_ident("key") {
            if settings_key.key.is_some() {
                return Err(meta.error("Only one 'group' path can be specified"));
            }
            meta.input.parse::<Token![=]>()?;
            let lit: LitStr = meta.input.parse()?;
            settings_key.key = Some(lit.value());
        } else if meta.path.is_ident("fallback_key") {
            if found_none {
                return Err(meta.error("Cannot specify 'fallback_key' and 'None'"));
            }

            if settings_key.fallback_key.is_some() {
                return Err(meta.error("Only one 'fallback_key' can be specified"));
            }

            meta.input.parse::<Token![=]>()?;
            let lit: LitStr = meta.input.parse()?;
            settings_key.fallback_key = Some(lit.value());
        }
        Ok(())
    })
    .unwrap_or_else(|e| panic!("in #[settings_key] attribute: {}", e));

    if found_none && settings_key.fallback_key.is_some() {
        panic!("in #[settings_key] attribute: Cannot specify 'None' and 'fallback_key'");
    }
    if found_none && settings_key.key.is_some() {
        panic!("in #[settings_key] attribute: Cannot specify 'None' and 'key'");
    }
    if !found_none && settings_key.key.is_none() {
        panic!("in #[settings_key] attribute: 'key' must be specified");
    }

    return Some(settings_key);
}

#[proc_macro_derive(SettingsKey, attributes(settings_key))]
pub fn derive_settings_key(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Handle generic parameters if present
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut settings_key = Option::<SettingsKey>::None;

    for attr in &input.attrs {
        let parsed_settings_key = parse_setting_key_attr(attr);
        if parsed_settings_key.is_some() && settings_key.is_some() {
            panic!("Duplicate #[settings_key] attribute");
        }
        settings_key = parsed_settings_key;
    }

    let Some(SettingsKey { key, fallback_key }) = settings_key else {
        panic!("Missing #[settings_key] attribute");
    };

    let key = key.map_or_else(|| quote! {None}, |key| quote! {Some(#key)});
    let fallback_key = fallback_key.map_or_else(
        || quote! {None},
        |fallback_key| quote! {Some(#fallback_key)},
    );

    let expanded = quote! {
        impl #impl_generics settings::SettingsKey for #name #ty_generics #where_clause {
            const KEY: Option<&'static str> = #key;

            const FALLBACK_KEY: Option<&'static str> = #fallback_key;
        };
    };

    proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use syn::{Attribute, parse_quote};

    use super::*;

    #[test]
    fn test_extract_key() {
        let input: Attribute = parse_quote!(
            #[settings_key(key = "my_key")]
        );
        let settings_key = parse_setting_key_attr(&input).unwrap();
        assert_eq!(settings_key.key, Some("my_key".to_string()));
        assert_eq!(settings_key.fallback_key, None);
    }

    #[test]
    fn test_empty_key() {
        let input: Attribute = parse_quote!(
            #[settings_key(None)]
        );
        let settings_key = parse_setting_key_attr(&input).unwrap();
        assert_eq!(settings_key.key, None);
        assert_eq!(settings_key.fallback_key, None);
    }
}
