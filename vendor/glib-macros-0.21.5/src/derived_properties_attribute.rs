// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Span, TokenStream};
use quote::quote;

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectImpl` trait";

pub fn impl_derived_properties(input: &syn::ItemImpl) -> syn::Result<TokenStream> {
    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = input;

    let trait_path = &trait_
        .as_ref()
        .ok_or_else(|| syn::Error::new(Span::call_site(), WRONG_PLACE_MSG))?
        .1;

    let mut has_property = false;
    let mut has_properties = false;
    let mut has_set_property = false;

    for item in items {
        if let syn::ImplItem::Fn(method) = item {
            let ident = &method.sig.ident;

            if ident == "properties" {
                has_properties = true;
            } else if ident == "set_property" {
                has_set_property = true;
            } else if ident == "property" {
                has_property = true;
            }
        }
    }

    let crate_ident = crate::utils::crate_ident_new();

    let properties = quote!(
        fn properties() -> &'static [#crate_ident::ParamSpec] {
            Self::derived_properties()
        }
    );

    let set_property = quote!(
        fn set_property(&self, id: usize, value: &#crate_ident::Value, pspec: &#crate_ident::ParamSpec) {
            Self::derived_set_property(self, id, value, pspec)
        }
    );

    let property = quote!(
        fn property(&self, id: usize, pspec: &#crate_ident::ParamSpec) -> #crate_ident::Value {
            Self::derived_property(self, id, pspec)
        }
    );

    let generated = [
        (!has_properties).then_some(properties),
        (!has_set_property).then_some(set_property),
        (!has_property).then_some(property),
    ];

    Ok(quote!(
        #(#attrs)*
        impl #generics #trait_path for #self_ty {
            #(#items)*
            #(#generated)*
        }
    ))
}
