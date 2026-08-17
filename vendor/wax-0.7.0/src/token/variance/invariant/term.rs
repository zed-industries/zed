use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

use crate::token::Composition;
use crate::token::variance::invariant::{Finalize, Invariant, One, Zero};
use crate::token::variance::ops::{self, Conjunction, Disjunction, Product};
use crate::token::variance::{Boundedness, TokenVariance, Variance};

pub type BoundaryTerm<T> = TreeTerm<SeparatedTerm<TokenVariance<T>>>;

impl<T> BoundaryTerm<T>
where
    TokenVariance<T>: Eq + Hash,
    T: Invariant,
{
    pub fn unbounded() -> Self {
        BoundaryTerm::Conjunctive(SeparatedTerm(
            Termination::Coalescent,
            Variance::unbounded(),
        ))
    }

    pub fn as_variance(&self) -> Variance<&T, Option<&Boundedness<T::Bound>>> {
        match self {
            BoundaryTerm::Conjunctive(term) => term.1.as_ref().map_variant(Some),
            BoundaryTerm::Disjunctive(_) => Variance::Variant(None),
        }
    }
}

pub type TreeTerm<T> = Composition<T, DisjunctiveTerm<T>>;

impl<T> Conjunction for TreeTerm<T>
where
    DisjunctiveTerm<T>:
        Conjunction<T, Output = DisjunctiveTerm<T>> + Conjunction<Output = DisjunctiveTerm<T>>,
    T: Conjunction<Output = T>
        + Conjunction<DisjunctiveTerm<T>, Output = DisjunctiveTerm<T>>
        + Eq
        + Hash,
{
    type Output = Self;

    fn conjunction(self, rhs: Self) -> Self::Output {
        use Composition::{Conjunctive, Disjunctive};

        match (self, rhs) {
            (Conjunctive(lhs), Conjunctive(rhs)) => Conjunctive(ops::conjunction(lhs, rhs)),
            (Conjunctive(lhs), Disjunctive(rhs)) => Disjunctive(ops::conjunction(lhs, rhs)),
            (Disjunctive(lhs), Conjunctive(rhs)) => Disjunctive(ops::conjunction(lhs, rhs)),
            (Disjunctive(lhs), Disjunctive(rhs)) => Disjunctive(ops::conjunction(lhs, rhs)),
        }
    }
}

impl<T> Disjunction for TreeTerm<T>
where
    DisjunctiveTerm<T>: Disjunction<T, Output = DisjunctiveTerm<T>>
        + Disjunction<DisjunctiveTerm<T>, Output = DisjunctiveTerm<T>>,
    T: Disjunction<T, Output = DisjunctiveTerm<T>>
        + Disjunction<DisjunctiveTerm<T>, Output = DisjunctiveTerm<T>>
        + Eq
        + Hash,
{
    type Output = Self;

    fn disjunction(self, rhs: Self) -> Self::Output {
        use Composition::{Conjunctive, Disjunctive};

        Disjunctive(match (self, rhs) {
            (Conjunctive(lhs), Conjunctive(rhs)) => ops::disjunction(lhs, rhs),
            (Conjunctive(lhs), Disjunctive(rhs)) => ops::disjunction(lhs, rhs),
            (Disjunctive(lhs), Conjunctive(rhs)) => ops::disjunction(lhs, rhs),
            (Disjunctive(lhs), Disjunctive(rhs)) => ops::disjunction(lhs, rhs),
        })
    }
}

impl<T> Finalize for TreeTerm<T>
where
    DisjunctiveTerm<T>: Finalize<Output = T::Output>,
    T: Eq + Finalize + Hash,
{
    type Output = T::Output;

    fn finalize(self) -> Self::Output {
        match self {
            TreeTerm::Conjunctive(term) => term.finalize(),
            TreeTerm::Disjunctive(term) => term.finalize(),
        }
    }
}

