//! Rules and limitations for token sequences.
//!
//! This module provides the `check` function, which examines a token sequence and emits an error
//! if the sequence violates rules. Rules are invariants that are difficult or impossible to
//! enforce when parsing text and primarily detect and reject token sequences that produce errant,
//! meaningless, or unexpected programs when compiled.

// TODO: The `check` function fails fast and either report no errors or exactly one error. To
//       better support diagnostics, `check` should probably perform an exhaustive analysis and
//       report zero or more errors.

use itertools::Itertools as _;
#[cfg(feature = "miette")]
use miette::{Diagnostic, LabeledSpan, SourceCode};
use std::borrow::Cow;
use std::collections::VecDeque;
use std::convert::Infallible;
#[cfg(feature = "miette")]
use std::fmt::Display;
use std::iter::Fuse;
use std::path::PathBuf;
use std::slice;
use thiserror::Error;

use crate::diagnostics::{CompositeSpan, CorrelatedSpan, SpanExt as _, Spanned};
use crate::token::walk::{self, TokenEntry};
use crate::token::{
    self, BranchKind, ExpressionMetadata, NaturalRange, Repetition, Size, Token, TokenTree,
    Tokenized,
};
use crate::{Any, BuildError, Glob, Pattern};

/// Maximum invariant size.
///
/// This size is equal to or greater than the maximum size of a path on supported platforms. The
/// primary purpose of this limit is to mitigate malicious or mistaken expressions that encode very
/// large invariant text, namely via repetitions.
///
/// This limit is independent of the back end encoding. This code does not rely on errors in the
/// encoder by design, such as size limitations.
const MAX_INVARIANT_SIZE: Size = Size::new(0x10000);

trait IteratorExt: Iterator + Sized {
    fn adjacent(self) -> Adjacent<Self>
    where
        Self::Item: Clone;
}

impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn adjacent(self) -> Adjacent<Self>
    where
        Self::Item: Clone,
    {
        Adjacent::new(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Adjacency<T> {
    Only { item: T },
    First { item: T, right: T },
    Middle { left: T, item: T, right: T },
    Last { left: T, item: T },
}

impl<T> Adjacency<T> {
    pub fn into_tuple(self) -> (Option<T>, T, Option<T>) {
        match self {
            Adjacency::Only { item } => (None, item, None),
            Adjacency::First { item, right } => (None, item, Some(right)),
            Adjacency::Middle { left, item, right } => (Some(left), item, Some(right)),
            Adjacency::Last { left, item } => (Some(left), item, None),
        }
    }
}

struct Adjacent<I>
where
    I: Iterator,
{
    input: Fuse<I>,
    adjacency: Option<Adjacency<I::Item>>,
}

impl<I> Adjacent<I>
where
    I: Iterator,
{
    fn new(input: I) -> Self {
        let mut input = input.fuse();
        let adjacency = match (input.next(), input.next()) {
            (Some(item), Some(right)) => Some(Adjacency::First { item, right }),
            (Some(item), None) => Some(Adjacency::Only { item }),
            (None, None) => None,
            // The input iterator is fused, so this cannot occur.
            (None, Some(_)) => unreachable!(),
        };
        Adjacent { input, adjacency }
    }
}

impl<I> Iterator for Adjacent<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Adjacency<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.input.next();
        self.adjacency.take().inspect(|adjacency| {
            self.adjacency = match adjacency.clone() {
                Adjacency::First {
                    item: left,
                    right: item,
                }
                | Adjacency::Middle {
                    item: left,
                    right: item,
                    ..
                } => {
                    if let Some(right) = next {
                        Some(Adjacency::Middle { left, item, right })
                    }
                    else {
                        Some(Adjacency::Last { left, item })
                    }
                },
                Adjacency::Only { .. } | Adjacency::Last { .. } => None,
            };
        })
    }
}

trait SliceExt<T> {
    fn terminals(&self) -> Option<Terminals<&T>>;
}

