//! Separating filters that do not discard residue.
//!
//! This module provides APIs for separating filters that partition a feed (input) into a filtrate
//! and residue on a per-item basis. The primary API is the [`SeparatingFilter`] trait, which
//! behaves much like the standard [`Iterator`] trait and is similarly composable via combinators.
//!
//! Unlike [`Iterator::filter`] and similar APIs, [`SeparatingFilter`]s do **not** discard residue.
//! Instead, both the filtrate and the residue is present and can therefore be observed by composed
//! filters. This is important if combinators have side effects that depend on observing filtered
//! data from upstream filters. For example, [`HierarchicalIterator`] provides combinators for
//! filtering that can affect the traversal of a tree by discarding a sub-tree. This behavior may
//! need to be invoked regardless of whether or not data is filtrate or residue.
//!
//! [`SeparatingFilter`]s may also be [`Iterator`]s, in which case only the filtrate is typically
//! exposed in the [`Iterator`] API (the associated `Item` type is the `Filtrate` type of the
//! filter's associated `Feed` type).
//!
//! [`Iterator`]: std::iter::Iterator
//! [`Iterator::filter`]: std::iter::Iterator::filter
//! [`SeparatingFilter`]: crate::walk::filter::SeparatingFilter

#![cfg_attr(not(feature = "walk"), allow(dead_code))]

use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

mod kind {
    #[derive(Debug)]
    pub enum FiltrateKind {}
    #[derive(Debug)]
    pub enum ResidueKind {}
}
use kind::*;

#[derive(Debug)]
pub struct Product<K, T> {
    inner: T,
    _phantom: PhantomData<fn() -> K>,
}

impl<K, T> Product<K, T> {
    pub(super) fn new(inner: T) -> Self {
        Product {
            inner,
            _phantom: PhantomData,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn map<U, F>(self, f: F) -> Product<K, U>
    where
        F: FnOnce(T) -> U,
    {
        Product::new(f(self.into_inner()))
    }

    pub fn get(&self) -> &T {
        &self.inner
    }
}

impl<K, T> Product<K, Option<T>> {
    pub fn transpose(self) -> Option<Product<K, T>> {
        self.inner.map(Product::new)
    }
}

impl<K, T, E> Product<K, Result<T, E>> {
    pub fn transpose(self) -> Result<Product<K, T>, Product<K, E>> {
        self.inner.map(Product::new).map_err(Product::new)
    }
}

impl<K, T> AsRef<T> for Product<K, T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<K, T> Copy for Product<K, T> where T: Copy {}

impl<K, T> Clone for Product<K, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Product::new(self.inner.clone())
    }
}

impl<K, T> Eq for Product<K, T> where T: Eq {}

impl<K, T> Hash for Product<K, T>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.inner.hash(state);
    }
}

impl<K, T> PartialEq<Self> for Product<K, T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

pub type Filtrate<T> = Product<FiltrateKind, T>;
pub type Residue<T> = Product<ResidueKind, T>;

impl<T> Filtrate<T> {
    pub fn filter(self) -> Residue<T> {
        self.filter_map(From::from)
    }

    pub fn filter_map<R, F>(self, f: F) -> Residue<R>
    where
        F: FnOnce(T) -> R,
    {
        Residue::new(f(self.into_inner()))
    }

    pub fn filter_node<R>(self) -> Residue<TreeResidue<R>>
    where
        R: From<T>,
    {
        self.filter_map_node(From::from)
    }

    pub fn filter_map_node<R, F>(self, f: F) -> Residue<TreeResidue<R>>
    where
        F: FnOnce(T) -> R,
    {
        Residue::new(TreeResidue::Node(f(self.into_inner())))
    }

