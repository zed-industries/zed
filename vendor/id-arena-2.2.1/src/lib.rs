//! [![](https://img.shields.io/crates/v/id-arena.svg)](https://crates.io/crates/id-arena)
//! [![](https://img.shields.io/crates/d/id-arena.svg)](https://crates.io/crates/id-arena)
//! [![Travis CI Build Status](https://travis-ci.org/fitzgen/id-arena.svg?branch=master)](https://travis-ci.org/fitzgen/id-arena)
//!
//! A simple, id-based arena.
//!
//! ## Id-based
//!
//! Allocate objects and get an identifier for that object back, *not* a
//! reference to the allocated object. Given an id, you can get a shared or
//! exclusive reference to the allocated object from the arena. This id-based
//! approach is useful for constructing mutable graph data structures.
//!
//! If you want allocation to return a reference, consider [the `typed-arena`
//! crate](https://github.com/SimonSapin/rust-typed-arena/) instead.
//!
//! ## No Deletion
//!
//! This arena does not support deletion, which makes its implementation simple
//! and allocation fast. If you want deletion, you need a way to solve the ABA
//! problem. Consider using [the `generational-arena`
//! crate](https://github.com/fitzgen/generational-arena) instead.
//!
//! ## Homogeneous
//!
//! This crate's arenas can only contain objects of a single type `T`. If you
//! need an arena of objects with heterogeneous types, consider another crate.
//!
//! ## `#![no_std]` Support
//!
//! Requires the `alloc` nightly feature. Disable the on-by-default `"std"` feature:
//!
//! ```toml
//! [dependencies.id-arena]
//! version = "2"
//! default-features = false
//! ```
//!
//! ## `rayon` Support
//!
//! If the `rayon` feature of this crate is activated:
//!
//! ```toml
//! [dependencies]
//! id-arena = { version = "2", features = ["rayon"] }
//! ```
//!
//! then you can use [`rayon`](https://crates.io/crates/rayon)'s support for
//! parallel iteration. The `Arena` type will have a `par_iter` family of
//! methods where appropriate.
//!
//! ## Example
//!
//! ```rust
//! use id_arena::{Arena, Id};
//!
//! type AstNodeId = Id<AstNode>;
//!
//! #[derive(Debug, Eq, PartialEq)]
//! pub enum AstNode {
//!     Const(i64),
//!     Var(String),
//!     Add {
//!         lhs: AstNodeId,
//!         rhs: AstNodeId,
//!     },
//!     Sub {
//!         lhs: AstNodeId,
//!         rhs: AstNodeId,
//!     },
//!     Mul {
//!         lhs: AstNodeId,
//!         rhs: AstNodeId,
//!     },
//!     Div {
//!         lhs: AstNodeId,
//!         rhs: AstNodeId,
//!     },
//! }
//!
//! let mut ast_nodes = Arena::<AstNode>::new();
//!
//! // Create the AST for `a * (b + 3)`.
//! let three = ast_nodes.alloc(AstNode::Const(3));
//! let b = ast_nodes.alloc(AstNode::Var("b".into()));
//! let b_plus_three = ast_nodes.alloc(AstNode::Add {
//!     lhs: b,
//!     rhs: three,
//! });
//! let a = ast_nodes.alloc(AstNode::Var("a".into()));
//! let a_times_b_plus_three = ast_nodes.alloc(AstNode::Mul {
//!     lhs: a,
//!     rhs: b_plus_three,
//! });
//!
//! // Can use indexing to access allocated nodes.
//! assert_eq!(ast_nodes[three], AstNode::Const(3));
//! ```

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
// In no-std mode, use the alloc crate to get `Vec`.
#![no_std]
#![cfg_attr(not(feature = "std"), feature(alloc))]

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::iter;
use core::marker::PhantomData;
use core::ops;
use core::slice;
use core::sync::atomic::{self, AtomicUsize, ATOMIC_USIZE_INIT};

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::{self, Vec};

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::vec::{self, Vec};

#[cfg(feature = "rayon")]
mod rayon;
#[cfg(feature = "rayon")]
pub use rayon::*;

/// A trait representing the implementation behavior of an arena and how
/// identifiers are represented.
///
/// ## When should I implement `ArenaBehavior` myself?
///
/// Usually, you should just use `DefaultArenaBehavior`, which is simple and
/// correct. However, there are some scenarios where you might want to implement
/// `ArenaBehavior` yourself:
///
/// * **Space optimizations:** The default identifier is two words in size,
/// which is larger than is usually necessary. For example, if you know that an
/// arena *cannot* contain more than 256 items, you could make your own
/// identifier type that stores the index as a `u8` and then you can save some
/// space.
///
/// * **Trait Coherence:** If you need to implement an upstream crate's traits
/// for identifiers, then defining your own identifier type allows you to work
/// with trait coherence rules.
///
/// * **Share identifiers across arenas:** You can coordinate and share
/// identifiers across different arenas to enable a "struct of arrays" style
/// data representation.
pub trait ArenaBehavior {
    /// The identifier type.
    type Id: Copy;

