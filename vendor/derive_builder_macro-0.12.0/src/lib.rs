//! Derive a builder for a struct

#![crate_type = "proc-macro"]
#![deny(warnings)]

extern crate proc_macro;
#[macro_use]
extern crate syn;
extern crate derive_builder_core;

use proc_macro::TokenStream;

/// Create a builder struct for the deriving struct.
///
/// See the `derive_builder` crate documentation for more details.
#[proc_macro_derive(
    Builder,
    attributes(
        builder,
        builder_field_attr,
        builder_impl_attr,
        builder_setter_attr,
        builder_struct_attr
    )
)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    derive_builder_core::builder_for_struct(ast).into()
}
