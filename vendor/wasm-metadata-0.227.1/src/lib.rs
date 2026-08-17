//! Read and manipulate WebAssembly metadata
//!
//! # Examples
//!
//! **Read metadata from a Wasm binary**
//!
//! ```no_run
//! # #![allow(unused)]
//! # fn main() -> Result<(), anyhow::Error> {
//! use wasm_metadata::Payload;
//! use std::fs;
//!
//! let wasm = fs::read("program.wasm")?;
//! let metadata = Payload::from_binary(&wasm)?.metadata();
//! # Ok(()) }
//! ```
//!
//! **Add metadata to a Wasm binary**
//!
//! ```no_run
//! # #![allow(unused)]
//! # fn main() -> Result<(), anyhow::Error> {
//! use wasm_metadata::*;
//! use std::fs;
//!
//! let wasm = fs::read("program.wasm")?;
//!
//! let metadata = AddMetadata {
//!     name: Some("program".to_owned()),
//!     language: vec![("tunalang".to_owned(), "1.0.0".to_owned())],
//!     processed_by: vec![("chashu-tools".to_owned(), "1.0.1".to_owned())],
//!     sdk: vec![],
//!     authors: Some(Authors::new("Chashu Cat")),
//!     description: Some(Description::new("Chashu likes tuna")),
//!     licenses: Some(Licenses::new("Apache-2.0 WITH LLVM-exception")?),
//!     source: Some(Source::new("https://github.com/chashu/chashu-tools")?),
//!     homepage: Some(Homepage::new("https://github.com/chashu/chashu-tools")?),
//!     revision: Some(Revision::new("de978e17a80c1118f606fce919ba9b7d5a04a5ad")),
//!     version: Some(Version::new("1.0.0")),
//! };
//!
//! let wasm = metadata.to_wasm(&wasm)?;
//! fs::write("program.wasm", &wasm)?;
//! # Ok(()) }
//! ```

#![warn(missing_debug_implementations, missing_docs)]

pub use add_metadata::AddMetadata;
pub use dependencies::Dependencies;
pub use metadata::Metadata;
pub use names::{ComponentNames, ModuleNames};
pub use oci_annotations::{Authors, Description, Homepage, Licenses, Revision, Source, Version};
pub use payload::Payload;
pub use producers::{Producers, ProducersField};

pub(crate) use rewrite::rewrite_wasm;

mod add_metadata;
mod dependencies;
mod metadata;
mod names;
mod oci_annotations;
mod payload;
mod producers;
mod rewrite;

pub(crate) mod utils;