    /// Construct a new object identifier from the given index and arena
    /// identifier.
    ///
    /// ## Panics
    ///
    /// Implementations are allowed to panic if the given index is larger than
    /// the underlying storage (e.g. the implementation uses a `u8` for storing
    /// indices and the given index value is larger than 255).
    fn new_id(arena_id: u32, index: usize) -> Self::Id;

    /// Get the given identifier's index.
    fn index(Self::Id) -> usize;

    /// Get the given identifier's arena id.
    fn arena_id(Self::Id) -> u32;

    /// Construct a new arena identifier.
    ///
    /// This is used to disambiguate `Id`s across different arenas. To make
    /// identifiers with the same index from different arenas compare false for
    /// equality, return a unique `u32` on every invocation. This is the
    /// default, provided implementation's behavior.
    ///
    /// To make identifiers with the same index from different arenas compare
    /// true for equality, return the same `u32` on every invocation.
    fn new_arena_id() -> u32 {
        static ARENA_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
        ARENA_COUNTER.fetch_add(1, atomic::Ordering::SeqCst) as u32
    }
}

/// An identifier for an object allocated within an arena.
pub struct Id<T> {
    idx: usize,
    arena_id: u32,
    _ty: PhantomData<fn() -> T>,
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Id").field("idx", &self.idx).finish()
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    #[inline]
    fn clone(&self) -> Id<T> {
        *self
    }
}

impl<T> PartialEq for Id<T> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.arena_id == rhs.arena_id && self.idx == rhs.idx
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    #[inline]
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.arena_id.hash(h);
        self.idx.hash(h);
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.arena_id
            .cmp(&rhs.arena_id)
            .then(self.idx.cmp(&rhs.idx))
    }
}

impl<T> Id<T> {
    /// Get the index within the arena that this id refers to.
    #[inline]
    pub fn index(&self) -> usize {
        self.idx
    }
}

/// The default `ArenaBehavior` implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultArenaBehavior<T> {
    _phantom: PhantomData<fn() -> T>,
}

impl<T> ArenaBehavior for DefaultArenaBehavior<T> {
    type Id = Id<T>;

    #[inline]
    fn new_id(arena_id: u32, idx: usize) -> Self::Id {
        Id {
            idx,
            arena_id,
            _ty: PhantomData,
        }
    }

    #[inline]
    fn index(id: Self::Id) -> usize {
        id.idx
    }

    #[inline]
    fn arena_id(id: Self::Id) -> u32 {
        id.arena_id
    }
}

/// An arena of objects of type `T`.
///
/// ```
/// use id_arena::Arena;
///
/// let mut arena = Arena::<&str>::new();
///
/// let a = arena.alloc("Albert");
/// assert_eq!(arena[a], "Albert");
///
/// arena[a] = "Alice";
/// assert_eq!(arena[a], "Alice");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arena<T, A = DefaultArenaBehavior<T>> {
    arena_id: u32,
    items: Vec<T>,
    _phantom: PhantomData<fn() -> A>,
}

impl<T, A> Default for Arena<T, A>
where
    A: ArenaBehavior,
{
    #[inline]
    fn default() -> Arena<T, A> {
        Arena {
            arena_id: A::new_arena_id(),
            items: Vec::new(),
            _phantom: PhantomData,
        }
    }
}

