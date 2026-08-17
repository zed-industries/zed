use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Attribute, Data, DataEnum, DeriveInput, Error, Fields, Generics, Ident, Lifetime,
    LifetimeParam, Variant, spanned::Spanned,
};
use zvariant_utils::macros;

use crate::utils::*;

pub enum ValueType {
    Value,
    OwnedValue,
}

pub fn expand_derive(ast: DeriveInput, value_type: ValueType) -> Result<TokenStream, Error> {
    let StructAttributes {
        signature,
        rename_all,
        crate_path: crate_attr,
        ..
    } = StructAttributes::parse(&ast.attrs)?;
    let crate_path = parse_crate_path(crate_attr.as_deref())?;
    let zv = zvariant_path(crate_path.as_ref());

    let signature = signature.map(|signature| match signature.as_str() {
        "dict" => "a{sv}".to_string(),
        _ => signature,
    });

    match &ast.data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(_) | Fields::Unnamed(_) => impl_struct(
                value_type,
                ast.ident,
                ast.generics,
                &ds.fields,
                signature,
                &zv,
                rename_all,
            ),
            Fields::Unit => Err(Error::new(ast.span(), "Unit structures not supported")),
        },
        Data::Enum(data) => impl_enum(value_type, ast.ident, ast.generics, ast.attrs, data, &zv),
        _ => Err(Error::new(
            ast.span(),
            "only structs and enums are supported",
        )),
    }
}

