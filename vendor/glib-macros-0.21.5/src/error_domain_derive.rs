// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;

use crate::utils::{crate_ident_new, gen_enum_from_glib, parse_nested_meta_items, NestedMetaItem};

pub fn impl_error_domain(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let enum_variants = match input.data {
        Data::Enum(ref e) => &e.variants,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "#[derive(glib::ErrorDomain)] only supports enums",
            ))
        }
    };

    let mut domain_name = NestedMetaItem::<syn::LitStr>::new("name")
        .required()
        .value_required();
    let found = parse_nested_meta_items(&input.attrs, "error_domain", &mut [&mut domain_name])?;

    if found.is_none() {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(glib::ErrorDomain)] requires #[error_domain(name = \"domain-name\")]",
        ));
    };
    let domain_name = domain_name.value.unwrap();
    let crate_ident = crate_ident_new();

    let from_glib = gen_enum_from_glib(name, enum_variants);

    Ok(quote! {
        impl #crate_ident::error::ErrorDomain for #name {
            #[inline]
            fn domain() -> #crate_ident::Quark {
                use #crate_ident::translate::from_glib;

                static QUARK: ::std::sync::OnceLock<#crate_ident::Quark> = ::std::sync::OnceLock::new();
                *QUARK.get_or_init(|| unsafe {
                    from_glib(#crate_ident::ffi::g_quark_from_static_string(concat!(#domain_name, "\0") as *const ::core::primitive::str as *const _))
                })
            }

            #[inline]
            fn code(self) -> i32 {
                self as i32
            }

            #[inline]
            fn from(value: i32) -> ::core::option::Option<Self>
            where
                Self: ::std::marker::Sized
            {
                #from_glib
            }
        }
    })
}
