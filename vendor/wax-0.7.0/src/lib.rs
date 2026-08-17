//! Wax provides opinionated and portable globs that can be matched against file paths and
//! directory trees. Globs use a familiar syntax and support expressive features with semantics
//! that emphasize component boundaries.
//!
//! See the [repository documentation](https://github.com/olson-sean-k/wax/blob/master/README.md)
//! for details about glob expressions and patterns.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/olson-sean-k/wax/master/doc/wax-favicon.svg?sanitize=true"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/olson-sean-k/wax/master/doc/wax.svg?sanitize=true"
)]
#![deny(
    clippy::cast_lossless,
    clippy::checked_conversions,
    clippy::cloned_instead_of_copied,
    clippy::explicit_into_iter_loop,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::from_iter_instead_of_collect,
    clippy::if_not_else,
    clippy::manual_ok_or,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_else,
    clippy::unreadable_literal,
    clippy::unused_self
)]

mod capture;
mod diagnostics;
mod encode;
mod filter;
pub mod query;
mod rule;
mod token;
pub mod walk;

/// Re-exports of commonly used items.
///
/// This module anonymously re-exports traits for matching [`Program`]s against file paths and
/// directory trees. A glob import of this module can be used instead of individual imports of
/// these traits.
///
/// # Examples
///
/// ```rust,no_run,ignore
/// use wax::prelude::*;
/// use wax::Glob;
///
/// // This code requires the `Entry` and `FileIterator` traits.
/// let glob = Glob::new("**/*.(?i){jpg,jpeg}").unwrap();
/// for entry in glob.walk("textures").not(["**/.*/**"]).unwrap().flatten() {
///     println!("JPEG: {:?}", entry.path());
/// }
/// ```
pub mod prelude {
    pub use crate::Program as _;
    pub use crate::query::LocatedError as _;
    #[cfg(feature = "walk")]
    pub use crate::walk::{Entry as _, FileIterator as _, PathExt as _};
}

#[cfg(feature = "miette")]
use miette::Diagnostic;
use regex::Regex;
use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::convert::Infallible;
use std::ffi::OsStr;
use std::fmt::{self, Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::str::{self, FromStr};
use thiserror::Error;

use crate::diagnostics::LocatedError;
use crate::encode::CompileError;
use crate::query::{CapturingToken, DepthVariance, TextVariance, When};
use crate::rule::{Checked, RuleError};
use crate::token::{
    ConcatenationTree, ExpressionMetadata, ParseError, Token, TokenTree, Tokenized,
};
#[cfg(feature = "walk")]
use crate::walk::WalkError;

pub use crate::capture::MatchedText;

#[cfg(windows)]
const PATHS_ARE_CASE_INSENSITIVE: bool = true;
#[cfg(not(windows))]
const PATHS_ARE_CASE_INSENSITIVE: bool = false;

trait CharExt: Sized {
    /// Returns `true` if the character (code point) has casing.
    fn has_casing(self) -> bool;
}

impl CharExt for char {
    fn has_casing(self) -> bool {
        self.is_lowercase() != self.is_uppercase()
    }
}

trait StrExt {
    /// Returns `true` if any characters in the string have casing.
    fn has_casing(&self) -> bool;
}

impl StrExt for str {
    fn has_casing(&self) -> bool {
        self.chars().any(CharExt::has_casing)
    }
}

/// A representation of a glob expression.
///
/// This trait is implemented by types that can be converted into a [`Program`], such as `str`
/// slices and compiled [`Program`] types like [`Glob`]. APIs that accept patterns typically do so
/// via this trait.
///
/// # Examples
///
/// [`Result`] types also implement this trait when the success type is a [`Program`] and the error
/// type is [`BuildError`]. This means that APIs that accept a `Pattern` can also accept the
/// intermediate result of constructing a [`Program`] without the need to explicitly inspect the
/// inner result first (error handling can be deferred).
///
/// ```rust
/// use wax::{Glob, Program};
///
/// # fn fallible() -> Result<(), wax::BuildError> {
/// // The inner results of `any` are not checked via `?` and can be passed directly to the
/// // outermost `any` call. This is true of all APIs that accept `Pattern`.
/// #[rustfmt::skip]
/// let any = wax::any([
///     wax::any([Glob::new("**/*.txt")]),
///     wax::any([
///         "**/*.pdf",
///         "**/*.tex",
///     ]),
/// ])?; // Defer error handling until the here.
///
/// # Ok(())
/// # }
/// ```
///
/// [`BuildError`]: crate::BuildError
/// [`Glob`]: crate::Glob
/// [`Program`]: crate::Program
/// [`Result`]: std::result::Result
pub trait Pattern<'t>:
    TryInto<Checked<Self::Tokens>, Error = <Self as Pattern<'t>>::Error>
{
    type Tokens: TokenTree<'t>;
    type Error: Into<BuildError>;
}

impl<'t, T> Pattern<'t> for Result<T, BuildError>
where
    T: Into<Checked<T::Tokens>> + Pattern<'t>,
{
    type Tokens = <T as Pattern<'t>>::Tokens;
    type Error = BuildError;
}

impl<'t> Pattern<'t> for &'t str {
    type Tokens = Tokenized<'t, ExpressionMetadata>;
    type Error = BuildError;
}

/// A compiled [`Pattern`] that can be inspected and matched against paths.
///
/// [`Glob::partition`]: crate::Glob::partition
/// [`Path`]: std::path::Path
/// [`PathBuf`]: std::path::PathBuf
/// [`Pattern`]: crate::Pattern
pub trait Program<'t>: Pattern<'t, Error = Infallible> {
    /// Returns `true` if the [candidate path][`CandidatePath`] matches the pattern.
    ///
    /// This is a logical operation and does **not** interact with the file system.
    ///
    /// [`CandidatePath`]: crate::CandidatePath
    fn is_match<'p>(&self, path: impl Into<CandidatePath<'p>>) -> bool;

    /// Gets the [matched text][`MatchedText`] in a [`CandidatePath`], if any.
    ///
    /// Returns `None` if the [`CandidatePath`] does not match the pattern. This is a logical
    /// operation and does **not** interact with the file system.
    ///
    /// [`CandidatePath`]: crate::CandidatePath
    /// [`MatchedText`]: crate::MatchedText
    fn matched<'p>(&self, path: &'p CandidatePath<'_>) -> Option<MatchedText<'p>>;

    /// Gets the depth variance of the pattern.
    fn depth(&self) -> DepthVariance;

    /// Gets the text variance of the pattern.
    ///
    /// # Examples
    ///
    /// Text variance can be used to determine if a pattern can be trivially represented by an
    /// equivalent native path using platform file system APIs.
    ///
    /// ```rust
    /// use std::path::Path;
    /// use wax::{Glob, Program};
    ///
    /// let glob = Glob::new("/home/user").unwrap();
    /// let text = glob.text();
    /// let path = text.as_path();
    ///
    /// assert_eq!(path, Some(Path::new("/home/user")));
    /// ```
    fn text(&self) -> TextVariance<'t>;

    /// Describes when the pattern matches candidate paths with a root.
    ///
    /// A glob expression that begins with a separator `/` has a root, but less trivial patterns
    /// like `/**` and `</root:1,>` can also root an expression. Some `Program` types may have
    /// indeterminate roots and may match both candidate paths with and without a root. In this
    /// case, this functions returns [`Sometimes`].
    ///
    /// [`Sometimes`]: crate::query::When::Sometimes
    fn has_root(&self) -> When;

    /// Describes when the pattern matches candidate paths exhaustively.
    ///
    /// A glob expression is exhaustive if, given a matched candidate path, it necessarily matches
    /// any and all sub-trees of that path. For example, given the pattern `.local/**` and the
    /// matched path `.local/bin`, any and all paths beneath `.local/bin` also match the pattern.
    ///
    /// Patterns that end with tree wildcards are more obviously exhaustive, but less trivial
    /// patterns like `<<?>/>`, `<doc/<*/:3,>>`, and `{a/**,b/**}` can also be exhaustive. Patterns
    /// with alternations may only be exhaustive for some matched paths. In this case, this
    /// function returns [`Sometimes`].
    ///
    /// [`Sometimes`]: crate::query::When::Sometimes
    fn is_exhaustive(&self) -> When;
}

/// General errors concerning [`Program`]s.
///
/// This is the most general error and each of its variants exposes a particular error type that
/// describes the details of its associated error condition. This error is not used in any Wax APIs
/// directly, but can be used to encapsulate the more specific errors that are.
///
/// # Examples
///
/// To encapsulate different errors in the Wax API behind a function, convert them into a
/// `GlobError` via `?`.
///
/// ```rust,no_run,ignore
/// use std::path::PathBuf;
/// use wax::{Glob, GlobError};
///
/// fn read_all(directory: impl Into<PathBuf>) -> Result<Vec<u8>, GlobError> {
///     let mut data = Vec::new();
///     let glob = Glob::new("**/*.data.bin")?;
///     for entry in glob.walk(directory) {
///         let entry = entry?;
///         // ...
///     }
///     Ok(data)
/// }
/// ```
///
/// [`Program`]: crate::Program
#[cfg_attr(feature = "miette", derive(Diagnostic))]
#[derive(Debug, Error)]
#[error(transparent)]
pub enum GlobError {
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Build(BuildError),
    #[cfg(feature = "walk")]
    #[cfg_attr(docsrs, doc(cfg(feature = "walk")))]
    #[cfg_attr(feature = "miette", diagnostic(code = "wax::glob::walk"))]
    Walk(WalkError),
}

