use itertools::Itertools;
use regex::Regex;
use std::borrow::Borrow;
use std::fs::{FileType, Metadata};
use std::path::{Component, Path, PathBuf};

use crate::capture::MatchedText;
use crate::encode::CompileError;
use crate::filter::{HierarchicalIterator, Separation};
use crate::token::{Token, TokenTree, Tokenized};
use crate::walk::{
    Entry, EntryResidue, FileIterator, JoinAndGetDepth, SplitAtDepth, TreeEntry, WalkBehavior,
    WalkError, WalkTree,
};
use crate::{BuildError, CandidatePath, Glob, Pattern};

/// APIs for matching globs against directory trees.
impl<'t> Glob<'t> {
    // TODO: Document the behavior of empty globs: they yield the root path of the walk and nothing
    //       more, which is not at all obvious.
    /// Gets an iterator over matching file paths in a directory tree.
    ///
    /// This function matches a `Glob` against a directory tree, returning a [`FileIterator`] that
    /// yields a [`GlobEntry`] for each matching file. `Glob`s are the only [`Program`]s that
    /// support this semantic operation; it is not possible to match combinators ([`Any`]) against
    /// directory trees.
    ///
    /// As with [`Path::join`] and [`PathBuf::push`], the base directory can be escaped or
    /// overridden by [a `Glob` that has a root][`Program::has_root`]. In many cases, the current
    /// working directory `.` is an appropriate base directory and will be intuitively ignored if
    /// the `Glob` is rooted, such as in `/mnt/media/**/*.mp4`.
    ///
    /// The [root path segment][`Entry::root_relative_paths`] is either the given directory or, if
    /// the `Glob` has a root, the [invariant prefix][`Glob::partition`] of the `Glob`. Either way,
    /// this function joins the given directory with any invariant prefix in the `Glob` to
    /// potentially begin the walk as far down the tree as possible. **The prefix and any [semantic
    /// literals][`Glob::has_semantic_literals`] in this prefix are interpreted semantically as a
    /// path**, so components like `.` and `..` that precede variant patterns interact with the
    /// base directory semantically. This means that expressions like `../**` escape the base
    /// directory as expected on Unix and Windows, for example.
    ///
    /// This function uses the default [`WalkBehavior`]. To configure the behavior of the
    /// traversal, see [`Glob::walk_with_behavior`].
    ///
    /// Unlike functions in [`Program`], **this operation is semantic and interacts with the file
    /// system**.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use wax::Glob;
    /// use wax::walk::Entry;
    ///
    /// let glob = Glob::new("**/*.(?i){jpg,jpeg}").unwrap();
    /// for entry in glob.walk("./Pictures") {
    ///     let entry = entry.unwrap();
    ///     println!("JPEG: {:?}", entry.path());
    /// }
    /// ```
    ///
    /// Glob expressions do not support general negations, but the [`not`] combinator can be used
    /// when walking a directory tree to filter entries using patterns. **Prefer this over
    /// functions like [`Iterator::filter`], because it avoids unnecessary reads of directory trees
    /// when matching [exhaustive negations][`Program::is_exhaustive`].**
    ///
    /// ```rust,no_run
    /// use wax::Glob;
    /// use wax::walk::{Entry, FileIterator};
    ///
    /// let glob = Glob::new("**/*.(?i){jpg,jpeg,png}").unwrap();
    /// for entry in glob
    ///     .walk("./Pictures")
    ///     .not("**/(i?){background<s:0,1>,wallpaper<s:0,1>}/**")
    ///     .unwrap()
    /// {
    ///     let entry = entry.unwrap();
    ///     println!("{:?}", entry.path());
    /// }
    /// ```
    ///
    /// [`Any`]: crate::Any
    /// [`Entry::root_relative_paths`]: crate::walk::Entry::root_relative_paths
    /// [`Glob::walk_with_behavior`]: crate::Glob::walk_with_behavior
    /// [`GlobEntry`]: crate::walk::GlobEntry
    /// [`FileIterator`]: crate::walk::FileIterator
    /// [`Iterator::filter`]: std::iter::Iterator::filter
    /// [`not`]: crate::walk::FileIterator::not
    /// [`Path::join`]: std::path::Path::join
    /// [`PathBuf::push`]: std::path::PathBuf::push
    /// [`Program`]: crate::Program
    /// [`Program::has_root`]: crate::Program::has_root
    /// [`Program::is_exhaustive`]: crate::Program::is_exhaustive
    /// [`WalkBehavior`]: crate::walk::WalkBehavior
    pub fn walk(&self, path: impl Into<PathBuf>) -> impl 'static + FileIterator<Entry = GlobEntry> {
        self.walk_with_behavior(path, WalkBehavior::default())
    }

    /// Gets an iterator over matching files in a directory tree.
    ///
    /// This function is the same as [`Glob::walk`], but it additionally accepts a [`WalkBehavior`]
    /// that configures how the traversal interacts with symbolic links, bounds on depth, etc.
    ///
    /// Depth is bounded relative to [the root path segment][`Entry::root_relative_paths`]
    /// of the traversal.
    ///
    /// See [`Glob::walk`] for more information.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use wax::Glob;
    /// use wax::walk::{Entry, WalkBehavior};
    ///
    /// let glob = Glob::new("**/*.(?i){jpg,jpeg}").unwrap();
    /// for entry in glob.walk_with_behavior("./Pictures", WalkBehavior::default()) {
    ///     let entry = entry.unwrap();
    ///     println!("JPEG: {:?}", entry.path());
    /// }
    /// ```
    ///
    /// By default, symbolic links are read as normal files and their targets are ignored. To
    /// follow symbolic links and traverse any directories that they reference, specify a
    /// [`LinkBehavior`].
    ///
    /// ```rust,no_run
    /// use wax::Glob;
    /// use wax::walk::{Entry, LinkBehavior};
    ///
    /// let glob = Glob::new("**/*.txt").unwrap();
    /// for entry in glob.walk_with_behavior("/var/log", LinkBehavior::ReadTarget) {
    ///     let entry = entry.unwrap();
    ///     println!("Log: {:?}", entry.path());
    /// }
    /// ```
    ///
    /// [`Entry::root_relative_paths`]: crate::walk::Entry::root_relative_paths
    /// [`Glob::walk`]: crate::Glob::walk
    /// [`LinkBehavior`]: crate::walk::LinkBehavior
    /// [`WalkBehavior`]: crate::walk::WalkBehavior
    pub fn walk_with_behavior(
        &self,
        path: impl Into<PathBuf>,
        behavior: impl Into<WalkBehavior>,
    ) -> impl 'static + FileIterator<Entry = GlobEntry> {
        GlobWalker {
            anchor: self.anchor(path),
            program: WalkProgram {
                complete: self.program.clone(),
                // Do not compile component programs for empty globs.
                //
                // An empty glob consists solely of an empty literal token and only matches empty
                // text (""). A walk program compiled from such a glob has an empty component
                // pattern and matches nothing. This means that walking an empty glob never yields
                // any paths. At first blush, this seems consistent with an empty glob. However,
                // walking conceptually matches a glob against the sub-trees in a path and there is
                // arguably an implicit empty tree. This is also more composable when partitioning
                // and (re)building paths.
                //
                // The result is that matching an empty glob against the path `foo` yields `foo`
                // and only `foo` (assuming that the path exists).
                components: if self.is_empty() {
                    vec![]
                }
                else {
                    WalkProgram::compile::<Tokenized<_>>(self.tree.as_ref())
                        .expect("failed to compile walk program")
                },
            },
        }
        .walk_with_behavior(behavior)
    }

    fn anchor(&self, path: impl Into<PathBuf>) -> Anchor {
        let path = path.into();
        let prefix: Option<PathBuf> = {
            let (_, prefix) = self.tree.as_ref().as_token().invariant_text_prefix();
            if prefix.is_empty() {
                None
            }
            else {
                Some(prefix.into())
            }
        };
        // Establish the root path and any pivot in that root path from the given directory and any
        // invariant prefix in the glob. The file system is traversed from this root path. The
        // pivot partitions the root path into the given directory and any invariant prefix by
        // specifying how many components from the end of the root path must be popped to restore
        // the given directory. The popped components form the invariant prefix of the glob. Either
        // partition of the root path may be empty depending on the given directory and the glob
        // pattern. In this way, any invariant prefix of the glob becomes a postfix in the root
        // path.
        //
        // Note that a rooted glob, like in `Path::join`, replaces the given directory when
        // establishing the root path. In this case, there is no invariant prefix (the pivot is
        // zero), as the entire root path is present in the glob expression and the given directory
        // is completely discarded.
        let (root, pivot) = match prefix {
            Some(prefix) => path.join_and_get_depth(prefix),
            _ => (path, 0),
        };
        Anchor { root, pivot }
    }
}

