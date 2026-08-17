//! `pbjson-types` provides the `google.protobuf` types, commonly known as well-known-types,
//! with [`serde::Serialize`][1] and [`serde::Deserialize`][2] implementations
//! that are compliant with the [protobuf JSON mapping][3]
//!
//! __Note: Coverage of all types is currently incomplete,
//! some may have non-compliant implementations__
//!
//! [1]: https://docs.rs/serde/1.0.130/serde/trait.Serialize.html
//! [2]: https://docs.rs/serde/1.0.130/serde/trait.Deserialize.html
//! [3]: https://developers.google.com/protocol-buffers/docs/proto3#json

#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls, rust_2018_idioms)]
#![warn(
    missing_debug_implementations,
    clippy::explicit_iter_loop,
    clippy::use_self,
    clippy::clone_on_ref_ptr,
    clippy::future_not_send
)]

#[allow(
    unused_imports,
    clippy::redundant_static_lifetimes,
    clippy::redundant_closure,
    clippy::redundant_field_names,
    clippy::clone_on_ref_ptr,
    clippy::enum_variant_names,
    clippy::use_self
)]
mod pb {
    pub mod google {
        pub mod protobuf {
            include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
            include!(concat!(env!("OUT_DIR"), "/google.protobuf.serde.rs"));
        }
    }
}

mod duration;
mod list_value;
mod null_value;
mod r#struct;
mod timestamp;
pub mod value;
mod wrappers;

pub use pb::google::protobuf::*;