impl From<BuildError> for GlobError {
    fn from(error: BuildError) -> Self {
        GlobError::Build(error)
    }
}

#[cfg(feature = "walk")]
impl From<WalkError> for GlobError {
    fn from(error: WalkError) -> Self {
        GlobError::Walk(error)
    }
}

// TODO: `Diagnostic` is implemented with macros for brevity and to ensure complete coverage of
//       features. However, this means that documentation does not annotate the implementation with
//       a feature flag requirement. If possible, perhaps in a later version of Rust, close this
//       gap.
/// Describes errors that occur when building a [`Program`] from a glob expression.
///
/// Glob expressions may fail to build if they cannot be parsed, violate rules, or cannot be
/// compiled. Parsing errors occur when a glob expression has invalid syntax. Patterns must also
/// follow rules as described in the [repository
/// documentation](https://github.com/olson-sean-k/wax/blob/master/README.md), which are designed
/// to avoid nonsense expressions and ambiguity. Lastly, compilation errors occur **only if the
/// size of the compiled program is too large** (all other compilation errors are considered
/// internal bugs and will panic).
///
/// When the `miette` feature is enabled, this and other error types implement the [`Diagnostic`]
/// trait. Due to a technical limitation, this may not be properly annotated in API documentation.
///
/// [`Diagnostic`]: miette::Diagnostic
/// [`Program`]: crate::Program
#[cfg_attr(feature = "miette", derive(Diagnostic))]
#[cfg_attr(feature = "miette", diagnostic(transparent))]
#[derive(Clone, Debug, Error)]
#[error(transparent)]
pub struct BuildError {
    kind: BuildErrorKind,
}

impl BuildError {
    /// Gets [`LocatedError`]s detailing the errors within a glob expression.
    ///
    /// This function returns an [`Iterator`] over the [`LocatedError`]s that detail where and why
    /// an error occurred when the error has associated [`Span`]s within a glob expression. For
    /// errors with no such associated information, the [`Iterator`] yields no items, such as
    /// compilation errors.
    ///
    /// # Examples
    ///
    /// [`LocatedError`]s can be used to provide information to users about which parts of a glob
    /// expression are associated with an error.
    ///
    /// ```rust
    /// use wax::Glob;
    ///
    /// // This glob expression violates rules. The error handling code prints details about the
    /// // alternation where the violation occurred.
    /// let expression = "**/{foo,**/bar,baz}";
    /// match Glob::new(expression) {
    ///     Ok(glob) => {
    ///         // ...
    ///     },
    ///     Err(error) => {
    ///         eprintln!("{}", error);
    ///         for error in error.locations() {
    ///             let (start, n) = error.span();
    ///             let fragment = &expression[start..][..n];
    ///             eprintln!("in sub-expression `{}`: {}", fragment, error);
    ///         }
    ///     },
    /// }
    /// ```
    ///
    /// [`Glob`]: crate::Glob
    /// [`Glob::partition`]: crate::Glob::partition
    /// [`Iterator`]: std::iter::Iterator
    /// [`LocatedError`]: crate::query::LocatedError
    /// [`Span`]: crate::query::Span
    pub fn locations(&self) -> impl Iterator<Item = &dyn LocatedError> {
        let locations: Vec<_> = match &self.kind {
            BuildErrorKind::Parse(error) => error
                .locations()
                .iter()
                .map(|location| location as &dyn LocatedError)
                .collect(),
            BuildErrorKind::Rule(error) => error
                .locations()
                .iter()
                .map(|location| location as &dyn LocatedError)
                .collect(),
            _ => vec![],
        };
        locations.into_iter()
    }
}

impl From<BuildErrorKind> for BuildError {
    fn from(kind: BuildErrorKind) -> Self {
        BuildError { kind }
    }
}

impl From<CompileError> for BuildError {
    fn from(error: CompileError) -> Self {
        BuildError {
            kind: BuildErrorKind::Compile(error),
        }
    }
}

impl From<Infallible> for BuildError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl<'t> From<ParseError<'t>> for BuildError {
    fn from(error: ParseError<'t>) -> Self {
        BuildError {
            kind: BuildErrorKind::Parse(error.into_owned()),
        }
    }
}

impl<'t> From<RuleError<'t>> for BuildError {
    fn from(error: RuleError<'t>) -> Self {
        BuildError {
            kind: BuildErrorKind::Rule(error.into_owned()),
        }
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
#[cfg_attr(feature = "miette", derive(Diagnostic))]
enum BuildErrorKind {
    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Compile(CompileError),
    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Parse(ParseError<'static>),
    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Rule(RuleError<'static>),
}

/// Path that can be matched against a [`Program`].
///
/// `CandidatePath`s are always UTF-8 encoded. On some platforms this requires a lossy conversion
/// that uses Unicode replacement codepoints `�` whenever a part of a path cannot be represented as
/// valid UTF-8 (such as Windows). This means that some byte sequences cannot be matched, though
/// this is uncommon in practice.
///
/// [`Program`]: crate::Program
#[derive(Clone)]
pub struct CandidatePath<'b> {
    text: Cow<'b, str>,
}

impl<'b> CandidatePath<'b> {
    /// Clones any borrowed data into an owning instance.
    pub fn into_owned(self) -> CandidatePath<'static> {
        CandidatePath {
            text: self.text.into_owned().into(),
        }
    }
}

impl AsRef<str> for CandidatePath<'_> {
    fn as_ref(&self) -> &str {
        self.text.as_ref()
    }
}

impl Debug for CandidatePath<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.text)
    }
}

impl Display for CandidatePath<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl<'b> From<&'b OsStr> for CandidatePath<'b> {
    fn from(text: &'b OsStr) -> Self {
        CandidatePath {
            text: text.to_string_lossy(),
        }
    }
}

impl<'b> From<&'b Path> for CandidatePath<'b> {
    fn from(path: &'b Path) -> Self {
        CandidatePath::from(path.as_os_str())
    }
}

impl<'b> From<&'b str> for CandidatePath<'b> {
    fn from(text: &'b str) -> Self {
        CandidatePath { text: text.into() }
    }
}

/// Program that can be matched against paths and directory trees.
///
/// `Glob`s are constructed from strings called glob expressions that resemble Unix paths
/// consisting of nominal components delimited by separators. Glob expressions support various
/// patterns that match and capture specified text in a path. These patterns can be used to
/// logically match individual paths and to semantically match and walk directory trees.
///
/// # Examples
///
/// A `Glob` can be used to determine if a path matches a pattern via the [`Program`] trait.
///
/// ```rust
/// use wax::{Glob, Program};
///
/// let glob = Glob::new("*.png").unwrap();
/// assert!(glob.is_match("apple.png"));
/// ```
///
/// Patterns form captures, which can be used to isolate matching sub-text.
///
/// ```rust
/// use wax::{CandidatePath, Glob, Program};
///
/// let glob = Glob::new("**/{*.{go,rs}}").unwrap();
/// let candidate = CandidatePath::from("src/lib.rs");
/// assert_eq!("lib.rs", glob.matched(&candidate).unwrap().get(2).unwrap());
/// ```
///
/// To match a `Glob` against a directory tree, the [`walk`] function can be used to get an
/// iterator over matching paths.
///
/// ```rust,no_run,ignore
/// use wax::walk::Entry;
/// use wax::Glob;
///
/// let glob = Glob::new("**/*.(?i){jpg,jpeg}").unwrap();
/// for entry in glob.walk("./Pictures") {
///     let entry = entry.unwrap();
///     println!("JPEG: {:?}", entry.path());
/// }
/// ```
///
/// [`Program`]: crate::Program
/// [`walk`]: crate::Glob::walk
#[derive(Clone, Debug)]
pub struct Glob<'t> {
    tree: Checked<Tokenized<'t, ExpressionMetadata>>,
    program: Regex,
}

impl<'t> Glob<'t> {
    // TODO: Document pattern syntax in the crate documentation and refer to it here.
    /// Constructs a [`Glob`] from a glob expression.
    ///
    /// A glob expression is UTF-8 encoded text that resembles a Unix path consisting of nominal
    /// components delimited by separators and patterns that can be matched against native paths.
    ///
    /// # Errors
    ///
    /// Returns an error if the glob expression fails to build. See [`BuildError`].
    ///
    /// [`Glob`]: crate::Glob
    /// [`BuildError`]: crate::BuildError
    pub fn new(expression: &'t str) -> Result<Self, BuildError> {
        let tree = parse_and_check(expression)?;
        let program = Glob::compile::<Tokenized<_>>(tree.as_ref())?;
        Ok(Glob { tree, program })
    }

