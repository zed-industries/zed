use proc_macro::TokenStream;

use quote::quote;
use syn::{
    Data, DeriveInput, Field, Fields, ItemEnum, ItemStruct, Type, parse_macro_input, parse_quote,
};

/// Derives the `MergeFrom` trait for a struct.
///
/// This macro automatically implements `MergeFrom` by calling `merge_from`
/// on all fields in the struct.
///
/// # Example
///
/// ```ignore
/// #[derive(Clone, MergeFrom)]
/// struct MySettings {
///     field1: Option<String>,
///     field2: SomeOtherSettings,
/// }
/// ```
#[proc_macro_derive(MergeFrom)]
pub fn derive_merge_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let merge_body = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_merges = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    quote! {
                        self.#field_name.merge_from(&other.#field_name);
                    }
                });

                quote! {
                    #(#field_merges)*
                }
            }
            Fields::Unnamed(fields) => {
                let field_merges = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let field_index = syn::Index::from(i);
                    quote! {
                        self.#field_index.merge_from(&other.#field_index);
                    }
                });

                quote! {
                    #(#field_merges)*
                }
            }
            Fields::Unit => {
                quote! {
                    // No fields to merge for unit structs
                }
            }
        },
        Data::Enum(_) => {
            quote! {
                *self = other.clone();
            }
        }
        Data::Union(_) => {
            panic!("MergeFrom cannot be derived for unions");
        }
    };

    let expanded = quote! {
        impl #impl_generics crate::merge_from::MergeFrom for #name #ty_generics #where_clause {
            fn merge_from(&mut self, other: &Self) {
                use crate::merge_from::MergeFrom as _;
                #merge_body
            }
        }
    };

    TokenStream::from(expanded)
}

/// Registers the setting type with the SettingsStore. Note that you need to
/// have `gpui` in your dependencies for this to work.
#[proc_macro_derive(RegisterSetting)]
pub fn derive_register_setting(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let type_name = &input.ident;

    quote! {
        settings::private::inventory::submit! {
            settings::private::RegisteredSetting {
                settings_value: || {
                    Box::new(settings::private::SettingValue::<#type_name> {
                        global_value: None,
                        local_values: Vec::new(),
                    })
                },
                from_settings: |content| Box::new(<#type_name as settings::Settings>::from_settings(content)),
                id: || std::any::TypeId::of::<#type_name>(),
            }
        }
    }
    .into()
}

// Adds serde attributes to each field with type Option<T>:
// #serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "settings::deserialize_fallible")
#[proc_macro_attribute]
pub fn with_fallible_options(_args: TokenStream, input: TokenStream) -> TokenStream {
    fn apply_on_fields(fields: &mut Fields) {
        match fields {
            Fields::Unit => {}
            Fields::Named(fields) => {
                for field in &mut fields.named {
                    add_if_option(field)
                }
            }
            Fields::Unnamed(fields) => {
                for field in &mut fields.unnamed {
                    add_if_option(field)
                }
            }
        }
    }

    fn add_if_option(field: &mut Field) {
        match &field.ty {
            Type::Path(syn::TypePath { qself: None, path })
                if path.leading_colon.is_none()
                    && path.segments.len() == 1
                    && path.segments[0].ident == "Option" => {}
            _ => return,
        }
        let attr = parse_quote!(
            #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with="crate::fallible_options::deserialize")]
        );
        field.attrs.push(attr);
    }

    if let Ok(mut input) = syn::parse::<ItemStruct>(input.clone()) {
        apply_on_fields(&mut input.fields);
        quote!(#input).into()
    } else if let Ok(mut input) = syn::parse::<ItemEnum>(input) {
        for variant in &mut input.variants {
            apply_on_fields(&mut variant.fields);
        }
        quote!(#input).into()
    } else {
        panic!("with_fallible_options can only be applied to struct or enum definitions.");
    }
}