impl<T> One for TreeTerm<T>
where
    T: Eq + Hash + One,
{
    fn one() -> Self {
        TreeTerm::Conjunctive(T::one())
    }
}

impl<T, U> Product<U> for TreeTerm<T>
where
    T: Eq + Hash + Product<U>,
    T::Output: Eq + Hash,
    U: Clone,
{
    type Output = TreeTerm<T::Output>;

    fn product(self, rhs: U) -> Self::Output {
        match self {
            TreeTerm::Conjunctive(lhs) => TreeTerm::Conjunctive(ops::product(lhs, rhs)),
            TreeTerm::Disjunctive(lhs) => TreeTerm::Disjunctive(ops::product(lhs, rhs)),
        }
    }
}

impl<T> Zero for TreeTerm<T>
where
    T: Eq + Hash + Zero,
{
    fn zero() -> Self {
        TreeTerm::Conjunctive(T::zero())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisjunctiveTerm<T>(HashSet<T>)
where
    T: Eq + Hash;

impl<T> DisjunctiveTerm<T>
where
    T: Eq + Hash,
{
    fn remap<U, F>(self, f: F) -> DisjunctiveTerm<U>
    where
        U: Eq + Hash,
        F: FnMut(T) -> U,
    {
        DisjunctiveTerm(self.0.into_iter().map(f).collect())
    }

    pub fn branches(&self) -> impl '_ + Iterator<Item = &'_ T> {
        self.0.iter()
    }
}

impl<T> Conjunction for DisjunctiveTerm<T>
where
    T: Clone + Conjunction<T, Output = T> + Eq + Hash,
{
    type Output = Self;

    fn conjunction(self, rhs: Self) -> Self::Output {
        DisjunctiveTerm(
            self.0
                .into_iter()
                .cartesian_product(rhs.0.iter())
                .map(|(lhs, rhs)| ops::conjunction(lhs, rhs.clone()))
                .collect(),
        )
    }
}

impl<T> Conjunction<T> for DisjunctiveTerm<T>
where
    T: Clone + Conjunction<T, Output = T> + Eq + Hash,
{
    type Output = Self;

    fn conjunction(self, rhs: T) -> Self::Output {
        self.remap(|lhs| ops::conjunction(lhs, rhs.clone()))
    }
}

// This is the same as the above `Conjunction` implementation, but with transposed operand order.
// However, it is limited to `SeparatedTerm`s (rather than any `T`), because otherwise this
// implementation conflicts with an implementation for `UnitBound`.
impl<T> Conjunction<DisjunctiveTerm<SeparatedTerm<T>>> for SeparatedTerm<T>
where
    SeparatedTerm<T>: Clone + Conjunction<Output = SeparatedTerm<T>> + Eq + Hash,
{
    type Output = DisjunctiveTerm<SeparatedTerm<T>>;

    fn conjunction(self, rhs: DisjunctiveTerm<SeparatedTerm<T>>) -> Self::Output {
        rhs.remap(|rhs| ops::conjunction(self.clone(), rhs))
    }
}

impl<T> Disjunction for DisjunctiveTerm<T>
where
    T: Eq + Hash,
{
    type Output = Self;

    fn disjunction(self, rhs: Self) -> Self::Output {
        DisjunctiveTerm(self.0.into_iter().chain(rhs.0).collect())
    }
}

impl<T> Disjunction<T> for DisjunctiveTerm<T>
where
    T: Eq + Hash,
{
    type Output = Self;

    fn disjunction(mut self, rhs: T) -> Self::Output {
        self.0.insert(rhs);
        self
    }
}

impl<T> Disjunction<DisjunctiveTerm<T>> for T
where
    T: Eq + Hash,
{
    type Output = DisjunctiveTerm<T>;

    fn disjunction(self, mut rhs: DisjunctiveTerm<T>) -> Self::Output {
        rhs.0.insert(self);
        rhs
    }
}

impl<T> Finalize for DisjunctiveTerm<T>
where
    T: Eq + Finalize + Hash,
    T::Output: Disjunction<T::Output, Output = T::Output> + Zero,
{
    type Output = T::Output;

    fn finalize(self) -> Self::Output {
        self.0
            .into_iter()
            .map(Finalize::finalize)
            .reduce(ops::disjunction)
            .unwrap_or_else(Zero::zero)
    }
}

impl<T, U> Product<U> for DisjunctiveTerm<T>
where
    T: Eq + Hash + Product<U>,
    T::Output: Eq + Hash,
    U: Clone,
{
    type Output = DisjunctiveTerm<T::Output>;

    fn product(self, rhs: U) -> Self::Output {
        self.remap(|lhs| ops::product(lhs, rhs.clone()))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Coalescence<T> {
    Left(T),
    Right(T),
    Neither(T),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Termination {
    Open,
    First,
    Last,
    Closed,
    Coalescent,
}

impl Conjunction for Termination {
    type Output = Coalescence<Self>;

    fn conjunction(self, rhs: Self) -> Self::Output {
        use Coalescence::{Left, Neither, Right};
        use Termination::{Closed, Coalescent, First, Last, Open};

        match (self, rhs) {
            (Coalescent, Coalescent) => Neither(Coalescent),
            (Closed, Closed) | (First, Last | Closed) | (Closed, Last) => Neither(Closed),
            (Last | Open, Closed) | (Last | Open, Last) => Neither(Last),
            (Closed, First | Open) | (First, First | Open) => Neither(First),
            (Open, First | Open) | (Last, First | Open) => Neither(Open),
            (Coalescent, Closed | Last) => Right(Closed),
            (Closed | First, Coalescent) => Left(Closed),
            (Last | Open, Coalescent) => Left(Last),
            (Coalescent, First | Open) => Right(First),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SeparatedTerm<T>(pub Termination, pub T);

impl<T> AsMut<T> for SeparatedTerm<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<T> AsRef<T> for SeparatedTerm<T> {
    fn as_ref(&self) -> &T {
        &self.1
    }
}

impl<T, U> Conjunction<SeparatedTerm<U>> for SeparatedTerm<T>
where
    SeparatedTerm<T>: Finalize<Output = T>,
    SeparatedTerm<U>: Finalize<Output = U>,
    T: Conjunction<U>,
{
    type Output = SeparatedTerm<T::Output>;

    fn conjunction(self, rhs: SeparatedTerm<U>) -> Self::Output {
        use Coalescence::{Left, Neither, Right};

        let lhs = self;
        let (termination, lhs, rhs) = match ops::conjunction(lhs.0, rhs.0) {
            Left(termination) => (termination, lhs.finalize(), rhs.1),
            Right(termination) => (termination, lhs.1, rhs.finalize()),
            Neither(termination) => (termination, lhs.1, rhs.1),
        };
        SeparatedTerm(termination, ops::conjunction(lhs, rhs))
    }
}

impl<T> Disjunction for SeparatedTerm<T>
where
    T: Eq + Hash,
{
    type Output = DisjunctiveTerm<Self>;

    fn disjunction(self, rhs: Self) -> Self::Output {
        DisjunctiveTerm([self, rhs].into_iter().collect())
    }
}

impl<T> One for SeparatedTerm<T>
where
    T: One,
{
    fn one() -> Self {
        SeparatedTerm(Termination::Closed, T::one())
    }
}

impl<T, U> Product<U> for SeparatedTerm<T>
where
    T: Product<U>,
{
    type Output = SeparatedTerm<T::Output>;

    fn product(self, rhs: U) -> Self::Output {
        SeparatedTerm(self.0, ops::product(self.1, rhs))
    }
}

impl<T> Zero for SeparatedTerm<T>
where
    T: Zero,
{
    fn zero() -> Self {
        SeparatedTerm(Termination::Open, T::zero())
    }
}
