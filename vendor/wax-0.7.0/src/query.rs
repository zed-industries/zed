//! [`Program`] diagnostics and inspection.
//!
//! [`Program`]: crate::Program

// This module proxies variance and bounds types from the `token` module through types defined here
// via conversions. For example, `VariantRange` has a public counterpart here that lacks many APIs
// that are unnecessary for downstream code. Types in this module generally present less complex
// APIs and insulate their more complex counterparts from the burdens of public export.

use std::borrow::Cow;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use crate::token::{self, Depth, Text, TokenVariance};

pub use crate::diagnostics::{LocatedError, Span};
pub use crate::token::Boundedness;

pub use Boundedness::{Bounded, Unbounded};

/// A range over naturals that is necessarily variant (not converged).
pub type VariantRange = Boundedness<BoundedVariantRange>;

impl VariantRange {
    /// Gets the lower bound of the range.
    pub fn lower(&self) -> Boundedness<NonZeroUsize> {
        match self {
            Bounded(range) => range.lower(),
            _ => Unbounded,
        }
    }

    /// Gets the upper bound of the range.
    pub fn upper(&self) -> Boundedness<NonZeroUsize> {
        match self {
            Bounded(range) => range.upper(),
            _ => Unbounded,
        }
    }
}

impl From<token::VariantRange> for VariantRange {
    fn from(range: token::VariantRange) -> Self {
        range.map_bounded(From::from)
    }
}

/// A range over naturals that is necessarily bounded and variant (not converged).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundedVariantRange {
    lower: Boundedness<NonZeroUsize>,
    upper: Boundedness<NonZeroUsize>,
}

impl BoundedVariantRange {
    /// Gets the lower bound of the range.
    pub fn lower(&self) -> Boundedness<NonZeroUsize> {
        self.lower
    }

    /// Gets the lower bound of the range.
    pub fn upper(&self) -> Boundedness<NonZeroUsize> {
        self.upper
    }
}

impl From<token::BoundedVariantRange> for BoundedVariantRange {
    fn from(range: token::BoundedVariantRange) -> Self {
        BoundedVariantRange {
            lower: range.lower().into_bound(),
            upper: range.upper().into_bound(),
        }
    }
}

/// The variance of a [`Program`] with respect to some quantity `T`.
///
/// Variance describes how a quantity may or may not vary in a [`Program`]. When invariant, the
/// quantity has a specific determinant value for any and all matched paths. When variant, the
/// quantity may vary in some way and may or may not be determinant and bounded.
///
/// See [`DepthVariance`] and [`TextVariance`].
///
/// [`Program`]: crate::Program
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variance<T, B> {
    /// The quantity `T` is invariant.
    Invariant(T),
    /// The quantity `T` is variant with the bounds `B`.
    ///
    /// Some quantities do not express bounds when variant (i.e., [text][`TextVariance`]).
    ///
    /// [`TextVariance`]: crate::query::TextVariance
    Variant(B),
}

impl<T, B> Variance<T, B> {
    pub fn invariant(self) -> Option<T> {
        match self {
            Variance::Invariant(invariant) => Some(invariant),
            _ => None,
        }
    }

    pub fn variant(self) -> Option<B> {
        match self {
            Variance::Variant(bound) => Some(bound),
            _ => None,
        }
    }

    /// Converts from `&Variance<T, B>` to `Variance<&T, &B>`.
    ///
    /// Produces a new `Variance` that contains a reference `self`, leaving `self` in place.
    pub fn as_ref(&self) -> Variance<&T, &B> {
        match self {
            Variance::Invariant(invariant) => Variance::Invariant(invariant),
            Variance::Variant(bound) => Variance::Variant(bound),
        }
    }

    /// Returns `true` if **in**variant.
    pub fn is_invariant(&self) -> bool {
        matches!(self, Variance::Invariant(_))
    }

    /// Returns `true` if variant.
    pub fn is_variant(&self) -> bool {
        matches!(self, Variance::Variant(_))
    }
}

/// Depth variance of a [`Program`].
///
/// Depth describes the number of levels into a directory tree from some root that a path
/// represents and a [`Program`] may match. When variant, the bounds of depth are described by
/// [`VariantRange`].
///
/// [`Program`]: crate::Program
/// [`VariantRange`]: crate::query::VariantRange
pub type DepthVariance = Variance<usize, VariantRange>;

impl From<TokenVariance<Depth>> for DepthVariance {
    fn from(depth: TokenVariance<Depth>) -> Self {
        match depth {
            TokenVariance::Invariant(depth) => DepthVariance::Invariant(depth.into()),
            TokenVariance::Variant(bounds) => DepthVariance::Variant(bounds.into()),
        }
    }
}

/// Text variance of a [`Program`].
///
/// Text describes the path text that a [`Program`] may match. When its text is invariant, a
/// [`Program`] resolves to a file no differently than a native path using platform file system
/// APIs. For example, the glob expression `{log,log}` has the invariant text `log` and so behaves
/// just like the native path `log`.
///
/// Text does not describe any bounds when variant.
///
/// [`Program`]: crate::Program
pub type TextVariance<'t> = Variance<Cow<'t, str>, ()>;

impl<'t> TextVariance<'t> {
    /// Converts the text variance into a [`PathBuf`].
    ///
    /// Returns `None` if variant.
    ///
    /// [`PathBuf`]: std::path::PathBuf
    pub fn into_path_buf(self) -> Option<PathBuf> {
        self.invariant().map(Cow::into_owned).map(From::from)
    }

