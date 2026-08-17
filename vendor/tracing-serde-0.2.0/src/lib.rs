//! # tracing-serde
//!
//! An adapter for serializing [`tracing`] types using [`serde`].
//!
//! [![Documentation][docs-badge]][docs-url]
//! [![Documentation (master)][docs-master-badge]][docs-master-url]
//!
//! [docs-badge]: https://docs.rs/tracing-serde/badge.svg
//! [docs-url]: https://docs.rs/tracing-serde
//! [docs-master-badge]: https://img.shields.io/badge/docs-master-blue
//! [docs-master-url]: https://tracing-rs.netlify.com/tracing_serde
//!
//! ## Overview
//!
//! [`tracing`] is a framework for instrumenting Rust programs to collect
//! scoped, structured, and async-aware diagnostics.`tracing-serde` enables
//! serializing `tracing` types using [`serde`].
//!
//! Traditional logging is based on human-readable text messages.
//! `tracing` gives us machine-readable structured diagnostic
//! information. This lets us interact with diagnostic data
//! programmatically. With `tracing-serde`, you can implement a
//! `Subscriber` to serialize your `tracing` types and make use of the
//! existing ecosystem of `serde` serializers to talk with distributed
//! tracing systems.
//!
//! Serializing diagnostic information allows us to do more with our logged
//! values. For instance, when working with logging data in JSON gives us
//! pretty-print when we're debugging in development and you can emit JSON
//! and tracing data to monitor your services in production.
//!
//! The `tracing` crate provides the APIs necessary for instrumenting
//! libraries and applications to emit trace data.
//!
//! *Compiler support: [requires `rustc` 1.63+][msrv]*
//!
//! [msrv]: #supported-rust-versions
//!
//! ## Usage
//!
//! First, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tracing = "0.1"
//! tracing-serde = "0.2"
//! ```
//!
//! Next, add this to your crate:
//!
//! ```rust
//! use tracing_serde::AsSerde;
//! ```
//!
//! Please read the [`tracing` documentation](https://docs.rs/tracing/latest/tracing/index.html)
//! for more information on how to create trace data.
//!
//! This crate provides the `as_serde` function, via the `AsSerde` trait,
//! which enables serializing the `Attributes`, `Event`, `Id`, `Metadata`,
//! and `Record` `tracing` values.
//!
//! For the full example, please see the [examples](../examples) folder.
//!
//! Implement a `Subscriber` to format the serialization of `tracing`
//! types how you'd like.
//!
//! ```rust
//! # use tracing_core::{Subscriber, Metadata, Event};
//! # use tracing_core::span::{Attributes, Id, Record};
//! # use std::sync::atomic::{AtomicUsize, Ordering};
//! use tracing_serde::AsSerde;
//! use serde_json::json;
//!
//! pub struct JsonSubscriber {
//!     next_id: AtomicUsize, // you need to assign span IDs, so you need a counter
//! }
//!
//! impl JsonSubscriber {
//!     fn new() -> Self {
//!         Self { next_id: 1.into() }
//!     }
//! }
//!
//! impl Subscriber for JsonSubscriber {
//!
//!     fn new_span(&self, attrs: &Attributes<'_>) -> Id {
//!         let id = self.next_id.fetch_add(1, Ordering::Relaxed);
//!         let id = Id::from_u64(id as u64);
//!         let json = json!({
//!         "new_span": {
//!             "attributes": attrs.as_serde(),
//!             "id": id.as_serde(),
//!         }});
//!         println!("{}", json);
//!         id
//!     }
//!
//!     fn event(&self, event: &Event<'_>) {
//!         let json = json!({
//!            "event": event.as_serde(),
//!         });
//!         println!("{}", json);
//!     }
//!
//!     // ...
//!     # fn enabled(&self, _: &Metadata<'_>) -> bool { true }
//!     # fn enter(&self, _: &Id) {}
//!     # fn exit(&self, _: &Id) {}
//!     # fn record(&self, _: &Id, _: &Record<'_>) {}
//!     # fn record_follows_from(&self, _: &Id, _: &Id) {}
//! }
//! ```
//!
//! After you implement your `Subscriber`, you can use your `tracing`
//! subscriber (`JsonSubscriber` in the above example) to record serialized
//! trace data.
//!
//! ### Unstable Features
//!
//! These feature flags enable **unstable** features. The public API may break in 0.1.x
//! releases. To enable these features, the `--cfg tracing_unstable` must be passed to
//! `rustc` when compiling.
//!
//! The following unstable feature flags are currently available:
//!
//! * `valuable`: Enables [`Visit::record_value`] implementations, for
//!   serializing values recorded using the [`valuable`] crate.
//!
//! #### Enabling Unstable Features
//!
//! The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
//! env variable when running `cargo` commands:
//!
//! ```shell
//! RUSTFLAGS="--cfg tracing_unstable" cargo build
//! ```
//! Alternatively, the following can be added to the `.cargo/config` file in a
//! project to automatically enable the cfg flag for that project:
//!
//! ```toml
//! [build]
//! rustflags = ["--cfg", "tracing_unstable"]
//! ```
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//! [`valuable`]: https://crates.io/crates/valuable
//!
//! ## Supported Rust Versions
//!
//! Tracing is built against the latest stable release. The minimum supported
//! version is 1.63. The current Tracing version is not guaranteed to build on
//! Rust versions earlier than the minimum supported version.
//!
//! Tracing follows the same compiler support policies as the rest of the Tokio
//! project. The current stable Rust compiler and the three most recent minor
//! versions before it will always be supported. For example, if the current
//! stable compiler version is 1.69, the minimum supported version will not be
//! increased past 1.66, three minor versions prior. Increasing the minimum
//! supported compiler version is not considered a semver breaking change as
//! long as doing so complies with this policy.
//!
//! [`tracing`]: https://crates.io/crates/tracing
//! [`serde`]: https://crates.io/crates/serde
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/logo-type.png",
    issue_tracker_base_url = "https://github.com/tokio-rs/tracing/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, deny(rustdoc::broken_intra_doc_links))]
