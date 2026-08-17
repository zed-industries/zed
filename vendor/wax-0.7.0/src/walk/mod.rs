//! Traversal and matching of files and directory trees.
//!
//! This module provides APIs for walking directory trees and matching files in a directory tree
//! against [`Program`]s using [`Iterator`]s. These iterators implement [`FileIterator`], which
//! supports efficient filtering that can cancel traversal into sub-trees that are discarded by
//! combinators.
//!
//! # Examples
//!
//! To iterate over the files in a directory tree, use the [`PathExt`] trait.
//!
//! ```rust,no_run
//! use std::path::Path;
//! use wax::walk::{Entry, PathExt as _};
//!
//! let root = Path::new(".config");
//! for entry in root.walk() {
//!     let entry = entry.unwrap();
//!     println!("{:?}", entry.path());
//! }
//! ```
//!
//! To match a [`Glob`] against a directory tree, use [`Glob::walk`]. This function constructs an
//! iterator that efficiently matches a [`Glob`] against the paths of files read from a directory
//! tree.
//!
//! ```rust,no_run
//! use wax::Glob;
//! use wax::walk::Entry;
//!
//! let glob = Glob::new("**/src/**").unwrap();
//! for entry in glob.walk("projects") {
//!     let entry = entry.unwrap();
//!     println!("{:?}", entry.path());
//! }
//! ```
//!
//! Any [`FileIterator`] (the iterators constructed by [`Glob::walk`], [`PathExt::walk`], etc.) can
//! be efficiently filtered. This filtering can cancel traversal into sub-trees that are discarded.
//! To filter files using [`Program`]s, use the [`not`] combinator.
//!
//! ```rust,no_run
//! use std::path::Path;
//! use wax::walk::{Entry, FileIterator, PathExt as _};
//!
//! let root = Path::new(".config");
//! for entry in root.walk().not("**/*.xml").unwrap() {
//!     let entry = entry.unwrap();
//!     println!("{:?}", entry.path());
//! }
//! ```
//!
//! More arbitrary (non-nominal) filtering is also possible via the [`filter_entry`] combinator.
//!
//! [`FileIterator`]: crate::walk::FileIterator
//! [`filter_entry`]: crate::walk::FileIterator::filter_entry
//! [`Glob`]: crate::Glob
//! [`Glob::walk`]: crate::Glob::walk
//! [`Iterator`]: std::iter::Iterator
//! [`not`]: crate::walk::FileIterator::not
//! [`PathExt`]: crate::walk::PathExt
//! [`PathExt::walk`]: crate::walk::PathExt::walk
//! [`Program`]: crate::Program

#![cfg(feature = "walk")]
#![cfg_attr(docsrs, doc(cfg(feature = "walk")))]

mod behavior;
mod glob;

use std::fs::{FileType, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::{self, DirEntry, WalkDir};

use crate::filter::{
    self, CancelWalk, HierarchicalIterator, Isomeric, SeparatingFilter, SeparatingFilterInput,
    Separation, TreeResidue, WalkCancellation,
};
use crate::walk::glob::FilterAny;
use crate::{BuildError, Pattern};

pub use crate::walk::behavior::{
    DepthBehavior, DepthMax, DepthMin, DepthMinMax, LinkBehavior, WalkBehavior,
};
pub use crate::walk::glob::GlobEntry;

type FileFiltrate<T> = Result<T, WalkError>;
type FileResidue<R> = TreeResidue<R>;
type FileFeed<T, R> = (FileFiltrate<T>, FileResidue<R>);

impl<T, R> Isomeric for (T, FileResidue<R>)
where
    T: Entry,
    R: Entry,
{
    // TODO: Using a trait object here is very flexible, but incurs a slight performance penalty.
    //       At time of writing, there are no public APIs that allow mapping of the entry types of
    //       separating filters, so this flexibility may not be worth its cost. The alternative is
    //       to use `TreeEntry` as the substituent and require that `T` is `AsRef<TreeEntry>` or
    //       similar. This does not require dynamic dispatch, but places more restrictive
    //       constraints on entry types. Revisit this.
    type Substituent<'a>
        = &'a dyn Entry
    where
        Self: 'a;

    fn substituent(separation: &Separation<Self>) -> Self::Substituent<'_> {
        match separation {
            Separation::Filtrate(filtrate) => filtrate.get(),
            Separation::Residue(residue) => residue.get().get(),
        }
    }
}

trait SplitAtDepth {
    fn split_at_depth(&self, depth: usize) -> (&Path, &Path);
}

impl SplitAtDepth for Path {
    fn split_at_depth(&self, depth: usize) -> (&Path, &Path) {
        let ancestor = self.ancestors().nth(depth).unwrap_or(Path::new(""));
        let descendant = self.strip_prefix(ancestor).unwrap();
        (ancestor, descendant)
    }
}

trait JoinAndGetDepth {
    fn join_and_get_depth(&self, path: impl AsRef<Path>) -> (PathBuf, usize);
}