impl<T> SliceExt<T> for [T] {
    fn terminals(&self) -> Option<Terminals<&T>> {
        match self.len() {
            0 => None,
            1 => Some(Terminals::Only(self.first().unwrap())),
            _ => Some(Terminals::StartEnd(
                self.first().unwrap(),
                self.last().unwrap(),
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Terminals<T> {
    Only(T),
    StartEnd(T, T),
}

impl<T> Terminals<T> {
    pub fn map<U, F>(self, mut f: F) -> Terminals<U>
    where
        F: FnMut(T) -> U,
    {
        match self {
            Terminals::Only(only) => Terminals::Only(f(only)),
            Terminals::StartEnd(start, end) => Terminals::StartEnd(f(start), f(end)),
        }
    }
}

/// Describes errors concerning rules and patterns in a glob expression.
///
/// Patterns must follow rules described in the [repository
/// documentation](https://github.com/olson-sean-k/wax/blob/master/README.md). These rules are
/// designed to avoid nonsense glob expressions and ambiguity. If a glob expression parses but
/// violates these rules or is otherwise malformed, then this error is returned by some APIs.
#[derive(Clone, Debug, Error)]
#[error("malformed glob expression: {kind}")]
pub struct RuleError<'t> {
    expression: Cow<'t, str>,
    kind: RuleErrorKind,
    location: CompositeSpan,
}

impl<'t> RuleError<'t> {
    fn new(expression: Cow<'t, str>, kind: RuleErrorKind, location: CompositeSpan) -> Self {
        RuleError {
            expression,
            kind,
            location,
        }
    }

    /// Clones any borrowed data into an owning instance.
    pub fn into_owned(self) -> RuleError<'static> {
        let RuleError {
            expression,
            kind,
            location,
        } = self;
        RuleError {
            expression: expression.into_owned().into(),
            kind,
            location,
        }
    }

    pub fn locations(&self) -> &[CompositeSpan] {
        slice::from_ref(&self.location)
    }

    /// Gets the glob expression that violated pattern rules.
    pub fn expression(&self) -> &str {
        self.expression.as_ref()
    }
}

#[cfg(feature = "miette")]
#[cfg_attr(docsrs, doc(cfg(feature = "miette")))]
impl Diagnostic for RuleError<'_> {
    fn code<'a>(&'a self) -> Option<Box<dyn 'a + Display>> {
        Some(Box::new(String::from(match self.kind {
            RuleErrorKind::RootedSubGlob => "wax::glob::rooted_sub_glob",
            RuleErrorKind::SingularTree => "wax::glob::singular_tree",
            RuleErrorKind::SingularZeroOrMore => "wax::glob::singular_zero_or_more",
            RuleErrorKind::AdjacentBoundary => "wax::glob::adjacent_boundary",
            RuleErrorKind::AdjacentZeroOrMore => "wax::glob::adjacent_zero_or_more",
            RuleErrorKind::OversizedInvariant => "wax::glob::oversized_invariant",
            RuleErrorKind::IncompatibleBounds => "wax::glob::incompatible_bounds",
        })))
    }

