pub mod invariant;
pub mod natural;
pub mod ops;

use std::marker::PhantomData;
use std::num::NonZeroUsize;

use crate::query::When;
use crate::token::variance::invariant::{
    BoundaryTerm, Breadth, Depth, Finalize, Invariant, InvariantBound, InvariantTerm, One, Text,
    UnitBound, Zero,
};
use crate::token::variance::natural::{
    BoundedVariantRange, NaturalRange, OpenedUpperBound, VariantRange,
};
use crate::token::variance::ops::{Conjunction, Disjunction, Product};
use crate::token::walk::{ChildToken, Fold, Forward, ParentToken, Sequencer};
use crate::token::{Boundary, BranchKind, LeafKind};

pub use Boundedness::{Bounded, Unbounded};

pub type TokenVariance<T> = Variance<T, Boundedness<InvariantBound<T>>>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Boundedness<T> {
    Bounded(T),
    Unbounded,
}

pub trait VarianceTerm<T>
where
    T: Invariant,
{
    fn term(&self) -> T::Term;
}

pub trait VarianceFold<T>
where
    T: Invariant,
{
    fn fold(&self, terms: Vec<T::Term>) -> Option<T::Term>;

    fn finalize(&self, term: T::Term) -> T::Term {
        term
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Variance<T, B = Boundedness<UnitBound>> {
    Invariant(T),
    Variant(B),
}

impl<T, B> Variance<T, B> {
    pub fn map_invariant<U, F>(self, f: F) -> Variance<U, B>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Variance::Invariant(invariant) => Variance::Invariant(f(invariant)),
            Variance::Variant(bound) => Variance::Variant(bound),
        }
    }

    pub fn map_variant<U, F>(self, f: F) -> Variance<T, U>
    where
        F: FnOnce(B) -> U,
    {
        match self {
            Variance::Invariant(invariant) => Variance::Invariant(invariant),
            Variance::Variant(bound) => Variance::Variant(f(bound)),
        }
    }

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

    pub fn as_ref(&self) -> Variance<&T, &B> {
        match self {
            Variance::Invariant(invariant) => Variance::Invariant(invariant),
            Variance::Variant(bound) => Variance::Variant(bound),
        }
    }

    pub fn is_invariant(&self) -> bool {
        matches!(self, Variance::Invariant(_))
    }

    pub fn is_variant(&self) -> bool {
        matches!(self, Variance::Variant(_))
    }
}

impl<T> Variance<T, T> {
    pub fn into_inner(self) -> T {
        match self {
            Variance::Invariant(inner) | Variance::Variant(inner) => inner,
        }
    }

    pub fn map<U, F>(self, f: F) -> Variance<U, U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Variance::Invariant(invariant) => Variance::Invariant(f(invariant)),
            Variance::Variant(variant) => Variance::Variant(f(variant)),
        }
    }
}

impl<T, B> Variance<T, Boundedness<B>> {
    pub const fn unbounded() -> Self {
        Variance::Variant(Unbounded)
    }

    pub fn is_bounded(&self) -> bool {
        !self.is_unbounded()
    }

    pub fn is_unbounded(&self) -> bool {
        self.as_ref()
            .variant()
            .is_some_and(Boundedness::is_unbounded)
    }
}

impl<T> Variance<T, VariantRange> {
    pub fn has_upper_bound(&self) -> bool {
        self.as_ref()
            .variant()
            .is_none_or(|range| range.upper().into_bound().is_bounded())
    }
}