/// Root path and pivot of a `Glob` when walking a particular target path.
///
/// For unrooted globs, the pivot can be used to isolate the target path given to walk functions
/// like `Glob::walk`. This is necessary to implement `Entry` and for interpreting depth behavior,
/// which is always relative to the target path (and ignores any invariant prefix in a glob).
#[derive(Clone, Debug)]
struct Anchor {
    /// The root path of the walk.
    ///
    /// This root, unlike in `PathExt::walk`, may include an invariant prefix from a glob.
    root: PathBuf,
    /// The number of components from the end of `root` that are present in any invariant prefix of
    /// the glob expression.
    ///
    /// The pivot partitions the root path into the target path and any invariant prefix in the
    /// `Glob` (this prefix becomes a postfix in the root path or, when rooted, replaces any target
    /// path).
    pivot: usize,
}

impl Anchor {
    pub fn walk_with_behavior(self, behavior: impl Into<WalkBehavior>) -> WalkTree {
        WalkTree::with_pivot_and_behavior(self.root, self.pivot, behavior)
    }
}

#[derive(Clone, Debug)]
struct WalkProgram {
    complete: Regex,
    components: Vec<Regex>,
}

impl WalkProgram {
    fn compile<'t, T>(tree: impl Borrow<T>) -> Result<Vec<Regex>, CompileError>
    where
        T: TokenTree<'t>,
    {
        let mut regexes = Vec::new();
        for component in tree.borrow().as_token().components() {
            if component.tokens().iter().any(Token::has_boundary) {
                // Stop at component boundaries, such as tree wildcards or any boundary within a
                // branch token.
                break;
            }
            regexes.push(Glob::compile(component)?);
        }
        Ok(regexes)
    }
}

