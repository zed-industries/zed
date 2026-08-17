// Copyright 2015, Yuheng Chen.
// Copyright 2023, Ethiraric.
// See the LICENSE file at the top-level directory of this distribution.

//! YAML 1.2 implementation in pure Rust.
//!
//! # Usage
//!
//! This crate is [on github](https://github.com/Ethiraric/yaml-rust2) and can be used by adding
//! `yaml-rust2` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! yaml-rust2 = "0.8.1"
//! ```
//!
//! # Examples
//! Parse a string into `Vec<Yaml>` and then serialize it as a YAML string.
//!
//! ```
//! use yaml_rust2::{YamlLoader, YamlEmitter};
//!
//! let docs = YamlLoader::load_from_str("[1, 2, 3]").unwrap();
//! let doc = &docs[0]; // select the first YAML document
//! assert_eq!(doc[0].as_i64().unwrap(), 1); // access elements by index
//!
//! let mut out_str = String::new();
//! let mut emitter = YamlEmitter::new(&mut out_str);
//! emitter.dump(doc).unwrap(); // dump the YAML object to a String
//!
//! ```
//!
//! # Features
//! **Note:** With all features disabled, this crate's MSRV is `1.65.0`.
//!
//! #### `encoding` (_enabled by default_)
//! Enables encoding-aware decoding of Yaml documents.
//!
//! The MSRV for this feature is `1.70.0`.
//!
//! #### `debug_prints`
//! Enables the `debug` module and usage of debug prints in the scanner and the parser. Do not
//! enable if you are consuming the crate rather than working on it as this can significantly
//! decrease performance.
//!
//! The MSRV for this feature is `1.70.0`.

#![warn(missing_docs, clippy::pedantic)]

extern crate hashlink;

pub(crate) mod char_traits;
#[macro_use]
pub(crate) mod debug;
pub mod emitter;
pub mod parser;
pub mod scanner;
pub mod yaml;

// reexport key APIs
pub use crate::emitter::{EmitError, YamlEmitter};
pub use crate::parser::Event;
pub use crate::scanner::ScanError;
pub use crate::yaml::{Yaml, YamlLoader};