impl<T> Conjunction for TokenVariance<T>
where
    T: Conjunction<Output = T> + Invariant,
    T::Bound: Conjunction<Output = T::Bound> + Conjunction<T, Output = T::Bound> + OpenedUpperBound,
{
    type Output = Self;

    fn conjunction(self, rhs: Self) -> Self::Output {
        use Variance::{Invariant, Variant};

        let lhs = self;
        match (lhs, rhs) {
            // Both terms are invariant. Conjunction is invariant over the sum of the invariants.
            (Invariant(lhs), Invariant(rhs)) => Invariant(ops::conjunction(lhs, rhs)),
            // Both terms are bounded. Conjunction is bounded over the sum of the bounds.
            (Variant(Bounded(lhs)), Variant(Bounded(rhs))) => {
                Variant(Bounded(ops::conjunction(lhs, rhs)))
            },
            // Both terms are unbounded. Conjunction is unbounded.
            (Variant(Unbounded), Variant(Unbounded)) => Variant(Unbounded),
            // One term is bounded and the other is invariant. Conjunction is bounded over the sum
            // of the bound and the invariant.
            (Variant(Bounded(bound)), Invariant(invariant))
            | (Invariant(invariant), Variant(Bounded(bound))) => {
                Variant(Bounded(ops::conjunction(bound, invariant)))
            },
            // One term is unbounded and the other is invariant. Conjunction is variant over the
            // bound of the invariant.
            (Variant(Unbounded), Invariant(invariant))
            | (Invariant(invariant), Variant(Unbounded)) => Variant(invariant.into_lower_bound()),
            // One term is unbounded and the other is bounded. Conjunction is variant over the
            // bounded term without its upper bound.
            (Variant(Unbounded), Variant(Bounded(bound)))
            | (Variant(Bounded(bound)), Variant(Unbounded)) => Variant(bound.opened_upper_bound()),
        }
    }
}

impl<T> Disjunction for TokenVariance<T>
where
    Self: PartialEq,
    T: Invariant,
    T::Bound: Disjunction<Output = Boundedness<T::Bound>>
        + Disjunction<T, Output = Boundedness<T::Bound>>,
{
    type Output = Self;

    fn disjunction(self, rhs: Self) -> Self::Output {
        use Variance::{Invariant, Variant};

        let lhs = self;
        match (lhs, rhs) {
            // The terms are equal. Disjunction is one of the terms (any term may be used).
            (lhs, rhs) if lhs == rhs => lhs,
            // The terms are invariant (and unequal). Disjunction is variant over the bound of the
            // invariants.
            (Invariant(lhs), Invariant(rhs)) => Variant(T::bound(lhs, rhs)),
            // A term is unbounded. Disjunction is unbounded.
            (Variant(Unbounded), _) | (_, Variant(Unbounded)) => Variant(Unbounded),
            // Both terms are bounded. Disjunction is variant over the sum of the bounds.
            (Variant(Bounded(lhs)), Variant(Bounded(rhs))) => Variant(ops::disjunction(lhs, rhs)),
            // One term is bounded and the other is invariant. Disjunction is variant over the sum
            // (and bound) of the bound and the invariant.
            (Variant(Bounded(bound)), Invariant(invariant))
            | (Invariant(invariant), Variant(Bounded(bound))) => {
                Variant(ops::disjunction(bound, invariant))
            },
        }
    }
}

impl<T> Finalize for TokenVariance<T>
where
    T: Invariant,
{
    type Output = Self;

    fn finalize(self) -> Self::Output {
        self
    }
}

impl<T, B> One for Variance<T, B>
where
    T: One,
{
    fn one() -> Self {
        Variance::Invariant(T::one())
    }
}

