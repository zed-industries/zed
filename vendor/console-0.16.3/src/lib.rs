//! console is a library for Rust that provides access to various terminal
//! features so you can build nicer looking command line interfaces.  It
//! comes with various tools and utilities for working with Terminals and
//! formatting text.
//!
//! Best paired with other libraries in the family:
//!
//! * [dialoguer](https://docs.rs/dialoguer)
//! * [indicatif](https://docs.rs/indicatif)
//!
//! # Terminal Access
//!
//! The terminal is abstracted through the `console::Term` type.  It can
//! either directly provide access to the connected terminal or by buffering
//! up commands.  A buffered terminal will however not be completely buffered
//! on windows where cursor movements are currently directly passed through.
//!
//! Example usage:
//!
//! ```
//! # fn test() -> Result<(), Box<dyn std::error::Error>> {
//! use std::thread;
//! use std::time::Duration;
//!
//! use console::Term;
//!
//! let term = Term::stdout();
//! term.write_line("Hello World!")?;
//! thread::sleep(Duration::from_millis(2000));
//! term.clear_line()?;
//! # Ok(()) } test().unwrap();
//! ```
//!
//! # Colors and Styles
//!
//! `console` automatically detects when to use colors based on the tty flag.  It also
//! provides higher level wrappers for styling text and other things that can be
//! displayed with the `style` function and utility types.
//!
//! Example usage:
//!
//! ```
//! use console::style;
//!
//! println!("This is {} neat", style("quite").cyan());
//! ```
//!
//! You can also store styles and apply them to text later:
//!
//! ```
//! use console::Style;
//!
//! let cyan = Style::new().cyan();
//! println!("This is {} neat", cyan.apply_to("quite"));
//! ```
//!
//! # Working with ANSI Codes
//!
//! The crate provides the function `strip_ansi_codes` to remove ANSI codes
//! from a string as well as `measure_text_width` to calculate the width of a
//! string as it would be displayed by the terminal.  Both of those together
//! are useful for more complex formatting.
//!
//! # Unicode Width Support
//!
//! By default this crate depends on the `unicode-width` crate to calculate
//! the width of terminal characters.  If you do not need this you can disable
//! the `unicode-width` feature which will cut down on dependencies.
//!
//! # Features
//!
//! By default all features are enabled.  The following features exist:
//!
//! * `unicode-width`: adds support for unicode width calculations
//! * `ansi-parsing`: adds support for parsing ansi codes (this adds support
//!   for stripping and taking ansi escape codes into account for length
//!   calculations).

#![warn(
    unreachable_pub,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc
)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub use crate::kb::Key;
#[cfg(feature = "std")]
pub use crate::term::{
    user_attended, user_attended_stderr, Term, TermFamily, TermFeatures, TermTarget,
};
#[cfg(feature = "std")]
pub use crate::utils::{
    colors_enabled, colors_enabled_stderr, measure_text_width, pad_str, pad_str_with,
    set_colors_enabled, set_colors_enabled_stderr, set_true_colors_enabled,
    set_true_colors_enabled_stderr, style, true_colors_enabled, true_colors_enabled_stderr,
    truncate_str, Alignment, Attribute, Color, Emoji, Style, StyledObject,
};

#[cfg(all(feature = "ansi-parsing", feature = "alloc"))]
pub use crate::ansi::strip_ansi_codes;
#[cfg(feature = "ansi-parsing")]
pub use crate::ansi::{AnsiCodeIterator, WithoutAnsi};

#[cfg(feature = "std")]
mod common_term;
#[cfg(feature = "alloc")]
mod kb;
#[cfg(feature = "std")]
mod term;
#[cfg(all(unix, not(target_arch = "wasm32"), feature = "std"))]
mod unix_term;
#[cfg(feature = "std")]
mod utils;
#[cfg(all(feature = "std", target_arch = "wasm32"))]
mod wasm_term;
#[cfg(all(feature = "std", windows))]
mod windows_term;

#[cfg(feature = "ansi-parsing")]
mod ansi;