impl<T, A> Arena<T, A>
where
    A: ArenaBehavior,
{
    /// Construct a new, empty `Arena`.
    ///
    /// ```
    /// use id_arena::Arena;
    ///
    /// let mut arena = Arena::<usize>::new();
    /// arena.alloc(42);
    /// ```
    #[inline]
    pub fn new() -> Arena<T, A> {
        Default::default()
    }

    /// Construct a new, empty `Arena` with capacity for the given number of
    /// elements.
    ///
    /// ```
    /// use id_arena::Arena;
    ///
    /// let mut arena = Arena::<usize>::with_capacity(100);
    /// for x in 0..100 {
    ///     arena.alloc(x * x);
    /// }
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Arena<T, A> {
        Arena {
            arena_id: A::new_arena_id(),
            items: Vec::with_capacity(capacity),
            _phantom: PhantomData,
        }
    }

    /// Allocate `item` within this arena and return its id.
    ///
    /// ```
    /// use id_arena::Arena;
    ///
    /// let mut arena = Arena::<usize>::new();
    /// let _id = arena.alloc(42);
    /// ```
    ///
    /// ## Panics
    ///
    /// Panics if the number of elements in the arena overflows a `usize` or
    /// `Id`'s index storage representation.
    #[inline]
    pub fn alloc(&mut self, item: T) -> A::Id {
        let id = self.next_id();
        self.items.push(item);
        id
    }

    /// Allocate an item with the id that it will be assigned.
    ///
    /// This is useful for structures that want to store their id as their own
    /// member.
    ///
    /// ```
    /// use id_arena::{Arena, Id};
    ///
    /// struct Cat {
    ///     id: Id<Cat>,
    /// }
    ///
    /// let mut arena = Arena::<Cat>::new();
    ///
    /// let kitty = arena.alloc_with_id(|id| Cat { id });
    /// assert_eq!(arena[kitty].id, kitty);
    /// ```
    #[inline]
    pub fn alloc_with_id(&mut self, f: impl FnOnce(A::Id) -> T) -> A::Id {
        let id = self.next_id();
        let val = f(id);
        self.alloc(val)
    }

    /// Get the id that will be used for the next item allocated into this
    /// arena.
    ///
    /// If you are allocating a `struct` that wants to have its id as a member
    /// of itself, prefer the less error-prone `Arena::alloc_with_id` method.
    #[inline]
    pub fn next_id(&self) -> A::Id {
        let arena_id = self.arena_id;
        let idx = self.items.len();
        A::new_id(arena_id, idx)
    }

    /// Get a shared reference to the object associated with the given `id` if
    /// it exists.
    ///
    /// If there is no object associated with `id` (for example, it might
    /// reference an object allocated within a different arena) then return
    /// `None`.
    ///
    /// ```
    /// use id_arena::Arena;
    ///
    /// let mut arena = Arena::<usize>::new();
    /// let id = arena.alloc(42);
    /// assert!(arena.get(id).is_some());
    ///
    /// let other_arena = Arena::<usize>::new();
    /// assert!(other_arena.get(id).is_none());
    /// ```
    #[inline]
    pub fn get(&self, id: A::Id) -> Option<&T> {
        if A::arena_id(id) != self.arena_id {
            None
        } else {
            self.items.get(A::index(id))
        }
    }

    /// Get an exclusive reference to the object associated with the given `id`
    /// if it exists.
    ///
    /// If there is no object associated with `id` (for example, it might
    /// reference an object allocated within a different arena) then return
    /// `None`.
    ///
    /// ```
    /// use id_arena::Arena;
    ///
    /// let mut arena = Arena::<usize>::new();
    /// let id = arena.alloc(42);
    /// assert!(arena.get_mut(id).is_some());
    ///
    /// let mut other_arena = Arena::<usize>::new();
    /// assert!(other_arena.get_mut(id).is_none());
    /// ```
    #[inline]
    pub fn get_mut(&mut self, id: A::Id) -> Option<&mut T> {
        if A::arena_id(id) != self.arena_id {
            None
        } else {
            self.items.get_mut(A::index(id))
        }
    }

    /// Iterate over this arena's items and their ids.
    ///
    /// ```
    /// use id_arena::Arena;
    ///
    /// let mut arena = Arena::<&str>::new();
    ///
    /// arena.alloc("hello");
    /// arena.alloc("hi");
    /// arena.alloc("yo");
    ///
    /// for (id, s) in arena.iter() {
    ///     assert_eq!(arena.get(id).unwrap(), s);
    ///     println!("{:?} -> {}", id, s);
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<T, A> {
        IntoIterator::into_iter(self)
    }

    /// Iterate over this arena's items and their ids, allowing mutation of each
    /// item.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T, A> {
        IntoIterator::into_iter(self)
    }

    /// Get the number of objects allocated in this arena.
    ///
    /// ```
    /// use id_arena::Arena;
    ///
    /// let mut arena = Arena::<&str>::new();
    ///
    /// arena.alloc("hello");
    /// arena.alloc("hi");
    ///
    /// assert_eq!(arena.len(), 2);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T, A> ops::Index<A::Id> for Arena<T, A>
where
    A: ArenaBehavior,
{
    type Output = T;

    #[inline]
    fn index(&self, id: A::Id) -> &T {
        assert_eq!(self.arena_id, A::arena_id(id));
        &self.items[A::index(id)]
    }
}