impl JoinAndGetDepth for Path {
    fn join_and_get_depth(&self, path: impl AsRef<Path>) -> (PathBuf, usize) {
        let path = path.as_ref();
        let joined = self.join(path);
        let depth = joined.components().count();
        let depth = if path.is_absolute() {
            // If `path` is absolute, then it replaces `self` (`joined` and `path` are the same).
            // In this case, the depth of the join is the depth of `joined` (there is no root
            // sub-path).
            depth
                .checked_add(1)
                .expect("overflow determining join depth")
        }
        else {
            depth.saturating_sub(self.components().count())
        };
        (joined, depth)
    }
}

/// Describes errors that occur when walking a directory tree.
///
/// `WalkError` implements conversion into [`io::Error`].
///
/// [`io::Error`]: std::io::Error
#[derive(Debug, Error)]
#[error("failed to match directory tree: {kind}")]
pub struct WalkError {
    depth: usize,
    kind: WalkErrorKind,
}

impl WalkError {
    /// Gets the path at which the error occurred, if any.
    ///
    /// Returns `None` if there is no path associated with the error.
    pub fn path(&self) -> Option<&Path> {
        self.kind.path()
    }

    /// Gets the depth at which the error occurred from the root directory of the traversal.
    ///
    /// **This depth may differ from the depth reported by [`Entry::depth`]** when matching a pattern
    /// against a directory tree.
    ///
    /// [`Entry::depth`]: crate::walk::Entry::depth
    pub fn depth(&self) -> usize {
        self.depth
    }
}

impl From<walkdir::Error> for WalkError {
    fn from(error: walkdir::Error) -> Self {
        let depth = error.depth();
        let path = error.path().map(From::from);
        if error.io_error().is_some() {
            WalkError {
                depth,
                kind: WalkErrorKind::Io {
                    path,
                    error: error.into_io_error().expect("incongruent error kind"),
                },
            }
        }
        else {
            WalkError {
                depth,
                kind: WalkErrorKind::LinkCycle {
                    root: error
                        .loop_ancestor()
                        .expect("incongruent error kind")
                        .into(),
                    leaf: path.expect("incongruent error kind"),
                },
            }
        }
    }
}

