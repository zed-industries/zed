//! This crate defines macros to easily define and use functions with a
//! versioned suffix, to facilitate using multiple versions of the same
//! crate that generate assembly.
//!
//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.

use quote::ToTokens;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn version(value: impl std::fmt::Display) -> String {
    format!("{}_{}", value, VERSION.replace('.', "_"))
}

fn versioned_lit_str(value: impl std::fmt::Display) -> syn::LitStr {
    syn::LitStr::new(version(value).as_str(), proc_macro2::Span::call_site())
}

#[proc_macro_attribute]
pub fn versioned_export(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = syn::parse_macro_input!(item as syn::ItemFn);

    let export_name = versioned_lit_str(&function.sig.ident);
    function
        .attrs
        .push(syn::parse_quote! { #[unsafe(export_name = #export_name)] });

    function.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn versioned_link(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = syn::parse_macro_input!(item as syn::ForeignItemFn);

    let link_name = versioned_lit_str(&function.sig.ident);
    function
        .attrs
        .push(syn::parse_quote! { #[link_name = #link_name] });

    function.to_token_stream().into()
}

#[proc_macro]
pub fn versioned_stringify_ident(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ident = syn::parse_macro_input!(item as syn::Ident);

    versioned_lit_str(ident).to_token_stream().into()
}

#[proc_macro]
pub fn versioned_suffix(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if !item.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "`versioned_suffix!` accepts no input",
        )
        .to_compile_error()
        .into();
    };

    versioned_lit_str("").to_token_stream().into()
}