#![warn(
    missing_debug_implementations,
    // missing_docs, // TODO: add documentation
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_interfaces,
    private_bounds,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
use std::fmt;

use serde::{
    ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeTupleStruct, Serializer},
    Serialize,
};

use tracing_core::{
    event::Event,
    field::{Field, FieldSet, Visit},
    metadata::{Level, Metadata},
    span::{Attributes, Id, Record},
};

pub mod fields;

#[derive(Debug)]
pub struct SerializeField<'a>(&'a Field);

impl<'a> Serialize for SerializeField<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.name())
    }
}

#[derive(Debug)]
pub struct SerializeFieldSet<'a>(&'a FieldSet);

impl<'a> Serialize for SerializeFieldSet<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for element in self.0 {
            seq.serialize_element(&SerializeField(&element))?;
        }
        seq.end()
    }
}

#[derive(Debug)]
pub struct SerializeLevel<'a>(&'a Level);

impl<'a> Serialize for SerializeLevel<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0 == &Level::ERROR {
            serializer.serialize_str("ERROR")
        } else if self.0 == &Level::WARN {
            serializer.serialize_str("WARN")
        } else if self.0 == &Level::INFO {
            serializer.serialize_str("INFO")
        } else if self.0 == &Level::DEBUG {
            serializer.serialize_str("DEBUG")
        } else if self.0 == &Level::TRACE {
            serializer.serialize_str("TRACE")
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct SerializeId<'a>(&'a Id);

impl<'a> Serialize for SerializeId<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_tuple_struct("Id", 1)?;
        state.serialize_field(&self.0.into_u64())?;
        state.end()
    }
}

#[derive(Debug)]
pub struct SerializeMetadata<'a>(&'a Metadata<'a>);