impl From<WalkError> for io::Error {
    fn from(error: WalkError) -> Self {
        let kind = match &error.kind {
            WalkErrorKind::Io { error, .. } => error.kind(),
            _ => io::ErrorKind::Other,
        };
        io::Error::new(kind, error)
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
enum WalkErrorKind {
    #[error("failed to read file at `{path:?}`: {error}")]
    Io {
        path: Option<PathBuf>,
        error: io::Error,
    },
    #[error("symbolic link cycle detected from `{root}` to `{leaf}`")]
    LinkCycle { root: PathBuf, leaf: PathBuf },
}

impl WalkErrorKind {
    pub fn path(&self) -> Option<&Path> {
        match self {
            WalkErrorKind::Io { path, .. } => path.as_ref().map(PathBuf::as_ref),
            WalkErrorKind::LinkCycle { leaf, .. } => Some(leaf.as_ref()),
        }
    }
}

/// Functions for walking a directory tree at a [`Path`].
///
/// [`Path`]: std::path::Path
pub trait PathExt {
    /// Gets an iterator over files in the directory tree at the path.
    ///
    /// If the path refers to a regular file, then only its path is yielded by the iterator.
    ///
    /// This function uses the default [`WalkBehavior`]. To configure the behavior of the
    /// traversal, see [`PathExt::walk_with_behavior`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::Path;
    /// use wax::walk::{Entry, PathExt};
    ///
    /// for entry in Path::new(".").walk() {
    ///     let entry = entry.unwrap();
    ///     println!("{:?}", entry.path());
    /// }
    /// ```
    ///
    /// [`PathExt::walk_with_behavior`]: crate::walk::PathExt::walk_with_behavior
    /// [`WalkBehavior`]: crate::walk::WalkBehavior
    fn walk(&self) -> WalkTree {
        self.walk_with_behavior(WalkBehavior::default())
    }

    /// Gets an iterator over files in the directory tree at the path.
    ///
    /// This function is the same as [`PathExt::walk`], but it additionally accepts a
    /// [`WalkBehavior`] that configures how the traversal interacts with symbolic links, bounds on
    /// depth, etc.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::Path;
    /// use wax::walk::{Entry, LinkBehavior, PathExt};
    ///
    /// // Read the target of symbolic links (follow links).
    /// for entry in Path::new("/home").walk_with_behavior(LinkBehavior::ReadTarget) {
    ///     let entry = entry.unwrap();
    ///     println!("{:?}", entry.path());
    /// }
    /// ```
    ///
    /// [`PathExt::walk`]: crate::walk::PathExt::walk
    /// [`WalkBehavior`]: crate::walk::WalkBehavior
    fn walk_with_behavior(&self, behavior: impl Into<WalkBehavior>) -> WalkTree;
}

impl PathExt for Path {
    fn walk_with_behavior(&self, behavior: impl Into<WalkBehavior>) -> WalkTree {
        WalkTree::with_behavior(self, behavior)
    }
}

/// Describes a file yielded from a [`FileIterator`].
///
/// [`FileIterator`]: crate::walk::FileIterator
pub trait Entry {
    /// Converts the entry into the path of the file.
    fn into_path(self) -> PathBuf
    where
        Self: Sized;

    /// Gets the path of the file.
    fn path(&self) -> &Path;

    /// Splits the path of the file into its root and relative segments, in that order.
    ///
    /// The root segment is the path from which the walk started. When walking a [`Path`] via
    /// functions in [`PathExt`], the root is always the same as the path itself. When walking a
    /// pattern like [`Glob`], the root segment differs depending on whether or not the pattern
    /// [has a root][`Program::has_root`]. If a pattern has a root, then the root segment is the
    /// invariant prefix in the pattern, otherwise the root segment is the path given to functions
    /// like [`Glob::walk`].
    ///
    /// The relative segment is the remainder (descendant) of the path of the file (relative to the
    /// root segment).
    ///
    /// [`Glob`]: crate::Glob
    /// [`Glob::walk`]: crate::Glob::walk
    /// [`Path`]: std::path::Path
    /// [`PathExt`]: crate::walk::PathExt
    /// [`Program::has_root`]: crate::Program::has_root
    fn root_relative_paths(&self) -> (&Path, &Path);

    /// Gets the [`Metadata`] of the file.
    ///
    /// This may require an additional read from the file system on some platforms.
    ///
    /// [`Metadata`]: std::fs::Metadata
    fn metadata(&self) -> Result<Metadata, WalkError>;

    /// Gets the type of the file (regular vs. directory).
    ///
    /// This information may be cached and so this function should be preferred over [`metadata`]
    /// if only the file type is needed.
    ///
    /// [`metadata`]: crate::walk::Entry::metadata
    fn file_type(&self) -> FileType;

    /// Gets the depth of the path of the file from the root segment.
    ///
    /// See [`root_relative_paths`].
    ///
    /// [`root_relative_paths`]: crate::walk::Entry::root_relative_paths
    fn depth(&self) -> usize;
}

/// Describes a file yielded from a [`WalkTree`] iterator.
///
/// [`WalkTree`]: crate::walk::WalkTree
#[derive(Clone, Debug)]
pub struct TreeEntry {
    entry: DirEntry,
}

impl Entry for TreeEntry {
    fn into_path(self) -> PathBuf {
        self.entry.into_path()
    }

    fn path(&self) -> &Path {
        self.entry.path()
    }

    fn root_relative_paths(&self) -> (&Path, &Path) {
        self.path().split_at_depth(self.depth())
    }

    fn metadata(&self) -> Result<Metadata, WalkError> {
        self.entry.metadata().map_err(From::from)
    }

    fn file_type(&self) -> FileType {
        self.entry.file_type()
    }

    fn depth(&self) -> usize {
        self.entry.depth()
    }
}

/// A [`FileIterator`] over files in a directory tree.
///
/// This iterator is constructed from [`Path`]s via extension functions in [`PathExt`], such as
/// [`PathExt::walk`].
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use wax::walk::{Entry, PathExt};
///
/// for entry in Path::new(".").walk() {
///     let entry = entry.unwrap();
///     println!("{:?}", entry.path());
/// }
/// ```
///
/// [`FileIterator`]: crate::walk::FileIterator
/// [`Path`]: std::path::Path
/// [`PathExt`]: crate::walk::PathExt
/// [`PathExt::walk`]: crate::walk::PathExt::walk
#[derive(Debug)]
pub struct WalkTree {
    is_dir: bool,
    input: walkdir::IntoIter,
}

impl WalkTree {
    fn with_behavior(root: impl Into<PathBuf>, behavior: impl Into<WalkBehavior>) -> Self {
        WalkTree::with_pivot_and_behavior(root, 0, behavior)
    }

    fn with_pivot_and_behavior(
        root: impl Into<PathBuf>,
        pivot: usize,
        behavior: impl Into<WalkBehavior>,
    ) -> Self {
        let root = root.into();
        let WalkBehavior { link, depth } = behavior.into();
        let builder = WalkDir::new(root.as_path()).follow_links(match link {
            LinkBehavior::ReadFile => false,
            LinkBehavior::ReadTarget => true,
        });
        let builder = match depth {
            DepthBehavior::Max(max) => builder.max_depth(max.max_at_pivot(pivot)),
            DepthBehavior::Min(min) => builder.min_depth(min.min_at_pivot(pivot)),
            DepthBehavior::MinMax(minmax) => {
                let (min, max) = minmax.min_max_at_pivot(pivot);
                builder.min_depth(min).max_depth(max)
            },
            DepthBehavior::Unbounded => builder,
        };
        WalkTree {
            is_dir: false,
            input: builder.into_iter(),
        }
    }
}

impl CancelWalk for WalkTree {
    fn cancel_walk_tree(&mut self) {
        // `IntoIter::skip_current_dir` discards the least recently yielded directory, but
        // `cancel_walk_tree` must act upon the most recently yielded node regardless of its
        // topology (leaf vs. branch).
        if self.is_dir {
            self.input.skip_current_dir();
        }
    }
}

impl Iterator for WalkTree {
    type Item = Result<TreeEntry, WalkError>;

    fn next(&mut self) -> Option<Self::Item> {
        let (is_dir, next) = match self.input.next() {
            Some(result) => match result {
                Ok(entry) => (entry.file_type().is_dir(), Some(Ok(TreeEntry { entry }))),
                Err(error) => (false, Some(Err(error.into()))),
            },
            _ => (false, None),
        };
        self.is_dir = is_dir;
        next
    }
}

impl SeparatingFilterInput for WalkTree {
    type Feed = (Result<TreeEntry, WalkError>, TreeResidue<TreeEntry>);
}

/// An [`Iterator`] over files in a directory tree.
///
/// This iterator is aware of its hierarchical structure and can cancel traversal into directories
/// that are discarded by filter combinators to avoid unnecessary work. The contents of discarded
/// directories are not read from the file system.
///
/// The iterators constructed by [`PathExt::walk`], [`Glob::walk`], etc. implement this trait.
///
/// [`Glob::walk`]: crate::Glob::walk
/// [`PathExt::walk`]: crate::walk::PathExt::walk
/// [`Iterator`]: std::iter::Iterator
pub trait FileIterator:
    HierarchicalIterator<Feed = FileFeed<Self::Entry, Self::Residue>>
    + Iterator<Item = FileFiltrate<Self::Entry>>
{
    /// The file entry type yielded by the iterator.
    ///
    /// `FileIterator`s implement [`Iterator`] where the associated `Item` type is
    /// `Result<Self::Entry, WalkError>`.
    ///
    /// [`Result`]: std::result::Result
    type Entry: Entry;
    type Residue: Entry + From<Self::Entry>;

    /// Filters file entries and controls the traversal of the directory tree.
    ///
    /// This function constructs a combinator that filters file entries and furthermore specifies
    /// how iteration proceeds to traverse the directory tree. It accepts a function that, when
    /// discarding an entry, returns an [`EntryResidue`]. If an entry refers to a directory and the
    /// filtering function returns [`EntryResidue::Tree`], then iteration does **not** descend into
    /// that directory and the tree is **not** read from the file system.
    ///
    /// The filtering function is called even when a composing filter has already discarded a file
    /// entry. This allows filtering combinators to observe previously filtered entries and
    /// potentially discard a directory tree regardless of how they are composed. Filtering is
    /// monotonic, meaning that filtered entries can only progress forward from unfiltered `None`
    /// to filtered file `Some(EntryResidue::File)` to filtered tree `Some(EntryResidue::Tree)`. An
    /// entry cannot be "unfiltered" and if a subsequent combinator specifies a lesser filter, then
    /// it has no effect.
    ///
    /// **Prefer this combinator over functions like [`Iterator::filter`] when discarded
    /// directories need not be read.**
    ///
    /// # Examples
    ///
    /// The [`FilterEntry`] combinator can apply arbitrary and non-nominal filtering that avoids
    /// unnecessary directory reads. The following example filters out hidden files on Unix and
    /// Windows. On Unix, hidden files are filtered out nominally via [`not`]. On Windows,
    /// `filter_entry` instead detects the [hidden attribute][attributes]. In both cases, the
    /// combinator does not read conventionally hidden directory trees.
    ///
    /// ```rust,no_run
    /// use wax::Glob;
    /// use wax::walk::{Entry, FileIterator};
    ///
    /// let glob = Glob::new("**/*.(?i){jpg,jpeg}").unwrap();
    /// let walk = glob.walk("./Pictures");
    /// // Filter out nominally hidden files on Unix. Like `filter_entry`, `not` does not perform
    /// // unnecessary reads of directory trees.
    /// #[cfg(unix)]
    /// let walk = walk.not("**/.*/**").unwrap();
    /// // Filter out files with the hidden attribute on Windows.
    /// #[cfg(windows)]
    /// let walk = walk.filter_entry(|entry| {
    ///     use std::os::windows::fs::MetadataExt as _;
    ///     use wax::walk::EntryResidue;
    ///
    ///     const ATTRIBUTE_HIDDEN: u32 = 0x2;
    ///
    ///     let attributes = entry.metadata().unwrap().file_attributes();
    ///     if (attributes & ATTRIBUTE_HIDDEN) == ATTRIBUTE_HIDDEN {
    ///         // Do not read hidden directory trees.
    ///         Some(EntryResidue::Tree)
    ///     }
    ///     else {
    ///         None
    ///     }
    /// });
    /// for entry in walk {
    ///     let entry = entry.unwrap();
    ///     println!("JPEG: {:?}", entry.path());
    /// }
    /// ```
    ///
    /// [`EntryResidue`]: crate::walk::EntryResidue
    /// [`EntryResidue::Tree`]: crate::walk::EntryResidue::Tree
    /// [`FilterEntry`]: crate::walk::FilterEntry
    /// [`Iterator::filter`]: std::iter::Iterator::filter
    /// [`not`]: crate::walk::FileIterator::not
    ///
    /// [attributes]: https://docs.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
    fn filter_entry<F>(self, f: F) -> FilterEntry<Self, F>
    where
        Self: Sized,
        F: FnMut(&dyn Entry) -> Option<EntryResidue>,
    {
        FilterEntry { input: self, f }
    }

    /// Filters file entries against a negated glob.
    ///
    /// This function constructs a combinator that discards files with paths that match the given
    /// pattern. When matching a [`Glob`] against a directory tree, this allows for broad negations
    /// that cannot be achieved using a positive glob expression alone.
    ///
    /// The combinator does **not** read directory trees from the file system when a directory
    /// matches an [exhaustive glob expression][`Program::is_exhaustive`] such as `**/private/**`
    /// or `hidden/<<?>/>*`.
    ///
    /// **Prefer this combinator over matching each file entry against [`Program`]s, since it
    /// avoids potentially large and unnecessary reads and may have better performance.**
    ///
    /// # Errors
    ///
    /// Returns an error if the pattern fails to build. If the pattern is a compiled [`Program`]
    /// type such as [`Glob`], then this only occurs if the combinator program is too large.
    ///
    /// # Examples
    ///
    /// Because glob expressions do not support general negations, it is sometimes impossible to
    /// express patterns that deny particular paths. In such cases, `not` can be used to apply
    /// additional patterns as a filter.
    ///
    /// ```rust,no_run
    /// use wax::Glob;
    /// use wax::walk::FileIterator;
    ///
    /// // Find image files by extension, but not if they are beneath a directory with a name that
    /// // suggests that they are private.
    /// let glob = Glob::new("**/*.(?i){jpg,jpeg,png}").unwrap();
    /// for entry in glob.walk(".").not("**/(?i)<.:0,1>private/**").unwrap() {
    ///     let entry = entry.unwrap();
    ///     // ...
    /// }
    /// ```
    ///
    /// [`Glob`]: crate::Glob
    /// [`Program`]: crate::Program
    /// [`Program::is_exhaustive`]: crate::Program::is_exhaustive
    fn not<'t, T>(self, pattern: T) -> Result<Not<Self>, BuildError>
    where
        Self: Sized,
        T: Pattern<'t>,
    {
        let tree = pattern.try_into().map_err(Into::into)?;
        FilterAny::any(tree.into_alternatives()).map(|filter| Not {
            input: self,
            filter,
        })
    }
}

impl<T, R, I> FileIterator for I
where
    T: Entry,
    R: Entry + From<T>,
    I: HierarchicalIterator<Feed = FileFeed<T, R>> + Iterator<Item = FileFiltrate<T>>,
{
    type Entry = T;
    type Residue = R;
}

// TODO: Implement this using combinators provided by the `filter` module and RPITIT once it lands
//       in stable Rust. Remove any use of `WalkCancellation::unchecked`.
/// Iterator combinator that filters file entries and controls the traversal of directory trees.
///
/// This combinator is returned by [`FileIterator::filter_entry`] and implements [`FileIterator`].
///
/// [`FileIterator`]: crate::walk::FileIterator
/// [`FileIterator::filter_entry`]: crate::walk::FileIterator::filter_entry
#[derive(Clone, Debug)]
pub struct FilterEntry<I, F> {
    input: I,
    f: F,
}

impl<I, F> CancelWalk for FilterEntry<I, F>
where
    I: CancelWalk,
{
    fn cancel_walk_tree(&mut self) {
        self.input.cancel_walk_tree()
    }
}

impl<T, R, I, F> SeparatingFilter for FilterEntry<I, F>
where
    T: 'static + Entry,
    R: 'static + Entry + From<T>,
    I: FileIterator<Entry = T, Residue = R>,
    F: FnMut(&dyn Entry) -> Option<EntryResidue>,
{
    type Feed = I::Feed;

    fn feed(&mut self) -> Option<Separation<Self::Feed>> {
        self.input
            .feed()
            .map(|separation| match separation.transpose_filtrate() {
                Ok(separation) => separation
                    .filter_tree_by_substituent(
                        WalkCancellation::unchecked(&mut self.input),
                        |substituent| (self.f)(substituent).map(From::from),
                    )
                    .map_filtrate(Ok),
                Err(error) => error.map(Err).into(),
            })
    }
}

impl<T, R, I, F> Iterator for FilterEntry<I, F>
where
    T: 'static + Entry,
    R: 'static + Entry + From<T>,
    I: FileIterator<Entry = T, Residue = R>,
    F: FnMut(&dyn Entry) -> Option<EntryResidue>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        filter::filtrate(self)
    }
}