    // TODO: Describe what an empty glob is. In particular, define what it does and does not match.
    pub fn empty() -> Self {
        Glob::new("").expect("failed to build empty glob")
    }

    // TODO: Describe what a tree glob is.
    pub fn tree() -> Self {
        Glob::new("**").expect("failed to build tree glob")
    }

    // TODO: Describe why and when the `Glob` postfix is `None`.
    /// Partitions a [`Glob`] into an invariant [`PathBuf`] prefix and variant [`Glob`] postfix.
    ///
    /// The invariant prefix contains no glob patterns nor other variant components and therefore
    /// can be interpreted as a native path. The [`Glob`] postfix is variant and contains the
    /// remaining components that follow the prefix. For example, the glob expression
    /// `.local/**/*.log` would produce the path `.local` and glob `**/*.log`. It is possible for
    /// either partition to be empty.
    ///
    /// Literal components may be considered variant if they contain characters with casing and the
    /// configured case sensitivity differs from the target platform's file system. For example,
    /// the case-insensitive literal expression `(?i)photos` is considered variant on Unix and
    /// invariant on Windows, because the literal `photos` resolves differently in Unix file system
    /// APIs.
    ///
    /// Partitioning a [`Glob`] allows any invariant prefix to be used as a native path to
    /// establish a working directory or to interpret semantic components that are not recognized
    /// by globs, such as parent directory `..` components.
    ///
    /// Partitioned [`Glob`]s are never rooted. If the glob expression has a root component, then
    /// it is always included in the invariant [`PathBuf`] prefix.
    ///
    /// # Examples
    ///
    /// To match paths against a [`Glob`] while respecting semantic components, the invariant
    /// prefix and candidate path can be canonicalized. The following example canonicalizes both
    /// the working directory joined with the prefix as well as the candidate path and then
    /// attempts to match the [`Glob`] if the candidate path contains the prefix.
    ///
    /// ```rust,no_run
    /// use dunce; // Avoids UNC paths on Windows.
    /// use std::path::Path;
    /// use wax::{Glob, Program};
    ///
    /// let path: &Path = /* ... */ // Candidate path.
    /// # Path::new("");
    ///
    /// let directory = Path::new("."); // Working directory.
    /// let (prefix, glob) = Glob::new("../../src/**").unwrap().partition();
    /// let prefix = dunce::canonicalize(directory.join(&prefix)).unwrap();
    /// if dunce::canonicalize(path)
    ///     .unwrap()
    ///     .strip_prefix(&prefix)
    ///     .ok()
    ///     .zip(glob)
    ///     .map_or(false, |(path, glob)| glob.is_match(path))
    /// {
    ///     // ...
    /// }
    /// ```
    ///
    /// [`Glob`]: crate::Glob
    /// [`ParseError`]: crate::ParseError
    /// [`PathBuf`]: std::path::PathBuf
    /// [`RuleError`]: crate::RuleError
    /// [`walk`]: crate::Glob::walk
    pub fn partition(self) -> (PathBuf, Option<Self>) {
        let Glob { tree, .. } = self;
        let (prefix, tree) = tree.partition();
        (
            prefix,
            tree.map(|tree| {
                let program = Glob::compile::<Tokenized<_>>(tree.as_ref())
                    .expect("failed to compile partitioned glob");
                Glob { tree, program }
            }),
        )
    }

    // TODO: Describe what an empty glob is and how this relates to `Glob::partition`.
    pub fn partition_or_empty(self) -> (PathBuf, Self) {
        let (prefix, glob) = self.partition();
        (prefix, glob.unwrap_or_else(Glob::empty))
    }

    // TODO: Describe what a tree glob is and how this relates to `Glob::partition`.
    pub fn partition_or_tree(self) -> (PathBuf, Self) {
        let (prefix, glob) = self.partition();
        (prefix, glob.unwrap_or_else(Glob::tree))
    }

    /// Clones any borrowed data into an owning instance.
    ///
    /// # Examples
    ///
    /// `Glob`s borrow data in the corresponding glob expression. To move a `Glob` beyond the scope
    /// of a glob expression, clone the data with this function.
    ///
    /// ```rust
    /// use wax::{BuildError, Glob};
    ///
    /// fn local() -> Result<Glob<'static>, BuildError> {
    ///     let expression = String::from("**/*.txt");
    ///     Glob::new(&expression).map(Glob::into_owned)
    /// }
    /// ```
    pub fn into_owned(self) -> Glob<'static> {
        let Glob { tree, program } = self;
        Glob {
            tree: tree.into_owned(),
            program,
        }
    }

    /// Gets metadata for capturing sub-expressions.
    ///
    /// This function returns an iterator over capturing tokens, which describe the index and
    /// location of sub-expressions that capture [matched text][`MatchedText`]. For example, in the
    /// expression `src/**/*.rs`, both `**` and `*` form captures.
    ///
    /// [`MatchedText`]: crate::MatchedText
    pub fn captures(&self) -> impl '_ + Clone + Iterator<Item = CapturingToken> {
        self.tree
            .as_ref()
            .as_token()
            .concatenation()
            .iter()
            .filter(|token| token.is_capturing())
            .enumerate()
            .map(|(index, token)| CapturingToken::new(index + 1, *token.annotation()))
    }

    /// Returns `true` if the glob has literals that have non-nominal semantics on the target
    /// platform.
    ///
    /// The most notable semantic literals are the relative path components `.` and `..`, which
    /// refer to a current and parent directory on Unix and Windows operating systems,
    /// respectively. These are interpreted as literals in glob expressions, and so only logically
    /// match paths that contain these exact nominal components (semantic meaning is lost).
    ///
    /// See [`Glob::partition`].
    ///
    /// [`Glob::partition`]: crate::Glob::partition
    pub fn has_semantic_literals(&self) -> bool {
        self.tree
            .as_ref()
            .as_token()
            .literals()
            .any(|(_, literal)| literal.is_semantic_literal())
    }

    pub fn is_empty(&self) -> bool {
        self.tree.as_ref().as_token().is_empty()
    }

    fn compile<T>(tree: impl Borrow<T>) -> Result<Regex, CompileError>
    where
        T: ConcatenationTree<'t>,
    {
        encode::compile(tree)
    }
}

impl Display for Glob<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.tree.as_ref().expression())
    }
}

impl FromStr for Glob<'static> {
    type Err = BuildError;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        Glob::new(expression).map(Glob::into_owned)
    }
}

impl<'t> Pattern<'t> for Glob<'t> {
    type Tokens = Tokenized<'t, ExpressionMetadata>;
    type Error = Infallible;
}

impl<'t> Program<'t> for Glob<'t> {
    fn is_match<'p>(&self, path: impl Into<CandidatePath<'p>>) -> bool {
        let path = path.into();
        self.program.is_match(path.as_ref())
    }

    fn matched<'p>(&self, path: &'p CandidatePath<'_>) -> Option<MatchedText<'p>> {
        self.program.captures(path.as_ref()).map(From::from)
    }

    fn depth(&self) -> DepthVariance {
        self.tree.as_ref().as_token().variance().into()
    }

    fn text(&self) -> TextVariance<'t> {
        self.tree.as_ref().as_token().variance().into()
    }

    fn has_root(&self) -> When {
        self.tree.as_ref().as_token().has_root()
    }

    fn is_exhaustive(&self) -> When {
        self.tree.as_ref().as_token().is_exhaustive()
    }
}

impl<'t> TryFrom<&'t str> for Glob<'t> {
    type Error = BuildError;

    fn try_from(expression: &'t str) -> Result<Self, Self::Error> {
        Glob::new(expression)
    }
}

/// Combinator that matches any of its component [`Program`]s.
///
/// An instance of `Any` is constructed using the [`any`] function, which combines multiple
/// [`Program`]s for more ergonomic and efficient matching.
///
/// [`any`]: crate::any
/// [`Program`]: crate::Program
#[derive(Clone, Debug)]
pub struct Any<'t> {
    tree: Checked<Token<'t, ()>>,
    program: Regex,
}

impl<'t> Any<'t> {
    fn compile(token: &Token<'t, ()>) -> Result<Regex, CompileError> {
        encode::compile::<Token<_>>(token)
    }
}

impl<'t> Pattern<'t> for Any<'t> {
    type Tokens = Token<'t, ()>;
    type Error = Infallible;
}

