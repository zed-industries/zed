//! Source file support for diagnostic reporting.
//!
//! The main trait defined in this module is the [`Files`] trait, which provides
//! provides the minimum amount of functionality required for printing [`Diagnostics`]
//! with the [`term::emit`] function.
//!
//! Simple implementations of this trait are implemented:
//!
//! - [`SimpleFile`]: For single-file use-cases
//! - [`SimpleFiles`]: For multi-file use-cases
//!
//! These data structures provide a pretty minimal API, however,
//! so end-users are encouraged to create their own implementations for their
//! own specific use-cases, such as an implementation that accesses the file
//! system directly (and caches the line start locations), or an implementation
//! using an incremental compilation library like [`salsa`].
//!
//! [`term::emit`]: crate::term::emit
//! [`Diagnostics`]: crate::diagnostic::Diagnostic
//! [`Files`]: Files
//! [`SimpleFile`]: SimpleFile
//! [`SimpleFiles`]: SimpleFiles
//!
//! [`salsa`]: https://crates.io/crates/salsa

use alloc::vec::Vec;
use core::ops::Range;

/// An enum representing an error that happened while looking up a file or a piece of content in that file.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A required file is not in the file database.
    FileMissing,
    /// The file is present, but does not contain the specified byte index.
    IndexTooLarge { given: usize, max: usize },
    /// The file is present, but does not contain the specified line index.
    LineTooLarge { given: usize, max: usize },
    /// The file is present and contains the specified line index, but the line does not contain
    /// the specified column index.
    ColumnTooLarge { given: usize, max: usize },
    /// The given index is contained in the file, but is not a boundary of a UTF-8 code point.
    InvalidCharBoundary { given: usize },
    /// There was a error while doing IO.
    #[cfg(feature = "std")]
    Io(std::io::Error),
    /// There was a error during formatting.
    FormatError,
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<core::fmt::Error> for Error {
    fn from(_err: core::fmt::Error) -> Error {
        Error::FormatError
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::FileMissing => write!(f, "file missing"),
            Error::IndexTooLarge { given, max } => {
                write!(f, "invalid index {}, maximum index is {}", given, max)
            }
            Error::LineTooLarge { given, max } => {
                write!(f, "invalid line {}, maximum line is {}", given, max)
            }
            Error::ColumnTooLarge { given, max } => {
                write!(f, "invalid column {}, maximum column {}", given, max)
            }
            Error::InvalidCharBoundary { .. } => write!(f, "index is not a code point boundary"),
            #[cfg(feature = "std")]
            Error::Io(err) => write!(f, "{}", err),
            Error::FormatError => write!(f, "formatting error"),
        }
    }
}

#[cfg(feature = "std")]
use std::error::Error as RustError;

#[cfg(not(feature = "std"))]
use core::error::Error as RustError;

