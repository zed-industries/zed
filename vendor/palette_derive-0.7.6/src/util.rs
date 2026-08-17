use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Ident, Type};

pub fn path<'a, P: AsRef<[&'a str]>>(path: P, internal: bool) -> TokenStream {
    let path = path
        .as_ref()
        .iter()
        .map(|&ident| Ident::new(ident, Span::call_site()));

    if internal {
        quote! {crate::#(#path)::*}
    } else {
        let crate_name = find_crate_name();
        quote! {#crate_name::#(#path)::*}
    }
}

pub fn path_type(path: &[&str], internal: bool) -> Type {
    let path = path
        .iter()
        .map(|&ident| Ident::new(ident, Span::call_site()));

    if internal {
        parse_quote! {crate::#(#path)::*}
    } else {
        let crate_name = find_crate_name();
        parse_quote! {#crate_name::#(#path)::*}
    }
}

#[cfg(feature = "find-crate")]
fn find_crate_name() -> Ident {
    use find_crate::Error;

    match find_crate::find_crate(|name| name == "palette") {
        Ok(package) => Ident::new(&package.name, Span::call_site()),
        Err(Error::NotFound) => Ident::new("palette", Span::call_site()),
        Err(error) => panic!(
            "error when trying to find the name of the `palette` crate: {}",
            error
        ),
    }
}

#[cfg(not(feature = "find-crate"))]
fn find_crate_name() -> Ident {
    Ident::new("palette", Span::call_site())
}
