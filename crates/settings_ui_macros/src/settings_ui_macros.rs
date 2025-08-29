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
/// use settings_ui_macros::SettingsUi;
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
                    // todo(settings_ui) try get KEY from Settings if possible, and once we do,
                    // if can get key from settings, throw error if path also passed
                    if path_name.is_some() {
                        return Err(meta.error("Only one 'path' can be specified"));
                    }
                    meta.input.parse::<Token![=]>()?;
                    let lit: LitStr = meta.input.parse()?;
                    path_name = Some(lit.value());
                }
                Ok(())
            })
            .unwrap_or_else(|e| panic!("in #[settings_ui] attribute: {}", e));
        }
    }

    if path_name.is_none() && group_name.is_some() {
        // todo(settings_ui) derive path from settings
        panic!("path is required when group is specified");
    }

    let ui_render_fn_body = generate_ui_item_body(group_name.as_ref(), path_name.as_ref(), &input);

    let settings_ui_item_fn_body = path_name
        .as_ref()
        .map(|path_name| map_ui_item_to_render(path_name, quote! { Self }))
        .unwrap_or(quote! {
            settings::SettingsUiEntry {
                item: settings::SettingsUiEntryVariant::None
            }
        });

    let expanded = quote! {
        impl #impl_generics settings::SettingsUi for #name #ty_generics #where_clause {
            fn settings_ui_item() -> settings::SettingsUiItem {
                #ui_render_fn_body
            }

            fn settings_ui_entry() -> settings::SettingsUiEntry {
                #settings_ui_item_fn_body
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn map_ui_item_to_render(path: &str, ty: TokenStream) -> TokenStream {
    quote! {
        settings::SettingsUiEntry {
            item: match #ty::settings_ui_item() {
                settings::SettingsUiItem::Group{title, items} => settings::SettingsUiEntryVariant::Group {
                    title,
                    path: #path,
                    items,
                },
                settings::SettingsUiItem::Single(item) => settings::SettingsUiEntryVariant::Item {
                    path: #path,
                    item,
                },
                settings::SettingsUiItem::None => settings::SettingsUiEntryVariant::None,
            }
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
        (None, None, Data::Struct(_)) => quote! {
            settings::SettingsUiItem::None
        },
        (Some(_), None, Data::Struct(_)) => quote! {
            settings::SettingsUiItem::None
        },
        (None, Some(_), Data::Struct(_)) => quote! {
            settings::SettingsUiItem::None
        },
        (Some(group_name), _, Data::Struct(data_struct)) => {
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
                .map(|(name, ty)| map_ui_item_to_render(&name, ty));

            quote! {
                settings::SettingsUiItem::Group{ title: #group_name, items: vec![#(#fields),*] }
            }
        }
        (None, _, Data::Enum(data_enum)) => {
            let mut lowercase = false;
            for attr in &input.attrs {
                if attr.path().is_ident("serde") {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("rename_all") {
                            meta.input.parse::<Token![=]>()?;
                            let lit = meta.input.parse::<LitStr>()?.value();
                            // todo(settings_ui) snake case
                            lowercase = lit == "lowercase" || lit == "snake_case";
                        }
                        Ok(())
                    })
                    .ok();
                }
            }
            let length = data_enum.variants.len();

            let variants = data_enum.variants.iter().map(|variant| {
                let string = variant.ident.clone().to_string();

                if lowercase {
                    string.to_lowercase()
                } else {
                    string
                }
            });

            if length > 6 {
                quote! {
                    settings::SettingsUiItem::Single(settings::SettingsUiItemSingle::DropDown(&[#(#variants),*]))
                }
            } else {
                quote! {
                    settings::SettingsUiItem::Single(settings::SettingsUiItemSingle::ToggleGroup(&[#(#variants),*]))
                }
            }
        }
        // todo(settings_ui) discriminated unions
        (_, _, Data::Enum(_)) => quote! {
            settings::SettingsUiItem::None
        },
    }
}
