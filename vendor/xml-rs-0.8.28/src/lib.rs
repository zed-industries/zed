#![warn(missing_docs)]
#![forbid(non_camel_case_types)]
#![forbid(unsafe_code)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::module_name_repetitions)]

//! This crate currently provides an almost XML 1.0/1.1-compliant pull parser.
//!
//! Please note that functions of this parser may panic.
//! If a panic could cause a Denial Of Service in your codebase, *you're* responsible for wrapping access to this library in `catch_unwind`.

#![cfg_attr(doctest, doc = include_str!("../README.md"))]

pub use crate::reader::{EventReader, ParserConfig};
pub use crate::util::Encoding;
pub use crate::writer::{EmitterConfig, EventWriter};

pub mod attribute;
pub mod common;
pub mod escape;
#[doc(hidden)] // FIXME: not supposed to be public
pub mod macros;
pub mod name;
pub mod namespace;
pub mod reader;
mod util;
pub mod writer;