    fn help<'a>(&'a self) -> Option<Box<dyn 'a + Display>> {
        match self.kind {
            RuleErrorKind::OversizedInvariant => Some(Box::new(String::from(
                "this error typically occurs when a repetition has a convergent bound that is too \
                 large",
            ))),
            _ => None,
        }
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.expression)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>> {
        Some(Box::new(self.location.labels().into_iter()))
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
enum RuleErrorKind {
    #[error("uncertain or overlapping roots in branch")]
    RootedSubGlob,
    #[error("singular tree wildcard `**` in branch")]
    SingularTree,
    #[error("singular zero-or-more wildcard `*` or `$` in branch")]
    SingularZeroOrMore,
    #[error("adjacent component boundaries `/` or `**`")]
    AdjacentBoundary,
    #[error("adjacent zero-or-more wildcards `*` or `$`")]
    AdjacentZeroOrMore,
    #[error("oversized invariant expression")]
    OversizedInvariant,
    #[error("incompatible repetition bounds")]
    IncompatibleBounds,
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Checked<T> {
    inner: T,
}

impl<T> Checked<T> {
    pub fn release(self) -> T {
        self.inner
    }
}

impl<'t, T> Checked<T>
where
    T: TokenTree<'t>,
{
    pub fn into_alternatives(self) -> Vec<Checked<Token<'t, T::Annotation>>> {
        self.release()
            .into_token()
            .into_alternatives()
            .into_iter()
            .map(|token| Checked { inner: token })
            .collect()
    }
}

impl<'t> Checked<Token<'t, ()>> {
    pub fn any<T, I>(trees: I) -> Self
    where
        T: TokenTree<'t>,
        I: IntoIterator<Item = Checked<T>>,
    {
        Checked {
            // `token::any` constructs an alternation from the input token trees. The alternation
            // is **not** checked, but the `any` combinator is explicitly allowed to ignore the
            // subset of rules that may be violated by this construction. In particular,
            // alternatives may or may not have roots such that the alternation can match
            // overlapping directory trees, because combinators do not support matching against
            // directory trees.
            inner: token::any(trees.into_iter().map(Checked::release)),
        }
    }
}

impl<'t, A> Checked<Token<'t, A>> {
    pub fn into_owned(self) -> Checked<Token<'static, A>> {
        Checked {
            inner: self.release().into_owned(),
        }
    }
}

impl<'t, A> Checked<Tokenized<'t, A>> {
    pub fn into_owned(self) -> Checked<Tokenized<'static, A>> {
        Checked {
            inner: self.release().into_owned(),
        }
    }
}

impl<'t, A> Checked<Tokenized<'t, A>>
where
    A: Default + Spanned,
{
    pub fn partition(self) -> (PathBuf, Option<Self>) {
        let tokenized = self.release();
        // This relies on the correctness of `Tokenized::partition`, which must not violate rules.
        let (path, tokenized) = tokenized.partition();
        (
            path,
            tokenized.map(|tokenized| Checked { inner: tokenized }),
        )
    }
}

impl<T> AsRef<T> for Checked<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<'t> From<Any<'t>> for Checked<Token<'t, ()>> {
    fn from(any: Any<'t>) -> Self {
        let Any { tree, .. } = any;
        tree
    }
}

impl<'t> From<Glob<'t>> for Checked<Tokenized<'t, ExpressionMetadata>> {
    fn from(glob: Glob<'t>) -> Self {
        let Glob { tree, .. } = glob;
        tree
    }
}

impl<'t, T> Pattern<'t> for Checked<T>
where
    T: TokenTree<'t>,
{
    type Tokens = T;
    type Error = Infallible;
}

impl<'t, T> TryFrom<Result<T, BuildError>> for Checked<T::Tokens>
where
    T: Into<Checked<T::Tokens>> + Pattern<'t>,
{
    type Error = BuildError;

    fn try_from(result: Result<T, BuildError>) -> Result<Self, Self::Error> {
        result.map(Into::into)
    }
}

impl<'t> TryFrom<&'t str> for Checked<Tokenized<'t, ExpressionMetadata>> {
    type Error = BuildError;

    fn try_from(expression: &'t str) -> Result<Self, Self::Error> {
        crate::parse_and_check(expression)
    }
}

pub fn check<A>(tree: Tokenized<'_, A>) -> Result<Checked<Tokenized<'_, A>>, RuleError<'_>>
where
    A: Spanned,
{
    boundary(&tree)?;
    bounds(&tree)?;
    branch(&tree)?;
    size(&tree)?;
    Ok(Checked { inner: tree })
}

fn boundary<'t, A>(tree: &Tokenized<'t, A>) -> Result<(), RuleError<'t>>
where
    A: Spanned,
{
    if let Some((left, right)) = walk::forward(tree)
        .chunk_by(TokenEntry::position)
        .into_iter()
        .flat_map(|(_, group)| {
            group
                .map(TokenEntry::into_token)
                .tuple_windows::<(_, _)>()
                .filter(|(left, right)| left.boundary().and(right.boundary()).is_some())
                .map(|(left, right)| (*left.annotation().span(), *right.annotation().span()))
        })
        .next()
    {
        Err(RuleError::new(
            tree.expression().clone(),
            RuleErrorKind::AdjacentBoundary,
            CompositeSpan::spanned("here", left.union(right)),
        ))
    }
    else {
        Ok(())
    }
}

