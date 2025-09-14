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
                    // todo(settings_ui) rely entirely on settings_key, remove path attribute
                    if path_name.is_some() {
                        return Err(meta.error("Only one 'path' can be specified, either with `path` in `settings_ui` or with `settings_key`"));
                    }
                    meta.input.parse::<Token![=]>()?;
                    let lit: LitStr = meta.input.parse()?;
                    path_name = Some(lit.value());
                } else if meta.path.is_ident("render") {
                    // Just consume the tokens even if we don't use them here
                    meta.input.parse::<Token![=]>()?;
                    let _lit: LitStr = meta.input.parse()?;
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

    let doc_str = parse_documentation_from_attrs(&input.attrs);

    let ui_item_fn_body = generate_ui_item_body(group_name.as_ref(), &input);

    // todo(settings_ui): make group name optional, repurpose group as tag indicating item is group, and have "title" tag for custom title
    let title = group_name.unwrap_or(input.ident.to_string().to_title_case());

    let ui_entry_fn_body = map_ui_item_to_entry(
        path_name.as_deref(),
        &title,
        doc_str.as_deref(),
        quote! { Self },
    );

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

fn map_ui_item_to_entry(
    path: Option<&str>,
    title: &str,
    doc_str: Option<&str>,
    ty: TokenStream,
) -> TokenStream {
    // todo(settings_ui): does quote! just work with options?
    let path = path.map_or_else(|| quote! {None}, |path| quote! {Some(#path)});
    let doc_str = doc_str.map_or_else(|| quote! {None}, |doc_str| quote! {Some(#doc_str)});
    let item = ui_item_from_type(ty);
    quote! {
        settings::SettingsUiEntry {
            title: #title,
            path: #path,
            item: #item,
            documentation: #doc_str,
        }
    }
}

fn ui_item_from_type(ty: TokenStream) -> TokenStream {
    let ty = extract_type_from_option(ty);
    return trait_method_call(ty, quote! {settings::SettingsUi}, quote! {settings_ui_item});
}

fn trait_method_call(
    ty: TokenStream,
    trait_name: TokenStream,
    method_name: TokenStream,
) -> TokenStream {
    // doing the <ty as settings::SettingsUi> makes the error message better:
    //  -> "#ty Doesn't implement settings::SettingsUi" instead of "no item "settings_ui_item" for #ty"
    // and ensures safety against name conflicts
    //
    // todo(settings_ui): Turn `Vec<T>` into `Vec::<T>` here as well
    quote! {
        <#ty as #trait_name>::#method_name()
    }
}

fn generate_ui_item_body(group_name: Option<&String>, input: &syn::DeriveInput) -> TokenStream {
    match (group_name, &input.data) {
        (_, Data::Union(_)) => unimplemented!("Derive SettingsUi for Unions"),
        (None, Data::Struct(_)) => quote! {
            settings::SettingsUiItem::None
        },
        (Some(_), Data::Struct(data_struct)) => {
            let parent_serde_attrs = parse_serde_attributes(&input.attrs);
            item_group_from_fields(&data_struct.fields, &parent_serde_attrs)
        }
        (None, Data::Enum(data_enum)) => {
            let serde_attrs = parse_serde_attributes(&input.attrs);
            let render_as = parse_render_as(&input.attrs);
            let length = data_enum.variants.len();

            let mut variants = Vec::with_capacity(length);
            let mut labels = Vec::with_capacity(length);

            for variant in &data_enum.variants {
                // todo(settings_ui): Can #[serde(rename = )] be on enum variants?
                let ident = variant.ident.clone().to_string();
                let variant_name = serde_attrs.rename_all.apply(&ident);
                let title = variant_name.to_title_case();

                variants.push(variant_name);
                labels.push(title);
            }

            let is_not_union = data_enum.variants.iter().all(|v| v.fields.is_empty());
            if is_not_union {
                return match render_as {
                    RenderAs::ToggleGroup if length > 6 => {
                        panic!("Can't set toggle group with more than six entries");
                    }
                    RenderAs::ToggleGroup => {
                        quote! {
                            settings::SettingsUiItem::Single(settings::SettingsUiItemSingle::ToggleGroup{ variants: &[#(#variants),*], labels: &[#(#labels),*] })
                        }
                    }
                    RenderAs::Default => {
                        quote! {
                            settings::SettingsUiItem::Single(settings::SettingsUiItemSingle::DropDown{ variants: &[#(#variants),*], labels: &[#(#labels),*] })
                        }
                    }
                };
            }
            // else: Union!
            let enum_name = &input.ident;

            let options = data_enum.variants.iter().map(|variant| {
                if variant.fields.is_empty() {
                    return quote! {None};
                }
                let name = &variant.ident;
                let item = item_group_from_fields(&variant.fields, &serde_attrs);
                // todo(settings_ui): documentation
                return quote! {
                    Some(settings::SettingsUiEntry {
                        path: None,
                        title: stringify!(#name),
                        documentation: None,
                        item: #item,
                    })
                };
            });
            let defaults = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                if variant.fields.is_empty() {
                    quote! {
                        serde_json::to_value(#enum_name::#variant_name).expect("Failed to serialize default value for #enum_name::#variant_name")
                    }
                } else {
                    let fields = variant.fields.iter().enumerate().map(|(index, field)| {
                        let field_name = field.ident.as_ref().map_or_else(|| syn::Index::from(index).into_token_stream(), |ident| ident.to_token_stream());
                        let field_type_is_option = option_inner_type(field.ty.to_token_stream()).is_some();
                        let field_default = if field_type_is_option {
                            quote! {
                                None
                            }
                        } else {
                            quote! {
                                ::std::default::Default::default()
                            }
                        };

                        quote!{
                            #field_name: #field_default
                        }
                    });
                    quote! {
                        serde_json::to_value(#enum_name::#variant_name {
                            #(#fields),*
                        }).expect("Failed to serialize default value for #enum_name::#variant_name")
                    }
                }
            });
            // todo(settings_ui): Identify #[default] attr and use it for index, defaulting to 0
            let default_variant_index: usize = 0;
            let determine_option_fn = {
                let match_arms = data_enum
                    .variants
                    .iter()
                    .enumerate()
                    .map(|(index, variant)| {
                        let variant_name = &variant.ident;
                        quote! {
                            Ok(#variant_name {..}) => #index
                        }
                    });
                quote! {
                    |value: &serde_json::Value, _cx: &gpui::App| -> usize {
                        use #enum_name::*;
                        match serde_json::from_value::<#enum_name>(value.clone()) {
                            #(#match_arms),*,
                            Err(_) => #default_variant_index,
                        }
                    }
                }
            };
            // todo(settings_ui) should probably always use toggle group for unions, dropdown makes less sense
            return quote! {
                settings::SettingsUiItem::Union(settings::SettingsUiItemUnion {
                    defaults: Box::new([#(#defaults),*]),
                    labels: &[#(#labels),*],
                    options: Box::new([#(#options),*]),
                    determine_option: #determine_option_fn,
                })
            };
            // panic!("Unhandled");
        }
        // todo(settings_ui) discriminated unions
        (_, Data::Enum(_)) => quote! {
            settings::SettingsUiItem::None
        },
    }
}

fn item_group_from_fields(fields: &syn::Fields, parent_serde_attrs: &SerdeOptions) -> TokenStream {
    let group_items = fields
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
            let field_serde_attrs = parse_serde_attributes(&field.attrs);
            let name = field.ident.as_ref().map(ToString::to_string);
            let title = name.as_ref().map_or_else(
                || "todo(settings_ui): Titles for tuple fields".to_string(),
                |name| name.to_title_case(),
            );
            let doc_str = parse_documentation_from_attrs(&field.attrs);

            (
                title,
                doc_str,
                name.filter(|_| !field_serde_attrs.flatten).map(|name| {
                    parent_serde_attrs.apply_rename_to_field(&field_serde_attrs, &name)
                }),
                field.ty.to_token_stream(),
            )
        })
        // todo(settings_ui): Re-format field name as nice title, and support setting different title with attr
        .map(|(title, doc_str, path, ty)| {
            map_ui_item_to_entry(path.as_deref(), &title, doc_str.as_deref(), ty)
        });

    quote! {
        settings::SettingsUiItem::Group(settings::SettingsUiItemGroup{ items: vec![#(#group_items),*] })
    }
}

struct SerdeOptions {
    rename_all: SerdeRenameAll,
    rename: Option<String>,
    flatten: bool,
    untagged: bool,
    _alias: Option<String>, // todo(settings_ui)
}

#[derive(PartialEq)]
enum SerdeRenameAll {
    Lowercase,
    SnakeCase,
    None,
}

impl SerdeRenameAll {
    fn apply(&self, name: &str) -> String {
        match self {
            SerdeRenameAll::Lowercase => name.to_lowercase(),
            SerdeRenameAll::SnakeCase => name.to_snake_case(),
            SerdeRenameAll::None => name.to_string(),
        }
    }
}

impl SerdeOptions {
    fn apply_rename_to_field(&self, field_options: &Self, name: &str) -> String {
        // field renames take precedence over struct rename all cases
        if let Some(rename) = &field_options.rename {
            return rename.clone();
        }
        return self.rename_all.apply(name);
    }
}

enum RenderAs {
    ToggleGroup,
    Default,
}

fn parse_render_as(attrs: &[syn::Attribute]) -> RenderAs {
    let mut render_as = RenderAs::Default;

    for attr in attrs {
        if !attr.path().is_ident("settings_ui") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("render") {
                meta.input.parse::<Token![=]>()?;
                let lit = meta.input.parse::<LitStr>()?.value();

                if lit == "toggle_group" {
                    render_as = RenderAs::ToggleGroup;
                } else {
                    return Err(meta.error(format!("invalid `render` attribute: {}", lit)));
                }
            }
            Ok(())
        })
        .unwrap();
    }

    render_as
}

fn parse_serde_attributes(attrs: &[syn::Attribute]) -> SerdeOptions {
    let mut options = SerdeOptions {
        rename_all: SerdeRenameAll::None,
        rename: None,
        flatten: false,
        untagged: false,
        _alias: None,
    };

    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                meta.input.parse::<Token![=]>()?;
                let lit = meta.input.parse::<LitStr>()?.value();

                if options.rename_all != SerdeRenameAll::None {
                    return Err(meta.error("duplicate `rename_all` attribute"));
                } else if lit == "lowercase" {
                    options.rename_all = SerdeRenameAll::Lowercase;
                } else if lit == "snake_case" {
                    options.rename_all = SerdeRenameAll::SnakeCase;
                } else {
                    return Err(meta.error(format!("invalid `rename_all` attribute: {}", lit)));
                }
                // todo(settings_ui): Other options?
            } else if meta.path.is_ident("flatten") {
                options.flatten = true;
            } else if meta.path.is_ident("rename") {
                if options.rename.is_some() {
                    return Err(meta.error("Can only have one rename attribute"));
                }

                meta.input.parse::<Token![=]>()?;
                let lit = meta.input.parse::<LitStr>()?.value();
                options.rename = Some(lit);
            } else if meta.path.is_ident("untagged") {
                options.untagged = true;
            }
            Ok(())
        })
        .unwrap();
    }

    return options;
}

fn parse_documentation_from_attrs(attrs: &[syn::Attribute]) -> Option<String> {
    let mut doc_str = Option::<String>::None;
    for attr in attrs {
        if attr.path().is_ident("doc") {
            // /// ...
            // becomes
            // #[doc = "..."]
            use syn::{Expr::Lit, ExprLit, Lit::Str, Meta, MetaNameValue};
            if let Meta::NameValue(MetaNameValue {
                value:
                    Lit(ExprLit {
                        lit: Str(ref lit_str),
                        ..
                    }),
                ..
            }) = attr.meta
            {
                let doc = lit_str.value();
                let doc_str = doc_str.get_or_insert_default();
                doc_str.push_str(doc.trim());
                doc_str.push('\n');
            }
        }
    }
    return doc_str;
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
        settings_key = settings_key.or(parsed_settings_key);
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