impl RustError for Error {
    fn source(&self) -> Option<&(dyn RustError + 'static)> {
        match &self {
            #[cfg(feature = "std")]
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

/// A minimal interface for accessing source files when rendering diagnostics.
///
/// A lifetime parameter `'a` is provided to allow any of the returned values to returned by reference.
/// This is to workaround the lack of higher kinded lifetime parameters.
/// This can be ignored if this is not needed, however.
pub trait Files<'a> {
    /// A unique identifier for files in the file provider. This will be used
    /// for rendering `diagnostic::Label`s in the corresponding source files.
    type FileId: 'a + Copy + PartialEq;
    /// The user-facing name of a file, to be displayed in diagnostics.
    type Name: 'a + core::fmt::Display;
    /// The source code of a file.
    type Source: 'a + AsRef<str>;

    /// The user-facing name of a file.
    fn name(&'a self, id: Self::FileId) -> Result<Self::Name, Error>;

    /// The source code of a file.
    fn source(&'a self, id: Self::FileId) -> Result<Self::Source, Error>;

    /// The index of the line at the given byte index.
    /// If the byte index is past the end of the file, returns the maximum line index in the file.
    /// This means that this function only fails if the file is not present.
    ///
    /// # Note for trait implementors
    ///
    /// This can be implemented efficiently by performing a binary search over
    /// a list of line starts that was computed by calling the [`line_starts`]
    /// function that is exported from the [`files`] module. It might be useful
    /// to pre-compute and cache these line starts.
    ///
    /// [`line_starts`]: crate::files::line_starts
    /// [`files`]: crate::files
    fn line_index(&'a self, id: Self::FileId, byte_index: usize) -> Result<usize, Error>;

    /// The user-facing line number at the given line index.
    /// It is not necessarily checked that the specified line index
    /// is actually in the file.
    ///
    /// # Note for trait implementors
    ///
    /// This is usually 1-indexed from the beginning of the file, but
    /// can be useful for implementing something like the
    /// [C preprocessor's `#line` macro][line-macro].
    ///
    /// [line-macro]: https://en.cppreference.com/w/c/preprocessor/line
    #[allow(unused_variables)]
    fn line_number(&'a self, id: Self::FileId, line_index: usize) -> Result<usize, Error> {
        Ok(line_index + 1)
    }

    /// The user-facing column number at the given line index and byte index.
    ///
    /// # Note for trait implementors
    ///
    /// This is usually 1-indexed from the the start of the line.
    /// A default implementation is provided, based on the [`column_index`]
    /// function that is exported from the [`files`] module.
    ///
    /// [`files`]: crate::files
    /// [`column_index`]: crate::files::column_index
    fn column_number(
        &'a self,
        id: Self::FileId,
        line_index: usize,
        byte_index: usize,
    ) -> Result<usize, Error> {
        let source = self.source(id)?;
        let line_range = self.line_range(id, line_index)?;
        let column_index = column_index(source.as_ref(), line_range, byte_index);

        Ok(column_index + 1)
    }

    /// Convenience method for returning line and column number at the given
    /// byte index in the file.
    fn location(&'a self, id: Self::FileId, byte_index: usize) -> Result<Location, Error> {
        let line_index = self.line_index(id, byte_index)?;

        Ok(Location {
            line_number: self.line_number(id, line_index)?,
            column_number: self.column_number(id, line_index, byte_index)?,
        })
    }

    /// The byte range of line in the source of the file.
    fn line_range(&'a self, id: Self::FileId, line_index: usize) -> Result<Range<usize>, Error>;
}

/// A user-facing location in a source file.
///
/// Returned by [`Files::location`].
///
/// [`Files::location`]: Files::location
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    /// The user-facing line number.
    pub line_number: usize,
    /// The user-facing column number.
    pub column_number: usize,
}

/// The column index at the given byte index in the source file.
/// This is the number of characters to the given byte index.
///
/// If the byte index is smaller than the start of the line, then `0` is returned.
/// If the byte index is past the end of the line, the column index of the last
/// character `+ 1` is returned.
///
/// # Example
///
/// ```rust
/// use codespan_reporting::files;
///
/// let source = "\n\n🗻∈🌏\n\n";
///
/// assert_eq!(files::column_index(source, 0..1, 0), 0);
/// assert_eq!(files::column_index(source, 2..13, 0), 0);
/// assert_eq!(files::column_index(source, 2..13, 2 + 0), 0);
/// assert_eq!(files::column_index(source, 2..13, 2 + 1), 0);
/// assert_eq!(files::column_index(source, 2..13, 2 + 4), 1);
/// assert_eq!(files::column_index(source, 2..13, 2 + 8), 2);
/// assert_eq!(files::column_index(source, 2..13, 2 + 10), 2);
/// assert_eq!(files::column_index(source, 2..13, 2 + 11), 3);
/// assert_eq!(files::column_index(source, 2..13, 2 + 12), 3);
/// ```
pub fn column_index(source: &str, line_range: Range<usize>, byte_index: usize) -> usize {
    let end_index = core::cmp::min(byte_index, core::cmp::min(line_range.end, source.len()));

    (line_range.start..end_index)
        .filter(|byte_index| source.is_char_boundary(byte_index + 1))
        .count()
}

/// Return the starting byte index of each line in the source string.
///
/// This can make it easier to implement [`Files::line_index`] by allowing
/// implementors of [`Files`] to pre-compute the line starts, then search for
/// the corresponding line range, as shown in the example below.
///
/// [`Files`]: Files
/// [`Files::line_index`]: Files::line_index
///
/// # Example
///
/// ```rust
/// use codespan_reporting::files;
///
/// let source = "foo\nbar\r\n\nbaz";
/// let line_starts: Vec<_> = files::line_starts(source).collect();
///
/// assert_eq!(
///     line_starts,
///     [
///         0,  // "foo\n"
///         4,  // "bar\r\n"
///         9,  // ""
///         10, // "baz"
///     ],
/// );
///
/// fn line_index(line_starts: &[usize], byte_index: usize) -> Option<usize> {
///     match line_starts.binary_search(&byte_index) {
///         Ok(line) => Some(line),
///         Err(next_line) => Some(next_line - 1),
///     }
/// }
///
/// assert_eq!(line_index(&line_starts, 5), Some(1));
/// ```
// NOTE: this is copied in `codespan::file::line_starts` and should be kept in sync.
pub fn line_starts(source: &str) -> impl '_ + Iterator<Item = usize> {
    core::iter::once(0).chain(source.match_indices('\n').map(|(i, _)| i + 1))
}

/// A file database that contains a single source file.
///
/// Because there is only single file in this database we use `()` as a [`FileId`].
///
/// This is useful for simple language tests, but it might be worth creating a
/// custom implementation when a language scales beyond a certain size.
///
/// [`FileId`]: Files::FileId
#[derive(Debug, Clone)]
pub struct SimpleFile<Name, Source> {
    /// The name of the file.
    name: Name,
    /// The source code of the file.
    source: Source,
    /// The starting byte indices in the source code.
    line_starts: Vec<usize>,
}

impl<Name, Source> SimpleFile<Name, Source>
where
    Name: core::fmt::Display,
    Source: AsRef<str>,
{
    /// Create a new source file.
    pub fn new(name: Name, source: Source) -> SimpleFile<Name, Source> {
        SimpleFile {
            name,
            line_starts: line_starts(source.as_ref()).collect(),
            source,
        }
    }

    /// Return the name of the file.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Return the source of the file.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Return the starting byte index of the line with the specified line index.
    /// Convenience method that already generates errors if necessary.
    fn line_start(&self, line_index: usize) -> Result<usize, Error> {
        use core::cmp::Ordering;

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(self
                .line_starts
                .get(line_index)
                .copied()
                .expect("failed despite previous check")),
            Ordering::Equal => Ok(self.source.as_ref().len()),
            Ordering::Greater => Err(Error::LineTooLarge {
                given: line_index,
                max: self.line_starts.len() - 1,
            }),
        }
    }
}

impl<'a, Name, Source> Files<'a> for SimpleFile<Name, Source>
where
    Name: 'a + core::fmt::Display + Clone,
    Source: 'a + AsRef<str>,
{
    type FileId = ();
    type Name = Name;
    type Source = &'a str;

    fn name(&self, (): ()) -> Result<Name, Error> {
        Ok(self.name.clone())
    }

    fn source(&self, (): ()) -> Result<&str, Error> {
        Ok(self.source.as_ref())
    }

    fn line_index(&self, (): (), byte_index: usize) -> Result<usize, Error> {
        Ok(self
            .line_starts
            .binary_search(&byte_index)
            .unwrap_or_else(|next_line| next_line - 1))
    }

    fn line_range(&self, (): (), line_index: usize) -> Result<Range<usize>, Error> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }
}

/// A file database that can store multiple source files.
///
/// This is useful for simple language tests, but it might be worth creating a
/// custom implementation when a language scales beyond a certain size.
/// It is a glorified `Vec<SimpleFile>` that implements the `Files` trait.
#[derive(Debug, Default, Clone)]
pub struct SimpleFiles<Name, Source> {
    files: Vec<SimpleFile<Name, Source>>,
}

impl<Name, Source> SimpleFiles<Name, Source>
where
    Name: core::fmt::Display,
    Source: AsRef<str>,
{
    /// Create a new files database.
    pub fn new() -> SimpleFiles<Name, Source> {
        SimpleFiles { files: Vec::new() }
    }

    /// Add a file to the database, returning the handle that can be used to
    /// refer to it again.
    pub fn add(&mut self, name: Name, source: Source) -> usize {
        let file_id = self.files.len();
        self.files.push(SimpleFile::new(name, source));
        file_id
    }

    /// Get the file corresponding to the given id.
    pub fn get(&self, file_id: usize) -> Result<&SimpleFile<Name, Source>, Error> {
        self.files.get(file_id).ok_or(Error::FileMissing)
    }
}

impl<'a, Name, Source> Files<'a> for SimpleFiles<Name, Source>
where
    Name: 'a + core::fmt::Display + Clone,
    Source: 'a + AsRef<str>,
{
    type FileId = usize;
    type Name = Name;
    type Source = &'a str;

    fn name(&self, file_id: usize) -> Result<Name, Error> {
        Ok(self.get(file_id)?.name().clone())
    }

    fn source(&self, file_id: usize) -> Result<&str, Error> {
        Ok(self.get(file_id)?.source().as_ref())
    }

    fn line_index(&self, file_id: usize, byte_index: usize) -> Result<usize, Error> {
        self.get(file_id)?.line_index((), byte_index)
    }

    fn line_range(&self, file_id: usize, line_index: usize) -> Result<Range<usize>, Error> {
        self.get(file_id)?.line_range((), line_index)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_SOURCE: &str = "foo\nbar\r\n\nbaz";

    #[test]
    fn line_starts() {
        let file = SimpleFile::new("test", TEST_SOURCE);

        assert_eq!(
            file.line_starts,
            [
                0,  // "foo\n"
                4,  // "bar\r\n"
                9,  // ""
                10, // "baz"
            ],
        );
    }

    #[test]
    fn line_span_sources() {
        let file = SimpleFile::new("test", TEST_SOURCE);

        let line_sources = (0..4)
            .map(|line| {
                let line_range = file.line_range((), line).unwrap();
                &file.source[line_range]
            })
            .collect::<Vec<_>>();

        assert_eq!(line_sources, ["foo\n", "bar\r\n", "\n", "baz"]);
    }
}