impl<'a> Serialize for SerializeMetadata<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Metadata", 9)?;
        state.serialize_field("name", self.0.name())?;
        state.serialize_field("target", self.0.target())?;
        state.serialize_field("level", &SerializeLevel(self.0.level()))?;
        state.serialize_field("module_path", &self.0.module_path())?;
        state.serialize_field("file", &self.0.file())?;
        state.serialize_field("line", &self.0.line())?;
        state.serialize_field("fields", &SerializeFieldSet(self.0.fields()))?;
        state.serialize_field("is_span", &self.0.is_span())?;
        state.serialize_field("is_event", &self.0.is_event())?;
        state.end()
    }
}

/// Implements `serde::Serialize` to write `Event` data to a serializer.
#[derive(Debug)]
pub struct SerializeEvent<'a>(&'a Event<'a>);

impl<'a> Serialize for SerializeEvent<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_struct("Event", 2)?;
        serializer.serialize_field("metadata", &SerializeMetadata(self.0.metadata()))?;
        let mut visitor = SerdeStructVisitor {
            serializer,
            state: Ok(()),
        };
        self.0.record(&mut visitor);
        visitor.finish()
    }
}

/// Implements `serde::Serialize` to write `Attributes` data to a serializer.
#[derive(Debug)]
pub struct SerializeAttributes<'a>(&'a Attributes<'a>);

impl<'a> Serialize for SerializeAttributes<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_struct("Attributes", 3)?;
        serializer.serialize_field("metadata", &SerializeMetadata(self.0.metadata()))?;
        serializer.serialize_field("parent", &self.0.parent().map(SerializeId))?;
        serializer.serialize_field("is_root", &self.0.is_root())?;

        let mut visitor = SerdeStructVisitor {
            serializer,
            state: Ok(()),
        };
        self.0.record(&mut visitor);
        visitor.finish()
    }
}

/// Implements `serde::Serialize` to write `Record` data to a serializer.
#[derive(Debug)]
pub struct SerializeRecord<'a>(&'a Record<'a>);

impl<'a> Serialize for SerializeRecord<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serializer = serializer.serialize_map(None)?;
        let mut visitor = SerdeMapVisitor::new(serializer);
        self.0.record(&mut visitor);
        visitor.finish()
    }
}

/// Implements `tracing_core::field::Visit` for some `serde::ser::SerializeMap`.
#[derive(Debug)]
pub struct SerdeMapVisitor<S: SerializeMap> {
    serializer: S,
    state: Result<(), S::Error>,
}

impl<S> SerdeMapVisitor<S>
where
    S: SerializeMap,
{
    /// Create a new map visitor.
    pub fn new(serializer: S) -> Self {
        Self {
            serializer,
            state: Ok(()),
        }
    }

    /// Completes serializing the visited object, returning `Ok(())` if all
    /// fields were serialized correctly, or `Error(S::Error)` if a field could
    /// not be serialized.
    pub fn finish(self) -> Result<S::Ok, S::Error> {
        self.state?;
        self.serializer.end()
    }

    /// Completes serializing the visited object, returning ownership of the underlying serializer
    /// if all fields were serialized correctly, or `Err(S::Error)` if a field could not be
    /// serialized.
    pub fn take_serializer(self) -> Result<S, S::Error> {
        self.state?;
        Ok(self.serializer)
    }
}

