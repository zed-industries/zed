//! Marker traits for types that can be formatted and/or be written to.
//!
//! # Features
//!
//! This module is only exported with the "fmt" feature

mod format_marker;
mod write_marker;

#[doc(inline)]
pub use self::{
    format_marker::{FormatMarker, IsAFormatMarker, IsArrayKind, IsNotStdKind, IsStdKind},
    write_marker::{IsAStrWriter, IsAWriteMarker, IsNotAStrWriter, WriteMarker},
};
