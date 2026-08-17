use std::str::FromStr;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Field, punctuated::Punctuated, spanned::Spanned};
use zvariant_utils::{macros, signature::Signature};

use crate::utils::*;

fn dict_name_for_field(
    f: &Field,
    rename_attr: Option<String>,
    rename_all_attr: Option<&str>,
) -> Result<String, Error> {
    let ident = f.ident.as_ref().unwrap().to_string();
    rename_identifier(ident, f.span(), rename_attr, rename_all_attr)
}

/// Whether the dict's value type is `Variant` (i.e. signature `a{?v}`).
///
/// Variant-typed values get wrapped/unwrapped via `as_value`; any other value type defers to
/// the field type's own `Serialize`/`Deserialize`.
fn dict_value_is_variant(signature: Option<&str>, span: Span) -> Result<bool, Error> {
    let Some(s) = signature else {
        return Ok(true);
    };
    if s == "dict" {
        return Ok(true);
    }
    let sig = Signature::from_str(s).map_err(|e| Error::new(span, e))?;
    match sig {
        Signature::Dict { value, .. } => match &*value {
            Signature::Variant => Ok(true),
            _ => Ok(false),
        },
        _ => Err(Error::new(
            span,
            "`*Dict` derive requires a dictionary signature (e.g. `a{sv}` or `a{sa{sv}}`)",
        )),
    }
}

/// Implements `Serialize` for structs as D-Bus dictionaries via a serde helper.
pub fn expand_serialize_derive(input: DeriveInput) -> Result<TokenStream, Error> {
    let StructAttributes {
        signature,
        rename_all,
        crate_path: crate_attr,
        ..
    } = StructAttributes::parse(&input.attrs)?;
    let value_is_variant = dict_value_is_variant(signature.as_deref(), input.span())?;
    let crate_path = parse_crate_path(crate_attr.as_deref())?;
    let rename_all_str = rename_all.as_deref().unwrap_or("snake_case");
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;
    let helper = format_ident!("__SerializeDict{}", name);
    let zv = zvariant_path(crate_path.as_ref());

    let mut field_defs = Vec::new();
    let mut field_inits = Vec::new();
    let Data::Struct(data) = &input.data else {
        return Err(Error::new(input.span(), "only structs supported"));
    };
    for field in &data.fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let FieldAttributes { rename, .. } = FieldAttributes::parse(&field.attrs)?;
        let dict_name = dict_name_for_field(field, rename, rename_all.as_deref())?;
        let is_opt = macros::ty_is_option(ty);
        let field_def = match (value_is_variant, is_opt) {
            (true, true) => {
                let path = format!("{}::as_value::optional", quote! { #zv });
                quote! {
                    #[serde(
                        rename = #dict_name,
                        with = #path,
                        skip_serializing_if = "Option::is_none",
                    )]
                    #ident: &'a #ty
                }
            }
            (true, false) => {
                let path = format!("{}::as_value", quote! { #zv });
                quote! {
                    #[serde(rename = #dict_name, with = #path)]
                    #ident: &'a #ty
                }
            }
            (false, true) => quote! {
                #[serde(
                    rename = #dict_name,
                    serialize_with = "__zv_dict_ser_opt",
                    skip_serializing_if = "Option::is_none",
                )]
                #ident: &'a #ty
            },
            (false, false) => quote! {
                #[serde(rename = #dict_name)]
                #ident: &'a #ty
            },
        };
        field_defs.push(field_def);
        field_inits.push(quote! { #ident: &self.#ident });
    }

    let opt_serializer = (!value_is_variant).then(|| {
        quote! {
            fn __zv_dict_ser_opt<T, S>(
                value: &::std::option::Option<T>,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error>
            where
                T: #zv::export::serde::Serialize,
                S: #zv::export::serde::Serializer,
            {
                <T as #zv::export::serde::Serialize>::serialize(
                    value.as_ref().unwrap(),
                    serializer,
                )
            }
        }
    });

    Ok(quote! {
        #[allow(deprecated)]
        impl #impl_generics #zv::export::serde::ser::Serialize for #name #ty_generics #where_clause {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: #zv::export::serde::ser::Serializer,
            {
                use #zv::export::serde::Serialize;

                #opt_serializer

                #[derive(Serialize)]
                #[serde(rename_all = #rename_all_str)]
                struct #helper<'a> {
                    #[serde(skip)]
                    phantom: ::std::marker::PhantomData<&'a ()>,
                    #(#field_defs,)*
                }

                let helper = #helper {
                    phantom: ::std::marker::PhantomData,
                    #(#field_inits,)*
                };

                helper.serialize(serializer)
            }
        }
    })
}

