#![doc = include_str!("../README.md")]

/// Types related to parse errors
pub mod error;
/// Types related to cfg expressions
pub mod expr;
/// Types related to rustc targets
pub mod targets;

pub use error::ParseError;
pub use expr::{Expression, Predicate, TargetPredicate};

#[cfg(feature = "targets")]
pub use target_lexicon;
