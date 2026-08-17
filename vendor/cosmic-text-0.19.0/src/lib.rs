// SPDX-License-Identifier: MIT OR Apache-2.0

//! # COSMIC Text
//!
//! This library provides advanced text handling in a generic way. It provides abstractions for
//! shaping, font discovery, font fallback, layout, rasterization, and editing. Shaping utilizes
//! harfrust, font discovery utilizes fontdb, and the rasterization is optional and utilizes
//! swash. The other features are developed internal to this library.
//!
//! It is recommended that you start by creating a [`FontSystem`], after which you can create a
//! [`Buffer`], provide it with some text, and then inspect the layout it produces. At this
//! point, you can use the `SwashCache` to rasterize glyphs into either images or pixels.
//!
//! ```
//! use cosmic_text::{Attrs, Color, FontSystem, SwashCache, Buffer, Metrics, Shaping};
//!
//! // A FontSystem provides access to detected system fonts, create one per application
//! let mut font_system = FontSystem::new();
//!
//! // A SwashCache stores rasterized glyphs, create one per application
//! let mut swash_cache = SwashCache::new();
//!
//! // Text metrics indicate the font size and line height of a buffer
//! let metrics = Metrics::new(14.0, 20.0);
//!
//! // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
//! let mut buffer = Buffer::new(&mut font_system, metrics);
//!
//! // Borrow buffer together with the font system for more convenient method calls
//! let mut buffer = buffer.borrow_with(&mut font_system);
//!
//! // Attributes indicate what font to choose
//! let attrs = Attrs::new();
//!
//! // Set size and text
//! buffer.set_size(Some(80.0), Some(25.0));
//! buffer.set_text("Hello, Rust! 🦀\n", &attrs, Shaping::Advanced, None);
//!
//! // Inspect the output runs
//! for run in buffer.layout_runs() {
//!     for glyph in run.glyphs.iter() {
//!         println!("{:#?}", glyph);
//!     }
//! }
//!
//! // Create a default text color
//! let text_color = Color::rgb(0xFF, 0xFF, 0xFF);
//!
//! // Draw the buffer (for performance, instead use SwashCache directly)
//! buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
//!     // Fill in your code here for drawing rectangles
//! });
//! ```

// Not interested in these lints
#![allow(clippy::new_without_default)]
// TODO: address occurrences and then deny
//
// Overflows can produce unpredictable results and are only checked in debug builds
#![allow(clippy::arithmetic_side_effects)]
// Indexing a slice can cause panics and that is something we always want to avoid
#![allow(clippy::indexing_slicing)]
// Soundness issues
//
// Dereferencing unaligned pointers may be undefined behavior
#![deny(clippy::cast_ptr_alignment)]
// Avoid panicking in without information about the panic. Use expect
#![deny(clippy::unwrap_used)]
// Ensure all types have a debug impl
#![deny(missing_debug_implementations)]
// This is usually a serious issue - a missing import of a define where it is interpreted
// as a catch-all variable in a match, for example
#![deny(unreachable_patterns)]
// Ensure that all must_use results are used
#![deny(unused_must_use)]
// Style issues
//
// Documentation not ideal
#![warn(clippy::doc_markdown)]
// Document possible errors
#![warn(clippy::missing_errors_doc)]
// Document possible panics
#![warn(clippy::missing_panics_doc)]
// Ensure semicolons are present
#![warn(clippy::semicolon_if_nothing_returned)]
// Ensure numbers are readable
#![warn(clippy::unreadable_literal)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[cfg(not(any(feature = "std", feature = "no_std")))]
compile_error!("Either the `std` or `no_std` feature must be enabled");

pub use self::attrs::*;
mod attrs;

pub use self::bidi_para::*;
mod bidi_para;

pub use self::buffer::*;
mod buffer;

pub use self::buffer_line::*;
mod buffer_line;

pub use self::cached::*;
mod cached;

pub use self::glyph_cache::*;
mod glyph_cache;

pub use self::cursor::*;
mod cursor;

pub use self::edit::*;
mod edit;

pub use self::font::*;
mod font;

pub use self::layout::*;
mod layout;

pub use self::line_ending::*;
mod line_ending;

pub use self::render::*;
mod render;

pub use self::shape::*;
mod shape;

pub use self::shape_run_cache::*;
mod shape_run_cache;

#[cfg(feature = "swash")]
pub use self::swash::*;
#[cfg(feature = "swash")]
mod swash;

mod math;

type BuildHasher = core::hash::BuildHasherDefault<rustc_hash::FxHasher>;

#[cfg(feature = "std")]
type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasher>;
#[cfg(not(feature = "std"))]
type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasher>;
