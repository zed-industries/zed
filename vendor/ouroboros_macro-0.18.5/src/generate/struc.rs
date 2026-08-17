use crate::{
    info_structures::StructInfo,
    utils::{self, replace_this_with_lifetime},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

/// Creates the struct that will actually store the data.
pub fn create_actual_struct_def(info: &StructInfo) -> Result<TokenStream, Error> {
    let visibility = utils::submodule_contents_visibility(&info.vis);
    let mut fields = Vec::new();
    for (ty, ident) in info.generic_consumers() {
        fields.push(quote! { #ident: ::core::marker::PhantomData<#ty> });
    }
    let generic_params = info.generic_params();
    let generic_args = info.generic_arguments();
    let generic_where = &info.generics.where_clause;
    let ident = &info.ident;
    let internal_ident = &info.internal_ident;
    Ok(quote! {
        #[repr(transparent)]
        #visibility struct #ident <#generic_params> #generic_where {
            actual_data: ::core::mem::MaybeUninit<#internal_ident<#(#generic_args),*>>,
        }
    })
}

/// Creates a struct with fields like the original struct. Instances of the
/// "actual" struct are reinterpreted as instances of the "internal" struct
/// whenever data needs to be accessed. (This gets around the problem that
/// references passed to functions must be valid through the entire function,
/// but references *created* inside a function can be considered invalid
/// whenever, even during the duration of the function.)
pub fn create_internal_struct_def(info: &StructInfo) -> Result<TokenStream, Error> {
    let ident = &info.internal_ident;
    let generics = &info.generics;

    let field_defs: Vec<_> = info
        .fields
        .iter()
        // Reverse the order of all fields. We ensure that items in the struct are only dependent
        // on references to items above them. Rust drops items in a struct in forward declaration order.
        // This would cause parents being dropped before children, necessitating the reversal.
        .rev()
        .map(|field| {
            let name = &field.name;
            let ty = field.stored_type();
            quote! {
                #[doc(hidden)]
                #name: #ty
            }
        })
        .collect();

    // Create the new struct definition.
    let mut where_clause = quote! {};
    if let Some(clause) = &generics.where_clause {
        where_clause = quote! { #clause };
    }
    let def = quote! {
        struct #ident #generics #where_clause {
            #(#field_defs),*
        }
    };

    // Finally, replace the fake 'this lifetime with the one we found.
    let fake_lifetime = info.fake_lifetime();
    let def = replace_this_with_lifetime(quote! { #def }, fake_lifetime.clone());

    Ok(def)
}
