use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Lit, Meta, NestedMeta, parse_macro_input};

pub fn derive_path_static_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let prefix = get_attr_value(&input.attrs, "prefix").unwrap_or_else(|| "".to_string());
    let suffix = get_attr_value(&input.attrs, "suffix").unwrap_or_else(|| "".to_string());
    let delimiter = get_attr_value(&input.attrs, "delimiter").unwrap_or_else(|| "/".to_string());

    let path_str_impl = impl_path_str(name, &input.data, &prefix, &suffix, &delimiter);

    let expanded = quote! {
        impl #name {
            pub fn path_str(&self) -> &'static str {
                #path_str_impl
            }
        }
    };

    TokenStream::from(expanded)
}

fn impl_path_str(
    name: &syn::Ident,
    data: &Data,
    prefix: &str,
    suffix: &str,
    delimiter: &str,
) -> proc_macro2::TokenStream {
    match *data {
        Data::Enum(ref data) => {
            let match_arms = data.variants.iter().map(|variant| {
                let ident = &variant.ident;
                let path = format!("{}{}{}{}{}", prefix, delimiter, ident, delimiter, suffix);
                quote! {
                    #name::#ident => #path,
                }
            });

            quote! {
                match self {
                    #(#match_arms)*
                }
            }
        }
        _ => panic!("DerivePathStr only supports enums"),
    }
}

fn get_attr_value(attrs: &[Attribute], key: &str) -> Option<String> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("derive_path_static_str"))
        .find_map(|attr| {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                meta_list.nested.iter().find_map(|nested_meta| {
                    if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested_meta {
                        if name_value.path.is_ident(key) {
                            if let Lit::Str(lit_str) = &name_value.lit {
                                return Some(lit_str.value());
                            }
                        }
                    }
                    None
                })
            } else {
                None
            }
        })
}
