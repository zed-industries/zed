//! Internal helper library for the `derive_builder` crate.
//!
//! **Important Note**:
//!
//! * You are probably looking for the [`derive_builder`] crate,
//!   which wraps this crate and is much more ergonomic to use.
//!
//! ## Purpose
//!
//! This is an internal helper library of [`derive_builder`], which allows for
//! all the logic of builder creation to be decoupled from the proc-macro entry
//! point.
//!
//!
//! [`derive_builder`]: https://!crates.io/crates/derive_builder
//! [`derive_builder_core`]: https://!crates.io/crates/derive_builder_core

#![deny(warnings, missing_docs)]
#![cfg_attr(test, recursion_limit = "100")]

#[macro_use]
extern crate darling;

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod block;
mod build_method;
mod builder;
mod builder_field;
mod change_span;
mod default_expression;
mod deprecation_notes;
mod doc_comment;
mod initializer;
mod macro_options;
mod options;
mod setter;

pub(crate) use block::BlockContents;
pub(crate) use build_method::BuildMethod;
pub(crate) use builder::Builder;
pub(crate) use builder_field::{BuilderField, BuilderFieldType};
pub(crate) use change_span::change_span;
use darling::FromDeriveInput;
pub(crate) use default_expression::DefaultExpression;
pub(crate) use deprecation_notes::DeprecationNotes;
pub(crate) use doc_comment::doc_comment_from;
pub(crate) use initializer::{FieldConversion, Initializer};
pub(crate) use options::{BuilderPattern, Each};
pub(crate) use setter::Setter;

const DEFAULT_STRUCT_NAME: &str = "__default";

/// Derive a builder for a struct
pub fn builder_for_struct(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let opts = match macro_options::Options::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors();
        }
    };

    let mut builder = opts.as_builder();
    let mut build_fn = opts.as_build_method();

    builder.doc_comment(format!(
        include_str!("doc_tpl/builder_struct.md"),
        struct_name = ast.ident
    ));
    build_fn.doc_comment(format!(
        include_str!("doc_tpl/builder_method.md"),
        struct_name = ast.ident
    ));

    for field in opts.fields() {
        builder.push_field(field.as_builder_field());
        builder.push_setter_fn(field.as_setter());
        build_fn.push_initializer(field.as_initializer());
    }

    builder.push_build_fn(build_fn);

    quote!(#builder)
}
