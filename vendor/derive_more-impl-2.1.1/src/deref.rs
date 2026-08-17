use crate::utils::{
    add_extra_where_clauses, numbered_vars, panic_one_field, SingleFieldData, State,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Result, Data, DeriveInput};

/// Provides the hook to expand `#[derive(Deref)]` into an implementation of `Deref`
pub fn expand(input: &DeriveInput, trait_name: &'static str) -> Result<TokenStream> {
    match input.data {
        Data::Struct(_) => expand_struct(input, trait_name),
        Data::Enum(_) => expand_enum(input, trait_name),
        _ => panic!("only structs and enums can use `derive({trait_name})`"),
    }
}

fn expand_enum(input: &DeriveInput, trait_name: &'static str) -> Result<TokenStream> {
    let state = State::with_field_ignore_and_forward(
        input,
        trait_name,
        trait_name.to_lowercase(),
    )?;

    let trait_path = &state.trait_path;
    let enum_name = &input.ident;

    let mut target = None;
    let mut match_arms = vec![];
    let mut predicates = vec![];

    for variant_state in state.enabled_variant_data().variant_states.into_iter() {
        let data = variant_state.enabled_fields_data();
        if data.fields.len() != 1 {
            panic_one_field(variant_state.trait_name, &variant_state.trait_attr);
        };

        let vars = numbered_vars(variant_state.fields.len(), "");
        let matcher = data.matcher(&data.field_indexes, &vars);

        let info = data.infos[0].clone();
        let field_type = data.field_types[0];

        let (target_, var) = if info.forward {
            let casted_trait = data.casted_traits[0].clone();
            predicates.push(quote! { #field_type: #trait_path });
            (
                quote! { #casted_trait::Target },
                quote! { #casted_trait::deref(__0) },
            )
        } else {
            (quote! { #field_type }, quote! { __0 })
        };

        if target.is_none() {
            target = Some(target_);
        }

        match_arms.push(quote! {
            #matcher => #var
        });
    }

    let target = target.unwrap();

    let generics = if predicates.is_empty() {
        &input.generics
    } else {
        &add_extra_where_clauses(&input.generics, quote! { where #(#predicates),* })
    };

    let (imp_generics, type_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #[allow(deprecated)] // omit warnings on deprecated fields/variants
        #[allow(unreachable_code)] // omit warnings for `!` and other unreachable types
        #[automatically_derived]
        impl #imp_generics #trait_path for #enum_name #type_generics #where_clause {
            type Target = #target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                match self {
                    #(#match_arms),*
                }
            }
        }
    })
}

fn expand_struct(input: &DeriveInput, trait_name: &'static str) -> Result<TokenStream> {
    let state = State::with_field_ignore_and_forward(
        input,
        trait_name,
        trait_name.to_lowercase(),
    )?;
    let SingleFieldData {
        input_type,
        field_type,
        trait_path,
        casted_trait,
        ty_generics,
        member,
        info,
        ..
    } = state.assert_single_enabled_field();

    let (target, body, generics) = if info.forward {
        (
            quote! { #casted_trait::Target },
            quote! { #casted_trait::deref(&#member) },
            add_extra_where_clauses(
                &input.generics,
                quote! {
                    where #field_type: #trait_path
                },
            ),
        )
    } else {
        (
            quote! { #field_type },
            quote! { &#member },
            input.generics.clone(),
        )
    };
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #[allow(deprecated)] // omit warnings on deprecated fields/variants
        #[allow(unreachable_code)] // omit warnings for `!` and other unreachable types
        #[automatically_derived]
        impl #impl_generics #trait_path for #input_type #ty_generics #where_clause {
            type Target = #target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                #body
            }
        }
    })
}