impl<S> Visit for SerdeMapVisitor<S>
where
    S: SerializeMap,
{
    #[cfg(all(tracing_unstable, feature = "valuable"))]
    #[cfg_attr(docsrs, doc(cfg(all(tracing_unstable, feature = "valuable"))))]
    fn record_value(&mut self, field: &Field, value: valuable_crate::Value<'_>) {
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_entry(field.name(), &valuable_serde::Serializable::new(value));
        }
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        // If previous fields serialized successfully, continue serializing,
        // otherwise, short-circuit and do nothing.
        if self.state.is_ok() {
            self.state = self.serializer.serialize_entry(field.name(), &value)
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_entry(field.name(), &format_args!("{:?}", value))
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if self.state.is_ok() {
            self.state = self.serializer.serialize_entry(field.name(), &value)
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        if self.state.is_ok() {
            self.state = self.serializer.serialize_entry(field.name(), &value)
        }
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        if self.state.is_ok() {
            self.state = self.serializer.serialize_entry(field.name(), &value)
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if self.state.is_ok() {
            self.state = self.serializer.serialize_entry(field.name(), &value)
        }
    }
}

/// Implements `tracing_core::field::Visit` for some `serde::ser::SerializeStruct`.
#[derive(Debug)]
pub struct SerdeStructVisitor<S: SerializeStruct> {
    serializer: S,
    state: Result<(), S::Error>,
}

impl<S> Visit for SerdeStructVisitor<S>
where
    S: SerializeStruct,
{
    #[cfg(all(tracing_unstable, feature = "valuable"))]
    #[cfg_attr(docsrs, doc(cfg(all(tracing_unstable, feature = "valuable"))))]
    fn record_value(&mut self, field: &Field, value: valuable_crate::Value<'_>) {
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_field(field.name(), &valuable_serde::Serializable::new(value));
        }
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        // If previous fields serialized successfully, continue serializing,
        // otherwise, short-circuit and do nothing.
        if self.state.is_ok() {
            self.state = self.serializer.serialize_field(field.name(), &value)
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_field(field.name(), &format_args!("{:?}", value))
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if self.state.is_ok() {
            self.state = self.serializer.serialize_field(field.name(), &value)
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        if self.state.is_ok() {
            self.state = self.serializer.serialize_field(field.name(), &value)
        }
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        if self.state.is_ok() {
            self.state = self.serializer.serialize_field(field.name(), &value)
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if self.state.is_ok() {
            self.state = self.serializer.serialize_field(field.name(), &value)
        }
    }
}

impl<S: SerializeStruct> SerdeStructVisitor<S> {
    /// Completes serializing the visited object, returning `Ok(())` if all
    /// fields were serialized correctly, or `Error(S::Error)` if a field could
    /// not be serialized.
    pub fn finish(self) -> Result<S::Ok, S::Error> {
        self.state?;
        self.serializer.end()
    }
}

pub trait AsSerde<'a>: self::sealed::Sealed {
    type Serializable: serde::Serialize + 'a;

    /// `as_serde` borrows a `tracing` value and returns the serialized value.
    fn as_serde(&'a self) -> Self::Serializable;
}

impl<'a> AsSerde<'a> for tracing_core::Metadata<'a> {
    type Serializable = SerializeMetadata<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeMetadata(self)
    }
}

impl<'a> AsSerde<'a> for tracing_core::Event<'a> {
    type Serializable = SerializeEvent<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeEvent(self)
    }
}

impl<'a> AsSerde<'a> for tracing_core::span::Attributes<'a> {
    type Serializable = SerializeAttributes<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeAttributes(self)
    }
}

impl<'a> AsSerde<'a> for tracing_core::span::Id {
    type Serializable = SerializeId<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeId(self)
    }
}

impl<'a> AsSerde<'a> for tracing_core::span::Record<'a> {
    type Serializable = SerializeRecord<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeRecord(self)
    }
}

impl<'a> AsSerde<'a> for Level {
    type Serializable = SerializeLevel<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeLevel(self)
    }
}

impl<'a> AsSerde<'a> for Field {
    type Serializable = SerializeField<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeField(self)
    }
}

impl<'a> AsSerde<'a> for FieldSet {
    type Serializable = SerializeFieldSet<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeFieldSet(self)
    }
}

impl<'a> self::sealed::Sealed for Event<'a> {}

impl<'a> self::sealed::Sealed for Attributes<'a> {}

impl self::sealed::Sealed for Id {}

impl self::sealed::Sealed for Level {}

impl<'a> self::sealed::Sealed for Record<'a> {}

impl<'a> self::sealed::Sealed for Metadata<'a> {}

impl self::sealed::Sealed for Field {}

impl self::sealed::Sealed for FieldSet {}

mod sealed {
    pub trait Sealed {}
}
