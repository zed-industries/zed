use crate::utils::{
    add_extra_generic_param, numbered_vars, replace_self::DeriveInputExt as _,
    AttrParams, DeriveType, MultiFieldData, State,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::utils::{
    attr::{self, ParseMultiple as _},
    HashMap, Spanning,
};

/// Provides the hook to expand `#[derive(TryInto)]` into an implementation of `TryInto`.
#[allow(clippy::cognitive_complexity)]
pub fn expand(
    input: &syn::DeriveInput,
    trait_name: &'static str,
) -> syn::Result<TokenStream> {
    let input = &mut input.replace_self_type();
    let trait_attr = "try_into";

    // TODO: Use `Vec::extract_if` once MSRV is bumped to 1.87 or above.
    let (error_indices, error_attrs) = input
        .attrs
        .iter()
        .enumerate()
        .filter(|(_, attr)| {
            attr.path().is_ident(trait_attr)
                && attr.parse_args_with(detect_error_attr).is_ok()
        })
        .map(|(i, attr)| (i, attr.clone()))
        .unzip::<_, _, Vec<_>, Vec<_>>();
    for i in error_indices {
        _ = &mut input.attrs.remove(i);
    }
    let custom_error =
        attr::Error::parse_attrs(error_attrs, &format_ident!("{trait_attr}"))?
            .map(Spanning::into_inner);

    let state = State::with_attr_params(
        input,
        trait_name,
        trait_attr.into(),
        AttrParams {
            enum_: vec!["ignore", "owned", "ref", "ref_mut"],
            variant: vec!["ignore", "owned", "ref", "ref_mut"],
            struct_: vec!["ignore", "owned", "ref", "ref_mut"],
            field: vec!["ignore"],
        },
    ).map_err(|e| {
        // Temporary adjustment of attribute parsing errors until these attributes are reimplemented
        // via `utils::attr` machinery.
        let msg = &e.to_string();
        if msg == "Only a single attribute is allowed" {
            syn::Error::new(
                e.span(),
                "Only a single attribute is allowed.\n\
                 For the top-level attribute, an additional `#[try_into(error(...))]` may be \
                 provided.",
            )
        } else if msg.starts_with(
            "Attribute parameter not supported. Supported attribute parameters are:",
        ) {
            syn::Error::new(e.span(), format!("{msg}, error"))
        } else {
            e
        }
    })?;
    assert!(
        state.derive_type == DeriveType::Enum,
        "only enums can derive `TryInto`",
    );

    let mut variants_per_types = HashMap::default();

    for variant_state in state.enabled_variant_data().variant_states {
        let multi_field_data = variant_state.enabled_fields_data();
        let MultiFieldData {
            variant_info,
            field_types,
            ..
        } = multi_field_data.clone();
        for ref_type in variant_info.ref_types() {
            variants_per_types
                .entry((ref_type, field_types.clone()))
                .or_insert_with(Vec::new)
                .push(multi_field_data.clone());
        }
    }

    let mut tokens = TokenStream::new();

    for ((ref_type, ref original_types), ref multi_field_data) in variants_per_types {
        let input_type = &input.ident;

        let pattern_ref = ref_type.pattern_ref();
        let lifetime = ref_type.lifetime();
        let reference_with_lifetime = ref_type.reference_with_lifetime();

        let mut matchers = vec![];
        let vars = &numbered_vars(original_types.len(), "");
        for multi_field_data in multi_field_data {
            let patterns: Vec<_> = vars
                .iter()
                .map(|var| quote! { #pattern_ref #var })
                .collect();
            matchers.push(
                multi_field_data.matcher(&multi_field_data.field_indexes, &patterns),
            );
        }

        let vars = if vars.len() == 1 {
            quote! { #(#vars)* }
        } else {
            quote! { (#(#vars),*) }
        };

        let output_type = if original_types.len() == 1 {
            quote! { #(#original_types)* }.to_string()
        } else {
            let types = original_types
                .iter()
                .map(|t| quote! { #t }.to_string())
                .collect::<Vec<_>>();
            format!("({})", types.join(", "))
        };
        let variant_names = multi_field_data
            .iter()
            .map(|d| {
                d.variant_name
                    .expect("somehow there was no variant name")
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join(", ");

        let generics_impl;
        let (_, ty_generics, where_clause) = input.generics.split_for_impl();
        let (impl_generics, _, _) = if ref_type.is_ref() {
            generics_impl = add_extra_generic_param(&input.generics, lifetime.clone());
            generics_impl.split_for_impl()
        } else {
            input.generics.split_for_impl()
        };

        let mut error_ty = quote! {
            derive_more::TryIntoError<#reference_with_lifetime #input_type #ty_generics>
        };
        let mut error_conv = quote! {};
        if let Some(custom_error) = custom_error.as_ref() {
            error_ty = custom_error.ty.to_token_stream();
            error_conv = custom_error.conv.as_ref().map_or_else(
                || quote! { .map_err(derive_more::core::convert::Into::into) },
                |conv| quote! { .map_err(#conv) },
            );
        }

        let try_from = quote! {
            #[automatically_derived]
            impl #impl_generics derive_more::core::convert::TryFrom<
                #reference_with_lifetime #input_type #ty_generics
            > for (#(#reference_with_lifetime #original_types),*) #where_clause {
                type Error = #error_ty;

                #[inline]
                fn try_from(
                    value: #reference_with_lifetime #input_type #ty_generics,
                ) -> derive_more::core::result::Result<Self, #error_ty> {
                    match value {
                        #(#matchers)|* => derive_more::core::result::Result::Ok(#vars),
                        _ => derive_more::core::result::Result::Err(
                            derive_more::TryIntoError::new(value, #variant_names, #output_type),
                        )#error_conv,
                    }
                }
            }
        };
        try_from.to_tokens(&mut tokens)
    }
    Ok(tokens)
}

/// Checks whether the inner of a [`syn::Attribute`] represents an [`attr::Error`].
fn detect_error_attr(input: syn::parse::ParseStream) -> syn::Result<()> {
    mod ident {
        syn::custom_keyword!(error);
    }

    let ahead = input.lookahead1();
    if ahead.peek(ident::error) {
        let _ = input.parse::<TokenStream>();
        Ok(())
    } else {
        Err(ahead.error())
    }
}
