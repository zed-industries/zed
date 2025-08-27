use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
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
                    // todo! try get KEY from Settings if possible, and once we do,
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

    // let mut root_item = vec![];
    // for field in struct {
    //
    // match field::settings_ui_render()
    // Group(items) => {
    // let item = items.map(|item| something);
    // item
    //     items.push(item::settings_ui_render());
    // root_item.push(Group(items));
    // },
    // Leaf(item) => {
    // root_item.push(item);
    // }
    // }
    // }
    //
    // group.items = struct.fields.map((field_name, field_type) => quote! { SettingsUIItem::Item {path: #field_type::settings_ui_path().unwrap_or_else(|| #field_name), item:  if field.attrs.render { #render } else field::settings_ui_render()}})
    // }

    fn map_ui_item_to_render(path: &str, ty: TokenStream) -> TokenStream {
        quote! {
            settings::SettingsUIItem {
                item: match #ty::settings_ui_render() {
                    settings::SettingsUIRender::Group{title, items} => settings::SettingsUIItemVariant::Group {
                        title,
                        path: #path,
                        group: settings::SettingsUIItemGroup { items },
                    },
                    settings::SettingsUIRender::Item(item) => settings::SettingsUIItemVariant::Item {
                        path: #path,
                        item,
                    },
                    settings::SettingsUIRender::None => settings::SettingsUIItemVariant::None,
                }
            }
        }
    }

    let ui_render_fn_body = if let Some(group_name) = group_name {
        let fields = match input.data {
            syn::Data::Struct(data_struct) => data_struct
                .fields
                .iter()
                .map(|field| {
                    (
                        field.ident.clone().expect("tuple fields").to_string(),
                        field.ty.to_token_stream(),
                    )
                })
                .collect(),
            syn::Data::Enum(data_enum) => vec![], // todo! enums
            syn::Data::Union(data_union) => unimplemented!("Derive SettingsUI for unions"),
        };
        let items = fields
            .into_iter()
            .map(|(name, ty)| map_ui_item_to_render(&name, ty));
        quote! {
            settings::SettingsUIRender::Group{ title: #group_name, items: vec![#(#items),*] }
        }
    } else {
        quote! {
            settings::SettingsUIRender::None
        }
    };

    let settings_ui_item_fn_body = map_ui_item_to_render(
        path_name.as_deref().unwrap_or("todo! no path specified"),
        quote! { Self },
    );

    let expanded = quote! {
        impl #impl_generics settings::SettingsUI for #name #ty_generics #where_clause {
            fn settings_ui_render() -> settings::SettingsUIRender {
                #ui_render_fn_body
            }

            fn settings_ui_item() -> settings::SettingsUIItem {
                #settings_ui_item_fn_body
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
