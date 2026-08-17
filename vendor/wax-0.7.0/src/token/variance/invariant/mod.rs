mod term;
mod text;

use std::num::NonZeroUsize;

use crate::query::When;
use crate::token::variance::natural::{
    BoundedVariantRange, OpenedUpperBound, define_natural_invariant,
};
use crate::token::variance::ops::{self, Conjunction, Disjunction, Product};
use crate::token::variance::{Boundedness, TokenVariance};

pub use crate::token::variance::invariant::term::{BoundaryTerm, SeparatedTerm, Termination};
pub use crate::token::variance::invariant::text::{IntoNominalText, IntoStructuralText, Text};

pub type InvariantTerm<T> = <T as Invariant>::Term;
pub type InvariantBound<T> = <T as Invariant>::Bound;

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait Finalize: Sized {
    type Output;

    fn finalize(self) -> Self::Output;
}

pub trait Invariant: Sized + Zero {
    type Term: Finalize<Output = TokenVariance<Self>> + Zero;
    type Bound;

    fn bound(lhs: Self, rhs: Self) -> Boundedness<Self::Bound>;

    fn into_lower_bound(self) -> Boundedness<Self::Bound>;
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UnitBound;

impl<T> Conjunction<T> for UnitBound {
    type Output = Self;

    fn conjunction(self, _: T) -> Self::Output {
        self
    }
}

impl Disjunction for UnitBound {
    type Output = Boundedness<Self>;

    fn disjunction(self, _: Self) -> Self::Output {
        self.into()
    }
}

impl From<()> for UnitBound {
    fn from(_: ()) -> Self {
        UnitBound
    }
}

impl From<BoundedVariantRange> for UnitBound {
    fn from(_: BoundedVariantRange) -> Self {
        UnitBound
    }
}

impl OpenedUpperBound for UnitBound {
    fn opened_upper_bound(self) -> Boundedness<Self> {
        UnitBound.into()
    }
}

impl Product<BoundedVariantRange> for UnitBound {
    type Output = Boundedness<Self>;

    fn product(self, _: BoundedVariantRange) -> Self::Output {
        self.into()
    }
}

impl Product<NonZeroUsize> for Boundedness<UnitBound> {
    type Output = Self;

    fn product(self, _: NonZeroUsize) -> Self::Output {
        self
    }
}

// Breadth is the maximum size (UTF-8 bytes) of component text in a glob expression. For example,
// the breadth of `{a/,a/b/}<c/d:2>` is two (bytes).
//
// Composition is not implemented for breadth and it is only queried from leaf tokens. No values
// are associated with its invariant nor bounds. Breadth is probably the least interesting and yet
// most difficult quantity to compute. Determining correct breadth values likely involves:
//
// - Complex terms that support both running sums and running maximums across boundaries.
// - Searches from the start and end of sub-trees in repetitions, where terminals may interact.
//   Consider `<a/b:2>` vs. `<a/b/:2>`, which have breadths of two and one, respectively.
// - Potentially large sets for bounds. Ranges lose too much information, in particular information
//   about boundaries when composing terms.
//
// There are likely other difficult composition problems. As such, breadth is only implemented as
// much as needed. Any requirements on more complete breadth computations ought to be considered
// carefully.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Breadth;

impl Breadth {
    pub const fn new() -> Self {
        Breadth
    }
}

impl Zero for Breadth {
    fn zero() -> Self {
        Breadth
    }
}

impl Invariant for Breadth {
    type Term = TokenVariance<Self>;
    type Bound = ();

    fn bound(_: Self, _: Self) -> Boundedness<Self::Bound> {
        ().into()
    }

    fn into_lower_bound(self) -> Boundedness<Self::Bound> {
        ().into()
    }
}

define_natural_invariant!(
    /// Depth of a glob expression.
    Depth => BoundaryTerm,
);

impl TokenVariance<Depth> {
    pub fn is_exhaustive(&self) -> bool {
        !self.has_upper_bound()
    }
}

impl BoundaryTerm<Depth> {
    pub fn is_exhaustive(&self) -> When {
        match self {
            BoundaryTerm::Conjunctive(SeparatedTerm(_, term)) => term.is_exhaustive().into(),
            BoundaryTerm::Disjunctive(term) => term
                .branches()
                .map(AsRef::as_ref)
                .map(TokenVariance::<Depth>::is_exhaustive)
                .map(When::from)
                .reduce(When::certainty)
                .unwrap_or(When::Never),
        }
    }
}

impl Finalize for SeparatedTerm<TokenVariance<Depth>> {
    type Output = TokenVariance<Depth>;

    fn finalize(self) -> Self::Output {
        use Termination::{Closed, Open};

        let SeparatedTerm(termination, term) = self;
        match termination {
            Open => ops::conjunction(term, One::one()),
            Closed => term.map_invariant(|term| term.map(|term| term.saturating_sub(1))),
            _ => term,
        }
    }
}

define_natural_invariant!(
    /// Size of a glob expression.
    Size,
);