// TODO: Implement this using combinators provided by the `filter` module and RPITIT once it lands
//       in stable Rust. Remove any use of `WalkCancellation::unchecked`.
/// Iterator combinator that filters file entries with paths that match patterns.
///
/// This combinator is returned by [`FileIterator::not`] and implements [`FileIterator`].
///
/// [`FileIterator`]: crate::walk::FileIterator
/// [`FileIterator::not`]: crate::walk::FileIterator::not
#[derive(Clone, Debug)]
pub struct Not<I> {
    input: I,
    filter: FilterAny,
}

impl<I> CancelWalk for Not<I>
where
    I: CancelWalk,
{
    fn cancel_walk_tree(&mut self) {
        self.input.cancel_walk_tree()
    }
}

impl<T, R, I> SeparatingFilter for Not<I>
where
    T: 'static + Entry,
    R: 'static + Entry + From<T>,
    I: FileIterator<Entry = T, Residue = R>,
{
    type Feed = I::Feed;

    fn feed(&mut self) -> Option<Separation<Self::Feed>> {
        self.input
            .feed()
            .map(|separation| match separation.transpose_filtrate() {
                Ok(separation) => separation
                    .filter_tree_by_substituent(
                        WalkCancellation::unchecked(&mut self.input),
                        |substituent| self.filter.residue(substituent).map(From::from),
                    )
                    .map_filtrate(Ok),
                Err(error) => error.map(Err).into(),
            })
    }
}