/// Describes iteration over matching files in a directory tree.
#[derive(Clone, Debug)]
struct GlobWalker {
    anchor: Anchor,
    program: WalkProgram,
}

impl GlobWalker {
    /// Converts a walker into an iterator over matching files in its directory tree.
    ///
    /// See [`Glob::walk_with_behavior`].
    ///
    /// [`Glob::walk_with_behavior`]: crate::Glob::walk_with_behavior
    pub fn walk_with_behavior(
        self,
        behavior: impl Into<WalkBehavior>,
    ) -> impl 'static + FileIterator<Entry = GlobEntry, Residue = TreeEntry> {
        let pivot = self.anchor.pivot;
        self.anchor
            .walk_with_behavior(behavior)
            .filter_map_tree(move |cancellation, separation| {
                use itertools::EitherOrBoth::{Both, Left, Right};
                use itertools::Position::{First, Last, Middle, Only};

                let filtrate = match separation.filtrate() {
                    Some(filtrate) => match filtrate.transpose() {
                        Ok(filtrate) => filtrate,
                        Err(error) => {
                            return Separation::from(error.map(Err));
                        },
                    },
                    // `Path::walk_with_behavior` yields no residue.
                    _ => unreachable!(),
                };
                let entry = filtrate.as_ref();
                let (_, path) = self::root_relative_paths(entry.path(), entry.depth(), pivot);
                let depth = entry.depth().saturating_sub(1);
                for (position, candidate) in path
                    .components()
                    .skip(depth)
                    .filter_map(|component| match component {
                        Component::Normal(component) => Some(CandidatePath::from(component)),
                        _ => None,
                    })
                    .zip_longest(self.program.components.iter().skip(depth))
                    .with_position()
                {
                    match (position, candidate) {
                        (First | Middle, Both(candidate, program)) => {
                            if !program.is_match(candidate.as_ref()) {
                                // Do not walk directories that do not match the corresponding
                                // component program.
                                return filtrate.filter_tree(cancellation).into();
                            }
                        },
                        (Last | Only, Both(candidate, program)) => {
                            return if program.is_match(candidate.as_ref()) {
                                let candidate = CandidatePath::from(path);
                                if let Some(matched) = self
                                    .program
                                    .complete
                                    .captures(candidate.as_ref())
                                    .map(MatchedText::from)
                                    .map(MatchedText::into_owned)
                                {
                                    filtrate
                                        .map(|entry| {
                                            Ok(GlobEntry {
                                                entry,
                                                pivot,
                                                matched,
                                            })
                                        })
                                        .into()
                                }
                                else {
                                    filtrate.filter_node().into()
                                }
                            }
                            else {
                                // Do not walk directories that do not match the corresponding
                                // component program.
                                filtrate.filter_tree(cancellation).into()
                            };
                        },
                        (_, Left(_candidate)) => {
                            let candidate = CandidatePath::from(path);
                            return if let Some(matched) = self
                                .program
                                .complete
                                .captures(candidate.as_ref())
                                .map(MatchedText::from)
                                .map(MatchedText::into_owned)
                            {
                                filtrate
                                    .map(|entry| {
                                        Ok(GlobEntry {
                                            entry,
                                            pivot,
                                            matched,
                                        })
                                    })
                                    .into()
                            }
                            else {
                                filtrate.filter_node().into()
                            };
                        },
                        (_, Right(_pattern)) => {
                            return filtrate.filter_node().into();
                        },
                    }
                }
                // If the component loop is not entered, then check for a match. This may indicate
                // that the `Glob` is empty and a single invariant path may be matched.
                let candidate = CandidatePath::from(path);
                if let Some(matched) = self
                    .program
                    .complete
                    .captures(candidate.as_ref())
                    .map(MatchedText::from)
                    .map(MatchedText::into_owned)
                {
                    return filtrate
                        .map(|entry| {
                            Ok(GlobEntry {
                                entry,
                                pivot,
                                matched,
                            })
                        })
                        .into();
                }
                filtrate.filter_node().into()
            })
    }
}