impl<'t> Program<'t> for Any<'t> {
    fn is_match<'p>(&self, path: impl Into<CandidatePath<'p>>) -> bool {
        let path = path.into();
        self.program.is_match(path.as_ref())
    }

    fn matched<'p>(&self, path: &'p CandidatePath<'_>) -> Option<MatchedText<'p>> {
        self.program.captures(path.as_ref()).map(From::from)
    }

    fn depth(&self) -> DepthVariance {
        self.tree.as_ref().as_token().variance().into()
    }

    fn text(&self) -> TextVariance<'t> {
        self.tree.as_ref().as_token().variance().into()
    }

    fn has_root(&self) -> When {
        self.tree.as_ref().as_token().has_root()
    }

    fn is_exhaustive(&self) -> When {
        self.tree.as_ref().as_token().is_exhaustive()
    }
}

// TODO: It may be useful to use dynamic dispatch via trait objects instead. This would allow for a
//       variety of types to be composed in an `any` call and would be especially useful if
//       additional combinators are introduced.
/// Constructs a combinator that matches if any of its input [`Pattern`]s match.
///
/// This function accepts an [`IntoIterator`] with items that implement [`Pattern`], such as
/// [`Glob`] and `&str`. The output [`Any`] implements [`Program`] by matching its component
/// [`Program`]s. [`Any`] is often more ergonomic and efficient than matching individually against
/// multiple [`Program`]s.
///
/// [`Any`] groups all captures and therefore only exposes the complete text of a match. It is not
/// possible to index a particular capturing token in the component patterns. Combinators only
/// support logical matching and cannot be used to semantically match (walk) a directory tree.
///
/// # Examples
///
/// To match a path against multiple patterns, the patterns can first be combined into an [`Any`].
///
/// ```rust
/// use wax::{Glob, Program};
///
/// let any = wax::any([
///     "src/**/*.rs",
///     "tests/**/*.rs",
///     "doc/**/*.md",
///     "pkg/**/PKGBUILD",
/// ])
/// .unwrap();
/// assert!(any.is_match("src/lib.rs"));
/// ```
///
/// [`Glob`]s and other compiled [`Program`]s can also be composed into an [`Any`].
///
/// ```rust
/// use wax::{Glob, Program};
///
/// let red = Glob::new("**/red/**/*.txt").unwrap();
/// let blue = Glob::new("**/*blue*.txt").unwrap();
/// assert!(wax::any([red, blue]).unwrap().is_match("red/potion.txt"));
/// ```
///
/// This function can only combine patterns of the same type, but intermediate combinators can be
/// used to combine different types into a single combinator.
///
/// ```rust
/// use wax::{Glob, Program};
///
/// # fn fallible() -> Result<(), wax::BuildError> {
/// let glob = Glob::new("**/*.txt")?;
///
/// // ...
///
/// #[rustfmt::skip]
/// let any = wax::any([
///     wax::any([glob]),
///     wax::any([
///         "**/*.pdf",
///         "**/*.tex",
///     ]),
/// ])?;
/// assert!(any.is_match("doc/lattice.tex"));
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if any of the inputs fail to build. If the inputs are a compiled [`Program`]
/// type such as [`Glob`], then this only occurs if the compiled program is too large.
///
/// [`Any`]: crate::Any
/// [`Glob`]: crate::Glob
/// [`IntoIterator`]: std::iter::IntoIterator
/// [`Pattern`]: crate::Pattern
/// [`Program`]: crate::Program
pub fn any<'t, I>(patterns: I) -> Result<Any<'t>, BuildError>
where
    I: IntoIterator,
    I::Item: Pattern<'t>,
{
    let tree = Checked::any(
        patterns
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)?,
    );
    let program = Any::compile(tree.as_ref())?;
    Ok(Any { tree, program })
}

// TODO: This function blindly escapes meta-characters, even if they are already escaped. Ignore
//       escaped meta-characters in the input.
/// Escapes text as a literal glob expression.
///
/// This function escapes any and all meta-characters in the given string, such that all text is
/// interpreted as a literal or separator when read as a glob expression.
///
/// # Examples
///
/// This function can be used to escape opaque strings, such as a string obtained from a user that
/// must be interpreted literally.
///
/// ```rust
/// use wax::Glob;
///
/// // An opaque file name that this code does not construct.
/// let name: String = {
///     /* ... */
///     # String::from("file.txt")
/// };
///
/// // Do not allow patterns in `name`.
/// let expression = format!("{}{}", "**/", wax::escape(&name));
/// if let Ok(glob) = Glob::new(&expression) { /* ... */ }
/// ```
///
/// Sometimes part of a path contains numerous meta-characters. This function can be used to
/// reliably escape them while making the unescaped part of the expression a bit easier to read.
///
/// ```rust
/// use wax::Glob;
///
/// let expression = format!("{}{}", "logs/**/", wax::escape("ingest[01](L).txt"));
/// let glob = Glob::new(&expression).unwrap();
/// ```
// It is possible to call this function using a mutable reference, which may appear to mutate the
// parameter in place.
#[must_use]
pub fn escape(unescaped: &str) -> Cow<'_, str> {
    const ESCAPE: char = '\\';

    if unescaped.chars().any(is_meta_character) {
        let mut escaped = String::new();
        for x in unescaped.chars() {
            if is_meta_character(x) {
                escaped.push(ESCAPE);
            }
            escaped.push(x);
        }
        escaped.into()
    }
    else {
        unescaped.into()
    }
}

// TODO: Is it possible for `:` and `,` to be contextual meta-characters?
/// Returns `true` if the given character is a meta-character.
///
/// This function does **not** return `true` for contextual meta-characters that may only be
/// escaped in particular contexts, such as hyphens `-` in character class expressions. To detect
/// these characters, use [`is_contextual_meta_character`].
///
/// [`is_contextual_meta_character`]: crate::is_contextual_meta_character
pub const fn is_meta_character(x: char) -> bool {
    matches!(
        x,
        '?' | '*' | '$' | ':' | '<' | '>' | '(' | ')' | '[' | ']' | '{' | '}' | ','
    )
}

/// Returns `true` if the given character is a contextual meta-character.
///
/// Contextual meta-characters may only be escaped in particular contexts, such as hyphens `-` in
/// character class expressions. Elsewhere, they are interpreted as literals. To detect
/// non-contextual meta-characters, use [`is_meta_character`].
///
/// [`is_meta_character`]: crate::is_meta_character
pub const fn is_contextual_meta_character(x: char) -> bool {
    matches!(x, '-')
}

fn parse_and_check(
    expression: &str,
) -> Result<Checked<Tokenized<'_, ExpressionMetadata>>, BuildError> {
    let tokenized = token::parse(expression)?;
    let checked = rule::check(tokenized)?;
    Ok(checked)
}

fn minmax<T>(lhs: T, rhs: T) -> [T; 2]
where
    T: Ord,
{
    use Ordering::{Equal, Greater, Less};

    match lhs.cmp(&rhs) {
        Equal | Less => [lhs, rhs],
        Greater => [rhs, lhs],
    }
}

#[cfg(test)]
pub mod harness {
    use expect_macro::expect;
    use itertools::Itertools;
    use std::fmt::Debug;
    use std::path::{Path, PathBuf};

    use crate::{Any, BuildError, CandidatePath, Glob, MatchedText, Pattern, Program};

    pub trait PartitionNonEmpty<'t>: Sized {
        fn assert_partition_non_empty(self) -> (PathBuf, Glob<'t>);
    }

