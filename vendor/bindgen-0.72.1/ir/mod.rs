//! The ir module defines bindgen's intermediate representation.
//!
//! Parsing C/C++ generates the IR, while code generation outputs Rust code from
//! the IR.
#![deny(clippy::missing_docs_in_private_items)]

pub(crate) mod analysis;
pub(crate) mod annotations;
pub(crate) mod comment;
pub(crate) mod comp;
pub(crate) mod context;
pub(crate) mod derive;
pub(crate) mod dot;
pub(crate) mod enum_ty;
pub(crate) mod function;
pub(crate) mod int;
pub(crate) mod item;
pub(crate) mod item_kind;
pub(crate) mod layout;
pub(crate) mod module;
pub(crate) mod objc;
pub(crate) mod template;
pub(crate) mod traversal;
pub(crate) mod ty;
pub(crate) mod var;