#[derive(Clone, Debug)]
enum FilterAnyProgram {
    Empty,
    Exhaustive(Regex),
    Nonexhaustive(Regex),
    // Partitioned programs are important here and are used to more reliably determine the
    // exhaustiveness of a match and avoid unnecessary reads. The exhaustiveness of a glob is not
    // the same as the exhaustiveness of a particular match. The former is not certain in the
    // absence of search text (that is, it may be exhaustive only sometimes), but the latter is.
    Partitioned {
        exhaustive: Regex,
        nonexhaustive: Regex,
    },
}

impl FilterAnyProgram {
    pub fn try_from_partitions<'t, I>(exhaustive: I, nonexhaustive: I) -> Result<Self, BuildError>
    where
        I: IntoIterator,
        I::Item: Pattern<'t>,
        I::IntoIter: ExactSizeIterator,
    {
        use FilterAnyProgram::{Empty, Exhaustive, Nonexhaustive, Partitioned};

        // It is important to distinguish between empty _partitions_ and empty _expressions_ here.
        // `FilterAnyProgram::compile` discards empty partitions. When matching against an empty
        // path, an explicit empty _expression_ must match but an empty _partition_ must not (such
        // a partition must never match anything).
        Ok(
            match (
                FilterAnyProgram::compile(exhaustive)?,
                FilterAnyProgram::compile(nonexhaustive)?,
            ) {
                (Some(exhaustive), Some(nonexhaustive)) => Partitioned {
                    exhaustive,
                    nonexhaustive,
                },
                (Some(exhaustive), None) => Exhaustive(exhaustive),
                (None, Some(nonexhaustive)) => Nonexhaustive(nonexhaustive),
                (None, None) => Empty,
            },
        )
    }

    pub fn residue(&self, candidate: CandidatePath<'_>) -> Option<EntryResidue> {
        use FilterAnyProgram::{Exhaustive, Nonexhaustive, Partitioned};

        match self {
            Exhaustive(exhaustive) | Partitioned { exhaustive, .. }
                if exhaustive.is_match(candidate.as_ref()) =>
            {
                Some(EntryResidue::Tree)
            },
            Nonexhaustive(nonexhaustive) | Partitioned { nonexhaustive, .. }
                if nonexhaustive.is_match(candidate.as_ref()) =>
            {
                Some(EntryResidue::File)
            },
            _ => None,
        }
    }

    fn compile<'t, I>(tokens: I) -> Result<Option<Regex>, BuildError>
    where
        I: IntoIterator,
        I::Item: Pattern<'t>,
        I::IntoIter: ExactSizeIterator,
    {
        let tokens = tokens.into_iter();
        if 0 == tokens.len() {
            Ok(None)
        }
        else {
            crate::any(tokens).map(|any| Some(any.program))
        }
    }
}

