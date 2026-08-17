#![forbid(unsafe_code)]
#[cfg(feature = "compression")]
#[cfg_attr(feature = "compression", doc(hidden))]
pub use include_flate::flate;

extern crate rust_embed_impl;
pub use rust_embed_impl::*;

pub use rust_embed_utils::{EmbeddedFile, Metadata};

#[doc(hidden)]
pub extern crate rust_embed_utils as utils;

/// A directory of binary assets.
///
/// The files in the specified folder will be embedded into the executable in
/// release builds. Debug builds will read the data from the file system at
/// runtime.
///
/// This trait is meant to be derived like so:
/// ```
/// use rust_embed::Embed;
///
/// #[derive(Embed)]
/// #[folder = "examples/public/"]
/// struct Asset;
///
/// fn main() {}
/// ```
pub trait RustEmbed {
  /// Get an embedded file and its metadata.
  ///
  /// If the feature `debug-embed` is enabled or the binary was compiled in
  /// release mode, the file information is embedded in the binary and the file
  /// data is returned as a `Cow::Borrowed(&'static [u8])`.
  ///
  /// Otherwise, the information is read from the file system on each call and
  /// the file data is returned as a `Cow::Owned(Vec<u8>)`.
  fn get(file_path: &str) -> Option<EmbeddedFile>;

  /// Iterates over the file paths in the folder.
  ///
  /// If the feature `debug-embed` is enabled or the binary is compiled in
  /// release mode, a static array containing the list of relative file paths
  /// is used.
  ///
  /// Otherwise, the files are listed from the file system on each call.
  fn iter() -> impl Iterator<Item = std::borrow::Cow<'static, str>> + 'static;
}

pub use RustEmbed as Embed;

/// An iterator over filenames.
///
/// This enum exists for optimization purposes, to avoid boxing the iterator in
/// some cases. Do not try and match on it, as different variants will exist
/// depending on the compilation context.
pub enum Filenames {
  /// Release builds use a named iterator type, which can be stack-allocated.
  #[cfg(any(not(debug_assertions), feature = "debug-embed"))]
  Embedded(std::slice::Iter<'static, &'static str>),

  /// The debug iterator type is currently unnameable and still needs to be
  /// boxed.
  #[cfg(all(debug_assertions, not(feature = "debug-embed")))]
  Dynamic(Box<dyn Iterator<Item = std::borrow::Cow<'static, str>>>),
}

impl Iterator for Filenames {
  type Item = std::borrow::Cow<'static, str>;
  fn next(&mut self) -> Option<Self::Item> {
    match self {
      #[cfg(any(not(debug_assertions), feature = "debug-embed"))]
      Filenames::Embedded(names) => names.next().map(|x| std::borrow::Cow::from(*x)),

      #[cfg(all(debug_assertions, not(feature = "debug-embed")))]
      Filenames::Dynamic(boxed) => boxed.next(),
    }
  }
}
