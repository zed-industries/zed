//! # Library to read and write protocol buffers data
//!
//! ## Features
//!
//! This crate has one feature, which is `with-bytes`.
//!
//! `with-bytes` enables `protobuf` crate support for
//! [`bytes` crate](https://github.com/tokio-rs/bytes):
//! when parsing bytes or strings from `bytes::Bytes`,
//! `protobuf` will be able to reference the input instead of allocating subarrays.
//!
//! Note, codegen also need to be instructed to generate `Bytes` or `Chars` for
//! `bytes` or `string` protobuf types instead of default `Vec<u8>` or `String`,
//! just enabling option on this crate is not enough.
//!
//! See `Customize` struct in [`protobuf-codegen` crate](https://docs.rs/protobuf-codegen).
//!
//! ## Accompanying crates
//!
//! * [`protobuf-json-mapping`](https://docs.rs/protobuf-json-mapping)
//!   implements JSON parsing and serialization for protobuf messages.
//! * [`protobuf-codegen`](https://docs.rs/protobuf-codegen)
//!   can be used to generate rust code from `.proto` crates.
//! * [`protoc-bin-vendored`](https://docs.rs/protoc-bin-vendored)
//!   contains `protoc` command packed into the crate.
//! * [`protobuf-parse`](https://docs.rs/protobuf-parse) contains
//!   `.proto` file parser. Rarely need to be used directly,
//!   but can be used for mechanical processing of `.proto` files.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

pub use crate::coded_input_stream::CodedInputStream;
pub use crate::coded_output_stream::CodedOutputStream;
pub use crate::enum_full::EnumFull;
pub use crate::enum_or_unknown::EnumOrUnknown;
pub use crate::enums::Enum;
pub use crate::message::Message;
pub use crate::message_dyn::MessageDyn;
pub use crate::message_field::MessageField;
pub use crate::message_full::MessageFull;
pub use crate::oneof::Oneof;
pub use crate::oneof_full::OneofFull;
pub use crate::special::SpecialFields;
pub use crate::unknown::UnknownFields;
pub use crate::unknown::UnknownFieldsIter;
pub use crate::unknown::UnknownValue;
pub use crate::unknown::UnknownValueRef;
pub(crate) mod wire_format;
#[cfg(feature = "bytes")]
pub use crate::chars::Chars;
pub use crate::error::Error;
pub use crate::error::Result;

// generated
pub mod descriptor;
pub mod plugin;
pub mod rustproto;

mod byteorder;
mod coded_input_stream;
mod coded_output_stream;
mod enum_full;
mod enum_or_unknown;
mod enums;
mod error;
pub mod ext;
mod lazy;
mod message;
mod message_dyn;
mod message_field;
mod message_full;
mod oneof;
mod oneof_full;
mod owning_ref;
pub mod reflect;
pub mod rt;
pub mod text_format;
pub mod well_known_types;
mod well_known_types_util;

// used by test
#[cfg(test)]
#[path = "../../test-crates/protobuf-test-common/src/hex.rs"]
mod hex;

mod cached_size;
mod chars;
mod fixed;
mod special;
mod unknown;
mod varint;
mod zigzag;

mod misc;

// This does not work: https://github.com/rust-lang/rust/issues/67295
#[cfg(doctest)]
mod doctest_pb;

/// This symbol is in generated `version.rs`, include here for IDE
#[cfg(never)]
pub const VERSION: &str = "";
/// This symbol is in generated `version.rs`, include here for IDE
#[cfg(never)]
#[doc(hidden)]
pub const VERSION_IDENT: &str = "";
include!(concat!(env!("OUT_DIR"), "/version.rs"));