fn branch<'t, A>(tree: &Tokenized<'t, A>) -> Result<(), RuleError<'t>>
where
    A: Spanned,
{
    use crate::token::LeafKind::{Separator, Wildcard};
    use crate::token::Wildcard::{Tree, ZeroOrMore};
    use Terminals::{Only, StartEnd};

    #[derive(Debug)]
    struct CorrelatedError {
        kind: RuleErrorKind,
        location: CorrelatedSpan,
    }

    impl CorrelatedError {
        fn new<A>(kind: RuleErrorKind, outer: Option<&Token<'_, A>>, inner: &Token<'_, A>) -> Self
        where
            A: Spanned,
        {
            CorrelatedError {
                kind,
                location: CorrelatedSpan::split_some(
                    outer.map(Token::annotation).map(A::span).copied(),
                    *inner.annotation().span(),
                ),
            }
        }
    }

    #[derive(Debug)]
    struct Outer<'i, 't, A> {
        left: Option<&'i Token<'t, A>>,
        right: Option<&'i Token<'t, A>>,
    }

    impl<'i, 't, A> Outer<'i, 't, A> {
        // This may appear to operate in place.
        #[must_use]
        pub fn or(self, left: Option<&'i Token<'t, A>>, right: Option<&'i Token<'t, A>>) -> Self {
            Outer {
                left: left.or(self.left),
                right: right.or(self.right),
            }
        }
    }

    impl<'i, 't, A> Clone for Outer<'i, 't, A> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<'i, 't, A> Copy for Outer<'i, 't, A> {}

    impl<'i, 't, A> Default for Outer<'i, 't, A> {
        fn default() -> Self {
            Outer {
                left: None,
                right: None,
            }
        }
    }

    fn is_some_and_any_in<'i, 't, A, R, I, P>(
        token: Option<&'i Token<'t, A>>,
        traversal: R,
        mut predicate: P,
    ) -> bool
    where
        R: FnOnce(&'i Token<'t, A>) -> I,
        I: Iterator<Item = TokenEntry<'i, 't, A>>,
        P: FnMut(&'i Token<'t, A>) -> bool,
    {
        token.is_some_and(|token| traversal(token).any(move |entry| predicate(entry.into_token())))
    }

    fn is_boundary<A>(token: &Token<'_, A>) -> bool {
        token.boundary().is_some()
    }

    fn is_zom<A>(token: &Token<'_, A>) -> bool {
        matches!(token.as_leaf(), Some(Wildcard(ZeroOrMore(_))))
    }

    fn has_starting_boundary<A>(token: Option<&Token<'_, A>>) -> bool {
        is_some_and_any_in(token, walk::starting, is_boundary)
    }

    fn has_ending_boundary<A>(token: Option<&Token<'_, A>>) -> bool {
        is_some_and_any_in(token, walk::ending, is_boundary)
    }

    fn has_starting_zom<A>(token: Option<&Token<'_, A>>) -> bool {
        is_some_and_any_in(token, walk::starting, is_zom)
    }

    fn has_ending_zom<A>(token: Option<&Token<'_, A>>) -> bool {
        is_some_and_any_in(token, walk::ending, is_zom)
    }

    fn diagnose<'i, 't, A>(
        // This is a somewhat unusual API, but it allows the lifetime `'t` of the `Cow` to be
        // properly forwarded to output values (`RuleError`).
        #[allow(clippy::ptr_arg)] expression: &'i Cow<'t, str>,
        token: &'i Token<'t, A>,
        label: &'static str,
    ) -> impl 'i + Copy + FnOnce(CorrelatedError) -> RuleError<'t>
    where
        't: 'i,
        A: Spanned,
    {
        move |CorrelatedError { kind, location }| {
            RuleError::new(
                expression.clone(),
                kind,
                CompositeSpan::correlated(label, *token.annotation().span(), location),
            )
        }
    }

    fn check_branch<'i, 't, A>(
        terminals: Terminals<&'i Token<'t, A>>,
        outer: Outer<'i, 't, A>,
    ) -> Result<(), CorrelatedError>
    where
        A: Spanned,
    {
        let Outer { left, right } = outer;
        match terminals.map(|token| (token, token.as_leaf())) {
            // The branch is preceded by component boundaries; disallow leading separators.
            //
            // For example, `foo/{bar,/}`.
            Only((inner, Some(Separator(_)))) | StartEnd((inner, Some(Separator(_))), _)
                if has_ending_boundary(left) =>
            {
                Err(CorrelatedError::new(
                    RuleErrorKind::AdjacentBoundary,
                    left,
                    inner,
                ))
            },
            // The branch is followed by component boundaries; disallow trailing separators.
            //
            // For example, `{foo,/}/bar`.
            Only((inner, Some(Separator(_)))) | StartEnd(_, (inner, Some(Separator(_))))
                if has_starting_boundary(right) =>
            {
                Err(CorrelatedError::new(
                    RuleErrorKind::AdjacentBoundary,
                    right,
                    inner,
                ))
            },
            // Disallow singular tree tokens.
            //
            // For example, `{foo,bar,**}`.
            Only((inner, Some(Wildcard(Tree { .. })))) => Err(CorrelatedError::new(
                RuleErrorKind::SingularTree,
                None,
                inner,
            )),
            // The branch is preceded by component boundaries; disallow leading tree tokens.
            //
            // For example, `foo/{bar,**/baz}`.
            StartEnd((inner, Some(Wildcard(Tree { .. }))), _) if has_ending_boundary(left) => Err(
                CorrelatedError::new(RuleErrorKind::AdjacentBoundary, left, inner),
            ),
            // The branch is followed by component boundaries; disallow trailing tree tokens.
            //
            // For example, `{foo,bar/**}/baz`.
            StartEnd(_, (inner, Some(Wildcard(Tree { .. })))) if has_starting_boundary(right) => {
                Err(CorrelatedError::new(
                    RuleErrorKind::AdjacentBoundary,
                    right,
                    inner,
                ))
            },
            // The branch is prefixed by a zero-or-more token; disallow leading zero-or-more tokens.
            //
            // For example, `foo*{bar,*,baz}`.
            Only((inner, Some(Wildcard(ZeroOrMore(_)))))
            | StartEnd((inner, Some(Wildcard(ZeroOrMore(_)))), _)
                if has_ending_zom(left) =>
            {
                Err(CorrelatedError::new(
                    RuleErrorKind::AdjacentZeroOrMore,
                    left,
                    inner,
                ))
            },
            // The branch is followed by a zero-or-more token; disallow trailing zero-or-more
            // tokens.
            //
            // For example, `{foo,*,bar}*baz`.
            Only((inner, Some(Wildcard(ZeroOrMore(_)))))
            | StartEnd(_, (inner, Some(Wildcard(ZeroOrMore(_)))))
                if has_starting_zom(right) =>
            {
                Err(CorrelatedError::new(
                    RuleErrorKind::AdjacentZeroOrMore,
                    right,
                    inner,
                ))
            },
            _ => Ok(()),
        }
    }

    fn check_alternation<'i, 't, A>(
        terminals: Terminals<&'i Token<'t, A>>,
        outer: Outer<'i, 't, A>,
    ) -> Result<(), CorrelatedError>
    where
        A: Spanned,
    {
        let Outer { left, .. } = outer;
        match terminals.map(|token| (token, token.as_leaf())) {
            // The alternation is preceded by a termination; disallow rooted sub-globs.
            //
            // For example, `{foo,/}` or `{foo,/bar}`.
            Only((inner, Some(Separator(_)))) | StartEnd((inner, Some(Separator(_))), _)
                if left.is_none() =>
            {
                Err(CorrelatedError::new(
                    RuleErrorKind::RootedSubGlob,
                    left,
                    inner,
                ))
            },
            // The alternation is preceded by a termination; disallow rooted sub-globs.
            //
            // For example, `{/**/foo,bar}`.
            Only((inner, Some(Wildcard(Tree { has_root: true }))))
            | StartEnd((inner, Some(Wildcard(Tree { has_root: true }))), _)
                if left.is_none() =>
            {
                Err(CorrelatedError::new(
                    RuleErrorKind::RootedSubGlob,
                    left,
                    inner,
                ))
            },
            _ => Ok(()),
        }
    }

    fn check_repetition<'i, 't, A>(
        terminals: Terminals<&'i Token<'t, A>>,
        outer: Outer<'i, 't, A>,
        variance: NaturalRange,
    ) -> Result<(), CorrelatedError>
    where
        A: Spanned,
    {
        let Outer { left, .. } = outer;
        let lower = variance.lower().into_bound();
        match terminals.map(|token| (token, token.as_leaf())) {
            // The repetition is preceded by a termination; disallow rooted sub-globs with a zero
            // lower bound.
            //
            // For example, `</foo:0,>`.
            Only((inner, Some(Separator(_)))) | StartEnd((inner, Some(Separator(_))), _)
                if left.is_none() && lower.is_unbounded() =>
            {
                Err(CorrelatedError::new(
                    RuleErrorKind::RootedSubGlob,
                    left,
                    inner,
                ))
            },
            // The repetition is preceded by a termination; disallow rooted sub-globs with a zero
            // lower bound.
            //
            // For example, `</**/foo>`.
            Only((inner, Some(Wildcard(Tree { has_root: true }))))
            | StartEnd((inner, Some(Wildcard(Tree { has_root: true }))), _)
                if left.is_none() && lower.is_unbounded() =>
            {
                Err(CorrelatedError::new(
                    RuleErrorKind::RootedSubGlob,
                    left,
                    inner,
                ))
            },
            // The repetition begins and ends with a separator.
            //
            // For example, `</foo/bar/:1,>`.
            StartEnd((left, _), (right, _)) if left.boundary().and(right.boundary()).is_some() => {
                Err(CorrelatedError::new(
                    RuleErrorKind::AdjacentBoundary,
                    Some(left),
                    right,
                ))
            },
            // The repetition is a singular separator.
            //
            // For example, `</:1,>`.
            Only((token, Some(Separator(_)))) => Err(CorrelatedError::new(
                RuleErrorKind::AdjacentBoundary,
                None,
                token,
            )),
            // The repetition is a singular zero-or-more wildcard.
            //
            // For example, `<*:1,>`.
            Only((token, Some(Wildcard(ZeroOrMore(_))))) => Err(CorrelatedError::new(
                RuleErrorKind::SingularZeroOrMore,
                None,
                token,
            )),
            _ => Ok(()),
        }
    }

    let mut outer = Outer::default();
    let mut tokens: VecDeque<_> = Some(tree.as_token()).into_iter().collect();
    while let Some(token) = tokens.pop_front() {
        use BranchKind::{Alternation, Repetition};

        for (left, token, right) in token
            .concatenation()
            .iter()
            .adjacent()
            .map(Adjacency::into_tuple)
        {
            match token.as_branch() {
                Some(Alternation(alternation)) => {
                    outer = outer.or(left, right);
                    let diagnose = diagnose(tree.expression(), token, "in this alternation");
                    for token in alternation.tokens() {
                        let concatenation = token.concatenation();
                        if let Some(terminals) = concatenation.terminals() {
                            check_branch(terminals, outer).map_err(diagnose)?;
                            check_alternation(terminals, outer).map_err(diagnose)?;
                        }
                    }
                    tokens.extend(alternation.tokens());
                },
                Some(Repetition(repetition)) => {
                    outer = outer.or(left, right);
                    let diagnose = diagnose(tree.expression(), token, "in this repetition");
                    let token = repetition.token();
                    let concatenation = token.concatenation();
                    if let Some(terminals) = concatenation.terminals() {
                        check_branch(terminals, outer).map_err(diagnose)?;
                        check_repetition(terminals, outer, repetition.variance())
                            .map_err(diagnose)?;
                    }
                    tokens.push_back(token);
                },
                _ => {},
            }
        }
    }
    Ok(())
}

