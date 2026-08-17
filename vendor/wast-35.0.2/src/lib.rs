//! A crate for low-level parsing of the WebAssembly text formats: WAT and WAST.
//!
//! This crate is intended to be a low-level detail of the `wat` crate,
//! providing a low-level parsing API for parsing WebAssembly text format
//! structures. The API provided by this crate is very similar to
//! [`syn`](https://docs.rs/syn) and provides the ability to write customized
//! parsers which may be an extension to the core WebAssembly text format. For
//! more documentation see the [`parser`] module.
//!
//! # High-level Overview
//!
//! This crate provides a few major pieces of functionality
//!
//! * [`lexer`] - this is a raw lexer for the wasm text format. This is not
//!   customizable, but if you'd like to iterate over raw tokens this is the
//!   module for you. You likely won't use this much.
//!
//! * [`parser`] - this is the workhorse of this crate. The [`parser`] module
//!   provides the [`Parse`][] trait primarily and utilities
//!   around working with a [`Parser`](`parser::Parser`) to parse streams of
//!   tokens.
//!
//! * [`Module`] - this contains an Abstract Syntax Tree (AST) of the
//!   WebAssembly Text format (WAT) as well as the unofficial WAST format. This
//!   also has a [`Module::encode`] method to emit a module in its binary form.
//!
//! # Stability and WebAssembly Features
//!
//! This crate provides support for many in-progress WebAssembly features such
//! as reference types, multi-value, etc. Be sure to check out the documentation
//! of the [`wast` crate](https://docs.rs/wast) for policy information on crate
//! stability vs WebAssembly Features. The tl;dr; version is that this crate
//! will issue semver-non-breaking releases which will break the parsing of the
//! text format. This crate, unlike `wast`, is expected to have numerous Rust
//! public API changes, all of which will be accompanied with a semver-breaking
//! release.
//!
//! # Compile-time Cargo features
//!
//! This crate has a `wasm-module` feature which is turned on by default which
//! includes all necessary support to parse full WebAssembly modules. If you
//! don't need this (for example you're parsing your own s-expression format)
//! then this feature can be disabled.
//!
//! [`Parse`]: parser::Parse
//! [`LexError`]: lexer::LexError

#![deny(missing_docs, broken_intra_doc_links)]

use std::fmt;
use std::path::{Path, PathBuf};

#[cfg(feature = "wasm-module")]
mod binary;
#[cfg(feature = "wasm-module")]
mod resolve;

mod ast;
pub use self::ast::*;

pub mod lexer;
pub mod parser;

/// A convenience error type to tie together all the detailed errors produced by
/// this crate.
///
/// This type can be created from a [`lexer::LexError`] or [`parser::Error`].
/// This also contains storage for file/text information so a nice error can be
/// rendered along the same lines of rustc's own error messages (minus the
/// color).
///
/// This type is typically suitable for use in public APIs for consumers of this
/// crate.
#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorInner>,
}

#[derive(Debug)]
struct ErrorInner {
    text: Option<Text>,
    file: Option<PathBuf>,
    span: Span,
    kind: ErrorKind,
}

#[derive(Debug)]
struct Text {
    line: usize,
    col: usize,
    snippet: String,
}

#[derive(Debug)]
enum ErrorKind {
    Lex(lexer::LexError),
    Custom(String),
}

impl Error {
    fn lex(span: Span, content: &str, kind: lexer::LexError) -> Error {
        let mut ret = Error {
            inner: Box::new(ErrorInner {
                text: None,
                file: None,
                span,
                kind: ErrorKind::Lex(kind),
            }),
        };
        ret.set_text(content);
        return ret;
    }

    fn parse(span: Span, content: &str, message: String) -> Error {
        let mut ret = Error {
            inner: Box::new(ErrorInner {
                text: None,
                file: None,
                span,
                kind: ErrorKind::Custom(message),
            }),
        };
        ret.set_text(content);
        return ret;
    }

    /// Creates a new error with the given `message` which is targeted at the
    /// given `span`
    ///
    /// Note that you'll want to ensure that `set_text` or `set_path` is called
    /// on the resulting error to improve the rendering of the error message.
    pub fn new(span: Span, message: String) -> Error {
        Error {
            inner: Box::new(ErrorInner {
                text: None,
                file: None,
                span,
                kind: ErrorKind::Custom(message),
            }),
        }
    }

    /// Return the `Span` for this error.
    pub fn span(&self) -> Span {
        self.inner.span
    }

    /// To provide a more useful error this function can be used to extract
    /// relevant textual information about this error into the error itself.
    ///
    /// The `contents` here should be the full text of the original file being
    /// parsed, and this will extract a sub-slice as necessary to render in the
    /// `Display` implementation later on.
    pub fn set_text(&mut self, contents: &str) {
        if self.inner.text.is_some() {
            return;
        }
        self.inner.text = Some(Text::new(contents, self.inner.span));
    }

    /// To provide a more useful error this function can be used to set
    /// the file name that this error is associated with.
    ///
    /// The `path` here will be stored in this error and later rendered in the
    /// `Display` implementation.
    pub fn set_path(&mut self, path: &Path) {
        if self.inner.file.is_some() {
            return;
        }
        self.inner.file = Some(path.to_path_buf());
    }

    /// Returns the underlying `LexError`, if any, that describes this error.
    pub fn lex_error(&self) -> Option<&lexer::LexError> {
        match &self.inner.kind {
            ErrorKind::Lex(e) => Some(e),
            _ => None,
        }
    }

    /// Returns the underlying message, if any, that describes this error.
    pub fn message(&self) -> String {
        match &self.inner.kind {
            ErrorKind::Lex(e) => e.to_string(),
            ErrorKind::Custom(e) => e.clone(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err = match &self.inner.kind {
            ErrorKind::Lex(e) => e as &dyn fmt::Display,
            ErrorKind::Custom(e) => e as &dyn fmt::Display,
        };
        let text = match &self.inner.text {
            Some(text) => text,
            None => {
                return write!(f, "{} at byte offset {}", err, self.inner.span.offset);
            }
        };
        let file = self
            .inner
            .file
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("<anon>");
        write!(
            f,
            "\
{err}
     --> {file}:{line}:{col}
      |
 {line:4} | {text}
      | {marker:>0$}",
            text.col + 1,
            file = file,
            line = text.line + 1,
            col = text.col + 1,
            err = err,
            text = text.snippet,
            marker = "^",
        )
    }
}

impl std::error::Error for Error {}

impl Text {
    fn new(content: &str, span: Span) -> Text {
        let (line, col) = span.linecol_in(content);
        let snippet = content.lines().nth(line).unwrap_or("").to_string();
        Text { line, col, snippet }
    }
}