    pub fn filter_tree<I, R>(self, cancellation: WalkCancellation<'_, I>) -> Residue<TreeResidue<R>>
    where
        I: CancelWalk,
        R: From<T>,
    {
        self.filter_map_tree(cancellation, From::from)
    }

    pub fn filter_map_tree<I, R, F>(
        self,
        cancellation: WalkCancellation<'_, I>,
        f: F,
    ) -> Residue<TreeResidue<R>>
    where
        I: CancelWalk,
        F: FnOnce(T) -> R,
    {
        cancellation.cancel_walk_tree();
        Residue::new(TreeResidue::Tree(f(self.into_inner())))
    }
}

impl<T> AsRef<T> for Residue<TreeResidue<T>> {
    fn as_ref(&self) -> &T {
        self.get().get()
    }
}

/// Describes the input and output types of a [`SeparatingFilter`].
///
/// `Feed` types are typically represented as tuples of the filtrate and residue types, in that
/// order.
///
/// [`SeparatingFilter`]: crate::walk::filter::SeparatingFilter
pub trait Feed {
    type Filtrate;
    type Residue;
}

impl<T, R> Feed for (T, R) {
    type Filtrate = T;
    type Residue = R;
}

/// A filter [`Feed`] wherein the filtrate and residue have or can produce a substituent.
///
/// A substituent is data that is common to both the filtrate and residue (that is, the filtrate
/// and residue are isomers). An `Isomeric` feed can be filtered nominally on the basis of its
/// substituent. See [`Separation::filter_tree_by_substituent`] and
/// [`HierarchicalIterator::filter_tree_by_substituent`], for example.
///
/// [`Feed`]: crate::walk::filter::Feed
/// [`HierarchicalIterator::filter_tree_by_substituent`]: crate::walk::filter::HierarchicalIterator::filter_tree_by_substituent
/// [`Separation::filter_tree_by_substituent`]: crate::walk::filter::Separation::filter_tree_by_substituent
pub trait Isomeric: Feed {
    type Substituent<'a>
    where
        Self: 'a;

