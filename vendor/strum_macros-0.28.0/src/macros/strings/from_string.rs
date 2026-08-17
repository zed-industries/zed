use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::helpers::{
    missing_parse_err_attr_error, non_enum_error, occurrence_error, HasInnerVariantProperties,
    HasStrumVariantProperties, HasTypeProperties,
};

pub fn from_string_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let type_properties = ast.get_type_properties()?;
    let strum_module_path = type_properties.crate_module_path();

    // It's an error to provide an err_fn but not an err_ty.
    if type_properties.parse_err_fn.is_some() && type_properties.parse_err_ty.is_none() {
        return Err(missing_parse_err_attr_error());
    }

    let mut default_kw = None;
    let mut default_match_arm = None;

    let mut phf_exact_match_arms = Vec::new();
    let mut standard_match_arms = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;

        if variant_properties.disabled.is_some() {
            continue;
        }

        if let Some(kw) = variant_properties.default {
            if let Some(fst_kw) = default_kw {
                return Err(occurrence_error(fst_kw, kw, "default"));
            }

            default_kw = Some(kw);

            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    default_match_arm = Some(quote! {
                        #name::#ident(s.into())
                    });
                }
                Fields::Named(ref f) if f.named.len() == 1 => {
                    let field_name = f.named.last().unwrap().ident.as_ref().unwrap();
                    default_match_arm = Some(quote! { #name::#ident { #field_name : s.into() } });
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "Default only works on newtype structs with a single String field",
                    ))
                }
            }

            continue;
        }

        let params = match &variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(fields) => {
                if let Some(ref value) = variant_properties.default_with {
                    let func = proc_macro2::Ident::new(&value.value(), value.span());
                    let defaults = vec![quote! { #func() }];
                    quote! { (#(#defaults),*) }
                } else {
                    let defaults =
                        ::core::iter::repeat(quote!(Default::default())).take(fields.unnamed.len());
                    quote! { (#(#defaults),*) }
                }
            }
            Fields::Named(fields) => {
                let mut defaults = vec![];
                for field in &fields.named {
                    let meta = field.get_variant_inner_properties()?;
                    let field = field.ident.as_ref().unwrap();

                    if let Some(default_with) = meta.default_with {
                        let func =
                            proc_macro2::Ident::new(&default_with.value(), default_with.span());
                        defaults.push(quote! {
                            #field: #func()
                        });
                    } else {
                        defaults.push(quote! { #field: Default::default() });
                    }
                }

                quote! { {#(#defaults),*} }
            }
        };

        let is_ascii_case_insensitive = variant_properties
            .ascii_case_insensitive
            .unwrap_or(type_properties.ascii_case_insensitive);

        // If we don't have any custom variants, add the default serialized name.
        for serialization in variant_properties.get_serializations(type_properties.case_style) {
            if type_properties.use_phf {
                phf_exact_match_arms.push(quote! { #serialization => #name::#ident #params, });

                if is_ascii_case_insensitive {
                    // Store the lowercase and UPPERCASE variants in the phf map to capture
                    let ser_string = serialization.value();

                    let lower =
                        syn::LitStr::new(&ser_string.to_ascii_lowercase(), serialization.span());
                    let upper =
                        syn::LitStr::new(&ser_string.to_ascii_uppercase(), serialization.span());
                    phf_exact_match_arms.push(quote! { #lower => #name::#ident #params, });
                    phf_exact_match_arms.push(quote! { #upper => #name::#ident #params, });
                    standard_match_arms.push(quote! { s if s.eq_ignore_ascii_case(#serialization) => #name::#ident #params, });
                }
            } else if !is_ascii_case_insensitive {
                standard_match_arms.push(quote! { #serialization => #name::#ident #params, });
            } else {
                standard_match_arms.push(quote! { s if s.eq_ignore_ascii_case(#serialization) => #name::#ident #params, });
            }
        }
    }

    // Determine the error type on FromStr and TryFrom based on what the user
    // has configured and whether there is a default variant.
    let is_infallible = default_match_arm.is_some();
    let has_custom_err_ty = type_properties.parse_err_ty.is_some();
    let err_ty = if let Some(ty) = type_properties.parse_err_ty {
        quote! { #ty }
    } else if is_infallible {
        quote! { ::core::convert::Infallible }
    } else {
        quote! { #strum_module_path::ParseError }
    };

    // Determine the default match arm behavior based on whether the user provided a "default"
    // or if the user provided a custom error function.
    let default_match_arm = if let Some(default_match_arm) = default_match_arm {
        default_match_arm
    } else if let Some(f) = type_properties.parse_err_fn {
        quote! { return ::core::result::Result::Err(#f(s)) }
    } else if has_custom_err_ty {
        // The user defined a custom error type, but not a custom error function. This is an error
        // if the method isn't infallible.
        return Err(missing_parse_err_attr_error());
    } else {
        quote! { return ::core::result::Result::Err(#strum_module_path::ParseError::VariantNotFound) }
    };

    let mut match_expression = if standard_match_arms.is_empty() {
        default_match_arm
    } else {
        quote! {
            match s {
                #(#standard_match_arms)*
                _ => #default_match_arm,
            }
        }
    };

    if !phf_exact_match_arms.is_empty() {
        match_expression = quote! {
            use #strum_module_path::_private_phf_reexport_for_macro_if_phf_feature as phf;
            static PHF: phf::Map<&'static str, #name> = phf::phf_map! {
                #(#phf_exact_match_arms)*
            };

            if let Some(value) = PHF.get(s).cloned() {
                value
            } else {
                #match_expression
            }
        }
    }

    let from_impl = if is_infallible && !has_custom_err_ty {
        quote! {
            #[allow(clippy::use_self)]
            #[automatically_derived]
            impl #impl_generics ::core::convert::From<&str> for #name #ty_generics #where_clause {
                #[inline]
                fn from(s: &str) -> #name #ty_generics {
                    #match_expression
                }
            }
        }
    } else {
        quote! {
            #[allow(clippy::use_self)]
            #[automatically_derived]
            impl #impl_generics ::core::convert::TryFrom<&str> for #name #ty_generics #where_clause {
                type Error = #err_ty;

                #[inline]
                fn try_from(s: &str) -> ::core::result::Result< #name #ty_generics , <Self as ::core::convert::TryFrom<&str>>::Error> {
                    Ok({
                        #match_expression
                    })
                }
            }
        }
    };

    let from_str = quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::core::str::FromStr for #name #ty_generics #where_clause {
            type Err = #err_ty;

            #[inline]
            fn from_str(s: &str) -> ::core::result::Result< #name #ty_generics , <Self as ::core::str::FromStr>::Err> {
                <Self as ::core::convert::TryFrom<&str>>::try_from(s)
            }
        }
    };

    Ok(quote! {
        #from_str
        #from_impl
    })
}
