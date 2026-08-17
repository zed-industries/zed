use proc_macro2::TokenStream;

use quote::quote;
use syn::parse::Error;
use syn::spanned::Spanned;
use syn::DeriveInput;

use crate::default_attr::{ConversionStrategy, DefaultAttr};
use crate::util::find_only;

pub fn impl_my_derive(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (default_expr, doc) = match input.data {
        syn::Data::Struct(ref body) => {
            let (body_assignment, doc) = default_body_tt(&body.fields)?;
            (
                quote! {
                    #name #body_assignment
                },
                format!("Return `{}{}`", name, doc),
            )
        }
        syn::Data::Enum(ref body) => {
            let default_variant = find_only(body.variants.iter(), |variant| {
                if let Some(meta) = DefaultAttr::find_in_attributes(&variant.attrs)? {
                    if meta.code.is_none() {
                        Ok(true)
                    } else {
                        Err(Error::new(
                            meta.code.span(),
                            "Attribute #[default] on variants should have no value",
                        ))
                    }
                } else {
                    Ok(false)
                }
            })?
            .ok_or_else(|| Error::new(input.span(), "No default variant"))?;
            let default_variant_name = &default_variant.ident;
            let (body_assignment, doc) = default_body_tt(&default_variant.fields)?;
            (
                quote! {
                    #name :: #default_variant_name #body_assignment
                },
                format!("Return `{}::{}{}`", name, default_variant_name, doc),
            )
        }
        syn::Data::Union(_) => {
            panic!()
        }
    };
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics Default for #name #ty_generics #where_clause {
            #[doc = #doc]
            fn default() -> Self {
                #default_expr
            }
        }
    })
}

/// Return a token-tree for the default "body" - the part after the name that contains the values.
/// That is, the `{ ... }` part for structs, the `(...)` part for tuples, and nothing for units.
fn default_body_tt(body: &syn::Fields) -> Result<(TokenStream, String), Error> {
    let mut doc = String::new();
    use std::fmt::Write;
    let body_tt = match body {
        syn::Fields::Named(ref fields) => {
            doc.push_str(" {");
            let result = {
                let field_assignments = fields
                    .named
                    .iter()
                    .map(|field| {
                        let field_name = field.ident.as_ref();
                        let (default_value, default_doc) = field_default_expr_and_doc(field)?;
                        write!(
                            &mut doc,
                            "\n    {}: {},",
                            field_name.expect("field value in struct is empty"),
                            default_doc
                        )
                        .unwrap();
                        // let default_value = default_value.into_token_stream();
                        Ok(quote! { #field_name : #default_value })
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                quote! {
                    {
                        #( #field_assignments ),*
                    }
                }
            };
            if doc.ends_with(',') {
                doc.pop();
                doc.push('\n');
            };
            doc.push('}');
            result
        }
        syn::Fields::Unnamed(ref fields) => {
            doc.push('(');
            let result = {
                let field_assignments = fields
                    .unnamed
                    .iter()
                    .map(|field| {
                        let (default_value, default_doc) = field_default_expr_and_doc(field)?;
                        write!(&mut doc, "{}, ", default_doc).unwrap();
                        Ok(default_value)
                    })
                    .collect::<Result<Vec<TokenStream>, Error>>()?;
                quote! {
                    (
                        #( #field_assignments ),*
                    )
                }
            };
            if doc.ends_with(", ") {
                doc.pop();
                doc.pop();
            };
            doc.push(')');
            result
        }
        &syn::Fields::Unit => quote! {},
    };
    Ok((body_tt, doc))
}

/// Return a default expression for a field based on it's `#[default = "..."]` attribute. Panic
/// if there is more than one, of if there is a `#[default]` attribute without value.
fn field_default_expr_and_doc(field: &syn::Field) -> Result<(TokenStream, String), Error> {
    if let Some(default_attr) = DefaultAttr::find_in_attributes(&field.attrs)? {
        let conversion_strategy = default_attr.conversion_strategy();
        let field_value = default_attr.code.ok_or_else(|| {
            Error::new(field.span(), "Expected #[default = ...] or #[default(...)]")
        })?;

        let field_value = match conversion_strategy {
            ConversionStrategy::NoConversion => field_value,
            ConversionStrategy::Into => quote!((#field_value).into()),
        };

        let field_doc = format!("{}", field_value);
        Ok((field_value, field_doc))
    } else {
        Ok((
            quote! {
                Default::default()
            },
            "Default::default()".to_owned(),
        ))
    }
}