    fn substituent(separation: &Separation<Self>) -> Self::Substituent<'_>;
}

// TODO: The derived trait implementations are likely incorrect and imply a bound on `S`. It just
//       so happens that `S` is typically a tuple and tuples will implement these same traits if
//       the composed types implement them. Instead, the bounds ought to be on the associated
//       `Filtrate` and `Residue` types.
/// The separated output of a [`SeparatingFilter`] feed.
///
/// [`SeparatingFilter`]: crate::walk::filter::SeparatingFilter
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Separation<S>
where
    S: Feed + ?Sized,
{
    Filtrate(Filtrate<S::Filtrate>),
    Residue(Residue<S::Residue>),
}

impl<S> Separation<S>
where
    S: Feed,
{
    fn from_inner_filtrate(filtrate: S::Filtrate) -> Self {
        Separation::Filtrate(Filtrate::new(filtrate))
    }

    fn from_inner_residue(residue: S::Residue) -> Self {
        Separation::Residue(Residue::new(residue))
    }

    pub fn filter_map<F>(self, f: F) -> Self
    where
        F: FnOnce(S::Filtrate) -> S::Residue,
    {
        match self {
            Separation::Filtrate(filtrate) => {
                Separation::from_inner_residue(f(filtrate.into_inner()))
            },
            separation => separation,
        }
    }

    pub fn map_filtrate<U, F>(self, f: F) -> Separation<(U, S::Residue)>
    where
        F: FnOnce(S::Filtrate) -> U,
    {
        match self {
            Separation::Filtrate(filtrate) => filtrate.map(f).into(),
            Separation::Residue(residue) => residue.into(),
        }
    }

    pub fn map_residue<U, F>(self, f: F) -> Separation<(S::Filtrate, U)>
    where
        F: FnOnce(S::Residue) -> U,
    {
        match self {
            Separation::Filtrate(filtrate) => filtrate.into(),
            Separation::Residue(residue) => residue.map(f).into(),
        }
    }

    pub fn filtrate(self) -> Option<Filtrate<S::Filtrate>> {
        match self {
            Separation::Filtrate(filtrate) => Some(filtrate),
            _ => None,
        }
    }

    pub fn as_filtrate(&self) -> Option<&Filtrate<S::Filtrate>> {
        match self {
            Separation::Filtrate(filtrate) => Some(filtrate),
            _ => None,
        }
    }

    pub fn as_ref(&self) -> Separation<(&S::Filtrate, &S::Residue)> {
        match self {
            Separation::Filtrate(filtrate) => {
                Separation::Filtrate(Filtrate::new(filtrate.as_ref()))
            },
            Separation::Residue(residue) => Separation::Residue(Residue::new(residue.as_ref())),
        }
    }

    pub fn substituent(&self) -> S::Substituent<'_>
    where
        S: Isomeric,
    {
        S::substituent(self)
    }
}

impl<T, R, S> Separation<S>
where
    S: Feed<Filtrate = T, Residue = TreeResidue<R>>,
{
    pub fn filter_map_node<F>(self, f: F) -> Self
    where
        F: FnOnce(T) -> R,
    {
        self.filter_map(|filtrate| TreeResidue::Node(f(filtrate)))
    }

    pub fn filter_map_tree<I, F>(self, cancellation: WalkCancellation<'_, I>, f: F) -> Self
    where
        I: CancelWalk,
        F: FnOnce(T) -> R,
    {
        match self {
            Separation::Filtrate(filtrate) => {
                cancellation.cancel_walk_tree();
                Separation::from_inner_residue(TreeResidue::Node(f(filtrate.into_inner())))
            },
            Separation::Residue(residue) => match residue.into_inner() {
                TreeResidue::Node(residue) => {
                    cancellation.cancel_walk_tree();
                    Separation::from_inner_residue(TreeResidue::Tree(residue))
                },
                residue => Separation::from_inner_residue(residue),
            },
        }
    }

    pub fn filter_tree_by_substituent<I, F>(
        self,
        cancellation: WalkCancellation<'_, I>,
        f: F,
    ) -> Self
    where
        S: Isomeric,
        R: From<T>,
        I: CancelWalk,
        F: FnOnce(S::Substituent<'_>) -> Option<TreeResidue<()>>,
    {
        match f(self.substituent()) {
            Some(TreeResidue::Tree(())) => self.filter_map_tree(cancellation, From::from),
            Some(TreeResidue::Node(())) => self.filter_map_node(From::from),
            _ => self,
        }
    }
}

impl<T, R> Separation<(Option<T>, R)> {
    pub fn transpose_filtrate(self) -> Option<Separation<(T, R)>> {
        match self {
            Separation::Filtrate(filtrate) => {
                filtrate.into_inner().map(Filtrate::new).map(From::from)
            },
            Separation::Residue(residue) => Some(residue.into()),
        }
    }
}

impl<T, E, R> Separation<(Result<T, E>, R)> {
    pub fn transpose_filtrate(self) -> Result<Separation<(T, R)>, Filtrate<E>> {
        match self {
            Separation::Filtrate(filtrate) => match filtrate.into_inner() {
                Ok(filtrate) => Ok(Filtrate::new(filtrate).into()),
                Err(error) => Err(Filtrate::new(error)),
            },
            Separation::Residue(residue) => Ok(residue.into()),
        }
    }
}

impl<S> From<Filtrate<S::Filtrate>> for Separation<S>
where
    S: Feed,
{
    fn from(filtrate: Filtrate<S::Filtrate>) -> Self {
        Separation::Filtrate(filtrate)
    }
}

impl<S> From<Residue<S::Residue>> for Separation<S>
where
    S: Feed,
{
    fn from(residue: Residue<S::Residue>) -> Self {
        Separation::Residue(residue)
    }
}

pub trait SeparatingFilter {
    type Feed: Feed;

    fn feed(&mut self) -> Option<Separation<Self::Feed>>;
}

impl<I> SeparatingFilter for I
where
    I: SeparatingFilterInput,
{
    type Feed = <I as SeparatingFilterInput>::Feed;

    fn feed(&mut self) -> Option<Separation<Self::Feed>> {
        self.next().map(Separation::from_inner_filtrate)
    }
}

/// [`Iterator`] that provides filtrate input for [`SeparatingFilter`]s.
///
/// **This trait provides the only API for implementing a [`SeparatingFilter`].** [`Iterator`]s can
/// implement this trait for a transitive [`SeparatingFilter`] implemention that provides all items
/// as filtrate. This bridges [`Iterator`]s into the input of a separating filter. See the
/// [`filtrate`] function for the output analog.
///
/// [`filtrate`]: crate::walk::filter::filtrate
/// [`Iterator`]: std::iter::Iterator
/// [`SeparatingFilter`]: crate::walk::filter::SeparatingFilter
pub trait SeparatingFilterInput: Iterator {
    type Feed: Feed<Filtrate = Self::Item>;
}

/// A tree traversing type that can cancel traversal into the most recently visited node.
///
/// See [`HierarchicalIterator`].
///
/// [`HierarchicalIterator`]: crate::walk::filter::HierarchicalIterator
pub trait CancelWalk {
    fn cancel_walk_tree(&mut self);
}

/// Cancels traversal into a sub-tree of a [`HierarchicalIterator`].
///
/// [`HierarchicalIterator`]: crate::walk::filter::HierarchicalIterator
#[derive(Debug)]
pub struct WalkCancellation<'i, I>(&'i mut I);

impl<'i, I> WalkCancellation<'i, I> {
    // TODO: This module should not allow this at all and `WalkCancellation`, much like
    //       `Product`, should not be possible to construct outside of the module. Instead,
    //       client code should rely solely on combinators, but this requires RPITIT to write
    //       combinators with arbitrary output types (like unnameable `FnMut`s).
    //
    //       RPITIT is slated to land at the end of December of 2023. Remove this and implement
    //       iterators using pure combinators when that happens.
    pub(crate) fn unchecked(tree: &'i mut I) -> Self {
        WalkCancellation(tree)
    }
}

impl<'i, I> WalkCancellation<'i, I>
where
    I: CancelWalk,
{
    fn cancel_walk_tree(self) {
        // Client code is only able to move a `WalkCancellation` into
        // `Separation::filter_map_tree`, at which point the filtered item should be the current
        // item of the iterator.
        self.0.cancel_walk_tree()
    }
}

/// Residue of a [`SeparatingFilter`] over a tree data structure.
///
/// [`SeparatingFilter`]: crate::walk::filter::SeparatingFilter
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TreeResidue<T> {
    Node(T),
    Tree(T),
}

impl<T> TreeResidue<T> {
    pub fn map<U, F>(self, f: F) -> TreeResidue<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            TreeResidue::Node(residue) => TreeResidue::Node(f(residue)),
            TreeResidue::Tree(residue) => TreeResidue::Tree(f(residue)),
        }
    }