    impl<'t> PartitionNonEmpty<'t> for Glob<'t> {
        fn assert_partition_non_empty(self) -> (PathBuf, Glob<'t>) {
            let (prefix, glob) = self.partition();
            (
                prefix,
                glob.expect("`Glob::partition` is `None`, but expected `Some`"),
            )
        }
    }

    pub fn assert_escaped_text_eq(unescaped: &str, escaped: &str, expected: &str) {
        assert!(
            escaped == expected,
            "unexpected output from `escape`:\
            \n\tunescaped: `{}`\n\tescaped: `{}`\n\texpected: `{}`",
            unescaped,
            escaped,
            expected,
        );
    }

    pub fn assert_new_glob_is_ok(expression: &str) -> Glob<'_> {
        let result = Glob::new(expression);
        let error = result.as_ref().err().cloned();
        expect!(
            result,
            "`Glob::new` is `Err`, but expected `Ok`: in expression: `{}`: error: \"{}\"",
            expression,
            error.unwrap(),
        )
    }

    pub fn assert_new_glob_is_err(expression: &str) -> BuildError {
        expect!(
            Glob::new(expression).err(),
            "`Glob::new` is `Ok`, but expected `Err`: in expression: `{}`",
            expression,
        )
    }

    pub fn assert_match_program_with<'t, T, F>(
        program: impl Program<'t>,
        candidate: impl Into<CandidatePath<'t>>,
        f: F,
    ) -> T
    where
        F: FnOnce(Option<MatchedText<'static>>) -> T,
    {
        let candidate = candidate.into();
        f(program.matched(&candidate).map(MatchedText::into_owned))
    }

    pub fn assert_matched_is_some(matched: Option<MatchedText<'_>>) -> MatchedText<'_> {
        matched.expect("matched text is `None`, but expected `Some`")
    }

    pub fn assert_matched_is_none(matched: Option<MatchedText<'_>>) {
        assert!(
            matched.is_none(),
            "matched text is `Some`, but expected `None`"
        );
    }

    pub fn assert_matched_has_text(
        expected: impl IntoIterator<Item = (usize, &'static str)>,
    ) -> impl FnOnce(Option<MatchedText<'_>>) {
        move |matched| {
            let mut has_matched_text = true;
            let matched = assert_matched_is_some(matched);
            let message = expected
                .into_iter()
                .filter_map(|(index, text)| match matched.get(index) {
                    Some(matched) if matched == text => None,
                    matched => Some(format!(
                        "\tmatched text at capture {} is `{:?}`, but expected `{}`",
                        index, matched, text,
                    )),
                })
                .inspect(|_| {
                    has_matched_text = false;
                })
                .join("\n");
            assert!(has_matched_text, "unexpected matched text:\n{}", message);
        }
    }

    pub fn assert_any_is_ok<'t, I>(patterns: I) -> Any<'t>
    where
        I: Clone + IntoIterator,
        I::Item: Debug + Pattern<'t>,
    {
        match crate::any(patterns.clone()) {
            Ok(any) => any,
            Err(error) => {
                panic!(
                    "`any` is `Err`, but expected `Ok`: error: \"{}\":{}",
                    error,
                    patterns
                        .into_iter()
                        .map(|pattern| format!("\n\tpattern: `{:?}`", pattern))
                        .join(""),
                )
            },
        }
    }

    pub fn assert_any_is_err<'t, I>(patterns: I) -> BuildError
    where
        I: Clone + IntoIterator,
        I::Item: Debug + Pattern<'t>,
    {
        match crate::any(patterns.clone()) {
            Ok(_) => {
                panic!(
                    "`any` is `Ok`, but expected `Err`:{}",
                    patterns
                        .into_iter()
                        .map(|pattern| format!("\n\tpattern: `{:?}`", pattern))
                        .join(""),
                )
            },
            Err(error) => error,
        }
    }

    pub fn assert_partitioned_has_prefix_and_is_match<'p>(
        partitioned: (PathBuf, Glob<'_>),
        expected: (impl AsRef<Path>, impl Into<CandidatePath<'p>>),
    ) -> MatchedText<'static> {
        let expected = (expected.0.as_ref(), expected.1.into());
        let (prefix, glob) = partitioned;
        assert!(
            prefix == expected.0,
            "partitioned prefix is `{}`, but expected `{}`: in `Glob`: `{}`",
            prefix.display(),
            expected.0.display(),
            glob,
        );
        assert_match_program_with(glob, expected.1, assert_matched_is_some)
    }

    pub fn assert_partitioned_has_prefix_and_expression<'t>(
        partitioned: (PathBuf, Glob<'t>),
        expected: (impl AsRef<Path>, &str),
    ) -> (PathBuf, Glob<'t>) {
        let (prefix, glob) = partitioned;
        let expected = (expected.0.as_ref(), expected.1);
        assert!(
            prefix == expected.0,
            "partitioned prefix is `{}`, but expected `{}`",
            prefix.display(),
            expected.0.display(),
        );
        let expression = glob.to_string();
        assert!(
            expression == expected.1,
            "partitioned glob has expression `{}`, but expected `{}`",
            expression,
            expected.1,
        );
        (prefix, glob)
    }
}

// TODO: Many of these tests need not be separated into distinct test functions, such as
//       `new_glob_{}_is_ok`. Consider consolidating such functions using case labels for clarity.
//       Finding the right balance may be tricky.
// TODO: Most of these tests operate against public APIs like `Glob::new`, `Glob::partition`, and
//       `crate::any`. Consider moving these into a `tests` directory as integration tests. Perhaps
//       more importantly, some of these tests ought to have more local counterparts that directly
//       test the parser, token tree, and encoder.
// TODO: Construct paths from components in tests. In practice, using string literals works, but is
//       technically specific to platforms that support `/` as a separator.
#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::fmt::Debug;

    use crate::diagnostics::Span;
    use crate::harness::{self, PartitionNonEmpty};
    use crate::{BuildError, BuildErrorKind, Glob, MatchedText, Pattern, Program};

    #[rstest]
    #[case::empty("", "")]
    #[case::all("?*$:<>()[]{},", "\\?\\*\\$\\:\\<\\>\\(\\)\\[\\]\\{\\}\\,")]
    #[case("record[D00,00].txt", "record\\[D00\\,00\\].txt")]
    #[case::whitespace("Do You Remember Love?.mp4", "Do You Remember Love\\?.mp4")]
    #[case::cjk("左{}右", "左\\{\\}右")]
    #[case::cjk("*中*", "\\*中\\*")]
    fn escape_with_unescaped_text_is_escaped(#[case] unescaped: &str, #[case] expected: &str) {
        let escaped = crate::escape(unescaped);
        harness::assert_escaped_text_eq(unescaped, escaped.as_ref(), expected);
    }

    // TODO: See `escaped`.
    //#[rstest]
    //#[case("\\?", "\\?")]
    //fn escape_with_escaped_text_is_not_escaped_again(
    //    #[case] unescaped: &str,
    //    #[case] expected: &str,
    //) {
    //    let escaped = crate::escape(unescaped);
    //    harness::assert_escaped_text_eq(unescaped, escaped.as_ref(), expected);
    //}

    #[rstest]
    #[case("a/[xy]")]
    #[case("a/[x-z]")]
    #[case("a/[xyi-k]")]
    #[case("a/[i-kxy]")]
    #[case("a/[!xy]")]
    #[case("a/[!x-z]")]
    #[case("a/[xy]b/c")]
    fn new_glob_with_class_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("a/\\[a-z\\]/c")]
    #[case("a/[\\[]/c")]
    #[case("a/[\\]]/c")]
    #[case("a/[a\\-z]/c")]
    fn new_glob_with_literal_escaped_class_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("")]
    #[case("a")]
    #[case("a/b/c")]
    #[case("abc")]
    fn new_glob_with_literal_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("?")]
    #[case("a/?")]
    #[case("?a")]
    #[case("a?")]
    #[case("a?b")]
    #[case("??a??b??")]
    #[case("/?")]
    fn new_glob_with_exactly_one_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("?*")]
    #[case("*?")]
    #[case("*/?")]
    #[case("?*?")]
    #[case("/?*")]
    #[case("?$")]
    fn new_glob_with_exactly_one_and_zom_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("*")]
    #[case("a/*")]
    #[case("*a")]
    #[case("a*")]
    #[case("a*b")]
    #[case("/*")]
    fn new_glob_with_eager_zom_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("$")]
    #[case("a/$")]
    #[case("$a")]
    #[case("a$")]
    #[case("a$b")]
    #[case("/$")]
    fn new_glob_with_lazy_zom_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("**")]
    #[case("**/")]
    #[case("/**")]
    #[case("**/a")]
    #[case("a/**")]
    #[case("**/a/**/b/**")]
    #[case("{**/a,b/c}")]
    #[case("{a/b,c/**}")]
    #[case("<**/a>")]
    #[case("<a/**>")]
    fn new_glob_with_tree_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("a/b\\?/c")]
    #[case("a/b\\$/c")]
    #[case("a/b\\*/c")]
    #[case("a/b\\*\\*/c")]
    fn new_glob_with_literal_escaped_wildcard_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("a/b[?]/c")]
    #[case("a/b[$]/c")]
    #[case("a/b[*]/c")]
    #[case("a/b[*][*]/c")]
    fn new_glob_with_class_escaped_wildcard_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("a/{x?z,y$}b*")]
    #[case("a/{???,x$y,frob}b*")]
    #[case("a/{???,x$y,frob}b*")]
    #[case("a/{???,{x*z,y$}}b*")]
    #[case("a{/**/b/,/b/**/}ca{t,b/**}")]
    fn new_glob_with_alternation_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("a/\\{\\}/c")]
    #[case("a/{x,y\\,,z}/c")]
    fn new_glob_with_literal_escaped_alternation_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("a/[{][}]/c")]
    #[case("a/{x,y[,],z}/c")]
    fn new_glob_with_class_escaped_alternation_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("<a:0,1>")]
    #[case("<a:0,>")]
    #[case("<a:2>")]
    #[case("<a:>")]
    #[case("<a>")]
    #[case("<a<b:0,>:0,>")]
    // Rooted repetitions are accepted if the lower bound is one or greater.
    #[case("</root:1,>")]
    #[case("<[!.]*/:0,>[!.]*")]
    fn new_glob_with_repetition_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case("(?i)a/b/c")]
    #[case("(?-i)a/b/c")]
    #[case("a/(?-i)b/c")]
    #[case("a/b/(?-i)c")]
    #[case("(?i)a/(?-i)b/(?i)c")]
    fn new_glob_with_flag_is_ok(#[case] expression: &str) {
        harness::assert_new_glob_is_ok(expression);
    }

    #[rstest]
    #[case([
        "src/**/*.rs",
        "doc/**/*.md",
        "pkg/**/PKGBUILD",
    ])]
    #[case([
        Glob::new("src/**/*.rs"),
        Glob::new("doc/**/*.md"),
        Glob::new("pkg/**/PKGBUILD"),
    ])]
    #[case([
        crate::any(["a/b", "c/d"]),
        crate::any(["{e,f,g}", "{h,i}"]),
    ])]
    #[case::overlapping_trees(["/root", "relative"])]
    fn any_is_ok<'t, I>(#[case] patterns: I)
    where
        I: Clone + IntoIterator,
        I::Item: Debug + Pattern<'t>,
    {
        let _ = harness::assert_any_is_ok(patterns);
    }

    #[rstest]
    #[case("//a")]
    #[case("a//b")]
    #[case("a/b//")]
    #[case("a//**")]
    #[case("{//}a")]
    #[case("{**//}")]
    fn new_glob_with_adjacent_separator_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("***")]
    #[case("****")]
    #[case("**/**")]
    #[case("a{**/**,/b}")]
    #[case("**/*/***")]
    #[case("**$")]
    #[case("**/$**")]
    #[case("{*$}")]
    #[case("<*$:1,>")]
    fn new_glob_with_adjacent_tree_and_zom_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("**a")]
    #[case("a**")]
    #[case("a**b")]
    #[case("a*b**")]
    #[case("**/**a/**")]
    fn new_glob_with_adjacent_tree_and_literal_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("**?")]
    #[case("?**")]
    #[case("?**?")]
    #[case("?*?**")]
    #[case("**/**?/**")]
    fn new_glob_with_adjacent_tree_and_exactly_one_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("a/[a-z-]/c")]
    #[case("a/[-a-z]/c")]
    #[case("a/[-]/c")]
    // Without special attention to escaping and character parsing, this could be mistakenly
    // interpreted as an empty range over the character `-`. This should be rejected.
    #[case("a/[---]/c")]
    #[case("a/[[]/c")]
    #[case("a/[]]/c")]
    fn new_glob_with_unescaped_meta_characters_in_class_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("*{okay,*}")]
    #[case("{okay,*}*")]
    #[case("${okay,*error}")]
    #[case("{okay,error*}$")]
    #[case("{*,okay}{okay,*}")]
    #[case("{okay,error*}{okay,*error}")]
    fn new_glob_with_adjacent_zom_over_alternation_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("/slash/{okay,/error}")]
    #[case("{okay,error/}/slash")]
    #[case("slash/{okay,/error/,okay}/slash")]
    #[case("{okay,error/}{okay,/error}")]
    // TODO: This is not really an adjacent boundary like the other test cases. Is there a better
    //       place for this?
    #[case("{**}")]
    #[case("slash/{**/error}")]
    #[case("{error/**}/slash")]
    #[case("slash/{okay/**,**/error}")]
    #[case("{**/okay,error/**}/slash")]
    #[case("{**/okay,prefix{error/**}}/slash")]
    #[case("{**/okay,slash/{**/error}}postfix")]
    #[case("{error/**}{okay,**/error")]
    fn new_glob_with_adjacent_boundary_over_alternation_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("{okay,/}")]
    #[case("{okay,/**}")]
    #[case("{okay,/error}")]
    #[case("{okay,/**/error}")]
    fn new_glob_with_rooted_alternation_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    // TODO: These are not really misordered bounds like the other test cases. Is there a better
    //       place for this?
    #[case("<a/:0,0>")]
    #[case("<a/:,1>")]
    #[case("<a/:1,0>")]
    #[case("<a/:10,1>")]
    #[case("<a/:10,9>")]
    fn new_glob_with_misordered_repetition_bounds_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("<*:0,>")]
    #[case("<a/*:0,>*")]
    #[case("*<*a:0,>")]
    fn new_glob_with_adjacent_zom_over_repetition_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("</:0,>")]
    #[case("</a/:0,>")]
    #[case("<a/:0,>/")]
    #[case("<**:0,>")]
    #[case("</**/a/**:0,>")]
    #[case("<a/**:0,>/")]
    #[case("/**</a:0,>")]
    fn new_glob_with_adjacent_boundary_over_repetition_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    // Rooted repetitions are rejected if their lower bound is zero; any other lower bound is
    // accepted.
    #[rstest]
    #[case("</root:0,>maybe")]
    #[case("</root>")]
    fn new_glob_with_sometimes_rooted_repetition_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("<a:65536>")]
    #[case("<long:16500>")]
    #[case("a<long:16500>b")]
    #[case("{<a:65536>,<long:16500>}")]
    fn new_glob_with_oversized_invariant_repetition_is_rule_err(#[case] expression: &str) {
        let error = harness::assert_new_glob_is_err(expression);
        assert!(
            matches!(
                error,
                BuildError {
                    kind: BuildErrorKind::Rule(_),
                    ..
                },
            ),
            "`Glob::new` is {:?}, but expected `RuleError`",
            error,
        );
    }

    #[rstest]
    #[case("<a*:1000000>")]
    fn new_glob_with_oversized_program_is_compile_err(#[case] expression: &str) {
        let error = harness::assert_new_glob_is_err(expression);
        assert!(
            matches!(
                error,
                BuildError {
                    kind: BuildErrorKind::Compile(_),
                    ..
                },
            ),
            "`Glob::new` is {:?}, but expected `CompileError`",
            error,
        );
    }

    #[rstest]
    #[case("(?)a")]
    #[case("(?-)a")]
    #[case("()a")]
    fn new_glob_with_incomplete_flag_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case("/(?i)/")]
    #[case("$(?i)$")]
    #[case("*(?i)*")]
    #[case("**(?i)?")]
    #[case("a(?i)**")]
    #[case("**(?i)a")]
    fn new_glob_with_adjacent_separator_and_zom_over_flag_is_err(#[case] expression: &str) {
        harness::assert_new_glob_is_err(expression);
    }

    #[rstest]
    #[case([
        "{a,b,c}",
        "{d,e}",
        "f/{g,/error,h}",
    ])]
    fn any_with_adjacent_boundary_is_err<'t, I>(#[case] patterns: I)
    where
        I: Clone + IntoIterator,
        I::Item: Debug + Pattern<'t>,
    {
        harness::assert_any_is_err(patterns);
    }

    #[rstest]
    #[case("", harness::assert_matched_has_text([(0, "")]))]
    #[case("abc", harness::assert_matched_is_none)]
    fn match_empty_glob<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(harness::assert_new_glob_is_ok(""), path, f);
    }

    #[rstest]
    #[case("a/b", harness::assert_matched_has_text([(0, "a/b")]))]
    #[case("aa/b", harness::assert_matched_is_none)]
    #[case("a/bb", harness::assert_matched_is_none)]
    #[case("a/b/c", harness::assert_matched_is_none)]
    fn match_invariant_glob<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(harness::assert_new_glob_is_ok("a/b"), path, f);
    }

    #[rstest]
    #[case("a/b", harness::assert_matched_has_text([(0, "a/b")]))]
    #[case("a/x/b", harness::assert_matched_has_text([(0, "a/x/b")]))]
    #[case("a/x/y/z/b", harness::assert_matched_has_text([(0, "a/x/y/z/b"), (1, "x/y/z/")]))]
    #[case("a", harness::assert_matched_is_none)]
    #[case("b/a", harness::assert_matched_is_none)]
    fn match_glob_with_tree<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(harness::assert_new_glob_is_ok("a/**/b"), path, f);
    }

    #[rstest]
    #[case(".ext", harness::assert_matched_has_text([(0, ".ext"), (2, "")]))]
    #[case("file.ext", harness::assert_matched_has_text([(0, "file.ext"), (2, "file")]))]
    #[case("a/b/file.ext", harness::assert_matched_has_text([
        (0, "a/b/file.ext"),
        (1, "a/b/"),
        (2, "file"),
    ]))]
    #[case("file", harness::assert_matched_is_none)]
    #[case("file.txt", harness::assert_matched_is_none)]
    fn match_glob_with_tree_and_zom<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(harness::assert_new_glob_is_ok("**/*.ext"), path, f);
    }

    #[rstest]
    #[case("prefix-file.ext", harness::assert_matched_has_text([
        (0, "prefix-file.ext"),
        (1, "prefix"),
        (2, "file"),
        (3, "ext"),
    ]))]
    #[case("a-b-c.ext", harness::assert_matched_has_text([
        (0, "a-b-c.ext"),
        (1, "a"),
        (2, "b-c"),
        (3, "ext"),
    ]))]
    #[case("file.ext", harness::assert_matched_is_none)]
    fn match_glob_with_eager_and_lazy_zom<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(harness::assert_new_glob_is_ok("$-*.*"), path, f);
    }

    #[rstest]
    #[case("a/x/file.ext", harness::assert_matched_has_text([
        (0, "a/x/file.ext"),
        (1, "x"),
        (2, "file.ext"),
    ]))]
    #[case("a/y/file.ext", harness::assert_matched_has_text([(1, "y")]))]
    #[case("a/i/file.ext", harness::assert_matched_has_text([(1, "i")]))]
    #[case("a/k/file.ext", harness::assert_matched_has_text([(1, "k")]))]
    #[case("a/j/file.ext", harness::assert_matched_has_text([(1, "j")]))]
    #[case("a/z/file.ext", harness::assert_matched_is_none)]
    fn match_glob_with_ascii_class<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(harness::assert_new_glob_is_ok("a/[xyi-k]/**"), path, f);
    }

    #[rstest]
    #[case("a/金/file.ext", harness::assert_matched_has_text([
        (0, "a/金/file.ext"),
        (1, "金"),
        (2, "file.ext"),
    ]))]
    #[case("a/銀/file.ext", harness::assert_matched_has_text([(1, "銀")]))]
    #[case("a/銅/file.ext", harness::assert_matched_is_none)]
    #[case("a/b/file.ext", harness::assert_matched_is_none)]
    fn match_glob_with_cjk_class<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(harness::assert_new_glob_is_ok("a/[金銀]/**"), path, f);
    }

    #[rstest]
    #[case("a/[/file.ext", harness::assert_matched_has_text([
        (0, "a/[/file.ext"),
        (1, "["),
        (2, "file.ext"),
    ]))]
    #[case("a/]/file.ext", harness::assert_matched_has_text([(1, "]")]))]
    #[case("a/-/file.ext", harness::assert_matched_has_text([(1, "-")]))]
    #[case("a/b/file.ext", harness::assert_matched_is_none)]
    fn match_glob_with_literal_escaped_class<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("a/[\\[\\]\\-]/**"),
            path,
            f,
        );
    }

    #[cfg(any(unix, windows))]
    #[rstest]
    #[case("", harness::assert_matched_is_none)]
    #[case("a", harness::assert_matched_is_none)]
    #[case("b", harness::assert_matched_is_none)]
    #[case("ab", harness::assert_matched_is_none)]
    #[case("a/b", harness::assert_matched_is_none)]
    fn match_glob_with_empty_class<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        // A class is empty if it only matches separators on the target platform. Such a class
        // never matches anything.
        harness::assert_match_program_with(harness::assert_new_glob_is_ok("a[/]b"), path, f);
    }

    #[rstest]
    #[case("a-c", harness::assert_matched_has_text([(0, "a-c"), (1, "-")]))]
    #[case("axc", harness::assert_matched_has_text([(0, "axc"), (1, "x")]))]
    #[case("a9c", harness::assert_matched_has_text([(0, "a9c"), (1, "9")]))]
    #[case("abc", harness::assert_matched_is_none)]
    #[case("a0c", harness::assert_matched_is_none)]
    #[case("a4c", harness::assert_matched_is_none)]
    #[case("a/c", harness::assert_matched_is_none)]
    fn match_glob_with_negated_class<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(harness::assert_new_glob_is_ok("a[!b0-4]c"), path, f);
    }

    #[rstest]
    #[case("a/xyzb/file.ext", harness::assert_matched_has_text([
        (0, "a/xyzb/file.ext"),
        (1, "xyz"),
        (2, "file.ext"),
    ]))]
    #[case("a/yb/file.ext", harness::assert_matched_has_text([
        (0, "a/yb/file.ext"),
        (1, "y"),
        (2, "file.ext"),
    ]))]
    #[case("a/xyz/file.ext", harness::assert_matched_is_none)]
    #[case("a/y/file.ext", harness::assert_matched_is_none)]
    #[case("a/xyzub/file.ext", harness::assert_matched_is_none)]
    fn match_glob_with_alternation<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("a/{x?z,y$}b/*"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("a/xyzb/file.ext", harness::assert_matched_has_text([
        (0, "a/xyzb/file.ext"),
        (1, "xyz"),
        (2, "file.ext"),
    ]))]
    #[case("a/xyz/file.ext", harness::assert_matched_is_none)]
    fn match_glob_with_nested_alternation<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("a/{y$,{x?z,?z}}b/*"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("prefix/a/b/c/postfix", harness::assert_matched_has_text([
        (0, "prefix/a/b/c/postfix"),
        (1, "/a/b/c"),
    ]))]
    #[case("prefix/a/c/postfix", harness::assert_matched_has_text([(1, "/a/c")]))]
    #[case("prefix/a/postfix", harness::assert_matched_has_text([(1, "/a")]))]
    #[case("prefix/a/b/postfix", harness::assert_matched_is_none)]
    fn match_glob_with_tree_in_alternation<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("prefix{/a,/b,/**/c}/postfix"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("log-000.txt", harness::assert_matched_has_text([(0, "log-000.txt"), (1, "000")]))]
    #[case("log-1970-01-01.txt", harness::assert_matched_has_text([
        (0, "log-1970-01-01.txt"),
        (1, "1970-01-01"),
    ]))]
    #[case("log-abc.txt", harness::assert_matched_is_none)]
    #[case("log-nope-no-no.txt", harness::assert_matched_is_none)]
    fn match_glob_with_repetition_in_alternation<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("log-{<[0-9]:3>,<[0-9]:4>-<[0-9]:2>-<[0-9]:2>}.txt"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("a/000000/file.ext", harness::assert_matched_has_text([
        (0, "a/000000/file.ext"),
        (1, "000000"),
        (2, "file.ext"),
    ]))]
    #[case("a/123456/file.ext", harness::assert_matched_has_text([
        (0, "a/123456/file.ext"),
        (1, "123456"),
        (2, "file.ext"),
    ]))]
    #[case("a/00000/file.ext", harness::assert_matched_is_none)]
    #[case("a/0000000/file.ext", harness::assert_matched_is_none)]
    #[case("a/bbbbbb/file.ext", harness::assert_matched_is_none)]
    fn match_glob_with_repetition<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("a/<[0-9]:6>/*"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("a/b/file.ext", harness::assert_matched_has_text([
        (0, "a/b/file.ext"),
        (1, "a/b/"),
        (2, "file.ext"),
    ]))]
    #[case(".a/b/file.ext", harness::assert_matched_is_none)]
    #[case("a/.b/file.ext", harness::assert_matched_is_none)]
    #[case("a/b/.file.ext", harness::assert_matched_is_none)]
    fn match_glob_with_negated_class_in_repetition<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("<[!.]*/><[!.]*:0,1>"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("log-000.txt", harness::assert_matched_has_text([(0, "log-000.txt"), (1, "-000")]))]
    #[case("log-123-456.txt", harness::assert_matched_has_text([
        (0, "log-123-456.txt"),
        (1, "-123-456"),
    ]))]
    #[case("log-abc.txt", harness::assert_matched_is_none)]
    #[case("log-123-456-789.txt", harness::assert_matched_is_none)]
    fn match_glob_with_nested_repetition<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("log<-<[0-9]:3>:1,2>.txt"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("a/file.ext", harness::assert_matched_has_text([
        (0, "a/file.ext"),
        (1, "a"),
        (2, "file.ext"),
    ]))]
    #[case("aaa/file.ext", harness::assert_matched_has_text([
        (0, "aaa/file.ext"),
        (1, "aaa"),
        (2, "file.ext"),
    ]))]
    #[case("b/file.ext", harness::assert_matched_has_text([(1, "b")]))]
    #[case("bbb/file.ext", harness::assert_matched_has_text([(1, "bbb")]))]
    #[case("file.ext", harness::assert_matched_is_none)]
    #[case("c/file.ext", harness::assert_matched_is_none)]
    fn match_glob_with_alternation_in_repetition<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("<{a,b}:1,>/**"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("/var/log/network.log", harness::assert_matched_has_text([
        (0, "/var/log/network.log"),
        (2, "var"),
        (3, "log/"),
        (4, "network"),
    ]))]
    #[case("/home/nobody/.var/network.log", harness::assert_matched_has_text([
        (0, "/home/nobody/.var/network.log"),
        // TODO: The match and capture behavior of `**` here seems not only to cross boundaries,
        //       but match only part of a component! Greedy or not, tree wildcards ought to operate
        //       exclusively on some number of **complete** components: 
        (1, "/home/nobody/."),
        (2, "var"),
        (4, "network"),
    ]))]
    #[case("./var/cron.log", harness::assert_matched_is_none)]
    #[case("mnt/var/log/cron.log", harness::assert_matched_is_none)]
    fn match_glob_with_rooted_tree<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("/**/{var,.var}/**/*.log"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("photos/flower.jpg", harness::assert_matched_has_text([
        (0, "photos/flower.jpg"),
        (2, "flower"),
        (3, "jpg"),
    ]))]
    #[case("photos/flower.JPEG", harness::assert_matched_has_text([(3, "JPEG")]))]
    #[case("Photos/flower.jpeg", harness::assert_matched_is_none)]
    fn match_glob_with_case_sensitivity_flag<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_new_glob_is_ok("(?-i)photos/**/*.(?i){jpg,jpeg}"),
            path,
            f,
        );
    }

    #[rstest]
    #[case("a(b)", harness::assert_matched_has_text([(0, "a(b)")]))]
    fn match_glob_with_literal_escaped_flag<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(harness::assert_new_glob_is_ok("a\\(b\\)"), path, f);
    }

    #[rstest]
    #[case("src/lib.rs", harness::assert_matched_has_text([(0, "src/lib.rs")]))]
    #[case("doc/api.md", harness::assert_matched_has_text([(0, "doc/api.md")]))]
    #[case(
        "pkg/arch/lib-git/PKGBUILD",
        harness::assert_matched_has_text([(0, "pkg/arch/lib-git/PKGBUILD")]),
    )]
    fn match_any<T, F>(#[case] path: &str, #[case] f: F)
    where
        F: FnOnce(Option<MatchedText<'_>>) -> T,
    {
        harness::assert_match_program_with(
            harness::assert_any_is_ok(["src/**/*.rs", "doc/**/*.md", "pkg/**/PKGBUILD"]),
            path,
            f,
        );
    }

    #[rstest]
    #[case::empty("", "", "")]
    #[case::prefixed_and_non_empty("a/b/x?z/*.ext", "a/b", "xyz/file.ext")]
    #[case::only_variant_wildcard("x?z/*.ext", "", "xyz/file.ext")]
    #[case::only_invariant_literal("a/b", "a/b", "")]
    #[case::variant_alternation("{x,z}/*.ext", "", "x/file.ext")]
    #[case::invariant_alternation("{a/b}/c", "a/b/c", "")]
    #[case::invariant_repetition("</a/b:3>/c", "/a/b/a/b/a/b/c", "")]
    #[case::literal_dots_and_tree("../**/*.ext", "..", "xyz/file.ext")]
    #[case::rooted_literal("/root/**/*.ext", "/root", "file.ext")]
    #[case::rooted_zom("/*/*.ext", "/", "xyz/file.ext")]
    #[case::rooted_tree("/**/*.ext", "/", "file.ext")]
    fn partition_glob_has_prefix_and_is_match(
        #[case] expression: &str,
        #[case] prefix: &str,
        #[case] path: &str,
    ) {
        harness::assert_partitioned_has_prefix_and_is_match(
            harness::assert_new_glob_is_ok(expression).partition_or_empty(),
            (prefix, path),
        );
    }

    #[rstest]
    #[case("/root/file.ext", "/root/file.ext")]
    #[case("<a:3>/file.ext", "aaa/file.ext")]
    fn partition_invariant_glob_has_prefix_and_empty_expression(
        #[case] expression: &str,
        #[case] prefix: &str,
    ) {
        harness::assert_partitioned_has_prefix_and_expression(
            harness::assert_new_glob_is_ok(expression).partition_or_empty(),
            (prefix, ""),
        );
    }

    #[rstest]
    #[case("**/file.ext", "", "**/file.ext")]
    #[case("/root/**/file.ext", "/root", "**/file.ext")]
    #[case("/root/**", "/root", "**")]
    fn partition_variant_glob_has_prefix_and_non_empty_expression(
        #[case] expression: &str,
        #[case] prefix: &str,
        #[case] postfix: &str,
    ) {
        harness::assert_partitioned_has_prefix_and_expression(
            harness::assert_new_glob_is_ok(expression).assert_partition_non_empty(),
            (prefix, postfix),
        );
    }

    #[rstest]
    #[case::empty("", "", "")]
    fn repartition_invariant_glob_has_empty_prefix_and_idempotent_expression(
        #[case] expression: &str,
        #[case] prefix: &str,
        #[case] postfix: &str,
    ) {
        let (_, glob) = harness::assert_partitioned_has_prefix_and_expression(
            harness::assert_new_glob_is_ok(expression).partition_or_empty(),
            (prefix, postfix),
        );
        harness::assert_partitioned_has_prefix_and_expression(
            glob.partition_or_empty(),
            ("", postfix),
        );
    }

    #[rstest]
    #[case("/root/**/file.ext", "/root", "**/file.ext")]
    #[case("/root/**", "/root", "**")]
    fn repartition_variant_glob_has_empty_prefix_and_idempotent_expression(
        #[case] expression: &str,
        #[case] prefix: &str,
        #[case] postfix: &str,
    ) {
        let (_, glob) = harness::assert_partitioned_has_prefix_and_expression(
            harness::assert_new_glob_is_ok(expression).assert_partition_non_empty(),
            (prefix, postfix),
        );
        harness::assert_partitioned_has_prefix_and_expression(
            glob.assert_partition_non_empty(),
            ("", postfix),
        );
    }

    #[rstest]
    #[case("/root", true)]
    #[case("/**", true)]
    #[case("</root:1,>", true)]
    #[case("", false)]
    // The leading forward slash in tree wildcards is meaningful. When omitted at the beginning of
    // an expression, the resulting glob is not rooted.
    #[case("**/", false)]
    // This is not rooted, because character classes never match separators. This example compiles
    // an empty character class, which never matches anything (even nothing).
    #[cfg_attr(any(unix, windows), case("[/]root", false))]
    fn query_glob_has_root_eq(#[case] expression: &str, #[case] expected: bool) {
        let glob = harness::assert_new_glob_is_ok(expression);
        let has_root = glob.has_root().is_always();
        assert!(
            has_root == expected,
            "`Glob::has_root` is `{}`, but expected `{}`: in `Glob`: `{}`",
            has_root,
            expected,
            glob,
        );
    }

    #[cfg(any(unix, windows))]
    #[rstest]
    #[case("../src/**")]
    #[case("*/a/../b.*")]
    #[case("{a,..}")]
    #[case("<a/..>")]
    #[case("<a/{b,..,c}/d>")]
    #[case("{a,<b/{c,..}/>}d")]
    #[case("./*.txt")]
    // TODO: `has_semantic_literals` does not inspect invariant text and so some less obvious
    //       components are not detected. See `Token::literals`.
    //#[case("[.]/a")]
    //#[case("a/[.][.]")]
    fn query_glob_with_dot_components_has_semantic_literals(#[case] expression: &str) {
        let glob = harness::assert_new_glob_is_ok(expression);
        assert!(
            glob.has_semantic_literals(),
            "`Glob::has_semantic_literals` is `false`, but expected `true`: in `Glob`: `{}`",
            glob,
        );
    }

    #[rstest]
    #[case("**/{a*,b*}/???", [1, 2, 3, 4, 5])]
    fn query_glob_captures_have_ordered_indices(
        #[case] expression: &str,
        #[case] expected: impl AsRef<[usize]>,
    ) {
        let glob = harness::assert_new_glob_is_ok(expression);
        let indices: Vec<_> = glob.captures().map(|token| token.index()).collect();
        let expected = expected.as_ref();
        assert!(
            indices == expected,
            "`Glob::captures` has indices `{:?}`, but expected `{:?}`: in `Glob`: `{}`",
            indices,
            expected,
            glob,
        );
    }

    #[rstest]
    #[case("**/{a*,b*}/$", [(0, 3), (3, 7), (11, 1)])]
    fn query_glob_captures_have_ordered_spans(
        #[case] expression: &str,
        #[case] expected: impl AsRef<[Span]>,
    ) {
        let glob = harness::assert_new_glob_is_ok(expression);
        let spans: Vec<_> = glob.captures().map(|token| token.span()).collect();
        let expected = expected.as_ref();
        assert!(
            spans == expected,
            "`Glob::captures` has spans `{:?}`, but expected `{:?}`: in `Glob`: `{}`",
            spans,
            expected,
            glob,
        );
    }

    #[rstest]
    #[case("", true)]
    #[case("/a/file.ext", true)]
    #[case("/a/{file.ext}", true)]
    #[case("/a/b/file.ext", true)]
    #[case("{a,a}", true)]
    #[case("<a/b:2>", true)]
    #[case("/a/{b,c}", false)]
    #[case("<a/b:1,>", false)]
    #[case("/[ab]/file.ext", false)]
    #[case("**", false)]
    #[case("/a/*.ext", false)]
    #[case("/a/b*", false)]
    #[cfg_attr(unix, case("/[a]/file.ext", true))]
    #[cfg_attr(unix, case("/[a-a]/file.ext", true))]
    #[cfg_attr(unix, case("/[a-aaa-a]/file.ext", true))]
    #[cfg_attr(unix, case("/a/(?i)file.ext", false))]
    #[cfg_attr(windows, case("{a,A}", true))]
    #[cfg_attr(windows, case("/a/(?-i)file.ext", false))]
    fn query_glob_text_is_invariant_eq(#[case] expression: &str, #[case] expected: bool) {
        let glob = harness::assert_new_glob_is_ok(expression);
        let is_invariant = glob.text().is_invariant();
        assert!(
            is_invariant == expected,
            "`Variance::is_invariant` is `{}`, but expected `{}`: in `Glob`: `{}`",
            is_invariant,
            expected,
            glob,
        );
    }
}