// Arguably, this function enforces _syntactic_ rules. Any bound specification can be used to
// construct a functional repetition token, but this is only because `NaturalRange` has an
// interpretation of these bounds (such as `<_:0,0>` and `<_:10,1>`). These interpretations may be
// counterintuitive and present multiple spellings of the same semantics, so they are rejected
// here.
fn bounds<'t, A>(tree: &Tokenized<'t, A>) -> Result<(), RuleError<'t>>
where
    A: Spanned,
{
    if let Some(token) = walk::forward(tree)
        .map(TokenEntry::into_token)
        .find(|token| {
            token
                .as_repetition()
                .map(Repetition::bound_specification)
                .and_then(|(lower, upper)| upper.map(|upper| (lower, upper)))
                .is_some_and(|(lower, upper)| (lower > upper) || (lower == 0 && lower == upper))
        })
    {
        Err(RuleError::new(
            tree.expression().clone(),
            RuleErrorKind::IncompatibleBounds,
            CompositeSpan::spanned("here", *token.annotation().span()),
        ))
    }
    else {
        Ok(())
    }
}

fn size<'t, A>(tree: &Tokenized<'t, A>) -> Result<(), RuleError<'t>>
where
    A: Spanned,
{
    if let Some(token) = walk::forward(tree)
        .map(TokenEntry::into_token)
        // TODO: This is expensive. For each token tree encountered, the tree is traversed to
        //       determine its variance. If variant, the tree is traversed and queried again,
        //       revisiting the same tokens to recompute their local variance.
        .find(|token| {
            token
                .variance::<Size>()
                .invariant()
                .is_some_and(|size| size >= MAX_INVARIANT_SIZE)
        })
    {
        Err(RuleError::new(
            tree.expression().clone(),
            RuleErrorKind::OversizedInvariant,
            CompositeSpan::spanned("here", *token.annotation().span()),
        ))
    }
    else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rstest::rstest;

    use crate::rule::{Adjacency, IteratorExt as _};

    #[rstest]
    #[case::empty([], [])]
    #[case::only([0], [Adjacency::Only { item: 0 }])]
    #[case::first_middle_last(
        0..3,
        [
            Adjacency::First { item: 0, right: 1 },
            Adjacency::Middle {
                left: 0,
                item: 1,
                right: 2,
            },
            Adjacency::Last { left: 1, item: 2 },
        ],
    )]
    fn into_iter_adjacent_eq(
        #[case] items: impl IntoIterator<Item = i32>,
        #[case] expected: impl IntoIterator<Item = Adjacency<i32>>,
    ) {
        use itertools::EitherOrBoth::{Both, Left, Right};

        for (index, zipped) in items
            .into_iter()
            .adjacent()
            .zip_longest(expected)
            .enumerate()
        {
            if let Err((item, expected)) = match zipped {
                Both(item, expected) if item != expected => Err((Some(item), Some(expected))),
                Left(item) => Err((Some(item), None)),
                Right(expected) => Err((None, Some(expected))),
                _ => Ok(()),
            } {
                panic!(
                    "unexpected item at index {} in `Iterator::<Item = i32>::adjacent`:\
                    \n\titem: `{:?}`\n\texpected: `{:?}`",
                    index, item, expected,
                );
            }
        }
    }
}