/// Negated glob combinator that efficiently filters file entries against patterns.
#[derive(Clone, Debug)]
pub struct FilterAny {
    program: FilterAnyProgram,
}

impl FilterAny {
    /// Combines patterns into a `FilterAny`.
    ///
    /// This function accepts an [`IntoIterator`] with items that implement [`Pattern`], such as
    /// [`Glob`] and `&str`.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the inputs fail to build. If the inputs are a compiled
    /// [`Program`] type such as [`Glob`], then this only occurs if the compiled program is too
    /// large.
    ///
    /// [`Glob`]: crate::Glob
    /// [`IntoIterator`]: std::iter::IntoIterator
    /// [`Pattern`]: crate::Pattern
    /// [`Program`]: crate::Program
    pub fn any<'t, I>(patterns: I) -> Result<Self, BuildError>
    where
        I: IntoIterator,
        I::Item: Pattern<'t>,
    {
        let (exhaustive, nonexhaustive) = patterns
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)?
            .into_iter()
            .partition::<Vec<_>, _>(|tree| tree.as_ref().as_token().is_exhaustive().is_always());
        FilterAnyProgram::try_from_partitions(exhaustive, nonexhaustive)
            .map(|program| FilterAny { program })
    }

    /// Gets the appropriate [`EntryResidue`] for the given [`Entry`].
    ///
    /// Notably, this function returns [`EntryResidue::Tree`] if the [`Entry`] matches an
    /// [exhaustive glob expression][`Program::is_exhaustive`], such as `secret/**`.
    ///
    /// [`Entry`]: crate::walk::Entry
    /// [`EntryResidue`]: crate::walk::EntryResidue
    /// [`EntryResidue::Tree`]: crate::walk::EntryResidue::Tree
    /// [`Program::is_exhaustive`]: crate::Program::is_exhaustive
    pub fn residue(&self, entry: &dyn Entry) -> Option<EntryResidue> {
        let candidate = CandidatePath::from(entry.root_relative_paths().1);
        self.program.residue(candidate)
    }
}

/// Describes a file with a path matching a [`Glob`] in a directory tree.
///
/// See [`Glob::walk`].
///
/// [`Glob`]: crate::Glob
/// [`Glob::walk`]: crate::Glob::walk
#[derive(Debug)]
pub struct GlobEntry {
    entry: TreeEntry,
    pivot: usize,
    matched: MatchedText<'static>,
}

impl GlobEntry {
    /// Converts the entry to the relative [`CandidatePath`].
    ///
    /// **This differs from [`Entry::path`] and [`Entry::into_path`], which are native paths and
    /// typically include the root path.** The [`CandidatePath`] is always relative to [the root
    /// path][`Entry::root_relative_paths`].
    ///
    /// [`CandidatePath`]: crate::CandidatePath
    /// [`Entry::into_path`]: crate::walk::Entry::into_path
    /// [`Entry::path`]: crate::walk::Entry::path
    /// [`matched`]: crate::walk::GlobEntry::matched
    pub fn to_candidate_path(&self) -> CandidatePath<'_> {
        self.matched.to_candidate_path()
    }

    /// Gets the matched text in the path of the file.
    pub fn matched(&self) -> &MatchedText<'static> {
        &self.matched
    }
}

impl Entry for GlobEntry {
    fn into_path(self) -> PathBuf {
        self.entry.into_path()
    }

    fn path(&self) -> &Path {
        self.entry.path()
    }

    fn root_relative_paths(&self) -> (&Path, &Path) {
        self::root_relative_paths(self.path(), self.entry.depth(), self.pivot)
    }

    fn file_type(&self) -> FileType {
        self.entry.file_type()
    }

    fn metadata(&self) -> Result<Metadata, WalkError> {
        self.entry.metadata()
    }

    fn depth(&self) -> usize {
        self.entry
            .depth()
            .checked_add(self.pivot)
            .expect("overflow determining depth")
    }
}

impl From<GlobEntry> for TreeEntry {
    fn from(entry: GlobEntry) -> Self {
        entry.entry
    }
}

fn root_relative_paths(path: &Path, depth: usize, pivot: usize) -> (&Path, &Path) {
    path.split_at_depth(
        depth
            .checked_add(pivot)
            .expect("overflow determining root and relative paths"),
    )
}
