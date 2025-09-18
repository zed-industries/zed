use proc_macro::TokenStream;
use quote::quote;
use settings_macro::MergeFrom;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

/// Derives the `MergeFrom` trait for a struct.
///
/// This macro automatically implements `MergeFrom` by calling `merge_from`
/// on all fields in the struct. For `Option<T>` fields, it merges by taking
/// the `other` value when `self` is `None`. For other types, it recursively
/// calls `merge_from` on the field.
///
/// # Example
///
/// ```ignore
/// #[derive(MergeFrom)]
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
                    let field_type = &field.ty;

                    if is_option_type(field_type) {
                        // For Option<T> fields, merge by taking the other value if self is None
                        quote! {
                            if let Some(other_value) = other.#field_name.as_ref() {
                                if self.#field_name.is_none() {
                                    self.#field_name = Some(other_value.clone());
                                } else if let Some(self_value) = self.#field_name.as_mut() {
                                    self_value.merge_from(Some(other_value));
                                }
                            }
                        }
                    } else {
                        // For non-Option fields, recursively call merge_from
                        quote! {
                            self.#field_name.merge_from(Some(&other.#field_name));
                        }
                    }
                });

                quote! {
                    if let Some(other) = other {
                        #(#field_merges)*
                    }
                }
            }
            Fields::Unnamed(_) => {
                panic!("MergeFrom can only be derived for structs with named fields");
            }
            Fields::Unit => {
                quote! {
                    // No fields to merge for unit structs
                }
            }
        },
        Data::Enum(_) => {
            quote! {
               if let Some(other) = other {
                   *self = other.clone();
               }
            }
        }
        Data::Union(_) => {
            panic!("MergeFrom cannot be derived for unions");
        }
    };

    let expanded = quote! {
        impl #impl_generics crate::merge_from::MergeFrom for #name #ty_generics #where_clause {
            fn merge_from(&mut self, other: ::core::option::Option<&Self>) {
                #merge_body
            }
        }
    };

    TokenStream::from(expanded)
}

/// Check if a type is `Option<T>`
fn is_option_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident == "Option"
            } else {
                false
            }
        }
        _ => false,
    }
}
