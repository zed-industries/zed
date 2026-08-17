//! Cross-platform path manipulation.
//!
//! This module is an async version of [`std::path`].
//!
//! This module provides two types, [`PathBuf`] and [`Path`][`Path`] (akin to [`String`]
//! and [`str`]), for working with paths abstractly. These types are thin wrappers
//! around [`OsString`] and [`OsStr`] respectively, meaning that they work directly
//! on strings according to the local platform's path syntax.
//!
//! Paths can be parsed into [`Component`]s by iterating over the structure
//! returned by the [`components`] method on [`Path`]. [`Component`]s roughly
//! correspond to the substrings between path separators (`/` or `\`). You can
//! reconstruct an equivalent path from components with the [`push`] method on
//! [`PathBuf`]; note that the paths may differ syntactically by the
//! normalization described in the documentation for the [`components`] method.
//!
//! [`std::path`]: https://doc.rust-lang.org/std/path/index.html
//!
//! ## Simple usage
//!
//! Path manipulation includes both parsing components from slices and building
//! new owned paths.
//!
//! To parse a path, you can create a [`Path`] slice from a [`str`]
//! slice and start asking questions:
//!
//! ```
//! use async_std::path::Path;
//! use std::ffi::OsStr;
//!
//! let path = Path::new("/tmp/foo/bar.txt");
//!
//! let parent = path.parent();
//! assert_eq!(parent, Some(Path::new("/tmp/foo")));
//!
//! let file_stem = path.file_stem();
//! assert_eq!(file_stem, Some(OsStr::new("bar")));
//!
//! let extension = path.extension();
//! assert_eq!(extension, Some(OsStr::new("txt")));
//! ```
//!
//! To build or modify paths, use [`PathBuf`]:
//!
//! ```
//! use async_std::path::PathBuf;
//!
//! // This way works...
//! let mut path = PathBuf::from("c:\\");
//!
//! path.push("windows");
//! path.push("system32");
//!
//! path.set_extension("dll");
//!
//! // ... but push is best used if you don't know everything up
//! // front. If you do, this way is better:
//! let path: PathBuf = ["c:\\", "windows", "system32.dll"].iter().collect();
//! ```
//!
//! [`Component`]: enum.Component.html
//! [`components`]: struct.Path.html#method.components
//! [`PathBuf`]: struct.PathBuf.html
//! [`Path`]: struct.Path.html
//! [`push`]: struct.PathBuf.html#method.push
//! [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
//!
//! [`str`]: https://doc.rust-lang.org/std/primitive.str.html
//! [`OsString`]: https://doc.rust-lang.org/std/ffi/struct.OsString.html
//! [`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html

mod ancestors;
mod components;
mod iter;
mod path;
mod pathbuf;

#[doc(inline)]
pub use std::path::{
    is_separator, Component, Display, Prefix, PrefixComponent, StripPrefixError, MAIN_SEPARATOR,
};

pub use ancestors::Ancestors;
pub use components::Components;
pub use iter::Iter;
pub use path::Path;
pub use pathbuf::PathBuf;
