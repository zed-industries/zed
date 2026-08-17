#![allow(
    clippy::cast_sign_loss,
    clippy::doc_markdown,
    clippy::elidable_lifetime_names,
    clippy::enum_glob_use,
    clippy::expl_impl_clone_on_copy, // https://github.com/rust-lang/rust-clippy/issues/15842
    clippy::inherent_to_string,
    clippy::items_after_statements,
    clippy::match_bool,
    clippy::match_like_matches_macro,
    clippy::match_same_arms,
    clippy::needless_lifetimes,
    clippy::needless_pass_by_value,
    clippy::nonminimal_bool,
    clippy::redundant_else,
    clippy::ref_option,
    clippy::similar_names,
    clippy::single_match_else,
    clippy::struct_field_names,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::toplevel_ref_arg,
    clippy::uninlined_format_args,
    clippy::wrong_self_convention
)]
#![cfg_attr(test, allow(dead_code, unfulfilled_lint_expectations))]
#![allow(unknown_lints, mismatched_lifetime_syntaxes)]

mod attrs;
mod cfg;
mod derive;
mod expand;
mod generics;
mod syntax;
#[cfg(test)]
mod tests;
mod tokens;
mod type_id;

use crate::syntax::file::Module;
use crate::syntax::namespace::Namespace;
use crate::syntax::qualified::QualifiedName;
use crate::type_id::Crate;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::parse_macro_input;

/// `#[cxx::bridge] mod ffi { ... }`
///
/// Refer to the crate-level documentation for the explanation of how this macro
/// is intended to be used.
///
/// The only additional thing to note here is namespace support &mdash; if the
/// types and functions on the `extern "C++"` side of our bridge are in a
/// namespace, specify that namespace as an argument of the cxx::bridge
/// attribute macro.
///
/// ```
/// #[cxx::bridge(namespace = "mycompany::rust")]
/// # mod ffi {}
/// ```
///
/// The types and functions from the `extern "Rust"` side of the bridge will be
/// placed into that same namespace in the generated C++ code.
#[proc_macro_attribute]
pub fn bridge(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = syntax::error::ERRORS;

    let namespace = match Namespace::parse_bridge_attr_namespace.parse(args) {
        Ok(namespace) => namespace,
        Err(err) => return err.to_compile_error().into(),
    };
    let mut ffi = parse_macro_input!(input as Module);
    ffi.namespace = namespace;

    expand::bridge(ffi)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[doc(hidden)]
#[proc_macro]
pub fn type_id(input: TokenStream) -> TokenStream {
    struct TypeId {
        krate: Crate,
        path: QualifiedName,
    }

    impl Parse for TypeId {
        fn parse(input: ParseStream) -> Result<Self> {
            let krate = input.parse().map(Crate::DollarCrate)?;
            let path = QualifiedName::parse_quoted_or_unquoted(input)?;
            Ok(TypeId { krate, path })
        }
    }

    let arg = parse_macro_input!(input as TypeId);
    type_id::expand(arg.krate, arg.path).into()
}