    /// Gets a [`Path`] from the invariant text.
    ///
    /// Returns `None` if variant.
    ///
    /// [`Path`]: std::path::Path
    pub fn as_path(&self) -> Option<&Path> {
        self.as_ref().invariant().map(|text| text.as_ref().as_ref())
    }
}

impl<'t> From<TokenVariance<Text<'t>>> for TextVariance<'t> {
    fn from(text: TokenVariance<Text<'t>>) -> Self {
        match text {
            TokenVariance::Invariant(text) => TextVariance::Invariant(text.to_string()),
            TokenVariance::Variant(_) => TextVariance::Variant(()),
        }
    }
}

/// Trivalent logic truth value.
///
/// `When` extends the bivalent Boolean logic of `bool` with a third truth value that represents
/// unknown, uncertainty, etc.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum When {
    /// Verum truth value for which a proposition is true or always occurs (i.e., `true`).
    Always,
    /// Indeterminant truth value for which a proposition is unknown or may sometimes occur.
    Sometimes,
    /// Falsum truth value for which a proposition is false or never occurs (i.e., `false`).
    Never,
}

impl When {
    /// Trivalent logical conjunction (AND) operator.
    pub fn and(self, other: Self) -> Self {
        use When::{Always, Never, Sometimes};

        match (self, other) {
            (Never, _) | (_, Never) => Never,
            (Sometimes, _) | (_, Sometimes) => Sometimes,
            (Always, Always) => Always,
        }
    }

    /// Trivalent logical disjunction (OR) operator.
    pub fn or(self, other: Self) -> Self {
        use When::{Always, Never, Sometimes};

        match (self, other) {
            (Always, _) | (_, Always) => Always,
            (Sometimes, _) | (_, Sometimes) => Sometimes,
            (Never, Never) => Never,
        }
    }

    /// Trivalent logical certainty conjunction operator.
    ///
    /// This operator is similar to the bivalent conjunction, but is [`Sometimes`] if any operand
    /// is [`Sometimes`], preserving a notion of uncertainty in the conjunction.
    ///
    /// [`Sometimes`]: crate::query::When::Sometimes
    pub fn certainty(self, other: Self) -> Self {
        use When::{Always, Never, Sometimes};

        match (self, other) {
            (Always, Always) => Always,
            (Never, Never) => Never,
            (Sometimes, _) | (_, Sometimes) | (Always, _) | (_, Always) => Sometimes,
        }
    }

    /// Returns `true` if the truth value is [`Always`] (true).
    ///
    /// [`Always`]: crate::query::When::Always
    pub fn is_always(&self) -> bool {
        matches!(self, When::Always)
    }

    /// Returns `true` if the truth value is [`Sometimes`].
    ///
    /// [`Sometimes`]: crate::query::When::Sometimes
    pub fn is_sometimes(&self) -> bool {
        matches!(self, When::Sometimes)
    }

    /// Returns `true` if the truth value is [`Never`] (false).
    ///
    /// [`Never`]: crate::query::When::Never
    pub fn is_never(&self) -> bool {
        matches!(self, When::Never)
    }

    /// Returns `true` if the truth value is [`Always`] or [`Sometimes`].
    ///
    /// [`Always`]: crate::query::When::Always
    /// [`Sometimes`]: crate::query::When::Sometimes
    pub fn is_maybe_true(&self) -> bool {
        !self.is_never()
    }

    /// Returns `true` if the truth value is [`Never`] or [`Sometimes`].
    ///
    /// [`Never`]: crate::query::When::Never
    /// [`Sometimes`]: crate::query::When::Sometimes
    pub fn is_maybe_false(&self) -> bool {
        !self.is_always()
    }
}

impl From<bool> for When {
    fn from(is_always: bool) -> Self {
        if is_always { When::Always } else { When::Never }
    }
}

impl From<Option<bool>> for When {
    fn from(when: Option<bool>) -> Self {
        match when {
            Some(true) => When::Always,
            None => When::Sometimes,
            Some(false) => When::Never,
        }
    }
}

impl From<When> for Option<bool> {
    fn from(when: When) -> Self {
        match when {
            When::Always => Some(true),
            When::Sometimes => None,
            When::Never => Some(false),
        }
    }
}

/// Token that captures matched text in a glob expression.
///
/// # Examples
///
/// `CapturingToken`s can be used to isolate sub-expressions.
///
/// ```rust
/// use wax::Glob;
///
/// let expression = "**/*.txt";
/// let glob = Glob::new(expression).unwrap();
/// for token in glob.captures() {
///     let (start, n) = token.span();
///     println!("capturing sub-expression: {}", &expression[start..][..n]);
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct CapturingToken {
    index: usize,
    span: Span,
}

impl CapturingToken {
    pub(crate) fn new(index: usize, span: Span) -> Self {
        CapturingToken { index, span }
    }

    /// Gets the index of the capture.
    ///
    /// Captures are one-indexed and the index zero always represents the implicit capture of the
    /// complete match, so the index of `CapturingToken`s is always one or greater. See
    /// [`MatchedText`].
    ///
    /// [`MatchedText`]: crate::MatchedText
    pub fn index(&self) -> usize {
        self.index
    }

    /// Gets the span of the token's sub-expression.
    pub fn span(&self) -> Span {
        self.span
    }
}