impl<T, R, I> Iterator for Not<I>
where
    T: 'static + Entry,
    R: 'static + Entry + From<T>,
    I: FileIterator<Entry = T, Residue = R>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        filter::filtrate(self)
    }
}

/// Describes how file entries are read and discarded by [`FileIterator::filter_entry`].
///
/// [`FileIterator::filter_entry`]: crate::walk::FileIterator::filter_entry
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EntryResidue {
    /// Discard the file.
    ///
    /// The entry for the given file is discarded. Only this particular file is ignored and if the
    /// entry refers to a directory, then its tree is still read from the file system.
    File,
    /// Discard the file **and its directory tree**, if any.
    ///
    /// The entry for the given file is discarded. If the entry refers to a directory, then its
    /// entire tree is ignored and is **not** read from the file system.
    ///
    /// If the entry refers to a normal file (not a directory), then this is the same as
    /// [`EntryResidue::File`].
    ///
    /// [`EntryResidue::File`]: crate::walk::EntryResidue::File
    Tree,
}

impl From<EntryResidue> for TreeResidue<()> {
    fn from(residue: EntryResidue) -> Self {
        match residue {
            EntryResidue::File => TreeResidue::Node(()),
            EntryResidue::Tree => TreeResidue::Tree(()),
        }
    }
}