/// Implements `Deserialize` for structs from D-Bus dictionaries via a serde helper.
pub fn expand_deserialize_derive(input: DeriveInput) -> Result<TokenStream, Error> {
    let StructAttributes {
        signature,
        rename_all,
        deny_unknown_fields,
        crate_path: crate_attr,
        ..
    } = StructAttributes::parse(&input.attrs)?;
    let value_is_variant = dict_value_is_variant(signature.as_deref(), input.span())?;
    let crate_path = parse_crate_path(crate_attr.as_deref())?;
    let rename_all_str = rename_all.as_deref().unwrap_or("snake_case");
    let zv = zvariant_path(crate_path.as_ref());

    // Create a new generics with a 'de lifetime
    let mut generics = input.generics.clone();
    let lifetime_param = syn::LifetimeParam {
        attrs: Vec::new(),
        lifetime: syn::Lifetime::new("'de", Span::call_site()),
        colon_token: None,
        bounds: Punctuated::new(),
    };
    generics
        .params
        .insert(0, syn::GenericParam::Lifetime(lifetime_param));

    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();
    let (_, orig_ty_generics, _) = input.generics.split_for_impl();
    let name = &input.ident;
    let helper = format_ident!("__DeserializeDict{}", name);

    let mut field_defs = Vec::new();
    let mut field_assignments = Vec::new();
    let mut non_optional_field_checks = Vec::new();
    let Data::Struct(data) = &input.data else {
        return Err(Error::new(input.span(), "only structs supported"));
    };
    let opt_path = if value_is_variant {
        format!("{}::as_value::optional", quote! { #zv })
    } else {
        "__zv_dict_de_opt".to_string()
    };
    for field in &data.fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let FieldAttributes { rename, .. } = FieldAttributes::parse(&field.attrs)?;
        let dict_name = dict_name_for_field(field, rename, rename_all.as_deref())?;
        let is_opt = macros::ty_is_option(ty);

        let with_attr = if value_is_variant {
            quote! { with = #opt_path }
        } else {
            quote! { deserialize_with = #opt_path }
        };

        if is_opt {
            field_defs.push(quote! {
                #[serde(rename = #dict_name, #with_attr, default)]
                #ident: #ty
            });
            field_assignments.push(quote! { #ident: helper.#ident });
        } else {
            field_defs.push(quote! {
                #[serde(rename = #dict_name, #with_attr, default)]
                #ident: ::std::option::Option<#ty>
            });

            non_optional_field_checks.push(quote! {
                if helper.#ident.is_none() {
                    return ::std::result::Result::Err(
                        <D::Error as #zv::export::serde::de::Error>::missing_field(#dict_name),
                    );
                }
            });

            field_assignments.push(quote! { #ident: helper.#ident.unwrap() });
        }
    }

    let deny_attr = if deny_unknown_fields {
        quote! { , deny_unknown_fields }
    } else {
        quote! {}
    };

    let opt_deserializer = (!value_is_variant).then(|| {
        quote! {
            fn __zv_dict_de_opt<'de, T, D>(
                deserializer: D,
            ) -> ::std::result::Result<::std::option::Option<T>, D::Error>
            where
                T: #zv::export::serde::Deserialize<'de>,
                D: #zv::export::serde::Deserializer<'de>,
            {
                <T as #zv::export::serde::Deserialize<'de>>::deserialize(deserializer)
                    .map(::std::option::Option::Some)
            }
        }
    });

    Ok(quote! {
        #[allow(deprecated)]
        impl #impl_generics #zv::export::serde::de::Deserialize<'de> for #name #orig_ty_generics
        #where_clause
        {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: #zv::export::serde::de::Deserializer<'de>,
            {
                use #zv::export::serde::Deserialize;

                #opt_deserializer

                #[derive(Deserialize, Default)]
                #[serde(default, rename_all = #rename_all_str #deny_attr)]
                struct #helper {
                    #(#field_defs,)*
                }

                let helper = #helper::deserialize(deserializer)?;

                #(#non_optional_field_checks)*

                Ok(Self {
                    #(#field_assignments,)*
                })
            }
        }
    })
}