impl<T, A> ops::IndexMut<A::Id> for Arena<T, A>
where
    A: ArenaBehavior,
{
    #[inline]
    fn index_mut(&mut self, id: A::Id) -> &mut T {
        assert_eq!(self.arena_id, A::arena_id(id));
        &mut self.items[A::index(id)]
    }
}

fn add_id<A, T>(item: Option<(usize, T)>, arena_id: u32) -> Option<(A::Id, T)>
where
    A: ArenaBehavior,
{
    item.map(|(idx, item)| (A::new_id(arena_id, idx), item))
}

/// An iterator over `(Id, &T)` pairs in an arena.
///
/// See [the `Arena::iter()` method](./struct.Arena.html#method.iter) for details.
#[derive(Debug)]
pub struct Iter<'a, T: 'a, A: 'a> {
    arena_id: u32,
    iter: iter::Enumerate<slice::Iter<'a, T>>,
    _phantom: PhantomData<fn() -> A>,
}

impl<'a, T: 'a, A: 'a> Iterator for Iter<'a, T, A>
where
    A: ArenaBehavior,
{
    type Item = (A::Id, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        add_id::<A, _>(self.iter.next(), self.arena_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T: 'a, A: 'a> DoubleEndedIterator for Iter<'a, T, A>
where
    A: ArenaBehavior,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        add_id::<A, _>(self.iter.next_back(), self.arena_id)
    }
}

impl<'a, T: 'a, A: 'a> ExactSizeIterator for Iter<'a, T, A>
where
    A: ArenaBehavior,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T, A> IntoIterator for &'a Arena<T, A>
where
    A: ArenaBehavior,
{
    type Item = (A::Id, &'a T);
    type IntoIter = Iter<'a, T, A>;

    #[inline]
    fn into_iter(self) -> Iter<'a, T, A> {
        Iter {
            arena_id: self.arena_id,
            iter: self.items.iter().enumerate(),
            _phantom: PhantomData,
        }
    }
}

/// An iterator over `(Id, &mut T)` pairs in an arena.
///
/// See [the `Arena::iter_mut()` method](./struct.Arena.html#method.iter_mut)
/// for details.
#[derive(Debug)]
pub struct IterMut<'a, T: 'a, A: 'a> {
    arena_id: u32,
    iter: iter::Enumerate<slice::IterMut<'a, T>>,
    _phantom: PhantomData<fn() -> A>,
}

impl<'a, T: 'a, A: 'a> Iterator for IterMut<'a, T, A>
where
    A: ArenaBehavior,
{
    type Item = (A::Id, &'a mut T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        add_id::<A, _>(self.iter.next(), self.arena_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T: 'a, A: 'a> DoubleEndedIterator for IterMut<'a, T, A>
where
    A: ArenaBehavior,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        add_id::<A, _>(self.iter.next_back(), self.arena_id)
    }
}

impl<'a, T: 'a, A: 'a> ExactSizeIterator for IterMut<'a, T, A>
where
    A: ArenaBehavior,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T, A> IntoIterator for &'a mut Arena<T, A>
where
    A: ArenaBehavior,
{
    type Item = (A::Id, &'a mut T);
    type IntoIter = IterMut<'a, T, A>;

    #[inline]
    fn into_iter(self) -> IterMut<'a, T, A> {
        IterMut {
            arena_id: self.arena_id,
            iter: self.items.iter_mut().enumerate(),
            _phantom: PhantomData,
        }
    }
}

/// An iterator over `(Id, T)` pairs in an arena.
#[derive(Debug)]
pub struct IntoIter<T, A> {
    arena_id: u32,
    iter: iter::Enumerate<vec::IntoIter<T>>,
    _phantom: PhantomData<fn() -> A>,
}

impl<T, A> Iterator for IntoIter<T, A>
where
    A: ArenaBehavior,
{
    type Item = (A::Id, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        add_id::<A, _>(self.iter.next(), self.arena_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, A> DoubleEndedIterator for IntoIter<T, A>
where
    A: ArenaBehavior,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        add_id::<A, _>(self.iter.next_back(), self.arena_id)
    }
}

impl<T, A> ExactSizeIterator for IntoIter<T, A>
where
    A: ArenaBehavior,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, A> IntoIterator for Arena<T, A>
where
    A: ArenaBehavior,
{
    type Item = (A::Id, T);
    type IntoIter = IntoIter<T, A>;

    #[inline]
    fn into_iter(self) -> IntoIter<T, A> {
        IntoIter {
            arena_id: self.arena_id,
            iter: self.items.into_iter().enumerate(),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        struct Foo;
        assert_send_sync::<Id<Foo>>();
    }
}
