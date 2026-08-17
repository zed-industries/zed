//! Helper traits for tupling/untupling

use crate::stats::Distribution;

/// Any tuple: `(A, B, ..)`
pub trait Tuple: Sized {
    /// A tuple of distributions associated with this tuple
    type Distributions: TupledDistributions<Item = Self>;

    /// A tuple of vectors associated with this tuple
    type Builder: TupledDistributionsBuilder<Item = Self>;
}

/// A tuple of distributions: `(Distribution<A>, Distribution<B>, ..)`
pub trait TupledDistributions: Sized {
    /// A tuple that can be pushed/inserted into the tupled distributions
    type Item: Tuple<Distributions = Self>;
}

/// A tuple of vecs used to build distributions.
pub trait TupledDistributionsBuilder: Sized {
    /// A tuple that can be pushed/inserted into the tupled distributions
    type Item: Tuple<Builder = Self>;

    /// Creates a new tuple of vecs
    fn new(size: usize) -> Self;

    /// Push one element into each of the vecs
    fn push(&mut self, tuple: Self::Item);

    /// Append one tuple of vecs to this one, leaving the vecs in the other tuple empty
    fn extend(&mut self, other: &mut Self);

    /// Convert the tuple of vectors into a tuple of distributions
    fn complete(self) -> <Self::Item as Tuple>::Distributions;
}

impl<A> Tuple for (A,)
where
    A: Copy,
{
    type Distributions = (Distribution<A>,);
    type Builder = (Vec<A>,);
}

impl<A> TupledDistributions for (Distribution<A>,)
where
    A: Copy,
{
    type Item = (A,);
}
impl<A> TupledDistributionsBuilder for (Vec<A>,)
where
    A: Copy,
{
    type Item = (A,);

    fn new(size: usize) -> (Vec<A>,) {
        (Vec::with_capacity(size),)
    }

    fn push(&mut self, tuple: (A,)) {
        (self.0).push(tuple.0);
    }

    fn extend(&mut self, other: &mut (Vec<A>,)) {
        (self.0).append(&mut other.0);
    }

    fn complete(self) -> (Distribution<A>,) {
        (Distribution(self.0.into_boxed_slice()),)
    }
}

impl<A, B> Tuple for (A, B)
where
    A: Copy,
    B: Copy,
{
    type Distributions = (Distribution<A>, Distribution<B>);
    type Builder = (Vec<A>, Vec<B>);
}

impl<A, B> TupledDistributions for (Distribution<A>, Distribution<B>)
where
    A: Copy,
    B: Copy,
{
    type Item = (A, B);
}
impl<A, B> TupledDistributionsBuilder for (Vec<A>, Vec<B>)
where
    A: Copy,
    B: Copy,
{
    type Item = (A, B);

    fn new(size: usize) -> (Vec<A>, Vec<B>) {
        (Vec::with_capacity(size), Vec::with_capacity(size))
    }

    fn push(&mut self, tuple: (A, B)) {
        (self.0).push(tuple.0);
        (self.1).push(tuple.1);
    }

    fn extend(&mut self, other: &mut (Vec<A>, Vec<B>)) {
        (self.0).append(&mut other.0);
        (self.1).append(&mut other.1);
    }

    fn complete(self) -> (Distribution<A>, Distribution<B>) {
        (
            Distribution(self.0.into_boxed_slice()),
            Distribution(self.1.into_boxed_slice()),
        )
    }
}

impl<A, B, C> Tuple for (A, B, C)
where
    A: Copy,
    B: Copy,
    C: Copy,
{
    type Distributions = (Distribution<A>, Distribution<B>, Distribution<C>);
    type Builder = (Vec<A>, Vec<B>, Vec<C>);
}

impl<A, B, C> TupledDistributions for (Distribution<A>, Distribution<B>, Distribution<C>)
where
    A: Copy,
    B: Copy,
    C: Copy,
{
    type Item = (A, B, C);
}
impl<A, B, C> TupledDistributionsBuilder for (Vec<A>, Vec<B>, Vec<C>)
where
    A: Copy,
    B: Copy,
    C: Copy,
{
    type Item = (A, B, C);

    fn new(size: usize) -> (Vec<A>, Vec<B>, Vec<C>) {
        (
            Vec::with_capacity(size),
            Vec::with_capacity(size),
            Vec::with_capacity(size),
        )
    }

    fn push(&mut self, tuple: (A, B, C)) {
        (self.0).push(tuple.0);
        (self.1).push(tuple.1);
        (self.2).push(tuple.2);
    }

    fn extend(&mut self, other: &mut (Vec<A>, Vec<B>, Vec<C>)) {
        (self.0).append(&mut other.0);
        (self.1).append(&mut other.1);
        (self.2).append(&mut other.2);
    }

    fn complete(self) -> (Distribution<A>, Distribution<B>, Distribution<C>) {
        (
            Distribution(self.0.into_boxed_slice()),
            Distribution(self.1.into_boxed_slice()),
            Distribution(self.2.into_boxed_slice()),
        )
    }
}

impl<A, B, C, D> Tuple for (A, B, C, D)
where
    A: Copy,
    B: Copy,
    C: Copy,
    D: Copy,
{
    type Distributions = (
        Distribution<A>,
        Distribution<B>,
        Distribution<C>,
        Distribution<D>,
    );
    type Builder = (Vec<A>, Vec<B>, Vec<C>, Vec<D>);
}

impl<A, B, C, D> TupledDistributions
    for (
        Distribution<A>,
        Distribution<B>,
        Distribution<C>,
        Distribution<D>,
    )
where
    A: Copy,
    B: Copy,
    C: Copy,
    D: Copy,
{
    type Item = (A, B, C, D);
}
impl<A, B, C, D> TupledDistributionsBuilder for (Vec<A>, Vec<B>, Vec<C>, Vec<D>)
where
    A: Copy,
    B: Copy,
    C: Copy,
    D: Copy,
{
    type Item = (A, B, C, D);

    fn new(size: usize) -> (Vec<A>, Vec<B>, Vec<C>, Vec<D>) {
        (
            Vec::with_capacity(size),
            Vec::with_capacity(size),
            Vec::with_capacity(size),
            Vec::with_capacity(size),
        )
    }

    fn push(&mut self, tuple: (A, B, C, D)) {
        (self.0).push(tuple.0);
        (self.1).push(tuple.1);
        (self.2).push(tuple.2);
        (self.3).push(tuple.3);
    }

    fn extend(&mut self, other: &mut (Vec<A>, Vec<B>, Vec<C>, Vec<D>)) {
        (self.0).append(&mut other.0);
        (self.1).append(&mut other.1);
        (self.2).append(&mut other.2);
        (self.3).append(&mut other.3);
    }

    fn complete(
        self,
    ) -> (
        Distribution<A>,
        Distribution<B>,
        Distribution<C>,
        Distribution<D>,
    ) {
        (
            Distribution(self.0.into_boxed_slice()),
            Distribution(self.1.into_boxed_slice()),
            Distribution(self.2.into_boxed_slice()),
            Distribution(self.3.into_boxed_slice()),
        )
    }
}
