//! Types and functions for parsing attribute options.
//!
//! Attribute parsing occurs in multiple stages:
//!
//! 1. Builder options on the struct are parsed into `OptionsBuilder<StructMode>`.
//! 1. The `OptionsBuilder<StructMode>` instance is converted into a starting point for the
//!    per-field options (`OptionsBuilder<FieldMode>`) and the finished struct-level config,
//!    called `StructOptions`.
//! 1. Each struct field is parsed, with discovered attributes overriding or augmenting the
//!    options specified at the struct level. This creates one `OptionsBuilder<FieldMode>` per
//!    struct field on the input/target type. Once complete, these get converted into
//!    `FieldOptions` instances.

mod darling_opts;

pub use self::darling_opts::Options;
