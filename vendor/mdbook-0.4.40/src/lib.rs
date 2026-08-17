//! # mdBook
//!
//! **mdBook** is a tool for rendering a collection of markdown documents into
//! a form more suitable for end users like HTML or EPUB.  It offers a command
//! line interface, but this crate can be used if more control is required.
//!
//! This is the API doc, the [user guide] is also available if you want
//! information about the command line tool, format, structure etc. It is also
//! rendered with mdBook to showcase the features and default theme.
//!
//! Some reasons why you would want to use the crate (over the cli):
//!
//! - Integrate mdbook in a current project
//! - Extend the capabilities of mdBook
//! - Do some processing or test before building your book
//! - Accessing the public API to help create a new Renderer
//! - ...
//!
//! > **Note:** While we try to ensure `mdbook`'s command-line interface and
//! > behaviour are backwards compatible, the tool's internals are still
//! > evolving and being iterated on. If you wish to prevent accidental
//! > breakages it is recommended to pin any tools building on top of the
//! > `mdbook` crate to a specific release.
//!
//! # Examples
//!
//! If creating a new book from scratch, you'll want to get a `BookBuilder` via
//! the `MDBook::init()` method.
//!
//! ```rust,no_run
//! use mdbook::MDBook;
//! use mdbook::config::Config;
//!
//! let root_dir = "/path/to/book/root";
//!
//! // create a default config and change a couple things
//! let mut cfg = Config::default();
//! cfg.book.title = Some("My Book".to_string());
//! cfg.book.authors.push("Michael-F-Bryan".to_string());
//!
//! MDBook::init(root_dir)
//!     .create_gitignore(true)
//!     .with_config(cfg)
//!     .build()
//!     .expect("Book generation failed");
//! ```
//!
//! You can also load an existing book and build it.
//!
//! ```rust,no_run
//! use mdbook::MDBook;
//!
//! let root_dir = "/path/to/book/root";
//!
//! let mut md = MDBook::load(root_dir)
//!     .expect("Unable to load the book");
//! md.build().expect("Building failed");
//! ```
//!
//! ## Implementing a new Backend
//!
//! `mdbook` has a fairly flexible mechanism for creating additional backends
//! for your book. The general idea is you'll add an extra table in the book's
//! `book.toml` which specifies an executable to be invoked by `mdbook`. This
//! executable will then be called during a build, with an in-memory
//! representation ([`RenderContext`]) of the book being passed to the
//! subprocess via `stdin`.
//!
//! The [`RenderContext`] gives the backend access to the contents of
//! `book.toml` and lets it know which directory all generated artefacts should
//! be placed in. For a much more in-depth explanation, consult the [relevant
//! chapter] in the *For Developers* section of the user guide.
//!
//! To make creating a backend easier, the `mdbook` crate can be imported
//! directly, making deserializing the `RenderContext` easy and giving you
//! access to the various methods for working with the [`Config`].
//!
//! [user guide]: https://rust-lang.github.io/mdBook/
//! [`RenderContext`]: renderer::RenderContext
//! [relevant chapter]: https://rust-lang.github.io/mdBook/for_developers/backends.html
//! [`Config`]: config::Config

#![deny(missing_docs)]
#![deny(rust_2018_idioms)]

pub mod book;
pub mod config;
pub mod preprocess;
pub mod renderer;
pub mod theme;
pub mod utils;

/// The current version of `mdbook`.
///
/// This is provided as a way for custom preprocessors and renderers to do
/// compatibility checks.
pub const MDBOOK_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use crate::book::BookItem;
pub use crate::book::MDBook;
pub use crate::config::Config;
pub use crate::renderer::Renderer;

/// The error types used through out this crate.
pub mod errors {
    pub(crate) use anyhow::{bail, ensure, Context};
    pub use anyhow::{Error, Result};
}
