//! # ANSI Escape Sequences provider & parser
//!
//! ## Sequences provider
//!
//! The `anes` crate provides ANSI escape sequences you can use to control the terminal
//! cursor (show, hide, ...), colors (foreground, background), display attributes (bold, ...)
//! and many others.
//!
//! Every sequence implements the standard library [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
//! trait. It means that these sequences can be used in macros like
//! [`format!`](https://doc.rust-lang.org/std/macro.format.html) or
//! [`write!`](https://doc.rust-lang.org/std/macro.write.html).
//!
//! Ask if you need more sequences or use the [`sequence!`](macro.sequence.html) macro to create
//! your own sequences.
//!
//!
//! ### Terminal Support
//!
//! Not all ANSI escape sequences are supported by all terminals. You can use the
//! [interactive-test](https://github.com/zrzka/anes-rs/tree/master/interactive-test) to test them.
//!
//! ### Examples
//!
//! Retrieve the sequence as a `String`:
//!
//! ```rust
//! use anes::SaveCursorPosition;
//!
//! let string = format!("{}", SaveCursorPosition);
//! assert_eq!(&string, "\x1B7");
//! ```
//!
//! Execute the sequence on the standard output:
//!
//! ```rust
//! use std::io::{Result, Write};
//!
//! use anes::execute;
//!
//! fn main() -> Result<()> {
//!     let mut stdout = std::io::stdout();
//!     execute!(&mut stdout, anes::ResetAttributes)
//! }
//! ```
//!
//! ## Sequences parser
//!
//! Parser isn't available with default features. You have to enable `parser` feature if you'd like to use it.
//! You can learn more about this feature in the [`parser`](parser/index.html) module documentation.
#![warn(rust_2018_idioms)]
#![deny(unused_imports, unused_must_use)]

// Keep it first to load all the macros before other modules.
#[macro_use]
mod macros;

pub use self::sequences::{
    attribute::{Attribute, ResetAttributes, SetAttribute},
    buffer::{
        ClearBuffer, ClearLine, ScrollBufferDown, ScrollBufferUp, SwitchBufferToAlternate,
        SwitchBufferToNormal,
    },
    color::{Color, SetBackgroundColor, SetForegroundColor},
    cursor::{
        DisableCursorBlinking, EnableCursorBlinking, HideCursor, MoveCursorDown, MoveCursorLeft,
        MoveCursorRight, MoveCursorTo, MoveCursorToColumn, MoveCursorToNextLine,
        MoveCursorToPreviousLine, MoveCursorUp, ReportCursorPosition, RestoreCursorPosition,
        SaveCursorPosition, ShowCursor,
    },
    terminal::{DisableMouseEvents, EnableMouseEvents, ResizeTextArea},
};

#[cfg(feature = "parser")]
pub mod parser;
mod sequences;