fn impl_struct(
    value_type: ValueType,
    name: Ident,
    generics: Generics,
    fields: &Fields,
    signature: Option<String>,
    zv: &TokenStream,
    rename_all: Option<String>,
) -> Result<TokenStream, Error> {
    let statc_lifetime = LifetimeParam::new(Lifetime::new("'static", Span::call_site()));
    let (
        value_type,
        value_lifetime,
        into_value_trait,
        into_value_method,
        into_value_error_decl,
        into_value_ret,
        into_value_error_transform,
    ) = match value_type {
        ValueType::Value => {
            let mut lifetimes = generics.lifetimes();
            let value_lifetime = lifetimes
                .next()
                .cloned()
                .unwrap_or_else(|| statc_lifetime.clone());
            if lifetimes.next().is_some() {
                return Err(Error::new(
                    name.span(),
                    "Type with more than 1 lifetime not supported",
                ));
            }

            (
                quote! { #zv::Value<#value_lifetime> },
                value_lifetime,
                quote! { From },
                quote! { from },
                quote! {},
                quote! { Self },
                quote! {},
            )
        }
        ValueType::OwnedValue => (
            quote! { #zv::OwnedValue },
            statc_lifetime,
            quote! { TryFrom },
            quote! { try_from },
            quote! { type Error = #zv::Error; },
            quote! { #zv::Result<Self> },
            quote! { .map_err(::std::convert::Into::into) },
        ),
    };

    let type_params = generics.type_params().cloned().collect::<Vec<_>>();
    let (from_value_where_clause, into_value_where_clause) = if !type_params.is_empty() {
        (
            Some(quote! {
                where
                #(
                    #type_params: ::std::convert::TryFrom<#zv::Value<#value_lifetime>> + #zv::Type,
                    <#type_params as ::std::convert::TryFrom<#zv::Value<#value_lifetime>>>::Error: ::std::convert::Into<#zv::Error>
                ),*
            }),
            Some(quote! {
                where
                #(
                    #type_params: ::std::convert::Into<#zv::Value<#value_lifetime>> + #zv::Type
                ),*
            }),
        )
    } else {
        (None, None)
    };
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    match fields {
        Fields::Named(_) => {
            let field_names: Vec<_> = fields
                .iter()
                .map(|field| field.ident.to_token_stream())
                .collect();
            let (from_value_impl, into_value_impl) = match signature {
                Some(signature) if signature == "a{sv}" => {
                    // User wants the type to be encoded as a dict.
                    // FIXME: Not the most efficient implementation.
                    let (fields_init, entries_init): (TokenStream, TokenStream) = fields
                        .iter()
                        .map(|field| {
                            let FieldAttributes { rename, .. } =
                                FieldAttributes::parse(&field.attrs).unwrap_or_default();
                            let field_name = field.ident.to_token_stream();
                            let key_name = rename_identifier(
                                field.ident.as_ref().unwrap().to_string(),
                                field.span(),
                                rename,
                                rename_all.as_deref(),
                            )
                            .unwrap_or(field_name.to_string());
                            let convert = if macros::ty_is_option(&field.ty) {
                                quote! {
                                    .map(#zv::Value::downcast)
                                    .transpose()?
                                }
                            } else {
                                quote! {
                                    .ok_or_else(|| #zv::Error::IncorrectType)?
                                    .downcast()?
                                }
                            };

                            let fields_init = quote! {
                                #field_name: fields
                                    .remove(#key_name)
                                    #convert,
                            };
                            let entries_init = if macros::ty_is_option(&field.ty) {
                                quote! {
                                    if let Some(v) = s.#field_name {
                                        fields.insert(
                                            #key_name,
                                            #zv::Value::from(v),
                                        );
                                    }
                                }
                            } else {
                                quote! {
                                    fields.insert(
                                        #key_name,
                                        #zv::Value::from(s.#field_name),
                                    );
                                }
                            };

                            (fields_init, entries_init)
                        })
                        .unzip();

                    (
                        quote! {
                            let mut fields = <::std::collections::HashMap::<
                                ::std::string::String,
                                #zv::Value,
                            >>::try_from(value)?;

                            ::std::result::Result::Ok(Self { #fields_init })
                        },
                        quote! {
                            let mut fields = ::std::collections::HashMap::new();
                            #entries_init

                            <#value_type>::#into_value_method(#zv::Value::from(fields))
                                #into_value_error_transform
                        },
                    )
                }
                Some(_) | None => (
                    quote! {
                        let mut fields = #zv::Structure::try_from(value)?.into_fields();

                        ::std::result::Result::Ok(Self {
                            #(
                                #field_names: fields.remove(0).downcast()?
                            ),*
                        })
                    },
                    quote! {
                        <#value_type>::#into_value_method(#zv::StructureBuilder::new()
                        #(
                            .add_field(s.#field_names)
                        )*
                        .build().unwrap())
                        #into_value_error_transform
                    },
                ),
            };
            Ok(quote! {
                impl #impl_generics ::std::convert::TryFrom<#value_type> for #name #ty_generics
                    #from_value_where_clause
                {
                    type Error = #zv::Error;

                    #[inline]
                    fn try_from(value: #value_type) -> #zv::Result<Self> {
                        #from_value_impl
                    }
                }

                impl #impl_generics #into_value_trait<#name #ty_generics> for #value_type
                    #into_value_where_clause
                {
                    #into_value_error_decl

                    #[inline]
                    fn #into_value_method(s: #name #ty_generics) -> #into_value_ret {
                        #into_value_impl
                    }
                }
            })
        }
        Fields::Unnamed(_) if fields.iter().next().is_some() => {
            // Newtype struct.
            Ok(quote! {
                impl #impl_generics ::std::convert::TryFrom<#value_type> for #name #ty_generics
                    #from_value_where_clause
                {
                    type Error = #zv::Error;

                    #[inline]
                    fn try_from(value: #value_type) -> #zv::Result<Self> {
                        ::std::convert::TryInto::try_into(value).map(Self)
                    }
                }

                impl #impl_generics #into_value_trait<#name #ty_generics> for #value_type
                    #into_value_where_clause
                {
                    #into_value_error_decl

                    #[inline]
                    fn #into_value_method(s: #name #ty_generics) -> #into_value_ret {
                        <#value_type>::#into_value_method(s.0) #into_value_error_transform
                    }
                }
            })
        }
        Fields::Unnamed(_) => panic!("impl_struct must not be called for tuples"),
        Fields::Unit => panic!("impl_struct must not be called for unit structures"),
    }
}

fn impl_enum(
    value_type: ValueType,
    name: Ident,
    _generics: Generics,
    attrs: Vec<Attribute>,
    data: &DataEnum,
    zv: &TokenStream,
) -> Result<TokenStream, Error> {
    let repr: TokenStream = match attrs.iter().find(|attr| attr.path().is_ident("repr")) {
        Some(repr_attr) => repr_attr.parse_args()?,
        None => quote! { u32 },
    };
    let enum_attrs = EnumAttributes::parse(&attrs)?;
    let str_enum = enum_attrs
        .signature
        .map(|sig| sig == "s")
        .unwrap_or_default();

    let mut variant_names = vec![];
    let mut str_values = vec![];
    for variant in &data.variants {
        let variant_attrs = VariantAttributes::parse(&variant.attrs)?;
        // Ensure all variants of the enum are unit type
        match variant.fields {
            Fields::Unit => {
                variant_names.push(&variant.ident);
                if str_enum {
                    let str_value = enum_name_for_variant(
                        variant,
                        variant_attrs.rename,
                        enum_attrs.rename_all.as_ref().map(AsRef::as_ref),
                    )?;
                    str_values.push(str_value);
                }
            }
            _ => return Err(Error::new(variant.span(), "must be a unit variant")),
        }
    }

    let into_val = if str_enum {
        quote! {
            match e {
                #(
                    #name::#variant_names => #str_values,
                )*
            }
        }
    } else {
        quote! { e as #repr }
    };

    let (value_type, into_value) = match value_type {
        ValueType::Value => (
            quote! { #zv::Value<'_> },
            quote! {
                impl ::std::convert::From<#name> for #zv::Value<'_> {
                    #[inline]
                    fn from(e: #name) -> Self {
                        <#zv::Value as ::std::convert::From<_>>::from(#into_val)
                    }
                }
            },
        ),
        ValueType::OwnedValue => (
            quote! { #zv::OwnedValue },
            quote! {
                impl ::std::convert::TryFrom<#name> for #zv::OwnedValue {
                    type Error = #zv::Error;

                    #[inline]
                    fn try_from(e: #name) -> #zv::Result<Self> {
                        <#zv::OwnedValue as ::std::convert::TryFrom<_>>::try_from(
                            <#zv::Value as ::std::convert::From<_>>::from(#into_val)
                        )
                    }
                }
            },
        ),
    };

    let from_val = if str_enum {
        quote! {
            let v: #zv::Str = ::std::convert::TryInto::try_into(value)?;

            ::std::result::Result::Ok(match v.as_str() {
                #(
                    #str_values => #name::#variant_names,
                )*
                _ => return ::std::result::Result::Err(#zv::Error::IncorrectType),
            })
        }
    } else {
        quote! {
            let v: #repr = ::std::convert::TryInto::try_into(value)?;

            ::std::result::Result::Ok(match v {
                #(
                    x if x == #name::#variant_names as #repr => #name::#variant_names
                 ),*,
                _ => return ::std::result::Result::Err(#zv::Error::IncorrectType),
            })
        }
    };

    Ok(quote! {
        impl ::std::convert::TryFrom<#value_type> for #name {
            type Error = #zv::Error;

            #[inline]
            fn try_from(value: #value_type) -> #zv::Result<Self> {
                #from_val
            }
        }

        #into_value
    })
}

fn enum_name_for_variant(
    v: &Variant,
    rename_attr: Option<String>,
    rename_all_attr: Option<&str>,
) -> Result<String, Error> {
    let ident = v.ident.to_string();

    rename_identifier(ident, v.span(), rename_attr, rename_all_attr)
}