#[cfg(test)]
pub mod harness {
    use build_fs_tree::{Build, FileSystemTree};
    use std::collections::HashSet;
    use std::ops::Deref;
    use std::path::{Path, PathBuf};
    use tempfile::{self, TempDir};

    use crate::walk::{Entry, FileIterator};

    macro_rules! assert_set_eq {
        ($left:expr, $right:expr $(,)?) => {{
            match (&$left, &$right) {
                (left, right) if !(*left == *right) => {
                    let lrdiff: Vec<_> = left.difference(right).collect();
                    let rldiff: Vec<_> = right.difference(left).collect();
                    panic!(
                        "assertion `left == right` failed\n\
                         left: {:#?}\n\
                         right: {:#?}\n\
                         left - right: {:#?}\n\
                         right - left: {:#?}",
                        left, right, lrdiff, rldiff,
                    )
                },
                _ => {},
            }
        }};
    }
    pub(crate) use assert_set_eq;

    #[derive(Debug)]
    pub struct TempTree {
        _root: TempDir,
        path: PathBuf,
    }

    impl TempTree {
        pub fn join_all<'a, I>(&'a self, paths: I) -> impl 'a + Iterator<Item = PathBuf>
        where
            I: IntoIterator,
            I::IntoIter: 'a,
            I::Item: AsRef<Path>,
        {
            paths.into_iter().map(|path| self.join(path))
        }
    }

    impl AsRef<Path> for TempTree {
        fn as_ref(&self) -> &Path {
            &self.path
        }
    }

    impl Deref for TempTree {
        type Target = Path;

        fn deref(&self) -> &Self::Target {
            &self.path
        }
    }

    pub fn temptree<P, C>(path: impl AsRef<Path>, tree: FileSystemTree<P, C>) -> TempTree
    where
        P: AsRef<Path> + Ord,
        C: AsRef<[u8]>,
    {
        let root = tempfile::tempdir().expect("failed to create temporary directory");
        let path = root.path().join(path);
        tree.build(&path)
            .expect("failed to write tree in temporary directory");
        TempTree { _root: root, path }
    }

    pub fn assert_walk_paths_eq<I>(walk: impl FileIterator, expected: I)
    where
        I: IntoIterator,
        I::Item: Into<PathBuf>,
    {
        let paths: HashSet<_> = walk
            .map(|entry| entry.expect("failed to read file"))
            .map(Entry::into_path)
            .collect();
        assert_set_eq!(paths, expected.into_iter().map(Into::into).collect());
    }
}

#[cfg(test)]
mod tests {
    use build_fs_tree::{dir, file};
    use rstest::{fixture, rstest};
    use std::collections::HashSet;

    use crate::Pattern;
    use crate::filter::{HierarchicalIterator, Separation, TreeResidue};
    use crate::walk::behavior::{DepthBehavior, DepthMax, DepthMin, DepthMinMax, LinkBehavior};
    use crate::walk::harness::{self, TempTree, assert_set_eq};
    use crate::walk::{Entry, FileIterator, PathExt};

    const ALL: [&str; 11] = [
        "",
        "doc",
        "doc/guide.md",
        "src",
        "src/glob.rs",
        "src/lib.rs",
        "tests",
        "tests/harness",
        "tests/harness/mod.rs",
        "tests/walk.rs",
        "README.md",
    ];