impl<T> Product<NaturalRange> for TokenVariance<T>
where
    Boundedness<T::Bound>: Product<NonZeroUsize, Output = Boundedness<T::Bound>>,
    T: Invariant + Product<VariantRange, Output = TokenVariance<T>> + Product<usize, Output = T>,
    T::Bound: Product<BoundedVariantRange, Output = Boundedness<T::Bound>>,
{
    type Output = Self;

    fn product(self, rhs: NaturalRange) -> Self::Output {
        use Variance::{Invariant, Variant};

        let lhs = self;
        match (lhs, rhs) {
            (Variant(Unbounded), Variant(Unbounded))
            | (Variant(Unbounded), Variant(Bounded(_)))
            | (Variant(Bounded(_)), Variant(Unbounded)) => Variant(Unbounded),
            // This inner product is incomplete (and cannot be complete). Ideally, the bound would
            // implement the product over `usize` with variance as the output, but it is impossible
            // to implement `Product` this way. Instead, a zero invariant is mapped to the zero
            // variance or else the product with the bound is computed.
            (Variant(lhs), Invariant(rhs)) => NonZeroUsize::new(rhs)
                .map_or_else(Variance::zero, |rhs| Variant(ops::product(lhs, rhs))),
            (Variant(Bounded(lhs)), Variant(Bounded(rhs))) => Variant(ops::product(lhs, rhs)),
            (Invariant(lhs), Variant(rhs)) => ops::product(lhs, rhs),
            (Invariant(lhs), Invariant(rhs)) => Invariant(ops::product(lhs, rhs)),
        }
    }
}

impl<T, B> Zero for Variance<T, B>
where
    T: Zero,
{
    fn zero() -> Self {
        Variance::Invariant(T::zero())
    }
}

pub struct TreeVariance<T>(PhantomData<fn() -> T>);

impl<T> Default for TreeVariance<T> {
    fn default() -> Self {
        TreeVariance(PhantomData)
    }
}

impl<'t, A, T> Fold<'t, A> for TreeVariance<T>
where
    BranchKind<'t, A>: VarianceFold<T>,
    LeafKind<'t>: VarianceTerm<T>,
    T: Invariant,
{
    type Sequencer = Forward;
    type Term = T::Term;

    fn sequencer() -> Self::Sequencer {
        Forward
    }

    fn fold(&mut self, branch: &BranchKind<'t, A>, terms: Vec<Self::Term>) -> Option<Self::Term> {
        branch.fold(terms)
    }

    fn finalize(&mut self, branch: &BranchKind<'t, A>, term: Self::Term) -> Self::Term {
        branch.finalize(term)
    }

    fn term(&mut self, leaf: &LeafKind<'t>) -> Self::Term {
        leaf.term()
    }
}

#[derive(Debug, Default)]
pub struct TreeExhaustiveness;

impl Sequencer for TreeExhaustiveness {
    fn enqueue<'i, 't, A>(
        &mut self,
        parent: ParentToken<'i, 't, A>,
    ) -> impl Iterator<Item = ChildToken<'i, 't, A>> {
        parent.into_tokens().rev().take_while(|token| {
            token.as_ref().as_leaf().is_none_or(|leaf| {
                if let Some(Boundary::Separator) = leaf.boundary() {
                    true
                }
                else {
                    let breadth = self::term::<Breadth>(leaf);
                    let text = self::term::<Text>(leaf);
                    breadth.is_unbounded() && text.is_unbounded()
                }
            })
        })
    }
}

impl<'t, A> Fold<'t, A> for TreeExhaustiveness {
    type Sequencer = Self;
    type Term = InvariantTerm<Depth>;

    fn sequencer() -> Self::Sequencer {
        Self
    }

    fn fold(&mut self, branch: &BranchKind<'t, A>, terms: Vec<Self::Term>) -> Option<Self::Term> {
        let n = terms.len();
        let sum = self::fold::<Depth>(branch, terms);
        if branch.tokens().into_inner().len() == n {
            // No terms were discarded. Yield the sum.
            sum
        }
        else {
            // Terms were discarded, meaning that some non-depth quantity was bounded. Yield the
            // sum only if it may be exhaustive, otherwise zero.
            if sum
                .as_ref()
                .map(BoundaryTerm::is_exhaustive)
                .as_ref()
                .is_some_and(When::is_maybe_true)
            {
                sum
            }
            else {
                Some(Zero::zero())
            }
        }
    }

