#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
//! Valuable provides object-safe value inspection. Use cases include passing
//! structured data to trait objects and object-safe serialization.
//!
//! # Getting started
//!
//! First, derive [`Valuable`][macro@crate::Valuable] on your types.
//!
//! ```
//! use valuable::Valuable;
//!
//! #[derive(Valuable)]
//! struct HelloWorld {
//!     message: Message,
//! }
//!
//! #[derive(Valuable)]
//! enum Message {
//!     HelloWorld,
//!     Custom(String),
//! }
//! ```
//!
//! Then, implement a [visitor][Visit] to inspect the data.
//!
//! ```
//! use valuable::{NamedValues, Value, Valuable, Visit};
//!
//! struct Print;
//!
//! impl Visit for Print {
//!     fn visit_value(&mut self, value: Value<'_>) {
//!         match value {
//!             Value::Structable(v) => {
//!                 println!("struct {}", v.definition().name());
//!                 v.visit(self);
//!             }
//!             Value::Enumerable(v) => {
//!                 println!("enum {}::{}", v.definition().name(), v.variant().name());
//!                 v.visit(self);
//!             }
//!             Value::Listable(v) => {
//!                 println!("list");
//!                 v.visit(self);
//!             }
//!             Value::Mappable(v) => {
//!                 println!("map");
//!                 v.visit(self);
//!             }
//!             _ => {
//!                 println!("value {:?}", value);
//!             }
//!         }
//!     }
//!
//!     fn visit_named_fields(&mut self, named_fields: &NamedValues<'_>) {
//!         for (field, value) in named_fields.iter() {
//!             println!("named field {}", field.name());
//!             value.visit(self);
//!         }
//!     }
//!
//!     fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
//!         for value in values {
//!             value.visit(self);
//!         }
//!     }
//!
//!     fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
//!         println!("key / value");
//!         key.visit(self);
//!         value.visit(self);
//!     }
//! }
//! ```
//!
//! Then, use the visitor to visit the value.
//!
//! ```
//! # use valuable::*;
//! # #[derive(Valuable)]
//! # struct HelloWorld { message: Message }
//! # #[derive(Valuable)]
//! # enum Message { HelloWorld }
//! # struct Print;
//! # impl Visit for Print {
//! #       fn visit_value(&mut self, _: Value<'_>) {}
//! # }
//! let hello_world = HelloWorld { message: Message::HelloWorld };
//! hello_world.visit(&mut Print);
//! ```
//!
//! # Related Crates
//!
//! - [`valuable-serde`] provides a bridge between `valuable` and the [`serde`]
//!   serialization ecosystem. Using [`valuable_serde::Serializable`] allows any
//!   type that implements [`Valuable`] to be serialized by any
//!   [`serde::ser::Serializer`].
//!
//! [`valuable-serde`]: https://crates.io/crates/valuable-serde
//! [`serde`]: https://crates.io/crates/serde
//! [`valuable_serde::Serializable`]: https://docs.rs/valuable-serde/latest/valuable_serde/struct.Serializable.html
//! [`serde::ser::Serializer`]:  https://docs.rs/serde/latest/serde/ser/trait.Serializer.html
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg, doc_cfg_hide))]
#![cfg_attr(
    docsrs,
    doc(cfg_hide(
        not(valuable_no_atomic_cas),
        not(valuable_no_atomic),
        not(valuable_no_atomic_64)
    ))
)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod enumerable;
pub use enumerable::{EnumDef, Enumerable, Variant, VariantDef};

mod field;
pub use field::{Fields, NamedField};

mod listable;
pub use listable::Listable;

mod mappable;
pub use mappable::Mappable;

mod named_values;
pub use named_values::NamedValues;

mod slice;
pub use slice::Slice;

mod structable;
pub use structable::{StructDef, Structable};

mod tuplable;
pub use tuplable::{Tuplable, TupleDef};

mod valuable;
pub use crate::valuable::Valuable;

mod value;
pub use value::Value;

mod visit;
pub use visit::{visit, Visit};

#[cfg(feature = "derive")]
pub use valuable_derive::Valuable;
