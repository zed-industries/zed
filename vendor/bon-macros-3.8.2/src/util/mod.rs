mod attrs;
mod expr;
mod fn_arg;
mod generic_param;
mod ident;
mod item;
mod iterator;
mod meta_list;
mod path;
mod punctuated;
mod ty;
mod vec;
mod visibility;

pub(crate) mod ide;

pub(crate) mod prelude {
    pub(crate) use proc_macro2::{Span, TokenStream};
    pub(crate) use quote::{format_ident, quote, quote_spanned, ToTokens};

    /// The `Error` type in in this crate is supposed to act like `anyhow::Error`
    /// providing a simple way to create and return errors from format strings.
    ///
    /// See [`err!()`] and [`bail!()`] macros for creating errors. Right now this
    /// is just a reexport of [`darling::Error`] because that error already provides
    /// the anyhow-like error handling experience.
    pub(crate) use darling::Error;

    pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;

    pub(crate) use super::attrs::AttributeExt;
    pub(crate) use super::expr::ExprExt;
    pub(crate) use super::fn_arg::FnArgExt;
    pub(crate) use super::generic_param::GenericParamExt;
    pub(crate) use super::ident::IdentExt;
    pub(crate) use super::item::ItemExt;
    pub(crate) use super::iterator::{IntoIteratorExt, IteratorExt};
    pub(crate) use super::meta_list::MetaListExt;
    pub(crate) use super::path::PathExt;
    pub(crate) use super::punctuated::PunctuatedExt;
    pub(crate) use super::ty::TypeExt;
    pub(crate) use super::vec::VecExt;
    pub(crate) use super::visibility::VisibilityExt;
    pub(crate) use super::{bail, err};
}

/// Inspired by `anyhow::bail`, but returns a [`Result`] with [`darling::Error`].
/// It accepts the value that implements [`syn::spanned::Spanned`] to attach the
/// span to the error.
#[allow(edition_2024_expr_fragment_specifier)]
macro_rules! bail {
    ($spanned:expr, $($tt:tt)*) => {
        return Err($crate::util::err!($spanned, $($tt)*))
    };
}

/// Inspired by `anyhow::anyhow`, but returns a [`darling::Error`].
/// It accepts the value that implements [`syn::spanned::Spanned`] to attach the
/// span to the error.
#[allow(edition_2024_expr_fragment_specifier)]
macro_rules! err {
    ($spanned:expr, $($tt:tt)*) => {
        ::darling::Error::custom(format_args!($($tt)*)).with_span($spanned)
    };
}

pub(crate) use {bail, err};