    fn finalize(&mut self, branch: &BranchKind<'t, A>, term: Self::Term) -> Self::Term {
        use Variance::{Invariant, Variant};

        match branch {
            branch @ BranchKind::Repetition(_) => match term.as_variance() {
                // When folding terms into a repetition, only finalize variant terms and the
                // multiplicative identity and annihilator (one and zero). This is necessary,
                // because natural bounds do not express the subset nor relationship of matched
                // values within the range. Consider `<*/*/>`. This pattern is unbounded w.r.t.
                // depth, but only matches paths with a depth that is a multiple of two and so is
                // nonexhaustive. However, the similar pattern `<*/>` is exhaustive and matches any
                // sub-tree of a match.
                Invariant(&Depth::ZERO) | Invariant(&Depth::ONE) | Variant(_) => {
                    self::finalize::<Depth>(branch, term)
                },
                _ => term,
            },
            branch => self::finalize::<Depth>(branch, term),
        }
    }

    fn term(&mut self, leaf: &LeafKind<'t>) -> Self::Term {
        self::term::<Depth>(leaf)
    }
}

pub fn fold<T>(token: &impl VarianceFold<T>, terms: Vec<T::Term>) -> Option<T::Term>
where
    T: Invariant,
{
    token.fold(terms)
}

pub fn finalize<T>(token: &impl VarianceFold<T>, term: T::Term) -> T::Term
where
    T: Invariant,
{
    token.finalize(term)
}

pub fn term<T>(token: &impl VarianceTerm<T>) -> T::Term
where
    T: Invariant,
{
    token.term()
}

#[cfg(test)]
pub mod harness {
    use std::fmt::Debug;

    use crate::query::When;
    use crate::token::variance::invariant::{Invariant, UnitBound};
    use crate::token::variance::natural::{BoundedVariantRange, NaturalRange};
    use crate::token::variance::{TokenVariance, TreeVariance, Variance};
    use crate::token::{Fold, TokenTree, Tokenized};

    pub fn invariant<T, U>(invariant: U) -> TokenVariance<T>
    where
        T: From<U> + Invariant,
    {
        Variance::Invariant(invariant.into())
    }

    pub fn bounded<T>() -> TokenVariance<T>
    where
        T: Invariant<Bound = UnitBound>,
    {
        Variance::Variant(UnitBound.into())
    }

    pub fn range<T>(lower: usize, upper: impl Into<Option<usize>>) -> TokenVariance<T>
    where
        T: From<usize> + Invariant<Bound = BoundedVariantRange>,
    {
        NaturalRange::from_closed_and_open(lower, upper).map_invariant(From::from)
    }

