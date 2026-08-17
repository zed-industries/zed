#![doc = include_str!("../rustdoc.md")]
#![deny(clippy::as_conversions)]
#![deny(clippy::enum_glob_use)]
#![deny(clippy::wildcard_imports)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::private_intra_doc_links)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)] // reason = "False positives on EditorConfig".
#![allow(clippy::module_name_repetitions)] // reason = "Affects re-exports from private modules."
#![allow(clippy::must_use_candidate)] // reason = "Too pedantic."
#![allow(clippy::semicolon_if_nothing_returned)] // reason = "Too pedantic."
#![allow(clippy::let_underscore_untyped)] // reason = "Too pedantic."
#![allow(clippy::missing_errors_doc)] // reason = "TODO: Fix."
#![cfg_attr(doc_unstable, feature(doc_auto_cfg))]

mod error;
mod fallback;
mod file;
mod glob;
mod linereader;
mod parser;
mod properties;
pub mod property;
pub mod rawvalue;
mod section;
#[cfg(test)]
mod tests;
mod traits;
pub mod version;

pub use error::{Error, ParseError};
pub use file::{ConfigFile, ConfigFiles};
pub use parser::ConfigParser;
pub use properties::{Properties, PropertiesSource};
pub use section::Section;
pub use traits::*;

#[cfg(feature = "language-tags")]
pub mod language_tags {
    //! Re-export of the `language-tags` crate.
    pub use ::language_tags::*;
}

/// Retrieves the [`Properties`] for a file at the given path.
///
/// This is the simplest way to use this library in an EditorConfig integration or plugin.
///
/// This function does not canonicalize the path,
/// but will join relative paths onto the current working directory.
///
/// EditorConfig files are assumed to be named `.editorconfig`.
/// If not, use [`properties_from_config_of`]
pub fn properties_of(path: impl AsRef<std::path::Path>) -> Result<Properties, Error> {
    properties_from_config_of(path, Option::<&std::path::Path>::None)
}

/// Retrieves the [`Properties`] for a file at the given path,
/// expecting EditorConfig files to be named matching `config_path_override`.
///
/// This function does not canonicalize the path,
/// but will join relative paths onto the current working directory.
///
/// If the provided config path is absolute, uses the EditorConfig file at that path.
/// If it's relative, joins it onto every ancestor of the target file,
/// and looks for config files at those paths.
/// If it's `None`, EditorConfig files are assumed to be named `.editorconfig`.
pub fn properties_from_config_of(
    target_path: impl AsRef<std::path::Path>,
    config_path_override: Option<impl AsRef<std::path::Path>>,
) -> Result<Properties, Error> {
    let mut retval = Properties::new();
    ConfigFiles::open(&target_path, config_path_override)?.apply_to(&mut retval, &target_path)?;
    Ok(retval)
}