    pub fn as_ref(&self) -> TreeResidue<&T> {
        match self {
            TreeResidue::Node(residue) => TreeResidue::Node(residue),
            TreeResidue::Tree(residue) => TreeResidue::Tree(residue),
        }
    }

    pub fn get(&self) -> &T {
        match self {
            TreeResidue::Node(residue) | TreeResidue::Tree(residue) => residue,
        }
    }
}

/// Hierarchical iterator over items in a tree data structure.
///
/// Here, _hierarchical_ means that the iterator traverses the tree in a manner that never yields a
/// node before its ancestors (e.g., a child node before its parent node). Both pre-order DFS and
/// BFS are examples of such a traversal.
///
/// `HierarchicalIterator` allows client code to control tree traversal when filtering items using
/// a `WalkCancellation`, which discards a particular node and cancels traversal to its child nodes
/// (sub-tree). Filtering a sub-tree completely discards that tree, and no filter separation is
/// produced (no filtrate nor residue). This is the only separating filter operation that
/// explicitly discards a retentate and no downstream filters can observe the discarded sub-tree.
pub trait HierarchicalIterator:
    CancelWalk + Iterator<Item = <Self::Feed as Feed>::Filtrate> + SeparatingFilter
{
    fn filter_tree_by_substituent<F>(self, f: F) -> FilterTreeBySubstituent<Self, F>
    where
        Self: Sized,
        Self::Feed: Isomeric,
        F: FnMut(<Self::Feed as Isomeric>::Substituent<'_>) -> Option<TreeResidue<()>>,
    {
        FilterTreeBySubstituent { input: self, f }
    }

    fn filter_map_tree<S, F>(self, f: F) -> FilterMapTree<Self, S, F>
    where
        Self: Sized,
        S: Feed,
        F: FnMut(WalkCancellation<Self>, Separation<Self::Feed>) -> Separation<S>,
    {
        FilterMapTree {
            input: self,
            f,
            _phantom: PhantomData,
        }
    }
}

impl<R, I> HierarchicalIterator for I
where
    I: CancelWalk + Iterator<Item = <Self::Feed as Feed>::Filtrate> + SeparatingFilter,
    I::Feed: Feed<Residue = TreeResidue<R>>,
{
}

#[derive(Clone, Debug)]
pub struct FilterTreeBySubstituent<I, F> {
    input: I,
    f: F,
}

impl<I, F> CancelWalk for FilterTreeBySubstituent<I, F>
where
    I: CancelWalk,
{
    fn cancel_walk_tree(&mut self) {
        self.input.cancel_walk_tree()
    }
}

impl<R, I, F> SeparatingFilter for FilterTreeBySubstituent<I, F>
where
    R: From<<I::Feed as Feed>::Filtrate>,
    I: HierarchicalIterator,
    I::Feed: Feed<Residue = TreeResidue<R>> + Isomeric,
    F: FnMut(<I::Feed as Isomeric>::Substituent<'_>) -> Option<TreeResidue<()>>,
{
    type Feed = I::Feed;

    fn feed(&mut self) -> Option<Separation<Self::Feed>> {
        let separation = self.input.feed();
        separation.map(|separation| {
            let substituent = separation.substituent();
            match (self.f)(substituent) {
                None => separation,
                Some(residue) => match residue {
                    TreeResidue::Node(_) => separation.filter_map_node(From::from),
                    TreeResidue::Tree(_) => {
                        separation.filter_map_tree(WalkCancellation(self), From::from)
                    },
                },
            }
        })
    }
}

impl<R, I, F> Iterator for FilterTreeBySubstituent<I, F>
where
    R: From<<I::Feed as Feed>::Filtrate>,
    I: HierarchicalIterator,
    I::Feed: Feed<Residue = TreeResidue<R>> + Isomeric,
    F: FnMut(<I::Feed as Isomeric>::Substituent<'_>) -> Option<TreeResidue<()>>,
{
    type Item = <I::Feed as Feed>::Filtrate;

    fn next(&mut self) -> Option<Self::Item> {
        filtrate(self)
    }
}

#[derive(Clone, Debug)]
pub struct FilterMapTree<I, S, F> {
    input: I,
    f: F,
    _phantom: PhantomData<fn() -> S>,
}

impl<I, S, F> CancelWalk for FilterMapTree<I, S, F>
where
    I: CancelWalk,
{
    fn cancel_walk_tree(&mut self) {
        self.input.cancel_walk_tree()
    }
}

impl<R, I, S, F> Iterator for FilterMapTree<I, S, F>
where
    I: SeparatingFilter,
    I::Feed: Feed<Residue = TreeResidue<R>>,
    S: Feed,
    F: FnMut(WalkCancellation<I>, Separation<I::Feed>) -> Separation<S>,
{
    type Item = <<Self as SeparatingFilter>::Feed as Feed>::Filtrate;

    fn next(&mut self) -> Option<Self::Item> {
        filtrate(self)
    }
}

impl<R, I, S, F> SeparatingFilter for FilterMapTree<I, S, F>
where
    I: SeparatingFilter,
    I::Feed: Feed<Residue = TreeResidue<R>>,
    S: Feed,
    F: FnMut(WalkCancellation<I>, Separation<I::Feed>) -> Separation<S>,
{
    type Feed = S;

    fn feed(&mut self) -> Option<Separation<Self::Feed>> {
        let separation = self.input.feed();
        separation.map(|separation| (self.f)(WalkCancellation(&mut self.input), separation))
    }
}

/// Feeds a [`SeparatingFilter`] and yields the next filtrate.
///
/// This function can be used to implement [`Iterator`] for [`SeparatingFilter`]s and bridges
/// [`SeparatingFilter`]s into the output of an iterator. See the [`SeparatingFilterInput`] trait
/// for the input analog.
///
/// [`Iterator`]: std::iter::Iterator
/// [`SeparatingFilter`]: crate::walk::filter::SeparatingFilter
/// [`SeparatingFilterInput`]: crate::walk::filter::SeparatingFilterInput
pub fn filtrate<I>(filter: &mut I) -> Option<<I::Feed as Feed>::Filtrate>
where
    I: SeparatingFilter,
{
    loop {
        if let Some(separation) = filter.feed() {
            return match separation.filtrate() {
                None => {
                    continue;
                },
                Some(filtrate) => Some(filtrate.into_inner()),
            };
        }
        return None;
    }
}