    pub fn assert_tokenized_variance_eq<'t, A, T>(
        tokenized: Tokenized<'t, A>,
        expected: TokenVariance<T>,
    ) -> Tokenized<'t, A>
    where
        TreeVariance<T>: Fold<'t, A, Term = T::Term>,
        T: Debug + Eq + Invariant,
        T::Bound: Debug + Eq,
    {
        let variance = tokenized.as_token().variance::<T>();
        assert!(
            variance == expected,
            "`Token::variance` is `{:?}`, but expected `{:?}`: in `Tokenized`: `{}`",
            variance,
            expected,
            tokenized.expression(),
        );
        tokenized
    }

    pub fn assert_tokenized_exhaustiveness_eq<A>(
        tokenized: Tokenized<'_, A>,
        expected: When,
    ) -> Tokenized<'_, A> {
        let is_exhaustive = tokenized.as_token().is_exhaustive();
        assert!(
            is_exhaustive == expected,
            "`Token::is_exhaustive` is `{:?}`, but expected `{:?}`: in `Tokenized`: `{}`",
            is_exhaustive,
            expected,
            tokenized.expression(),
        );
        tokenized
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::query::When;
    use crate::token::parse;
    use crate::token::variance::invariant::{Depth, Size, Text};
    use crate::token::variance::{TokenVariance, Variance, harness};

    use When::{Always, Never, Sometimes};

    #[rstest]
    #[case("a", harness::invariant(1))]
    #[case("a/", harness::invariant(1))]
    #[case("a/b", harness::invariant(2))]
    #[case("(?i)a(?-i)b", harness::invariant(1))]
    #[case("{a/b}", harness::invariant(2))]
    #[case("{a,a/b}", harness::range(1, 2))]
    #[case("x{a,a/b}", harness::range(1, 2))]
    #[case("x/{a,a/b}", harness::range(2, 3))]
    #[case("{a,a/b}x", harness::range(1, 2))]
    #[case("{a,a/b}/x", harness::range(2, 3))]
    #[case("{a,{a/b,a/b/c}}", harness::range(1, 3))]
    #[case("{a/,{a/b/,a/b/c/}}x", harness::range(2, 4))]
    #[case("<a/:3>", harness::invariant(3))]
    #[case("<a/:0,3>", harness::range(0, 3))]
    #[case("x<a/:3>", harness::invariant(3))]
    #[case("<a/:3>x", harness::invariant(4))]
    // TODO: Open components must not be empty. This means that `*` must match something if it
    //       comprises a component, for example. This is perhaps more obvious in patterns like
    //       `a/*/b`. However, this is not yet tested and is not consistently emitted by the
    //       encoder: globs likely match incorrectly for such patterns!
    //
    //       Note that patterns that include terminations in components may be empty. For example,
    //       `<*/>` explicitly includes the boundary `/`, and so may match empty content. This is
    //       also true of coalescing patterns (the tree wildcard `**`).
    #[case("*", harness::invariant(1))]
    #[case("a/*", harness::invariant(2))]
    #[case("a/*/b", harness::invariant(3))]
    #[case("*/a", harness::invariant(2))]
    #[case("{a,*}", harness::invariant(1))]
    #[case("<a*/:1,>*", harness::range(2, None))]
    #[case("<*/>", Variance::unbounded())]
    #[case("<*/>*", harness::range(1, None))]
    #[case("<<?>/>*", harness::range(1, None))]
    #[case("<a*/>*", harness::range(1, None))]
    #[case("**", Variance::unbounded())]
    #[case("a/**", harness::range(1, None))]
    #[case("a/**/b", harness::range(2, None))]
    #[case("a/**/b/**/c", harness::range(3, None))]
    #[case("*/**", harness::range(1, None))]
    #[case("**/*", harness::range(1, None))]
    #[case("**/*/**", harness::range(1, None))]
    #[case("{a/b/,c/**/}*.ext", harness::range(2, None))]
    #[case("a/<*/>", harness::range(1, None))]
    #[case("a/<*/>b", harness::range(2, None))]
    #[case("<*/>a</*>", harness::range(1, None))]
    fn parse_expression_depth_variance_eq(
        #[case] expression: &str,
        #[case] expected: TokenVariance<Depth>,
    ) {
        harness::assert_tokenized_variance_eq(
            parse::harness::assert_parse_expression_is_ok(expression),
            expected,
        );
    }

    #[rstest]
    #[case("a", harness::invariant(1))]
    #[case("a/b", harness::invariant(3))]
    #[case("a/**", harness::range(1, None))]
    #[case("<a*/:1,>*", harness::range(2, None))]
    #[case("**", Variance::unbounded())]
    #[case("<*/>*", Variance::unbounded())]
    #[case("<<?>/>*", Variance::unbounded())]
    #[case("<a*/>*", Variance::unbounded())]
    fn parse_expression_size_variance_eq(
        #[case] expression: &str,
        #[case] expected: TokenVariance<Size>,
    ) {
        harness::assert_tokenized_variance_eq(
            parse::harness::assert_parse_expression_is_ok(expression),
            expected,
        );
    }

    #[rstest]
    #[case("a", harness::invariant(Text::from_components(["a"]).unwrap()))]
    #[case("a/b", harness::invariant(Text::from_components(["a", "b"]).unwrap()))]
    #[case("a/**", harness::bounded())]
    #[case("<a*/:1,>*", harness::bounded())]
    #[case("**", Variance::unbounded())]
    #[case("<*/>*", Variance::unbounded())]
    #[case("<<?>/>*", Variance::unbounded())]
    #[case("<a*/>*", Variance::unbounded())]
    fn parse_expression_text_variance_eq(
        #[case] expression: &str,
        #[case] expected: TokenVariance<Text>,
    ) {
        harness::assert_tokenized_variance_eq(
            parse::harness::assert_parse_expression_is_ok(expression),
            expected,
        );
    }

    // TODO: Document the expressions in this test more thoroughly.
    // TODO: Alternations with exhaustive branches that generalize all other branches are always
    //       exhaustive, but this is not detected and `is_exhaustive` incorrectly claims such
    //       alternations are _sometimes_ exhaustive. Consider `{/**,/**/a}`. The `/**/a` branch is
    //       nonexhaustive, but the `/**` branch is exhaustive and also generalizes `/**/a`. The
    //       alternation is always exhaustive, because any match must (also) match the general
    //       branch, and that branch is exhaustive.
    // NOTE: False positives in `Token::is_exhaustive` can cause matching bugs in patterns used as
    //       negations in `FileIterator::not`. False negatives do not introduce logic errors, but
    //       may prevent performance optimizations (e.g., may cause unnecessary file system reads).
    #[rstest]
    #[case("**", Always)]
    #[case("a/**", Always)]
    #[case("<a/**>", Always)]
    #[case("<a/**:1,2>", Always)]
    #[case("<a/<*/:3,>>", Always)]
    #[case("<*/:1,>", Always)]
    #[case("<*/>", Always)]
    #[case("<a/<*/>>", Always)]
    #[case("a/<*/>", Always)]
    #[case("a/<*/>*", Always)]
    #[case("a/<<?>/>*", Always)]
    #[case("{a/**,b/**}", Always)]
    #[case("{{a/**,b/**},c/**}", Always)]
    #[case("<{{a/**,b/**},c/**}:2,4>", Always)]
    #[case("{a/**,b/**,[xyz]<*/>}", Always)]
    #[case("<<?>/>", Always)]
    #[case("<<?>/>*", Always)]
    #[case("<*{/}>", Always)]
    #[case("<*{/,/}>", Always)]
    #[case("<*{/,/**}>", Always)]
    #[case("<{<<{{a/b}}{{/**}}>>}>", Always)]
    // This pattern may match a nonexhaustive alternative that does **not** overlap with another
    // exhaustive alternative (there is no generalizing branch).
    //
    // Consider the match `foo/bar/b`. This matches the second branch of the alternation, but not
    // the first. This pattern does not match any sub-tree of this match other than more `b`
    // components (the first branch will never be matched).
    #[case("{a/**,**/b}", Sometimes)]
    #[case("<{a/**,**/b}:1>", Sometimes)]
    #[case("{{<a:1,>{/**},**/b},c/**}", Sometimes)]
    #[case("", Never)]
    #[case("**/a", Never)]
    #[case("a/**/b", Never)]
    #[case("a/*", Never)]
    #[case("<a/*>", Never)]
    #[case("<*/a>", Never)]
    #[case("<a/*>/*", Never)]
    // This pattern only matches components in groups of two.
    #[case("<*/*/>", Never)]
    // This pattern only matches at most four components.
    #[case("<*/:0,4>", Never)]
    // This pattern only matches two components.
    #[case("a/<?>", Never)]
    #[case("a</**/b>", Never)]
    #[case("<?>", Never)]
    #[case("<?/>", Never)]
    #[case("{**/a,**/b}", Never)]
    fn parse_expression_is_exhaustive_eq(#[case] expression: &str, #[case] expected: When) {
        harness::assert_tokenized_exhaustiveness_eq(
            parse::harness::assert_parse_expression_is_ok(expression),
            expected,
        );
    }
}
