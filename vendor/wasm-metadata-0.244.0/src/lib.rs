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
//! let mut add = AddMetadata ::default();
//! add.name = AddMetadataField::Set("program".to_owned());
//! add.language = vec![("tunalang".to_owned(), "1.0.0".to_owned())];
//! add.processed_by = vec![("chashu-tools".to_owned(), "1.0.1".to_owned())];
//! add.sdk = vec![];
//! add.authors = AddMetadataField::Set(Authors::new("Chashu Cat"));
//! add.description = AddMetadataField::Set(Description::new("Chashu likes tuna"));
//! add.licenses = AddMetadataField::Set(Licenses::new("Apache-2.0 WITH LLVM-exception")?);
//! add.source = AddMetadataField::Set(Source::new("https://github.com/chashu/chashu-tools")?);
//! add.homepage = AddMetadataField::Set(Homepage::new("https://github.com/chashu/chashu-tools")?);
//! add.revision = AddMetadataField::Set(Revision::new("de978e17a80c1118f606fce919ba9b7d5a04a5ad"));
//! add.version = AddMetadataField::Set(Version::new("1.0.0"));
//!
//! let wasm = add.to_wasm(&wasm)?;
//! fs::write("program.wasm", &wasm)?;
//! # Ok(()) }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs)]

pub use add_metadata::{AddMetadata, AddMetadataField};
pub use names::{ComponentNames, ModuleNames};
pub use producers::{Producers, ProducersField};

pub(crate) use rewrite::rewrite_wasm;

mod add_metadata;
#[cfg(feature = "clap")]
mod clap;
mod names;
mod producers;
mod rewrite;

pub(crate) mod utils;

#[cfg(feature = "oci")]
mod dependencies;
#[cfg(feature = "oci")]
pub use dependencies::Dependencies;
#[cfg(feature = "oci")]
mod oci_annotations;
#[cfg(feature = "oci")]
pub use oci_annotations::{Authors, Description, Homepage, Licenses, Revision, Source, Version};
#[cfg(feature = "oci")]
mod metadata;
#[cfg(feature = "oci")]
pub use metadata::Metadata;
#[cfg(feature = "oci")]
mod payload;

#[cfg(feature = "oci")]
pub use payload::Payload;

#[cfg(feature = "clap")]
pub use clap::AddMetadataOpts;
