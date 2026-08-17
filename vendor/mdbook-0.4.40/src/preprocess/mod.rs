//! Book preprocessing.

pub use self::cmd::CmdPreprocessor;
pub use self::index::IndexPreprocessor;
pub use self::links::LinkPreprocessor;

mod cmd;
mod index;
mod links;

use crate::book::Book;
use crate::config::Config;
use crate::errors::*;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

/// Extra information for a `Preprocessor` to give them more context when
/// processing a book.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreprocessorContext {
    /// The location of the book directory on disk.
    pub root: PathBuf,
    /// The book configuration (`book.toml`).
    pub config: Config,
    /// The `Renderer` this preprocessor is being used with.
    pub renderer: String,
    /// The calling `mdbook` version.
    pub mdbook_version: String,
    #[serde(skip)]
    pub(crate) chapter_titles: RefCell<HashMap<PathBuf, String>>,
    #[serde(skip)]
    __non_exhaustive: (),
}

impl PreprocessorContext {
    /// Create a new `PreprocessorContext`.
    pub(crate) fn new(root: PathBuf, config: Config, renderer: String) -> Self {
        PreprocessorContext {
            root,
            config,
            renderer,
            mdbook_version: crate::MDBOOK_VERSION.to_string(),
            chapter_titles: RefCell::new(HashMap::new()),
            __non_exhaustive: (),
        }
    }
}

/// An operation which is run immediately after loading a book into memory and
/// before it gets rendered.
pub trait Preprocessor {
    /// Get the `Preprocessor`'s name.
    fn name(&self) -> &str;

    /// Run this `Preprocessor`, allowing it to update the book before it is
    /// given to a renderer.
    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book>;

    /// A hint to `MDBook` whether this preprocessor is compatible with a
    /// particular renderer.
    ///
    /// By default, always returns `true`.
    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
