use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::GenericParam;

use crate::{
    covariance_detection::apparent_std_container_type, info_structures::StructInfo,
    utils::replace_this_with_lifetime,
};

pub fn make_type_asserts(info: &StructInfo) -> TokenStream {
    let mut checks = Vec::new();
    let fake_lifetime = if let Some(GenericParam::Lifetime(param)) = info.generic_params().first() {
        param.lifetime.ident.clone()
    } else {
        format_ident!("static")
    };
    for field in &info.fields {
        let field_type = &field.typ;
        if let Some((std_type, _eltype)) = apparent_std_container_type(field_type) {
            let checker_name = match std_type {
                "Box" => "is_std_box_type",
                "Arc" => "is_std_arc_type",
                "Rc" => "is_std_rc_type",
                _ => unreachable!(),
            };
            let checker_name = format_ident!("{}", checker_name);
            let static_field_type =
                replace_this_with_lifetime(quote! { #field_type }, fake_lifetime.clone());
            checks.push(quote! {
                ::ouroboros::macro_help::CheckIfTypeIsStd::<#static_field_type>::#checker_name();
            });
        }
    }
    let generic_params = info.generic_params();
    let generic_where = &info.generics.where_clause;
    quote! {
        fn type_asserts <#generic_params>() #generic_where {
            #(#checks)*
        }
    }
}