    fn except<'t>(paths: impl IntoIterator<Item = &'t str>) -> impl Iterator<Item = &'t str> {
        let paths: HashSet<_> = paths.into_iter().collect();
        ALL.into_iter().filter(move |path| !paths.contains(path))
    }

    // TODO: Rust's testing framework does not provide a mechanism for maintaining shared state nor
    //       hooks for the start and end of testing. This means that tests that write to the file
    //       system must do so for each test rather than writing before and after all tests have
    //       run. The `once` attribute can be applied to fixtures to prevent this, but because
    //       `rstest` has no way to know when testing has completed, such fixtures leak their
    //       outputs and the temporary files are not removed. Is there a way to both share the
    //       fixture and also drop its output?
    //
    //       See https://github.com/la10736/rstest/issues/209
    /// Writes a testing directory tree to a temporary location on the file system.
    #[fixture]
    fn temptree() -> TempTree {
        harness::temptree::<&str, &str>(
            "project",
            dir! {
                "doc" => dir! {
                    "guide.md" => file!(""),
                },
                "src" => dir! {
                    "glob.rs" => file!(""),
                    "lib.rs" => file!(""),
                },
                "tests" => dir! {
                    "harness" => dir! {
                        "mod.rs" => file!(""),
                    },
                    "walk.rs" => file!(""),
                },
                "README.md" => file!(""),
            },
        )
    }

    /// Writes a testing directory tree that includes a re-entrant symbolic link to a temporary
    /// location on the file system.
    #[cfg(any(unix, windows))]
    #[fixture]
    fn temptree_with_cyclic_link() -> TempTree {
        use std::io;
        use std::path::Path;

        #[cfg(unix)]
        fn link(target: impl AsRef<Path>, link: impl AsRef<Path>) -> io::Result<()> {
            std::os::unix::fs::symlink(target, link)
        }

        #[cfg(windows)]
        fn link(target: impl AsRef<Path>, link: impl AsRef<Path>) -> io::Result<()> {
            std::os::windows::fs::symlink_dir(target, link)
        }

        // Get a temporary tree and create a re-entrant symbolic link.
        let temptree = temptree();
        link(&temptree, temptree.join("tests/cycle"))
            .expect("failed to write symbolic link in temporary tree");
        temptree
    }

    #[rstest]
    fn walk_path_includes_all_paths(temptree: TempTree) {
        harness::assert_walk_paths_eq(temptree.walk(), temptree.join_all(ALL));
    }

    #[rstest]
    #[case::subtree(
        "tests/**",
        [
            "",
            "doc",
            "doc/guide.md",
            "src",
            "src/glob.rs",
            "src/lib.rs",
            "README.md",
        ],
    )]
    #[case::extension_from_root("*.md", except(["README.md"]))]
    #[case::extension_from_any_tree("**/*.md", except(["doc/guide.md", "README.md"]))]
    #[case::any(
        crate::any(["**/*.rs", "tests/**"]),
        ["", "doc", "doc/guide.md", "src", "README.md"],
    )]
    // The root path ("") must **not** be present, because the empty `not` pattern matches the
    // empty relative path at the root.
    #[case::empty("", except([""]))]
    fn walk_path_with_not_excludes_only_matching_paths<'t>(
        temptree: TempTree,
        #[case] pattern: impl Pattern<'t>,
        #[case] expected: impl IntoIterator<Item = &'t str>,
    ) {
        harness::assert_walk_paths_eq(
            temptree.walk().not(pattern).unwrap(),
            temptree.join_all(expected),
        );
    }

    #[rstest]
    #[case::max(DepthMax(1), ["", "doc", "src", "tests", "README.md"])]
    #[case::min(
        DepthMin::from_min_or_unbounded(2),
        [
            "doc/guide.md",
            "src/glob.rs",
            "src/lib.rs",
            "tests/harness",
            "tests/harness/mod.rs",
            "tests/walk.rs",
        ],
    )]
    #[case::minmax(
        DepthMinMax::from_depths_or_max(2, 2),
        ["doc/guide.md", "src/glob.rs", "src/lib.rs", "tests/harness", "tests/walk.rs"],
    )]
    fn walk_path_with_depth_behavior_includes_only_paths_at_depth<'t>(
        temptree: TempTree,
        #[case] depth: impl Into<DepthBehavior>,
        #[case] expected: impl IntoIterator<Item = &'t str>,
    ) {
        harness::assert_walk_paths_eq(
            temptree.walk_with_behavior(depth.into()),
            temptree.join_all(expected),
        );
    }

    #[rstest]
    #[case::tree("**", ALL)]
    #[case::bounded_terminating_component("**/*.md", ["doc/guide.md", "README.md"])]
    #[case::invariant_intermediate_component("**/src/**/*.rs", ["src/glob.rs", "src/lib.rs"])]
    #[case::invariant("src/lib.rs", ["src/lib.rs"])]
    fn walk_glob_includes_only_matching_paths<'t>(
        temptree: TempTree,
        #[case] expression: &str,
        #[case] expected: impl IntoIterator<Item = &'t str>,
    ) {
        harness::assert_walk_paths_eq(
            crate::harness::assert_new_glob_is_ok(expression).walk(temptree.as_ref()),
            temptree.join_all(expected),
        );
    }

    #[rstest]
    fn walk_empty_partitioned_glob_at_non_empty_prefix_includes_only_prefix(temptree: TempTree) {
        let (prefix, glob) =
            crate::harness::assert_new_glob_is_ok("src/lib.rs").partition_or_empty();
        harness::assert_walk_paths_eq(
            glob.walk(temptree.join(prefix)),
            [temptree.join("src/lib.rs")],
        );
    }

    #[rstest]
    fn walk_glob_with_exhaustive_not_cancels_walk(temptree: TempTree) {
        #[derive(Debug, Eq, Hash, PartialEq)]
        enum TestSeparation<T> {
            Filtrate(T),
            Residue(TreeResidue<T>),
        }
        use TestSeparation::{Filtrate, Residue};
        use TreeResidue::{Node, Tree};

        let glob = crate::harness::assert_new_glob_is_ok("**/*.{md,rs}");
        let mut paths = HashSet::new();
        glob.walk(temptree.as_ref())
            .not("**/harness/**")
            .unwrap()
            // Inspect the feed rather than the `Iterator` output (filtrate). While it is trivial
            // to provide a way to collect the feed, it is difficult to inspect its contents. In
            // particular, it is not possible to construct `Product`s outside of the `filter`
            // module (by design). Instead, the feed is collected into a simpler format in
            // `filter_map_tree`.
            .filter_map_tree(|_, separation| {
                paths.insert(match separation.as_ref() {
                    Separation::Filtrate(filtrate) => Filtrate(
                        filtrate
                            .get()
                            .as_ref()
                            .expect("failed to read file")
                            .path()
                            .to_path_buf(),
                    ),
                    Separation::Residue(residue) => Residue(
                        residue
                            .get()
                            .as_ref()
                            .map(|residue| residue.path().to_path_buf()),
                    ),
                });
                separation
            })
            .for_each(drop);
        assert_set_eq!(
            paths,
            [
                Residue(Node(temptree.to_path_buf())),
                Residue(Node(temptree.join("doc"))),
                Filtrate(temptree.join("doc/guide.md")),
                Residue(Node(temptree.join("src"))),
                Filtrate(temptree.join("src/glob.rs")),
                Filtrate(temptree.join("src/lib.rs")),
                Residue(Node(temptree.join("tests"))),
                // This entry is important. The glob does **not** match this path and will separate
                // it into node residue (not tree residue). The glob **does** match paths beneath
                // it. The `not` iterator must subsequently observe the residue and map it from
                // node to tree and cancel the walk. Nothing beneath this directory must be present
                // at all, even as residue.
                Residue(Tree(temptree.join("tests/harness"))),
                Filtrate(temptree.join("tests/walk.rs")),
                Filtrate(temptree.join("README.md")),
            ]
            .into_iter()
            .collect(),
        );
    }

    #[rstest]
    #[case::non_zero_max(DepthMax(1), "**", ["", "doc", "src", "tests", "README.md"])]
    #[case::zero_max(DepthMax(0), "**", [""])]
    #[case::min(
        DepthMin::from_min_or_unbounded(2),
        "**",
        [
            "doc/guide.md",
            "src/glob.rs",
            "src/lib.rs",
            "tests/harness",
            "tests/harness/mod.rs",
            "tests/walk.rs",
        ],
    )]
    #[case::prefixed_minmax(
        DepthMinMax::from_depths_or_max(2, 2),
        "tests/**",
        ["tests/harness", "tests/walk.rs"],
    )]
    fn walk_glob_with_depth_behavior_includes_only_paths_at_depth<'t>(
        temptree: TempTree,
        #[case] depth: impl Into<DepthBehavior>,
        #[case] expression: &str,
        #[case] expected: impl IntoIterator<Item = &'t str>,
    ) {
        harness::assert_walk_paths_eq(
            crate::harness::assert_new_glob_is_ok(expression)
                .walk_with_behavior(temptree.as_ref(), depth.into()),
            temptree.join_all(expected),
        );
    }

    #[cfg(any(unix, windows))]
    #[rstest]
    fn walk_glob_with_read_link_file_behavior_includes_link_file(
        #[from(temptree_with_cyclic_link)] temptree: TempTree,
    ) {
        harness::assert_walk_paths_eq(
            crate::harness::assert_new_glob_is_ok("**")
                .walk_with_behavior(temptree.as_ref(), LinkBehavior::ReadFile),
            temptree.join_all([
                "",
                "doc",
                "doc/guide.md",
                "src",
                "src/glob.rs",
                "src/lib.rs",
                "tests",
                "tests/cycle",
                "tests/harness",
                "tests/harness/mod.rs",
                "tests/walk.rs",
                "README.md",
            ]),
        );
    }

    #[cfg(any(unix, windows))]
    #[rstest]
    fn walk_glob_with_read_link_target_behavior_excludes_cyclic_link_target(
        #[from(temptree_with_cyclic_link)] temptree: TempTree,
    ) {
        // Collect paths into `Vec`s so that duplicates can be detected.
        let expected = vec![
            #[allow(clippy::redundant_clone)]
            temptree.to_path_buf(),
            temptree.join("README.md"),
            temptree.join("doc"),
            temptree.join("doc/guide.md"),
            temptree.join("src"),
            temptree.join("src/glob.rs"),
            temptree.join("src/lib.rs"),
            temptree.join("tests"),
            temptree.join("tests/harness"),
            temptree.join("tests/harness/mod.rs"),
            temptree.join("tests/walk.rs"),
        ];
        let glob = crate::harness::assert_new_glob_is_ok("**");
        let mut paths: Vec<_> = glob
            .walk_with_behavior(temptree.as_ref(), LinkBehavior::ReadTarget)
            .flatten()
            // Take an additional item. This prevents an infinite loop if there is a problem with
            // detecting the cycle while also introducing unexpected files so that the error can be
            // detected.
            .take(expected.len() + 1)
            .map(Entry::into_path)
            .collect();
        paths.sort_unstable();
        assert_eq!(paths, expected);
    }
}
